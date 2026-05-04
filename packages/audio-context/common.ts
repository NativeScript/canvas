import { knownFolders, Observable } from '@nativescript/core';

export const native_ = Symbol('[[native]]');
export const context_ = Symbol('[[context]]');
export const nativeCtor_ = Symbol('[[nativeConstructor]]');

export type AudioContextState = 'suspended' | 'running' | 'closed';

export class AudioEventTarget extends Observable {
	addEventListener(type: string, callback: any, thisArg?: any) {
		if (typeof thisArg === 'boolean') thisArg = { capture: thisArg };
		let listener: any = callback;
		if (callback && typeof callback === 'object') {
			listener = callback[`on${type}`];
		}
		if (typeof listener !== 'function') return;
		const wasListening = this.hasListeners(type);
		super.addEventListener(type, listener, thisArg);
		if (!wasListening) this._onFirstListenerAdded?.(type);
	}

	removeEventListener(type: string, callback?: any, thisArg?: any) {
		if (typeof thisArg === 'boolean') thisArg = { capture: thisArg };
		let listener: any = callback;
		if (callback && typeof callback === 'object') {
			listener = callback[`on${type}`];
		}
		if (typeof listener === 'function') {
			super.removeEventListener(type, listener, thisArg);
		} else {
			super.removeEventListener(type);
		}
		if (!this.hasListeners(type)) this._onLastListenerRemoved?.(type);
	}

	dispatchEvent(event: { type: string; [k: string]: any }): boolean {
		this.notify({ ...event, eventName: event.type, object: this });
		return true;
	}

	protected _onFirstListenerAdded?(type: string): void;

	protected _onLastListenerRemoved?(type: string): void;
}

export class BaseAudioContext extends AudioEventTarget {
	protected _state: AudioContextState = 'running';
	private _onstatechange: ((ev: { type: 'statechange' }) => void) | null = null;

	get state(): AudioContextState {
		return this._state;
	}

	get onstatechange(): ((ev: { type: 'statechange' }) => void) | null {
		return this._onstatechange;
	}

	set onstatechange(cb: ((ev: { type: 'statechange' }) => void) | null) {
		if (this._onstatechange) super.removeEventListener('statechange', this._onstatechange);
		this._onstatechange = typeof cb === 'function' ? cb : null;
		if (this._onstatechange) super.addEventListener('statechange', this._onstatechange);
	}

	protected _setState(next: AudioContextState) {
		if (this._state === next) return;
		this._state = next;
		this.dispatchEvent({ type: 'statechange', target: this });
	}
}

export class AudioNodeBase extends AudioEventTarget {
	protected [context_]: BaseAudioContext;

	constructor(context: BaseAudioContext) {
		super();
		if (!(context instanceof BaseAudioContext)) {
			throw new TypeError(`${this.constructor.name} constructor: Argument 1 does not implement interface BaseAudioContext.`);
		}
		this[context_] = context;
	}

	get context(): BaseAudioContext {
		return this[context_];
	}
}

export function normalizeSourcePath(source: string) {
	let filePath = source;
	if (filePath.startsWith('~/')) {
		filePath = filePath.replace('~/', knownFolders.currentApp().path + '/');
	}
	if (filePath.startsWith('file://')) {
		filePath = filePath.replace('file://', '');
	}
	return filePath;
}

export function looksLikePath(source: string): boolean {
	return source.indexOf('/') >= 0 || source.indexOf('file:') === 0 || source.indexOf('~') === 0;
}

export type LatencyHint = 'interactive' | 'balanced' | 'playback' | number;

export interface AudioContextOptions {
	sampleRate?: number;
	latencyHint?: LatencyHint;
}

export interface AudioBufferCopyOptions {
	startInChannel?: number;
	byteOffset?: number;
}

const LATENCY_HINT_SECONDS = {
	interactive: 0.005,
	balanced: 0.012,
	playback: 0.025,
} as const;

export function resolveLatencyHint(hint: LatencyHint | undefined): number {
	if (hint == null) return 0;
	if (typeof hint === 'number') return Math.max(0, hint);
	return LATENCY_HINT_SECONDS[hint] ?? LATENCY_HINT_SECONDS.balanced;
}

export interface PannerOptions {
	positionX?: number;
	positionY?: number;
	positionZ?: number;
	orientationX?: number;
	orientationY?: number;
	orientationZ?: number;
	pan?: number;
	distanceModel?: DistanceModelType | number;
	panningModel?: PanningModelType | number;
	refDistance?: number;
	maxDistance?: number;
	rolloffFactor?: number;
	coneInnerAngle?: number;
	coneOuterAngle?: number;
	coneOuterGain?: number;
}

export interface ResolvedPannerOptions {
	positionX: number;
	positionY: number;
	positionZ: number;
	orientationX: number;
	orientationY: number;
	orientationZ: number;
	pan: number;
	distanceModel: number;
	panningModel: number;
	refDistance: number;
	maxDistance: number;
	rolloffFactor: number;
	coneInnerAngle: number;
	coneOuterAngle: number;
	coneOuterGain: number;
}

export function resolvePannerOptions(o: PannerOptions = {}): ResolvedPannerOptions {
	return {
		positionX: typeof o.positionX === 'number' ? o.positionX : 0,
		positionY: typeof o.positionY === 'number' ? o.positionY : 0,
		positionZ: typeof o.positionZ === 'number' ? o.positionZ : 0,
		orientationX: typeof o.orientationX === 'number' ? o.orientationX : 1.0,
		orientationY: typeof o.orientationY === 'number' ? o.orientationY : 0.0,
		orientationZ: typeof o.orientationZ === 'number' ? o.orientationZ : 0.0,
		pan: typeof o.pan === 'number' ? o.pan : 0.0,
		distanceModel: distanceModelToNumber(o.distanceModel ?? 'inverse'),
		panningModel: panningModelToNumber(o.panningModel ?? 'equalpower'),
		refDistance: typeof o.refDistance === 'number' ? o.refDistance : 1.0,
		maxDistance: typeof o.maxDistance === 'number' ? o.maxDistance : 10000.0,
		rolloffFactor: typeof o.rolloffFactor === 'number' ? o.rolloffFactor : 1.0,
		coneInnerAngle: typeof o.coneInnerAngle === 'number' ? o.coneInnerAngle : 360.0,
		coneOuterAngle: typeof o.coneOuterAngle === 'number' ? o.coneOuterAngle : 360.0,
		coneOuterGain: typeof o.coneOuterGain === 'number' ? o.coneOuterGain : 0.0,
	};
}

export interface StereoPannerOptions {
	pan?: number;
}
export interface ConstantSourceOptions {
	offset?: number;
}
export interface DelayOptions {
	delayTime?: number;
	maxDelayTime?: number;
}
export interface AnalyserOptions {
	fftSize?: number;
	smoothingTimeConstant?: number;
	minDecibels?: number;
	maxDecibels?: number;
}
export interface WaveShaperOptions {
	curve?: Float32Array | number[] | null;
	oversample?: 'none' | '2x' | '4x';
}
export interface IIRFilterOptions {
	feedforward: number[];
	feedback: number[];
}
export interface ConvolverOptions {
	disableNormalization?: boolean;
}
export interface DynamicsCompressorOptions {
	threshold?: number;
	knee?: number;
	ratio?: number;
	attack?: number;
	release?: number;
}
export interface ChannelSplitterOptions {
	numberOfOutputs?: number;
}
export interface ChannelMergerOptions {
	numberOfInputs?: number;
}
export interface PeriodicWaveOptions {
	real?: Float32Array | number[];
	imag?: Float32Array | number[];
	disableNormalization?: boolean;
}

export interface AudioParamHooks {
	nativeSet(value: number): void;
	nativeScheduleSet(value: number, time: number): void;
	nativeScheduleLinearRamp(value: number, time: number): void;
	nativeCancel(time: number): void;
	nativeScheduleExpRamp?(value: number, time: number): void;
	nativeScheduleTarget?(value: number, startTime: number, timeConstant: number): void;
	nativeScheduleCurve?(values: Float32Array, startTime: number, duration: number): void;
	nativeGetValue?(): number;
	nativeSetAutomationRate?(rate: string): void;
	nativeCancelAndHold(heldValue: number, time: number): void;
}

export function makeStubParamHooks(): AudioParamHooks {
	let current = 0;
	return {
		nativeSet(v) {
			current = v;
		},
		nativeScheduleSet(v) {
			current = v;
		},
		nativeScheduleLinearRamp(v) {
			current = v;
		},
		nativeCancel() {},
		nativeCancelAndHold(v) {
			current = v;
		},
		nativeGetValue() {
			return current;
		},
		nativeSetAutomationRate() {},
	};
}

export class AudioParamBase {
	private readonly hooks: AudioParamHooks;
	private _cachedValue: number;
	private _automationRate: string = 'a-rate';

	constructor(hooks: AudioParamHooks, initialValue = 0) {
		this.hooks = hooks;
		this._cachedValue = initialValue;
	}

	get value(): number {
		return this._cachedValue;
	}

	set value(v: number) {
		const nv = +v;
		if (nv === this._cachedValue) return;
		this._cachedValue = nv;
		this.hooks.nativeSet(nv);
	}

	protected _syncFromNative() {
		if (this.hooks.nativeGetValue) {
			const v = this.hooks.nativeGetValue();
			if (Number.isFinite(v)) this._cachedValue = v;
		}
	}

	get automationRate(): string {
		return this._automationRate;
	}

	set automationRate(v: string) {
		this._automationRate = v;
		this.hooks.nativeSetAutomationRate?.(v);
	}

	setValueAtTime(value: number, time: number = 0): AudioParamBase {
		const nv = +value;
		this._cachedValue = nv;
		this.hooks.nativeScheduleSet(nv, time || 0);
		return this;
	}

	linearRampToValueAtTime(value: number, time: number): AudioParamBase {
		const nv = +value;
		this._cachedValue = nv;
		this.hooks.nativeScheduleLinearRamp(nv, time);
		return this;
	}

	exponentialRampToValueAtTime(value: number, time: number): AudioParamBase {
		const nv = +value;
		if (nv <= 0) throw new RangeError('exponentialRampToValueAtTime: value must be > 0');
		this._cachedValue = nv;
		if (this.hooks.nativeScheduleExpRamp) {
			this.hooks.nativeScheduleExpRamp(nv, time);
		} else {
			this.hooks.nativeScheduleLinearRamp(nv, time);
		}
		return this;
	}

	setTargetAtTime(target: number, startTime: number, timeConstant: number): AudioParamBase {
		const nv = +target;
		if (timeConstant < 0) throw new RangeError('setTargetAtTime: timeConstant must be >= 0');
		this._cachedValue = nv;
		if (this.hooks.nativeScheduleTarget) {
			this.hooks.nativeScheduleTarget(nv, startTime, timeConstant);
		} else {
			const end = startTime + Math.max(0.001, timeConstant * 5);
			this.hooks.nativeScheduleLinearRamp(nv, end);
		}
		return this;
	}

	setValueCurveAtTime(values: Float32Array | number[], startTime: number, duration: number): AudioParamBase {
		if (!values || (values as { length: number }).length < 2) {
			throw new RangeError('setValueCurveAtTime: values must have length >= 2');
		}
		const f32 = values instanceof Float32Array ? values : Float32Array.from(values);
		this._cachedValue = f32[f32.length - 1];
		if (this.hooks.nativeScheduleCurve) {
			this.hooks.nativeScheduleCurve(f32, startTime, duration);
			return this;
		}

		const n = f32.length;
		const step = duration / (n - 1);
		this.hooks.nativeScheduleSet(f32[0], startTime);
		for (let i = 1; i < n; i++) {
			this.hooks.nativeScheduleLinearRamp(f32[i], startTime + step * i);
		}
		return this;
	}

	cancelScheduledValues(time: number): AudioParamBase {
		this.hooks.nativeCancel(time || 0);
		return this;
	}

	cancelAndHoldAtTime(time: number): AudioParamBase {
		const t = time || 0;
		const held = this._cachedValue;
		this.hooks.nativeCancelAndHold(held, t);
		return this;
	}
}

export class AudioListenerBase {
	protected [context_]: BaseAudioContext;
	protected _positionX: AudioParamBase | null = null;
	protected _positionY: AudioParamBase | null = null;
	protected _positionZ: AudioParamBase | null = null;
	protected _forwardX: AudioParamBase | null = null;
	protected _forwardY: AudioParamBase | null = null;
	protected _forwardZ: AudioParamBase | null = null;
	protected _upX: AudioParamBase | null = null;
	protected _upY: AudioParamBase | null = null;
	protected _upZ: AudioParamBase | null = null;

	constructor(context: BaseAudioContext) {
		if (!(context instanceof BaseAudioContext)) throw new TypeError('AudioListener constructor: invalid BaseAudioContext');
		this[context_] = context;
	}

	get positionX() {
		return this._positionX!;
	}
	get positionY() {
		return this._positionY!;
	}
	get positionZ() {
		return this._positionZ!;
	}

	get forwardX() {
		return this._forwardX!;
	}
	get forwardY() {
		return this._forwardY!;
	}
	get forwardZ() {
		return this._forwardZ!;
	}

	get upX() {
		return this._upX!;
	}
	get upY() {
		return this._upY!;
	}
	get upZ() {
		return this._upZ!;
	}

	setPosition(x: number, y: number, z: number) {
		const nx = +x;
		const ny = +y;
		const nz = +z;
		this.positionX.value = nx;
		this.positionY.value = ny;
		this.positionZ.value = nz;
	}

	setOrientation(forwardX: number, forwardY: number, forwardZ: number, upX?: number, upY?: number, upZ?: number) {
		const fx = +forwardX;
		const fy = +forwardY;
		const fz = +forwardZ;
		const ux = typeof upX === 'number' ? +upX : 0;
		const uy = typeof upY === 'number' ? +upY : 1;
		const uz = typeof upZ === 'number' ? +upZ : 0;
		this.forwardX.value = fx;
		this.forwardY.value = fy;
		this.forwardZ.value = fz;
		this.upX.value = ux;
		this.upY.value = uy;
		this.upZ.value = uz;
	}
}

export type DistanceModelType = 'linear' | 'inverse' | 'exponential';
export type PanningModelType = 'equalpower' | 'HRTF';

const DISTANCE_MODEL_NAMES: DistanceModelType[] = ['inverse', 'linear', 'exponential'];
const PANNING_MODEL_NAMES: PanningModelType[] = ['equalpower', 'HRTF'];

export function distanceModelToNumber(value: DistanceModelType | number | null | undefined): number {
	if (typeof value === 'number') return value;
	if (typeof value !== 'string') return 0;
	const i = DISTANCE_MODEL_NAMES.indexOf(value.toLowerCase() as DistanceModelType);
	return i === -1 ? 0 : i;
}

export function distanceModelFromNumber(value: number): DistanceModelType {
	return DISTANCE_MODEL_NAMES[value | 0] ?? 'inverse';
}

export function panningModelToNumber(value: PanningModelType | number | null | undefined): number {
	if (typeof value === 'number') return value;
	if (typeof value !== 'string') return 0;
	const normalized = value.toLowerCase();
	if (normalized === 'equalpower') return 0;
	if (normalized === 'hrtf') return 1;
	return 0;
}

export function panningModelFromNumber(value: number): PanningModelType {
	return PANNING_MODEL_NAMES[value | 0] ?? 'equalpower';
}

export interface MediaElementTapProvider {
	attachAudioContextTap?(contextNative: any): any;
	detachAudioContextTap?(): void;
}

export interface MediaElementLike extends MediaElementTapProvider {
	src?: string;
	currentTime?: number;
	loop?: boolean;
	volume?: number;
	play?(): Promise<void> | void;
	pause?(): void;
	addEventListener?(type: string, cb: Function, opts?: any): void;
	removeEventListener?(type: string, cb: Function, opts?: any): void;
	[k: string]: any;
}

const usedMediaElements: WeakSet<MediaElementLike> = new WeakSet();

export function resolveNativePlayer(mediaElement: MediaElementLike): any {
	const m = mediaElement as any;
	return m?.helper?.player ?? m?.player ?? m?._player ?? m?._audio ?? null;
}

export function assertMediaElementUsable(context: BaseAudioContext, mediaElement: MediaElementLike): void {
	if (!(context instanceof BaseAudioContext)) {
		throw new TypeError('MediaElementAudioSourceNode: invalid context');
	}
	if (!mediaElement || (typeof mediaElement.play !== 'function' && typeof mediaElement.src !== 'string')) {
		throw new TypeError('MediaElementAudioSourceNode: requires a media element with `src` and `play()`');
	}
	if (usedMediaElements.has(mediaElement)) {
		if (typeof DOMException !== 'undefined') {
			throw new DOMException('The media element has already been connected to an AudioContext', 'InvalidStateError');
		}
		throw new Error('The media element has already been connected to an AudioContext');
	}
}

export function markMediaElementUsed(mediaElement: MediaElementLike): void {
	try {
		usedMediaElements.add(mediaElement);
	} catch (e) {}
}

export function unmarkMediaElementUsed(mediaElement: MediaElementLike): void {
	try {
		usedMediaElements.delete(mediaElement);
	} catch (e) {}
}

export function throwInvalidMediaElement(): never {
	if (typeof DOMException !== 'undefined') {
		throw new DOMException('Failed to attach a native source for the media element', 'InvalidStateError');
	}
	throw new Error('Failed to attach a native source for the media element');
}
