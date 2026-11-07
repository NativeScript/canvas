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

const LOCAL = Symbol('localUrl');

// Linux: the runtime's fetch (and Request) speak http(s) only. Local files are read by the native
// module off the JS thread, straight into the response's ArrayBuffer.
if (typeof __LINUX__ !== 'undefined' && __LINUX__ && typeof global.fetch === 'function' && typeof Response === 'function') {
	if (typeof global.Request === 'function') {
		const RemoteRequest = global.Request;
		// A local URL rides on a placeholder the runtime accepts (loaders build Requests first).
		global.Request = class Request extends RemoteRequest {
			constructor(input: any, init?: any) {
				const url = typeof input === 'string' ? input : (input?.[LOCAL] ?? null);
				const local = typeof url === 'string' && localPath(url) ? url : null;
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
