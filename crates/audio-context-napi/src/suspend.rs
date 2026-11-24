//! `OfflineAudioContext.suspend(when)` / `resume()`. web-audio-api only takes suspends made before
//! rendering starts, so a hidden processor holds the render thread at each requested quantum.

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use web_audio_api::worklet::{AudioParamValues, AudioWorkletGlobalScope, AudioWorkletProcessor};

use crate::worklet::QUANTUM;

type Notify = Box<dyn Fn(u64) + Send + Sync>;

#[derive(Default)]
struct State {
  frames: BTreeSet<u64>,
  started: bool,
  paused: bool,
  finished: bool,
}

pub struct Suspends {
  length: u64,
  sample_rate: f64,
  state: Mutex<State>,
  resumed: Condvar,
  /// The earliest scheduled frame, so the render thread skips the lock.
  next: AtomicU64,
  notify: Mutex<Option<Notify>>,
}

impl Suspends {
  pub fn new(length: u64, sample_rate: f64) -> Arc<Self> {
    Arc::new(Self {
      length,
      sample_rate,
      state: Mutex::default(),
      resumed: Condvar::new(),
      next: AtomicU64::new(u64::MAX),
      notify: Mutex::new(None),
    })
  }

  fn lock(&self) -> std::sync::MutexGuard<'_, State> {
    self.state.lock().unwrap_or_else(|e| e.into_inner())
  }

  fn update_next(&self, state: &State) {
    self.next.store(state.frames.first().copied().unwrap_or(u64::MAX), Ordering::Release);
  }

  /// `notify` runs on the render thread with the frame it stopped at.
  pub fn set_notify(&self, notify: Option<Notify>) {
    *self.notify.lock().unwrap_or_else(|e| e.into_inner()) = notify;
  }

  /// Schedules a suspend at `time` rounded up to a quantum; returns that frame.
  pub fn schedule(&self, time: f64, current: u64) -> Result<u64, &'static str> {
    if !time.is_finite() || time < 0.0 {
      return Err("InvalidStateError: suspendTime must be a non-negative number");
    }
    let quantum = QUANTUM as f64;
    // The epsilon keeps an exact boundary from rounding up a quantum.
    let frame = (((time * self.sample_rate) / quantum) - 1e-9).ceil().max(0.0) as u64 * QUANTUM as u64;
    let mut state = self.lock();
    if state.finished {
      return Err("InvalidStateError: the OfflineAudioContext is closed");
    }
    if frame >= self.length {
      return Err("InvalidStateError: suspendTime is at or past the end of the render");
    }
    if frame < current || (state.started && frame <= current) {
      return Err("InvalidStateError: suspendTime is not after the current time");
    }
    if !state.frames.insert(frame) {
      return Err("InvalidStateError: a suspend is already scheduled at that time");
    }
    self.update_next(&state);
    Ok(frame)
  }

  pub fn start(&self) {
    self.lock().started = true;
  }

  pub fn resume(&self) -> Result<(), &'static str> {
    let mut state = self.lock();
    if state.finished {
      return Err("InvalidStateError: the OfflineAudioContext is closed");
    }
    if !state.started {
      return Err("InvalidStateError: rendering has not started");
    }
    state.paused = false;
    self.resumed.notify_all();
    Ok(())
  }

  /// Render thread, before the quantum at `frame`: holds there until `resume` if a suspend is due.
  pub fn reach(&self, frame: u64) -> bool {
    if self.next.load(Ordering::Acquire) > frame {
      return false;
    }
    let mut state = self.lock();
    if !state.frames.remove(&frame) {
      // Scheduled while this quantum was rendering: missed.
      while state.frames.first().is_some_and(|&f| f < frame) {
        state.frames.pop_first();
      }
      self.update_next(&state);
      return false;
    }
    self.update_next(&state);
    state.paused = true;
    if let Some(notify) = self.notify.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
      notify(frame);
    }
    while state.paused && !state.finished {
      state = self.resumed.wait(state).unwrap_or_else(|e| e.into_inner());
    }
    true
  }

  /// Rendering ended, or the context went away: releases a held render thread.
  pub fn finish(&self) {
    let mut state = self.lock();
    state.finished = true;
    state.paused = false;
    state.frames.clear();
    self.update_next(&state);
    self.resumed.notify_all();
  }
}

pub struct SuspendProcessor {
  suspends: Arc<Suspends>,
}

impl AudioWorkletProcessor for SuspendProcessor {
  type ProcessorOptions = Arc<Suspends>;

  fn constructor(suspends: Arc<Suspends>) -> Self {
    Self { suspends }
  }

  fn process<'a, 'b>(
    &mut self,
    _inputs: &'b [&'a [&'a [f32]]],
    outputs: &'b mut [&'a mut [&'a mut [f32]]],
    _params: AudioParamValues<'b>,
    scope: &'b AudioWorkletGlobalScope,
  ) -> bool {
    outputs.iter_mut().flat_map(|channels| channels.iter_mut()).for_each(|samples| samples.fill(0.));
    // Changes made while held apply from the next quantum; an event makes the offline loop take them.
    if self.suspends.reach(scope.current_frame + QUANTUM as u64) {
      scope.post_message(Box::new(()));
    }
    true
  }
}
