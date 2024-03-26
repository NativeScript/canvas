import { CancellablePromise, Http } from '../http/http';
import { HttpError, HttpRequestOptions, ProgressEvent } from '../http/http-request-common';
import { FileManager } from '../file/file';
import { isNullOrUndefined, isObject, isFunction } from '@nativescript/core/utils/types';
import { knownFolders, path as filePath, File as fsFile } from '@nativescript/core';

enum XMLHttpRequestResponseType {
	empty = '',
	text = 'text',
	json = 'json',
	document = 'document',
	arraybuffer = 'arraybuffer',
	blob = 'blob',
}

const statuses = {
	100: 'Continue',
	101: 'Switching Protocols',
	200: 'OK',
	201: 'Created',
	202: 'Accepted',
	203: 'Non - Authoritative Information',
	204: 'No Content',
	205: 'Reset Content',
	206: 'Partial Content',
	300: 'Multiple Choices',
	301: 'Moved Permanently',
	302: 'Found',
	303: 'See Other',
	304: 'Not Modified',
	305: 'Use Proxy',
	307: 'Temporary Redirect',
	400: 'Bad Request',
	401: 'Unauthorized',
	402: 'Payment Required',
	403: 'Forbidden',
	404: 'Not Found',
	405: 'Method Not Allowed',
	406: 'Not Acceptable',
	407: 'Proxy Authentication Required',
	408: 'Request Timeout',
	409: 'Conflict',
	410: 'Gone',
	411: 'Length Required',
	412: 'Precondition Failed',
	413: 'Request Entity Too Large',
	414: 'Request - URI Too Long',
	415: 'Unsupported Media Type',
	416: 'Requested Range Not Satisfiable',
	417: 'Expectation Failed',
	500: 'Internal Server Error',
	501: 'Not Implemented',
	502: 'Bad Gateway',
	503: 'Service Unavailable',
	504: 'Gateway Timeout',
	505: 'HTTP Version Not Supported',
};

enum Status {
	UNSENT = 0,
	OPENED = 0,
	LOADING = 200,
	DONE = 200,
}

export class TNSXMLHttpRequestUpload {
	private _request: TNSXMLHttpRequest;
	private _listeners: Map<string, Array<Function>> = new Map<string, Array<Function>>();

	constructor(req) {
		this._request = req;
	}

	public addEventListener(eventName: string, handler: (e) => void) {
		const handlers = this._listeners.get(eventName) || [];
		handlers.push(handler);
		this._listeners.set(eventName, handlers);
	}

	public removeEventListener(eventName: string, toDetach: (e) => void) {
		let handlers = this._listeners.get(eventName) || [];
		handlers = handlers.filter((handler) => handler !== toDetach);
		this._listeners.set(eventName, handlers);
	}

	_emitEvent(eventName: string, ...args: Array<any>) {
		const handlers = this._listeners.get(eventName) || [];
		handlers.forEach((handler) => {
			handler.apply(this, ...args);
		});
	}
}

export class TNSXMLHttpRequest {
	public UNSENT = 0;
	public OPENED = 1;
	public HEADERS_RECEIVED = 2;
	public LOADING = 3;
	public DONE = 4;
	public onreadystatechange: (...args: any[]) => void;
	public ontimeout: (...args: any[]) => void;
	public onabort: (...args: any[]) => void;
	public onerror: (...args: any[]) => void;
	public onload: (...args: any[]) => void;
	public onloadend: (...args: any[]) => void;
	public onloadstart: (...args: any[]) => void;
	public onprogress: (...args: any[]) => void;
	timeout: number = 0;
	private _readyState = this.UNSENT;
	private _response: any = '';
	private _responseType: XMLHttpRequestResponseType = null;
	private _responseText: any = null;
	private _status: number;
	private _request: {
		method: string;
		url: string;
		async: boolean;
		username: string | null;
		password: string | null;
	} = null;
	private _http: Http;
	private _currentRequest: CancellablePromise;
	private _lastProgress: {
		lengthComputable: boolean;
		loaded: number;
		total: number;
		target: any;
	} = { lengthComputable: false, loaded: 0, total: 0, target: this };

	private _headers: any;
	private _responseURL: string = '';
	private _httpContent: any;
	private _upload: TNSXMLHttpRequestUpload;
	private _listeners: Map<string, Array<Function>> = new Map<string, Array<Function>>();
	withCredentials: boolean;

	constructor() {
		this._status = Status.UNSENT;
		this._http = new Http();
		this._upload = new TNSXMLHttpRequestUpload(this);
	}

	get readyState(): number {
		return this._readyState;
	}

	get response() {
		return this._response;
	}

	private _didUserSetResponseType = false;

	get responseType(): any {
		return this._responseType;
	}

	get responseText() {
		if (this._responseType === XMLHttpRequestResponseType.text || this._responseType === XMLHttpRequestResponseType.json) {
			return this._responseText;
		}
		return null;
	}

	get responseURL() {
		return this._responseURL;
	}

	get status(): number {
		return this._status;
	}

	get statusText(): string {
		if (this._readyState === this.UNSENT || this._readyState === this.OPENED) {
			return '';
		}
		return statuses[this.status];
	}

	get upload(): any {
		return this._upload;
	}

	private textTypes: string[] = ['text/plain', 'application/xml', 'application/rss+xml', 'text/html', 'text/xml'];

	get responseXML(): any {
		const header = this.getResponseHeader('Content-Type') || this.getResponseHeader('content-type');
		const contentType = header && header.toLowerCase();
		if (this.isTextContentType(contentType)) {
			if (this._responseType === XMLHttpRequestResponseType.document) {
				return this.responseText;
			}
		}
		return '';
	}

	private isTextContentType(contentType: string): boolean {
		let result = false;
		for (let i = 0; i < this.textTypes.length; i++) {
			if (contentType.toLowerCase().indexOf(this.textTypes[i]) >= 0) {
				result = true;
				break;
			}
		}
		return result;
	}

	private _setResponseType() {
		const header = this.getResponseHeader('Content-Type') || this.getResponseHeader('content-type');
		const contentType = header && header.toLowerCase();
		if (contentType) {
			if (contentType.indexOf('application/json') >= 0 || contentType.indexOf('+json') >= 0) {
				this._responseType = XMLHttpRequestResponseType.json;
			} else if (this.isTextContentType(contentType)) {
				if (contentType.indexOf('text/html') || contentType.indexOf('text/xml')) {
					this._responseType = XMLHttpRequestResponseType.document;
				}
				this._responseType = XMLHttpRequestResponseType.text;
			}
		} else {
			this._responseType = XMLHttpRequestResponseType.text;
		}
	}

	public getAllResponseHeaders(): string {
		if (this._readyState < 2) {
			return '';
		}

		let result = '';

		if (typeof this._headers === 'object') {
			const keys = Object.keys(this._headers);
			for (let key of keys) {
				result += key + ': ' + this._headers[key] + '\r\n';
			}
		}

		return result.substr(0, result.length - 2);
	}

	public getResponseHeader(header: string): string {
		if (typeof header === 'string' && this._readyState > 1 && this._headers) {
			header = header.toLowerCase();
			if (typeof this._headers === 'object') {
				const keys = Object.keys(this._headers);
				for (let key of keys) {
					const item = key.toLowerCase();
					if (item === header) {
						return this._headers[key];
					}
				}
			}
			return null;
		}

		return null;
	}

	public overrideMimeType(mime: string) {}

	set responseType(value: any) {
		this._didUserSetResponseType = true;
		if (value === XMLHttpRequestResponseType.empty || value in XMLHttpRequestResponseType) {
			this._responseType = value;
		} else {
			throw new Error(`Response type of '${value}' not supported.`);
		}
	}

	private _addToStringOnResponse() {
		// Add toString() method to ease debugging and
		// make Angular2 response.text() method work properly.
		if (isNullOrUndefined(this.response)) {
			return;
		}
		if (this.response instanceof ArrayBuffer) {
			return;
		}
		if (isObject(this.response)) {
			Object.defineProperty(this._response, 'toString', {
				configurable: true,
				enumerable: false,
				writable: true,
				value: () => this.responseText,
			});
		}
	}

	private _toJSString(data): string {
		const encodings = ['UTF-8', 'US-ASCII', 'ISO-8859-1', null];
		let value;
		const count = encodings.length;

		try {
			const decoder = new TextDecoder();
			const ret = decoder.decode(data);
			return ret;
		} catch (error) {}

		for (let i = 0; i < count; i++) {
			let encodingType = encodings[i];
			// let java decide :D
			if (encodingType === null) {
				value = java.nio.charset.Charset.defaultCharset().decode(data).toString();
				break;
			}
			try {
				const encoding = java.nio.charset.Charset.forName(encodingType);
				value = encoding.decode(data).toString();
				break;
			} catch (e) {}
		}
		return value;
	}

	open(method: string, url: string, async: boolean = true, username: string | null = null, password: string | null = null): void {
		this._headers = {};
		this._responseURL = '';
		this._httpContent = null;
		this._request = {
			method,
			url,
			async,
			username,
			password,
		};
		this._updateReadyStateChange(this.OPENED);
	}

	setRequestHeader(header: string, value) {
		if (this._readyState !== this.OPENED) {
			throw new Error("Failed to execute 'setRequestHeader' on 'XMLHttpRequest': The object's state must be OPENED.");
		}
		if (typeof this._headers === 'object') {
			this._headers[header] = value;
		}
	}

	send(body: any = null): void {
		if (this._readyState !== this.OPENED) {
			throw new Error("Failed to execute 'send' on 'XMLHttpRequest': The object's state must be OPENED");
		}

		if (!this._headers['Accept']) {
			this._headers['Accept'] = '*/*';
		}

		if (typeof this._request.method === 'string' && this._request.method.toLowerCase() === 'get' && typeof this._request.url === 'string' && !this._request.url.startsWith('http')) {
			let path;
			let isBlob = false;
			if (this._request.url.startsWith('file://')) {
				path = this._request.url.replace('file://', '');
			} else if (this._request.url.startsWith('~/')) {
				path = filePath.join(knownFolders.currentApp().path, this._request.url.replace('~/', ''));
			} else if (this._request.url.startsWith('/')) {
				path = this._request.url;
			} else if (this._request.url.startsWith('blob:nativescript')) {
				//path = (URL as any)?.InternalAccessor?.getPath?.(this._request.url);
				path = (URL as any)?.InternalAccessor?.getData?.(this._request.url).blob;
				isBlob = true;
			}
			if (isBlob) {
				const buf = (Blob as any).InternalAccessor.getBuffer(path) as Uint8Array;
				const responseURL = this._request.url;

				let contentLength = -1;

				this._lastProgress = {
					lengthComputable: contentLength > -1,
					loaded: 0,
					total: contentLength,
					target: this,
				};

				const startEvent = new ProgressEvent('loadstart', this._lastProgress);

				if (this.onloadstart) {
					this.onloadstart(startEvent);
				}

				this.emitEvent('loadstart', startEvent);

				this._updateReadyStateChange(this.LOADING);

				if (!buf) {
					this._status = 404;
					const errorEvent = new ProgressEvent('error', this._lastProgress);
					this._responseText = 'Invalid URL';

					if (this.onerror) {
						this.onerror(errorEvent);
					}

					this.emitEvent('error', errorEvent);

					const loadendEvent = new ProgressEvent('loadend', this._lastProgress);

					if (this.onloadend) {
						this.onloadend(loadendEvent);
					}

					this.emitEvent('loadend', loadendEvent);

					this._updateReadyStateChange(this.DONE);
				} else {
					if (!this._didUserSetResponseType) {
						this._setResponseType();
					}
					this._status = 200;
					const data = buf.buffer;
					this._httpContent = data;
					this._responseURL = responseURL;

					if (this.responseType === XMLHttpRequestResponseType.json) {
						try {
							this._responseText = this._toJSString(data);
							this._response = JSON.parse(this._responseText);
						} catch (e) {
							console.error('json parse error', e);
							// this should probably be caught before the promise resolves
						}
					} else if (this.responseType === XMLHttpRequestResponseType.text) {
						const response = this._toJSString(data);
						this._responseText = this._response = response ? response : '';
					} else if (this.responseType === XMLHttpRequestResponseType.document) {
						let response = this._toJSString(data);
						this._responseText = this._response = response ? response : '';
					} else if (this.responseType === XMLHttpRequestResponseType.arraybuffer) {
						this._response = data;
					} else if (this.responseType === XMLHttpRequestResponseType.blob) {
						let buffer: ArrayBuffer = data;
						this._response = new Blob([buffer]);
					}

					const size = data?.byteLength ?? 0;

					contentLength = size;

					this._lastProgress = {
						lengthComputable: contentLength > -1,
						loaded: size,
						total: contentLength,
						target: this,
					};

					const progressEvent = new ProgressEvent('progress', this._lastProgress);
					if (this.onprogress) {
						this.onprogress(progressEvent);
					}
					this.emitEvent('progress', progressEvent);

					this._addToStringOnResponse();

					const loadEvent = new ProgressEvent('load', this._lastProgress);

					if (this.onload) {
						this.onload(loadEvent);
					}

					this.emitEvent('load', loadEvent);

					const loadendEvent = new ProgressEvent('loadend', this._lastProgress);

					if (this.onloadend) {
						this.onloadend(loadendEvent);
					}

					this.emitEvent('loadend', loadendEvent);

					this._updateReadyStateChange(this.DONE);
				}

				return;
			}

			const responseURL = `file://${path}`;

			let contentLength = -1;
			const file = fsFile.fromPath(path);
			if (fsFile.exists(path)) {
				contentLength = file.size;
			}

			this._lastProgress = {
				lengthComputable: contentLength > -1,
				loaded: 0,
				total: contentLength,
				target: this,
			};

			const startEvent = new ProgressEvent('loadstart', this._lastProgress);

			if (this.onloadstart) {
				this.onloadstart(startEvent);
			}

			this.emitEvent('loadstart', startEvent);

			this._updateReadyStateChange(this.LOADING);

			FileManager.readFile(path, {}, (error, data) => {
				if (error) {
					const errorEvent = new ProgressEvent('error', this._lastProgress);
					this._responseText = error.message;

					if (this.onerror) {
						this.onerror(errorEvent);
					}

					this.emitEvent('error', errorEvent);

					const loadendEvent = new ProgressEvent('loadend', this._lastProgress);

					if (this.onloadend) {
						this.onloadend(loadendEvent);
					}

					this.emitEvent('loadend', loadendEvent);

					this._updateReadyStateChange(this.DONE);
				} else {
					if (!this._didUserSetResponseType) {
						this._setResponseType();
					}
					this._status = 200;
					this._httpContent = data;
					this._responseURL = responseURL;

					const fastRead = (FileManager as any)._readFile !== undefined;

					if (this.responseType === XMLHttpRequestResponseType.json) {
						try {
							if (fastRead) {
								this._responseText = this._toJSString(data);
								this._response = JSON.parse(this._responseText);
							} else {
								if ((global as any).isAndroid) {
									this._responseText = this._toJSString(data);
									this._response = JSON.parse(this._responseText);
								} else {
									this._responseText = NSString.alloc().initWithDataEncoding(data, NSUTF8StringEncoding);
									if (!this._responseText) {
										this._responseText = NSString.alloc().initWithDataEncoding(data, NSISOLatin1StringEncoding);
									}
									this._response = JSON.parse(this._responseText);
								}
							}
						} catch (e) {
							console.error('json parse error', e);
							// this should probably be caught before the promise resolves
						}
					} else if (this.responseType === XMLHttpRequestResponseType.text) {
						if (fastRead) {
							const response = this._toJSString(data);
							this._responseText = this._response = response ? response : '';
						} else {
							if ((global as any).isIOS) {
								let code = NSUTF8StringEncoding; // long:4
								let encodedString = NSString.alloc().initWithDataEncoding(data, code);

								// If UTF8 string encoding fails try with ISO-8859-1
								if (!encodedString) {
									code = NSISOLatin1StringEncoding; // long:5
									encodedString = NSString.alloc().initWithDataEncoding(data, code);
								}
								this._responseText = this._response = encodedString.toString();
							} else {
								const response = this._toJSString(data);
								this._responseText = this._response = response ? response.toString() : '';
							}
						}
					} else if (this.responseType === XMLHttpRequestResponseType.document) {
						if (fastRead) {
							let response = this._toJSString(data);
							this._responseText = this._response = response ? response : '';
						} else {
							if ((global as any).isIOS) {
								let code = NSUTF8StringEncoding; // long:4
								let encodedString = NSString.alloc().initWithDataEncoding(data, code);

								// If UTF8 string encoding fails try with ISO-8859-1
								if (!encodedString) {
									code = NSISOLatin1StringEncoding; // long:5
									encodedString = NSString.alloc().initWithDataEncoding(data, code);
								}

								this._responseText = this._response = encodedString.toString();
							} else {
								let response = this._toJSString(data);
								this._responseText = this._response = response ? response : '';
							}
						}
					} else if (this.responseType === XMLHttpRequestResponseType.arraybuffer) {
						this._response = data;
						if (!fastRead) {
							if ((global as any).isIOS) {
								this._response = interop.bufferFromData(data);
							} else {
								this._response = (ArrayBuffer as any).from(java.nio.ByteBuffer.wrap(data).rewind());
							}
						}
					} else if (this.responseType === XMLHttpRequestResponseType.blob) {
						let buffer: ArrayBuffer = data;

						if (!fastRead) {
							if ((global as any).isIOS) {
								buffer = interop.bufferFromData(data);
							} else {
								//	buffer = data;
								const buf = java.nio.ByteBuffer.wrap(data).rewind();
								buffer = (ArrayBuffer as any).from(buf);
							}
						}
						this._response = new Blob([buffer]);
					}

					let size = 0;
					if ((global as any).isIOS) {
						if (data instanceof NSData) {
							size = data.length;
						}
					} else {
						size = data?.length ?? data?.byteLength ?? 0;
					}

					this._lastProgress = {
						lengthComputable: contentLength > -1,
						loaded: size,
						total: contentLength,
						target: this,
					};

					const progressEvent = new ProgressEvent('progress', this._lastProgress);
					if (this.onprogress) {
						this.onprogress(progressEvent);
					}
					this.emitEvent('progress', progressEvent);

					this._addToStringOnResponse();

					const loadEvent = new ProgressEvent('load', this._lastProgress);

					if (this.onload) {
						this.onload(loadEvent);
					}

					this.emitEvent('load', loadEvent);

					const loadendEvent = new ProgressEvent('loadend', this._lastProgress);

					if (this.onloadend) {
						this.onloadend(loadendEvent);
					}

					this.emitEvent('loadend', loadendEvent);

					this._updateReadyStateChange(this.DONE);
				}
			});

			return;
		}
		const method = this._request.method.toLowerCase();
		const request: HttpRequestOptions = {
			content: body,
			method: this._request.method,
			url: this._request.url,
			headers: this._headers,
			onLoading: () => {
				if (this.onloadstart) {
					this.onloadstart();
				}
				let contentLength = -1;
				if (typeof this._headers === 'object') {
					if (this._headers['Content-Length']) {
						contentLength = parseInt(this._headers['Content-Length'], 10) || -1;
					}

					if (this._headers['content-length']) {
						contentLength = parseInt(this._headers['content-length'], 10) || -1;
					}
				}
				this._lastProgress = {
					lengthComputable: contentLength > -1,
					loaded: 0,
					total: contentLength,
					target: this,
				};

				const loadStartEvent = new ProgressEvent('loadstart', this._lastProgress);

				if (this._upload && (method === 'post' || method === 'put')) {
					this._upload._emitEvent('loadstart', loadStartEvent);
				}
				this.emitEvent('loadstart', loadStartEvent);

				this._updateReadyStateChange(this.LOADING);
			},
			onHeaders: (event) => {
				if (!isNaN(event.status)) {
					this._status = event.status;
				}
				if (event.headers) {
					this._headers = event.headers;
				}
				this._updateReadyStateChange(this.HEADERS_RECEIVED);
			},
		};

		// TODO: ideally we could avoid wiring up progress since it's chatty
		// With Angular integrations could determine based on reportProgress flag in options
		// right now for brevity since GET requests are for more frequent than others,
		// just enabling for post and put temporarily
		if (method === 'post' || method === 'put') {
			request.onProgress = (event) => {
				this._lastProgress = {
					...(this._lastProgress || {}),
					...event,
				};
				if (event.loaded > 0) {
					const progressEvent = new ProgressEvent('progress', this._lastProgress);
					if (this._upload && (method === 'post' || method === 'put')) {
						this._upload._emitEvent('progress', progressEvent);
					}
					this.emitEvent('progress', progressEvent);
				}
			};
		}

		if (this.timeout > 0) {
			request['timeout'] = this.timeout;
		}

		this._currentRequest = this._http.request(request);

		this._currentRequest
			.then((res) => {
				if (!this._didUserSetResponseType) {
					this._setResponseType();
				}
				this._status = res.statusCode;
				this._httpContent = res.content;
				this._responseURL = res.url;
				if (this.responseType === XMLHttpRequestResponseType.json) {
					if (typeof res.content === 'string') {
						this._responseText = res.content;
						try {
							this._response = JSON.parse(this.responseText);
						} catch (err) {
							// this should probably be caught before the promise resolves
						}
					} else if (typeof res.content === 'object') {
						this._response = res.content;
						this._responseText = res.responseText;
					} else {
						if ((global as any).isIOS) {
							if (res.content instanceof NSData) {
								let code = NSUTF8StringEncoding; // long:4

								let encodedString = NSString.alloc().initWithDataEncoding(res.content, code);

								// If UTF8 string encoding fails try with ISO-8859-1
								if (!encodedString) {
									code = NSISOLatin1StringEncoding; // long:5
									encodedString = NSString.alloc().initWithDataEncoding(res.content, code);
								}

								this._responseText = encodedString.toString();
								this._response = JSON.parse(this._responseText);
							}
						} else {
							if (res.content instanceof java.nio.ByteBuffer) {
								this._responseText = this._toJSString(res.content.array());
								this._response = JSON.parse(this._responseText);
							}
						}
					}
				} else if (this.responseType === XMLHttpRequestResponseType.text) {
					if (typeof res.content === 'string') {
						this._responseText = res.content;
					} else if (typeof res.content === 'object') {
						this._responseText = JSON.stringify(res.content); // Stringify or build manually 🧐
					} else {
						if ((global as any).isIOS) {
							if (res.content instanceof NSData) {
								let code = NSUTF8StringEncoding; // long:4

								let encodedString = NSString.alloc().initWithDataEncoding(res.content, code);

								// If UTF8 string encoding fails try with ISO-8859-1
								if (!encodedString) {
									code = NSISOLatin1StringEncoding; // long:5
									encodedString = NSString.alloc().initWithDataEncoding(res.content, code);
								}

								this._responseText = this._response = encodedString.toString();
							}
						} else {
							if (res.content instanceof java.nio.ByteBuffer) {
								this._responseText = this._response = this._toJSString(res.content.array());
							}
						}
					}
					this._response = this._responseText;
				} else if (this.responseType === XMLHttpRequestResponseType.document) {
					if (typeof res.content === 'string') {
						this._responseText = res.content;
					} else {
						if ((global as any).isIOS) {
							if (res.content instanceof NSData) {
								let code = NSUTF8StringEncoding; // long:4

								let encodedString = NSString.alloc().initWithDataEncoding(res.content, code);

								// If UTF8 string encoding fails try with ISO-8859-1
								if (!encodedString) {
									code = NSISOLatin1StringEncoding; // long:5
									encodedString = NSString.alloc().initWithDataEncoding(res.content, code);
								}

								this._responseText = this._response = encodedString.toString();
							}
						} else {
							if (res.content instanceof java.nio.ByteBuffer) {
								this._responseText = this._response = this._toJSString(res.content.array());
							}
						}
					}
				} else if (this.responseType === XMLHttpRequestResponseType.arraybuffer) {
					if ((global as any).isIOS) {
						this._response = interop.bufferFromData(res.content);
					} else {
						this._response = (ArrayBuffer as any).from(res.content);
					}
				} else if (this.responseType === XMLHttpRequestResponseType.blob) {
					if ((global as any).isIOS) {
						if (typeof res.content === 'string') {
							const encoder = new TextEncoder();
							const buffer = encoder.encode(res.content);
							this._response = new Blob([buffer]);
						} else {
							if (res.content !== null && res.content !== undefined && typeof res.content === 'object') {
								const encoder = new TextEncoder();
								const buffer = encoder.encode(res.content.toString());
								this._response = new Blob([buffer]);
							} else {
								const buffer = interop.bufferFromData(res.content);
								this._response = new Blob([buffer]);
							}
						}
					} else {
						if (typeof res.content === 'string') {
							const encoder = new TextEncoder();
							const buffer = encoder.encode(res.content);
							this._response = new Blob([buffer]);
						} else {
							if (res.content !== null && res.content !== undefined && typeof res.content === 'object') {
								const encoder = new TextEncoder();
								const buffer = encoder.encode(res.content.toString());
								this._response = new Blob([buffer]);
							} else {
								const buffer = (ArrayBuffer as any).from(res.content);
								this._response = new Blob([buffer]);
							}
						}
					}
				}

				this._addToStringOnResponse();

				if (this.onload) {
					this.onload();
				}
				const loadEvent = new ProgressEvent('load', this._lastProgress);

				if (this._upload && (method === 'post' || method === 'put')) {
					this._upload._emitEvent('load', loadEvent);
				}
				this.emitEvent('load', loadEvent);

				if (this.onloadend) {
					this.onloadend();
				}

				const loadendEvent = new ProgressEvent('loadend', this._lastProgress);

				if (this._upload && (method === 'post' || method === 'put')) {
					this._upload._emitEvent('loadend', loadendEvent);
				}
				this.emitEvent('loadend', loadendEvent);

				this._updateReadyStateChange(this.DONE);
			})
			.catch((error) => {
				const type: HttpError = error.type;
				const method = this._request.method.toLowerCase();
				switch (type) {
					case HttpError.Cancelled:
						if (this.onabort) {
							this.onabort();
						}
						const abortEvent = new ProgressEvent('abort', this._lastProgress);

						if (this._upload && (method === 'post' || method === 'put')) {
							this._upload._emitEvent('abort', abortEvent);
						}
						this.emitEvent('abort', abortEvent);

						if (this.onloadend) {
							this.onloadend();
						}

						const _loadendEvent = new ProgressEvent('loadend', this._lastProgress);

						if (this._upload && (method === 'post' || method === 'put')) {
							this._upload._emitEvent('loadend', _loadendEvent);
						}
						this.emitEvent('loadend', _loadendEvent);

						if (this._readyState === this.UNSENT || this._readyState === this.OPENED || this._readyState === this.DONE) {
							this._updateReadyStateChange(this.UNSENT);
						} else {
							this._updateReadyStateChange(this.DONE);
						}
						this._currentRequest = null;
						break;
					case HttpError.Timeout:
						if (this.ontimeout) {
							this.ontimeout();
						}
						const timeoutEvent = new ProgressEvent('timeout', this._lastProgress);

						if (this._upload && (method === 'post' || method === 'put')) {
							this._upload._emitEvent('timeout', timeoutEvent);
						}
						this.emitEvent('timeout', timeoutEvent);
						break;
					case HttpError.Error:
						if (this.onerror) {
							this.onerror(error.message);
						}

						const errorEvent = new ProgressEvent('error', this._lastProgress);

						if (this._upload && (method === 'post' || method === 'put')) {
							this._upload._emitEvent('error', errorEvent);
						}
						this.emitEvent('error', errorEvent);

						if (this.onloadend) {
							this.onloadend();
						}

						const loadendEvent = new ProgressEvent('loadend', this._lastProgress);

						if (this._upload && (method === 'post' || method === 'put')) {
							this._upload._emitEvent('loadend', loadendEvent);
						}
						this.emitEvent('loadend', loadendEvent);
						break;
				}
				this._updateReadyStateChange(this.DONE);
			});
	}

	abort() {
		if (this._currentRequest) {
			this._currentRequest.cancel();
		}
	}

	public addEventListener(eventName: string, handler: (e) => void) {
		const handlers = this._listeners.get(eventName) || [];
		handlers.push(handler);
		this._listeners.set(eventName, handlers);
	}

	public removeEventListener(eventName: string, toDetach: (e) => void) {
		let handlers = this._listeners.get(eventName) || [];
		handlers = handlers.filter((handler) => handler !== toDetach);
		this._listeners.set(eventName, handlers);
	}

	private emitEvent(eventName: string, ...args: Array<any>) {
		const handlers = this._listeners.get(eventName) || [];
		handlers.forEach((handler) => {
			handler.apply(this, ...args);
		});
	}

	dispatchEvent(event: Event): boolean {
		return false;
	}

	private _updateReadyStateChange(state) {
		this._readyState = state;
		if (this.onreadystatechange) {
			this.onreadystatechange();
		}
	}
}

export class FormData {
	private _data: Map<string, any>;

	constructor() {
		this._data = new Map<string, any>();
	}

	append(name: string, value: any) {
		this._data.set(name, value);
	}

	toString(): string {
		let arr = new Array<string>();

		this._data.forEach(function (value, name, map) {
			arr.push(`${encodeURIComponent(name)}=${encodeURIComponent(value)}`);
		});

		return arr.join('&');
	}
}

export class Blob {
	// Note: only for use by XHR
	public static InternalAccessor = class {
		public static getBuffer(blob: Blob) {
			return blob._buffer;
		}
	};

	private _buffer: Uint8Array;
	private _size: number;
	private _type: string;

	public get size() {
		return this._size;
	}

	public get type() {
		return this._type;
	}

	constructor(chunks: Array<BufferSource | DataView | Blob | string> = [], opts: { type?: string } = {}) {
		const dataChunks: Uint8Array[] = [];
		for (const chunk of chunks) {
			if (chunk instanceof Blob) {
				dataChunks.push(chunk._buffer);
			} else if (typeof chunk === 'string') {
				const textEncoder = new TextEncoder();
				dataChunks.push(textEncoder.encode(chunk));
			} else if (chunk instanceof DataView) {
				dataChunks.push(new Uint8Array(chunk.buffer.slice(0)));
			} else if (chunk instanceof ArrayBuffer || ArrayBuffer.isView(chunk)) {
				dataChunks.push(new Uint8Array(ArrayBuffer.isView(chunk) ? chunk.buffer.slice(0) : chunk.slice(0)));
			} else {
				const textEncoder = new TextEncoder();
				dataChunks.push(textEncoder.encode(String(chunk)));
			}
		}

		const size = dataChunks.reduce((size, chunk) => size + chunk.byteLength, 0);
		const buffer = new Uint8Array(size);
		let offset = 0;
		for (let i = 0; i < dataChunks.length; i++) {
			const chunk = dataChunks[i];
			buffer.set(chunk, offset);
			offset += chunk.byteLength;
		}

		this._buffer = buffer;
		this._size = this._buffer.byteLength;

		this._type = opts.type || '';
		if (/[^\u0020-\u007E]/.test(this._type)) {
			this._type = '';
		} else {
			this._type = this._type.toLowerCase();
		}
	}

	public arrayBuffer(): Promise<ArrayBuffer> {
		return Promise.resolve(this._buffer);
	}

	public text(): Promise<string> {
		const textDecoder = new TextDecoder();

		return Promise.resolve(textDecoder.decode(this._buffer));
	}

	public slice(start?: number, end?: number, type?: string): Blob {
		const slice = this._buffer.slice(start || 0, end || this._buffer.length);

		return new Blob([slice], { type: type });
	}

	public stream() {
		throw new Error('stream is currently not supported');
	}

	public toString() {
		return '[object Blob]';
	}

	[Symbol.toStringTag] = 'Blob';
}

export class File extends Blob {
	private _name: string;
	private _lastModified: number;

	public get name() {
		return this._name;
	}

	public get lastModified() {
		return this._lastModified;
	}

	constructor(chunks: Array<BufferSource | DataView | Blob | string>, name: string, opts: { type?: string; lastModified?: number } = {}) {
		super(chunks, opts);
		this._name = name.replace(/\//g, ':');
		this._lastModified = opts.lastModified ? new Date(opts.lastModified).valueOf() : Date.now();
	}

	public toString() {
		return '[object File]';
	}

	[Symbol.toStringTag] = 'File';
}

export class FileReader {
	public EMPTY = 0;
	public LOADING = 1;
	public DONE = 2;

	public onabort: (...args: any[]) => void;
	public onerror: (...args: any[]) => void;
	public onload: (...args: any[]) => void;
	public onloadend: (...args: any[]) => void;
	public onloadstart: (...args: any[]) => void;
	public onprogress: (...args: any[]) => void;

	private _readyState: number;
	private _result: string | ArrayBuffer | null;

	private _listeners: Map<string, Array<Function>> = new Map<string, Array<Function>>();

	public get readyState(): number {
		return this._readyState;
	}

	public get result(): string | ArrayBuffer | null {
		return this._result;
	}

	constructor() {
		//
	}

	private _array2base64(input: Uint8Array): string {
		var byteToCharMap = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=';

		var output = [];

		for (var i = 0; i < input.length; i += 3) {
			var byte1 = input[i];
			var haveByte2 = i + 1 < input.length;
			var byte2 = haveByte2 ? input[i + 1] : 0;
			var haveByte3 = i + 2 < input.length;
			var byte3 = haveByte3 ? input[i + 2] : 0;

			var outByte1 = byte1 >> 2;
			var outByte2 = ((byte1 & 0x03) << 4) | (byte2 >> 4);
			var outByte3 = ((byte2 & 0x0f) << 2) | (byte3 >> 6);
			var outByte4 = byte3 & 0x3f;

			if (!haveByte3) {
				outByte4 = 64;

				if (!haveByte2) {
					outByte3 = 64;
				}
			}

			output.push(byteToCharMap[outByte1], byteToCharMap[outByte2], byteToCharMap[outByte3], byteToCharMap[outByte4]);
		}

		return output.join('');
	}

	private _read(blob, kind) {
		if (!(blob instanceof Blob)) {
			throw new TypeError(`Failed to execute '${kind}' on 'FileReader': parameter 1 is not of type 'Blob'.`);
		}

		this._result = '';
		setTimeout(() => {
			this._readyState = this.LOADING;
			this.emitEvent('load');
			this.emitEvent('loadend');
		});
	}

	private emitEvent(eventName: string, ...args: Array<any>) {
		if (isFunction(this['on' + eventName])) {
			this['on' + eventName](...args);
		}

		let handlers = this._listeners.get(eventName) || [];
		handlers.forEach((handler) => {
			handler(this, ...args);
		});
	}

	public addEventListener(eventName: string, handler: Function) {
		if (['abort', 'error', 'load', 'loadend', 'loadstart', 'progress'].indexOf(eventName) === -1) {
			throw new Error('Event not supported: ' + eventName);
		}

		let handlers = this._listeners.get(eventName) || [];
		handlers.push(handler);
		this._listeners.set(eventName, handlers);
	}

	public removeEventListener(eventName: string, toDetach: Function) {
		let handlers = this._listeners.get(eventName) || [];
		handlers = handlers.filter((handler) => handler !== toDetach);
		this._listeners.set(eventName, handlers);
	}

	public readAsDataURL(blob: Blob) {
		this._read(blob, 'readAsDataURL');
		this._result = `data:${blob.type};base64,${this._array2base64(Blob.InternalAccessor.getBuffer(blob))}`;
	}

	public readAsText(blob: Blob) {
		this._read(blob, 'readAsText');
		const textDecoder = new TextDecoder();
		this._result = textDecoder.decode(Blob.InternalAccessor.getBuffer(blob));
	}

	public readAsArrayBuffer(blob: Blob) {
		this._read(blob, 'readAsArrayBuffer');
		this._result = Blob.InternalAccessor.getBuffer(blob).buffer.slice(0);
	}

	public abort() {
		//
	}

	public toString() {
		return '[object FileReader]';
	}

	[Symbol.toStringTag] = 'FileReader';
}
