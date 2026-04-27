import { AnalyserOptions, AudioContextOptions, BaseAudioContext, ConstantSourceOptions, ConvolverOptions, DelayOptions, IIRFilterOptions, LatencyHint, PannerOptions, PeriodicWaveOptions, StereoPannerOptions, WaveShaperOptions } from './common';

export { AnalyserOptions, AudioContextOptions, ConstantSourceOptions, ConvolverOptions, DelayOptions, IIRFilterOptions, LatencyHint, PannerOptions, PeriodicWaveOptions, StereoPannerOptions, WaveShaperOptions };

export declare class AudioParam {
	value: number;

	automationRate: string;

	setValueAtTime(value: number, time?: number): AudioParam;

	linearRampToValueAtTime(value: number, time: number): AudioParam;

	exponentialRampToValueAtTime(value: number, time: number): AudioParam;

	setTargetAtTime(target: number, startTime: number, timeConstant: number): AudioParam;

	setValueCurveAtTime(values: Float32Array | number[], startTime: number, duration: number): AudioParam;

	cancelScheduledValues(time: number): AudioParam;

	cancelAndHoldAtTime(time: number): AudioParam;
}

export declare class AudioNode {
	constructor(context: BaseAudioContext);

	readonly context: BaseAudioContext;

	connect();

	connect(destination: AudioNode | AudioParam);

	connect(destination: AudioNode | AudioParam, outputIndex: number);

	connect(destination: AudioNode | AudioParam, outputIndex: number, inputIndex: number);

	disconnect();

	disconnect(destination: AudioNode | AudioParam);

	disconnect(destination: AudioNode | AudioParam, output: number);

	disconnect(destination: AudioNode | AudioParam, output: number, input: number);
}

export declare class AudioBuffer {
	constructor(options: { length: number; numberOfChannels: number; sampleRate: number });

	readonly sampleRate: number;

	readonly length: number;

	readonly duration: number;

	readonly numberOfChannels: number;

	getChannelData(channel: number): Float32Array | null;

	copyFromChannel(dest: Float32Array, channel: number, startInChannel?: number);

	copyToChannel(source: Float32Array, channel: number, startInChannel?: number);
}

export declare class GainNode extends AudioNode {
	constructor(context: BaseAudioContext, options: { gain?: number });
	readonly gain: AudioParam;
}

export declare class BiquadFilterNode extends AudioNode {
	constructor(context: BaseAudioContext);
	constructor(context: BaseAudioContext, options: { type?: string; frequency?: number; Q?: number; gain?: number });

	readonly frequency: AudioParam;
	readonly Q: AudioParam;
	readonly gain: AudioParam;
	readonly detune: AudioParam;

	type: string;
}

export declare class PannerNode extends AudioNode {
	constructor(context: BaseAudioContext);
	constructor(context: BaseAudioContext, options: PannerOptions);

	readonly positionX: AudioParam;
	readonly positionY: AudioParam;
	readonly positionZ: AudioParam;

	readonly orientationX: AudioParam;
	readonly orientationY: AudioParam;
	readonly orientationZ: AudioParam;

	distanceModel: number;
	panningModel: number;
	refDistance: number;
	maxDistance: number;
	rolloffFactor: number;
	coneInnerAngle: number;
	coneOuterAngle: number;
	coneOuterGain: number;

	setPosition(x: number, y: number, z: number);
	setOrientation(x: number, y: number, z: number);
}

export declare class AudioDestinationNode extends GainNode {}

export class AudioScheduledSourceNode extends AudioNode {
	start();
	stop();
}

export declare class OscillatorNode extends AudioScheduledSourceNode {
	constructor(context: BaseAudioContext, options: { type?: string; frequency?: number });

	readonly frequency: AudioParam;

	setPeriodicWave(wave: PeriodicWave): void;
}

export declare class StereoPannerNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: StereoPannerOptions);
	readonly pan: AudioParam;
}

export declare class DelayNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: DelayOptions);
	readonly delayTime: AudioParam;
	readonly maxDelayTime: number;
}

export declare class ConstantSourceNode extends AudioScheduledSourceNode {
	constructor(context: BaseAudioContext, options?: ConstantSourceOptions);
	readonly offset: AudioParam;
}

export declare class AnalyserNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: AnalyserOptions);
	fftSize: number;
	readonly frequencyBinCount: number;
	smoothingTimeConstant: number;
	minDecibels: number;
	maxDecibels: number;
	getFloatTimeDomainData(dest: Float32Array): void;
	getByteTimeDomainData(dest: Uint8Array): void;
	getFloatFrequencyData(dest: Float32Array): void;
	getByteFrequencyData(dest: Uint8Array): void;
}

export declare class WaveShaperNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: WaveShaperOptions);
	curve: Float32Array | null;
	oversample: 'none' | '2x' | '4x';
}

export declare class IIRFilterNode extends AudioNode {
	constructor(context: BaseAudioContext, options: IIRFilterOptions);
	getFrequencyResponse(frequencyHz: Float32Array, magResponse: Float32Array, phaseResponse: Float32Array): void;
}

export declare class ConvolverNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: ConvolverOptions);
	buffer: AudioBuffer | null;
	normalize: boolean;
}

export declare class PeriodicWave {
	constructor(context: BaseAudioContext, options?: PeriodicWaveOptions);
}

export declare class AudioBufferSourceNode extends AudioScheduledSourceNode {
	constructor(context: BaseAudioContext, options: { buffer?: AudioBuffer });
	loop: boolean;
	buffer: AudioBuffer | null;
}

export declare class AudioContext extends BaseAudioContext {
	constructor(options?: AudioContextOptions);

	readonly destination: AudioDestinationNode;

	createGain(options?: { gain?: number }): GainNode;
	createBiquad(options?: { type?: string; frequency?: number; Q?: number; gain?: number }): BiquadFilterNode;
	createPanner(options?: PannerOptions): PannerNode;
	createOscillator(options?: { type?: string; frequency?: number }): OscillatorNode;
	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number }): AudioBuffer;
	createBufferSource(options?: { buffer?: AudioBuffer }): AudioBufferSourceNode;
	createStereoPanner(options?: StereoPannerOptions): StereoPannerNode;
	createDelay(options?: DelayOptions): DelayNode;
	createConstantSource(options?: ConstantSourceOptions): ConstantSourceNode;
	createAnalyser(options?: AnalyserOptions): AnalyserNode;
	createWaveShaper(options?: WaveShaperOptions): WaveShaperNode;
	createIIRFilter(feedforward: number[], feedback: number[]): IIRFilterNode;
	createConvolver(options?: ConvolverOptions): ConvolverNode;
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }): PeriodicWave;

	readonly sampleRate: number;
	readonly currentTime: number;

	resume(): Promise<void>;
	suspend(): Promise<void>;
	close(): Promise<void>;

	/**
	 * Currently selected output device id. `'default'` when no override is in
	 * effect; `'speaker'` on iOS when forced to the built-in speaker; an
	 * `AudioDeviceInfo.id` (Android) or `AVAudioSessionPortDescription.UID`
	 * (iOS) otherwise.
	 */
	readonly sinkId: string;

	/**
	 * Mirror of W3C `AudioContext.setSinkId`. Pass `'default'` (or empty) to
	 * clear an override, `'speaker'` to force the built-in speaker on iOS, or
	 * the platform device id string. Resolves on success, rejects with the
	 * underlying error otherwise.
	 */
	setSinkId(deviceId: string): Promise<void>;

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView): Promise<AudioBuffer>;
}

export declare class OfflineAudioContext extends BaseAudioContext {
	readonly destination: AudioDestinationNode;

	constructor(numberOfChannels: number, lengthInFrames: number, sampleRate: number);

	createGain(options?: { gain?: number }): GainNode;
	createBiquad(options?: { type?: string; frequency?: number; Q?: number; gain?: number }): BiquadFilterNode;
	createPanner(options?: PannerOptions): PannerNode;
	createOscillator(options?: { type?: string; frequency?: number }): OscillatorNode;
	createBufferSource(options?: { buffer?: AudioBuffer }): AudioBufferSourceNode;
	createStereoPanner(options?: StereoPannerOptions): StereoPannerNode;
	createDelay(options?: DelayOptions): DelayNode;
	createConstantSource(options?: ConstantSourceOptions): ConstantSourceNode;
	createAnalyser(options?: AnalyserOptions): AnalyserNode;
	createWaveShaper(options?: WaveShaperOptions): WaveShaperNode;
	createIIRFilter(feedforward: number[], feedback: number[]): IIRFilterNode;
	createConvolver(options?: ConvolverOptions): ConvolverNode;
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }): PeriodicWave;

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView): Promise<AudioBuffer>;

	startRendering(): Promise<AudioBuffer>;
}
