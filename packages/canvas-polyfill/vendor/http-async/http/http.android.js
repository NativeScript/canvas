import { fileNameFromPath, HttpError, isImageUrl, SaveImageStorageKey, TNSHttpSettings } from './http-request-common';
import { NetworkAgent } from '@nativescript/core/debugger';
import { File, Folder, knownFolders, path, ApplicationSettings } from '@nativescript/core';
import { FileManager } from '../file/file';
import { isString, isObject, isNullOrUndefined } from '@nativescript/core/utils/types';
export var HttpResponseEncoding;
(function (HttpResponseEncoding) {
    HttpResponseEncoding[HttpResponseEncoding["UTF8"] = 0] = "UTF8";
    HttpResponseEncoding[HttpResponseEncoding["GBK"] = 1] = "GBK";
})(HttpResponseEncoding || (HttpResponseEncoding = {}));
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
    505: 'HTTP Version Not Supported'
};
function parseJSON(source) {
    const src = source.trim();
    if (src.lastIndexOf(')') === src.length - 1) {
        return JSON.parse(src.substring(src.indexOf('(') + 1, src.lastIndexOf(')')));
    }
    return JSON.parse(src);
}
const textTypes = [
    'text/plain',
    'application/xml',
    'application/rss+xml',
    'text/html',
    'text/xml'
];
const isTextContentType = (contentType) => {
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
    constructor() {
    }
    static buildJavaOptions(options) {
        if (!isString(options.url)) {
            throw new Error('Http request must provide a valid url.');
        }
        let javaOptions = new com.github.triniwiz.async.Async.Http.RequestOptions();
        javaOptions.url = options.url;
        let method;
        if (isString(typeof options.method)) {
            javaOptions.method = options.method;
            method = options.method.toLowerCase();
        }
        if ((method && method === 'post') || method === 'put') {
            if (isString(options.content)) {
                javaOptions.content = new java.lang.String(options.content);
            }
            else if (isObject(options.content)) {
                javaOptions.content = serialize(options.content);
            }
        }
        if (typeof options.timeout === 'number') {
            javaOptions.timeout = options.timeout;
        }
        if (options.headers) {
            const arrayList = new java.util.ArrayList();
            const pair = com.github.triniwiz.async.Async.Http.KeyValuePair;
            if (options.headers instanceof Map) {
                options.headers.forEach((value, key) => {
                    arrayList.add(new pair(key, value + ''));
                });
            }
            else {
                for (let key in options.headers) {
                    arrayList.add(new pair(key, options.headers[key] + ''));
                }
            }
            javaOptions.headers = arrayList;
        }
        return javaOptions;
    }
    static buildJavaDownloadOptions(options) {
        if (!isString(options.url)) {
            throw new Error('Http request must provide a valid url.');
        }
        const javaOptions = new com.github.triniwiz.async.Async.Http.DownloadRequestOptions();
        javaOptions.url = options.url;
        if (typeof options.timeout === 'number') {
            javaOptions.timeout = options.timeout;
        }
        if (typeof options.filePath === 'string') {
            javaOptions.filePath = options.filePath;
        }
        else {
            Folder.fromPath(path.join(knownFolders.temp().path, 'async_http'));
            javaOptions.filePath = path.join(knownFolders.temp().path, 'async_http', java.util.UUID.randomUUID().toString());
        }
        if (options.headers) {
            const arrayList = new java.util.ArrayList();
            const pair = com.github.triniwiz.async.Async.Http.KeyValuePair;
            if (options.headers instanceof Map) {
                options.headers.forEach((value, key) => {
                    arrayList.add(new pair(key, value + ''));
                });
            }
            else {
                for (let key in options.headers) {
                    arrayList.add(new pair(key, options.headers[key] + ''));
                }
            }
            javaOptions.headers = arrayList;
        }
        return javaOptions;
    }
    request(options) {
        const headers = {};
        let statusCode = 0;
        let id;
        const counter = requestIdCounter;
        const request = new Promise((resolve, reject) => {
            try {
                const javaOptions = Http.buildJavaOptions(options);
                if (TNSHttpSettings.debug) {
                    if (global.__inspector && global.__inspector.isConnected) {
                        NetworkAgent.requestWillBeSent(requestIdCounter, options);
                    }
                }
                const makeRemoteRequest = () => {
                    const callback = new com.github.triniwiz.async.Async.Http.Callback({
                        onCancel(param) {
                            reject({
                                type: HttpError.Cancelled,
                                result: param
                            });
                            requestCallbacks.delete(id);
                        },
                        onComplete(result) {
                            let content;
                            let responseText;
                            let _isString = false;
                            if (result.content instanceof org.json.JSONObject || result.content instanceof org.json.JSONArray) {
                                content = deserialize(result.content);
                                responseText = result.contentText;
                                _isString = true;
                            }
                            else {
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
                            let contentType = headers['Content-Type'];
                            if (isNullOrUndefined(contentType)) {
                                contentType = headers['content-type'];
                            }
                            let acceptHeader;
                            if (isNullOrUndefined(contentType)) {
                                acceptHeader = headers['Accept'];
                                if (isNullOrUndefined(acceptHeader)) {
                                    acceptHeader = headers['accept'];
                                }
                            }
                            else {
                                acceptHeader = contentType;
                            }
                            let returnType = 'text/plain';
                            if (!isNullOrUndefined(acceptHeader) && isString(acceptHeader)) {
                                let acceptValues = acceptHeader.split(',');
                                let quality = [];
                                let defaultQuality = [];
                                let customQuality = [];
                                for (let value of acceptValues) {
                                    if (value.indexOf(';q=') > -1) {
                                        customQuality.push(value);
                                    }
                                    else {
                                        defaultQuality.push(value);
                                    }
                                }
                                customQuality = customQuality.sort((a, b) => {
                                    const a_quality = parseFloat(a.substring(a.indexOf(';q=')).replace(';q=', ''));
                                    const b_quality = parseFloat(b.substring(b.indexOf(';q=')).replace(';q=', ''));
                                    return (b_quality - a_quality);
                                });
                                quality.push(...defaultQuality);
                                quality.push(...customQuality);
                                returnType = quality[0];
                            }
                            result['statusCode'] = statusCode;
                            if (TNSHttpSettings.debug) {
                                if (global.__inspector && global.__inspector.isConnected) {
                                    NetworkAgent.responseReceived(counter, {
                                        url: result.url,
                                        statusCode,
                                        headers,
                                        responseAsString: isString ? (result.contentText ? result.contentText : result.content.toString()) : null,
                                        responseAsImage: null
                                    }, headers);
                                }
                            }
                            if (isTextContentType(returnType) && isNullOrUndefined(responseText)) {
                                responseText = result.contentText;
                            }
                            if (TNSHttpSettings.saveImage && TNSHttpSettings.currentlySavedImages && TNSHttpSettings.currentlySavedImages[this._url]) {
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
                                headers: headers
                            });
                            requestCallbacks.delete(id);
                        },
                        onError(param0, param1) {
                            reject({
                                type: HttpError.Error,
                                message: param0
                            });
                            requestCallbacks.delete(id);
                        },
                        onHeaders(jHeaders, status) {
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
                        }, onLoading() {
                            options.onLoading();
                            requestCallbacks.delete(id);
                        }, onProgress(lengthComputable, loaded, total) {
                            if (options.onProgress) {
                                options.onProgress({
                                    lengthComputable,
                                    loaded,
                                    total
                                });
                            }
                            requestCallbacks.delete(id);
                        },
                        onTimeout() {
                            reject({
                                type: HttpError.Timeout
                            });
                            requestCallbacks.delete(id);
                        }
                    });
                    id = com.github.triniwiz.async.Async.Http.makeRequest(javaOptions, callback);
                    requestCallbacks.set(id, callback);
                };
                if (TNSHttpSettings.saveImage && isImageUrl(options.url)) {
                    if (!TNSHttpSettings.currentlySavedImages) {
                        const stored = ApplicationSettings.getString(SaveImageStorageKey);
                        if (stored) {
                            try {
                                TNSHttpSettings.currentlySavedImages = JSON.parse(stored);
                            }
                            catch (err) {
                                TNSHttpSettings.currentlySavedImages = {};
                            }
                        }
                        else {
                            TNSHttpSettings.currentlySavedImages = {};
                        }
                    }
                    const imageSetting = TNSHttpSettings.currentlySavedImages[options.url];
                    const requests = imageSetting ? imageSetting.requests : 0;
                    let localPath;
                    if (imageSetting && imageSetting.localPath && File.exists(imageSetting.localPath)) {
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
                                    'Content-Type': 'arraybuffer'
                                }
                            });
                        });
                    }
                    else if (requests >= TNSHttpSettings.saveImage.numberOfRequests) {
                        let filename = fileNameFromPath(options.url);
                        if (filename.indexOf('?')) {
                            filename = filename.split('?')[0];
                        }
                        localPath = path.join(knownFolders.documents().path, filename);
                        makeRemoteRequest();
                    }
                    TNSHttpSettings.currentlySavedImages[options.url] = Object.assign(Object.assign({}, (imageSetting || {})), { date: Date.now(), requests: requests + 1, localPath });
                    ApplicationSettings.setString(SaveImageStorageKey, JSON.stringify(TNSHttpSettings.currentlySavedImages));
                }
                else {
                    makeRemoteRequest();
                }
                requestIdCounter++;
            }
            catch (ex) {
                reject({
                    type: HttpError.Error,
                    message: ex.message
                });
            }
        });
        request['cancel'] = function () {
            com.github.triniwiz.async.Async.Http.cancelRequest(id);
        };
        return request;
    }
    static getFile(options) {
        const headers = {};
        let statusCode = 0;
        let id;
        const counter = requestIdCounter;
        const request = new Promise((resolve, reject) => {
            try {
                const javaOptions = Http.buildJavaDownloadOptions(options);
                if (TNSHttpSettings.debug) {
                    if (global.__inspector && global.__inspector.isConnected) {
                        NetworkAgent.requestWillBeSent(requestIdCounter, options);
                    }
                }
                const makeRemoteRequest = () => {
                    const callback = new com.github.triniwiz.async.Async.Http.Callback({
                        onCancel(param) {
                            reject({
                                type: HttpError.Cancelled,
                                result: param
                            });
                            requestCallbacks.delete(id);
                        },
                        onComplete(result) {
                            if (result && result.headers) {
                                const length = result.headers.size();
                                let pair;
                                for (let i = 0; i < length; i++) {
                                    pair = result.headers.get(i);
                                    addHeader(headers, pair.key, pair.value);
                                }
                            }
                            let contentType = headers['Content-Type'];
                            if (isNullOrUndefined(contentType)) {
                                contentType = headers['content-type'];
                            }
                            let acceptHeader;
                            if (isNullOrUndefined(contentType)) {
                                acceptHeader = headers['Accept'];
                                if (isNullOrUndefined(acceptHeader)) {
                                    acceptHeader = headers['accept'];
                                }
                            }
                            else {
                                acceptHeader = contentType;
                            }
                            let returnType = 'text/plain';
                            if (!isNullOrUndefined(acceptHeader) && isString(acceptHeader)) {
                                let acceptValues = acceptHeader.split(',');
                                let quality = [];
                                let defaultQuality = [];
                                let customQuality = [];
                                for (let value of acceptValues) {
                                    if (value.indexOf(';q=') > -1) {
                                        customQuality.push(value);
                                    }
                                    else {
                                        defaultQuality.push(value);
                                    }
                                }
                                customQuality = customQuality.sort((a, b) => {
                                    const a_quality = parseFloat(a.substring(a.indexOf(';q=')).replace(';q=', ''));
                                    const b_quality = parseFloat(b.substring(b.indexOf(';q=')).replace(';q=', ''));
                                    return (b_quality - a_quality);
                                });
                                quality.push(...defaultQuality);
                                quality.push(...customQuality);
                                returnType = quality[0];
                            }
                            result['statusCode'] = statusCode;
                            if (TNSHttpSettings.debug) {
                                if (global.__inspector && global.__inspector.isConnected) {
                                    NetworkAgent.responseReceived(counter, {
                                        url: result.url,
                                        statusCode,
                                        headers,
                                        responseAsString: isString ? (result.contentText ? result.contentText : result.content.toString()) : null,
                                        responseAsImage: null
                                    }, headers);
                                }
                            }
                            resolve(result.filePath);
                            requestCallbacks.delete(id);
                        },
                        onError(param0, param1) {
                            reject({
                                type: HttpError.Error,
                                message: param0
                            });
                            requestCallbacks.delete(id);
                        },
                        onHeaders(jHeaders, status) {
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
                        }, onLoading() {
                            options.onLoading();
                            requestCallbacks.delete(id);
                        }, onProgress(lengthComputable, loaded, total) {
                            if (options.onProgress) {
                                options.onProgress({
                                    lengthComputable,
                                    loaded,
                                    total
                                });
                            }
                            requestCallbacks.delete(id);
                        },
                        onTimeout() {
                            reject({
                                type: HttpError.Timeout
                            });
                            requestCallbacks.delete(id);
                        }
                    });
                    id = com.github.triniwiz.async.Async.Http.getFileRequest(javaOptions, callback);
                    requestCallbacks.set(id, callback);
                };
                makeRemoteRequest();
                requestIdCounter++;
            }
            catch (ex) {
                reject({
                    type: HttpError.Error,
                    message: ex.message
                });
            }
        });
        request['cancel'] = function () {
            com.github.triniwiz.async.Async.Http.cancelRequest(id);
        };
        return request;
    }
}
function serialize(data) {
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
function deserialize(data) {
    if (isNullOrUndefined(data)) {
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
    }
    else {
        return data;
    }
}
function decodeResponse(raw, encoding) {
    let charsetName = 'UTF-8';
    if (encoding === HttpResponseEncoding.GBK) {
        charsetName = 'GBK';
    }
    return new java.lang.String(raw.array(), charsetName);
}
export function addHeader(headers, key, value) {
    if (!headers[key]) {
        headers[key] = value;
    }
    else if (Array.isArray(headers[key])) {
        headers[key].push(value);
    }
    else {
        const values = [headers[key]];
        values.push(value);
        headers[key] = values;
    }
}
//# sourceMappingURL=http.android.js.map
