import { File, Utils, knownFolders } from '@nativescript/core';
import {
	AnalyserOptions,
	AudioBufferCopyOptions,
	AudioContextOptions,
	AudioNodeBase,
	AudioParamBase,
	AudioParamHooks,
	BaseAudioContext,
	ChannelMergerOptions,
	ChannelSplitterOptions,
	ConstantSourceOptions,
	ConvolverOptions,
	DelayOptions,
	distanceModelFromNumber,
	distanceModelToNumber,
	DistanceModelType,
	DynamicsCompressorOptions,
	IIRFilterOptions,
	MediaElementLike,
	panningModelFromNumber,
	panningModelToNumber,
	PanningModelType,
	PannerOptions,
	PeriodicWaveOptions,
	StereoPannerOptions,
	WaveShaperOptions,
	context_,
	isBase64Source,
	base64Payload,
	native_,
	nativeCtor_,
	normalizeSourcePath,
	resolveLatencyHint,
	resolvePannerOptions,
	AudioListenerBase,
	makeStubParamHooks,
	assertMediaElementUsable,
	markMediaElementUsed,
	unmarkMediaElementUsed,
	resolveNativePlayer,
	throwInvalidMediaElement,
	OfflineSuspends,
} from './common';
import { AUDIO_WORKLET_SCOPE } from './audio-worklet-scope';
import { SHARED_RING_BRIDGE, createWorkletRing, workletRingLayout } from './audio-worklet-ring';
import { AudioParamMap, AudioWorkletNodeOptions, AudioWorkletPort, WorkletHost, closeWithContext, domError, isWorkerSupported, loadWorkletModule, resolveWorkletNodeOptions } from './audio-worklet-host';
export { AudioWorkletNodeOptions };

type NativeBaseAudioContext = BaseAudioContext & { native: org.nativescript.audiocontext.AudioContextInstance };

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

function makeAndroidHooks(native: org.nativescript.audiocontext.AudioParam): AudioParamHooks {
	return {
		nativeSet(v) {
			native.setValue(v);
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
		nativeScheduleTarget(v, t, timeConstant) {
			native.setTargetAtTime(v, t, timeConstant);
		},
		nativeScheduleCurve(values, t, duration) {
			// A fresh array, so the buffer Java reads is exactly the curve.
			native.setValueCurveAtTime(Float32Array.from(values) as any, t, duration);
		},
		nativeCancel(t) {
			native.cancelScheduledValues(t);
		},
		nativeCancelAndHold(v, t) {
			native.cancelAndHoldAtTime(v, t);
		},
		nativeGetValue() {
			return native.getValue();
		},
		nativeSetAutomationRate(rate) {
			native.setAutomationRate(rate);
		},
	};
}
const FLT_MAX = 3.4028234663852886e38;
const MIN_RATE = 3000;
const MAX_RATE = 768000;

function checkSampleRate(what: string, rate: number) {
	if (!(rate >= MIN_RATE && rate <= MAX_RATE)) throw domError('NotSupportedError', `${what}: sampleRate ${rate} is outside ${MIN_RATE}..${MAX_RATE}`);
}

function checkChannels(what: string, count: number) {
	if (!(count >= 1 && count <= 32)) throw domError('NotSupportedError', `${what}: ${count} channels is outside 1..32`);
}

// WebIDL double: non-finite is a TypeError, then negative a RangeError.
function checkTime(what: string, value: number): number {
	if (!Number.isFinite(value)) throw new TypeError(`${what}: ${value} is not a finite number`);
	if (value < 0) throw new RangeError(`${what}: ${value} is negative`);
	return value;
}

export interface AudioParamMeta {
	defaultValue?: number;
	minValue?: number;
	maxValue?: number;
	fixedRate?: 'a-rate' | 'k-rate';
}

export class AudioParam extends AudioParamBase {
	private [native_]: org.nativescript.audiocontext.AudioParam;
	readonly defaultValue: number;
	readonly minValue: number;
	readonly maxValue: number;
	private _fixedRate?: string;

	constructor(a?: any, b?: any, meta: AudioParamMeta = {}) {
		if (a !== nativeCtor_ || !b) {
			throw new TypeError('Illegal constructor.');
		}
		super(makeAndroidHooks(b), b.getValue());
		this[native_] = b;
		this.defaultValue = meta.defaultValue ?? b.getValue();
		this.minValue = meta.minValue ?? -FLT_MAX;
		this.maxValue = meta.maxValue ?? FLT_MAX;
		this._fixedRate = meta.fixedRate;
	}

	get native() {
		return this[native_];
	}

	get automationRate(): string {
		return this._fixedRate ?? super.automationRate;
	}

	set automationRate(v: string) {
		if (v !== 'a-rate' && v !== 'k-rate') return;
		if (this._fixedRate && v !== this._fixedRate) throw domError('InvalidStateError', `AudioParam.automationRate is fixed at ${this._fixedRate}`);
		super.automationRate = v;
	}

	getValuesForRange(startTime: number, sampleRate: number, frameCount: number): number[] {
		const range = this.native.getValuesForRange(startTime, sampleRate, frameCount);
		if (!range) return [];
		return Utils.dataDeserialize(range);
	}
}

const COUNT_MODES = ['max', 'clamped-max', 'explicit'];
const INTERPRETATIONS = ['speakers', 'discrete'];

export class AudioNode extends AudioNodeBase {
	private _channelCount?: number;
	private _channelCountMode?: string;
	private _channelInterpretation?: string;

	constructor(context: NativeBaseAudioContext) {
		super(context);
	}

	private get _graphNative(): org.nativescript.audiocontext.AudioNode {
		return (this as never as { native: org.nativescript.audiocontext.AudioNode }).native;
	}

	// The Web Audio defaults for this node's kind.
	private _channelDefaults(): [number, string, string] {
		if (this instanceof ChannelSplitterNode) return [this.numberOfOutputs, 'explicit', 'discrete'];
		if (this instanceof ChannelMergerNode) return [1, 'explicit', 'speakers'];
		if (this instanceof AudioDestinationNode) return [this.context instanceof OfflineAudioContext ? (this.context as any)._channels : 2, 'explicit', 'speakers'];
		if (this instanceof PannerNode || this instanceof StereoPannerNode || this instanceof ConvolverNode || this instanceof DynamicsCompressorNode) return [2, 'clamped-max', 'speakers'];
		return [2, 'max', 'speakers'];
	}

	protected get _contextRate(): number {
		return (this.context as any).sampleRate;
	}

	get numberOfInputs(): number {
		return 1;
	}

	get numberOfOutputs(): number {
		return 1;
	}

	// Splitter, merger and offline destination have a fixed channel config.
	private _fixedConfig(what: string, ok: boolean, destination = false) {
		if (!ok && (this instanceof ChannelSplitterNode || this instanceof ChannelMergerNode || (destination && this instanceof AudioDestinationNode && this.context instanceof OfflineAudioContext))) {
			throw domError('InvalidStateError', `${this.constructor.name}.${what} cannot be changed`);
		}
	}

	private get _stereoOnly(): any {
		return this instanceof PannerNode || this instanceof StereoPannerNode || this instanceof ConvolverNode || this instanceof DynamicsCompressorNode;
	}

	get channelCount(): number {
		return this._channelCount ?? this._channelDefaults()[0];
	}

	set channelCount(value: number) {
		const count = Math.floor(value);
		if (!(count >= 1 && count <= 32)) throw domError('NotSupportedError', `AudioNode.channelCount: ${value} is out of range`);
		this._fixedConfig('channelCount', count === this._channelDefaults()[0], true);
		if (this._stereoOnly && count > 2) throw domError('NotSupportedError', `${this.constructor.name}.channelCount: ${count} is more than 2`);
		if (this instanceof AudioDestinationNode && count > this.maxChannelCount) throw domError('IndexSizeError', `AudioDestinationNode.channelCount: ${count} is more than maxChannelCount`);
		this._channelCount = count;
		this._graphNative?.setChannelConfig(count, -1, -1);
	}

	get channelCountMode(): string {
		return this._channelCountMode ?? this._channelDefaults()[1];
	}

	set channelCountMode(value: string) {
		const mode = COUNT_MODES.indexOf(value);
		if (mode < 0) return;
		this._fixedConfig('channelCountMode', value === 'explicit');
		if (this._stereoOnly && value === 'max') throw domError('NotSupportedError', `${this.constructor.name}.channelCountMode cannot be max`);
		this._channelCountMode = value;
		this._graphNative?.setChannelConfig(-1, mode, -1);
	}

	get channelInterpretation(): string {
		return this._channelInterpretation ?? this._channelDefaults()[2];
	}

	set channelInterpretation(value: string) {
		const interpretation = INTERPRETATIONS.indexOf(value);
		if (interpretation < 0) return;
		if (this instanceof ChannelSplitterNode && value !== 'discrete') throw domError('InvalidStateError', 'ChannelSplitterNode.channelInterpretation cannot be changed');
		this._channelInterpretation = value;
		this._graphNative?.setChannelConfig(-1, -1, interpretation);
	}

	connect(node: any, output?: number, input?: number) {
		if (node instanceof AudioWorkletNode && ((input ?? 0) < 0 || (input ?? 0) >= node.numberOfInputs)) {
			throw domError('IndexSizeError', `AudioNode.connect: input ${input ?? 0} is out of range`);
		}
		const native = this._graphNative;
		const target = node?.native || node;
		try {
			if (node instanceof AudioParamBase) native.connect(target as org.nativescript.audiocontext.AudioParam, output ?? 0);
			else if (typeof output === 'number' && typeof input === 'number') native.connect(target, output, input);
			else if (typeof output === 'number') native.connect(target, output);
			else native.connect(target);
		} catch (e) {
			console.warn('AudioNode.connect failed:', e);
		}

		if (!(node instanceof AudioParamBase)) return node;
	}

	disconnect(destinationOrOutput?: any, output?: number, input?: number) {
		const native = this._graphNative;
		if (destinationOrOutput === undefined || destinationOrOutput === null) {
			native.disconnect();
			return;
		}
		if (typeof destinationOrOutput === 'number') {
			native.disconnect(destinationOrOutput);
			return;
		}
		const target = destinationOrOutput.native ?? destinationOrOutput;
		if (destinationOrOutput instanceof AudioParamBase) {
			if (typeof output === 'number') native.disconnect(target as org.nativescript.audiocontext.AudioParam, output);
			else native.disconnect(target as org.nativescript.audiocontext.AudioParam);
			return;
		}
		if (typeof output !== 'number') native.disconnect(target);
		else if (typeof input !== 'number') native.disconnect(target, output);
		else native.disconnect(target, output, input);
	}
}

export class AudioBuffer {
	[native_]: org.nativescript.audiocontext.AudioBuffer;

	constructor(options: { length: number; numberOfChannels?: number; sampleRate?: number } | Symbol, nativeBuffer?: org.nativescript.audiocontext.AudioBuffer) {
		if (!options) throw new TypeError('AudioBuffer constructor requires options');
		if (options === nativeCtor_ && nativeBuffer) {
			this[native_] = nativeBuffer;
			return;
		}

		const opts = options as { length: number; numberOfChannels?: number; sampleRate?: number };
		const length = Math.floor(opts.length);
		const ch = opts.numberOfChannels != null ? Math.floor(opts.numberOfChannels) : 1;
		const sr = opts.sampleRate ?? 48000;
		if (!(length >= 1)) throw domError('NotSupportedError', `AudioBuffer: length ${opts.length} must be at least 1`);
		checkChannels('AudioBuffer', ch);
		checkSampleRate('AudioBuffer', sr);
		this._sampleRate = sr;
		this[native_] = new org.nativescript.audiocontext.AudioBuffer(length, ch, Math.round(sr));
	}

	// The native buffer keeps an integer rate.
	private _sampleRate?: number;

	get native() {
		return this[native_];
	}

	get sampleRate(): number {
		return this._sampleRate ?? this.native.getSampleRate();
	}
	get length(): number {
		return this.native.getLength();
	}
	get duration(): number {
		return this._sampleRate ? this.length / this._sampleRate : this.native.getDuration();
	}
	get numberOfChannels(): number {
		return this.native.getNumberOfChannels();
	}

	private _checkChannel(what: string, channel: number) {
		if (!(channel >= 0 && channel < this.numberOfChannels)) throw domError('IndexSizeError', `AudioBuffer.${what}: channel ${channel} is out of range`);
	}

	getChannelData(channel: number): Float32Array | null {
		this._checkChannel('getChannelData', channel);
		const data = this.native.getChannelDataRaw(channel);
		if (!data) return null;
		return new Float32Array((ArrayBuffer as any).from(data));
	}

	copyFromChannel(dest: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!dest) return;
		this._checkChannel('copyFromChannel', channel);
		const options = resolveAudioBufferCopyOptions(dest, startInChannel);
		const target = viewAtByteOffset(dest, options.byteOffset);
		if (target.length === 0) return;
		this.native.copyFromChannel(target as any, channel, options.startInChannel);
	}

	copyToChannel(source: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!source) return;
		this._checkChannel('copyToChannel', channel);
		const options = resolveAudioBufferCopyOptions(source, startInChannel);
		const input = viewAtByteOffset(source, options.byteOffset);
		if (input.length === 0) return;
		this.native.copyToChannel(input, channel, options.startInChannel);
	}
}

export class GainNode extends AudioNode {
	private _gainParam: AudioParam | null = null;
	[native_]: org.nativescript.audiocontext.GainNode;
	constructor(context: NativeBaseAudioContext, options: { gain?: number } = {}) {
		super(context);
		const gain = AudioContext.getInstance().createGain(context.native);
		if (typeof options?.gain === 'number' && options.gain !== 1.0) {
			gain.setValue(options.gain);
		}
		this[native_] = gain;
	}

	get native() {
		return this[native_];
	}

	get gain(): AudioParam {
		if (!this._gainParam) this._gainParam = new AudioParam(nativeCtor_, this.native.getGain(), { defaultValue: 1 });
		return this._gainParam;
	}
}

export class AudioDestinationNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.GainNode;
	constructor(context: NativeBaseAudioContext, node: org.nativescript.audiocontext.GainNode) {
		super(context);
		this[native_] = node;
	}
	get native() {
		return this[native_];
	}
	get maxChannelCount(): number {
		return this.context instanceof OfflineAudioContext ? (this.context as any)._channels : 2;
	}
	get numberOfOutputs(): number {
		return 0;
	}
}

export class BiquadFilterNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AudioBiquadNode;
	private _frequencyParam: AudioParam | null = null;
	private _qParam: AudioParam | null = null;
	private _gainParam: AudioParam | null = null;
	private _detuneParam: AudioParam | null = null;

	constructor(context: NativeBaseAudioContext, options: { type?: string; frequency?: number; Q?: number; gain?: number } = {}) {
		super(context);
		const initialFreq = typeof options.frequency === 'number' ? options.frequency : 350;
		const initialQ = typeof options.Q === 'number' ? options.Q : 1.0;
		const initialGain = typeof options.gain === 'number' ? options.gain : 0.0;
		this[native_] = AudioContext.getInstance().createBiquad(context.native, options.type ?? 'lowpass', initialFreq, initialQ, initialGain);
	}

	get native() {
		return this[native_];
	}
	get type(): string {
		return this.native.getType();
	}
	set type(value: string) {
		if (!BIQUAD_TYPES.includes(value)) return;
		this.native.setType(value);
	}
	get frequency(): AudioParam {
		if (!this._frequencyParam) this._frequencyParam = new AudioParam(nativeCtor_, this.native.getFrequency(), { defaultValue: 350, minValue: 0, maxValue: this._contextRate / 2 });
		return this._frequencyParam;
	}
	get Q(): AudioParam {
		if (!this._qParam) this._qParam = new AudioParam(nativeCtor_, this.native.getQ(), { defaultValue: 1 });
		return this._qParam;
	}
	get gain(): AudioParam {
		if (!this._gainParam) this._gainParam = new AudioParam(nativeCtor_, this.native.getGain(), { defaultValue: 0, maxValue: 40 * Math.log10(FLT_MAX) });
		return this._gainParam;
	}
	get detune(): AudioParam {
		if (!this._detuneParam) this._detuneParam = new AudioParam(nativeCtor_, this.native.getDetune(), { defaultValue: 0, minValue: -1200 * Math.log2(FLT_MAX), maxValue: 1200 * Math.log2(FLT_MAX) });
		return this._detuneParam;
	}

	getFrequencyResponse(frequencyHz: Float32Array, magResponse: Float32Array, phaseResponse: Float32Array) {
		if (frequencyHz.length !== magResponse.length || frequencyHz.length !== phaseResponse.length) throw domError('InvalidAccessError', 'BiquadFilterNode.getFrequencyResponse: array lengths differ');
		const nyquist = this._contextRate / 2;
		const [b0, b1, b2, a1, a2] = biquadCoeffs(this.type, this.frequency.value, this.Q.value, this.gain.value, this.detune.value, nyquist);
		for (let i = 0; i < frequencyHz.length; i++) {
			const f = frequencyHz[i];
			if (!(f >= 0 && f <= nyquist)) {
				magResponse[i] = phaseResponse[i] = NaN;
				continue;
			}
			// H(z) at z = e^{jw}: (b0 + b1 z^-1 + b2 z^-2) / (1 + a1 z^-1 + a2 z^-2).
			const w = (Math.PI * f) / nyquist;
			const [c1, s1, c2, s2] = [Math.cos(w), Math.sin(w), Math.cos(2 * w), Math.sin(2 * w)];
			const [nr, ni] = [b0 + b1 * c1 + b2 * c2, -(b1 * s1 + b2 * s2)];
			const [dr, di] = [1 + a1 * c1 + a2 * c2, -(a1 * s1 + a2 * s2)];
			const d = dr * dr + di * di;
			const [hr, hi] = [(nr * dr + ni * di) / d, (ni * dr - nr * di) / d];
			magResponse[i] = Math.hypot(hr, hi);
			phaseResponse[i] = Math.atan2(hi, hr);
		}
	}
}

const BIQUAD_TYPES = ['lowpass', 'highpass', 'bandpass', 'lowshelf', 'highshelf', 'peaking', 'notch', 'allpass'];

// Mirrors biquadCoeffs in audio_graph.cpp: [b0, b1, b2, a1, a2], normalised by a0.
function biquadCoeffs(type: string, frequency: number, Q: number, gain: number, detune: number, nyquist: number): number[] {
	const set = (b0: number, b1: number, b2: number, a0: number, a1: number, a2: number) => [b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0];
	const scale = (k: number) => [k, 0, 0, 0, 0];
	const f = Math.max(0, Math.min(frequency * Math.pow(2, detune / 1200), nyquist)) / nyquist;
	const w0 = Math.PI * f;
	const cs = Math.cos(w0);
	const sn = Math.sin(w0);
	const A = Math.pow(10, gain / 40);
	const inside = f > 0 && f < 1;
	switch (type) {
		case 'lowpass':
		case 'highpass': {
			const low = type === 'lowpass';
			if (f >= 1) return scale(low ? 1 : 0);
			if (f <= 0) return scale(low ? 0 : 1);
			const alpha = sn / (2 * Math.pow(10, Q / 20));
			return low ? set((1 - cs) / 2, 1 - cs, (1 - cs) / 2, 1 + alpha, -2 * cs, 1 - alpha) : set((1 + cs) / 2, -(1 + cs), (1 + cs) / 2, 1 + alpha, -2 * cs, 1 - alpha);
		}
		case 'bandpass':
			if (!inside) return scale(0);
			if (Q <= 0) return scale(1);
			return ((alpha) => set(alpha, 0, -alpha, 1 + alpha, -2 * cs, 1 - alpha))(sn / (2 * Q));
		case 'notch':
			if (!inside) return scale(1);
			if (Q <= 0) return scale(0);
			return ((alpha) => set(1, -2 * cs, 1, 1 + alpha, -2 * cs, 1 - alpha))(sn / (2 * Q));
		case 'peaking':
			if (!inside) return scale(1);
			if (Q <= 0) return scale(A * A);
			return ((alpha) => set(1 + alpha * A, -2 * cs, 1 - alpha * A, 1 + alpha / A, -2 * cs, 1 - alpha / A))(sn / (2 * Q));
		case 'allpass':
			if (!inside) return scale(1);
			if (Q <= 0) return scale(-1);
			return ((alpha) => set(1 - alpha, -2 * cs, 1 + alpha, 1 + alpha, -2 * cs, 1 - alpha))(sn / (2 * Q));
		case 'lowshelf':
		case 'highshelf': {
			const low = type === 'lowshelf';
			if (f >= 1) return scale(low ? A * A : 1);
			if (f <= 0) return scale(low ? 1 : A * A);
			const k = Math.sqrt(A) * sn * Math.SQRT2;
			if (low) return set(A * (A + 1 - (A - 1) * cs + k), 2 * A * (A - 1 - (A + 1) * cs), A * (A + 1 - (A - 1) * cs - k), A + 1 + (A - 1) * cs + k, -2 * (A - 1 + (A + 1) * cs), A + 1 + (A - 1) * cs - k);
			return set(A * (A + 1 + (A - 1) * cs + k), -2 * A * (A - 1 + (A + 1) * cs), A * (A + 1 + (A - 1) * cs - k), A + 1 - (A - 1) * cs + k, 2 * (A - 1 - (A + 1) * cs), A + 1 - (A - 1) * cs - k);
		}
	}
	return scale(1);
}

export class PannerNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AudioPannerNode;

	constructor(context: NativeBaseAudioContext, options: PannerOptions = {}) {
		super(context);
		const p = resolvePannerOptions(options);
		checkPanner('refDistance', p.refDistance);
		checkPanner('maxDistance', p.maxDistance);
		checkPanner('rolloffFactor', p.rolloffFactor);
		checkPanner('coneOuterGain', p.coneOuterGain);
		const node = AudioContext.getInstance().createPanner(context.native);
		this[native_] = node;
		AudioContext.getInstance().setPannerParams(node, p.positionX, p.positionY, p.positionZ, p.orientationX, p.orientationY, p.orientationZ, p.pan, p.distanceModel, p.panningModel, p.refDistance, p.maxDistance, p.rolloffFactor, p.coneInnerAngle, p.coneOuterAngle, p.coneOuterGain);
	}

	get native() {
		return this[native_];
	}

	private _positionX!: AudioParam;
	private _positionY!: AudioParam;
	private _positionZ!: AudioParam;
	private _orientationX!: AudioParam;
	private _orientationY!: AudioParam;
	private _orientationZ!: AudioParam;

	get positionX() {
		return this._positionX || (this._positionX = new AudioParam(nativeCtor_, this.native.getPositionXParam(), { defaultValue: 0 }));
	}
	get positionY() {
		return this._positionY || (this._positionY = new AudioParam(nativeCtor_, this.native.getPositionYParam(), { defaultValue: 0 }));
	}
	get positionZ() {
		return this._positionZ || (this._positionZ = new AudioParam(nativeCtor_, this.native.getPositionZParam(), { defaultValue: 0 }));
	}
	get orientationX() {
		return this._orientationX || (this._orientationX = new AudioParam(nativeCtor_, this.native.getOrientationXParam(), { defaultValue: 1 }));
	}
	get orientationY() {
		return this._orientationY || (this._orientationY = new AudioParam(nativeCtor_, this.native.getOrientationYParam(), { defaultValue: 0 }));
	}
	get orientationZ() {
		return this._orientationZ || (this._orientationZ = new AudioParam(nativeCtor_, this.native.getOrientationZParam(), { defaultValue: 0 }));
	}

	get distanceModel(): DistanceModelType {
		return distanceModelFromNumber(this.native.getDistanceModel());
	}
	set distanceModel(v: DistanceModelType | number) {
		this.native.setDistanceModel(distanceModelToNumber(v));
	}
	get panningModel(): PanningModelType {
		return panningModelFromNumber(this.native.getPanningModel());
	}
	set panningModel(v: PanningModelType | number) {
		this.native.setPanningModel(panningModelToNumber(v));
	}
	get refDistance() {
		return this.native.getRefDistance();
	}
	set refDistance(v: number) {
		checkPanner('refDistance', v);
		this.native.setRefDistance(v);
	}
	get maxDistance() {
		return this.native.getMaxDistance();
	}
	set maxDistance(v: number) {
		checkPanner('maxDistance', v);
		this.native.setMaxDistance(v);
	}
	get rolloffFactor() {
		return this.native.getRolloffFactor();
	}
	set rolloffFactor(v: number) {
		checkPanner('rolloffFactor', v);
		this.native.setRolloffFactor(v);
	}
	get coneInnerAngle() {
		return this.native.getConeInnerAngle();
	}
	set coneInnerAngle(v: number) {
		this.native.setConeInnerAngle(v);
	}
	get coneOuterAngle() {
		return this.native.getConeOuterAngle();
	}
	set coneOuterAngle(v: number) {
		this.native.setConeOuterAngle(v);
	}
	get coneOuterGain() {
		return this.native.getConeOuterGain();
	}
	set coneOuterGain(v: number) {
		checkPanner('coneOuterGain', v);
		this.native.setConeOuterGain(v);
	}

	setPosition(x: number, y: number, z: number) {
		this.native.setPosition(x, y, z);
	}
	setOrientation(x: number, y: number, z: number) {
		this.native.setOrientation(x, y, z);
	}
}

function checkPanner(name: string, v: number) {
	if (name === 'coneOuterGain') {
		if (!(v >= 0 && v <= 1)) throw domError('InvalidStateError', `PannerNode.coneOuterGain: ${v} is outside [0, 1]`);
	} else if (name === 'maxDistance' ? !(v > 0) : !(v >= 0)) {
		throw new RangeError(`PannerNode.${name}: ${v} is out of range`);
	}
}

export class AudioScheduledSourceNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AudioScheduledSourceNode;
	private _onended: ((ev: { type: 'ended' }) => void) | null = null;
	private _nativeEndedWired = false;
	private _javaEndedListener: any = null;
	private _endedFired = false;

	constructor(context: NativeBaseAudioContext, node: org.nativescript.audiocontext.AudioScheduledSourceNode) {
		super(context);
		if (!(node instanceof org.nativescript.audiocontext.AudioScheduledSourceNode)) throw new TypeError('Illegal constructor.');
		this[native_] = node;
	}

	get native() {
		return this[native_];
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

	get numberOfInputs(): number {
		return 0;
	}

	private _ensureNativeEndedWired() {
		if (this._nativeEndedWired) return;
		this._nativeEndedWired = true;
		try {
			this._javaEndedListener = new org.nativescript.audiocontext.AudioContext.EndedListener({
				onEnded: () => this._fireEnded(),
			});
			this.native.addEndedListener(this._javaEndedListener);
		} catch (e) {
			console.warn('AudioScheduledSourceNode: native ended listener wire failed:', e);
		}
	}

	// The native id changes with the buffer; ended listeners are keyed by it.
	protected _rewireEnded(swap: () => void) {
		const listener = this._javaEndedListener;
		if (listener) this.native.removeEndedListener(listener);
		try {
			swap();
		} finally {
			if (listener) this.native.addEndedListener(listener);
		}
	}

	// Native stop() fires ended synchronously; the event is queued as a task.
	protected _fireEnded() {
		if (!this._started || this._endedFired) return;
		this._endedFired = true;
		setTimeout(() => this.dispatchEvent({ type: 'ended', target: this }));
	}

	private _started = false;

	start(when = 0, offset?: number, duration?: number) {
		if (this._started) throw domError('InvalidStateError', `${this.constructor.name}.start: already started`);
		when = checkTime('start when', when);
		if (offset !== undefined) offset = checkTime('start offset', offset);
		if (duration !== undefined) duration = checkTime('start duration', duration);
		this._started = true;
		const now = (this.context as any)?.currentTime ?? 0;
		this._startNative(when > now ? when : 0, offset ?? 0);
		if (duration !== undefined) this.stop(Math.max(now, when) + duration);
	}

	protected _startNative(when: number, _offset: number) {
		if (when > 0) {
			this.native.startAt(when);
		} else {
			this.native.start();
		}
	}

	stop(when = 0) {
		if (!this._started) throw domError('InvalidStateError', `${this.constructor.name}.stop: not started`);
		when = checkTime('stop when', when);
		const now = (this.context as any)?.currentTime ?? 0;
		if (when > now) {
			this.native.stopAt(when);
		} else {
			this.native.stop();
		}
	}
}

export class AudioBufferSourceNode extends AudioScheduledSourceNode {
	private _playbackRateParam: AudioParam | null = null;
	private _buffer: AudioBuffer | null = null;
	private _bufferSet = false;
	constructor(context: NativeBaseAudioContext, options: { buffer?: AudioBuffer | null } = {}) {
		super(context, AudioContext.getInstance().createBufferSource(context.native, options?.buffer?.native ?? (null as never)));
		this._buffer = options?.buffer ?? null;
		this._bufferSet = !!this._buffer;
	}
	get native() {
		return this[native_] as org.nativescript.audiocontext.AudioBufferSourceNode;
	}
	protected _startNative(when: number, offset: number) {
		this.native.startAt(when, offset);
	}
	get loop(): boolean {
		return this.native.getLoop();
	}
	set loop(v: boolean) {
		this.native.setLoop(v);
	}
	get loopStart(): number {
		return this.native.getLoopStart();
	}
	set loopStart(v: number) {
		this.native.setLoopStart(v);
	}
	get loopEnd(): number {
		return this.native.getLoopEnd();
	}
	set loopEnd(v: number) {
		this.native.setLoopEnd(v);
	}
	private _detuneParam: AudioParam | null = null;
	get detune(): AudioParam {
		if (!this._detuneParam) this._detuneParam = new AudioParam(nativeCtor_, this.native.getDetune(), { defaultValue: 0, fixedRate: 'k-rate' });
		return this._detuneParam;
	}
	get buffer(): AudioBuffer | null {
		if (this._buffer) return this._buffer;
		const b = this.native.getBuffer();
		return b ? new AudioBuffer(nativeCtor_, b) : null;
	}
	set buffer(v: AudioBuffer | null) {
		if (v && this._bufferSet) throw domError('InvalidStateError', 'AudioBufferSourceNode.buffer can only be set once');
		this._rewireEnded(() => this.native.setBuffer(v?.native ?? (null as never)));
		this._buffer = v ?? null;
		if (v) this._bufferSet = true;
	}

	get playbackRate(): AudioParam | null {
		if (this._playbackRateParam) return this._playbackRateParam;
		const param = this.native.getPlaybackRateParam();
		if (param) this._playbackRateParam = new AudioParam(nativeCtor_, param, { defaultValue: 1, fixedRate: 'k-rate' });
		return this._playbackRateParam;
	}
}

export class MediaElementAudioSourceNode extends AudioNode {
	private [native_]: org.nativescript.audiocontext.ExternalPcmSourceNode;
	private _mediaElement: MediaElementLike;
	private _playbackRateParam: AudioParam | null = null;

	constructor(context: AudioContext, mediaElement: MediaElementLike, preExistingNative?: org.nativescript.audiocontext.ExternalPcmSourceNode) {
		super(context as unknown as NativeBaseAudioContext);
		assertMediaElementUsable(context, mediaElement);

		const native = preExistingNative ?? MediaElementAudioSourceNode._createNative(context, mediaElement);
		if (!native) throwInvalidMediaElement();

		this._mediaElement = mediaElement;
		this[native_] = native;
		markMediaElementUsed(mediaElement);
	}

	private static _createNative(context: AudioContext, mediaElement: MediaElementLike): org.nativescript.audiocontext.ExternalPcmSourceNode | null {
		const ctxNative: any = context.native;
		if (!ctxNative) return null;
		const player = resolveNativePlayer(mediaElement);
		if (player && typeof ctxNative.createSourceNodeFromMediaPlayer === 'function') {
			try {
				const node = ctxNative.createSourceNodeFromMediaPlayer(player);
				if (node) return node;
			} catch (e) {}
		}
		if (typeof mediaElement.attachAudioContextTap === 'function') {
			try {
				const node = mediaElement.attachAudioContextTap(ctxNative);
				if (node) return node;
			} catch (e) {}
		}
		return null;
	}

	get context() {
		return this[context_] as AudioContext;
	}

	get native() {
		return this[native_];
	}

	get mediaElement(): MediaElementLike {
		return this._mediaElement;
	}

	get numberOfInputs(): number {
		return 0;
	}

	get playbackRate(): AudioParam | null {
		if (this._playbackRateParam) return this._playbackRateParam;
		const native: any = this[native_];
		try {
			const param = typeof native?.getPlaybackRateParam === 'function' ? native.getPlaybackRateParam() : null;
			if (param) {
				this._playbackRateParam = new AudioParam(nativeCtor_, param, { defaultValue: 1 });
				return this._playbackRateParam;
			}
		} catch (e) {}
		return null;
	}

	disposeMediaElementSource() {
		const native = this[native_];
		const el = this._mediaElement;
		try {
			const ctxNative: any = (this.context as any)?.native;
			if (native && ctxNative && typeof ctxNative.detachSource === 'function') {
				ctxNative.detachSource(native);
			} else if (typeof el?.detachAudioContextTap === 'function') {
				el.detachAudioContextTap();
			}
		} catch (e) {}
		this._playbackRateParam = null;
		this[native_] = null as never;
		unmarkMediaElementUsed(el);
	}
}

const QUANTUM = 128;

function createAudioBuffer(numberOfChannels: number | { length: number; numberOfChannels?: number; sampleRate: number }, length?: number, sampleRate?: number) {
	return new AudioBuffer(typeof numberOfChannels === 'object' ? numberOfChannels : { numberOfChannels, length: length ?? 0, sampleRate });
}

function decodeAudioData(ctx: org.nativescript.audiocontext.AudioContextInstance, source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
	const ret = new Promise<AudioBuffer>((resolve, reject) => {
		const callback = new org.nativescript.audiocontext.DecodeCallback({
			onResult(buffer) {
				if (!buffer) return reject(domError('EncodingError', 'decodeAudioData: unable to decode the audio data'));
				resolve(new AudioBuffer(nativeCtor_, buffer));
			},
		});
		const engine = AudioContext.getInstance();
		if (typeof source !== 'string') engine.decodeAudioDataFromBufferAsync(source as never, ctx, callback);
		else if (isBase64Source(source)) engine.decodeAudioDataAsync(base64Payload(source), ctx, callback);
		else engine.decodeAudioDataFromFileAsync(normalizeSourcePath(source), ctx, callback);
	});
	// The callback chain never rejects; the caller handles the returned promise.
	if (successCallback || errorCallback) ret.then(successCallback, (e) => errorCallback?.(e));
	return ret;
}

export class OfflineAudioContext extends BaseAudioContext {
	[native_]: org.nativescript.audiocontext.AudioContextInstance;
	destination: AudioDestinationNode | null = null;

	private _length: number;
	private _sampleRate: number;
	private _channels: number;
	private _renderer: org.nativescript.audiocontext.OfflineRenderer;
	private _rendering: Promise<AudioBuffer> | null = null;
	private _suspends = new OfflineSuspends();
	private _listener: AudioListener | null = null;
	private _audioWorklet: AudioWorklet | null = null;

	get audioWorklet(): AudioWorklet | undefined {
		if (!this._audioWorklet && isWorkerSupported()) this._audioWorklet = new AudioWorklet(nativeCtor_, this);
		return this._audioWorklet ?? undefined;
	}

	static getInstance() {
		if (!audioContextInstance) audioContextInstance = org.nativescript.audiocontext.AudioContext.getInstance();
		return audioContextInstance;
	}

	constructor(numberOfChannels: number | { numberOfChannels?: number; length: number; sampleRate: number }, lengthInFrames?: number, sampleRate?: number) {
		super();
		const options = typeof numberOfChannels === 'object' ? numberOfChannels : { numberOfChannels, length: lengthInFrames, sampleRate };
		const channels = Math.floor(options.numberOfChannels ?? 1);
		const length = Math.floor(options?.length ?? 0);
		const rate = options.sampleRate ?? 48000;
		checkChannels('OfflineAudioContext', channels);
		if (!(length >= 1)) throw domError('NotSupportedError', `OfflineAudioContext: length ${options.length} must be at least 1`);
		checkSampleRate('OfflineAudioContext', rate);
		this._channels = channels;
		this._length = length;
		this._sampleRate = rate;
		OfflineAudioContext.getInstance();
		this[native_] = org.nativescript.audiocontext.AudioContextInstance.createOffline(this._sampleRate, this._channels);
		this._renderer = this[native_].createRenderer(this._length, this._channels, this._sampleRate);
		this.destination = new AudioDestinationNode(this, this[native_].getDestination());
		this._state = 'suspended';
	}

	get native() {
		return this[native_];
	}

	get sampleRate(): number {
		return this._sampleRate;
	}

	get length(): number {
		return this._length;
	}

	private _oncomplete: ((ev: { type: 'complete'; renderedBuffer: AudioBuffer }) => void) | null = null;
	get oncomplete() {
		return this._oncomplete;
	}
	set oncomplete(cb: ((ev: { type: 'complete'; renderedBuffer: AudioBuffer }) => void) | null) {
		if (this._oncomplete) this.removeEventListener('complete', this._oncomplete);
		this._oncomplete = typeof cb === 'function' ? cb : null;
		if (this._oncomplete) this.addEventListener('complete', this._oncomplete);
	}

	get currentTime(): number {
		return this[native_].getCurrentTime();
	}

	get listener() {
		if (!this._listener) this._listener = new AudioListener(nativeCtor_, this as unknown as NativeBaseAudioContext);
		return this._listener;
	}

	/** Holds rendering at `suspendTime`, rounded up to a 128-frame quantum. */
	suspend(suspendTime: number): Promise<void> {
		const fail = (reason: string) => Promise.reject(domError('InvalidStateError', `OfflineAudioContext.suspend: ${reason}`));
		if (this._state === 'closed') return fail('rendering has finished');
		const time = Number(suspendTime);
		if (!Number.isFinite(time) || time < 0) return fail('suspendTime must be a non-negative number');
		// The epsilon keeps an exact boundary from rounding up a quantum.
		const frame = Math.max(0, Math.ceil((time * this._sampleRate) / QUANTUM - 1e-9)) * QUANTUM;
		if (frame >= this._length) return fail('suspendTime is at or past the end of the render');
		if (this._suspends.has(frame)) return fail('a suspend is already scheduled at that time');
		if (!this._renderer.suspendAt(frame)) return fail('suspendTime is not after the current time');
		return this._suspends.add(frame);
	}

	resume(): Promise<void> {
		if (this._state === 'closed') return Promise.reject(domError('InvalidStateError', 'OfflineAudioContext: rendering has finished'));
		if (!this._rendering) return Promise.reject(domError('InvalidStateError', 'OfflineAudioContext: rendering has not started'));
		this._setState('running');
		this._renderer.resume();
		return Promise.resolve();
	}

	startRendering(): Promise<AudioBuffer> {
		if (this._rendering) return Promise.reject(domError('InvalidStateError', 'OfflineAudioContext: rendering has already started'));
		this._setState('running');
		const ref = new WeakRef(this);
		this._rendering = new Promise<AudioBuffer>((resolve, reject) => {
			this._renderer.start(
				new org.nativescript.audiocontext.OfflineRenderer.Callback({
					onSuspend(frame: number) {
						const context = ref.deref();
						if (!context) return;
						context._setState('suspended');
						context._suspends.reached(frame);
					},
					onComplete(buffer) {
						const context = ref.deref();
						if (context) {
							context._audioWorklet?._scope(false)?.close();
							connectedWorkletNodes.forEach((node) => {
								if (node.context === context) node._stop();
							});
							context._setState('closed');
							context._suspends.rejectAll(() => domError('InvalidStateError', 'OfflineAudioContext: rendering finished before the suspend'));
						}
						if (!buffer) return reject(new Error('startRendering failed'));
						const renderedBuffer = new AudioBuffer(nativeCtor_, buffer);
						resolve(renderedBuffer);
						context?.dispatchEvent({ type: 'complete', renderedBuffer, target: context });
					},
				}),
			);
		});
		return this._rendering;
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
		return new StereoPannerNode(this as never, options ?? {});
	}
	createDelay(options?: DelayOptions | number) {
		return new DelayNode(this as never, options && typeof options === 'object' ? options : { maxDelayTime: options as number });
	}
	createConstantSource(options?: ConstantSourceOptions) {
		return new ConstantSourceNode(this as never, options ?? {});
	}
	createAnalyser(options?: AnalyserOptions) {
		return new AnalyserNode(this as never, options ?? {});
	}
	createWaveShaper(options?: WaveShaperOptions) {
		return new WaveShaperNode(this as never, options ?? {});
	}
	createIIRFilter(feedforward: number[], feedback: number[]) {
		return new IIRFilterNode(this as never, { feedforward, feedback });
	}
	createConvolver(options?: ConvolverOptions) {
		return new ConvolverNode(this as never, options ?? {});
	}
	createDynamicsCompressor(options?: DynamicsCompressorOptions) {
		return new DynamicsCompressorNode(this as never, options ?? {});
	}
	createChannelSplitter(options?: ChannelSplitterOptions) {
		return new ChannelSplitterNode(this as never, options ?? {});
	}
	createChannelMerger(options?: ChannelMergerOptions) {
		return new ChannelMergerNode(this as never, options ?? {});
	}
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }) {
		return new PeriodicWave(this as never, { real, imag, disableNormalization: options?.disableNormalization });
	}
	createBuffer(numberOfChannels: number | { length: number; numberOfChannels?: number; sampleRate: number }, length?: number, sampleRate?: number) {
		return createAudioBuffer(numberOfChannels, length, sampleRate);
	}
	createBufferSource(options?: { buffer?: AudioBuffer | null }) {
		return new AudioBufferSourceNode(this, options ?? {});
	}

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
		return decodeAudioData(this.native, source, successCallback, errorCallback);
	}
}

let workletScriptPath: string | null = null;

function workletScript(): string {
	if (!workletScriptPath) {
		const file = knownFolders.temp().getFile('ns-audio-worklet-scope.js');
		file.writeTextSync(SHARED_RING_BRIDGE + '\n' + AUDIO_WORKLET_SCOPE);
		workletScriptPath = file.path;
	}
	return workletScriptPath;
}

function readWorkletModule(source: string): Promise<string> {
	if (source.startsWith('data:')) {
		const comma = source.indexOf(',');
		const data = source.slice(comma + 1);
		return Promise.resolve(/;base64$/i.test(source.slice(0, comma)) ? atob(data) : decodeURIComponent(data));
	}
	if (!File.exists(source)) return Promise.reject(new Error(`${source}: no such file`));
	return File.fromPath(source).readText();
}

function workletPath(path: string): string {
	if (path.startsWith('file://')) path = decodeURIComponent(path.slice('file://'.length));
	return path.startsWith('~/') ? knownFolders.currentApp().path + path.slice(1) : path;
}

// Header word the native side marks: tells a shared ring from a copy.
const RING_NATIVE_MARK = 5;
const RING_CLOSED = 3;

let nextWorkletNode = 1;
// Connected nodes keep playing without a JS reference, as in browsers.
const connectedWorkletNodes = new Set<AudioWorkletNode>();
// The native voice reads the ring until the render thread drops it, a callback after stop().
const stopWorkletNode = (held: { native: org.nativescript.audiocontext.WorkletNode; ring: SharedArrayBuffer }) => {
	held.native.stop();
	setTimeout(() => held.ring.byteLength, 2000);
};
const workletNodeRegistry = typeof FinalizationRegistry === 'function' ? new FinalizationRegistry(stopWorkletNode) : null;

type WorkletContext = AudioContext | OfflineAudioContext;

export class AudioWorklet {
	private [context_]: WorkletContext;
	private _host: WorkletHost | null = null;

	constructor(a?: any, context?: WorkletContext) {
		if (a !== nativeCtor_ || !context) throw new TypeError('Illegal constructor.');
		this[context_] = context;
	}

	/** @internal */
	_scope(create: boolean): WorkletHost | null {
		if (!this._host && create) {
			const context = this[context_];
			this._host = new WorkletHost(workletScript(), { scope: 0, sampleRate: context.sampleRate, addon: '' }, { android: { priority: 'audio' } });
			closeWithContext?.register(context, this._host);
		}
		return this._host;
	}

	addModule(moduleURL: string | { href: string }, _options?: { credentials?: string }): Promise<void> {
		let host: WorkletHost | null = null;
		try {
			if (this[context_].state === 'closed') throw domError('InvalidStateError', 'The AudioContext is closed');
			host = this._scope(true);

			if (!host) {
				throw domError('InvalidStateError', 'Unable to initialize AudioWorklet');
			}
		} catch (e) {
			return Promise.reject(e);
		}
		const url = typeof moduleURL === 'string' ? moduleURL : String(moduleURL?.href ?? moduleURL);
		return loadWorkletModule(url, readWorkletModule, workletPath).then(({ source, url }) => host.addModule(source, url));
	}
}

const ANDROID_WORKLET_MAX_CHANNELS = 8;

export class AudioWorkletNode extends AudioNode {
	readonly port: AudioWorkletPort;
	readonly parameters: AudioParamMap<AudioParam>;
	[native_]: org.nativescript.audiocontext.WorkletNode;
	private _ring: SharedArrayBuffer;
	private _numberOfInputs: number;
	private _numberOfOutputs: number;
	private _onprocessorerror: ((event: any) => void) | null = null;

	constructor(context: WorkletContext, name: string, options: AudioWorkletNodeOptions = {}) {
		const worklet = context?.audioWorklet;
		const resolved = resolveWorkletNodeOptions(worklet?._scope(false), name, options);
		const { host, descriptors, numberOfInputs, numberOfOutputs, parameterData } = resolved;
		// Each input sums into channelCount channels.
		const inputChannels = Array.from({ length: numberOfInputs }, () => Math.max(1, (options as any).channelCount ?? 2) | 0);
		// One input and one output: the output follows the input; otherwise one channel per output.
		const outputChannelCount = resolved.outputChannelCount ?? Array.from({ length: numberOfOutputs }, () => (numberOfInputs === 1 && numberOfOutputs === 1 ? inputChannels[0] : 1));
		if ([...inputChannels, ...outputChannelCount].some((count) => count > ANDROID_WORKLET_MAX_CHANNELS)) {
			throw domError('NotSupportedError', `AudioWorkletNode: at most ${ANDROID_WORKLET_MAX_CHANNELS} channels per input or output on Android`);
		}
		if (numberOfInputs > 16 || numberOfOutputs > 16) throw domError('NotSupportedError', 'AudioWorkletNode: at most 16 inputs and 16 outputs on Android');
		super(context as any);
		const layout = workletRingLayout(numberOfInputs, outputChannelCount, descriptors.length);
		const ring = createWorkletRing(layout);
		const ints = (values: number[]) => {
			const array = (Array as any).create('int', values.length);
			values.forEach((value, i) => (array[i] = value));
			return array;
		};
		const defaults = (Array as any).create('double', descriptors.length);
		descriptors.forEach((d, i) => (defaults[i] = parameterData?.[d.name] ?? d.defaultValue));
		const native = AudioContext.getInstance().createWorklet(context.native, ring as any, ints(inputChannels), ints(outputChannelCount), ints([layout.slotWords, layout.header, layout.inputCounts, layout.inputData, layout.outputCounts, layout.paramLengths, layout.paramData]), ints(layout.outputData), defaults);
		if (!native) throw domError('NotSupportedError', 'AudioWorkletNode: the audio engine is unavailable');
		const header = new Uint32Array(ring, layout.header * 4, 8);
		if (Atomics.load(header, RING_NATIVE_MARK) !== 0x574b4c54) {
			native.stop();
			throw domError('NotSupportedError', 'AudioWorkletNode: this runtime copies SharedArrayBuffers passed to native code');
		}
		this[native_] = native;
		this._ring = ring;
		this._numberOfInputs = numberOfInputs;
		this._numberOfOutputs = numberOfOutputs;
		this.parameters = new AudioParamMap(new Map(descriptors.map((d, i) => [d.name, new AudioParam(nativeCtor_, native.getParam(i), { defaultValue: d.defaultValue, minValue: d.minValue, maxValue: d.maxValue })])));
		const id = nextWorkletNode++;
		this.port = new AudioWorkletPort(host, id);
		workletNodeRegistry?.register(this, { native, ring }, this);
		host.create(this, id, String(name), { ...options, numberOfInputs, numberOfOutputs, outputChannelCount, parameterData }, { buffer: ring, layout });
	}

	get native() {
		return this[native_];
	}

	get numberOfInputs() {
		return this._numberOfInputs;
	}

	get numberOfOutputs() {
		return this._numberOfOutputs;
	}

	connect(node: any, output?: number, input?: number) {
		if (output !== undefined && (output < 0 || output >= this._numberOfOutputs)) throw domError('IndexSizeError', `AudioWorkletNode.connect: output ${output} is out of range`);
		connectedWorkletNodes.add(this);
		return super.connect(node, output, input);
	}

	disconnect(destinationOrOutput?: any, output?: number, input?: number) {
		const native = this[native_];
		const checkOutput = (index: number) => {
			if (index < 0 || index >= this._numberOfOutputs) throw domError('IndexSizeError', `AudioWorkletNode.disconnect: output ${index} is out of range`);
		};
		if (destinationOrOutput === undefined || destinationOrOutput === null) {
			connectedWorkletNodes.delete(this);
			native.disconnect();
		} else if (typeof destinationOrOutput === 'number') {
			checkOutput(destinationOrOutput);
			native.disconnect(destinationOrOutput);
		} else {
			const target = destinationOrOutput.native || destinationOrOutput;
			if (typeof output !== 'number') native.disconnect(target);
			else {
				checkOutput(output);
				if (typeof input === 'number') native.disconnect(target, output, input);
				else native.disconnect(target, output);
			}
		}
	}

	get onprocessorerror() {
		return this._onprocessorerror;
	}

	set onprocessorerror(callback: ((event: any) => void) | null) {
		if (this._onprocessorerror) this.removeEventListener('processorerror', this._onprocessorerror);
		this._onprocessorerror = typeof callback === 'function' ? callback : null;
		if (this._onprocessorerror) this.addEventListener('processorerror', this._onprocessorerror);
	}

	/** @internal */
	_processorError(message: string) {
		// An offline render stops waiting on a failed processor.
		this._close();
		this.dispatchEvent({ type: 'processorerror', message, error: new Error(message), target: this } as any);
	}

	private _close() {
		Atomics.store(new Uint32Array(this._ring, this._ring.byteLength - 32, 8), RING_CLOSED, 1);
	}

	/** @internal */
	_stop() {
		workletNodeRegistry?.unregister(this);
		connectedWorkletNodes.delete(this);
		this._close();
		stopWorkletNode({ native: this[native_], ring: this._ring });
	}
}

let audioContextInstance: org.nativescript.audiocontext.AudioContext | null = null;

export class AudioContext extends BaseAudioContext {
	destination: AudioDestinationNode | null = null;
	[native_]: org.nativescript.audiocontext.AudioContextInstance;

	static getInstance() {
		if (!audioContextInstance) audioContextInstance = org.nativescript.audiocontext.AudioContext.getInstance();
		return audioContextInstance;
	}

	constructor(options?: AudioContextOptions) {
		super();
		const hint = options?.latencyHint;
		if (typeof hint === 'string' && !['balanced', 'interactive', 'playback'].includes(hint)) throw new TypeError(`AudioContext: invalid latencyHint '${hint}'`);
		if (options?.sampleRate !== undefined) checkSampleRate('AudioContext', options.sampleRate);
		audioContextInstance = org.nativescript.audiocontext.AudioContext.getInstance();
		const sampleRate = options?.sampleRate ?? 0;
		const latencyHintSec = resolveLatencyHint(options?.latencyHint);
		this[native_] = audioContextInstance.createContextInstance(sampleRate, latencyHintSec);

		this.destination = new AudioDestinationNode(this, this[native_].getDestination());
		this._setState('running');
	}

	private _audioWorklet: AudioWorklet | null = null;

	get audioWorklet(): AudioWorklet | undefined {
		if (!this._audioWorklet && isWorkerSupported()) this._audioWorklet = new AudioWorklet(nativeCtor_, this);
		return this._audioWorklet ?? undefined;
	}

	createMediaElementSource(mediaElement: MediaElementLike): MediaElementAudioSourceNode {
		return new MediaElementAudioSourceNode(this, mediaElement);
	}

	createMediaStreamSource(_stream: any): never {
		throw domError('NotSupportedError', 'AudioContext.createMediaStreamSource: NativeScript has no MediaStream');
	}

	createMediaStreamTrackSource(_track: any): never {
		throw domError('NotSupportedError', 'AudioContext.createMediaStreamTrackSource: NativeScript has no MediaStream');
	}

	createMediaStreamDestination(_options?: any): never {
		throw domError('NotSupportedError', 'AudioContext.createMediaStreamDestination: NativeScript has no MediaStream');
	}

	createSourceNodeFromPlayer(playerNative: any): MediaElementAudioSourceNode | null {
		try {
			const native: any = this.native;
			if (!native || !playerNative || typeof native.createSourceNodeFromMediaPlayer !== 'function') return null;
			const tapNative = native.createSourceNodeFromMediaPlayer(playerNative);
			if (!tapNative) return null;
			const stub: MediaElementLike = { src: '', play() {}, pause() {} };
			return new MediaElementAudioSourceNode(this, stub, tapNative);
		} catch (e) {
			return null;
		}
	}

	get native() {
		return this[native_];
	}

	get sampleRate(): number {
		return this.native.getSampleRate();
	}

	get baseLatency(): number {
		return this.native.getBaseLatency();
	}

	get outputLatency(): number {
		return this.native.getOutputLatency();
	}

	get currentTime(): number {
		return this.native.getCurrentTime();
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
		return new StereoPannerNode(this as never, options ?? {});
	}
	createDelay(options?: DelayOptions | number) {
		return new DelayNode(this as never, options && typeof options === 'object' ? options : { maxDelayTime: options as number });
	}
	createConstantSource(options?: ConstantSourceOptions) {
		return new ConstantSourceNode(this as never, options ?? {});
	}
	createAnalyser(options?: AnalyserOptions) {
		return new AnalyserNode(this as never, options ?? {});
	}
	createWaveShaper(options?: WaveShaperOptions) {
		return new WaveShaperNode(this as never, options ?? {});
	}
	createIIRFilter(feedforward: number[], feedback: number[]) {
		return new IIRFilterNode(this as never, { feedforward, feedback });
	}
	createConvolver(options?: ConvolverOptions) {
		return new ConvolverNode(this as never, options ?? {});
	}
	createDynamicsCompressor(options?: DynamicsCompressorOptions) {
		return new DynamicsCompressorNode(this as never, options ?? {});
	}
	createChannelSplitter(options?: ChannelSplitterOptions) {
		return new ChannelSplitterNode(this as never, options ?? {});
	}
	createChannelMerger(options?: ChannelMergerOptions) {
		return new ChannelMergerNode(this as never, options ?? {});
	}
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }) {
		return new PeriodicWave(this as never, { real, imag, disableNormalization: options?.disableNormalization });
	}
	createBuffer(numberOfChannels: number | { length: number; numberOfChannels?: number; sampleRate: number }, length?: number, sampleRate?: number) {
		return createAudioBuffer(numberOfChannels, length, sampleRate);
	}
	createBufferSource(options?: { buffer?: AudioBuffer }) {
		return new AudioBufferSourceNode(this, options);
	}

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
		return decodeAudioData(this.native, source, successCallback, errorCallback);
	}

	resume(): Promise<void> {
		if (this._state === 'closed' || this._closing) return Promise.reject(domError('InvalidStateError', 'AudioContext.resume: the context is closed'));
		return new Promise((resolve, reject) => {
			const ref = new WeakRef(this);
			AudioContext.getInstance().resumeAsync(
				new org.nativescript.audiocontext.AudioContext.AsyncCallback({
					onComplete(ok) {
						if (ok) {
							const that = ref.get();
							if (that) {
								that._setState('running');
							}
							resolve();
						} else {
							reject(new Error('AudioContext resume failed'));
						}
					},
				}),
			);
		});
	}

	suspend(): Promise<void> {
		if (this._state === 'closed' || this._closing) return Promise.reject(domError('InvalidStateError', 'AudioContext.suspend: the context is closed'));
		return new Promise((resolve, reject) => {
			const ref = new WeakRef(this);
			AudioContext.getInstance().suspendAsync(
				new org.nativescript.audiocontext.AudioContext.AsyncCallback({
					onComplete(ok) {
						if (ok) {
							const that = ref.get();
							if (that) {
								that._setState('suspended');
							}
							resolve();
						} else {
							reject(new Error('AudioContext suspend failed'));
						}
					},
				}),
			);
		});
	}

	close(): Promise<void> {
		if (this._state === 'closed' || this._closing) return Promise.reject(domError('InvalidStateError', 'AudioContext.close: the context is already closed'));
		this._closing = true;
		this._audioWorklet?._scope(false)?.close();
		connectedWorkletNodes.forEach((node) => {
			if (node.context === this) node._stop();
		});
		return new Promise((resolve, reject) => {
			const ref = new WeakRef(this);
			this.native.closeAsync(
				new org.nativescript.audiocontext.AudioContext.AsyncCallback({
					onComplete(ok) {
						if (ok) {
							const that = ref.get();
							if (that) {
								that._setState('closed');
							}
							resolve();
						} else {
							reject(new Error('AudioContext close failed'));
						}
					},
				}),
			);
		});
	}

	private _closing = false;

	getOutputTimestamp(): { contextTime: number; performanceTime: number } {
		let time = 0;
		let latency = 0;
		try {
			time = this.currentTime;
			if (this._state === 'running') latency = this.outputLatency || 0;
		} catch (e) {}
		const now = (globalThis as any).performance?.now?.() ?? Date.now();
		return { contextTime: Math.max(0, time - latency), performanceTime: Math.max(0, now - latency * 1000) };
	}

	private _sinkId = 'default';
	get sinkId(): string {
		return this._sinkId;
	}

	setSinkId(deviceId: string): Promise<void> {
		return new Promise((resolve, reject) => {
			if (this._state === 'closed') return reject(domError('InvalidStateError', 'setSinkId: the context is closed'));
			const value = deviceId == null || deviceId === '' ? 'default' : `${deviceId}`;
			if (value === this._sinkId) return resolve();
			try {
				if (!this.native.setSinkId(value)) return reject(domError('NotFoundError', `setSinkId: no output device ${deviceId}`));
			} catch (e) {
				return reject(e);
			}
			this._sinkId = value;
			resolve();
			this.dispatchEvent({ type: 'sinkchange', target: this });
		});
	}

	private _onsinkchange: ((ev: any) => void) | null = null;
	get onsinkchange() {
		return this._onsinkchange;
	}
	set onsinkchange(cb: ((ev: any) => void) | null) {
		if (this._onsinkchange) this.removeEventListener('sinkchange', this._onsinkchange);
		this._onsinkchange = cb;
		if (cb) this.addEventListener('sinkchange', cb);
	}

	private _listener: AudioListener | null = null;

	get listener() {
		if (!this._listener) this._listener = new AudioListener(nativeCtor_, this as unknown as NativeBaseAudioContext);
		return this._listener;
	}
}

export class OscillatorNode extends AudioScheduledSourceNode {
	private _periodicWave: PeriodicWave | null = null;
	constructor(context: NativeBaseAudioContext, options: { type?: string; frequency?: number; periodicWave?: PeriodicWave } = {}) {
		const type = options?.type || 'sine';
		if (type === 'custom' && !options.periodicWave) throw domError('InvalidStateError', "OscillatorNode: type 'custom' needs a periodicWave");
		const freq = typeof options?.frequency === 'number' ? options.frequency : 440;
		super(context, context.native.createOscillatorNodeFrequency(type === 'custom' ? 'sine' : type, freq));
		if (options.periodicWave) this.setPeriodicWave(options.periodicWave);
	}

	get native() {
		return this[native_] as org.nativescript.audiocontext.AudioOscillatorNode;
	}

	get type(): string {
		return this._periodicWave ? 'custom' : this.native.getType();
	}

	set type(v: string) {
		if (v === 'custom') throw domError('InvalidStateError', "OscillatorNode.type: set a PeriodicWave instead of 'custom'");
		if (!['sine', 'square', 'sawtooth', 'triangle'].includes(v)) return;
		this._periodicWave = null;
		this.native.setType(v);
	}

	private _detuneParam: AudioParam | null = null;
	get detune(): AudioParam {
		if (!this._detuneParam) this._detuneParam = new AudioParam(nativeCtor_, this.native.getDetune(), { defaultValue: 0, minValue: -1200 * Math.log2(FLT_MAX), maxValue: 1200 * Math.log2(FLT_MAX) });
		return this._detuneParam;
	}
	private _frequencyParam!: AudioParam;
	get frequency(): AudioParam {
		const nyquist = this._contextRate / 2;
		if (!this._frequencyParam) this._frequencyParam = new AudioParam(nativeCtor_, this.native.getFrequency(), { defaultValue: 440, minValue: -nyquist, maxValue: nyquist });
		return this._frequencyParam;
	}

	setPeriodicWave(wave: PeriodicWave) {
		if (!(wave instanceof PeriodicWave)) throw new TypeError('OscillatorNode.setPeriodicWave: argument is not a PeriodicWave');
		this._periodicWave = wave;
		this.native.setPeriodicWave(wave.native);
	}
}

export class StereoPannerNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AudioPannerNode;
	private _panParam: AudioParam | null = null;
	constructor(context: NativeBaseAudioContext, options: StereoPannerOptions = {}) {
		super(context);
		const p = resolvePannerOptions({ pan: options.pan });
		const node = AudioContext.getInstance().createPanner(context.native);
		this[native_] = node;
		// Panning model 2: the engine pans by `pan` alone, without distance or cone.
		node.setPanningModel(2);
		AudioContext.getInstance().setPannerParams(node, p.positionX, p.positionY, p.positionZ, p.orientationX, p.orientationY, p.orientationZ, p.pan, p.distanceModel, 2, p.refDistance, p.maxDistance, p.rolloffFactor, p.coneInnerAngle, p.coneOuterAngle, p.coneOuterGain);
	}
	get native() {
		return this[native_];
	}
	get pan(): AudioParam {
		if (!this._panParam) {
			this._panParam = new AudioParam(nativeCtor_, this.native.getPan(), { defaultValue: 0, minValue: -1, maxValue: 1 });
		}
		return this._panParam;
	}
}

export class DelayNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.DelayNode;
	private _delayTimeParam: AudioParamBase | null = null;
	readonly maxDelayTime: number;
	constructor(context: NativeBaseAudioContext, options: DelayOptions = {}) {
		super(context);
		this.maxDelayTime = options.maxDelayTime ?? 1.0;
		if (!(this.maxDelayTime > 0 && this.maxDelayTime < 180)) throw domError('NotSupportedError', `DelayNode: maxDelayTime ${options.maxDelayTime} is outside (0, 180)`);
		const delay = AudioContext.getInstance().createDelay(context.native, this.maxDelayTime);
		if (typeof options.delayTime === 'number' && options.delayTime !== 0) {
			delay.getDelayTime().setValue(options.delayTime);
		}
		this[native_] = delay;
	}
	get native() {
		return this[native_];
	}
	get delayTime(): AudioParamBase {
		if (!this._delayTimeParam) {
			const param = this.native.getDelayTime();
			this._delayTimeParam = new AudioParam(nativeCtor_, param, { defaultValue: 0, minValue: 0, maxValue: this.maxDelayTime });
		}
		return this._delayTimeParam;
	}
}

export class ConstantSourceNode extends AudioScheduledSourceNode {
	private _offsetParam: AudioParamBase | null = null;
	constructor(context: NativeBaseAudioContext, options: ConstantSourceOptions = {}) {
		const node = AudioContext.getInstance().createConstantSource(context.native);
		super(context, node as never);
		if (typeof options.offset === 'number') {
			node.getOffsetParam().setValue(options.offset);
		}
	}
	get native() {
		return this[native_] as org.nativescript.audiocontext.ConstantSourceNode;
	}

	get offset(): AudioParamBase {
		if (!this._offsetParam) {
			this._offsetParam = new AudioParam(nativeCtor_, this.native.getOffsetParam(), { defaultValue: 1 });
		}
		return this._offsetParam;
	}
}

export class AnalyserNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AnalyserNode;
	constructor(context: NativeBaseAudioContext, options: AnalyserOptions = {}) {
		super(context);
		this[native_] = AudioContext.getInstance().createAnalyser(context.native);
		if (options.fftSize !== undefined) this.fftSize = options.fftSize;
		if (options.smoothingTimeConstant !== undefined) this.smoothingTimeConstant = options.smoothingTimeConstant;
		const min = options.minDecibels ?? this.minDecibels;
		const max = options.maxDecibels ?? this.maxDecibels;
		if (!(min < max)) throw domError('IndexSizeError', `AnalyserNode: minDecibels ${min} must be less than maxDecibels ${max}`);
		this.native.setMinDecibels(min);
		this.native.setMaxDecibels(max);
	}
	get native() {
		return this[native_];
	}
	get fftSize() {
		return this.native.getFftSize();
	}
	set fftSize(v: number) {
		const size = Math.floor(v);
		if (!(size >= 32 && size <= 32768 && (size & (size - 1)) === 0)) throw domError('IndexSizeError', `AnalyserNode.fftSize: ${v} is not a power of two in 32..32768`);
		this.native.setFftSize(size);
	}
	get frequencyBinCount() {
		return (this.native.getFftSize() / 2) | 0;
	}
	get smoothingTimeConstant() {
		return this.native.getSmoothingTimeConstant();
	}
	set smoothingTimeConstant(v: number) {
		if (!(v >= 0 && v <= 1)) throw domError('IndexSizeError', `AnalyserNode.smoothingTimeConstant: ${v} is outside [0, 1]`);
		this.native.setSmoothingTimeConstant(v);
	}
	get minDecibels() {
		return this.native.getMinDecibels();
	}
	set minDecibels(v: number) {
		if (!(v < this.maxDecibels)) throw domError('IndexSizeError', `AnalyserNode.minDecibels: ${v} must be less than maxDecibels`);
		this.native.setMinDecibels(v);
	}
	get maxDecibels() {
		return this.native.getMaxDecibels();
	}
	set maxDecibels(v: number) {
		if (!(v > this.minDecibels)) throw domError('IndexSizeError', `AnalyserNode.maxDecibels: ${v} must be more than minDecibels`);
		this.native.setMaxDecibels(v);
	}
	getFloatTimeDomainData(dest: Float32Array) {
		if (!dest || dest.length === 0) return;
		this.native.getFloatTimeDomainData(dest as never);
	}
	getByteTimeDomainData(dest: Uint8Array) {
		if (!dest || dest.length === 0) return;
		this.native.getByteTimeDomainData(dest as never);
	}
	getFloatFrequencyData(dest: Float32Array) {
		if (!dest || dest.length === 0) return;
		this.native.getFloatFrequencyData(dest as never);
	}
	getByteFrequencyData(dest: Uint8Array) {
		if (!dest || dest.length === 0) return;
		this.native.getByteFrequencyData(dest as never);
	}
}

export class WaveShaperNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.WaveShaperNode;
	private _curve: Float32Array | null = null;
	constructor(context: NativeBaseAudioContext, options: WaveShaperOptions = {}) {
		super(context);
		this[native_] = AudioContext.getInstance().createWaveShaper(context.native);
		if (options.curve) this.curve = options.curve as Float32Array;
		if (options.oversample) this.oversample = options.oversample;
	}
	get native() {
		return this[native_];
	}

	get curve(): Float32Array | null {
		return this._curve ? this._curve.slice() : null;
	}
	set curve(v: Float32Array | null) {
		if (v && v.length < 2) throw domError('InvalidStateError', `WaveShaperNode.curve: length ${v.length} is less than 2`);
		this._curve = v ? Float32Array.from(v) : null;
		this.native.setCurveFromData(this._curve ?? (null as any));
	}
	get oversample() {
		return this.native.getOversample() as 'none' | '2x' | '4x';
	}
	set oversample(v: 'none' | '2x' | '4x') {
		this.native.setOversample(v);
	}
}

export class IIRFilterNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AudioIIRNode;
	constructor(context: NativeBaseAudioContext, options: IIRFilterOptions) {
		super(context);
		if (!options?.feedforward || !options?.feedback) throw new TypeError('IIRFilterNode: feedforward and feedback are required');
		const { feedforward, feedback } = options;
		if (!(feedforward.length >= 1 && feedforward.length <= 20)) throw domError('NotSupportedError', `IIRFilterNode: feedforward length ${feedforward.length} is outside 1..20`);
		if (!(feedback.length >= 1 && feedback.length <= 20)) throw domError('NotSupportedError', `IIRFilterNode: feedback length ${feedback.length} is outside 1..20`);
		if (Array.prototype.every.call(feedforward, (v: number) => v === 0)) throw domError('InvalidStateError', 'IIRFilterNode: feedforward is all zero');
		if (feedback[0] === 0) throw domError('InvalidStateError', 'IIRFilterNode: feedback[0] is zero');
		this[native_] = AudioContext.getInstance().createIIR(context.native, options.feedforward as never, options.feedback as never);
	}
	get native() {
		return this[native_];
	}
	getFrequencyResponse(frequencyHz: Float32Array, magResponse: Float32Array, phaseResponse: Float32Array) {
		if (frequencyHz.length !== magResponse.length || frequencyHz.length !== phaseResponse.length) throw domError('InvalidAccessError', 'IIRFilterNode.getFrequencyResponse: array lengths differ');
		this.native.getFrequencyResponse(frequencyHz, magResponse, phaseResponse);
	}
}

export class ConvolverNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.ConvolverNode;
	constructor(context: NativeBaseAudioContext, options: ConvolverOptions & { buffer?: AudioBuffer | null } = {}) {
		super(context);
		this[native_] = AudioContext.getInstance().createConvolver(context.native);
		this.normalize = !options.disableNormalization;
		if (options.buffer) this.buffer = options.buffer;
	}
	get native() {
		return this[native_];
	}

	get buffer(): AudioBuffer | null {
		const buffer = this.native.getBuffer();
		if (!buffer) return null;
		return new AudioBuffer(nativeCtor_, buffer);
	}
	set buffer(value: AudioBuffer | null) {
		if (value && ![1, 2, 4].includes(value.numberOfChannels)) throw domError('NotSupportedError', `ConvolverNode.buffer: ${value.numberOfChannels} channels`);
		if (value && value.sampleRate !== this._contextRate) throw domError('NotSupportedError', `ConvolverNode.buffer: sampleRate ${value.sampleRate} differs from the context's ${this._contextRate}`);
		this.native.setBuffer(value?.native ?? (null as never));
	}
	get normalize(): boolean {
		return this.native.getNormalize();
	}
	set normalize(value: boolean) {
		this.native.setNormalize(value);
	}
}

export class DynamicsCompressorNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.DynamicsCompressorNode;
	private _threshold: AudioParam | null = null;
	private _knee: AudioParam | null = null;
	private _ratio: AudioParam | null = null;
	private _attack: AudioParam | null = null;
	private _release: AudioParam | null = null;

	constructor(context: NativeBaseAudioContext, options: DynamicsCompressorOptions = {}) {
		super(context);
		this[native_] = AudioContext.getInstance().createDynamicsCompressor(context.native);

		if (typeof options.threshold === 'number') this.threshold.value = options.threshold;
		if (typeof options.knee === 'number') this.knee.value = options.knee;
		if (typeof options.ratio === 'number') this.ratio.value = options.ratio;
		if (typeof options.attack === 'number') this.attack.value = options.attack;
		if (typeof options.release === 'number') this.release.value = options.release;
	}

	get native() {
		return this[native_];
	}

	get threshold() {
		return this._threshold || (this._threshold = new AudioParam(nativeCtor_, this.native.getThreshold(), { defaultValue: -24, minValue: -100, maxValue: 0, fixedRate: 'k-rate' }));
	}

	get knee() {
		return this._knee || (this._knee = new AudioParam(nativeCtor_, this.native.getKnee(), { defaultValue: 30, minValue: 0, maxValue: 40, fixedRate: 'k-rate' }));
	}

	get ratio() {
		return this._ratio || (this._ratio = new AudioParam(nativeCtor_, this.native.getRatio(), { defaultValue: 12, minValue: 1, maxValue: 20, fixedRate: 'k-rate' }));
	}

	get attack() {
		return this._attack || (this._attack = new AudioParam(nativeCtor_, this.native.getAttack(), { defaultValue: 0.003, minValue: 0, maxValue: 1, fixedRate: 'k-rate' }));
	}

	get release() {
		return this._release || (this._release = new AudioParam(nativeCtor_, this.native.getRelease(), { defaultValue: 0.25, minValue: 0, maxValue: 1, fixedRate: 'k-rate' }));
	}

	get reduction(): number {
		return this.native.getReduction();
	}
}

export class ChannelSplitterNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.ChannelSplitterNode;
	private _numberOfOutputs: number;

	constructor(context: NativeBaseAudioContext, options: ChannelSplitterOptions = {}) {
		super(context);
		this._numberOfOutputs = options.numberOfOutputs ?? 6;
		if (!(this._numberOfOutputs >= 1 && this._numberOfOutputs <= 32)) throw domError('IndexSizeError', `ChannelSplitterNode: numberOfOutputs ${this._numberOfOutputs} is outside 1..32`);
		this[native_] = AudioContext.getInstance().createChannelSplitter(context.native, this._numberOfOutputs);
	}

	get native() {
		return this[native_];
	}

	get numberOfOutputs() {
		const native = this.native as org.nativescript.audiocontext.ChannelSplitterNode;
		if (native && typeof native.getNumberOfOutputs === 'function') {
			const value = native.getNumberOfOutputs();
			if (typeof value === 'number' && Number.isFinite(value)) return value;
		}
		return this._numberOfOutputs;
	}
}

export class ChannelMergerNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.ChannelMergerNode;
	private _numberOfInputs: number;

	constructor(context: NativeBaseAudioContext, options: ChannelMergerOptions = {}) {
		super(context);
		this._numberOfInputs = options.numberOfInputs ?? 6;
		if (!(this._numberOfInputs >= 1 && this._numberOfInputs <= 32)) throw domError('IndexSizeError', `ChannelMergerNode: numberOfInputs ${this._numberOfInputs} is outside 1..32`);
		this[native_] = AudioContext.getInstance().createChannelMerger(context.native, this._numberOfInputs);
	}

	get native() {
		return this[native_];
	}

	get numberOfInputs() {
		const native = this.native as org.nativescript.audiocontext.ChannelMergerNode;
		if (native && typeof native.getNumberOfInputs === 'function') {
			const value = native.getNumberOfInputs();
			if (typeof value === 'number' && Number.isFinite(value)) return value;
		}
		return this._numberOfInputs;
	}
}

export class PeriodicWave {
	[native_]: org.nativescript.audiocontext.PeriodicWave;
	private real: Float32Array;
	private imag: Float32Array;
	private disableNormalization: boolean;
	constructor(_context: NativeBaseAudioContext, options: PeriodicWaveOptions = {}) {
		const imagOnly = !options.real ? options.imag : undefined;
		this.real = options.real ? Float32Array.from(options.real) : new Float32Array(imagOnly ? imagOnly.length : 2);
		this.imag = options.imag ? Float32Array.from(options.imag) : new Float32Array(this.real.length);
		if (this.real.length !== this.imag.length || this.real.length < 2) throw domError('IndexSizeError', `PeriodicWave: real (${this.real.length}) and imag (${this.imag.length}) must match and be at least 2 long`);
		this.disableNormalization = !!options.disableNormalization;
		this[native_] = AudioContext.getInstance().createPeriodicWave(_context.native, this.real as never, this.imag as never, this.disableNormalization as never);
	}
	get native() {
		return this[native_];
	}
}

export class AudioListener extends AudioListenerBase {
	constructor(guard: Symbol, context: NativeBaseAudioContext) {
		if (guard !== nativeCtor_) throw new TypeError('Illegal constructor.');
		super(context);
	}

	get context() {
		return this[context_] as AudioContext;
	}
	get positionX() {
		return this._positionX || (this._positionX = new AudioParam(nativeCtor_, this.context.native.getListenerPositionXParam(), { defaultValue: 0 }));
	}
	get positionY() {
		return this._positionY || (this._positionY = new AudioParam(nativeCtor_, this.context.native.getListenerPositionYParam(), { defaultValue: 0 }));
	}
	get positionZ() {
		return this._positionZ || (this._positionZ = new AudioParam(nativeCtor_, this.context.native.getListenerPositionZParam(), { defaultValue: 0 }));
	}

	get forwardX() {
		return this._forwardX || (this._forwardX = new AudioParam(nativeCtor_, this.context.native.getListenerForwardXParam(), { defaultValue: 0 }));
	}
	get forwardY() {
		return this._forwardY || (this._forwardY = new AudioParam(nativeCtor_, this.context.native.getListenerForwardYParam(), { defaultValue: 0 }));
	}
	get forwardZ() {
		return this._forwardZ || (this._forwardZ = new AudioParam(nativeCtor_, this.context.native.getListenerForwardZParam(), { defaultValue: -1 }));
	}

	get upX() {
		return this._upX || (this._upX = new AudioParam(nativeCtor_, this.context.native.getListenerUpXParam(), { defaultValue: 0 }));
	}
	get upY() {
		return this._upY || (this._upY = new AudioParam(nativeCtor_, this.context.native.getListenerUpYParam(), { defaultValue: 1 }));
	}
	get upZ() {
		return this._upZ || (this._upZ = new AudioParam(nativeCtor_, this.context.native.getListenerUpZParam(), { defaultValue: 0 }));
	}

	setPosition(x: number, y: number, z: number) {
		this.context.native.setListenerParams(x, y, z, this.forwardX.value, this.forwardY.value, this.forwardZ.value, this.upX.value, this.upY.value, this.upZ.value);
	}

	setOrientation(forwardX: number, forwardY: number, forwardZ: number, upX?: number, upY?: number, upZ?: number) {
		this.context.native.setListenerParams(this.positionX.value, this.positionY.value, this.positionZ.value, forwardX, forwardY, forwardZ, upX ?? 0, upY ?? 1, upZ ?? 0);
	}
}
