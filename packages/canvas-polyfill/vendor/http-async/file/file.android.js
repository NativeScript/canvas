export class FileManager {
    static writeFile(bytes, path, callback) {
        const listener = new com.github.triniwiz.async.Async.FileManager.Callback({
            onError(param0, param1) {
                callback(param0, null);
            },
            onComplete(param0) {
                callback(null, param0);
            }
        });
        if (bytes instanceof java.nio.ByteBuffer) {
            com.github.triniwiz.async.Async.FileManager.writeFile(bytes.array(), path, listener);
        }
        else if (bytes instanceof ArrayBuffer) {
            if (bytes.nativeObject) {
                com.github.triniwiz.async.Async.FileManager.writeFile(bytes.nativeObject.array(), path, listener);
            }
        }
        else {
            com.github.triniwiz.async.Async.FileManager.writeFile(bytes, path, listener);
        }
    }
    static readFile(path, options = { asStream: false }, callback) {
        com.github.triniwiz.async.Async.FileManager.readFile(path, null, new com.github.triniwiz.async.Async.FileManager.Callback({
            onError(param0, param1) {
                callback(param0, null);
            },
            onComplete(param0) {
                callback(null, param0);
            }
        }));
    }
    static deleteFile(path, options = { asStream: false }, callback) {
        callback(null, true);
    }
}
//# sourceMappingURL=file.android.js.map