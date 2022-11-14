declare var com;

export class FileManager {

    public static writeFile(bytes: any, path: string, callback: (...args) => void) {
        const listener = new com.github.triniwiz.async.Async2.FileManager.Callback({
            onError(param0: string, param1: java.lang.Exception): void {
                callback(param0, null);
            },
            onComplete(param0: any): void {
                callback(null, param0);
            }
        });
        if (bytes instanceof java.nio.ByteBuffer) {
            com.github.triniwiz.async.Async2.FileManager.writeFile(bytes, path, listener);
        } else if (bytes instanceof ArrayBuffer) {
            if ((bytes as any).nativeObject) {
                com.github.triniwiz.async.Async2.FileManager.writeFile((bytes as any).nativeObject, path, listener);
            }
        } else {
            com.github.triniwiz.async.Async2.FileManager.writeFile(bytes, path, listener);
        }
    }

    public static readFile(path: string, options: Options = {asStream: false}, callback: (...args) => void) {
        //const opts = new com.github.triniwiz.async.Async2.FileManager.Options();
        //opts.asStream = options.asStream;
        com.github.triniwiz.async.Async2.FileManager.readFileBuffer(path, null, new com.github.triniwiz.async.Async2.FileManager.Callback({
            onError(param0: string, param1: java.lang.Exception): void {
                callback(param0, null);
            },
            onComplete(param0: any): void {
                callback(null, param0);
            }
        }));
    }

    public static deleteFile(path: string, options: Options = {asStream: false}, callback: (...args) => void) {
      // TODO
      callback(null, true);
    }
}

export interface Options {
    asStream?: boolean;
}
