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
	looksLikePath,
	native_,
	nativeCtor_,
	normalizeSourcePath,
	resolveLatencyHint,
	resolvePannerOptions,
	AudioListenerBase,
	assertMediaElementUsable,
	markMediaElementUsed,
	unmarkMediaElementUsed,
	resolveNativePlayer,
	throwInvalidMediaElement,
} from './common';

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
		nativeGetValue() {
			return native.value;
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

export class AudioNode extends AudioNodeBase {
	constructor(context: NativeBaseAudioContext) {
		super(context);
	}

	connect(node: any, output?: number, input?: number) {
		const that = this as never as AudioNode & { native: NSCAudioNode };
		const target = node?.native || node;
		try {
			if (typeof output === 'number' && typeof input === 'number') that.native.connectToOutputInput(target, output, input);
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
				if (typeof output === 'number' && typeof input === 'number') that.native.disconnectToOutputInput(dest, output, input);
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
			const length = Math.max(0, opts.length | 0);
			const ch = opts.numberOfChannels != null ? Math.max(1, opts.numberOfChannels | 0) : 1;
			const sr = opts.sampleRate != null ? opts.sampleRate : 48000;

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
		if (channel < 0 || channel >= this.numberOfChannels) return null;
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
		const native = this.native;
		const options = resolveAudioBufferCopyOptions(dest, startInChannel);
		native.copyFromChannelWithByteOffsetByteLength(dest.buffer as any, channel, options.startInChannel, options.byteOffset, dest.byteLength);
	}

	copyToChannel(source: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!source) return;
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
		this._initialGain = typeof options?.gain === 'number' ? options!.gain! : (this._native.gain ?? 1);
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

	start() {
		this._endedFired = false;
		this.native.start();
	}

	stop() {
		this.native.stop();
		setTimeout(() => this._fireEnded(), 0);
	}
}

export class OscillatorNode extends AudioScheduledSourceNode {
	constructor(context: NativeBaseAudioContext, options: { type?: string; frequency?: number } = {}) {
		const type = options?.type || 'sine';
		const freq = options?.frequency || 440;
		super(context, context.native.createOscillatorNodeFrequency(type, freq));
	}

	get native() {
		return this[native_] as NSCAudioOscillatorNode;
	}

	private _frequencyParam!: AudioParam;
	get frequency(): AudioParam {
		if (!this._frequencyParam) this._frequencyParam = new AudioParam(nativeCtor_, this.native.frequencyParam);
		return this._frequencyParam;
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
		this[native_] = context.native.createDelayNodeMaxDelayTime(delay, max);
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
		if (typeof options.fftSize === 'number') this.native.fftSize = options.fftSize;
		if (typeof options.smoothingTimeConstant === 'number') this.native.smoothingTimeConstant = options.smoothingTimeConstant;
		if (typeof options.minDecibels === 'number') this.native.minDecibels = options.minDecibels;
		if (typeof options.maxDecibels === 'number') this.native.maxDecibels = options.maxDecibels;
	}
	get native() {
		return this[native_];
	}
	get fftSize() {
		return this.native.fftSize;
	}
	set fftSize(v: number) {
		this.native.fftSize = v;
	}
	get frequencyBinCount() {
		return this.native.frequencyBinCount;
	}
	get smoothingTimeConstant() {
		return this.native.smoothingTimeConstant;
	}
	set smoothingTimeConstant(v: number) {
		this.native.smoothingTimeConstant = v;
	}
	get minDecibels() {
		return this.native.minDecibels;
	}
	set minDecibels(v: number) {
		this.native.minDecibels = v;
	}
	get maxDecibels() {
		return this.native.maxDecibels;
	}
	set maxDecibels(v: number) {
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
	}
	get native() {
		return this[native_];
	}
	get curve(): Float32Array | null {
		return this._curve;
	}
	set curve(v: Float32Array | null) {
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
		if (!ff.length || !fb.length) throw new TypeError('IIRFilterNode: feedforward and feedback are required');
		this[native_] = context.native.createIIRFilterNodeFeedback(ff as never, fb as never);
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
		this.native.normalize = !options.disableNormalization;
	}
	get native() {
		return this[native_];
	}
	get buffer(): AudioBuffer | null {
		const b = this.native.buffer;
		return b ? new AudioBuffer(nativeCtor_, b) : null;
	}
	set buffer(v: AudioBuffer | null) {
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
		this._numberOfOutputs = Math.max(1, options.numberOfOutputs ?? 6);
		this[native_] = context.native.createChannelSplitterNode(this._numberOfOutputs);
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
		this._numberOfInputs = Math.max(1, options.numberOfInputs ?? 6);
		this[native_] = context.native.createChannelMergerNode(this._numberOfInputs);
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
	constructor(context: NativeBaseAudioContext, options: { buffer?: AudioBuffer } = {}) {
		const param: NSCAudioBuffer | null = options?.buffer != null ? options.buffer.native : null;
		super(context, context.native.createBufferSourceNode(param as never));
	}

	get native() {
		return this[native_] as NSCAudioBufferSourceNode;
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
		this.native.buffer = v?.native ?? (null as never);
	}

	get playbackRate(): AudioParam | null {
		if (this._playbackRateParam) return this._playbackRateParam;
		this._playbackRateParam = new AudioParam(nativeCtor_, this.native.getPlaybackRateParam());
		return this._playbackRateParam;
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

export class AudioContext extends BaseAudioContext {
	private [native_]: NSCAudioContext;

	private _destination: AudioDestinationNode | null = null;

	private _listener: AudioListener | null = null;

	constructor(options?: AudioContextOptions) {
		super();
		const sampleRate = options?.sampleRate ?? 48000;
		const latencyHint = resolveLatencyHint(options?.latencyHint);
		this[native_] = NSCAudioContext.alloc().initWithSampleRateLatencyHint(sampleRate, latencyHint);
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
	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number }) {
		return new AudioBuffer(options);
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
	get sampleRate(): number {
		return this.native.sampleRate;
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
		return this.native.currentTime;
	}

	resume(): Promise<void> {
		if (this._state === 'closed') return Promise.reject(new Error('AudioContext is closed'));
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
		if (this._state === 'closed') return Promise.reject(new Error('AudioContext is closed'));
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
		const native = this.native;
		return new Promise((resolve) => {
			const finish = () => {
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
					if (looksLikePath(source)) {
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

export class OfflineAudioContext extends BaseAudioContext {
	private [native_]: NSCOfflineAudioContext;
	destination: AudioDestinationNode | null = null;

	constructor(numberOfChannels: number, lengthInFrames: number, sampleRate: number) {
		super();
		this[native_] = NSCOfflineAudioContext.new();
		this[native_].configure(numberOfChannels, lengthInFrames, sampleRate);
		try {
			const nd = this[native_].destination;
			this.destination = nd ? new AudioDestinationNode(this, nd) : null;
		} catch (e) {
			this.destination = null;
		}
	}

	get native() {
		return this[native_];
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
					if (looksLikePath(source)) {
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
		return new Promise((resolve, reject) => {
			this.native.startRenderingAsync((wrapper: any) => {
				if (!wrapper) return reject(new Error('startRendering failed'));
				resolve(new AudioBuffer(nativeCtor_, wrapper));
			});
		});
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
