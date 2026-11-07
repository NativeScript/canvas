import { Utils } from '@nativescript/core';
import { FileManager } from '../file/file';
import { Headers, HttpDownloadRequestOptions, HttpError, HttpRequestOptions, TNSHttpSettings } from './http-request-common';

export type CancellablePromise = Promise<any> & { cancel: () => void };

export enum HttpResponseEncoding {
	UTF8,
	GBK,
}

const GET = 'GET';
const TEXT_TYPES = ['text/plain', 'application/xml', 'application/rss+xml', 'text/html', 'text/xml', 'image/svg+xml'];

export function addHeader(headers: Headers, key: string, value: string): void {
	if (!headers[key]) {
		headers[key] = value;
	} else if (Array.isArray(headers[key])) {
		(<string[]>headers[key]).push(value);
	} else {
		const values: string[] = [<string>headers[key]];
		values.push(value);
		headers[key] = values;
	}
}

function isTextContentType(contentType: string) {
	const type = typeof contentType === 'string' ? contentType.toLowerCase() : '';
	return TEXT_TYPES.some((text) => type.indexOf(text) >= 0);
}

/** The highest-quality type of an Accept (or Content-Type) value, as the iOS implementation picks it. */
function preferredType(value: string): string {
	const plain: string[] = [];
	const weighted: string[] = [];
	for (const type of value.split(',')) {
		(type.indexOf(';q=') > -1 ? weighted : plain).push(type.trim());
	}
	const quality = (type: string) => parseFloat(type.substring(type.indexOf(';q=') + 3));
	weighted.sort((a, b) => quality(b) - quality(a));
	return [...plain, ...weighted][0] ?? 'text/plain';
}

function headerValue(headers: Headers, name: string): string | undefined {
	const lower = name.toLowerCase();
	const entries: [string, any][] = headers instanceof Map ? Array.from(headers.entries()) : Object.entries(headers ?? {});
	const match = entries.find(([key]) => key.toLowerCase() === lower);
	return match ? String(Array.isArray(match[1]) ? match[1][0] : match[1]) : undefined;
}

function requestBody(content: any): any {
	if (content === undefined || content === null) {
		return undefined;
	}
	if (Utils.isString(content) || content instanceof ArrayBuffer || ArrayBuffer.isView(content)) {
		return content;
	}
	return Utils.isObject(content) ? JSON.stringify(content) : String(content);
}

function decodeText(bytes: ArrayBuffer): string {
	try {
		return new TextDecoder('utf-8').decode(new Uint8Array(bytes));
	} catch (e) {
		// Latin-1, as the iOS implementation falls back to.
		let text = '';
		const view = new Uint8Array(bytes);
		for (let i = 0; i < view.length; i++) {
			text += String.fromCharCode(view[i]);
		}
		return text;
	}
}

/** Requests through the runtime's `fetch` (its own network threads, bodies as ArrayBuffers). */
export class Http {
	constructor() {}

	request(options: HttpRequestOptions): CancellablePromise {
		const abort = typeof AbortController !== 'undefined' ? new AbortController() : null;
		let failure: HttpError | undefined;
		let timer: any;

		const request = <CancellablePromise>new Promise<any>((resolve, reject) => {
			if (!options.url) {
				reject(new Error('Request url was empty.'));
				return;
			}
			const fail = (type: HttpError, message: string) => {
				clearTimeout(timer);
				reject({ type, message });
			};
			if (Utils.isNumber(options.timeout) && options.timeout > 0) {
				timer = setTimeout(() => {
					failure = HttpError.Timeout;
					abort?.abort();
				}, options.timeout);
			}
			const headers: Record<string, string> = {};
			const entries: [string, any][] = options.headers instanceof Map ? Array.from(options.headers.entries()) : Object.entries(options.headers ?? {});
			for (const [key, value] of entries) {
				headers[key] = String(value);
			}
			fetch(options.url.trim(), { method: Utils.isDefined(options.method) ? options.method : GET, headers, body: requestBody(options.content), signal: abort?.signal } as any)
				.then(async (response: any) => {
					const statusCode: number = response.status;
					const url: string = response.url || options.url;
					const responseHeaders: Headers = {};
					response.headers?.forEach?.((value: string, key: string) => addHeader(responseHeaders, key, value));
					options.onHeaders?.({ headers: responseHeaders, status: statusCode });

					const length = parseInt(headerValue(responseHeaders, 'Content-Length') ?? '', 10);
					const lengthComputable = !isNaN(length) && length > -1;
					options.onProgress?.({ lengthComputable, loaded: 0, total: lengthComputable ? length : 0 });

					const bytes: ArrayBuffer = await response.arrayBuffer();
					options.onLoading?.();
					options.onProgress?.({ lengthComputable, loaded: bytes.byteLength, total: lengthComputable ? length : 0 });

					const requestType = headerValue(options.headers, 'Content-Type') ?? headerValue(options.headers, 'Accept');
					let returnType = requestType ? preferredType(requestType) : '*/*';
					if (returnType === '*/*') {
						returnType = headerValue(responseHeaders, 'Content-Type') ?? '';
					}

					let body: any = bytes;
					let responseText: string | undefined;
					if (isTextContentType(returnType)) {
						responseText = body = decodeText(bytes);
					} else if (returnType.indexOf('application/json') > -1) {
						responseText = decodeText(bytes);
						try {
							body = JSON.parse(responseText);
						} catch (err) {
							fail(HttpError.Error, String(err));
							return;
						}
					}

					const saved = TNSHttpSettings.saveImage && TNSHttpSettings.currentlySavedImages?.[url];
					if (saved?.localPath) {
						FileManager.writeFile(bytes, saved.localPath, (error, result) => {
							if (TNSHttpSettings.debug) {
								console.log('http image save:', error ? error : result);
							}
						});
					}
					clearTimeout(timer);
					resolve({ url, content: body, responseText, statusCode, headers: responseHeaders });
				})
				.catch((error: any) => fail(failure ?? (abort?.signal?.aborted ? HttpError.Cancelled : HttpError.Error), error?.message ?? String(error)));
		});
		request['cancel'] = function () {
			if (!failure) {
				failure = HttpError.Cancelled;
				abort?.abort();
			}
		};
		return request;
	}

	public static getFile(options: HttpDownloadRequestOptions): CancellablePromise {
		return null;
	}
}
