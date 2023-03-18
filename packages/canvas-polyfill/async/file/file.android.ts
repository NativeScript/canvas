declare var com;

export class FileManager {
	public static writeFile(bytes: any, path: string, callback: (...args) => void) {
		const listener = new com.github.triniwiz.async.Async2.FileManager.Callback({
			onError(param0: string, param1: java.lang.Exception): void {
				callback(param0, null);
			},
			onComplete(param0: any): void {
				callback(null, param0);
			},
		});
		if (bytes instanceof java.nio.ByteBuffer) {
			com.github.triniwiz.async.Async2.FileManager.writeFile(bytes.array(), path, listener);
		} else if (bytes instanceof ArrayBuffer) {
			if ((bytes as any).nativeObject) {
				com.github.triniwiz.async.Async2.FileManager.writeFile((bytes as any).nativeObject.array(), path, listener);
			}
		} else {
			com.github.triniwiz.async.Async2.FileManager.writeFile(bytes, path, listener);
		}
	}

	static _readFile;

	public static readFile(path: string, options: Options = { asStream: false }, callback: (...args) => void) {
		//const opts = new com.github.triniwiz.async.Async2.FileManager.Options();
		//opts.asStream = options.asStream;
		if (this._readFile === undefined) {
			this._readFile = global?.CanvasJSIModule?.readFile;
		}
		if (this._readFile) {
			this._readFile(path, (error, buffer) => {
				if (error) {
					callback(new Error(error), null);
				} else {
					callback(null, buffer);
				}
			});
		} else {
			com.github.triniwiz.async.Async2.FileManager.readFile(
				path,
				null,
				new com.github.triniwiz.async.Async2.FileManager.Callback({
					onError(param0: string, param1: java.lang.Exception): void {
						callback(param0, null);
					},
					onComplete(param0: any): void {
						callback(null, param0);
					},
				})
			);
		}
	}

	public static deleteFile(path: string, options: Options = { asStream: false }, callback: (...args) => void) {
		// TODO
		callback(null, true);
	}
}

export interface Options {
	asStream?: boolean;
}
