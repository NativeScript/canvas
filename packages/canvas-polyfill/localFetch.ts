import { knownFolders } from '@nativescript/core';
import { Helpers } from '@nativescript/canvas/helpers';

declare const __LINUX__: boolean;

/** An app-local path (`~/`, absolute or `file://`), or null for anything fetch handles itself. */
function localPath(url: string): string | null {
	if (url.startsWith('~/')) {
		return `${knownFolders.currentApp().path}/${url.substring(2)}`;
	}
	if (url.startsWith('file://')) {
		return decodeURIComponent(url.substring('file://'.length));
	}
	return url.startsWith('/') ? url : null;
}

/** The Blob an object URL names, or null when it isn't one or was revoked */
function objectUrlBlob(url: string): Blob | null {
	if (!url.startsWith('blob:')) {
		return null;
	}
	const entry = (URL as any).InternalAccessor?.getData?.(url);
	return entry?.blob ?? null;
}

/** Served here rather than by the runtime's fetch */
function isLocal(url: string): boolean {
	return !!localPath(url) || url.startsWith('blob:');
}

const LOCAL = Symbol('localUrl');

// Linux: the runtime's fetch (and Request) speak http(s) only. Local files are read by the native
// module off the JS thread, straight into the response's ArrayBuffer. `blob:` URLs are served here too.
if (typeof __LINUX__ !== 'undefined' && __LINUX__ && typeof global.fetch === 'function' && typeof Response === 'function') {
	if (typeof global.Request === 'function') {
		const RemoteRequest = global.Request;
		// A local URL rides on a placeholder the runtime accepts (loaders build Requests first).
		global.Request = class Request extends RemoteRequest {
			constructor(input: any, init?: any) {
				const url = typeof input === 'string' ? input : (input?.[LOCAL] ?? null);
				const local = typeof url === 'string' && isLocal(url) ? url : null;
				super(local ? 'http://localhost/' : input, init);
				if (local) {
					Object.defineProperty(this, LOCAL, { value: local });
					Object.defineProperty(this, 'url', { value: local, configurable: true, enumerable: true });
				}
			}
		} as any;
	}
	const fetchRemote = global.fetch;
	global.fetch = function (input: any, init?: any) {
		const url = typeof input === 'string' ? input : (input?.[LOCAL] ?? input?.url);
		if (typeof url === 'string' && url.startsWith('blob:')) {
			const blob = objectUrlBlob(url);
			if (!blob) {
				return Promise.reject(new TypeError(`Failed to fetch ${url}: no such object URL`));
			}
			// Bytes, not the Blob: the runtime's Response would stringify the polyfill's Blob.
			return blob.arrayBuffer().then((data: ArrayBuffer | ArrayBufferView) => {
				const bytes = ArrayBuffer.isView(data) ? new Uint8Array(data.buffer, data.byteOffset, data.byteLength) : new Uint8Array(data);
				return new Response(bytes, { status: 200, headers: blob.type ? { 'Content-Type': blob.type } : undefined });
			});
		}
		const path = typeof url === 'string' ? localPath(url) : null;
		if (!path) {
			return fetchRemote(input, init);
		}
		Helpers.initialize();
		return new Promise<Response>((resolve, reject) => {
			global.CanvasModule.readFile(path, (error: any, result: { buffer: ArrayBuffer; mime?: string }) => {
				if (error) {
					reject(new TypeError(`Failed to fetch ${url}: ${error}`));
					return;
				}
				const headers = result.mime ? { 'Content-Type': result.mime } : undefined;
				resolve(new Response(result.buffer, { status: 200, headers }));
			});
		});
	} as typeof fetch;
}
