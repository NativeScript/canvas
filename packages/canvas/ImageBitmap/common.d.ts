export declare abstract class ImageBitmapBase {
    private nativeInstance;
    constructor(nativeInstance: any);
    get native(): any;
    abstract readonly width: number;
    abstract readonly height: number;
    abstract close(): any;
}
