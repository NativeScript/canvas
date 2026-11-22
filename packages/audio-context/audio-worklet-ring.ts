// A worklet node's quantum ring in a SharedArrayBuffer, for runtimes that share one with native
// code (iOS, Android). Same slot layout as crates/audio-context-napi/src/worklet.rs.

export const WORKLET_QUANTUM = 128;
export const WORKLET_SLOTS = 64;
export const WORKLET_MAX_INPUT_CHANNELS = 8;
// u32 words after the slots: requested, done, read, closed, released.
export const WORKLET_HEADER_WORDS = 8;

export interface WorkletRingLayout {
	quantum: number;
	slots: number;
	maxInputChannels: number;
	inputs: number;
	outputCapacity: number[];
	params: number;
	alive: number;
	inputCounts: number;
	outputCounts: number;
	paramLengths: number;
	inputData: number;
	outputData: number[];
	paramData: number;
	slotWords: number;
	header: number;
}

const align4 = (words: number) => (words + 3) & ~3;

export function workletRingLayout(inputs: number, outputCapacity: number[], params: number): WorkletRingLayout {
	const inputCounts = 4;
	const outputCounts = inputCounts + inputs;
	const paramLengths = outputCounts + outputCapacity.length;
	const inputData = align4(paramLengths + params);
	let next = inputData + inputs * WORKLET_MAX_INPUT_CHANNELS * WORKLET_QUANTUM;
	const outputData = outputCapacity.map((channels) => {
		const offset = next;
		next += channels * WORKLET_QUANTUM;
		return offset;
	});
	const paramData = next;
	const slotWords = align4(paramData + params * WORKLET_QUANTUM);
	return {
		quantum: WORKLET_QUANTUM,
		slots: WORKLET_SLOTS,
		maxInputChannels: WORKLET_MAX_INPUT_CHANNELS,
		inputs,
		outputCapacity,
		params,
		alive: 2,
		inputCounts,
		outputCounts,
		paramLengths,
		inputData,
		outputData,
		paramData,
		slotWords,
		header: slotWords * WORKLET_SLOTS,
	};
}

export function createWorkletRing(layout: WorkletRingLayout): SharedArrayBuffer {
	return new SharedArrayBuffer((layout.header + WORKLET_HEADER_WORDS) * 4);
}

// Worker-side bridge over the shared rings: prepended to AUDIO_WORKLET_SCOPE. The render thread
// can't wake a Worker here, so it polls.
export const SHARED_RING_BRIDGE = String.raw`(function (global) {
	var REQUESTED = 0, DONE = 1, CLOSED = 3, RELEASED = 4;
	function Ring(ring) {
		this._layout = ring.layout;
		this._buffer = ring.buffer;
		this._h = new Uint32Array(ring.buffer, ring.layout.header * 4, 8);
	}
	Ring.prototype.layout = function () { return this._layout; };
	Ring.prototype.buffer = function () { return this._buffer; };
	Ring.prototype.pending = function () { return (Atomics.load(this._h, REQUESTED) - Atomics.load(this._h, DONE)) >>> 0; };
	Object.defineProperty(Ring.prototype, 'cursor', { get: function () { return Atomics.load(this._h, DONE) % this._layout.slots; } });
	Object.defineProperty(Ring.prototype, 'closed', { get: function () { return Atomics.load(this._h, CLOSED) !== 0; } });
	Ring.prototype.commit = function (count) { Atomics.add(this._h, DONE, Math.min(count, this.pending())); };
	Ring.prototype.release = function () { Atomics.store(this._h, RELEASED, 1); };

	function Bridge(drain) {
		this.rings = new Map();
		this.timer = setInterval(drain, 2);
	}
	Bridge.prototype.adopt = function (id, ring) { this.rings.set(id, new Ring(ring)); };
	Bridge.prototype.node = function (id) { return this.rings.get(id) || null; };
	Bridge.prototype.beginDrain = function () {};
	Bridge.prototype.close = function () {
		clearInterval(this.timer);
		this.rings.clear();
	};

	global.__audioWorkletLoadAddon = function () {
		return { AudioWorkletBridge: { attach: function (scope, drain) { return new Bridge(drain); } } };
	};
})(typeof globalThis !== 'undefined' ? globalThis : this);
`;
