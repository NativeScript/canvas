import {Http} from '../http/http';
import {HttpError, ProgressEvent} from '../http/http-request-common';
import {FileManager} from '../file/file';
import {isNullOrUndefined, isObject, isFunction} from '@nativescript/core/utils/types';
import {knownFolders, path as filePath, File as fsFile} from '@nativescript/core';

var XMLHttpRequestResponseType;
(function (XMLHttpRequestResponseType) {
  XMLHttpRequestResponseType["empty"] = "";
  XMLHttpRequestResponseType["text"] = "text";
  XMLHttpRequestResponseType["json"] = "json";
  XMLHttpRequestResponseType["document"] = "document";
  XMLHttpRequestResponseType["arraybuffer"] = "arraybuffer";
  XMLHttpRequestResponseType["blob"] = "blob";
})(XMLHttpRequestResponseType || (XMLHttpRequestResponseType = {}));
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
var Status;
(function (Status) {
  Status[Status["UNSENT"] = 0] = "UNSENT";
  Status[Status["OPENED"] = 0] = "OPENED";
  Status[Status["LOADING"] = 200] = "LOADING";
  Status[Status["DONE"] = 200] = "DONE";
})(Status || (Status = {}));

export class TNSXMLHttpRequestUpload {
  constructor(req) {
    this._listeners = new Map();
    this._request = req;
  }

  addEventListener(eventName, handler) {
    const handlers = this._listeners.get(eventName) || [];
    handlers.push(handler);
    this._listeners.set(eventName, handlers);
  }

  removeEventListener(eventName, toDetach) {
    let handlers = this._listeners.get(eventName) || [];
    handlers = handlers.filter(handler => handler !== toDetach);
    this._listeners.set(eventName, handlers);
  }

  _emitEvent(eventName, ...args) {
    const handlers = this._listeners.get(eventName) || [];
    handlers.forEach(handler => {
      handler.apply(this, ...args);
    });
  }
}

export class TNSXMLHttpRequest {
  constructor() {
    this.UNSENT = 0;
    this.OPENED = 1;
    this.HEADERS_RECEIVED = 2;
    this.LOADING = 3;
    this.DONE = 4;
    this.timeout = 0;
    this._readyState = this.UNSENT;
    this._response = '';
    this._responseType = null;
    this._responseText = null;
    this._request = null;
    this._lastProgress = {lengthComputable: false, loaded: 0, total: 0, target: this};
    this._responseURL = '';
    this._listeners = new Map();
    this.textTypes = [
      'text/plain',
      'application/xml',
      'application/rss+xml',
      'text/html',
      'text/xml',
    ];
    this._status = Status.UNSENT;
    this._http = new Http();
    this._upload = new TNSXMLHttpRequestUpload(this);
  }

  get readyState() {
    return this._readyState;
  }

  get response() {
    return this._response;
  }

  get responseType() {
    return this._responseType;
  }

  get responseText() {
    if (this._responseType === XMLHttpRequestResponseType.text ||
      this._responseType === XMLHttpRequestResponseType.json) {
      return this._responseText;
    }
    return null;
  }

  get responseURL() {
    return this._responseURL;
  }

  get status() {
    return this._status;
  }

  get statusText() {
    if (this._readyState === this.UNSENT ||
      this._readyState === this.OPENED) {
      return '';
    }
    return statuses[this.status];
  }

  get upload() {
    return this._upload;
  }

  get responseXML() {
    const header = this.getResponseHeader('Content-Type') ||
      this.getResponseHeader('content-type');
    const contentType = header && header.toLowerCase();
    if (this.isTextContentType(contentType)) {
      if (this._responseType === XMLHttpRequestResponseType.document) {
        return this.responseText;
      }
    }
    return '';
  }

  isTextContentType(contentType) {
    let result = false;
    for (let i = 0; i < this.textTypes.length; i++) {
      if (contentType.toLowerCase().indexOf(this.textTypes[i]) >= 0) {
        result = true;
        break;
      }
    }
    return result;
  }

  _setResponseType() {
    const header = this.getResponseHeader('Content-Type') ||
      this.getResponseHeader('content-type');
    const contentType = header && header.toLowerCase();
    if (contentType) {
      if (contentType.indexOf('application/json') >= 0 ||
        contentType.indexOf('+json') >= 0) {
        this._responseType = XMLHttpRequestResponseType.json;
      } else if (this.isTextContentType(contentType)) {
        if (contentType.indexOf('text/html') ||
          contentType.indexOf('text/xml')) {
          this._responseType = XMLHttpRequestResponseType.document;
        }
        this._responseType = XMLHttpRequestResponseType.text;
      }
    } else {
      this._responseType = XMLHttpRequestResponseType.text;
    }
  }

  getAllResponseHeaders() {
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

  getResponseHeader(header) {
    if (typeof header === 'string' &&
      this._readyState > 1 &&
      this._headers) {
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

  overrideMimeType(mime) {
  }

  set responseType(value) {
    if (value === XMLHttpRequestResponseType.empty ||
      value in XMLHttpRequestResponseType) {
      this._responseType = value;
    } else {
      throw new Error(`Response type of '${value}' not supported.`);
    }
  }

  _addToStringOnResponse() {
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

  _toJSString(data) {
    const encodings = ['UTF-8', 'US-ASCII', 'ISO-8859-1', null];
    let value;
    const count = encodings.length;
    for (let i = 0; i < count; i++) {
      let encodingType = encodings[i];
      if (encodingType === null) {
        value = (new java.lang.String(data)).toString();
        break;
      }
      try {
        let encoding = java.nio.charset.Charset.forName(encodingType);
        value = (new java.lang.String(data, encoding)).toString();
        break;
      } catch (e) {
      }
    }
    return value;
  }

  open(method, url, async = true, username = null, password = null) {
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

  setRequestHeader(header, value) {
    if (this._readyState !== this.OPENED) {
      throw new Error("Failed to execute 'setRequestHeader' on 'XMLHttpRequest': The object's state must be OPENED.");
    }
    if (typeof this._headers === 'object') {
      this._headers[header] = value;
    }
  }

  send(body = null) {
    if (this._readyState !== this.OPENED) {
      throw new Error("Failed to execute 'send' on 'XMLHttpRequest': The object's state must be OPENED");
    }
    if (!this._headers['Accept']) {
      this._headers['Accept'] = '*/*';
    }
    if (typeof this._request.method === 'string' &&
      this._request.method.toLowerCase() === 'get' &&
      typeof this._request.url === 'string' && !this._request.url.startsWith('http')) {
      let path;
      if (this._request.url.startsWith('file://')) {
        path = this._request.url.replace('file://', '');
      } else if (this._request.url.startsWith('~/')) {
        path = filePath.join(knownFolders.currentApp().path, this._request.url.replace('~/', ''));
      } else if (this._request.url.startsWith('/')) {
        path = this._request.url;
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
          if (!this.responseType) {
            this._setResponseType();
          }
          this._status = 200;
          this._httpContent = data;
          this._responseURL = responseURL;
          if (this.responseType === XMLHttpRequestResponseType.json) {
            try {
              if (global.isAndroid) {
                this._responseText = this._toJSString(data);
                this._response = JSON.parse(this._responseText);
              } else {
                this._responseText = NSString.alloc().initWithDataEncoding(data, NSUTF8StringEncoding);
                if (!this._responseText) {
                  this._responseText = NSString.alloc().initWithDataEncoding(data, NSISOLatin1StringEncoding);
                }
                this._response = JSON.parse(this._responseText);
              }
            } catch (e) {
              console.log('json parse error', e);
            }
          } else if (this.responseType === XMLHttpRequestResponseType.text) {
            if (global.isIOS) {
              let code = NSUTF8StringEncoding;
              let encodedString = NSString.alloc().initWithDataEncoding(data, code);
              if (!encodedString) {
                code = NSISOLatin1StringEncoding;
                encodedString = NSString.alloc().initWithDataEncoding(data, code);
              }
              this._responseText = this._response = encodedString.toString();
            } else {
              const response = this._toJSString(data);
              this._responseText = this._response = response
                ? response.toString()
                : '';
            }

          } else if (this.responseType === XMLHttpRequestResponseType.document) {
            if (global.isIOS) {
              let code = NSUTF8StringEncoding;
              let encodedString = NSString.alloc().initWithDataEncoding(data, code);
              if (!encodedString) {
                code = NSISOLatin1StringEncoding;
                encodedString = NSString.alloc().initWithDataEncoding(data, code);
              }
              this._responseText = this._response = encodedString.toString();
            } else {
              let response = this._toJSString(data);
              this._responseText = this._response = response
                ? response
                : '';
            }
          } else if (this.responseType === XMLHttpRequestResponseType.arraybuffer) {
            if (global.isIOS) {
              this._response = interop.bufferFromData(data);
            } else {
              this._response = ArrayBuffer.from(java.nio.ByteBuffer.wrap(data));
            }
          } else if (this.responseType === XMLHttpRequestResponseType.blob) {
            let buffer;
            if (global.isIOS) {
              buffer = interop.bufferFromData(data);
            } else {
              buffer = ArrayBuffer.from(java.nio.ByteBuffer.wrap(data));
            }
            this._response = new Blob([buffer]);
          }
          let size = 0;
          if (global.isIOS) {
            if (data instanceof NSData) {
              size = data.length;
            }
          } else {
            size = data ? data.length : 0;
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
    const request = {
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
            contentLength =
              parseInt(this._headers['Content-Length'], 10) || -1;
          }
          if (this._headers['content-length']) {
            contentLength =
              parseInt(this._headers['content-length'], 10) || -1;
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
    if (method === 'post' || method === 'put') {
      request.onProgress = (event) => {
        this._lastProgress = Object.assign(Object.assign({}, (this._lastProgress || {})), event);
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
        this._setResponseType();
        this._status = res.statusCode;
        this._httpContent = res.content;
        this._responseURL = res.url;
        if (this.responseType === XMLHttpRequestResponseType.json) {
          if (typeof res.content === 'string') {
            this._responseText = res.content;
            try {
              this._response = JSON.parse(this.responseText);
            } catch (err) {
            }
          } else if (typeof res.content === 'object') {
            this._response = res.content;
            this._responseText = res.responseText;
          } else {
            if (global.isIOS) {
              if (res.content instanceof NSData) {
                let code = NSUTF8StringEncoding;
                let encodedString = NSString.alloc().initWithDataEncoding(res.content, code);
                if (!encodedString) {
                  code = NSISOLatin1StringEncoding;
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
            this._responseText = JSON.stringify(res.content);
          } else {
            if (global.isIOS) {
              if (res.content instanceof NSData) {
                let code = NSUTF8StringEncoding;
                let encodedString = NSString.alloc().initWithDataEncoding(res.content, code);
                if (!encodedString) {
                  code = NSISOLatin1StringEncoding;
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
            if (global.isIOS) {
              if (res.content instanceof NSData) {
                let code = NSUTF8StringEncoding;
                let encodedString = NSString.alloc().initWithDataEncoding(res.content, code);
                if (!encodedString) {
                  code = NSISOLatin1StringEncoding;
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
          if (global.isIOS) {
            this._response = interop.bufferFromData(res.content);
          } else {
            this._response = ArrayBuffer.from(res.content);
          }
        } else if (this.responseType === XMLHttpRequestResponseType.blob) {
          const buffer = ArrayBuffer.from(res.content);
          this._response = new Blob([buffer]);
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
        const type = error.type;
        const method = this._request.method.toLowerCase();
        switch (type) {
          case HttpError.Cancelled:
            if (this.onabort) {
              this.onabort();
            }
            const abortEvent = new ProgressEvent('abort', this._lastProgress);
            if (this._upload &&
              (method === 'post' || method === 'put')) {
              this._upload._emitEvent('abort', abortEvent);
            }
            this.emitEvent('abort', abortEvent);
            if (this.onloadend) {
              this.onloadend();
            }
            const _loadendEvent = new ProgressEvent('loadend', this._lastProgress);
            if (this._upload &&
              (method === 'post' || method === 'put')) {
              this._upload._emitEvent('loadend', _loadendEvent);
            }
            this.emitEvent('loadend', _loadendEvent);
            if (this._readyState === this.UNSENT ||
              this._readyState === this.OPENED ||
              this._readyState === this.DONE) {
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
            if (this._upload &&
              (method === 'post' || method === 'put')) {
              this._upload._emitEvent('timeout', timeoutEvent);
            }
            this.emitEvent('timeout', timeoutEvent);
            break;
          case HttpError.Error:
            if (this.onerror) {
              this.onerror(error.message);
            }
            const errorEvent = new ProgressEvent('error', this._lastProgress);
            if (this._upload &&
              (method === 'post' || method === 'put')) {
              this._upload._emitEvent('error', errorEvent);
            }
            this.emitEvent('error', errorEvent);
            if (this.onloadend) {
              this.onloadend();
            }
            const loadendEvent = new ProgressEvent('loadend', this._lastProgress);
            if (this._upload &&
              (method === 'post' || method === 'put')) {
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

  addEventListener(eventName, handler) {
    const handlers = this._listeners.get(eventName) || [];
    handlers.push(handler);
    this._listeners.set(eventName, handlers);
  }

  removeEventListener(eventName, toDetach) {
    let handlers = this._listeners.get(eventName) || [];
    handlers = handlers.filter((handler) => handler !== toDetach);
    this._listeners.set(eventName, handlers);
  }

  emitEvent(eventName, ...args) {
    const handlers = this._listeners.get(eventName) || [];
    handlers.forEach((handler) => {
      handler.apply(this, ...args);
    });
  }

  dispatchEvent(event) {
    return false;
  }

  _updateReadyStateChange(state) {
    this._readyState = state;
    if (this.onreadystatechange) {
      this.onreadystatechange();
    }
  }
}

export class FormData {
  constructor() {
    this._data = new Map();
  }

  append(name, value) {
    this._data.set(name, value);
  }

  toString() {
    let arr = [];
    this._data.forEach(function (value, name, map) {
      arr.push(`${encodeURIComponent(name)}=${encodeURIComponent(value)}`);
    });
    return arr.join("&");
  }
}

export class Blob {
  constructor(chunks = [], opts = {}) {
    this[Symbol.toStringTag] = "Blob";
    const dataChunks = [];
    for (const chunk of chunks) {
      if (chunk instanceof Blob) {
        dataChunks.push(chunk._buffer);
      } else if (typeof chunk === "string") {
        const textEncoder = new TextEncoder();
        dataChunks.push(textEncoder.encode(chunk));
      } else if (chunk instanceof DataView) {
        dataChunks.push(new Uint8Array(chunk.buffer.slice(0)));
      } else if (chunk instanceof ArrayBuffer || ArrayBuffer.isView(chunk)) {
        dataChunks.push(new Uint8Array(ArrayBuffer.isView(chunk)
          ? chunk.buffer.slice(0)
          : chunk.slice(0)));
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
    this._type = opts.type || "";

    if(this._type === ''){
      var arr = buffer.subarray(0, 4);
      var header = '';
      for (var i = 0; i < arr.length; i++) {
        header += arr[i].toString(16);
      }
      // Check the file signature against known types
      var type = '';
      switch (header) {
        case '89504e47':
          this._type = 'image/png';
          break;
        case '47494638':
          this._type = 'image/gif';
          break;
        case 'ffd8ffe0':
        case 'ffd8ffe1':
        case 'ffd8ffdb':
          this._type = 'image/jpg';
          break;
        case 'ffd8ffe2':
          this._type = 'image/jpeg';
          break;
        case '25504446':
          this._type = 'application/pdf';
          break;
      }
    }
    if (/[^\u0020-\u007E]/.test(this._type)) {
      this._type = "";
    } else {
      this._type = this._type.toLowerCase();
    }
  }

  get size() {
    return this._size;
  }

  get type() {
    return this._type;
  }

  arrayBuffer() {
    return Promise.resolve(this._buffer);
  }

  text() {
    const textDecoder = new TextDecoder();
    return Promise.resolve(textDecoder.decode(this._buffer));
  }

  slice(start, end, type) {
    const slice = this._buffer.slice(start || 0, end || this._buffer.length);
    return new Blob([slice], {type: type});
  }

  stream() {
    throw new Error("stream is currently not supported");
  }

  toString() {
    return "[object Blob]";
  }
}

Blob.InternalAccessor = class {
  static getBuffer(blob) {
    return blob._buffer;
  }
};

export class File extends Blob {
  constructor(chunks, name, opts = {}) {
    super(chunks, opts);
    this[Symbol.toStringTag] = "File";
    this._name = name.replace(/\//g, ":");
    this._lastModified =
      opts.lastModified
        ? new Date(opts.lastModified).valueOf()
        : Date.now();
  }

  get name() {
    return this._name;
  }

  get lastModified() {
    return this._lastModified;
  }

  toString() {
    return "[object File]";
  }
}

export class FileReader {
  constructor() {
    this.EMPTY = 0;
    this.LOADING = 1;
    this.DONE = 2;
    this._listeners = new Map();
    this[Symbol.toStringTag] = "FileReader";
  }

  get readyState() {
    return this._readyState;
  }

  get result() {
    return this._result;
  }

  _array2base64(input) {
    var byteToCharMap = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    var output = [];
    for (var i = 0; i < input.length; i += 3) {
      var byte1 = input[i];
      var haveByte2 = i + 1 < input.length;
      var byte2 = haveByte2 ? input[i + 1] : 0;
      var haveByte3 = i + 2 < input.length;
      var byte3 = haveByte3 ? input[i + 2] : 0;
      var outByte1 = byte1 >> 2;
      var outByte2 = ((byte1 & 0x03) << 4) | (byte2 >> 4);
      var outByte3 = ((byte2 & 0x0F) << 2) | (byte3 >> 6);
      var outByte4 = byte3 & 0x3F;
      if (!haveByte3) {
        outByte4 = 64;
        if (!haveByte2) {
          outByte3 = 64;
        }
      }
      output.push(byteToCharMap[outByte1], byteToCharMap[outByte2], byteToCharMap[outByte3], byteToCharMap[outByte4]);
    }
    return output.join("");
  }

  _read(blob, kind) {
    if (!(blob instanceof Blob)) {
      throw new TypeError(`Failed to execute '${kind}' on 'FileReader': parameter 1 is not of type 'Blob'.`);
    }
    this._result = "";
    setTimeout(() => {
      this._readyState = this.LOADING;
      this.emitEvent("load");
      this.emitEvent("loadend");
    });
  }

  emitEvent(eventName, ...args) {
    if (isFunction(this["on" + eventName])) {
      this["on" + eventName](...args);
    }
    let handlers = this._listeners.get(eventName) || [];
    handlers.forEach((handler) => {
      handler(this, ...args);
    });
  }

  addEventListener(eventName, handler) {
    if (["abort", "error", "load", "loadend", "loadstart", "progress"].indexOf(eventName) === -1) {
      throw new Error("Event not supported: " + eventName);
    }
    let handlers = this._listeners.get(eventName) || [];
    handlers.push(handler);
    this._listeners.set(eventName, handlers);
  }

  removeEventListener(eventName, toDetach) {
    let handlers = this._listeners.get(eventName) || [];
    handlers = handlers.filter((handler) => handler !== toDetach);
    this._listeners.set(eventName, handlers);
  }

  readAsDataURL(blob) {
    this._read(blob, "readAsDataURL");
    this._result = `data:${blob.type};base64,${this._array2base64(Blob.InternalAccessor.getBuffer(blob))}`;
  }

  readAsText(blob) {
    this._read(blob, "readAsText");
    const textDecoder = new TextDecoder();
    this._result = textDecoder.decode(Blob.InternalAccessor.getBuffer(blob));
  }

  readAsArrayBuffer(blob) {
    this._read(blob, "readAsArrayBuffer");
    this._result = Blob.InternalAccessor.getBuffer(blob).buffer.slice(0);
  }

  abort() {
  }

  toString() {
    return "[object FileReader]";
  }
}

//# sourceMappingURL=TNSXMLHttpRequest.js.map
