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
	native_,
	nativeCtor_,
	OfflineSuspends,
	normalizeSourcePath,
	resolveLatencyHint,
	resolvePannerOptions,
	AudioListenerBase,
	assertMediaElementUsable,
	markMediaElementUsed,
	unmarkMediaElementUsed,
	resolveNativePlayer,
	throwInvalidMediaElement,
	applyConvolverOptions,
	AudioNodeChannelOptions,
	CHANNEL_RULES,
	checkChannelSetting,
} from './common';
import { File, knownFolders } from '@nativescript/core';
import { AUDIO_WORKLET_SCOPE } from './audio-worklet-scope';
import { SHARED_RING_BRIDGE, createWorkletRing, workletRingLayout } from './audio-worklet-ring';
import { AudioParamMap, AudioWorkletNodeOptions, AudioWorkletPort, WorkletHost, closeWithContext, domError, isWorkerSupported, loadWorkletModule, resolveWorkletNodeOptions } from './audio-worklet-host';
export { AudioWorkletNodeOptions };

type NativeBaseAudioContext = BaseAudioContext & { native: NSCAudioContext };

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

function makeIOSHooks(native: NSCAudioParam): AudioParamHooks {
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
		nativeCancel(t) {
			native.cancelScheduledValues(t);
		},
		nativeCancelAndHold(v, t) {
			native.cancelAndHoldAtTime(v, t);
		},
		nativeScheduleExpRamp(v, t) {
			native.exponentialRampToValueAtTime(v, t);
		},
		nativeScheduleTarget(v, t, c) {
			native.setTargetAtTime(v, t, c);
		},
		nativeScheduleCurve(values, t, d) {
			const copy = values.slice();
			native.setValueCurveAtTime(copy.buffer as never, t, d);
		},
		nativeGetValue() {
			return native.value;
		},
		nativeComputedValue() {
			return native.computedValue();
		},
		nativeSetAutomationRate(rate) {
			native.automationRate = rate;
		},
	};
}
export class AudioParam extends AudioParamBase {
	private [native_]: NSCAudioParam;

	constructor(a?: any, b?: any) {
		if (a !== nativeCtor_ || !(b instanceof NSCAudioParam)) {
			throw new TypeError('Illegal constructor.');
		}
		super(makeIOSHooks(b), b.value);
		this[native_] = b;
	}

	get native() {
		return this[native_];
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

	get numberOfInputs(): number {
		return this instanceof AudioScheduledSourceNode || this instanceof MediaElementAudioSourceNode ? 0 : 1;
	}

	get numberOfOutputs(): number {
		return this instanceof AudioDestinationNode ? 0 : 1;
	}

	/** AudioNodeOptions, applied once the native node exists. */
	protected _applyNodeOptions(options?: AudioNodeChannelOptions) {
		if (!options) return;
		if (options.channelCount !== undefined) this.channelCount = options.channelCount;
		if (options.channelCountMode !== undefined) this.channelCountMode = options.channelCountMode;
		if (options.channelInterpretation !== undefined) this.channelInterpretation = options.channelInterpretation;
	}

	private _checkChannel(prop: 'channelCount' | 'channelCountMode' | 'channelInterpretation', value: any) {
		if (this instanceof AudioDestinationNode) {
			if (prop === 'channelCount' && this.context instanceof OfflineAudioContext && value !== this.channelCount) throw domError('InvalidStateError', 'OfflineAudioContext destination channelCount cannot change');
			if (prop === 'channelCount' && value > this.maxChannelCount) throw domError('IndexSizeError', `channelCount ${value} is above maxChannelCount`);
		}
		checkChannelSetting(this._channelRule(), prop, value, this.channelCount, domError);
	}

	private _channelRule() {
		if (this instanceof PannerNode) return CHANNEL_RULES.PannerNode;
		if (this instanceof StereoPannerNode) return CHANNEL_RULES.StereoPannerNode;
		if (this instanceof ConvolverNode) return CHANNEL_RULES.ConvolverNode;
		if (this instanceof DynamicsCompressorNode) return CHANNEL_RULES.DynamicsCompressorNode;
		if (this instanceof ChannelSplitterNode) return CHANNEL_RULES.ChannelSplitterNode;
		if (this instanceof ChannelMergerNode) return CHANNEL_RULES.ChannelMergerNode;
		return undefined;
	}

	// The Web Audio defaults for this node's kind.
	private _channelDefaults(): [number, string, string] {
		if (this instanceof ChannelSplitterNode) return [this.numberOfOutputs, 'explicit', 'discrete'];
		if (this instanceof ChannelMergerNode) return [1, 'explicit', 'speakers'];
		if (this instanceof AudioDestinationNode) return [this.context instanceof OfflineAudioContext ? ((this.context as any).native?.format?.channelCount ?? 2) : 2, 'explicit', 'speakers'];
		if (this instanceof PannerNode || this instanceof StereoPannerNode || this instanceof ConvolverNode || this instanceof DynamicsCompressorNode) return [2, 'clamped-max', 'speakers'];
		return [2, 'max', 'speakers'];
	}

	private _applyChannelConfig(count: number, mode: number, interpretation: number) {
		try {
			(this as never as { native: NSCAudioNode }).native?.setChannelConfigModeInterpretation(count, mode, interpretation);
		} catch (e) {}
	}

	get channelCount(): number {
		return this._channelCount ?? this._channelDefaults()[0];
	}

	set channelCount(value: number) {
		const count = Math.floor(value);
		this._checkChannel('channelCount', count);
		this._channelCount = count;
		this._applyChannelConfig(count, -1, -1);
	}

	get channelCountMode(): string {
		return this._channelCountMode ?? this._channelDefaults()[1];
	}

	set channelCountMode(value: string) {
		const mode = COUNT_MODES.indexOf(value);
		if (mode < 0) return;
		this._checkChannel('channelCountMode', value);
		this._channelCountMode = value;
		this._applyChannelConfig(-1, mode, -1);
	}

	get channelInterpretation(): string {
		return this._channelInterpretation ?? this._channelDefaults()[2];
	}

	set channelInterpretation(value: string) {
		const interpretation = INTERPRETATIONS.indexOf(value);
		if (interpretation < 0) return;
		this._checkChannel('channelInterpretation', value);
		this._channelInterpretation = value;
		this._applyChannelConfig(-1, -1, interpretation);
	}

	connect(node: any, output?: number, input?: number) {
		const that = this as never as AudioNode & { native: NSCAudioNode };
		const target = node?.native || node;
		try {
			if (target instanceof NSCAudioParam) that.native.connectToParamOutput(target, typeof output === 'number' ? output : (null as never));
			else if (typeof output === 'number' && typeof input === 'number') that.native.connectToOutputInput(target, output, input);
			else if (typeof output === 'number') that.native.connectToOutput(target, output);
			else that.native.connectTo(target);
		} catch (e) {
			console.warn(`AudioNode.connect failed:`, e);
		}
		if (!(node instanceof AudioParamBase)) return node;
	}

	disconnect(destinationOrOutput?: any, output?: number, input?: number) {
		const that = this as never as AudioNode & { native: NSCAudioNode };
		try {
			if (!destinationOrOutput) {
				that.native.disconnect();
				return;
			}
			if (typeof destinationOrOutput === 'number') {
				try {
					that.native.disconnectOutput(destinationOrOutput);
				} catch (e) {
					that.native.disconnect();
				}
				return;
			}
			const dest = destinationOrOutput.native || destinationOrOutput;
			try {
				if (dest instanceof NSCAudioParam) that.native.disconnectFromParamOutput(dest, typeof output === 'number' ? output : (null as never));
				else if (typeof output === 'number' && typeof input === 'number') that.native.disconnectToOutputInput(dest, output, input);
				else if (typeof output === 'number') that.native.disconnectToOutput(dest, output);
				else that.native.disconnectTo(dest);
			} catch (e) {
				try {
					that.native.disconnect();
				} catch (ee) {}
			}
		} catch (e) {}
	}
}

export class AudioBuffer {
	private [native_]: NSCAudioBuffer;

	constructor(options: { length: number; numberOfChannels?: number; sampleRate?: number } | Symbol, buffer?: NSCAudioBuffer) {
		if (!options) throw new TypeError('AudioBuffer constructor requires options');

		if (options === nativeCtor_ && buffer instanceof NSCAudioBuffer) {
			this[native_] = buffer;
		} else {
			const opts = options as { length: number; numberOfChannels?: number; sampleRate?: number };
			const length = opts.length | 0;
			const ch = opts.numberOfChannels ?? 1;
			const sr = opts.sampleRate ?? 48000;
			if (!(ch >= 1 && ch <= 32)) throw domError('NotSupportedError', `AudioBuffer: numberOfChannels ${ch} is outside 1 to 32`);
			if (!(length >= 1)) throw domError('NotSupportedError', `AudioBuffer: length ${opts.length} must be at least 1`);
			if (!(sr >= 3000 && sr <= 768000)) throw domError('NotSupportedError', `AudioBuffer: sampleRate ${sr} is outside 3000 to 768000`);

			this[native_] = NSCAudioBuffer.alloc().initWithLengthNumberOfChannelsSampleRate(length, ch, sr);
		}
	}

	get native() {
		return this[native_];
	}

	get sampleRate(): number {
		return this.native.sampleRate;
	}
	get length(): number {
		return this.native.length;
	}
	get duration(): number {
		return this.native.duration;
	}
	get numberOfChannels(): number {
		return this.native.numberOfChannels;
	}

	getChannelData(channel: number): Float32Array | null {
		if (!(channel >= 0 && channel < this.numberOfChannels)) throw domError('IndexSizeError', `getChannelData: channel ${channel} is out of range`);
		const native = this.native as any;
		const audioModule = (globalThis as any)?.AudioModule;

		if (audioModule) {
			try {
				const address = native.getPCMBufferAddress();
				if (typeof address === 'string' && address.length > 0) {
					const value = audioModule.createFloat32ArrayFromPCMBufferAddress(address, channel);
					if (value instanceof Float32Array) {
						return value;
					}
				}
			} catch (e) {}
		}

		const data = new Float32Array(this.length);
		native.copyFromChannelWithByteOffsetByteLength(data.buffer as any, channel, 0, 0, data.byteLength);
		return data;
	}

	copyFromChannel(dest: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!dest) return;
		if (!(channel >= 0 && channel < this.numberOfChannels)) throw domError('IndexSizeError', `copyFromChannel: channel ${channel} is out of range`);
		const native = this.native;
		const options = resolveAudioBufferCopyOptions(dest, startInChannel);
		native.copyFromChannelWithByteOffsetByteLength(dest.buffer as any, channel, options.startInChannel, options.byteOffset, dest.byteLength);
	}

	copyToChannel(source: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!source) return;
		if (!(channel >= 0 && channel < this.numberOfChannels)) throw domError('IndexSizeError', `copyToChannel: channel ${channel} is out of range`);
		const native = this.native;
		const options = resolveAudioBufferCopyOptions(source, startInChannel);
		native.copyToChannelWithByteOffsetByteLength(source.buffer as any, channel, options.startInChannel, options.byteOffset, source.byteLength);
	}
}

export class GainNode extends AudioNode {
	private _native: NSCGainNode;
	private _gainParam!: AudioParam;
	private _initialGain: number;
	constructor(context: NativeBaseAudioContext, options: { gain?: number } = {}) {
		super(context);
		this._native = context.native.createGainNode();
		this._initialGain = typeof options?.gain === 'number' ? options.gain : (this._native.gain ?? 1);
		if (typeof options?.gain === 'number') {
			this._native.gain = options.gain;
		}
		this._applyNodeOptions(options);
	}
	get native() {
		return this._native;
	}
	get gain(): AudioParam {
		if (!this._gainParam) {
			const gain = this._native.gainParam;
			if (this._initialGain !== 1) gain.setValue(this._initialGain);
			this._gainParam = new AudioParam(nativeCtor_, gain);
		}
		return this._gainParam;
	}
}

export class BiquadFilterNode extends AudioNode {
	private [native_]: NSCBiquadNode;
	private _frequencyParam!: AudioParam;
	private _qParam!: AudioParam;
	private _gainParam!: AudioParam;
	private _detuneParam!: AudioParam;
	constructor(context: NativeBaseAudioContext, options: { type?: string; frequency?: number; Q?: number; gain?: number } = {}) {
		super(context);
		const initialFreq = typeof options?.frequency === 'number' ? options!.frequency! : 440;
		const initialQ = typeof options?.Q === 'number' ? options!.Q! : 1.0;
		const initialGain = typeof options?.gain === 'number' ? options!.gain! : 0.0;
		this[native_] = context.native.createBiquadNodeFrequencyQGain(options.type ?? 'lowpass', initialFreq, initialQ, initialGain);
		if (typeof (options as any).detune === 'number') this.detune.value = (options as any).detune;
		this._applyNodeOptions(options);
	}
	get native() {
		return this[native_];
	}
	get frequency(): AudioParam {
		if (!this._frequencyParam) this._frequencyParam = new AudioParam(nativeCtor_, this.native.frequencyParam);
		return this._frequencyParam;
	}
	get Q(): AudioParam {
		if (!this._qParam) this._qParam = new AudioParam(nativeCtor_, this.native.qParam);
		return this._qParam;
	}
	get gain(): AudioParam {
		if (!this._gainParam) this._gainParam = new AudioParam(nativeCtor_, this.native.gainParam);
		return this._gainParam;
	}
	get detune(): AudioParam {
		if (!this._detuneParam) this._detuneParam = new AudioParam(nativeCtor_, this.native.detuneParam);
		return this._detuneParam;
	}
	set type(v: string) {
		this.native.setType(v);
	}
	get type(): string {
		return this.native.getType();
	}
}

export class PannerNode extends AudioNode {
	[native_]: NSCAudioPannerNode;

	constructor(context: NativeBaseAudioContext, options: PannerOptions = {}) {
		super(context);
		const p = resolvePannerOptions(options);
		this[native_] = NSCAudioPannerNode.alloc().initWithContextAndParams(context.native, p.positionX, p.positionY, p.positionZ, p.orientationX, p.orientationY, p.orientationZ, p.pan, p.distanceModel, p.panningModel, p.refDistance, p.maxDistance, p.rolloffFactor, p.coneInnerAngle, p.coneOuterAngle, p.coneOuterGain);
		this._applyNodeOptions(options);
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
		return this._positionX || (this._positionX = new AudioParam(nativeCtor_, this.native.positionXParam));
	}
	get positionY() {
		return this._positionY || (this._positionY = new AudioParam(nativeCtor_, this.native.positionYParam));
	}
	get positionZ() {
		return this._positionZ || (this._positionZ = new AudioParam(nativeCtor_, this.native.positionZParam));
	}
	get orientationX() {
		return this._orientationX || (this._orientationX = new AudioParam(nativeCtor_, this.native.orientationXParam));
	}
	get orientationY() {
		return this._orientationY || (this._orientationY = new AudioParam(nativeCtor_, this.native.orientationYParam));
	}
	get orientationZ() {
		return this._orientationZ || (this._orientationZ = new AudioParam(nativeCtor_, this.native.orientationZParam));
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
		this.native.setRefDistance(v);
	}
	get maxDistance() {
		return this.native.getMaxDistance();
	}
	set maxDistance(v: number) {
		this.native.setMaxDistance(v);
	}
	get rolloffFactor() {
		return this.native.getRolloffFactor();
	}
	set rolloffFactor(v: number) {
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
		this.native.setConeOuterGain(v);
	}

	setPosition(x: number, y: number, z: number) {
		this.native.setPosition(x, y, z);
	}
	setOrientation(x: number, y: number, z: number) {
		this.native.setOrientation(x, y, z);
	}
}

export class AudioDestinationNode extends AudioNode {
	[native_]: NSCAudioNode;
	get maxChannelCount(): number {
		if (this.context instanceof OfflineAudioContext) return this.channelCount;
		try {
			return Math.max(2, AVAudioSession.sharedInstance().maximumOutputNumberOfChannels);
		} catch (e) {
			return 2;
		}
	}
	constructor(context: NativeBaseAudioContext, node: NSCAudioNode) {
		super(context);
		if (!(node instanceof NSCAudioNode)) throw new TypeError('Illegal constructor.');
		this[native_] = node;
	}
	get native() {
		return this[native_];
	}
}

export class AudioScheduledSourceNode extends AudioNode {
	[native_]: NSCAudioScheduledSourceNode;
	private _onended: ((ev: { type: 'ended' }) => void) | null = null;
	private _nativeEndedWired = false;
	private _endedFired = false;
	private _started = false;

	constructor(context: NativeBaseAudioContext, node: NSCAudioScheduledSourceNode) {
		super(context);
		if (!(node instanceof NSCAudioScheduledSourceNode)) throw new TypeError('Illegal constructor.');
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

	private _ensureNativeEndedWired() {
		if (this._nativeEndedWired) return;
		this._nativeEndedWired = true;
		this[native_].onended = () => this._fireEnded();
	}

	protected _fireEnded() {
		if (this._endedFired) return;
		this._endedFired = true;
		this.dispatchEvent({ type: 'ended', target: this });
	}

	start(when?: number, offset?: number, duration?: number) {
		if (this._started) throw domError('InvalidStateError', `${this.constructor.name}.start: already started`);
		if ((typeof when === 'number' && when < 0) || (typeof offset === 'number' && offset < 0) || (typeof duration === 'number' && duration < 0)) throw new RangeError(`${this.constructor.name}.start: arguments must be >= 0`);
		this._started = true;
		this._endedFired = false;
		const at = typeof when === 'number' && Number.isFinite(when) && when > 0 ? when : 0;
		if (at > 0) this._ensureNativeEndedWired();
		this._startNative(at, typeof offset === 'number' && Number.isFinite(offset) ? Math.max(0, offset) : 0);
		if (typeof duration === 'number' && Number.isFinite(duration) && duration >= 0) {
			const now = (this.context as any)?.currentTime ?? 0;
			const startAt = typeof when === 'number' && Number.isFinite(when) ? Math.max(now, when) : now;
			this.stop(startAt + duration);
		}
	}

	protected _startNative(when: number, _offset: number) {
		if (when > 0) {
			this.native.startAt(when);
		} else {
			this.native.start();
		}
	}

	stop(when?: number) {
		if (!this._started) throw domError('InvalidStateError', `${this.constructor.name}.stop: not started`);
		if (typeof when === 'number' && when < 0) throw new RangeError(`${this.constructor.name}.stop: when must be >= 0`);
		const now = (this.context as any)?.currentTime ?? 0;
		if (typeof when === 'number' && Number.isFinite(when) && when > now) {
			this._ensureNativeEndedWired();
			this.native.stopAt(when);
		} else {
			this.native.stop();
			setTimeout(() => this._fireEnded(), 0);
		}
	}
}

export class OscillatorNode extends AudioScheduledSourceNode {
	constructor(context: NativeBaseAudioContext, options: { type?: string; frequency?: number; detune?: number; periodicWave?: PeriodicWave } & AudioNodeChannelOptions = {}) {
		const type = options?.type || 'sine';
		if (type === 'custom' && !options.periodicWave) throw domError('InvalidStateError', "OscillatorNode: type 'custom' needs a periodicWave");
		const freq = typeof options?.frequency === 'number' ? options.frequency : 440;
		super(context, context.native.createOscillatorNodeFrequency(type === 'custom' ? 'sine' : type, freq));
		if (typeof options.detune === 'number') this.detune.value = options.detune;
		if (options.periodicWave) this.setPeriodicWave(options.periodicWave);
		this._applyNodeOptions(options);
	}

	get native() {
		return this[native_] as NSCAudioOscillatorNode;
	}

	get type(): string {
		return this.native.type;
	}

	set type(v: string) {
		if (v === 'custom') throw domError('InvalidStateError', "OscillatorNode.type: use setPeriodicWave for 'custom'");
		if (!['sine', 'square', 'sawtooth', 'triangle'].includes(v)) return;
		this.native.type = v;
	}

	private _frequencyParam!: AudioParam;
	get frequency(): AudioParam {
		if (!this._frequencyParam) this._frequencyParam = new AudioParam(nativeCtor_, this.native.frequencyParam);
		return this._frequencyParam;
	}

	private _detuneParam!: AudioParam;
	get detune(): AudioParam {
		if (!this._detuneParam) this._detuneParam = new AudioParam(nativeCtor_, this.native.detuneParam);
		return this._detuneParam;
	}

	setPeriodicWave(wave: PeriodicWave) {
		if (!wave) return;
		this.native.setPeriodicWave(wave.native);
	}
}

export class StereoPannerNode extends AudioNode {
	[native_]: NSCStereoPannerNode;
	private _panParam!: AudioParam;
	constructor(context: NativeBaseAudioContext, options: StereoPannerOptions = {}) {
		super(context);
		const pan = typeof options.pan === 'number' ? options.pan : 0;
		this[native_] = context.native.createStereoPannerNode(pan);
		this._applyNodeOptions(options);
	}
	get native() {
		return this[native_];
	}
	get pan(): AudioParam {
		if (!this._panParam) this._panParam = new AudioParam(nativeCtor_, this.native.panParam);
		return this._panParam;
	}
}

export class DelayNode extends AudioNode {
	[native_]: NSCDelayNode;
	private _delayTimeParam: AudioParamBase | null = null;
	constructor(context: NativeBaseAudioContext, options: DelayOptions = {}) {
		super(context);
		const delay = typeof options.delayTime === 'number' ? options.delayTime : 0;
		const max = typeof options.maxDelayTime === 'number' ? options.maxDelayTime : 1.0;
		if (!(max > 0 && max < 180)) throw domError('NotSupportedError', `DelayNode: maxDelayTime ${max} must be above 0 and below 180`);
		this[native_] = context.native.createDelayNodeMaxDelayTime(delay, max);
		this._applyNodeOptions(options);
	}
	get native() {
		return this[native_];
	}
	get delayTime(): AudioParamBase {
		if (!this._delayTimeParam) {
			const n = this.native;
			this._delayTimeParam = new AudioParam(nativeCtor_, n.delayTimeParam);
		}
		return this._delayTimeParam;
	}
	get maxDelayTime() {
		return this.native.maxDelayTime;
	}
}

export class ConstantSourceNode extends AudioScheduledSourceNode {
	private _offsetParam: AudioParamBase | null = null;
	constructor(context: NativeBaseAudioContext, options: ConstantSourceOptions = {}) {
		const offset = typeof options.offset === 'number' ? options.offset : 1.0;
		super(context, context.native.createConstantSourceNode(offset) as NSCAudioScheduledSourceNode);
		this._applyNodeOptions(options as any);
	}
	get native() {
		return this[native_] as NSCConstantSourceNode;
	}
	get offset(): AudioParamBase {
		if (!this._offsetParam) {
			const n = this.native;
			this._offsetParam = new AudioParam(nativeCtor_, n.offsetParam);
		}
		return this._offsetParam;
	}
}

export class AnalyserNode extends AudioNode {
	[native_]: NSCAnalyserNode;
	constructor(context: NativeBaseAudioContext, options: AnalyserOptions = {}) {
		super(context);
		this[native_] = context.native.createAnalyserNode();
		if (typeof options.fftSize === 'number') this.fftSize = options.fftSize;
		if (typeof options.smoothingTimeConstant === 'number') this.smoothingTimeConstant = options.smoothingTimeConstant;
		const min = options.minDecibels ?? this.minDecibels;
		const max = options.maxDecibels ?? this.maxDecibels;
		if (min >= max) throw domError('IndexSizeError', 'AnalyserNode: minDecibels must be below maxDecibels');
		this.native.minDecibels = min;
		this.native.maxDecibels = max;
		this._applyNodeOptions(options);
	}
	get native() {
		return this[native_];
	}
	get fftSize() {
		return this.native.fftSize;
	}
	set fftSize(v: number) {
		if (!(v >= 32 && v <= 32768 && (v & (v - 1)) === 0)) throw domError('IndexSizeError', `AnalyserNode.fftSize: ${v} must be a power of two from 32 to 32768`);
		this.native.fftSize = v;
	}
	get frequencyBinCount() {
		return this.native.frequencyBinCount;
	}
	get smoothingTimeConstant() {
		return this.native.smoothingTimeConstant;
	}
	set smoothingTimeConstant(v: number) {
		if (!(v >= 0 && v <= 1)) throw domError('IndexSizeError', `AnalyserNode.smoothingTimeConstant: ${v} is outside 0 to 1`);
		this.native.smoothingTimeConstant = v;
	}
	get minDecibels() {
		return this.native.minDecibels;
	}
	set minDecibels(v: number) {
		if (v >= this.maxDecibels) throw domError('IndexSizeError', 'AnalyserNode.minDecibels must be below maxDecibels');
		this.native.minDecibels = v;
	}
	get maxDecibels() {
		return this.native.maxDecibels;
	}
	set maxDecibels(v: number) {
		if (v <= this.minDecibels) throw domError('IndexSizeError', 'AnalyserNode.maxDecibels must be above minDecibels');
		this.native.maxDecibels = v;
	}
	getFloatTimeDomainData(dest: Float32Array) {
		if (!dest || dest.length === 0) return;
		const native: any = this.native;
		const byteOffset = normalizeCopyByteOffset(dest);
		if (byteOffset === 0) {
			native.getFloatTimeDomainData(dest as any);
			return;
		}
		native.getFloatTimeDomainDataWithByteOffset(dest as any, byteOffset);
	}
	getByteTimeDomainData(dest: Uint8Array) {
		if (!dest || dest.length === 0) return;
		const native = this.native;
		native.getByteTimeDomainData(dest as never);
	}
	getFloatFrequencyData(dest: Float32Array) {
		if (!dest || dest.length === 0) return;
		const native: any = this.native;
		const byteOffset = normalizeCopyByteOffset(dest);
		if (byteOffset === 0) {
			native.getFloatFrequencyData(dest as any);
			return;
		}
		native.getFloatFrequencyDataWithByteOffset(dest as any, byteOffset);
	}
	getByteFrequencyData(dest: Uint8Array) {
		if (!dest || dest.length === 0) return;
		const native = this.native;
		native.getByteFrequencyData(dest as never);
	}
}

export class WaveShaperNode extends AudioNode {
	[native_]: NSCWaveShaperNode;
	private _curve: Float32Array | null = null;
	constructor(context: NativeBaseAudioContext, options: WaveShaperOptions = {}) {
		super(context);
		this[native_] = context.native.createWaveShaperNode();
		if (options.curve) this.curve = options.curve instanceof Float32Array ? options.curve : Float32Array.from(options.curve);
		if (options.oversample) this.oversample = options.oversample;
		this._applyNodeOptions(options);
	}
	get native() {
		return this[native_];
	}
	get curve(): Float32Array | null {
		return this._curve;
	}
	set curve(v: Float32Array | null) {
		if (v && v.length < 2) throw domError('InvalidStateError', 'WaveShaperNode.curve needs at least 2 values');
		this._curve = v;
		if (!v) {
			this.native.setCurveFromData(null as never);
			return;
		}
		const byteOffset = normalizeCopyByteOffset(v);
		if (byteOffset === 0) {
			if (v.byteOffset === 0 && v.byteLength === v.buffer.byteLength) {
				this.native.setCurveFromData(v.buffer as any);
				return;
			}
			this.native.setCurveFromData(v as any);
			return;
		}
		this.native.setCurveFromDataWithByteOffset(v as any, byteOffset);
	}
	get oversample() {
		return this.native.oversample as 'none' | '2x' | '4x';
	}
	set oversample(v: 'none' | '2x' | '4x') {
		this.native.oversample = v;
	}
}

export class IIRFilterNode extends AudioNode {
	[native_]: NSCIIRFilterNode;
	constructor(context: NativeBaseAudioContext, options: IIRFilterOptions) {
		super(context);
		const ff = options?.feedforward ?? [];
		const fb = options?.feedback ?? [];
		if (!ff.length || !fb.length || ff.length > 20 || fb.length > 20) throw domError('NotSupportedError', 'IIRFilterNode: feedforward and feedback need 1 to 20 coefficients');
		if (ff.every((v) => v === 0)) throw domError('InvalidStateError', 'IIRFilterNode: feedforward is all zero');
		if (fb[0] === 0) throw domError('InvalidStateError', 'IIRFilterNode: feedback[0] is zero');
		this[native_] = context.native.createIIRFilterNodeFeedback(ff as never, fb as never);
		this._applyNodeOptions(options as any);
	}
	get native() {
		return this[native_];
	}
	getFrequencyResponse(frequencyHz: Float32Array, magResponse: Float32Array, phaseResponse: Float32Array) {
		if (!frequencyHz || !magResponse || !phaseResponse) return;
		const native: any = this.native;
		const frequencyHzByteOffset = normalizeCopyByteOffset(frequencyHz);
		const magResponseByteOffset = normalizeCopyByteOffset(magResponse);
		const phaseResponseByteOffset = normalizeCopyByteOffset(phaseResponse);
		if (frequencyHzByteOffset === 0 && magResponseByteOffset === 0 && phaseResponseByteOffset === 0) {
			native.getFrequencyResponseMagResponsePhaseResponse(frequencyHz as any, magResponse as any, phaseResponse as any);
			return;
		}
		native.getFrequencyResponseMagResponsePhaseResponseWithByteOffsets(frequencyHz as any, frequencyHzByteOffset, magResponse as any, magResponseByteOffset, phaseResponse as any, phaseResponseByteOffset);
	}
}

export class ConvolverNode extends AudioNode {
	[native_]: NSCConvolverNode;
	constructor(context: NativeBaseAudioContext, options: ConvolverOptions = {}) {
		super(context);
		this[native_] = context.native.createConvolverNode();
		applyConvolverOptions(this, options);
		this._applyNodeOptions(options as any);
	}
	get native() {
		return this[native_];
	}
	get buffer(): AudioBuffer | null {
		const b = this.native.buffer;
		return b ? new AudioBuffer(nativeCtor_, b) : null;
	}
	set buffer(v: AudioBuffer | null) {
		if (v) {
			if (![1, 2, 4].includes(v.numberOfChannels)) throw domError('NotSupportedError', `ConvolverNode.buffer: ${v.numberOfChannels} channels; it takes 1, 2 or 4`);
			if (v.sampleRate !== (this.context as any).sampleRate) throw domError('NotSupportedError', "ConvolverNode.buffer: the buffer's sampleRate differs from the context's");
		}
		this.native.buffer = v?.native ?? (null as never);
	}

	get normalize() {
		return this.native.normalize;
	}
	set normalize(v: boolean) {
		this.native.normalize = v;
	}
}

export class DynamicsCompressorNode extends AudioNode {
	[native_]: NSCDynamicsCompressorNode;
	private _threshold: AudioParam | null = null;
	private _knee: AudioParam | null = null;
	private _ratio: AudioParam | null = null;
	private _attack: AudioParam | null = null;
	private _release: AudioParam | null = null;
	private _reduction: AudioParam | null = null;

	constructor(context: NativeBaseAudioContext, options: DynamicsCompressorOptions = {}) {
		super(context);
		this[native_] = context.native.createDynamicsCompressorNode();

		if (typeof options.threshold === 'number') this.threshold.value = options.threshold;
		if (typeof options.knee === 'number') this.knee.value = options.knee;
		if (typeof options.ratio === 'number') this.ratio.value = options.ratio;
		if (typeof options.attack === 'number') this.attack.value = options.attack;
		if (typeof options.release === 'number') this.release.value = options.release;
		this._applyNodeOptions(options as any);
	}

	get native() {
		return this[native_];
	}

	get threshold() {
		return this._threshold || (this._threshold = new AudioParam(nativeCtor_, this.native.thresholdParam));
	}

	get knee() {
		return this._knee || (this._knee = new AudioParam(nativeCtor_, this.native.kneeParam));
	}

	get ratio() {
		return this._ratio || (this._ratio = new AudioParam(nativeCtor_, this.native.ratioParam));
	}

	get attack() {
		return this._attack || (this._attack = new AudioParam(nativeCtor_, this.native.attackParam));
	}

	get release() {
		return this._release || (this._release = new AudioParam(nativeCtor_, this.native.releaseParam));
	}

	get reduction() {
		return this._reduction || (this._reduction = new AudioParam(nativeCtor_, this.native.reductionParam));
	}
}

export class ChannelSplitterNode extends AudioNode {
	[native_]: NSCChannelSplitterNode;
	private _numberOfOutputs: number;

	constructor(context: NativeBaseAudioContext, options: ChannelSplitterOptions = {}) {
		super(context);
		const outputs = options.numberOfOutputs ?? 6;
		if (!(outputs >= 1 && outputs <= 32)) throw domError('IndexSizeError', `ChannelSplitterNode: numberOfOutputs ${outputs} is outside 1 to 32`);
		this._numberOfOutputs = outputs;
		this[native_] = context.native.createChannelSplitterNode(this._numberOfOutputs);
		this._applyNodeOptions(options as any);
	}

	get native() {
		return this[native_];
	}

	get numberOfOutputs() {
		const value = this.native?.numberOfOutputs;
		if (typeof value === 'number' && Number.isFinite(value)) return value;
		return this._numberOfOutputs;
	}
}

export class ChannelMergerNode extends AudioNode {
	[native_]: NSCChannelMergerNode;
	private _numberOfInputs: number;

	constructor(context: NativeBaseAudioContext, options: ChannelMergerOptions = {}) {
		super(context);
		const inputs = options.numberOfInputs ?? 6;
		if (!(inputs >= 1 && inputs <= 32)) throw domError('IndexSizeError', `ChannelMergerNode: numberOfInputs ${inputs} is outside 1 to 32`);
		this._numberOfInputs = inputs;
		this[native_] = context.native.createChannelMergerNode(this._numberOfInputs);
		this._applyNodeOptions(options as any);
	}

	get native() {
		return this[native_];
	}

	get numberOfInputs() {
		const value = this.native?.numberOfInputs;
		if (typeof value === 'number' && Number.isFinite(value)) return value;
		return this._numberOfInputs;
	}
}

export class PeriodicWave {
	[native_]: NSCPeriodicWave;
	constructor(_context: NativeBaseAudioContext, options: PeriodicWaveOptions = {}) {
		const real = options.real ? (options.real instanceof Float32Array ? options.real : Float32Array.from(options.real)) : new Float32Array(2);
		const imag = options.imag ? (options.imag instanceof Float32Array ? options.imag : Float32Array.from(options.imag)) : new Float32Array(real.length);
		if (real.length !== imag.length || real.length < 2) throw domError('IndexSizeError', 'PeriodicWave: real and imag need the same length, at least 2');
		const realByteOffset = normalizeCopyByteOffset(real);
		const imagByteOffset = normalizeCopyByteOffset(imag);

		if (realByteOffset === 0 && imagByteOffset === 0) {
			if (real.byteOffset === 0 && real.byteLength === real.buffer.byteLength && imag.byteOffset === 0 && imag.byteLength === imag.buffer.byteLength) {
				this[native_] = NSCPeriodicWave.alloc().initWithRealImagDisableNormalization(real.buffer as any, imag.buffer as any, !!options.disableNormalization);
				return;
			}
			this[native_] = NSCPeriodicWave.alloc().initWithRealImagDisableNormalization(real as any, imag as any, !!options.disableNormalization);
			return;
		}

		this[native_] = NSCPeriodicWave.alloc().initWithRealImagDisableNormalizationRealByteOffsetImagByteOffset(real as any, imag as any, !!options.disableNormalization, realByteOffset, imagByteOffset);
	}
	get native() {
		return this[native_];
	}
}

export class AudioBufferSourceNode extends AudioScheduledSourceNode {
	private _playbackRateParam: AudioParam | null = null;
	constructor(context: NativeBaseAudioContext, options: { buffer?: AudioBuffer; loop?: boolean; loopStart?: number; loopEnd?: number; playbackRate?: number; detune?: number } & AudioNodeChannelOptions = {}) {
		const param: NSCAudioBuffer | null = options?.buffer != null ? options.buffer.native : null;
		super(context, context.native.createBufferSourceNode(param as never));
		if (options.buffer) this._bufferSet = true;
		if (options.loop) this.loop = true;
		if (typeof options.loopStart === 'number') this.loopStart = options.loopStart;
		if (typeof options.loopEnd === 'number') this.loopEnd = options.loopEnd;
		if (typeof options.playbackRate === 'number') this.playbackRate.value = options.playbackRate;
		if (typeof options.detune === 'number') this.detune.value = options.detune;
		this._applyNodeOptions(options);
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

	get native() {
		return this[native_] as NSCAudioBufferSourceNode;
	}

	protected _startNative(when: number, offset: number) {
		this.native.startAtOffset(when > 0 ? when : (null as never), offset);
	}

	set loop(v: boolean) {
		this.native.loop = v;
	}
	get loop() {
		return this.native.loop;
	}

	get buffer(): AudioBuffer | null {
		const b = this.native.buffer;
		return b ? new AudioBuffer(nativeCtor_, b) : null;
	}
	set buffer(v: AudioBuffer | null) {
		if (v && this._bufferSet) throw domError('InvalidStateError', 'AudioBufferSourceNode.buffer can only be set once');
		if (v) this._bufferSet = true;
		this.native.buffer = v?.native ?? (null as never);
	}
	private _bufferSet = false;

	get playbackRate(): AudioParam | null {
		if (this._playbackRateParam) return this._playbackRateParam;
		this._playbackRateParam = new AudioParam(nativeCtor_, this.native.getPlaybackRateParam());
		return this._playbackRateParam;
	}

	private _detuneParam: AudioParam | null = null;
	get detune(): AudioParam {
		if (!this._detuneParam) this._detuneParam = new AudioParam(nativeCtor_, this.native.detuneParam);
		return this._detuneParam;
	}
}

export class MediaElementAudioSourceNode extends AudioNode {
	private [native_]: NSCAudioNode;
	private _mediaElement: MediaElementLike;
	private _playbackRateParam: AudioParam | null = null;

	constructor(context: AudioContext, mediaElement: MediaElementLike, preExistingNative?: NSCAudioNode) {
		super(context as unknown as NativeBaseAudioContext);
		assertMediaElementUsable(context, mediaElement);

		const native = preExistingNative ?? MediaElementAudioSourceNode._createNative(context, mediaElement?._audio ?? mediaElement);
		if (!native) throwInvalidMediaElement();

		this._mediaElement = mediaElement;
		this[native_] = native;
		markMediaElementUsed(mediaElement);
	}

	private static _createNative(context: AudioContext, mediaElement: MediaElementLike): NSCAudioNode | null {
		const ctxNative = context.native;
		if (!ctxNative) return null;
		const player = resolveNativePlayer(mediaElement);
		if (player && typeof ctxNative.createSourceNodeFromMediaPlayer === 'function') {
			try {
				const node = ctxNative.createSourceNodeFromMediaPlayer(player);
				if (node instanceof NSCAudioNode) return node;
			} catch (e) {}
		}
		if (typeof mediaElement.attachAudioContextTap === 'function') {
			try {
				const node = mediaElement.attachAudioContextTap(ctxNative);
				if (node instanceof NSCAudioNode) return node;
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

	get playbackRate(): AudioParam | null {
		if (this._playbackRateParam) return this._playbackRateParam;
		const native: any = this[native_];
		try {
			const param = typeof native?.getPlaybackRateParam === 'function' ? native.getPlaybackRateParam() : null;
			if (param instanceof NSCAudioParam) {
				this._playbackRateParam = new AudioParam(nativeCtor_, param);
				return this._playbackRateParam;
			}
		} catch (e) {}
		return null;
	}

	disposeMediaElementSource() {
		const native = this[native_];
		const el = this._mediaElement;
		try {
			const ctxNative = (this.context as any)?.native;
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
	return path.startsWith('~/') ? knownFolders.currentApp().path + path.slice(1) : path;
}

let nextWorkletNode = 1;
const IOS_WORKLET_MAX_CHANNELS = 8;
// Connected nodes keep playing without a JS reference, as in browsers.
const connectedWorkletNodes = new Set<AudioWorkletNode>();

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
			this._host = new WorkletHost(workletScript(), { scope: 0, sampleRate: context.sampleRate, addon: '' });
			closeWithContext?.register(context, this._host);
		}
		return this._host;
	}

	addModule(moduleURL: string | { href: string }, _options?: { credentials?: string }): Promise<void> {
		let host: WorkletHost;
		try {
			if (this[context_].state === 'closed') throw domError('InvalidStateError', 'The AudioContext is closed');
			host = this._scope(true);
		} catch (e) {
			return Promise.reject(e);
		}
		const url = typeof moduleURL === 'string' ? moduleURL : String(moduleURL?.href ?? moduleURL);
		return loadWorkletModule(url, readWorkletModule, workletPath).then(({ source, url }) => host.addModule(source, url));
	}
}

export class AudioWorkletNode extends AudioNode {
	readonly port: AudioWorkletPort;
	readonly parameters: AudioParamMap<AudioParam>;
	private _native: NSCAudioWorkletNode;
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
		if ([...inputChannels, ...outputChannelCount].some((count) => count > IOS_WORKLET_MAX_CHANNELS)) {
			throw domError('NotSupportedError', `AudioWorkletNode: at most ${IOS_WORKLET_MAX_CHANNELS} channels per input or output on iOS`);
		}
		super(context);
		const layout = workletRingLayout(numberOfInputs, outputChannelCount, descriptors.length);
		const ring = createWorkletRing(layout);
		const defaults = descriptors.map((d) => parameterData?.[d.name] ?? d.defaultValue);
		this._ring = ring;
		this._numberOfInputs = numberOfInputs;
		this._numberOfOutputs = numberOfOutputs;
		this._native = NSCAudioWorkletNode.alloc().initWithContextRingInputChannelsOutputChannelsSlotWordsHeaderInputCountsInputDataOutputCountsOutputDataParamLengthsParamDataParamDefaults(
			context.native,
			ring as any,
			inputChannels,
			outputChannelCount,
			layout.slotWords,
			layout.header,
			layout.inputCounts,
			layout.inputData,
			layout.outputCounts,
			layout.outputData,
			layout.paramLengths,
			layout.paramData,
			defaults,
		);
		if (!this._native) throw domError('NotSupportedError', 'AudioWorkletNode: the audio unit could not be created');
		const params = this._native.params;
		this.parameters = new AudioParamMap(new Map(descriptors.map((d, i) => [d.name, new AudioParam(nativeCtor_, params.objectAtIndex(i))])));
		const id = nextWorkletNode++;
		this.port = new AudioWorkletPort(host, id);
		host.create(this, id, String(name), { ...options, numberOfInputs, numberOfOutputs, outputChannelCount, parameterData }, { buffer: ring, layout });
	}

	get numberOfInputs() {
		return this._numberOfInputs;
	}

	get numberOfOutputs() {
		return this._numberOfOutputs;
	}

	get native() {
		return this._native;
	}

	connect(node: any, output?: number, input?: number) {
		if (output !== undefined && (output < 0 || output >= this._numberOfOutputs)) throw domError('IndexSizeError', `AudioWorkletNode.connect: output ${output} is out of range`);
		connectedWorkletNodes.add(this);
		return super.connect(node, output, input);
	}

	disconnect(...args: any[]) {
		if (args.length === 0) connectedWorkletNodes.delete(this);
		return (super.disconnect as any)(...args);
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
		this._native?.close();
		this.dispatchEvent({ type: 'processorerror', message, error: new Error(message), target: this } as any);
	}
}

export class AudioContext extends BaseAudioContext {
	private [native_]: NSCAudioContext;

	private _destination: AudioDestinationNode | null = null;

	private _listener: AudioListener | null = null;

	private _audioWorklet: AudioWorklet | null = null;

	get audioWorklet(): AudioWorklet | undefined {
		if (!this._audioWorklet && isWorkerSupported()) this._audioWorklet = new AudioWorklet(nativeCtor_, this);
		return this._audioWorklet ?? undefined;
	}

	constructor(options?: AudioContextOptions) {
		super();
		const sampleRate = options?.sampleRate ?? 0;
		if (options?.sampleRate !== undefined && !(sampleRate >= 3000 && sampleRate <= 768000)) throw domError('NotSupportedError', `AudioContext: sampleRate ${sampleRate} is out of range`);
		const latencyHint = resolveLatencyHint(options?.latencyHint);
		this[native_] = NSCAudioContext.alloc().initWithSampleRateLatencyHint(sampleRate, latencyHint);
		this._sampleRate = this[native_].sampleRate;
		try {
			const nd = this[native_].destination;
			this._destination = nd ? new AudioDestinationNode(this, nd) : null;
		} catch (e) {
			this._destination = null;
		}
		this._setState('running');
	}

	createMediaElementSource(mediaElement: MediaElementLike): MediaElementAudioSourceNode {
		return new MediaElementAudioSourceNode(this, mediaElement);
	}

	createSourceNodeFromPlayer(playerNative: any): MediaElementAudioSourceNode | null {
		try {
			const native = this.native;
			if (!native || !playerNative || typeof native.createSourceNodeFromMediaPlayer !== 'function') return null;
			const tapNative = native.createSourceNodeFromMediaPlayer(playerNative);
			if (!(tapNative instanceof NSCAudioNode)) return null;
			const stub: MediaElementLike = { src: '', play() {}, pause() {} };
			return new MediaElementAudioSourceNode(this, stub, tapNative);
		} catch (e) {
			return null;
		}
	}

	get listener() {
		if (!this._listener) this._listener = new AudioListener(nativeCtor_, this as unknown as NativeBaseAudioContext);
		return this._listener;
	}

	get destination() {
		return this._destination;
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
	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number } | number, length?: number, sampleRate?: number) {
		return new AudioBuffer(typeof options === 'number' ? { numberOfChannels: options, length, sampleRate } : options);
	}
	createBufferSource(options?: { buffer?: AudioBuffer }) {
		return new AudioBufferSourceNode(this, options ?? {});
	}
	createStereoPanner(options?: StereoPannerOptions) {
		return new StereoPannerNode(this, options ?? {});
	}
	createDelay(options?: DelayOptions) {
		return new DelayNode(this, options ?? {});
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
	createChannelSplitter(options?: ChannelSplitterOptions) {
		return new ChannelSplitterNode(this, options ?? {});
	}
	createChannelMerger(options?: ChannelMergerOptions) {
		return new ChannelMergerNode(this, options ?? {});
	}
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }) {
		return new PeriodicWave(this, { real, imag, disableNormalization: options?.disableNormalization });
	}

	get native() {
		return this[native_];
	}
	private _sampleRate = 0;
	private _closedTime = 0;
	get sampleRate(): number {
		return this._sampleRate;
	}
	get baseLatency(): number {
		if (typeof this.native.baseLatency === 'number') return this.native.baseLatency;
		try {
			const session = AVAudioSession.sharedInstance();
			//@ts-ignore
			return session.IOBufferDuration ?? session.ioBufferDuration ?? 0;
		} catch (e) {
			return 0;
		}
	}
	get outputLatency(): number {
		if (typeof this.native.outputLatency === 'number') return this.native.outputLatency;
		try {
			return AVAudioSession.sharedInstance().outputLatency;
		} catch (e) {
			return 0;
		}
	}
	get currentTime(): number {
		return this.native ? this.native.currentTime : this._closedTime;
	}

	/** The context time last sent to the output, and when (performance.now()). */
	getOutputTimestamp(): { contextTime: number; performanceTime: number } {
		if (!this.native || this._state !== 'running') return { contextTime: this.currentTime, performanceTime: 0 };
		const contextTime = Math.max(0, this.currentTime - this.outputLatency);
		return { contextTime, performanceTime: performance.now() - (this.currentTime - contextTime) * 1000 };
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

	resume(): Promise<void> {
		if (this._state === 'closed') return Promise.reject(domError('InvalidStateError', 'AudioContext is closed'));
		const native = this.native;
		if (!native) return Promise.reject(new Error('AudioContext: native engine unavailable'));
		return new Promise((resolve, reject) => {
			native.resumeAsync((ok) => {
				if (ok) {
					this._setState('running');
					resolve();
				} else {
					reject(new Error('AudioContext.resume: engine failed to start'));
				}
			});
		});
	}

	suspend(): Promise<void> {
		if (this._state === 'closed') return Promise.reject(domError('InvalidStateError', 'AudioContext is closed'));
		const native = this.native;
		if (!native) return Promise.reject(new Error('AudioContext: native engine unavailable'));
		return new Promise((resolve, reject) => {
			native.suspendAsync((ok) => {
				if (ok) {
					this._setState('suspended');
					resolve();
				} else {
					reject(new Error('AudioContext.suspend: engine failed to pause'));
				}
			});
		});
	}

	close(): Promise<void> {
		if (this._state === 'closed') return Promise.resolve();
		this._audioWorklet?._scope(false)?.close();
		connectedWorkletNodes.forEach((node) => {
			if (node.context === this) connectedWorkletNodes.delete(node);
		});
		const native = this.native;
		return new Promise((resolve) => {
			const finish = () => {
				this._closedTime = native?.currentTime ?? 0;
				this[native_] = null as any;
				this._destination = null;
				this._listener = null;
				this._setState('closed');
				resolve();
			};
			if (native) native.closeAsync(finish);
			else finish();
		});
	}

	get sinkId(): string {
		try {
			return this.native.currentSinkId();
		} catch (e) {
			return 'default';
		}
	}

	setSinkId(deviceId: string): Promise<void> {
		return new Promise((resolve, reject) => {
			try {
				const ok = this.native.setSinkId(deviceId ?? 'default');
				if (ok) resolve();
				else reject(new DOMException(`setSinkId(${deviceId}) rejected by AVAudioSession`, 'NotFoundError'));
			} catch (e) {
				reject(e);
			}
		});
	}

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
		const ret = new Promise<AudioBuffer>((resolve, reject) => {
			try {
				if (typeof source === 'string') {
					if (!isBase64Source(source)) {
						const filePath = normalizeSourcePath(source);
						this.native.decodeAudioDataFromFileAsync(filePath, (wrapper) => {
							if (!wrapper) return reject(new Error('decodeAudioData failed'));
							resolve(new AudioBuffer(nativeCtor_, wrapper));
						});
						return;
					}
					this.native.decodeAudioDataAsync(source, (wrapper) => {
						if (!wrapper) return reject(new Error('decodeAudioData failed'));
						resolve(new AudioBuffer(nativeCtor_, wrapper));
					});
					return;
				}

				this.native.decodeAudioDataFromDataAsync(source as never, (wrapper) => {
					if (!wrapper) return reject(new Error('decodeAudioData failed'));
					resolve(new AudioBuffer(nativeCtor_, wrapper));
				});
			} catch (e) {
				reject(e);
			}
		});

		if (successCallback) ret.then(successCallback);
		if (errorCallback) ret.catch(errorCallback);

		return ret;
	}
}

const offlineSuspendErrors: Record<number, string> = {
	[-1]: 'suspendTime must be a non-negative number',
	[-2]: 'rendering has finished',
	[-3]: 'suspendTime is at or past the end of the render',
	[-4]: 'suspendTime is not after the current time',
	[-5]: 'a suspend is already scheduled at that time',
};

export class OfflineAudioContext extends BaseAudioContext {
	private [native_]: NSCOfflineAudioContext;
	destination: AudioDestinationNode | null = null;
	private _listener: AudioListener | null = null;
	private _audioWorklet: AudioWorklet | null = null;
	private _rendering: Promise<AudioBuffer> | null = null;
	private _suspends = new OfflineSuspends();

	constructor(numberOfChannels: number | { numberOfChannels?: number; length: number; sampleRate: number }, lengthInFrames?: number, sampleRate?: number) {
		super();
		const options = typeof numberOfChannels === 'object' ? numberOfChannels : { numberOfChannels, length: lengthInFrames, sampleRate };
		this[native_] = NSCOfflineAudioContext.new();
		this[native_].configure(Math.max(1, (options.numberOfChannels ?? 1) | 0), Math.max(0, options.length | 0), options.sampleRate && Number.isFinite(options.sampleRate) ? options.sampleRate : 48000);
		try {
			const nd = this[native_].destination;
			this.destination = nd ? new AudioDestinationNode(this, nd) : null;
		} catch (e) {
			this.destination = null;
		}
		this._state = 'suspended';
		const ref = new WeakRef(this);
		this[native_].onSuspend = (frame) => ref.deref()?._reached(frame);
	}

	private _reached(frame: number) {
		this._setState('suspended');
		this._suspends.reached(frame);
	}

	/** Holds rendering at `suspendTime`, rounded up to a 128-frame quantum. */
	suspend(suspendTime: number): Promise<void> {
		if (this._state === 'closed') return Promise.reject(domError('InvalidStateError', 'OfflineAudioContext: rendering has finished'));
		const frame = this.native.scheduleSuspend(Number(suspendTime));
		if (frame < 0) return Promise.reject(domError('InvalidStateError', `OfflineAudioContext.suspend: ${offlineSuspendErrors[frame] ?? 'invalid suspendTime'}`));
		return this._suspends.add(frame);
	}

	resume(): Promise<void> {
		if (this._state === 'closed') return Promise.reject(domError('InvalidStateError', 'OfflineAudioContext: rendering has finished'));
		if (!this._rendering) return Promise.reject(domError('InvalidStateError', 'OfflineAudioContext: rendering has not started'));
		this._setState('running');
		this.native.resumeRendering();
		return Promise.resolve();
	}

	get native() {
		return this[native_];
	}

	get sampleRate(): number {
		return this.native.sampleRate;
	}

	get currentTime(): number {
		return this.native.currentTime;
	}

	get length(): number {
		return this.native.length;
	}

	get listener() {
		if (!this._listener) this._listener = new AudioListener(nativeCtor_, this as unknown as NativeBaseAudioContext);
		return this._listener;
	}

	get audioWorklet(): AudioWorklet | undefined {
		if (!this._audioWorklet && isWorkerSupported()) this._audioWorklet = new AudioWorklet(nativeCtor_, this);
		return this._audioWorklet ?? undefined;
	}

	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number } | number, length?: number, sampleRate?: number) {
		return new AudioBuffer(typeof options === 'number' ? { numberOfChannels: options, length, sampleRate } : options);
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
	createBufferSource(options?: { buffer?: AudioBuffer }) {
		return new AudioBufferSourceNode(this, options ?? {});
	}
	createStereoPanner(options?: StereoPannerOptions) {
		return new StereoPannerNode(this, options ?? {});
	}
	createDelay(options?: DelayOptions) {
		return new DelayNode(this, options ?? {});
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
	createChannelSplitter(options?: ChannelSplitterOptions) {
		return new ChannelSplitterNode(this, options ?? {});
	}
	createChannelMerger(options?: ChannelMergerOptions) {
		return new ChannelMergerNode(this, options ?? {});
	}
	createPeriodicWave(real: Float32Array | number[], imag: Float32Array | number[], options?: { disableNormalization?: boolean }) {
		return new PeriodicWave(this, { real, imag, disableNormalization: options?.disableNormalization });
	}

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
		const ret = new Promise<AudioBuffer>((resolve, reject) => {
			try {
				if (typeof source === 'string') {
					if (!isBase64Source(source)) {
						const filePath = normalizeSourcePath(source);
						this.native.decodeAudioDataFromFileAsyncOffline(filePath, (wrapper) => {
							if (!wrapper) return reject(new Error('decodeAudioData failed'));
							resolve(new AudioBuffer(nativeCtor_, wrapper));
						});
						return;
					}
					this.native.decodeAudioDataAsync(source, (wrapper) => {
						if (!wrapper) return reject(new Error('decodeAudioData failed'));
						resolve(new AudioBuffer(nativeCtor_, wrapper));
					});
					return;
				}

				this.native.decodeAudioDataFromDataAsync(source as never, (wrapper) => {
					if (!wrapper) return reject(new Error('decodeAudioData failed'));
					resolve(new AudioBuffer(nativeCtor_, wrapper));
				});
			} catch (e) {
				reject(e);
			}
		});

		if (successCallback) ret.then(successCallback);
		if (errorCallback) ret.catch(errorCallback);

		return ret;
	}

	startRendering(): Promise<AudioBuffer> {
		if (this._rendering) return Promise.reject(domError('InvalidStateError', 'OfflineAudioContext: rendering has already started'));
		this._setState('running');
		this._rendering = new Promise<AudioBuffer>((resolve, reject) => {
			this.native.startRenderingAsync((wrapper: any) => {
				this._audioWorklet?._scope(false)?.close();
				connectedWorkletNodes.forEach((node) => {
					if (node.context === this) connectedWorkletNodes.delete(node);
				});
				this._setState('closed');
				this._suspends.rejectAll(() => domError('InvalidStateError', 'OfflineAudioContext: rendering finished before the suspend'));
				if (!wrapper) return reject(new Error('startRendering failed'));
				resolve(new AudioBuffer(nativeCtor_, wrapper));
			});
		});
		return this._rendering;
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
		if (!this._positionX) {
			this._positionX = new AudioParam(nativeCtor_, this.context.native.getListenerPositionXParam());
		}
		return this._positionX;
	}
	get positionY() {
		if (!this._positionY) {
			this._positionY = new AudioParam(nativeCtor_, this.context.native.getListenerPositionYParam());
		}
		return this._positionY;
	}
	get positionZ() {
		if (!this._positionZ) {
			this._positionZ = new AudioParam(nativeCtor_, this.context.native.getListenerPositionZParam());
		}
		return this._positionZ;
	}

	get forwardX() {
		if (!this._forwardX) {
			this._forwardX = new AudioParam(nativeCtor_, this.context.native.getListenerForwardXParam());
		}
		return this._forwardX;
	}
	get forwardY() {
		if (!this._forwardY) {
			this._forwardY = new AudioParam(nativeCtor_, this.context.native.getListenerForwardYParam());
		}
		return this._forwardY;
	}
	get forwardZ() {
		if (!this._forwardZ) {
			this._forwardZ = new AudioParam(nativeCtor_, this.context.native.getListenerForwardZParam());
		}
		return this._forwardZ;
	}

	get upX() {
		if (!this._upX) {
			this._upX = new AudioParam(nativeCtor_, this.context.native.getListenerUpXParam());
		}
		return this._upX;
	}
	get upY() {
		if (!this._upY) {
			this._upY = new AudioParam(nativeCtor_, this.context.native.getListenerUpYParam());
		}
		return this._upY;
	}
	get upZ() {
		if (!this._upZ) {
			this._upZ = new AudioParam(nativeCtor_, this.context.native.getListenerUpZParam());
		}
		return this._upZ;
	}

	setPosition(x: number, y: number, z: number) {
		this.context.native.setListenerPosition(x, y, z);
	}

	setOrientation(forwardX: number, forwardY: number, forwardZ: number, upX?: number, upY?: number, upZ?: number) {
		this.context.native.setListenerOrientation(forwardX, forwardY, forwardZ, upX ?? 0, upY ?? 1, upZ ?? 0);
	}
}
