# @nativescript/audio-context

Web Audio API style audio graph support for NativeScript (iOS and Android).

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

## Optional: Global Polyfill Integration

If you also use `@nativescript/canvas-polyfill`, it can register Web Audio globals such as:

- `AudioContext`
- `webkitAudioContext`
- `OfflineAudioContext`
- node constructors (`GainNode`, `PannerNode`, etc.)

This is useful when running libraries that expect browser-like globals.

## License

Apache License Version 2.0
