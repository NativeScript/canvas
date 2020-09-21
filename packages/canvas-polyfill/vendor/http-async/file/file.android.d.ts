export declare class FileManager {
    static writeFile(bytes: any, path: string, callback: (...args: any[]) => void): void;
    static readFile(path: string, options: Options, callback: (...args: any[]) => void): void;
    static deleteFile(path: string, options: Options, callback: (...args: any[]) => void): void;
}
export interface Options {
    asStream?: boolean;
}
