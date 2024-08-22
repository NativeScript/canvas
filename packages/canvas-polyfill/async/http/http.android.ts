import { fileNameFromPath, Headers, HttpDownloadRequestOptions, HttpError, HttpRequestOptions, isImageUrl, SaveImageStorageKey, TNSHttpSettings } from './http-request-common';
import { NetworkAgent } from '@nativescript/core/debugger';
import { File, Folder, knownFolders, path, Utils, Application, ApplicationSettings } from '@nativescript/core';
import { FileManager } from '../file/file';

export type CancellablePromise = Promise<any> & { cancel: () => void };

declare var com;

export enum HttpResponseEncoding {
	UTF8,
	GBK,
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

function parseJSON(source: string): any {
	const src = source.trim();
	if (src.lastIndexOf(')') === src.length - 1) {
		return JSON.parse(src.substring(src.indexOf('(') + 1, src.lastIndexOf(')')));
	}

	return JSON.parse(src);
}

const textTypes: string[] = ['text/plain', 'application/xml', 'application/rss+xml', 'text/html', 'text/xml', 'image/svg+xml'];

const isTextContentType = (contentType: string): boolean => {
	let result = false;
	for (let i = 0; i < textTypes.length; i++) {
		if (contentType.toLowerCase().indexOf(textTypes[i]) >= 0) {
			result = true;
			break;
		}
	}
	return result;
};

const requestCallbacks = new Map();
let requestIdCounter = 0;

export class Http {
	constructor() {}

	private static buildJavaOptions(options: HttpRequestOptions) {
		if (!Utils.isString(options.url)) {
			throw new Error('Http request must provide a valid url.');
		}

		let javaOptions = new com.github.triniwiz.async.Async2.Http.RequestOptions();

		javaOptions.url = options.url;

		let method;
		if (Utils.isString(typeof options.method)) {
			javaOptions.method = options.method;
			method = options.method.toLowerCase();
		}
		if ((method && method === 'post') || method === 'put') {
			if (Utils.isString(options.content)) {
				javaOptions.content = new java.lang.String(options.content);
			} else if (Utils.isObject(options.content)) {
				javaOptions.content = serialize(options.content);
			}
		}
		if (typeof options.timeout === 'number') {
			javaOptions.timeout = options.timeout;
		}

		if (options.headers) {
			const arrayList = new java.util.ArrayList<any>();
			const pair = com.github.triniwiz.async.Async2.Http.KeyValuePair;

			if (options.headers instanceof Map) {
				options.headers.forEach((value, key) => {
					arrayList.add(new pair(key, value + ''));
				});
			} else {
				for (let key in options.headers) {
					arrayList.add(new pair(key, options.headers[key] + ''));
				}
			}

			javaOptions.headers = arrayList;
		}
		return javaOptions;
	}

	private static buildJavaDownloadOptions(options: HttpDownloadRequestOptions) {
		if (!Utils.isString(options.url)) {
			throw new Error('Http request must provide a valid url.');
		}

		const javaOptions = new com.github.triniwiz.async.Async2.Http.DownloadRequestOptions();
		javaOptions.url = options.url;

		if (typeof options.timeout === 'number') {
			javaOptions.timeout = options.timeout;
		}

		if (typeof options.filePath === 'string') {
			javaOptions.filePath = options.filePath;
		} else {
			// creates directory
			Folder.fromPath(path.join(knownFolders.temp().path, 'async_http'));
			javaOptions.filePath = path.join(knownFolders.temp().path, 'async_http', java.util.UUID.randomUUID().toString());
		}

		if (options.headers) {
			const arrayList = new java.util.ArrayList<any>();
			const pair = com.github.triniwiz.async.Async2.Http.KeyValuePair;

			if (options.headers instanceof Map) {
				options.headers.forEach((value, key) => {
					arrayList.add(new pair(key, value + ''));
				});
			} else {
				for (let key in options.headers) {
					arrayList.add(new pair(key, options.headers[key] + ''));
				}
			}

			javaOptions.headers = arrayList;
		}
		return javaOptions;
	}

	request(options: HttpRequestOptions): CancellablePromise {
		const headers: Headers = {};
		let statusCode = 0;
		let id;
		const counter = requestIdCounter;
		const request = <CancellablePromise>new Promise<any>((resolve, reject) => {
			try {
				// initialize the options
				const javaOptions = Http.buildJavaOptions(options);

				if (TNSHttpSettings.debug) {
					// @ts-ignore
					if (global.__inspector && global.__inspector.isConnected) {
						NetworkAgent.requestWillBeSent(requestIdCounter, options);
					}
				}

				const makeRemoteRequest = () => {
					const callback = new com.github.triniwiz.async.Async2.Http.Callback({
						onCancel(param: any): void {
							reject({
								type: HttpError.Cancelled,
								result: param,
							});
							requestCallbacks.delete(id);
						},
						onComplete(result: any): void {
							let content;
							let responseText;
							let _isString = false;
							if (result.content instanceof org.json.JSONObject || result.content instanceof org.json.JSONArray) {
								content = deserialize(result.content);
								responseText = result.contentText;
								_isString = true;
							} else {
								content = result.content;
								responseText = result.contentText;
							}
							if (result && result.headers) {
								const length = result.headers.size();
								let pair;
								for (let i = 0; i < length; i++) {
									pair = result.headers.get(i);
									addHeader(headers, pair.key, pair.value);
								}
							}

							// send response data (for requestId) to network debugger

							let contentType = headers['Content-Type'];
							const hasContentType = !Utils.isNullOrUndefined(contentType);
							if (!hasContentType) {
								contentType = headers['content-type'];
							}

							let acceptHeader;
							if (!hasContentType) {
								acceptHeader = headers['Accept'];
								if (Utils.isNullOrUndefined(acceptHeader)) {
									acceptHeader = headers['accept'];
								}
							} else {
								acceptHeader = contentType;
							}

							let returnType = 'text/plain';
							if (!Utils.isNullOrUndefined(acceptHeader) && Utils.isString(acceptHeader)) {
								let acceptValues = acceptHeader.split(',');
								let quality = [];
								let defaultQuality = [];
								let customQuality = [];
								for (let value of acceptValues) {
									if (value.indexOf(';q=') > -1) {
										customQuality.push(value);
									} else {
										defaultQuality.push(value);
									}
								}
								customQuality = customQuality.sort((a, b) => {
									const a_quality = parseFloat(a.substring(a.indexOf(';q=')).replace(';q=', ''));
									const b_quality = parseFloat(b.substring(b.indexOf(';q=')).replace(';q=', ''));
									return b_quality - a_quality;
								});
								quality.push(...defaultQuality);
								quality.push(...customQuality);
								returnType = quality[0] as string;
							}

							if (returnType === '*/*' || !hasContentType) {
								returnType = contentType as string;
							}

							result['statusCode'] = statusCode;

							if (TNSHttpSettings.debug) {
								// send response data (for requestId) to network debugger
								// @ts-ignore
								if (global.__inspector && global.__inspector.isConnected) {
									NetworkAgent.responseReceived(
										counter,
										{
											url: result.url,
											statusCode,
											headers,
											responseAsString: Utils.isString(result.contentText) ? result.contentText : result.content?.toString() || '',
											responseAsImage: null, // TODO needs base64 Image
										} as any,
										headers
									);
								}
							}

							if (isTextContentType(returnType) && Utils.isNullOrUndefined(responseText)) {
								responseText = result.contentText;
							}

							if (TNSHttpSettings.saveImage && TNSHttpSettings.currentlySavedImages && TNSHttpSettings.currentlySavedImages[this._url]) {
								// ensure saved to disk
								if (TNSHttpSettings.currentlySavedImages[this._url].localPath) {
									FileManager.writeFile(content, TNSHttpSettings.currentlySavedImages[this._url].localPath, function (error, result) {
										if (TNSHttpSettings.debug) {
											console.log('http image save:', error ? error : result);
										}
									});
								}
							}

							resolve({
								url: result.url,
								content,
								responseText,
								statusCode: statusCode,
								headers: headers,
							});
							requestCallbacks.delete(id);
						},
						onError(param0: string, param1: java.lang.Exception): void {
							reject({
								type: HttpError.Error,
								message: param0,
							});
							requestCallbacks.delete(id);
						},
						onHeaders(jHeaders: any, status: number): void {
							statusCode = status;
							const length = jHeaders.size();
							let pair;
							for (let i = 0; i < length; i++) {
								pair = jHeaders.get(i);
								addHeader(headers, pair.key, pair.value);
							}
							if (options.onHeaders) {
								options.onHeaders(headers, statusCode);
							}
							requestCallbacks.delete(id);
						},
						onLoading(): void {
							options.onLoading();
							requestCallbacks.delete(id);
						},
						onProgress(lengthComputable: boolean, loaded: number, total: number): void {
							if (options.onProgress) {
								options.onProgress({
									lengthComputable,
									loaded,
									total,
								});
							}
							requestCallbacks.delete(id);
						},
						onTimeout(): void {
							reject({
								type: HttpError.Timeout,
							});
							requestCallbacks.delete(id);
						},
					});
					id = com.github.triniwiz.async.Async2.Http.makeRequest(javaOptions, callback);
					requestCallbacks.set(id, callback);
				};

				if (TNSHttpSettings.saveImage && isImageUrl(options.url)) {
					// handle saved images to disk
					if (!TNSHttpSettings.currentlySavedImages) {
						const stored = ApplicationSettings.getString(SaveImageStorageKey);
						if (stored) {
							try {
								TNSHttpSettings.currentlySavedImages = JSON.parse(stored);
							} catch (err) {
								TNSHttpSettings.currentlySavedImages = {};
							}
						} else {
							TNSHttpSettings.currentlySavedImages = {};
						}
					}

					const imageSetting = TNSHttpSettings.currentlySavedImages[options.url];
					const requests = imageSetting ? imageSetting.requests : 0;
					let localPath: string;
					if (imageSetting && imageSetting.localPath && File.exists(imageSetting.localPath)) {
						// previously saved to disk
						FileManager.readFile(imageSetting.localPath, null, (error, file) => {
							if (error) {
								if (TNSHttpSettings.debug) {
									console.log('http image load error:', error);
								}
							}
							resolve({
								url: options.url,
								responseText: '',
								statusCode: 200,
								content: file,
								headers: {
									'Content-Type': 'arraybuffer',
								},
							});
						});
					} else if (requests >= TNSHttpSettings.saveImage.numberOfRequests) {
						// setup to write to disk when response finishes
						let filename = fileNameFromPath(options.url);
						if (filename.indexOf('?')) {
							// strip any params if were any
							filename = filename.split('?')[0];
						}
						localPath = path.join(knownFolders.documents().path, filename);
						makeRemoteRequest();
					}

					// save settings
					TNSHttpSettings.currentlySavedImages[options.url] = {
						...(imageSetting || {}),
						date: Date.now(),
						requests: requests + 1,
						localPath,
					};
					ApplicationSettings.setString(SaveImageStorageKey, JSON.stringify(TNSHttpSettings.currentlySavedImages));
				} else {
					makeRemoteRequest();
				}

				requestIdCounter++;
			} catch (ex) {
				reject({
					type: HttpError.Error,
					message: ex.message,
				});
			}
		});
		request['cancel'] = function () {
			com.github.triniwiz.async.Async2.Http.cancelRequest(id);
		};
		return request;
	}

	public static getFile(options: HttpDownloadRequestOptions): CancellablePromise {
		const headers: Headers = {};
		let statusCode = 0;
		let id;
		const counter = requestIdCounter;
		const request = <CancellablePromise>new Promise<any>((resolve, reject) => {
			try {
				// initialize the options
				const javaOptions = Http.buildJavaDownloadOptions(options);

				if (TNSHttpSettings.debug) {
					// @ts-ignore
					if (global.__inspector && global.__inspector.isConnected) {
						NetworkAgent.requestWillBeSent(requestIdCounter, options);
					}
				}

				const makeRemoteRequest = () => {
					const callback = new com.github.triniwiz.async.Async2.Http.Callback({
						onCancel(param: any): void {
							reject({
								type: HttpError.Cancelled,
								result: param,
							});
							requestCallbacks.delete(id);
						},
						onComplete(result: any): void {
							if (result && result.headers) {
								const length = result.headers.size();
								let pair;
								for (let i = 0; i < length; i++) {
									pair = result.headers.get(i);
									addHeader(headers, pair.key, pair.value);
								}
							}
							// send response data (for requestId) to network debugger

							let contentType = headers['Content-Type'];
							const hasContentType = !Utils.isNullOrUndefined(contentType);
							if (!hasContentType) {
								contentType = headers['content-type'];
							}
							let acceptHeader;

							if (!hasContentType) {
								acceptHeader = headers['Accept'];
								if (Utils.isNullOrUndefined(acceptHeader)) {
									acceptHeader = headers['accept'];
								}
							} else {
								acceptHeader = contentType;
							}

							let returnType = 'text/plain';
							if (!Utils.isNullOrUndefined(acceptHeader) && Utils.isString(acceptHeader)) {
								let acceptValues = acceptHeader.split(',');
								let quality = [];
								let defaultQuality = [];
								let customQuality = [];
								for (let value of acceptValues) {
									if (value.indexOf(';q=') > -1) {
										customQuality.push(value);
									} else {
										defaultQuality.push(value);
									}
								}
								customQuality = customQuality.sort((a, b) => {
									const a_quality = parseFloat(a.substring(a.indexOf(';q=')).replace(';q=', ''));
									const b_quality = parseFloat(b.substring(b.indexOf(';q=')).replace(';q=', ''));
									return b_quality - a_quality;
								});
								quality.push(...defaultQuality);
								quality.push(...customQuality);
								returnType = quality[0];
							}

							result['statusCode'] = statusCode;

							if (returnType === '*/*' || !hasContentType) {
								returnType = contentType as string;
							}

							if (TNSHttpSettings.debug) {
								// send response data (for requestId) to network debugger
								// @ts-ignore
								if (global.__inspector && global.__inspector.isConnected) {
									NetworkAgent.responseReceived(
										counter,
										{
											url: result.url,
											statusCode,
											headers,
											responseAsString: Utils.isString(result.contentText) ? result.contentText : result.content?.toString() || '',
											responseAsImage: null, // TODO needs base64 Image
										} as any,
										headers
									);
								}
							}

							resolve(result.filePath);
							requestCallbacks.delete(id);
						},
						onError(param0: string, param1: java.lang.Exception): void {
							reject({
								type: HttpError.Error,
								message: param0,
							});
							requestCallbacks.delete(id);
						},
						onHeaders(jHeaders: any, status: number): void {
							statusCode = status;
							const length = jHeaders.size();
							for (let i = 0; i < length; i++) {
								const pair = jHeaders.get(i);
								addHeader(headers, pair.key, pair.value);
							}
							if (options.onHeaders) {
								options.onHeaders(headers, statusCode);
							}
							requestCallbacks.delete(id);
						},
						onLoading(): void {
							options.onLoading();
							requestCallbacks.delete(id);
						},
						onProgress(lengthComputable: boolean, loaded: number, total: number): void {
							if (options.onProgress) {
								options.onProgress({
									lengthComputable,
									loaded,
									total,
								});
							}
							requestCallbacks.delete(id);
						},
						onTimeout(): void {
							reject({
								type: HttpError.Timeout,
							});
							requestCallbacks.delete(id);
						},
					});
					id = com.github.triniwiz.async.Async2.Http.getFileRequest(javaOptions, callback);
					requestCallbacks.set(id, callback);
				};
				makeRemoteRequest();
				requestIdCounter++;
			} catch (ex) {
				reject({
					type: HttpError.Error,
					message: ex.message,
				});
			}
		});
		request['cancel'] = function () {
			com.github.triniwiz.async.Async2.Http.cancelRequest(id);
		};
		return request;
	}
}

function serialize(data: any): any {
	let store;
	switch (typeof data) {
		case 'string':
		case 'boolean':
		case 'number': {
			return data;
		}

		case 'object': {
			if (!data) {
				return null;
			}

			if (data instanceof Date) {
				return data.toJSON();
			}
			if (Array.isArray(data)) {
				store = new org.json.JSONArray();
				data.forEach((item) => store.put(serialize(item)));
				return store;
			}
			store = new org.json.JSONObject();
			Object.keys(data).forEach((key) => store.put(key, serialize(data[key])));
			return store;
		}

		default:
			return null;
	}
}

function deserialize(data): any {
	if (Utils.isNullOrUndefined(data)) {
		return null;
	}
	if (typeof data !== 'object') {
		return data;
	}

	if (typeof data.getClass === 'function') {
		let store;
		switch (data.getClass().getName()) {
			case 'java.lang.String': {
				return String(data);
			}
			case 'java.lang.Boolean': {
				return String(data) === 'true';
			}
			case 'java.lang.Integer':
			case 'java.lang.Long':
			case 'java.lang.Double':
			case 'java.lang.Short': {
				return Number(data);
			}
			case 'org.json.JSONArray': {
				store = [];
				for (let j = 0; j < data.length(); j++) {
					store[j] = deserialize(data.get(j));
				}
				break;
			}
			case 'org.json.JSONObject': {
				store = {};
				let i = data.keys();
				while (i.hasNext()) {
					let key = i.next();
					store[key] = deserialize(data.get(key));
				}
				break;
			}
			default:
				store = null;
				break;
		}
		return store;
	} else {
		return data;
	}
}

function decodeResponse(raw: any, encoding?: HttpResponseEncoding): any {
	let charsetName = 'UTF-8';
	if (encoding === HttpResponseEncoding.GBK) {
		charsetName = 'GBK';
	}
	return new java.lang.String(raw.array(), charsetName);
}

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
