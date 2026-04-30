import { AnalyserOptions, AudioContextOptions, AudioContextState, BaseAudioContext, ConstantSourceOptions, ConvolverOptions, DelayOptions, DistanceModelType, IIRFilterOptions, LatencyHint, MediaElementLike, MediaElementTapProvider, PanningModelType, PannerOptions, PeriodicWaveOptions, StereoPannerOptions, WaveShaperOptions } from './common';

export { AnalyserOptions, AudioContextOptions, AudioContextState, ConstantSourceOptions, ConvolverOptions, DelayOptions, DistanceModelType, IIRFilterOptions, LatencyHint, MediaElementLike, MediaElementTapProvider, PanningModelType, PannerOptions, PeriodicWaveOptions, StereoPannerOptions, WaveShaperOptions };

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

	distanceModel: DistanceModelType;
	panningModel: PanningModelType;
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
	start(when?: number, offset?: number, duration?: number);
	stop(when?: number);
	onended: ((ev: { type: 'ended' }) => void) | null;
	addEventListener(type: 'ended', listener: (ev: { type: 'ended' }) => void): void;
	removeEventListener(type: 'ended', listener: (ev: { type: 'ended' }) => void): void;
}

export declare class MediaElementAudioSourceNode extends AudioNode {
	readonly mediaElement: MediaElementLike;
	readonly playbackRate: AudioParam | null;
	disposeMediaElementSource(): void;
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
	readonly playbackRate: AudioParam;
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
	createMediaElementSource(mediaElement: MediaElementLike): MediaElementAudioSourceNode;

	createSourceNodeFromPlayer(playerNative: any): AudioNode | null;

	readonly state: AudioContextState;
	onstatechange: ((ev: { type: 'statechange' }) => void) | null;
	addEventListener(type: 'statechange', listener: (ev: { type: 'statechange' }) => void): void;
	removeEventListener(type: 'statechange', listener: (ev: { type: 'statechange' }) => void): void;

	readonly sampleRate: number;
	readonly currentTime: number;

	resume(): Promise<void>;
	suspend(): Promise<void>;
	close(): Promise<void>;
	readonly sinkId: string;
	setSinkId(deviceId: string): Promise<void>;

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer>;
}

export declare class OfflineAudioContext extends BaseAudioContext {
	readonly destination: AudioDestinationNode;

	constructor(numberOfChannels: number, lengthInFrames: number, sampleRate: number);

	createGain(options?: { gain?: number }): GainNode;
	createBiquadFilter(options?: { type?: string; frequency?: number; Q?: number; gain?: number }): BiquadFilterNode;
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

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer>;

	startRendering(): Promise<AudioBuffer>;
}

export class AudioListener {
	readonly positionX: AudioParam;
	readonly positionY: AudioParam;
	readonly positionZ: AudioParam;

	readonly forwardX: AudioParam;
	readonly forwardY: AudioParam;
	readonly forwardZ: AudioParam;

	readonly upX: AudioParam;
	readonly upY: AudioParam;
	readonly upZ: AudioParam;

	setPosition(x: number, y: number, z: number): void;

	setOrientation(forwardX: number, forwardY: number, forwardZ: number, upX?: number, upY?: number, upZ?: number): void;
}
