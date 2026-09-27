import {
	AnalyserOptions,
	AudioBufferCopyOptions,
	AudioContextOptions,
	AudioContextState,
	AudioListenerBase,
	AudioNodeBase,
	AudioParamBase,
	AudioParamHooks,
	BaseAudioContext,
	ChannelMergerOptions,
	ChannelSplitterOptions,
	ConstantSourceOptions,
	ConvolverOptions,
	DelayOptions,
	DistanceModelType,
	DynamicsCompressorOptions,
	IIRFilterOptions,
	MediaElementLike,
	PanningModelType,
	PannerOptions,
	PeriodicWaveOptions,
	StereoPannerOptions,
	WaveShaperOptions,
	assertMediaElementUsable,
	context_,
	distanceModelFromNumber,
	distanceModelToNumber,
	looksLikePath,
	markMediaElementUsed,
	native_,
	nativeCtor_,
	normalizeSourcePath,
	panningModelFromNumber,
	panningModelToNumber,
	throwInvalidMediaElement,
	unmarkMediaElementUsed,
} from './common';

declare const __non_webpack_require__: (specifier: string) => any;

interface NativeParam {
	value: number;
	readonly defaultValue: number;
	readonly minValue: number;
	readonly maxValue: number;
	automationRate: string;
	setValueAtTime(value: number, time: number): void;
	linearRampToValueAtTime(value: number, time: number): void;
	exponentialRampToValueAtTime(value: number, time: number): void;
	setTargetAtTime(value: number, startTime: number, timeConstant: number): void;
	setValueCurveAtTime(values: Float32Array, startTime: number, duration: number): void;
	cancelScheduledValues(time: number): void;
	cancelAndHoldAtTime(time: number): void;
}

interface NativeBuffer {
	readonly sampleRate: number;
	readonly length: number;
	readonly duration: number;
	readonly numberOfChannels: number;
	channelData(channel: number): Float32Array;
	copyFromChannel(destination: Float32Array, channel: number, startInChannel: number): void;
}

/** Any node; members a kind does not have throw. */
interface NativeNode {
	readonly numberOfInputs: number;
	readonly numberOfOutputs: number;
	channelCount: number;
	channelCountMode: string;
	channelInterpretation: string;
	connect(destination: NativeNode, output?: number, input?: number): void;
	connectParam(destination: NativeParam, output?: number): void;
	disconnect(): void;
	disconnectOutput(output: number): void;
	disconnectNode(destination: NativeNode, output?: number, input?: number): void;
	disconnectParam(destination: NativeParam, output?: number): void;
	param(name: string): NativeParam | null;
	[member: string]: any;
}

interface NativeContext {
	readonly sampleRate: number;
	readonly currentTime: number;
	readonly state: AudioContextState;
	readonly baseLatency: number;
	readonly outputLatency: number;
	readonly sinkId: string;
	readonly length: number;
	setOnstatechange(callback: (() => void) | null): void;
	resume(): Promise<void>;
	suspend(): Promise<void>;
	close(): Promise<void>;
	setSinkId(sinkId: string): Promise<void>;
	startRendering(): Promise<NativeBuffer>;
	decodeAudioData(data: Uint8Array): Promise<NativeBuffer>;
	decodeAudioBase64(data: string): Promise<NativeBuffer>;
	decodeAudioFile(path: string): Promise<NativeBuffer>;
	destination(): NativeNode;
	listenerParam(name: string): NativeParam | null;
	createPeriodicWave(real: Float32Array, imag: Float32Array, disableNormalization: boolean): unknown;
	[factory: string]: any;
}

// Loaded on import, so a missing module leaves AudioContext undefined in canvas-polyfill's probe.
const Native = __non_webpack_require__('system_lib://audiocontext.node');

type NativeBaseAudioContext = BaseAudioContext & { native: NativeContext };

function nativeContext(context: BaseAudioContext): NativeContext {
	const native = (context as NativeBaseAudioContext)?.native;
	if (!(context instanceof BaseAudioContext) || !native) {
		throw new TypeError('Argument 1 does not implement interface BaseAudioContext.');
	}
	return native;
}

function toFloat32Array(values: Float32Array | ArrayLike<number>): Float32Array {
	return values instanceof Float32Array ? values : Float32Array.from(values);
}

function normalizeCopyByteOffset(view: ArrayBufferView, byteOffset?: number): number {
	let offset = typeof byteOffset === 'number' && Number.isFinite(byteOffset) ? byteOffset : view.byteOffset;
	offset = Math.max(0, offset | 0);
	if (offset > view.buffer.byteLength) offset = view.buffer.byteLength;
	const align = ((view as any).BYTES_PER_ELEMENT as number) || 1;
	return offset - (offset % align);
}

function resolveAudioBufferCopyOptions(view: ArrayBufferView, startOrOptions?: number | AudioBufferCopyOptions): { startInChannel: number; byteOffset: number } {
	let startInChannel = 0;
	let byteOffset: number | undefined;
	if (typeof startOrOptions === 'number') {
		startInChannel = startOrOptions;
	} else if (startOrOptions) {
		if (typeof startOrOptions.startInChannel === 'number') startInChannel = startOrOptions.startInChannel;
		if (typeof startOrOptions.byteOffset === 'number') byteOffset = startOrOptions.byteOffset;
	}
	if (!Number.isFinite(startInChannel)) startInChannel = 0;
	startInChannel = Math.max(0, startInChannel | 0);
	return {
		startInChannel,
		byteOffset: normalizeCopyByteOffset(view, byteOffset),
	};
}

function viewAtByteOffset(view: Float32Array, byteOffset: number): Float32Array {
	if (byteOffset === view.byteOffset) return view;
	if (byteOffset >= view.buffer.byteLength) return new Float32Array(0);
	const availableBytes = view.buffer.byteLength - byteOffset;
	const length = Math.floor(availableBytes / Float32Array.BYTES_PER_ELEMENT);
	if (length <= 0) return new Float32Array(0);
	return new Float32Array(view.buffer, byteOffset, length);
}

/** `file:///C:/x` and `/C:/x` name a drive path on Windows. */
function windowsPath(source: string): string {
	const path = normalizeSourcePath(source.startsWith('file:///') ? source.substring('file://'.length) : source);
	return /^\/[a-zA-Z]:[\\/]/.test(path) ? path.substring(1) : path;
}

function makeWindowsHooks(native: NativeParam): AudioParamHooks {
	return {
		nativeSet(v) {
			native.value = v;
		},
		nativeScheduleSet(v, t) {
			native.setValueAtTime(v, t);
		},
		nativeScheduleLinearRamp(v, t) {
			native.linearRampToValueAtTime(v, t);
		},
		nativeScheduleExpRamp(v, t) {
			native.exponentialRampToValueAtTime(v, t);
		},
		nativeScheduleTarget(v, startTime, timeConstant) {
			native.setTargetAtTime(v, startTime, timeConstant);
		},
		nativeScheduleCurve(values, startTime, duration) {
			native.setValueCurveAtTime(values, startTime, duration);
		},
		nativeCancel(t) {
			native.cancelScheduledValues(t);
		},
		nativeCancelAndHold(_v, t) {
			native.cancelAndHoldAtTime(t);
		},
		nativeGetValue() {
			return native.value;
		},
		nativeSetAutomationRate(rate) {
			native.automationRate = rate;
		},
	};
}

export class AudioParam extends AudioParamBase {
	private [native_]: NativeParam;

	constructor(a?: any, b?: NativeParam) {
		if (a !== nativeCtor_ || !b) {
			throw new TypeError('Illegal constructor.');
		}
		super(makeWindowsHooks(b), b.value);
		this[native_] = b;
	}

	get native() {
		return this[native_];
	}

	/** The value the render thread last computed, automation included. */
	get value(): number {
		return this[native_].value;
	}

	set value(v: number) {
		this[native_].value = +v;
	}

	get defaultValue(): number {
		return this[native_].defaultValue;
	}

	get minValue(): number {
		return this[native_].minValue;
	}

	get maxValue(): number {
		return this[native_].maxValue;
	}

	get automationRate(): string {
		return this[native_].automationRate;
	}

	set automationRate(v: string) {
		this[native_].automationRate = v;
	}
}

function param(node: NativeNode, name: string): AudioParam {
	return new AudioParam(nativeCtor_, node.param(name));
}

export class AudioNode extends AudioNodeBase {
	protected [native_]: NativeNode;
	private _params: { [name: string]: AudioParam } = {};

	constructor(context: BaseAudioContext, native: NativeNode) {
		super(context);
		this[native_] = native;
	}

	get native() {
		return this[native_];
	}

	/** The node's `name` param, one wrapper per param. */
	protected _param(name: string): AudioParam {
		return this._params[name] || (this._params[name] = param(this[native_], name));
	}

	get numberOfInputs(): number {
		return this[native_].numberOfInputs;
	}

	get numberOfOutputs(): number {
		return this[native_].numberOfOutputs;
	}

	get channelCount(): number {
		return this[native_].channelCount;
	}

	set channelCount(value: number) {
		this[native_].channelCount = value;
	}

	get channelCountMode(): string {
		return this[native_].channelCountMode;
	}

	set channelCountMode(value: string) {
		this[native_].channelCountMode = value;
	}

	get channelInterpretation(): string {
		return this[native_].channelInterpretation;
	}

	set channelInterpretation(value: string) {
		this[native_].channelInterpretation = value;
	}

	connect(destination: any, output?: number, input?: number) {
		if (destination instanceof AudioParam) {
			this[native_].connectParam(destination.native, output);
			return;
		}
		if (!(destination instanceof AudioNode)) {
			throw new TypeError('AudioNode.connect: Argument 1 is not an AudioNode or AudioParam.');
		}
		this[native_].connect(destination.native, output, input);
		return destination;
	}

	disconnect(destinationOrOutput?: any, output?: number, input?: number) {
		const native = this[native_];
		if (destinationOrOutput == null) {
			native.disconnect();
		} else if (typeof destinationOrOutput === 'number') {
			native.disconnectOutput(destinationOrOutput);
		} else if (destinationOrOutput instanceof AudioParam) {
			native.disconnectParam(destinationOrOutput.native, output);
		} else if (destinationOrOutput instanceof AudioNode) {
			native.disconnectNode(destinationOrOutput.native, output, input);
		} else {
			throw new TypeError('AudioNode.disconnect: Argument 1 is not an AudioNode or AudioParam.');
		}
	}
}

/**
 * Samples live in the JS channel arrays once read or written (`getChannelData` hands those out
 * for in-place writes); the native buffer a node plays is rebuilt from them when it acquires one.
 */
export class AudioBuffer {
	private _native: NativeBuffer | null = null;
	private _channels: Float32Array[] | null = null;
	private _dirty = false;
	private readonly _sampleRate: number;
	private readonly _length: number;
	private readonly _numberOfChannels: number;

	constructor(options: { length: number; numberOfChannels?: number; sampleRate?: number } | Symbol, nativeBuffer?: NativeBuffer) {
		if (!options) throw new TypeError('AudioBuffer constructor requires options');
		if (options === nativeCtor_ && nativeBuffer) {
			this._native = nativeBuffer;
			this._sampleRate = nativeBuffer.sampleRate;
			this._length = nativeBuffer.length;
			this._numberOfChannels = nativeBuffer.numberOfChannels;
			return;
		}
		const opts = options as { length: number; numberOfChannels?: number; sampleRate?: number };
		const length = Math.max(0, opts.length | 0);
		const channels = opts.numberOfChannels != null ? opts.numberOfChannels | 0 : 1;
		const sampleRate = opts.sampleRate != null ? +opts.sampleRate : 48000;
		// The native constructor applies the spec's limits.
		this._native = new Native.AudioBuffer(channels, length, sampleRate);
		this._sampleRate = sampleRate;
		this._length = length;
		this._numberOfChannels = channels;
	}

	/** The samples as a native buffer, rebuilt after JS writes. */
	get native(): NativeBuffer {
		if (this._dirty || !this._native) {
			this._native = Native.AudioBuffer.fromChannels(this._channels, this._sampleRate);
			this._dirty = false;
		}
		return this._native;
	}

	get sampleRate(): number {
		return this._sampleRate;
	}
	get length(): number {
		return this._length;
	}
	get duration(): number {
		return this._length / this._sampleRate;
	}
	get numberOfChannels(): number {
		return this._numberOfChannels;
	}

	private _channel(channel: number): Float32Array {
		if (!(channel >= 0 && channel < this._numberOfChannels)) {
			throw new RangeError(`IndexSizeError: channel ${channel} is out of bounds (${this._numberOfChannels} channels)`);
		}
		if (!this._channels) {
			const native = this._native;
			this._channels = [];
			for (let i = 0; i < this._numberOfChannels; i++) this._channels.push(native.channelData(i));
		}
		return this._channels[channel | 0];
	}

	getChannelData(channel: number): Float32Array {
		const data = this._channel(channel);
		this._dirty = true;
		return data;
	}

	copyFromChannel(dest: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!dest) return;
		const options = resolveAudioBufferCopyOptions(dest, startInChannel);
		const target = viewAtByteOffset(dest, options.byteOffset);
		if (target.length === 0 || options.startInChannel >= this._length) return;
		if (this._channels) {
			const source = this._channel(channel);
			target.set(source.subarray(options.startInChannel, options.startInChannel + target.length));
		} else {
			this._native.copyFromChannel(target, channel, options.startInChannel);
		}
	}

	copyToChannel(source: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!source) return;
		const options = resolveAudioBufferCopyOptions(source, startInChannel);
		const input = viewAtByteOffset(source, options.byteOffset);
		if (input.length === 0 || options.startInChannel >= this._length) return;
		const target = this._channel(channel);
		target.set(input.subarray(0, this._length - options.startInChannel), options.startInChannel);
		this._dirty = true;
	}
}

export class GainNode extends AudioNode {
	constructor(context: BaseAudioContext, options: { gain?: number } = {}) {
		super(context, nativeContext(context).createGain());
		if (typeof options?.gain === 'number') this.gain.value = options.gain;
	}

	get gain(): AudioParam {
		return this._param('gain');
	}
}

export class AudioDestinationNode extends AudioNode {
	constructor(context: BaseAudioContext, node: NativeNode) {
		super(context, node);
	}

	get maxChannelCount(): number {
		return this.native.maxChannelCount;
	}
}

export class BiquadFilterNode extends AudioNode {
	constructor(context: BaseAudioContext, options: { type?: string; frequency?: number; Q?: number; gain?: number; detune?: number } = {}) {
		super(context, nativeContext(context).createBiquadFilter());
		if (options.type) this.type = options.type;
		if (typeof options.frequency === 'number') this.frequency.value = options.frequency;
		if (typeof options.Q === 'number') this.Q.value = options.Q;
		if (typeof options.gain === 'number') this.gain.value = options.gain;
		if (typeof options.detune === 'number') this.detune.value = options.detune;
	}

	get type(): string {
		return this.native.type;
	}
	set type(value: string) {
		this.native.type = value;
	}
	get frequency(): AudioParam {
		return this._param('frequency');
	}
	get Q(): AudioParam {
		return this._param('Q');
	}
	get gain(): AudioParam {
		return this._param('gain');
	}
	get detune(): AudioParam {
		return this._param('detune');
	}

	getFrequencyResponse(frequencyHz: Float32Array, magResponse: Float32Array, phaseResponse: Float32Array) {
		this.native.getFrequencyResponse(frequencyHz, magResponse, phaseResponse);
	}
}

export class PannerNode extends AudioNode {
	constructor(context: BaseAudioContext, options: PannerOptions = {}) {
		super(context, nativeContext(context).createPanner());
		const o = options ?? {};
		for (const name of ['positionX', 'positionY', 'positionZ', 'orientationX', 'orientationY', 'orientationZ']) {
			if (typeof o[name] === 'number') this._param(name).value = o[name];
		}
		if (o.panningModel != null) this.panningModel = o.panningModel;
		if (o.distanceModel != null) this.distanceModel = o.distanceModel;
		for (const name of ['refDistance', 'maxDistance', 'rolloffFactor', 'coneInnerAngle', 'coneOuterAngle', 'coneOuterGain']) {
			if (typeof o[name] === 'number') this[name] = o[name];
		}
	}

	get positionX() {
		return this._param('positionX');
	}
	get positionY() {
		return this._param('positionY');
	}
	get positionZ() {
		return this._param('positionZ');
	}
	get orientationX() {
		return this._param('orientationX');
	}
	get orientationY() {
		return this._param('orientationY');
	}
	get orientationZ() {
		return this._param('orientationZ');
	}

	get distanceModel(): DistanceModelType {
		return this.native.distanceModel;
	}
	set distanceModel(v: DistanceModelType | number) {
		this.native.distanceModel = distanceModelFromNumber(distanceModelToNumber(v));
	}
	get panningModel(): PanningModelType {
		return this.native.panningModel;
	}
	set panningModel(v: PanningModelType | number) {
		this.native.panningModel = panningModelFromNumber(panningModelToNumber(v));
	}
	get refDistance(): number {
		return this.native.refDistance;
	}
	set refDistance(v: number) {
		this.native.refDistance = v;
	}
	get maxDistance(): number {
		return this.native.maxDistance;
	}
	set maxDistance(v: number) {
		this.native.maxDistance = v;
	}
	get rolloffFactor(): number {
		return this.native.rolloffFactor;
	}
	set rolloffFactor(v: number) {
		this.native.rolloffFactor = v;
	}
	get coneInnerAngle(): number {
		return this.native.coneInnerAngle;
	}
	set coneInnerAngle(v: number) {
		this.native.coneInnerAngle = v;
	}
	get coneOuterAngle(): number {
		return this.native.coneOuterAngle;
	}
	set coneOuterAngle(v: number) {
		this.native.coneOuterAngle = v;
	}
	get coneOuterGain(): number {
		return this.native.coneOuterGain;
	}
	set coneOuterGain(v: number) {
		this.native.coneOuterGain = v;
	}

	setPosition(x: number, y: number, z: number) {
		this.positionX.value = +x;
		this.positionY.value = +y;
		this.positionZ.value = +z;
	}
	setOrientation(x: number, y: number, z: number) {
		this.orientationX.value = +x;
		this.orientationY.value = +y;
		this.orientationZ.value = +z;
	}
}

export class AudioScheduledSourceNode extends AudioNode {
	private _onended: ((ev: { type: 'ended' }) => void) | null = null;
	private _nativeEndedWired = false;

	constructor(context: BaseAudioContext, node: NativeNode) {
		super(context, node);
		if (!node || typeof node.start !== 'function') throw new TypeError('Illegal constructor.');
	}

	get onended() {
		return this._onended;
	}
	set onended(cb: ((ev: { type: 'ended' }) => void) | null) {
		if (this._onended) super.removeEventListener('ended', this._onended);
		this._onended = typeof cb === 'function' ? cb : null;
		if (this._onended) {
			super.addEventListener('ended', this._onended);
			this._ensureNativeEndedWired();
		}
	}

	protected _onFirstListenerAdded(type: string) {
		if (type === 'ended') this._ensureNativeEndedWired();
	}

	private _ensureNativeEndedWired() {
		if (this._nativeEndedWired) return;
		this._nativeEndedWired = true;
		// Holds this node until it ends, as a playing node with a listener must stay reachable.
		this.native.setOnended(() => this.dispatchEvent({ type: 'ended', target: this }));
	}

	start(when?: number, offset?: number, duration?: number) {
		this.native.start(when, offset, duration);
	}

	stop(when?: number) {
		this.native.stop(when);
	}
}

export class AudioBufferSourceNode extends AudioScheduledSourceNode {
	private _buffer: AudioBuffer | null = null;

	constructor(context: BaseAudioContext, options: { buffer?: AudioBuffer; loop?: boolean; loopStart?: number; loopEnd?: number; playbackRate?: number; detune?: number } = {}) {
		super(context, nativeContext(context).createBufferSource());
		const o = options ?? {};
		if (o.buffer) this.buffer = o.buffer;
		if (typeof o.loop === 'boolean') this.loop = o.loop;
		if (typeof o.loopStart === 'number') this.loopStart = o.loopStart;
		if (typeof o.loopEnd === 'number') this.loopEnd = o.loopEnd;
		if (typeof o.playbackRate === 'number') this.playbackRate.value = o.playbackRate;
		if (typeof o.detune === 'number') this.detune.value = o.detune;
	}

	get loop(): boolean {
		return this.native.loop;
	}
	set loop(v: boolean) {
		this.native.loop = !!v;
	}
	get loopStart(): number {
		return this.native.loopStart;
	}
	set loopStart(v: number) {
		this.native.loopStart = v;
	}
	get loopEnd(): number {
		return this.native.loopEnd;
	}
	set loopEnd(v: number) {
		this.native.loopEnd = v;
	}

	get buffer(): AudioBuffer | null {
		return this._buffer;
	}
	/** Takes the buffer's samples as they are now; a source takes one buffer. */
	set buffer(v: AudioBuffer | null) {
		if (!v) {
			this._buffer = null;
			return;
		}
		this.native.setBuffer(v.native);
		this._buffer = v;
	}

	get playbackRate(): AudioParam {
		return this._param('playbackRate');
	}

	get detune(): AudioParam {
		return this._param('detune');
	}
}

/**
 * A media element's audio in the graph. canvas-media taps the element's WinRT MediaPlayer with an
 * audio effect: while connected the element itself is silent and its audio (at its volume) plays
 * through the graph, as on the web.
 */
export class MediaElementAudioSourceNode extends AudioNode {
	private _mediaElement: MediaElementLike;

	constructor(context: AudioContext, mediaElement: MediaElementLike) {
		super(context, MediaElementAudioSourceNode._createNative(context, mediaElement));
		this._mediaElement = mediaElement;
		markMediaElementUsed(mediaElement);
	}

	/** The canvas-media element behind canvas-polyfill's <audio> / <video>, or the element itself. */
	private static _tapProvider(mediaElement: MediaElementLike): MediaElementLike {
		return mediaElement?._audio ?? mediaElement?._video ?? mediaElement;
	}

	private static _createNative(context: AudioContext, mediaElement: MediaElementLike): NativeNode {
		assertMediaElementUsable(context, mediaElement);
		const tap = MediaElementAudioSourceNode._tapProvider(mediaElement)?.attachAudioContextTap?.(undefined);
		if (!tap?.address) {
			throwInvalidMediaElement();
		}
		return nativeContext(context).createMediaElementSourceFromTap(tap.address);
	}

	get mediaElement(): MediaElementLike {
		return this._mediaElement;
	}

	get playbackRate(): AudioParam | null {
		return null;
	}

	/** Gives the element its own output back (not in the spec: an element stays connected there). */
	disposeMediaElementSource() {
		const element = this._mediaElement;
		try {
			MediaElementAudioSourceNode._tapProvider(element)?.detachAudioContextTap?.();
		} catch (e) {}
		unmarkMediaElementUsed(element);
	}
}

export class OscillatorNode extends AudioScheduledSourceNode {
	constructor(context: BaseAudioContext, options: { type?: string; frequency?: number; detune?: number; periodicWave?: PeriodicWave } = {}) {
		super(context, nativeContext(context).createOscillator());
		const o = options ?? {};
		if (o.periodicWave) this.setPeriodicWave(o.periodicWave);
		else if (o.type) this.type = o.type;
		if (typeof o.frequency === 'number') this.frequency.value = o.frequency;
		if (typeof o.detune === 'number') this.detune.value = o.detune;
	}

	get type(): string {
		return this.native.type;
	}
	set type(value: string) {
		this.native.type = value;
	}

	get frequency(): AudioParam {
		return this._param('frequency');
	}

	get detune(): AudioParam {
		return this._param('detune');
	}

	setPeriodicWave(wave: PeriodicWave) {
		if (!wave) return;
		this.native.setPeriodicWave(wave.native);
	}
}

export class StereoPannerNode extends AudioNode {
	constructor(context: BaseAudioContext, options: StereoPannerOptions = {}) {
		super(context, nativeContext(context).createStereoPanner());
		if (typeof options?.pan === 'number') this.pan.value = options.pan;
	}

	get pan(): AudioParam {
		return this._param('pan');
	}
}

export class DelayNode extends AudioNode {
	readonly maxDelayTime: number;

	constructor(context: BaseAudioContext, options: DelayOptions = {}) {
		const maxDelayTime = typeof options?.maxDelayTime === 'number' ? options.maxDelayTime : 1.0;
		super(context, nativeContext(context).createDelay(maxDelayTime));
		this.maxDelayTime = maxDelayTime;
		if (typeof options?.delayTime === 'number') this.delayTime.value = options.delayTime;
	}

	get delayTime(): AudioParam {
		return this._param('delayTime');
	}
}

export class ConstantSourceNode extends AudioScheduledSourceNode {
	constructor(context: BaseAudioContext, options: ConstantSourceOptions = {}) {
		super(context, nativeContext(context).createConstantSource());
		if (typeof options?.offset === 'number') this.offset.value = options.offset;
	}

	get offset(): AudioParam {
		return this._param('offset');
	}
}

export class AnalyserNode extends AudioNode {
	constructor(context: BaseAudioContext, options: AnalyserOptions = {}) {
		super(context, nativeContext(context).createAnalyser());
		const o = options ?? {};
		if (typeof o.fftSize === 'number') this.fftSize = o.fftSize;
		if (typeof o.smoothingTimeConstant === 'number') this.smoothingTimeConstant = o.smoothingTimeConstant;
		// Both at once could pass through a min >= max state one at a time.
		if (typeof o.minDecibels === 'number' && o.minDecibels >= this.maxDecibels) {
			if (typeof o.maxDecibels === 'number') this.maxDecibels = o.maxDecibels;
			this.minDecibels = o.minDecibels;
		} else {
			if (typeof o.minDecibels === 'number') this.minDecibels = o.minDecibels;
			if (typeof o.maxDecibels === 'number') this.maxDecibels = o.maxDecibels;
		}
	}

	get fftSize(): number {
		return this.native.fftSize;
	}
	set fftSize(v: number) {
		this.native.fftSize = v | 0;
	}
	get frequencyBinCount(): number {
		return this.native.frequencyBinCount;
	}
	get smoothingTimeConstant(): number {
		return this.native.smoothingTimeConstant;
	}
	set smoothingTimeConstant(v: number) {
		this.native.smoothingTimeConstant = v;
	}
	get minDecibels(): number {
		return this.native.minDecibels;
	}
	set minDecibels(v: number) {
		this.native.minDecibels = v;
	}
	get maxDecibels(): number {
		return this.native.maxDecibels;
	}
	set maxDecibels(v: number) {
		this.native.maxDecibels = v;
	}

	getFloatTimeDomainData(dest: Float32Array) {
		if (!dest || dest.length === 0) return;
		this.native.getFloatTimeDomainData(dest);
	}
	getByteTimeDomainData(dest: Uint8Array) {
		if (!dest || dest.length === 0) return;
		this.native.getByteTimeDomainData(dest);
	}
	getFloatFrequencyData(dest: Float32Array) {
		if (!dest || dest.length === 0) return;
		this.native.getFloatFrequencyData(dest);
	}
	getByteFrequencyData(dest: Uint8Array) {
		if (!dest || dest.length === 0) return;
		this.native.getByteFrequencyData(dest);
	}
}

export class WaveShaperNode extends AudioNode {
	private _curve: Float32Array | null = null;

	constructor(context: BaseAudioContext, options: WaveShaperOptions = {}) {
		super(context, nativeContext(context).createWaveShaper());
		if (options?.curve) this.curve = toFloat32Array(options.curve);
		if (options?.oversample) this.oversample = options.oversample;
	}

	get curve(): Float32Array | null {
		return this._curve;
	}
	/** The engine takes one curve per node: a second one (or clearing it) throws. */
	set curve(v: Float32Array | null) {
		if (v == null) {
			if (this._curve) throw new Error('InvalidStateError: a WaveShaperNode curve cannot be cleared on Windows');
			return;
		}
		const curve = toFloat32Array(v);
		this.native.setCurve(curve);
		this._curve = curve;
	}
	get oversample(): 'none' | '2x' | '4x' {
		return this.native.oversample;
	}
	set oversample(v: 'none' | '2x' | '4x') {
		this.native.oversample = v;
	}
}

export class IIRFilterNode extends AudioNode {
	constructor(context: BaseAudioContext, options: IIRFilterOptions) {
		if (!options?.feedforward?.length || !options?.feedback?.length) throw new TypeError('IIRFilterNode: feedforward and feedback are required');
		super(context, nativeContext(context).createIirFilter(Array.from(options.feedforward), Array.from(options.feedback)));
	}

	getFrequencyResponse(frequencyHz: Float32Array, magResponse: Float32Array, phaseResponse: Float32Array) {
		if (!frequencyHz || !magResponse || !phaseResponse) return;
		this.native.getFrequencyResponse(frequencyHz, magResponse, phaseResponse);
	}
}

export class ConvolverNode extends AudioNode {
	private _buffer: AudioBuffer | null = null;

	constructor(context: BaseAudioContext, options: ConvolverOptions & { buffer?: AudioBuffer } = {}) {
		super(context, nativeContext(context).createConvolver());
		this.normalize = !options?.disableNormalization;
		if (options?.buffer) this.buffer = options.buffer;
	}

	get buffer(): AudioBuffer | null {
		return this._buffer;
	}
	set buffer(value: AudioBuffer | null) {
		if (!value) {
			this._buffer = null;
			return;
		}
		this.native.setBuffer(value.native);
		this._buffer = value;
	}
	get normalize(): boolean {
		return this.native.normalize;
	}
	set normalize(value: boolean) {
		this.native.normalize = !!value;
	}
}

export class DynamicsCompressorNode extends AudioNode {
	private _reduction: AudioParam | null = null;

	constructor(context: BaseAudioContext, options: DynamicsCompressorOptions = {}) {
		super(context, nativeContext(context).createDynamicsCompressor());
		const o = options ?? {};
		if (typeof o.threshold === 'number') this.threshold.value = o.threshold;
		if (typeof o.knee === 'number') this.knee.value = o.knee;
		if (typeof o.ratio === 'number') this.ratio.value = o.ratio;
		if (typeof o.attack === 'number') this.attack.value = o.attack;
		if (typeof o.release === 'number') this.release.value = o.release;
	}

	get threshold() {
		return this._param('threshold');
	}
	get knee() {
		return this._param('knee');
	}
	get ratio() {
		return this._param('ratio');
	}
	get attack() {
		return this._param('attack');
	}
	get release() {
		return this._param('release');
	}

	/** The current gain reduction in dB, read-only; an AudioParam as on iOS and Android. */
	get reduction(): AudioParam {
		if (!this._reduction) {
			const native = this.native;
			const readOnly = () => {};
			const reduction: NativeParam = {
				get value() {
					return native.reduction;
				},
				set value(_) {},
				defaultValue: 0,
				minValue: -Infinity,
				maxValue: 0,
				automationRate: 'k-rate',
				setValueAtTime: readOnly,
				linearRampToValueAtTime: readOnly,
				exponentialRampToValueAtTime: readOnly,
				setTargetAtTime: readOnly,
				setValueCurveAtTime: readOnly,
				cancelScheduledValues: readOnly,
				cancelAndHoldAtTime: readOnly,
			};
			this._reduction = new AudioParam(nativeCtor_, reduction);
		}
		return this._reduction;
	}
}

export class ChannelSplitterNode extends AudioNode {
	constructor(context: BaseAudioContext, options: ChannelSplitterOptions = {}) {
		super(context, nativeContext(context).createChannelSplitter(Math.max(1, options?.numberOfOutputs ?? 6)));
	}
}

export class ChannelMergerNode extends AudioNode {
	constructor(context: BaseAudioContext, options: ChannelMergerOptions = {}) {
		super(context, nativeContext(context).createChannelMerger(Math.max(1, options?.numberOfInputs ?? 6)));
	}
}

export class PeriodicWave {
	[native_]: unknown;

	constructor(context: BaseAudioContext, options: PeriodicWaveOptions = {}) {
		const real = options.real ? toFloat32Array(options.real) : null;
		const imag = options.imag ? toFloat32Array(options.imag) : null;
		const length = real?.length ?? imag?.length ?? 2;
		this[native_] = nativeContext(context).createPeriodicWave(real ?? new Float32Array(length), imag ?? new Float32Array(length), !!options.disableNormalization);
	}

	get native() {
		return this[native_];
	}
}

export class AudioListener extends AudioListenerBase {
	constructor(guard: Symbol, context: BaseAudioContext) {
		if (guard !== nativeCtor_) throw new TypeError('Illegal constructor.');
		super(context);
	}

	private _listenerParam(name: string): AudioParam {
		return new AudioParam(nativeCtor_, nativeContext(this[context_]).listenerParam(name));
	}

	get positionX() {
		return this._positionX || (this._positionX = this._listenerParam('positionX'));
	}
	get positionY() {
		return this._positionY || (this._positionY = this._listenerParam('positionY'));
	}
	get positionZ() {
		return this._positionZ || (this._positionZ = this._listenerParam('positionZ'));
	}

	get forwardX() {
		return this._forwardX || (this._forwardX = this._listenerParam('forwardX'));
	}
	get forwardY() {
		return this._forwardY || (this._forwardY = this._listenerParam('forwardY'));
	}
	get forwardZ() {
		return this._forwardZ || (this._forwardZ = this._listenerParam('forwardZ'));
	}

	get upX() {
		return this._upX || (this._upX = this._listenerParam('upX'));
	}
	get upY() {
		return this._upY || (this._upY = this._listenerParam('upY'));
	}
	get upZ() {
		return this._upZ || (this._upZ = this._listenerParam('upZ'));
	}
}

/** What AudioContext and OfflineAudioContext share: node factories, decoding, the listener. */
abstract class NativeAudioContextBase extends BaseAudioContext {
	protected [native_]: NativeContext;
	readonly destination: AudioDestinationNode;
	private _listener: AudioListener | null = null;

	protected constructor(native: NativeContext) {
		super();
		this[native_] = native;
		this._state = native.state;
		this.destination = new AudioDestinationNode(this, native.destination());
		// Weak, so an unreferenced context can still be collected (and its device released).
		const ref = new WeakRef(this);
		native.setOnstatechange(() => ref.deref()?._syncState());
	}

	get native() {
		return this[native_];
	}

	protected _syncState() {
		this._setState(this[native_].state);
	}

	get sampleRate(): number {
		return this[native_].sampleRate;
	}

	get currentTime(): number {
		return this[native_].currentTime;
	}

	get listener() {
		if (!this._listener) this._listener = new AudioListener(nativeCtor_, this);
		return this._listener;
	}

	createGain(options?: { gain?: number }) {
		return new GainNode(this, options ?? {});
	}
	createBiquadFilter(options?: { type?: string; frequency?: number; Q?: number; gain?: number }) {
		return new BiquadFilterNode(this, options ?? {});
	}
	createPanner(options?: PannerOptions) {
		return new PannerNode(this, options ?? {});
	}
	createOscillator(options?: { type?: string; frequency?: number }) {
		return new OscillatorNode(this, options ?? {});
	}
	createStereoPanner(options?: StereoPannerOptions) {
		return new StereoPannerNode(this, options ?? {});
	}
	createDelay(options?: DelayOptions | number) {
		return new DelayNode(this, typeof options === 'number' ? { maxDelayTime: options } : (options ?? {}));
	}
	createConstantSource(options?: ConstantSourceOptions) {
		return new ConstantSourceNode(this, options ?? {});
	}
	createAnalyser(options?: AnalyserOptions) {
		return new AnalyserNode(this, options ?? {});
	}
	createWaveShaper(options?: WaveShaperOptions) {
		return new WaveShaperNode(this, options ?? {});
	}
	createIIRFilter(feedforward: number[], feedback: number[]) {
		return new IIRFilterNode(this, { feedforward, feedback });
	}
	createConvolver(options?: ConvolverOptions) {
		return new ConvolverNode(this, options ?? {});
	}
	createDynamicsCompressor(options?: DynamicsCompressorOptions) {
		return new DynamicsCompressorNode(this, options ?? {});
	}
	createChannelSplitter(options?: ChannelSplitterOptions | number) {
		return new ChannelSplitterNode(this, typeof options === 'number' ? { numberOfOutputs: options } : (options ?? {}));
	}
	createChannelMerger(options?: ChannelMergerOptions | number) {
		return new ChannelMergerNode(this, typeof options === 'number' ? { numberOfInputs: options } : (options ?? {}));
	}
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }) {
		return new PeriodicWave(this, { real, imag, disableNormalization: options?.disableNormalization });
	}
	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number } | number, length?: number, sampleRate?: number) {
		if (typeof options === 'number') return new AudioBuffer({ numberOfChannels: options, length, sampleRate });
		return new AudioBuffer(options);
	}
	createBufferSource(options?: { buffer?: AudioBuffer }) {
		return new AudioBufferSourceNode(this, options ?? {});
	}

	/** Paths (`~/`, `file://`, absolute) are read natively; `data:` URLs and other strings are base64. */
	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
		const native = this[native_];
		let decoded: Promise<NativeBuffer>;
		try {
			if (typeof source === 'string') {
				// base64 has '/' too, so it is recognised before looksLikePath.
				const base64 = source.startsWith('data:') || /^[A-Za-z0-9+/\s]{16,}={0,2}$/.test(source) || !looksLikePath(source);
				decoded = base64 ? native.decodeAudioBase64(source) : native.decodeAudioFile(windowsPath(source));
			} else if (source instanceof ArrayBuffer) {
				decoded = native.decodeAudioData(new Uint8Array(source));
			} else if (ArrayBuffer.isView(source)) {
				decoded = native.decodeAudioData(new Uint8Array(source.buffer, source.byteOffset, source.byteLength));
			} else {
				throw new TypeError('decodeAudioData: source must be a path, base64 string, ArrayBuffer or view');
			}
		} catch (e) {
			decoded = Promise.reject(e);
		}
		const ret = decoded.then((buffer) => new AudioBuffer(nativeCtor_, buffer));
		if (successCallback) ret.then(successCallback);
		if (errorCallback) ret.catch(errorCallback);
		return ret;
	}
}

export class OfflineAudioContext extends NativeAudioContextBase {
	constructor(numberOfChannels: number | { numberOfChannels?: number; length: number; sampleRate: number }, lengthInFrames?: number, sampleRate?: number) {
		const options = typeof numberOfChannels === 'object' ? numberOfChannels : { numberOfChannels, length: lengthInFrames, sampleRate };
		super(Native.AudioContext.offline(Math.max(1, (options.numberOfChannels ?? 1) | 0), Math.max(0, options.length | 0), options.sampleRate && Number.isFinite(options.sampleRate) ? options.sampleRate : 48000));
	}

	get length(): number {
		return this[native_].length;
	}

	startRendering(): Promise<AudioBuffer> {
		let rendering: Promise<NativeBuffer>;
		try {
			rendering = this[native_].startRendering();
		} catch (e) {
			return Promise.reject(e);
		}
		return rendering.then((buffer) => {
			this._syncState();
			return new AudioBuffer(nativeCtor_, buffer);
		});
	}
}

export class AudioContext extends NativeAudioContextBase {
	constructor(options?: AudioContextOptions & { sinkId?: string }) {
		super(
			new Native.AudioContext({
				sampleRate: options?.sampleRate,
				latencyHint: options?.latencyHint,
				sinkId: options?.sinkId === 'default' ? '' : options?.sinkId,
			}),
		);
	}

	get baseLatency(): number {
		return this[native_].baseLatency;
	}

	get outputLatency(): number {
		return this[native_].outputLatency;
	}

	/** `''` for the default output device. */
	get sinkId(): string {
		return this[native_].sinkId;
	}

	setSinkId(deviceId: string): Promise<void> {
		const value = deviceId == null || deviceId === 'default' ? '' : String(deviceId);
		try {
			return this[native_].setSinkId(value);
		} catch (e) {
			return Promise.reject(e);
		}
	}

	resume(): Promise<void> {
		return this._changeState((native) => native.resume());
	}

	suspend(): Promise<void> {
		return this._changeState((native) => native.suspend());
	}

	close(): Promise<void> {
		if (this._state === 'closed') return Promise.resolve();
		return this._changeState((native) => native.close());
	}

	private _changeState(change: (native: NativeContext) => Promise<void>): Promise<void> {
		if (this._state === 'closed') return Promise.reject(new Error('InvalidStateError: AudioContext is closed'));
		let changed: Promise<void>;
		try {
			changed = change(this[native_]);
		} catch (e) {
			return Promise.reject(e);
		}
		// The state event arrives on its own schedule; `state` is current once the promise settles.
		return changed.then(() => this._syncState());
	}

	createMediaElementSource(mediaElement: MediaElementLike): MediaElementAudioSourceNode {
		return new MediaElementAudioSourceNode(this, mediaElement);
	}

	createSourceNodeFromPlayer(_playerNative: any): MediaElementAudioSourceNode | null {
		return null;
	}
}
