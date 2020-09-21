import { Headers, HttpDownloadRequestOptions, HttpRequestOptions } from './http-request-common';
export declare type CancellablePromise = Promise<any> & {
    cancel: () => void;
};
export declare enum HttpResponseEncoding {
    UTF8 = 0,
    GBK = 1
}
export declare function addHeader(headers: Headers, key: string, value: string): void;
export declare class Http {
    static _tasks: Map<string, NSURLSessionDataTask>;
    private _session;
    private _sessionDelegate;
    constructor();
    request(options: HttpRequestOptions): CancellablePromise;
    static getFile(options: HttpDownloadRequestOptions): CancellablePromise;
}
