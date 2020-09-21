import { Headers, HttpDownloadRequestOptions, HttpRequestOptions } from './http-request-common';
export declare type CancellablePromise = Promise<any> & {
    cancel: () => void;
};
export declare enum HttpResponseEncoding {
    UTF8 = 0,
    GBK = 1
}
export declare class Http {
    constructor();
    private static buildJavaOptions;
    private static buildJavaDownloadOptions;
    request(options: HttpRequestOptions): CancellablePromise;
    static getFile(options: HttpDownloadRequestOptions): CancellablePromise;
}
export declare function addHeader(headers: Headers, key: string, value: string): void;
