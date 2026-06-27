import { Utils } from '@nativescript/core';
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
	makeStubParamHooks,
	assertMediaElementUsable,
	markMediaElementUsed,
	unmarkMediaElementUsed,
	resolveNativePlayer,
	throwInvalidMediaElement,
} from './common';

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
export class AudioParam extends AudioParamBase {
	private [native_]: org.nativescript.audiocontext.AudioParam;

	constructor(a?: any, b?: any) {
		if (a !== nativeCtor_ || !b) {
			throw new TypeError('Illegal constructor.');
		}
		super(makeAndroidHooks(b), b.getValue());
		this[native_] = b;
	}

	get native() {
		return this[native_];
	}

	getValuesForRange(startTime: number, sampleRate: number, frameCount: number): number[] {
		const range = this.native.getValuesForRange(startTime, sampleRate, frameCount);
		if (!range) return [];
		return Utils.dataDeserialize(range);
	}
}

export class AudioNode extends AudioNodeBase {
	constructor(context: NativeBaseAudioContext) {
		super(context);
	}

	connect(node: any, output?: number, input?: number) {
		const that = this as never as AudioNode & { native: org.nativescript.audiocontext.AudioNode };
		const target = node?.native || node;
		try {
			if (typeof output === 'number' && typeof input === 'number') that.native.connect(target, output, input);
			else if (typeof output === 'number') that.native.connect(target, output);
			else that.native.connect(target);
		} catch (e) {
			console.warn('AudioNode.connect failed:', e);
		}

		if (!(node instanceof AudioParamBase)) return node;
	}

	disconnect(destinationOrOutput?: any, output?: number, input?: number) {
		const that = this as never as AudioNode & { native: org.nativescript.audiocontext.AudioNode };
		try {
			if (!destinationOrOutput) {
				that.native.disconnect();
				return;
			}
			if (typeof destinationOrOutput === 'number') {
				try {
					(that.native as any).disconnectOutput(destinationOrOutput);
				} catch (e) {
					console.warn('AudioNode.disconnect(output): falling back to full disconnect:', e);
					that.native.disconnect();
				}
				return;
			}
			const dest = destinationOrOutput.native || destinationOrOutput;
			try {
				if (typeof output === 'number' && typeof input === 'number') (that.native as any).disconnectTo(dest, output, input);
				else if (typeof output === 'number') (that.native as any).disconnectTo(dest, output);
				else (that.native as any).disconnectTo(dest);
			} catch (e) {
				console.warn('AudioNode.disconnect(target): falling back to full disconnect:', e);
				try {
					that.native.disconnect();
				} catch (ee) {
					console.warn('AudioNode.disconnect fallback also failed:', ee);
				}
			}
		} catch (e) {
			console.warn('AudioNode.disconnect failed:', e);
		}
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
		const length = Math.max(0, opts.length | 0);
		const ch = opts.numberOfChannels != null ? Math.max(1, opts.numberOfChannels | 0) : 1;
		const sr = opts.sampleRate != null ? opts.sampleRate | 0 : 48000;
		this[native_] = new org.nativescript.audiocontext.AudioBuffer(length, ch, sr);
	}

	get native() {
		return this[native_];
	}

	get sampleRate(): number {
		return this.native.getSampleRate();
	}
	get length(): number {
		return this.native.getLength();
	}
	get duration(): number {
		return this.native.getDuration();
	}
	get numberOfChannels(): number {
		return this.native.getNumberOfChannels();
	}

	getChannelData(channel: number): Float32Array | null {
		const data = this.native.getChannelDataRaw(channel);
		if (!data) return null;
		return new Float32Array((ArrayBuffer as any).from(data));
	}

	copyFromChannel(dest: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!dest) return;
		const options = resolveAudioBufferCopyOptions(dest, startInChannel);
		const target = viewAtByteOffset(dest, options.byteOffset);
		if (target.length === 0) return;
		this.native.copyFromChannel(target as any, channel, options.startInChannel);
	}

	copyToChannel(source: Float32Array, channel: number, startInChannel: number | AudioBufferCopyOptions = 0) {
		if (!source) return;
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
		if (!this._gainParam) this._gainParam = new AudioParam(nativeCtor_, this.native.getGain());
		return this._gainParam;
	}
}

export class AudioDestinationNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AudioNode;
	constructor(context: NativeBaseAudioContext, node: org.nativescript.audiocontext.GainNode) {
		super(context);
		this[native_] = node;
	}
	get native() {
		return this[native_];
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
	get frequency(): AudioParam {
		if (!this._frequencyParam) this._frequencyParam = new AudioParam(nativeCtor_, this.native.getFrequency());
		return this._frequencyParam;
	}
	get Q(): AudioParam {
		if (!this._qParam) this._qParam = new AudioParam(nativeCtor_, this.native.getQ());
		return this._qParam;
	}
	get gain(): AudioParam {
		if (!this._gainParam) this._gainParam = new AudioParam(nativeCtor_, this.native.getGain());
		return this._gainParam;
	}
	get detune(): AudioParam {
		if (!this._detuneParam) this._detuneParam = new AudioParam(nativeCtor_, this.native.getDetune());
		return this._detuneParam;
	}
}

export class PannerNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AudioPannerNode;

	constructor(context: NativeBaseAudioContext, options: PannerOptions = {}) {
		super(context);
		const p = resolvePannerOptions(options);
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
		return this._positionX || (this._positionX = new AudioParam(nativeCtor_, this.native.getPositionXParam()));
	}
	get positionY() {
		return this._positionY || (this._positionY = new AudioParam(nativeCtor_, this.native.getPositionYParam()));
	}
	get positionZ() {
		return this._positionZ || (this._positionZ = new AudioParam(nativeCtor_, this.native.getPositionZParam()));
	}
	get orientationX() {
		return this._orientationX || (this._orientationX = new AudioParam(nativeCtor_, this.native.getOrientationXParam()));
	}
	get orientationY() {
		return this._orientationY || (this._orientationY = new AudioParam(nativeCtor_, this.native.getOrientationYParam()));
	}
	get orientationZ() {
		return this._orientationZ || (this._orientationZ = new AudioParam(nativeCtor_, this.native.getOrientationZParam()));
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
	}
}

export class AudioBufferSourceNode extends AudioScheduledSourceNode {
	private _playbackRateParam: AudioParam | null = null;
	constructor(context: NativeBaseAudioContext, options: { buffer?: AudioBuffer } = {}) {
		super(context, AudioContext.getInstance().createBufferSource(context.native, options?.buffer?.native ?? (null as never)));
	}
	get native() {
		return this[native_] as org.nativescript.audiocontext.AudioBufferSourceNode;
	}
	get loop(): boolean {
		return this.native.getLoop();
	}
	set loop(v: boolean) {
		this.native.setLoop(v);
	}
	get buffer(): AudioBuffer | null {
		const b = this.native.getBuffer();
		return b ? new AudioBuffer(nativeCtor_, b) : null;
	}
	set buffer(v: AudioBuffer | null) {
		this.native.setBuffer(v?.native ?? (null as never));
	}

	get playbackRate(): AudioParam | null {
		if (this._playbackRateParam) return this._playbackRateParam;
		this._playbackRateParam = new AudioParam(nativeCtor_, this.native.getPlaybackRateParam());
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

	get playbackRate(): AudioParam | null {
		if (this._playbackRateParam) return this._playbackRateParam;
		const native: any = this[native_];
		try {
			const param = typeof native?.getPlaybackRateParam === 'function' ? native.getPlaybackRateParam() : null;
			if (param) {
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

export class OfflineAudioContext extends BaseAudioContext {
	[native_]: org.nativescript.audiocontext.AudioContextInstance;
	destination: AudioDestinationNode | null = null;

	private _frames: number = 0;
	private _sampleRate: number = 48000;
	private _channels: number = 1;

	static getInstance() {
		if (!audioContextInstance) audioContextInstance = org.nativescript.audiocontext.AudioContext.getInstance();
		return audioContextInstance;
	}

	constructor(numberOfChannels: number, lengthInFrames: number, sampleRate: number) {
		super();
		this._frames = Math.max(0, lengthInFrames | 0);
		this._sampleRate = sampleRate && Number.isFinite(sampleRate) ? sampleRate : 48000;
		this._channels = Math.max(1, numberOfChannels | 0);

		this[native_] = OfflineAudioContext.getInstance().createContextInstance();
		this.destination = new AudioDestinationNode(this, this[native_].getDestination());
	}

	get native() {
		return this[native_];
	}

	configure(numberOfChannels: number, lengthInFrames: number, sampleRate: number) {
		this._channels = Math.max(1, numberOfChannels | 0);
		this._frames = Math.max(0, lengthInFrames | 0);
		this._sampleRate = sampleRate && Number.isFinite(sampleRate) ? sampleRate : this._sampleRate;
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
	createDelay(options?: DelayOptions) {
		return new DelayNode(this as never, options ?? {});
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
	createBufferSource() {
		return new AudioBufferSourceNode(this);
	}

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
		const ret = new Promise<AudioBuffer>((resolve, reject) => {
			if (typeof source === 'string') {
				if (looksLikePath(source)) {
					const filePath = normalizeSourcePath(source);
					OfflineAudioContext.getInstance().decodeAudioDataFromFileAsync(
						filePath,
						this.native,
						new org.nativescript.audiocontext.DecodeCallback({
							onResult(buffer) {
								if (!buffer) return reject(new Error('decodeAudioData failed'));
								resolve(new AudioBuffer(nativeCtor_, buffer));
							},
						}),
					);
					return;
				}

				OfflineAudioContext.getInstance().decodeAudioDataAsync(
					source,
					this.native,
					new org.nativescript.audiocontext.DecodeCallback({
						onResult(buffer) {
							if (!buffer) return reject(new Error('decodeAudioData failed'));
							resolve(new AudioBuffer(nativeCtor_, buffer));
						},
					}),
				);
				return;
			}

			OfflineAudioContext.getInstance().decodeAudioDataFromBufferAsync(
				source as never,
				this.native,
				new org.nativescript.audiocontext.DecodeCallback({
					onResult(buffer) {
						if (!buffer) return reject(new Error('decodeAudioData failed'));
						resolve(new AudioBuffer(nativeCtor_, buffer));
					},
				}),
			);
		});

		if (successCallback) ret.then(successCallback);
		if (errorCallback) ret.catch(errorCallback);

		return ret;
	}

	startRendering(): Promise<AudioBuffer> {
		return new Promise((resolve, reject) => {
			this.native.renderOfflineAsync(
				this._frames,
				this._sampleRate,
				this._channels,
				new org.nativescript.audiocontext.DecodeCallback({
					onResult(buffer) {
						if (!buffer) return reject(new Error('startRendering failed'));
						resolve(new AudioBuffer(nativeCtor_, buffer));
					},
				}),
			);
		});
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
		audioContextInstance = org.nativescript.audiocontext.AudioContext.getInstance();
		const sampleRate = options?.sampleRate ?? 0;
		const latencyHintSec = resolveLatencyHint(options?.latencyHint);
		this[native_] = audioContextInstance.createContextInstance(sampleRate, latencyHintSec);

		this.destination = new AudioDestinationNode(this, this[native_].getDestination());
		this._setState('running');
	}

	createMediaElementSource(mediaElement: MediaElementLike): MediaElementAudioSourceNode {
		return new MediaElementAudioSourceNode(this, mediaElement);
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
	createDelay(options?: DelayOptions) {
		return new DelayNode(this as never, options ?? {});
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
	createBuffer(options: { length: number; numberOfChannels: number; sampleRate: number }) {
		return new AudioBuffer(options);
	}
	createBufferSource(options?: { buffer?: AudioBuffer }) {
		return new AudioBufferSourceNode(this, options);
	}

	renderOffline(trackIds: string[], frames: number, sampleRate: number, channels: number): Promise<AudioBuffer> {
		return new Promise((resolve, reject) => {
			AudioContext.getInstance().renderOfflineAsync(
				trackIds,
				frames,
				sampleRate,
				channels,
				new org.nativescript.audiocontext.DecodeCallback({
					onResult(buffer) {
						if (!buffer) return reject(new Error('renderOffline failed'));
						resolve(new AudioBuffer(nativeCtor_, buffer));
					},
				}),
			);
		});
	}

	decodeAudioData(source: string | ArrayBuffer | ArrayBufferView, successCallback?: (buffer: AudioBuffer) => void, errorCallback?: (error: Error) => void): Promise<AudioBuffer> {
		const ret = new Promise<AudioBuffer>((resolve, reject) => {
			if (typeof source === 'string') {
				if (looksLikePath(source)) {
					const filePath = normalizeSourcePath(source);
					AudioContext.getInstance().decodeAudioDataFromFileAsync(
						filePath,
						this.native,
						new org.nativescript.audiocontext.DecodeCallback({
							onResult(buffer) {
								if (!buffer) return reject(new Error('decodeAudioData failed'));
								resolve(new AudioBuffer(nativeCtor_, buffer));
							},
						}),
					);
					return;
				}

				AudioContext.getInstance().decodeAudioDataAsync(
					source,
					this.native,
					new org.nativescript.audiocontext.DecodeCallback({
						onResult(buffer) {
							if (!buffer) return reject(new Error('decodeAudioData failed'));
							resolve(new AudioBuffer(nativeCtor_, buffer));
						},
					}),
				);
				return;
			}

			AudioContext.getInstance().decodeAudioDataFromBufferAsync(
				source as never,
				this.native,
				new org.nativescript.audiocontext.DecodeCallback({
					onResult(buffer) {
						if (!buffer) return reject(new Error('decodeAudioData failed'));
						resolve(new AudioBuffer(nativeCtor_, buffer));
					},
				}),
			);
		});

		if (successCallback) ret.then(successCallback);
		if (errorCallback) ret.catch(errorCallback);

		return ret;
	}

	resume(): Promise<void> {
		if (this._state === 'closed') return Promise.reject(new Error('AudioContext is closed'));
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
		if (this._state === 'closed') return Promise.reject(new Error('AudioContext is closed'));
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
		if (this._state === 'closed') return Promise.resolve();
		return new Promise((resolve, reject) => {
			const ref = new WeakRef(this);
			AudioContext.getInstance().closeAsync(
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

	private _sinkId = 'default';
	get sinkId(): string {
		return this._sinkId;
	}

	setSinkId(deviceId: string): Promise<void> {
		return new Promise((resolve, reject) => {
			try {
				const value = deviceId == null || deviceId === '' ? 'default' : String(deviceId);
				const ok = this.native.setSinkId(value);
				if (ok) {
					this._sinkId = value;
					resolve();
				} else {
					reject(new Error(`setSinkId(${deviceId}) rejected by native engine`));
				}
			} catch (e) {
				reject(e);
			}
		});
	}

	private _listener: AudioListener | null = null;

	get listener() {
		if (!this._listener) this._listener = new AudioListener(nativeCtor_, this as unknown as NativeBaseAudioContext);
		return this._listener;
	}
}

export class OscillatorNode extends AudioScheduledSourceNode {
	private _periodicWave: PeriodicWave | null = null;
	constructor(context: NativeBaseAudioContext, options: { type?: string; frequency?: number } = {}) {
		const type = options?.type || 'sine';
		const freq = options?.frequency || 440;
		super(context, context.native.createOscillatorNodeFrequency(type, freq));
	}

	get native() {
		return this[native_] as org.nativescript.audiocontext.AudioOscillatorNode;
	}

	private _frequencyParam!: AudioParam;
	get frequency(): AudioParam {
		if (!this._frequencyParam) this._frequencyParam = new AudioParam(nativeCtor_, this.native.getFrequency());
		return this._frequencyParam;
	}

	setPeriodicWave(wave: PeriodicWave) {
		if (!wave) return;
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
		AudioContext.getInstance().setPannerParams(node, p.positionX, p.positionY, p.positionZ, p.orientationX, p.orientationY, p.orientationZ, p.pan, p.distanceModel, p.panningModel, p.refDistance, p.maxDistance, p.rolloffFactor, p.coneInnerAngle, p.coneOuterAngle, p.coneOuterGain);
	}
	get native() {
		return this[native_];
	}
	get pan(): AudioParam {
		if (!this._panParam) {
			this._panParam = new AudioParam(nativeCtor_, this.native.getPan());
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
		this.maxDelayTime = typeof options.maxDelayTime === 'number' ? options.maxDelayTime : 1.0;
		const delay = AudioContext.getInstance().createDelay(context.native);
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
			this._delayTimeParam = new AudioParam(nativeCtor_, param);
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
			this._offsetParam = new AudioParam(nativeCtor_, this.native.getOffsetParam());
		}
		return this._offsetParam;
	}
}

export class AnalyserNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.AnalyserNode;
	constructor(context: NativeBaseAudioContext, options: AnalyserOptions = {}) {
		super(context);
		this[native_] = AudioContext.getInstance().createAnalyser(context.native);
		if (typeof options.fftSize === 'number') this.native.setFftSize(options.fftSize | 0);
		if (typeof options.smoothingTimeConstant === 'number') this.native.setSmoothingTimeConstant(options.smoothingTimeConstant);
		if (typeof options.minDecibels === 'number') this.native.setMinDecibels(options.minDecibels);
		if (typeof options.maxDecibels === 'number') this.native.setMaxDecibels(options.maxDecibels);
	}
	get native() {
		return this[native_];
	}
	get fftSize() {
		return this.native.getFftSize();
	}
	set fftSize(v: number) {
		this.native.setFftSize(v | 0);
	}
	get frequencyBinCount() {
		return (this.native.getFftSize() / 2) | 0;
	}
	get smoothingTimeConstant() {
		return this.native.getSmoothingTimeConstant();
	}
	set smoothingTimeConstant(v: number) {
		this.native.setSmoothingTimeConstant(v);
	}
	get minDecibels() {
		return this.native.getMinDecibels();
	}
	set minDecibels(v: number) {
		this.native.setMinDecibels(v);
	}
	get maxDecibels() {
		return this.native.getMaxDecibels();
	}
	set maxDecibels(v: number) {
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
		this.native.setCurveFromData(v ?? (null as any));
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
		if (!options?.feedforward?.length || !options?.feedback?.length) throw new TypeError('IIRFilterNode: feedforward and feedback are required');
		this[native_] = AudioContext.getInstance().createIIR(context.native, options.feedforward as never, options.feedback as never);
	}
	get native() {
		return this[native_];
	}
	getFrequencyResponse(frequencyHz: Float32Array, magResponse: Float32Array, phaseResponse: Float32Array) {
		if (!frequencyHz || !magResponse || !phaseResponse) return;
		this.native.getFrequencyResponse(frequencyHz, magResponse, phaseResponse);
	}
}

export class ConvolverNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.ConvolverNode;
	constructor(context: NativeBaseAudioContext, options: ConvolverOptions = {}) {
		super(context);
		this[native_] = AudioContext.getInstance().createConvolver(context.native);
		this.normalize = !options.disableNormalization;
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
	private _reduction: AudioParam | null = null;

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
		return this._threshold || (this._threshold = new AudioParam(nativeCtor_, this.native.getThreshold()));
	}

	get knee() {
		return this._knee || (this._knee = new AudioParam(nativeCtor_, this.native.getKnee()));
	}

	get ratio() {
		return this._ratio || (this._ratio = new AudioParam(nativeCtor_, this.native.getRatio()));
	}

	get attack() {
		return this._attack || (this._attack = new AudioParam(nativeCtor_, this.native.getAttack()));
	}

	get release() {
		return this._release || (this._release = new AudioParam(nativeCtor_, this.native.getRelease()));
	}

	get reduction() {
		return this._reduction || (this._reduction = new AudioParam(nativeCtor_, this.native.getReduction()));
	}
}

export class ChannelSplitterNode extends AudioNode {
	[native_]: org.nativescript.audiocontext.ChannelSplitterNode;
	private _numberOfOutputs: number;

	constructor(context: NativeBaseAudioContext, options: ChannelSplitterOptions = {}) {
		super(context);
		this._numberOfOutputs = Math.max(1, options.numberOfOutputs ?? 6);
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
		this._numberOfInputs = Math.max(1, options.numberOfInputs ?? 6);
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
		this.real = options.real ? (options.real instanceof Float32Array ? options.real : Float32Array.from(options.real)) : new Float32Array(2);
		this.imag = options.imag ? (options.imag instanceof Float32Array ? options.imag : Float32Array.from(options.imag)) : new Float32Array(this.real.length);
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
		return this._positionX || (this._positionX = new AudioParam(nativeCtor_, this.context.native.getListenerPositionXParam()));
	}
	get positionY() {
		return this._positionY || (this._positionY = new AudioParam(nativeCtor_, this.context.native.getListenerPositionYParam()));
	}
	get positionZ() {
		return this._positionZ || (this._positionZ = new AudioParam(nativeCtor_, this.context.native.getListenerPositionZParam()));
	}

	get forwardX() {
		return this._forwardX || (this._forwardX = new AudioParam(nativeCtor_, this.context.native.getListenerForwardXParam()));
	}
	get forwardY() {
		return this._forwardY || (this._forwardY = new AudioParam(nativeCtor_, this.context.native.getListenerForwardYParam()));
	}
	get forwardZ() {
		return this._forwardZ || (this._forwardZ = new AudioParam(nativeCtor_, this.context.native.getListenerForwardZParam()));
	}

	get upX() {
		return this._upX || (this._upX = new AudioParam(nativeCtor_, this.context.native.getListenerUpXParam()));
	}
	get upY() {
		return this._upY || (this._upY = new AudioParam(nativeCtor_, this.context.native.getListenerUpYParam()));
	}
	get upZ() {
		return this._upZ || (this._upZ = new AudioParam(nativeCtor_, this.context.native.getListenerUpZParam()));
	}

	setPosition(x: number, y: number, z: number) {
		this.context.native.setListenerParams(x, y, z, this.forwardX.value, this.forwardY.value, this.forwardZ.value, this.upX.value, this.upY.value, this.upZ.value);
	}

	setOrientation(forwardX: number, forwardY: number, forwardZ: number, upX?: number, upY?: number, upZ?: number) {
		this.context.native.setListenerParams(this.positionX.value, this.positionY.value, this.positionZ.value, forwardX, forwardY, forwardZ, upX ?? 0, upY ?? 1, upZ ?? 0);
	}
}
