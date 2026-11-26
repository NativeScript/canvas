import { AnalyserOptions, AudioBufferCopyOptions, AudioContextOptions, AudioContextState, BaseAudioContext, ChannelMergerOptions, ChannelSplitterOptions, ConstantSourceOptions, ConvolverOptions, DelayOptions, DistanceModelType, DynamicsCompressorOptions, IIRFilterOptions, LatencyHint, MediaElementLike, MediaElementTapProvider, PanningModelType, PannerOptions, PeriodicWaveOptions, StereoPannerOptions, WaveShaperOptions } from './common';

export { AnalyserOptions, AudioBufferCopyOptions, AudioContextOptions, AudioContextState, ChannelMergerOptions, ChannelSplitterOptions, ConstantSourceOptions, ConvolverOptions, DelayOptions, DistanceModelType, DynamicsCompressorOptions, IIRFilterOptions, LatencyHint, MediaElementLike, MediaElementTapProvider, PanningModelType, PannerOptions, PeriodicWaveOptions, StereoPannerOptions, WaveShaperOptions };

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
	readonly numberOfInputs: number;
	readonly numberOfOutputs: number;
	channelCount: number;
	channelCountMode: 'max' | 'clamped-max' | 'explicit';
	channelInterpretation: 'speakers' | 'discrete';

	connect(): void;

	connect(destination: AudioNode): AudioNode;

	connect(destination: AudioNode, outputIndex: number): AudioNode;

	connect(destination: AudioNode, outputIndex: number, inputIndex: number): AudioNode;

	connect(destination: AudioParam): void;

	connect(destination: AudioParam, outputIndex: number): void;

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

	copyFromChannel(dest: Float32Array, channel: number, startInChannel?: number | AudioBufferCopyOptions);

	copyToChannel(source: Float32Array, channel: number, startInChannel?: number | AudioBufferCopyOptions);
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

export declare class AudioDestinationNode extends AudioNode {
	readonly maxChannelCount: number;
}

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
	constructor(context: BaseAudioContext, options?: { type?: string; frequency?: number; detune?: number; periodicWave?: PeriodicWave });

	type: string;

	readonly frequency: AudioParam;
	readonly detune: AudioParam;

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

export declare class DynamicsCompressorNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: DynamicsCompressorOptions);
	readonly threshold: AudioParam;
	readonly knee: AudioParam;
	readonly ratio: AudioParam;
	readonly attack: AudioParam;
	readonly release: AudioParam;
	readonly reduction: AudioParam;
}

export declare class ChannelSplitterNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: ChannelSplitterOptions);
	readonly numberOfOutputs: number;
}

export declare class ChannelMergerNode extends AudioNode {
	constructor(context: BaseAudioContext, options?: ChannelMergerOptions);
	readonly numberOfInputs: number;
}

export declare class PeriodicWave {
	constructor(context: BaseAudioContext, options?: PeriodicWaveOptions);
}

export declare class AudioBufferSourceNode extends AudioScheduledSourceNode {
	constructor(context: BaseAudioContext, options?: { buffer?: AudioBuffer; loop?: boolean; loopStart?: number; loopEnd?: number; playbackRate?: number; detune?: number });
	loop: boolean;
	loopStart: number;
	loopEnd: number;
	buffer: AudioBuffer | null;
	readonly playbackRate: AudioParam;
	readonly detune: AudioParam;
}

export interface AudioWorkletNodeOptions {
	numberOfInputs?: number;
	numberOfOutputs?: number;
	outputChannelCount?: number[];
	parameterData?: Record<string, number>;
	processorOptions?: any;
	channelCount?: number;
	channelCountMode?: string;
	channelInterpretation?: string;
}

/** Windows, Linux and Android. */
export declare class AudioWorklet {
	addModule(moduleURL: string | URL, options?: { credentials?: string }): Promise<void>;
}

/** Windows, Linux and Android (a source there: no inputs, one output). */
export declare class AudioWorkletNode extends AudioNode {
	constructor(context: BaseAudioContext, name: string, options?: AudioWorkletNodeOptions);
	readonly port: MessagePort;
	readonly parameters: ReadonlyMap<string, AudioParam>;
	onprocessorerror: ((event: { type: 'processorerror'; message: string; error: Error }) => void) | null;
}

export declare class AudioContext extends BaseAudioContext {
	constructor(options?: AudioContextOptions);

	readonly destination: AudioDestinationNode;
	/** Undefined where the runtime has no Worker. */
	readonly audioWorklet: AudioWorklet | undefined;

	createGain(options?: { gain?: number }): GainNode;
	createBiquadFilter(options?: { type?: string; frequency?: number; Q?: number; gain?: number }): BiquadFilterNode;
	createPanner(options?: PannerOptions): PannerNode;
	createOscillator(options?: { type?: string; frequency?: number }): OscillatorNode;
	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number }): AudioBuffer;
	createBuffer(numberOfChannels: number, length: number, sampleRate: number): AudioBuffer;
	createBufferSource(): AudioBufferSourceNode;
	createStereoPanner(options?: StereoPannerOptions): StereoPannerNode;
	createDelay(options?: DelayOptions): DelayNode;
	createConstantSource(options?: ConstantSourceOptions): ConstantSourceNode;
	createAnalyser(options?: AnalyserOptions): AnalyserNode;
	createWaveShaper(options?: WaveShaperOptions): WaveShaperNode;
	createIIRFilter(feedforward: number[], feedback: number[]): IIRFilterNode;
	createConvolver(options?: ConvolverOptions): ConvolverNode;
	createDynamicsCompressor(options?: DynamicsCompressorOptions): DynamicsCompressorNode;
	createChannelSplitter(options?: ChannelSplitterOptions): ChannelSplitterNode;
	createChannelMerger(options?: ChannelMergerOptions): ChannelMergerNode;
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }): PeriodicWave;
	createMediaElementSource(mediaElement: MediaElementLike): MediaElementAudioSourceNode;

	createSourceNodeFromPlayer(playerNative: any): AudioNode | null;

	readonly state: AudioContextState;
	onstatechange: ((ev: { type: 'statechange' }) => void) | null;
	addEventListener(type: 'statechange', listener: (ev: { type: 'statechange' }) => void): void;
	removeEventListener(type: 'statechange', listener: (ev: { type: 'statechange' }) => void): void;

	readonly sampleRate: number;
	readonly currentTime: number;
	readonly baseLatency: number;
	readonly outputLatency: number;
	readonly listener: AudioListener;
	getOutputTimestamp(): { contextTime: number; performanceTime: number };
	/** Throws NotSupportedError: NativeScript has no MediaStream. */
	createMediaStreamSource(stream: any): never;
	createMediaStreamTrackSource(track: any): never;
	createMediaStreamDestination(options?: any): never;

	resume(): Promise<void>;
	suspend(): Promise<void>;
	close(): Promise<void>;
	readonly sinkId: string;
	setSinkId(deviceId: string): Promise<void>;

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer>;
}

export declare class OfflineAudioContext extends BaseAudioContext {
	readonly destination: AudioDestinationNode;
	readonly audioWorklet: AudioWorklet | undefined;

	constructor(numberOfChannels: number, lengthInFrames: number, sampleRate: number);
	constructor(options: { numberOfChannels?: number; length: number; sampleRate: number });

	readonly sampleRate: number;
	readonly currentTime: number;
	readonly listener: AudioListener;
	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number }): AudioBuffer;
	createBuffer(numberOfChannels: number, length: number, sampleRate: number): AudioBuffer;

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
	createDynamicsCompressor(options?: DynamicsCompressorOptions): DynamicsCompressorNode;
	createChannelSplitter(options?: ChannelSplitterOptions): ChannelSplitterNode;
	createChannelMerger(options?: ChannelMergerOptions): ChannelMergerNode;
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }): PeriodicWave;

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer>;

	startRendering(): Promise<AudioBuffer>;
	/** Holds rendering at `suspendTime`, rounded up to a 128-frame quantum; resolves once held there. */
	suspend(suspendTime: number): Promise<void>;
	/** Continues a render held at a suspend. Rejects before `startRendering`. */
	resume(): Promise<void>;
	readonly length: number;
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
