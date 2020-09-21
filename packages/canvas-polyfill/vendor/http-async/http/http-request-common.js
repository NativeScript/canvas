export const SaveImageStorageKey = 'http.saved-images';
export function isImageUrl(url) {
    return url && /(http(s?):)([/|.|\w|\s|-])*\.(?:jpg|gif|png)/ig.test(url);
}
export function fileNameFromPath(fullPath) {
    let filename = '';
    if (fullPath) {
        filename = fullPath.replace(/^.*[\\\/]/ig, '');
    }
    return filename;
}
export class TNSHttpSettings {
}
TNSHttpSettings.debug = false;
TNSHttpSettings.saveImage = {};
TNSHttpSettings.currentlySavedImages = {};
export class ProgressEvent {
    constructor(type, data = {
        lengthComputable: false,
        loaded: 0,
        total: 0,
        target: {}
    }) {
        this._type = type;
        this._lengthComputable = data.lengthComputable;
        this._loaded = data.loaded;
        this._total = data.total;
        this._target = data.target;
    }
    get lengthComputable() {
        return this._lengthComputable;
    }
    get loaded() {
        return this._loaded;
    }
    get total() {
        return this._total;
    }
    get type() {
        return this._type;
    }
    get target() {
        return this._target;
    }
}
export var HttpError;
(function (HttpError) {
    HttpError[HttpError["Error"] = 0] = "Error";
    HttpError[HttpError["Timeout"] = 1] = "Timeout";
    HttpError[HttpError["Cancelled"] = 2] = "Cancelled";
})(HttpError || (HttpError = {}));
export var HttpResponseEncoding;
(function (HttpResponseEncoding) {
    HttpResponseEncoding[HttpResponseEncoding["UTF8"] = 0] = "UTF8";
    HttpResponseEncoding[HttpResponseEncoding["GBK"] = 1] = "GBK";
})(HttpResponseEncoding || (HttpResponseEncoding = {}));
//# sourceMappingURL=http-request-common.js.map