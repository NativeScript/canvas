// The AudioWorkletGlobalScope's Worker script, kept as a string so bundlers leave it alone.
// It spells module keywords split up: the Windows runtime regex-rewrites Worker module syntax.
export const AUDIO_WORKLET_SCOPE = String.raw`(function (global) {
	'use strict';
	var bridge = null;
	var initError = null;
	var rate = 0;
	var frame = 0;
	var definitions = new Map();
	var nodes = new Map();
	var constructingPort = null;
	var MODULE_SYNTAX = new RegExp('(^|[\\n;])\\s*(im' + 'port|ex' + 'port)\\b');

	function send(message, transfer) {
		if (transfer && transfer.length) global.postMessage(message, transfer);
		else global.postMessage(message);
	}

	function describe(error) {
		if (error && typeof error === 'object') return (error.name ? error.name + ': ' : '') + (error.message || String(error));
		return String(error);
	}

	function Port(id) {
		this._id = id;
		this._onmessage = null;
		this._listeners = [];
		this._queue = [];
		this._started = false;
		this._closed = false;
	}
	Port.prototype.postMessage = function (data, transfer) {
		if (this._closed) return;
		send({ t: 'port', id: this._id, data: data }, Array.isArray(transfer) ? transfer : transfer && transfer.transfer);
	};
	Port.prototype.start = function () {
		if (this._started) return;
		this._started = true;
		var queued = this._queue;
		this._queue = [];
		for (var i = 0; i < queued.length; i++) this._dispatch(queued[i]);
	};
	Port.prototype.close = function () {
		this._closed = true;
		this._queue = [];
	};
	Port.prototype.addEventListener = function (type, listener) {
		if (type !== 'message' || !listener || this._listeners.indexOf(listener) >= 0) return;
		this._listeners.push(listener);
		this.start();
	};
	Port.prototype.removeEventListener = function (type, listener) {
		var index = this._listeners.indexOf(listener);
		if (type === 'message' && index >= 0) this._listeners.splice(index, 1);
	};
	Port.prototype.dispatchEvent = function (event) {
		if (typeof this._onmessage === 'function') this._onmessage.call(this, event);
		var listeners = this._listeners.slice();
		for (var i = 0; i < listeners.length; i++) {
			var listener = listeners[i];
			if (typeof listener === 'function') listener.call(this, event);
			else if (listener && typeof listener.handleEvent === 'function') listener.handleEvent(event);
		}
		return true;
	};
	Port.prototype._receive = function (data) {
		if (this._closed) return;
		if (this._started) this._dispatch(data);
		else this._queue.push(data);
	};
	Port.prototype._dispatch = function (data) {
		try {
			this.dispatchEvent({ type: 'message', data: data, target: this, currentTarget: this, ports: [] });
		} catch (e) {
			console.error('AudioWorkletProcessor port: ' + describe(e));
		}
	};
	Object.defineProperty(Port.prototype, 'onmessage', {
		get: function () {
			return this._onmessage;
		},
		set: function (value) {
			this._onmessage = typeof value === 'function' ? value : null;
			if (this._onmessage) this.start();
		},
	});

	function AudioWorkletProcessor() {
		if (!constructingPort) throw new TypeError("Failed to construct 'AudioWorkletProcessor': Illegal constructor");
		Object.defineProperty(this, 'port', { value: constructingPort, enumerable: true });
		constructingPort = null;
	}

	function registerProcessor(name, processorCtor) {
		name = String(name);
		if (!name) throw new DOMException('registerProcessor: the name is empty', 'NotSupportedError');
		if (definitions.has(name)) throw new DOMException("registerProcessor: '" + name + "' is registered already", 'NotSupportedError');
		if (typeof processorCtor !== 'function' || typeof processorCtor.prototype !== 'object') throw new TypeError('registerProcessor: the processor is not a constructor');
		var list = processorCtor.parameterDescriptors;
		var descriptors = [];
		var names = {};
		if (list !== undefined && list !== null) {
			if (typeof list[Symbol.iterator] !== 'function') throw new TypeError('registerProcessor: parameterDescriptors is not iterable');
			for (var item of list) {
				var descriptor = {
					name: String(item.name),
					defaultValue: item.defaultValue === undefined ? 0 : +item.defaultValue,
					minValue: item.minValue === undefined ? -3.4028234663852886e38 : +item.minValue,
					maxValue: item.maxValue === undefined ? 3.4028234663852886e38 : +item.maxValue,
					automationRate: item.automationRate === undefined ? 'a-rate' : String(item.automationRate),
				};
				if (names[descriptor.name]) throw new DOMException("registerProcessor: duplicate parameter '" + descriptor.name + "'", 'NotSupportedError');
				if (!(descriptor.minValue <= descriptor.defaultValue && descriptor.defaultValue <= descriptor.maxValue)) {
					throw new DOMException("registerProcessor: '" + descriptor.name + "' defaultValue is out of its range", 'InvalidStateError');
				}
				if (descriptor.automationRate !== 'a-rate' && descriptor.automationRate !== 'k-rate') throw new TypeError("registerProcessor: invalid automationRate '" + descriptor.automationRate + "'");
				names[descriptor.name] = true;
				descriptors.push(descriptor);
			}
		}
		definitions.set(name, { ctor: processorCtor, descriptors: descriptors });
		send({ t: 'registered', name: name, descriptors: descriptors });
	}

	if (typeof global.DOMException !== 'function') {
		global.DOMException = function DOMException(message, name) {
			var error = new Error(message);
			error.name = name || 'Error';
			return error;
		};
	}
	global.AudioWorkletProcessor = AudioWorkletProcessor;
	global.registerProcessor = registerProcessor;
	Object.defineProperty(global, 'sampleRate', { get: function () { return rate; }, configurable: true });
	Object.defineProperty(global, 'currentFrame', { get: function () { return frame; }, configurable: true });
	Object.defineProperty(global, 'currentTime', { get: function () { return rate ? frame / rate : 0; }, configurable: true });

	function evaluate(source, url) {
		try {
			(0, eval)(source + '\n//# sourceURL=' + (url || 'audioworklet-module.js'));
			return Promise.resolve();
		} catch (e) {
			if (!(e instanceof SyntaxError) || !url || !MODULE_SYNTAX.test(source)) return Promise.reject(e);
			return import(url).then(function () {});
		}
	}

	function Views(ring, descriptors) {
		var layout = ring.layout();
		var buffer = ring.buffer();
		this.ring = ring;
		this.layout = layout;
		this.f32 = new Float32Array(buffer);
		this.u32 = new Uint32Array(buffer);
		this.f64 = new Float64Array(buffer);
		this.names = descriptors.map(function (d) { return d.name; });
		this.inputs = layout.inputs;
		this.slots = [];
		var q = layout.quantum;
		for (var s = 0; s < layout.slots; s++) {
			var base = s * layout.slotWords;
			var slot = { base: base, inputChannels: [], inputArrays: [], outputChannels: [], outputArrays: [], params: {}, paramViews: [] };
			for (var i = 0; i < this.inputs; i++) {
				var channels = [];
				for (var c = 0; c < layout.maxInputChannels; c++) {
					var at = base + layout.inputData + (i * layout.maxInputChannels + c) * q;
					channels.push(this.f32.subarray(at, at + q));
				}
				slot.inputChannels.push(channels);
				slot.inputArrays.push({});
			}
			for (var o = 0; o < layout.outputData.length; o++) {
				var outs = [];
				for (var oc = 0; oc < layout.outputCapacity[o]; oc++) {
					var ot = base + layout.outputData[o] + oc * q;
					outs.push(this.f32.subarray(ot, ot + q));
				}
				slot.outputChannels.push(outs);
				slot.outputArrays.push({});
			}
			for (var p = 0; p < this.names.length; p++) {
				var pt = base + layout.paramData + p * q;
				slot.paramViews.push([this.f32.subarray(pt, pt + 1), this.f32.subarray(pt, pt + q)]);
			}
			this.slots.push(slot);
		}
	}

	function channelsOf(cache, all, count) {
		return cache[count] || (cache[count] = all.slice(0, count));
	}

	function Node(id, name, options) {
		this.id = id;
		this.processor = null;
		this.port = new Port(id);
		this.state = 'running';
		var definition = definitions.get(name);
		var ring = bridge && bridge.node(id);
		if (!definition) throw new DOMException("AudioWorkletNode: '" + name + "' is not registered", 'InvalidStateError');
		if (!ring) throw new DOMException('AudioWorkletNode: the node is gone', 'InvalidStateError');
		this.views = new Views(ring, definition.descriptors);
		constructingPort = this.port;
		try {
			var processor = Reflect.construct(definition.ctor, [options]);
			if (!processor || typeof processor !== 'object' || processor.port !== this.port) {
				throw new TypeError('AudioWorkletProcessor: the constructor did not return its processor');
			}
			this.processor = processor;
		} finally {
			constructingPort = null;
		}
	}

	Node.prototype.fail = function (error) {
		this.state = 'failed';
		this.processor = null;
		send({ t: 'processorerror', id: this.id, message: describe(error) });
	};

	Node.prototype.render = function (index) {
		var views = this.views;
		var layout = views.layout;
		var slot = views.slots[index];
		var base = slot.base;
		var u32 = views.u32;
		var outputs = [];
		for (var o = 0; o < slot.outputChannels.length; o++) {
			var out = channelsOf(slot.outputArrays[o], slot.outputChannels[o], u32[base + layout.outputCounts + o]);
			for (var c = 0; c < out.length; c++) out[c].fill(0);
			outputs.push(out);
		}
		if (this.state !== 'running') {
			u32[base + layout.alive] = 0;
			return;
		}
		var inputs = [];
		for (var i = 0; i < slot.inputChannels.length; i++) {
			inputs.push(channelsOf(slot.inputArrays[i], slot.inputChannels[i], u32[base + layout.inputCounts + i]));
		}
		var params = slot.params;
		for (var p = 0; p < views.names.length; p++) {
			params[views.names[p]] = slot.paramViews[p][u32[base + layout.paramLengths + p] === 1 ? 0 : 1];
		}
		frame = views.f64[base / 2];
		var alive = false;
		try {
			var processor = this.processor;
			if (typeof processor.process !== 'function') throw new TypeError('AudioWorkletProcessor: process is not a function');
			alive = !!processor.process(inputs, outputs, params);
		} catch (e) {
			this.fail(e);
		}
		if (!alive && this.state === 'running') {
			this.state = 'finished';
			this.processor = null;
		}
		u32[base + layout.alive] = alive ? 1 : 0;
	};

	Node.prototype.serve = function () {
		if (!this.views) return;
		var ring = this.views.ring;
		var pending = ring.pending();
		if (pending > 0) {
			var cursor = ring.cursor;
			var slots = this.views.layout.slots;
			for (var k = 0; k < pending; k++) this.render((cursor + k) % slots);
			ring.commit(pending);
		}
		if (ring.closed && ring.pending() === 0) {
			// The port outlives the graph's node until the main thread releases it.
			ring.release();
			this.state = 'closed';
			this.processor = null;
			this.views = null;
		}
	};

	function drain() {
		if (!bridge) return;
		bridge.beginDrain();
		nodes.forEach(function (node) {
			try {
				node.serve();
			} catch (e) {
				console.error('AudioWorklet: ' + describe(e));
			}
		});
	}

	function init(message) {
		rate = message.sampleRate;
		try {
			var load = typeof global.__audioWorkletLoadAddon === 'function' ? global.__audioWorkletLoadAddon : function (specifier) { return require(specifier); };
			var addon = load(message.addon);
			bridge = addon.AudioWorkletBridge.attach(message.scope, drain);
		} catch (e) {
			initError = e;
		}
	}

	function receive(message) {
		switch (message && message.t) {
			case 'init':
				init(message);
				break;
			case 'module':
				(initError ? Promise.reject(initError) : evaluate(message.source, message.url)).then(
					function () { send({ t: 'module', id: message.id }); },
					function (e) { send({ t: 'module', id: message.id, error: describe(e) }); },
				);
				break;
			case 'create':
				try {
					if (initError) throw initError;
					nodes.set(message.id, new Node(message.id, message.name, message.options));
					drain();
				} catch (e) {
					send({ t: 'processorerror', id: message.id, message: describe(e) });
				}
				break;
			case 'port': {
				var node = nodes.get(message.id);
				if (node && node.state !== 'failed') node.port._receive(message.data);
				break;
			}
			case 'release': {
				var released = nodes.get(message.id);
				if (released && released.views) released.views.ring.release();
				nodes.delete(message.id);
				break;
			}
			case 'close':
				if (bridge) bridge.close();
				bridge = null;
				nodes.clear();
				send({ t: 'closed' });
				break;
		}
	}

	global.onmessage = function (event) {
		receive(event && event.data);
	};
})(typeof globalThis !== 'undefined' ? globalThis : this);
`;
