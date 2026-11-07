import { getFileAccess } from '@nativescript/core';

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
		let failure: Error | null = null;
		getFileAccess().deleteFile(path, (error) => (failure = error instanceof Error ? error : new Error(String(error))));
		// Async like the other platforms.
		setTimeout(() => (failure ? callback(failure, null) : callback(null, true)));
	}
}

export interface Options {
	asStream?: boolean;
}
