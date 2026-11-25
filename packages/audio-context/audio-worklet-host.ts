import { knownFolders } from '@nativescript/core';
import type { WorkletRingLayout } from './audio-worklet-ring';

export interface WorkletDescriptor {
	name: string;
	defaultValue: number;
	minValue: number;
	maxValue: number;
	automationRate: string;
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

export interface WorkletNodeTarget {
	readonly port: AudioWorkletPort;
	_processorError(message: string): void;
}

export function domError(name: string, message: string): Error {
	const DOMExceptionCtor = (globalThis as any).DOMException;
	if (typeof DOMExceptionCtor === 'function') return new DOMExceptionCtor(message, name);
	const error = new Error(message);
	error.name = name;
	return error;
}

export function isWorkerSupported(): boolean {
	return typeof (globalThis as any).Worker === 'function';
}

/** A context's AudioWorkletGlobalScope: a Worker running audio-worklet-scope.ts. */
export class WorkletHost {
	readonly definitions = new Map<string, WorkletDescriptor[]>();
	private worker: any;
	private modules = new Map<number, { resolve: () => void; reject: (e: Error) => void }>();
	private nextModule = 1;
	private nodes = new Map<number, WeakRef<WorkletNodeTarget>>();
	private released: FinalizationRegistry<number> | null = null;
	private closing: Promise<void> | null = null;

	constructor(scriptPath: string, init: { scope: number; sampleRate: number; addon: string }, workerOptions?: any) {
		const WorkerCtor = (globalThis as any).Worker;
		this.worker = workerOptions ? new WorkerCtor(scriptPath, workerOptions) : new WorkerCtor(scriptPath);
		this.worker.onmessage = (event: { data: any }) => this.receive(event?.data);
		if (typeof FinalizationRegistry === 'function') {
			this.released = new FinalizationRegistry((id) => {
				this.nodes.delete(id);
				this.post({ t: 'release', id });
			});
		}
		this.post({ t: 'init', ...init });
	}

	post(message: any, transfer?: any[]) {
		if (this.closing) return;
		if (transfer?.length) this.worker.postMessage(message, transfer);
		else this.worker.postMessage(message);
	}

	addModule(source: string, url: string): Promise<void> {
		if (this.closing) return Promise.reject(domError('InvalidStateError', 'The AudioContext is closed'));
		const id = this.nextModule++;
		return new Promise<void>((resolve, reject) => {
			this.modules.set(id, { resolve, reject });
			this.post({ t: 'module', id, source, url });
		});
	}

	create(node: WorkletNodeTarget, id: number, name: string, options: AudioWorkletNodeOptions, ring?: { buffer: SharedArrayBuffer; layout: WorkletRingLayout }) {
		this.nodes.set(id, new WeakRef(node));
		this.released?.register(node, id);
		this.post(ring ? { t: 'create', id, name, options, ring } : { t: 'create', id, name, options });
	}

	private receive(message: any) {
		switch (message?.t) {
			case 'registered':
				this.definitions.set(message.name, message.descriptors);
				break;
			case 'module': {
				const pending = this.modules.get(message.id);
				this.modules.delete(message.id);
				if (message.error) pending?.reject(domError('AbortError', `AudioWorklet.addModule: ${message.error}`));
				else pending?.resolve();
				break;
			}
			case 'port':
				this.nodes.get(message.id)?.deref()?.port._receive(message.data);
				break;
			case 'processorerror':
				this.nodes.get(message.id)?.deref()?._processorError(message.message);
				break;
		}
	}

	/** The Worker drops its wakeups before termination; the runtimes don't. */
	close(): Promise<void> {
		if (!this.closing) {
			this.post({ t: 'close' });
			const worker = this.worker;
			this.closing = new Promise<void>((resolve) => {
				worker.onmessage = (event: { data: any }) => {
					if (event?.data?.t !== 'closed') return;
					worker.terminate();
					resolve();
				};
			});
			this.modules.forEach((pending) => pending.reject(domError('InvalidStateError', 'The AudioContext is closed')));
			this.modules.clear();
		}
		return this.closing;
	}
}

export const closeWithContext = typeof FinalizationRegistry === 'function' ? new FinalizationRegistry<WorkletHost>((host) => host.close()) : null;

/** The validated node options a platform's AudioWorkletNode builds on. */
export function resolveWorkletNodeOptions(host: WorkletHost | null | undefined, name: string, options: AudioWorkletNodeOptions = {}) {
	const descriptors = host?.definitions.get(String(name));
	if (!host || !descriptors) {
		throw domError('InvalidStateError', `AudioWorkletNode: '${name}' is not defined in the AudioWorkletGlobalScope`);
	}
	const numberOfInputs = options.numberOfInputs ?? 1;
	const numberOfOutputs = options.numberOfOutputs ?? 1;
	const outputChannelCount = options.outputChannelCount ? Array.from(options.outputChannelCount, (count) => count | 0) : undefined;
	if (numberOfInputs === 0 && numberOfOutputs === 0) throw domError('NotSupportedError', 'AudioWorkletNode: numberOfInputs and numberOfOutputs are both 0');
	if (outputChannelCount) {
		if (outputChannelCount.length !== numberOfOutputs) throw domError('IndexSizeError', 'AudioWorkletNode: outputChannelCount.length differs from numberOfOutputs');
		if (outputChannelCount.some((count) => count < 1 || count > 32)) throw domError('NotSupportedError', 'AudioWorkletNode: an outputChannelCount is out of range');
	}
	const parameterData: Record<string, number> = {};
	for (const key of Object.keys(options.parameterData ?? {})) parameterData[key] = +options.parameterData[key];
	return { host, descriptors, numberOfInputs, numberOfOutputs, outputChannelCount, parameterData };
}

/** `addModule` paths: `~/`, `file://`, absolute, or relative to the app folder. */
function workletModulePath(url: string, nativePath: (path: string) => string): string {
	if (url.startsWith('~/') || url.startsWith('file:') || url.startsWith('/') || /^[a-zA-Z]:[\\/]/.test(url)) return nativePath(url);
	return knownFolders.currentApp().path + '/' + url.replace(/^\.\//, '');
}

/** The source of an `addModule` URL; `read` reads a file path or a `data:` URL. */
export function loadWorkletModule(url: string, read: (source: string) => Promise<string>, nativePath: (path: string) => string): Promise<{ source: string; url: string }> {
	if (/^(https?|blob):/i.test(url)) {
		if (typeof fetch !== 'function') return Promise.reject(domError('AbortError', `AudioWorklet.addModule: cannot fetch ${url}`));
		return fetch(url).then((response: any) => {
			if (response.ok === false) throw domError('AbortError', `AudioWorklet.addModule: ${url}: HTTP ${response.status}`);
			return response.text().then((source: string) => ({ source, url }));
		});
	}
	if (url.startsWith('data:')) return read(url).then((source) => ({ source, url: '' }));
	const path = workletModulePath(url, nativePath);
	const loaded = read(path).then((source) => ({ source, url: path }));
	if (!url.startsWith('/')) return loaded;
	// Bundlers emit asset URLs like `/assets/x.js`: relative to the app folder here.
	const inApp = knownFolders.currentApp().path + url;
	return loaded.catch((error: Error) =>
		read(inApp).then(
			(source) => ({ source, url: inApp }),
			() => Promise.reject(error),
		),
	);
}

export class AudioParamMap<P> {
	private _map: Map<string, P>;

	constructor(map: Map<string, P>) {
		this._map = map;
	}
	get size() {
		return this._map.size;
	}
	get(name: string) {
		return this._map.get(name);
	}
	has(name: string) {
		return this._map.has(name);
	}
	keys() {
		return this._map.keys();
	}
	values() {
		return this._map.values();
	}
	entries() {
		return this._map.entries();
	}
	forEach(callback: (value: P, key: string, map: AudioParamMap<P>) => void, thisArg?: any) {
		this._map.forEach((value, key) => callback.call(thisArg, value, key, this));
	}
	[Symbol.iterator]() {
		return this._map.entries();
	}
}

/** The node's end of the port to its processor. */
export class AudioWorkletPort {
	private _onmessage: ((event: any) => void) | null = null;
	private _listeners: any[] = [];
	private _queue: any[] = [];
	private _started = false;
	private _closed = false;

	constructor(
		private _host: WorkletHost,
		private _id: number,
	) {}

	get onmessage() {
		return this._onmessage;
	}
	set onmessage(value: ((event: any) => void) | null) {
		this._onmessage = typeof value === 'function' ? value : null;
		if (this._onmessage) this.start();
	}
	onmessageerror: ((event: any) => void) | null = null;

	postMessage(data: any, transfer?: any[] | { transfer?: any[] }) {
		if (this._closed) return;
		this._host.post({ t: 'port', id: this._id, data }, Array.isArray(transfer) ? transfer : transfer?.transfer);
	}
	start() {
		if (this._started) return;
		this._started = true;
		const queued = this._queue;
		this._queue = [];
		queued.forEach((data) => this._dispatch(data));
	}
	close() {
		this._closed = true;
		this._queue = [];
	}
	addEventListener(type: string, listener: any) {
		if (type !== 'message' || !listener || this._listeners.includes(listener)) return;
		this._listeners.push(listener);
		this.start();
	}
	removeEventListener(type: string, listener: any) {
		const index = this._listeners.indexOf(listener);
		if (type === 'message' && index >= 0) this._listeners.splice(index, 1);
	}
	dispatchEvent(event: any) {
		this._onmessage?.call(this, event);
		for (const listener of this._listeners.slice()) {
			if (typeof listener === 'function') listener.call(this, event);
			else listener?.handleEvent?.(event);
		}
		return true;
	}

	/** @internal */
	_receive(data: any) {
		if (this._closed) return;
		if (this._started) this._dispatch(data);
		else this._queue.push(data);
	}

	private _dispatch(data: any) {
		this.dispatchEvent({ type: 'message', data, target: this, currentTarget: this, ports: [] });
	}
}
