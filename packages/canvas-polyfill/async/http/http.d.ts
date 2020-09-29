import { Headers, HttpDownloadRequestOptions, HttpRequestOptions } from './http-request-common';

export enum HttpResponseEncoding {
    UTF8,
    GBK
}

export type CancellablePromise = Promise<any> & { cancel: () => void };

export function addHeader(headers: Headers, key: string, value: string): void;

export class Http {
    constructor();

    request(options: HttpRequestOptions): CancellablePromise;

    public static getFile(options: HttpDownloadRequestOptions): CancellablePromise;

    cancel();
}
