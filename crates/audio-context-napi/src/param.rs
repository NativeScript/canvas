use napi::bindgen_prelude::Float32ArraySlice;
use napi::Result;
use napi_derive::napi;
use web_audio_api::AutomationRate;

use crate::{error, guard};

/// An `AudioParam` of a node or of the context's listener; clones share the one timeline.
#[napi]
pub struct AudioParam {
  pub(crate) inner: web_audio_api::AudioParam,
}

impl AudioParam {
  pub(crate) fn wrap(param: &web_audio_api::AudioParam) -> Self {
    Self { inner: param.clone() }
  }
}

#[napi]
impl AudioParam {
  /// The value the render thread last computed (or the last one set, before it has rendered).
  #[napi(getter)]
  pub fn value(&self) -> f64 {
    self.inner.value() as f64
  }

  #[napi(setter, js_name = "value")]
  pub fn set_value(&self, value: f64) -> Result<()> {
    guard(|| {
      self.inner.set_value(value as f32);
    })
  }

  #[napi(getter)]
  pub fn default_value(&self) -> f64 {
    self.inner.default_value() as f64
  }

  #[napi(getter)]
  pub fn min_value(&self) -> f64 {
    self.inner.min_value() as f64
  }

  #[napi(getter)]
  pub fn max_value(&self) -> f64 {
    self.inner.max_value() as f64
  }

  #[napi(getter)]
  pub fn automation_rate(&self) -> &'static str {
    match self.inner.automation_rate() {
      AutomationRate::A => "a-rate",
      AutomationRate::K => "k-rate",
    }
  }

  #[napi(setter, js_name = "automationRate")]
  pub fn set_automation_rate(&self, value: String) -> Result<()> {
    let rate = match value.as_str() {
      "a-rate" => AutomationRate::A,
      "k-rate" => AutomationRate::K,
      _ => return Err(error(format!("TypeError: '{value}' is not a valid AutomationRate"))),
    };
    guard(|| self.inner.set_automation_rate(rate))
  }

  #[napi]
  pub fn set_value_at_time(&self, value: f64, start_time: f64) -> Result<()> {
    guard(|| {
      self.inner.set_value_at_time(value as f32, start_time);
    })
  }

  #[napi]
  pub fn linear_ramp_to_value_at_time(&self, value: f64, end_time: f64) -> Result<()> {
    guard(|| {
      self.inner.linear_ramp_to_value_at_time(value as f32, end_time);
    })
  }

  #[napi]
  pub fn exponential_ramp_to_value_at_time(&self, value: f64, end_time: f64) -> Result<()> {
    guard(|| {
      self.inner.exponential_ramp_to_value_at_time(value as f32, end_time);
    })
  }

  #[napi]
  pub fn set_target_at_time(&self, value: f64, start_time: f64, time_constant: f64) -> Result<()> {
    guard(|| {
      self.inner.set_target_at_time(value as f32, start_time, time_constant);
    })
  }

  #[napi]
  pub fn set_value_curve_at_time(&self, values: Float32ArraySlice, start_time: f64, duration: f64) -> Result<()> {
    guard(|| {
      self.inner.set_value_curve_at_time(&values, start_time, duration);
    })
  }

  #[napi]
  pub fn cancel_scheduled_values(&self, cancel_time: f64) -> Result<()> {
    guard(|| {
      self.inner.cancel_scheduled_values(cancel_time);
    })
  }

  #[napi]
  pub fn cancel_and_hold_at_time(&self, cancel_time: f64) -> Result<()> {
    guard(|| {
      self.inner.cancel_and_hold_at_time(cancel_time);
    })
  }
}
