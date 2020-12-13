import { TextDecoderBase } from './common';

declare var com;

export class TextDecoder extends TextDecoderBase {

	constructor(encoding: string = 'utf-8') {
		super(new com.github.triniwiz.canvas.TNSTextDecoder(encoding));
	}

	get encoding(): string {
		return this.native.getEncoding();
	}

	decode(buffer?: ArrayBuffer | ArrayBufferView, options?: any): string {
		if (
			buffer instanceof ArrayBuffer ||
			buffer instanceof Uint8Array ||
			buffer instanceof Int8Array ||
			buffer instanceof Uint16Array ||
			buffer instanceof Int16Array ||
			buffer instanceof Uint32Array ||
			buffer instanceof Int32Array ||
			buffer instanceof Float32Array || buffer instanceof Uint8ClampedArray
		) {
			if (buffer instanceof Uint8Array || buffer instanceof Int8Array || buffer instanceof Uint8ClampedArray) {
				const nativeBuffer = java.nio.ByteBuffer.wrap(Array.from(buffer));
				return this.native.decode(nativeBuffer.array());
			} else if (buffer instanceof Uint16Array || buffer instanceof Int16Array) {
				const nativeBuffer = java.nio.ShortBuffer.wrap(Array.from(buffer));
				return this.native.decode(nativeBuffer.array());
			} else if (buffer instanceof Int32Array || buffer instanceof Uint32Array) {
				const nativeBuffer = java.nio.IntBuffer.wrap(Array.from(buffer));
				return this.native.decode(nativeBuffer.array());
			} else if (buffer instanceof Float32Array) {
				const nativeBuffer = java.nio.FloatBuffer.wrap(Array.from(buffer));
				return this.native.decode(nativeBuffer.array());
			} else if (buffer instanceof Float64Array) {
				const nativeBuffer = java.nio.DoubleBuffer.wrap(Array.from(buffer));
				return this.native.decode(nativeBuffer.array());
			}

			const nativeBuffer = java.nio.ByteBuffer.wrap(Array.from(buffer as any));
			return this.native.decode(nativeBuffer.array());
		} else {
			return '';
		}
	}
}
