import { getFileAccess } from '@nativescript/core';

declare const Windows: any, NSWinRT: any;

/** The bytes of an ArrayBuffer or a view, as an ArrayBuffer (no copy when it is a whole buffer). */
function toArrayBuffer(bytes: any): ArrayBuffer | null {
	if (bytes instanceof ArrayBuffer) {
		return bytes;
	}
	if (ArrayBuffer.isView(bytes)) {
		if (bytes.byteOffset === 0 && bytes.byteLength === bytes.buffer.byteLength) {
			return bytes.buffer as ArrayBuffer;
		}
		return bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
	}
	return null;
}

export class FileManager {
	/** `readFile` is always the native module's (off the JS thread, straight into an ArrayBuffer). */
	static supportFastRead = true;

	public static writeFile(bytes: any, path: string, callback: (...args) => void) {
		const buffer = toArrayBuffer(bytes);
		if (!buffer) {
			callback(new Error('writeFile expects an ArrayBuffer or an ArrayBufferView'), null);
			return;
		}
		getFileAccess()
			.writeBufferAsync(path, buffer)
			.then(
				() => callback(null, path),
				(error) => callback(error instanceof Error ? error : new Error(String(error)), null),
			);
	}

	public static readFile(path: string, options: Options = { asStream: false }, callback: (...args) => void) {
		// Read off the JS thread by the native module, straight into an ArrayBuffer.
		global.CanvasModule.readFile(path, function (error, result: { buffer: ArrayBuffer; mime?: string; extension?: string }) {
			if (error) {
				callback(new Error(error), null);
			} else {
				callback(null, result);
			}
		});
	}

	public static deleteFile(path: string, options: Options = { asStream: false }, callback: (...args) => void) {
		// Through WinRT promises: the operation completes (or rejects for a missing file) later.
		NSWinRT.toPromise(Windows.Storage.StorageFile.GetFileFromPathAsync(path))
			.then((file) => NSWinRT.toPromise(file.DeleteAsync()))
			.then(
				() => callback(null, true),
				(error) => callback(error instanceof Error ? error : new Error(String(error?.message ?? error)), null),
			);
	}
}

export interface Options {
	asStream?: boolean;
}
