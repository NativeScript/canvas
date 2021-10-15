var ImageBitmapBase = /** @class */ (function () {
    function ImageBitmapBase(nativeInstance) {
        this.nativeInstance = nativeInstance;
    }
    Object.defineProperty(ImageBitmapBase.prototype, "native", {
        get: function () {
            return this.nativeInstance;
        },
        enumerable: false,
        configurable: true
    });
    return ImageBitmapBase;
}());
export { ImageBitmapBase };
//# sourceMappingURL=common.js.map