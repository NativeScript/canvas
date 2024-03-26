declare const qos_class_t;

declare const NSSCanvasHelpers;

export class FileManager {
	public static writeFile(bytes: any, path: string, callback: (...args) => void) {
		let native;
		if (bytes instanceof NSData) {
			native = bytes;
		} else if (bytes instanceof ArrayBuffer) {
			native = NSData.dataWithData(bytes as any);
		}

		NSSCanvasHelpers.writeFile(bytes, path, (error, result) => {
			if (error) {
				callback(new Error(error), null);
			} else {
				callback(null, result);
			}
		});
	}

	static _readFile;

	public static readFile(path: string, options: Options = { asStream: false }, callback: (...args) => void) {
		if (this._readFile === undefined) {
			this._readFile = global?.CanvasModule?.readFile;
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
			NSSCanvasHelpers.readFile(path, (error, data) => {
				if (error) {
					callback(new Error(error), null);
				} else {
					callback(null, data);
				}
			});
		}
	}

	public static deleteFile(path: string, options: Options = { asStream: false }, callback: (...args) => void) {
		NSSCanvasHelpers.deleteFile(path, (error, done) => {
			if (error) {
				callback(new Error(error), null);
			} else {
				callback(null, done);
			}
		});
	}
}

export interface Options {
	asStream?: boolean;
}
