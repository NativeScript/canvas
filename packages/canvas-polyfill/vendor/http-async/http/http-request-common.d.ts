export interface ISaveImageSettings {
    numberOfRequests?: number;
    removeAfterDays?: number;
    storageKey?: string;
}
export declare const SaveImageStorageKey = "http.saved-images";
export declare function isImageUrl(url: string): boolean;
export declare function fileNameFromPath(fullPath: string): string;
export declare class TNSHttpSettings {
    static debug: boolean;
    static saveImage: ISaveImageSettings;
    static currentlySavedImages: {
        [url: string]: {
            date: number;
            requests: number;
            localPath?: string;
        };
    };
}
export declare class ProgressEvent {
    private _type;
    private _lengthComputable;
    private _loaded;
    private _total;
    private _target;
    constructor(type: string, data?: {
        lengthComputable: boolean;
        loaded: number;
        total: number;
        target: any;
    });
    get lengthComputable(): boolean;
    get loaded(): number;
    get total(): number;
    get type(): string;
    get target(): any;
}
export declare type Headers = {
    [key: string]: string | string[];
} | Map<string, string>;
export declare enum HttpError {
    Error = 0,
    Timeout = 1,
    Cancelled = 2
}
export interface HttpRequestOptions {
    method: string;
    url: string;
    headers?: Headers;
    content?: any;
    timeout?: number;
    onProgress?: (event: any) => void;
    onHeaders?: (...args: any[]) => void;
    onLoading?: () => void;
}
export interface HttpDownloadRequestOptions {
    url: string;
    headers?: Headers;
    filePath?: any;
    timeout?: number;
    onProgress?: (event: any) => void;
    onHeaders?: (...args: any[]) => void;
    onLoading?: () => void;
}
export declare enum HttpResponseEncoding {
    UTF8 = 0,
    GBK = 1
}
export interface HttpResponse {
    statusCode: number;
    content: any;
    headers: Headers;
    url: string;
}
export interface HttpDownloadResponse {
    statusCode: number;
    filePath: string;
    headers: Headers;
    url: string;
}
