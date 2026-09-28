import { Utils } from '@nativescript/core';
import { FileManager } from '../file/file';
import { Headers, HttpDownloadRequestOptions, HttpError, HttpRequestOptions, TNSHttpSettings } from './http-request-common';

declare const Windows: any, NSWinRT: any;

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

/** Name/value pairs of an IIterable<IKeyValuePair<string, string>> header collection. */
function collectHeaders(collection: any, into: Headers) {
	try {
		const iterator = collection.First();
		while (iterator && iterator.HasCurrent) {
			const pair = iterator.Current;
			addHeader(into, pair.Key, pair.Value);
			iterator.MoveNext();
		}
	} catch (e) {}
}

function headerValue(headers: Headers, name: string): string | undefined {
	const lower = name.toLowerCase();
	const entries: [string, any][] = headers instanceof Map ? Array.from(headers.entries()) : Object.entries(headers ?? {});
	const match = entries.find(([key]) => key.toLowerCase() === lower);
	return match ? String(Array.isArray(match[1]) ? match[1][0] : match[1]) : undefined;
}

/** An IBuffer's bytes as an ArrayBuffer: a view over the buffer where the runtime supports it. */
function bufferToArrayBuffer(buffer: any): ArrayBuffer {
	const view = NSWinRT.interop?.arrayBufferFromBuffer?.(buffer);
	if (view instanceof ArrayBuffer) {
		return view;
	}
	const bytes = new Uint8Array(buffer.Length);
	Windows.Storage.Streams.DataReader.FromBuffer(buffer).ReadBytes(bytes);
	return bytes.buffer;
}

function bytesToBuffer(bytes: Uint8Array): any {
	const writer = new Windows.Storage.Streams.DataWriter();
	writer.WriteBytes(bytes);
	return writer.DetachBuffer();
}

function requestContent(content: any): any {
	const Http = Windows.Web.Http;
	if (Utils.isString(content)) {
		return new Http.HttpStringContent(content.toString());
	}
	if (content instanceof ArrayBuffer) {
		return new Http.HttpBufferContent(bytesToBuffer(new Uint8Array(content)));
	}
	if (ArrayBuffer.isView(content)) {
		return new Http.HttpBufferContent(bytesToBuffer(new Uint8Array(content.buffer, content.byteOffset, content.byteLength)));
	}
	if (Utils.isObject(content)) {
		return new Http.HttpStringContent(JSON.stringify(content));
	}
	return null;
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

export class Http {
	/** Pending WinRT operations by request, for cancel(). */
	static _tasks: Map<number, any> = new Map();
	private static _nextId = 0;
	private static _client: any;

	constructor() {}

	private static get client() {
		// One client for the app, as HttpClient is designed to be shared (connection reuse).
		if (!Http._client) {
			Http._client = new Windows.Web.Http.HttpClient();
		}
		return Http._client;
	}

	request(options: HttpRequestOptions): CancellablePromise {
		const id = ++Http._nextId;
		let settled = false;
		let failure: HttpError | undefined;
		let timer: any;

		const request = <CancellablePromise>new Promise<any>((resolve, reject) => {
			if (!options.url) {
				reject(new Error('Request url was empty.'));
				return;
			}
			const fail = (type: HttpError, message: string) => {
				if (!settled) {
					settled = true;
					reject({ type, message });
				}
			};
			const done = () => {
				settled = true;
				clearTimeout(timer);
				Http._tasks.delete(id);
			};

			try {
				const Http_ = Windows.Web.Http;
				const method = Utils.isDefined(options.method) ? options.method : GET;
				const message = new Http_.HttpRequestMessage(new Http_.HttpMethod(method), new Windows.Foundation.Uri(options.url.trim()));

				const content = options.content !== undefined && options.content !== null ? requestContent(options.content) : null;
				const headers: [string, any][] = options.headers instanceof Map ? Array.from(options.headers.entries()) : Object.entries(options.headers ?? {});
				for (const [key, value] of headers) {
					const text = String(value);
					if (key.toLowerCase() === 'content-type') {
						if (content) {
							content.Headers.ContentType = Http_.Headers.HttpMediaTypeHeaderValue.Parse(text);
						}
						continue;
					}
					// Content headers (Content-Language, ...) belong on the content.
					if (!message.Headers.TryAppendWithoutValidation(key, text) && content) {
						content.Headers.TryAppendWithoutValidation(key, text);
					}
				}
				if (content) {
					message.Content = content;
				}

				if (Utils.isNumber(options.timeout) && options.timeout > 0) {
					timer = setTimeout(() => {
						failure = HttpError.Timeout;
						Http._tasks.get(id)?.Cancel?.();
						fail(HttpError.Timeout, 'The request timed out.');
					}, options.timeout);
				}

				// Headers first (ResponseHeadersRead), then the body, so onHeaders fires early.
				const send = Http.client.SendRequestAsync(message, Http_.HttpCompletionOption.ResponseHeadersRead);
				Http._tasks.set(id, send);
				NSWinRT.toPromise(send)
					.then((response: any) => {
						const statusCode = response.StatusCode;
						const url = response.RequestMessage?.RequestUri?.AbsoluteUri ?? options.url;
						const responseHeaders: Headers = {};
						collectHeaders(response.Headers, responseHeaders);
						collectHeaders(response.Content?.Headers, responseHeaders);
						options.onHeaders?.({ headers: responseHeaders, status: statusCode });

						const length = parseInt(headerValue(responseHeaders, 'Content-Length') ?? '', 10);
						const lengthComputable = !isNaN(length) && length > -1;
						options.onProgress?.({ lengthComputable, loaded: 0, total: lengthComputable ? length : 0 });

						const read = response.Content.ReadAsBufferAsync();
						Http._tasks.set(id, read);
						return NSWinRT.toPromise(read).then((buffer: any) => {
							const bytes = bufferToArrayBuffer(buffer);
							const progress = { lengthComputable, loaded: bytes.byteLength, total: lengthComputable ? length : 0 };
							options.onLoading?.();
							options.onProgress?.(progress);

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
									done();
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

							if (!settled) {
								done();
								resolve({ url, content: body, responseText, statusCode, headers: responseHeaders });
							}
						});
					})
					.catch((error: any) => {
						done();
						fail(failure ?? HttpError.Error, error?.message ?? String(error));
					});
			} catch (ex) {
				done();
				fail(HttpError.Error, ex?.message ?? String(ex));
			}
		});
		request['cancel'] = function () {
			const operation = Http._tasks.get(id);
			if (operation && !settled) {
				failure = HttpError.Cancelled;
				operation.Cancel?.();
			}
		};
		return request;
	}

	public static getFile(options: HttpDownloadRequestOptions): CancellablePromise {
		return null;
	}
}
