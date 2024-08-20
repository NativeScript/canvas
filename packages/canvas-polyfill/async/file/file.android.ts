declare var com;
import { File } from '@nativescript/core';
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

	static supportFastRead;

	public static readFile(path: string, options: Options = { asStream: false }, callback: (...args) => void) {
		//const opts = new com.github.triniwiz.async.Async2.FileManager.Options();
		//opts.asStream = options.asStream;
		if (this.supportFastRead === undefined) {
			this.supportFastRead = typeof global?.CanvasModule?.readFile === 'function';
		}
		if (this.supportFastRead) {
			global?.CanvasModule?.readFile(path, (error, buffer: ArrayBuffer) => {
				if (error) {
					callback(new Error(error), null);
				} else {
					callback(null, buffer);
				}
			});
		} else {
			com.github.triniwiz.async.Async2.FileManager.readFileBuffer(
				path,
				null,
				new com.github.triniwiz.async.Async2.FileManager.Callback({
					onError(param0: string, param1: java.lang.Exception): void {
						callback(param0, null);
					},
					onComplete(param0: any): void {
						callback(null, (<any>ArrayBuffer).from(param0));
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
