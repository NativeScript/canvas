# @nativescript/audio-context

Web Audio API style audio graph support for NativeScript (iOS, Android, Windows and Linux).

This package provides a native-backed `AudioContext`/`OfflineAudioContext` with common Web Audio nodes so you can keep a familiar Web Audio programming model in NativeScript apps.

## Installation

```bash
ns plugin add @nativescript/audio-context
```

or

```bash
npm install @nativescript/audio-context
```

## Quick Start

```ts
import { AudioContext } from '@nativescript/audio-context';

const ctx = new AudioContext({ latencyHint: 'interactive' });

const osc = ctx.createOscillator({ type: 'sine', frequency: 440 });
const gain = ctx.createGain({ gain: 0.12 });

osc.connect(gain);
gain.connect(ctx.destination);

await ctx.resume();
osc.start();

setTimeout(async () => {
	osc.stop();
	await ctx.close();
}, 1200);
```

## Decode And Play Audio

`decodeAudioData` supports:

- App-relative file paths (`~/...`)
- `file://` paths
- Other string sources handled by native decoders (for example URL/data string inputs)
- `ArrayBuffer` / `ArrayBufferView`

```ts
import { AudioContext } from '@nativescript/audio-context';

const ctx = new AudioContext({ latencyHint: 'playback' });
await ctx.resume();

const buffer = await ctx.decodeAudioData('~/assets/file-assets/audio/sine441stereo.mp3');

const source = ctx.createBufferSource();
source.buffer = buffer;
source.connect(ctx.destination);
source.start();

source.onended = async () => {
	await ctx.close();
};
```

## Media Element Source

Use `createMediaElementSource` to route a media element/player through the audio graph.

```ts
import { AudioContext } from '@nativescript/audio-context';
import { Audio } from '@nativescript/canvas-media';

const ctx = new AudioContext();
await ctx.resume();

const media = new Audio();
media.src = '~/assets/file-assets/audio/gs-16b-1c-44100hz.wav';
media.loop = true;

const mediaSource = ctx.createMediaElementSource(media);
const panner = ctx.createPanner({ panningModel: 'equalpower' });

mediaSource.connect(panner);
panner.connect(ctx.destination);

await media.play();

// Cleanup when done.
media.pause();
await ctx.close();
```

## Offline Rendering

```ts
import { OfflineAudioContext } from '@nativescript/audio-context';

const sampleRate = 44100;
const seconds = 2;
const off = new OfflineAudioContext(1, sampleRate * seconds, sampleRate);

const osc = off.createOscillator({ type: 'sine', frequency: 440 });
const gain = off.createGain({ gain: 0.2 });

osc.connect(gain);
gain.connect(off.destination);
osc.start();

const rendered = await off.startRendering();
console.log(rendered.length, rendered.sampleRate, rendered.numberOfChannels);
```

`suspend(time)` holds rendering at `time`, rounded up to a 128-frame quantum, and resolves once it
is held there; changes made before `resume()` apply from that frame. Suspends can be scheduled
before `startRendering()` or while held. On iOS,
connecting a node once rendering has started clears effect state (filter memory, reverb tails);
a `DelayNode` keeps its delay line.

```ts
off.suspend(0.5).then(() => {
  osc.frequency.setValueAtTime(880, off.currentTime);
  off.resume();
});
```

## Supported API Surface

Main contexts:

- `AudioContext`
- `OfflineAudioContext`

Core graph primitives:

- `AudioNode`, `AudioParam`, `AudioBuffer`
- `AudioBufferSourceNode`, `MediaElementAudioSourceNode`
- `GainNode`, `BiquadFilterNode`, `PannerNode`, `StereoPannerNode`, `DelayNode`
- `ConstantSourceNode`, `OscillatorNode`
- `AnalyserNode`, `WaveShaperNode`, `IIRFilterNode`, `ConvolverNode`, `PeriodicWave`

Context lifecycle and routing:

- `resume()`, `suspend()`, `close()`
- `state`, `onstatechange`
- `sinkId`, `setSinkId(deviceId)`

On Android the engine renders the node graph itself, a 128-frame quantum at a time in
topological order: a node's inputs sum every connection, an output can feed any number of nodes,
and each input mixes up or down by its `channelCount`, `channelCountMode` and
`channelInterpretation` (the speaker rules for mono, stereo, quad and 5.1). A cycle plays only
through a `DelayNode`, whose delay is then at least one quantum, as in the spec; a cycle without
one is silent. An `OfflineAudioContext` renders on its own engine with nothing reaching the
device, and a source connected to it reads an `AudioBuffer` in place. A `ConvolverNode` passes its
input through.

On iOS a `GainNode` and a `DelayNode` evaluate `gain` and `delayTime` every frame, so automation
lands on its frame, in an `OfflineAudioContext` and a realtime one, and `gain` can go above 1 or
below 0. A `DelayNode`'s `maxDelayTime` defaults to 1 s and can be up to 180 s; a `delayTime`
between two frames interpolates linearly.

On iOS `disconnect(destination)`, `disconnect(output)`, `disconnect(destination, output, input)`
and `disconnect(param)` remove only the matching connections. Each connection into a
`ChannelMergerNode`, `ChannelSplitterNode` or `AudioWorkletNode` input gets its own bus, mixed by
the speaker rules for mono, stereo, quad and 5.1 (other counts mix discretely); each splitter
output plays its own channel, and the merger puts each input on its channel. Each output of a
multi-output `AudioWorkletNode` plays from its own node, so any number of them can be connected.
`GainNode`, `DelayNode` and `BiquadFilterNode` mix their inputs to `channelCount` when
`channelCountMode` is `'clamped-max'` or `'explicit'` (up to 8 channels); other nodes keep the
`channelCount`, `channelCountMode` and `channelInterpretation` you set without applying them. A
`PannerNode` mixes its inputs to at most two channels and pans with the spec's equal-power law; a
`'HRTF'` panner pans the same way. A cycle plays only through a `DelayNode`, whose delay is then
at least one render call (one quantum offline, the device's buffer in a realtime context); a
connection that would close a cycle without one is not made. An `AudioBuffer` with more channels
than the context plays all of them.

On iOS `node.connect(param)` adds the node's output, mixed down to mono, to the param every frame
for `GainNode.gain`, `DelayNode.delayTime`, the `BiquadFilterNode` params, oscillator `frequency`
and `detune`, `ConstantSourceNode.offset`, `AudioBufferSourceNode.playbackRate` and `detune`, and
the params of an `AudioWorkletNode` with inputs. Other params, such as `StereoPannerNode.pan`,
the `PannerNode` position and orientation and a worklet's params when it has no inputs, follow
the connected audio once per quantum, a quantum late.

On Android `node.connect(param)` adds the node's output, mixed down to mono, to the param every
frame for `GainNode.gain`, `DelayNode.delayTime`, `ConstantSourceNode.offset`,
`AudioBufferSourceNode.playbackRate`, the `DynamicsCompressorNode` params, panner
params and `AudioWorkletNode` params (for a worklet with inputs). `BiquadFilterNode` and listener
params ignore audio connected to them, and oscillator `frequency` follows it but drifts from
the automated value.

## Android memory

Decoded audio, and an `OfflineAudioContext`'s rendered buffer and delay lines, are held in native
memory, not on the Java heap, so long tracks can't throw `OutOfMemoryError`. The samples are freed when the `AudioBuffer` is garbage collected. Neither
V8 nor ART sees that memory, so while a lot of native audio is live the plugin asks Java to
collect. For V8 to follow, enable the runtime's GC bridge in `nativescript.config.ts`:

```ts
android: {
  v8Flags: '--expose_gc',
  gcThrottleTime: 1000,
}
```

Without it the memory is still freed, but only when V8 happens to collect.

## Windows

On Windows the graph is rendered by the [web-audio-api](https://github.com/orottier/web-audio-api-rs)
engine and played through WASAPI (`audiocontext.node`, a Node-API module). `decodeAudioData`
reads wav, mp3, ogg/vorbis, flac and aac. Differences from iOS and Android:

- `createMediaElementSource` throws `NotSupportedError` and `createSourceNodeFromPlayer`
  returns `null`: `@nativescript/canvas-media` has no Windows backend yet.
- A `WaveShaperNode` takes one curve; assigning a second one throws `InvalidStateError`.
- `sinkId` is `''` for the default device; `'none'` renders without a device.

### AudioWorklet

`context.audioWorklet.addModule(url)` and `new AudioWorkletNode(context, name, options)` work as on
the web: `registerProcessor`, `AudioWorkletProcessor`, `port`, `parameters`, `processorOptions`,
`sampleRate`, `currentFrame` and `currentTime`. Processors run in a Worker (one per context), not
on the app's JS thread.

- `url` is a path (`~/`, relative to the app folder, absolute or `file://`), a `data:` URL, or an
  `http(s):` / `blob:` URL the runtime's `fetch` can read.
- A realtime node's output plays about one device buffer late (a few ms at the `interactive`
  latency hint); its params and inputs reach `process()` that much early. Offline rendering is
  sample-exact.
- On iOS the Worker needs `@nativescript/ios` 9.1 or later; earlier runtimes give Workers no
  timers, so `addModule` rejects.
- On iOS each input and output of an `AudioWorkletNode` carries up to 8 channels. An input always
  has `channelCount` channels, as if `channelCountMode` were `'explicit'`. In an
  `OfflineAudioContext` rendering waits for the Worker each quantum, so it is sample-exact too.
- An input or a channel-following output carries at most 8 channels.
- An input with nothing playing into it is empty, as in browsers. A source that has stopped
  still feeds silent channels until it is garbage collected.
- `audioWorklet` is `undefined` where the runtime has no `Worker` (the QuickJS, Hermes and JSC
  builds).
- On Android an `AudioWorkletNode` has up to 16 inputs and 16 outputs of up to 8 channels each,
  with the same input mixing as iOS, and works in an `OfflineAudioContext`. Its ring is a
  `SharedArrayBuffer`; the engine wakes the Worker's looper each time it requests a quantum, so
  realtime output plays up to a device buffer late and an offline render doesn't wait on a timer.
  (`Atomics.notify` can't come from the render thread, so the wake is native.)
- On Android each input is mixed from every source's own channels straight to its
  `channelCount`, with the speaker rules for mono, stereo, quad and 5.1 (other counts mix
  discretely), in any context.

## Linux

Linux uses the same engine and module as Windows (`audiocontext.node`), playing through ALSA
(PulseAudio and PipeWire through their ALSA plugins). The differences listed for Windows apply.

## Optional: Global Polyfill Integration

If you also use `@nativescript/canvas-polyfill`, it can register Web Audio globals such as:

- `AudioContext`
- `webkitAudioContext`
- `OfflineAudioContext`
- node constructors (`GainNode`, `PannerNode`, etc.)

This is useful when running libraries that expect browser-like globals.

## License

Apache License Version 2.0
