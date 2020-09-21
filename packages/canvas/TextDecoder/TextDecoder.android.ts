import {TextDecoderBase} from './common';

declare var com;

export class TextDecoder extends TextDecoderBase {
    constructor(encoding: string = 'utf-8') {
        super(new com.github.triniwiz.canvas.TextDecoder(encoding));
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
            buffer instanceof Float32Array
        ) {
            if (buffer instanceof Uint8Array || buffer instanceof Int8Array) {
                // const native = Array.create('byte', buffer.byteLength);
                // const length = buffer.byteLength;
                //   for (let i = 0; i < length; i++) {
                //     native[i] = buffer[i];
                //   }
                const nativeBuffer = java.nio.ByteBuffer.wrap(Array.from(buffer));
                return this.native.decode(nativeBuffer.array());
            } else if (buffer instanceof Uint16Array || buffer instanceof Int16Array) {
                // const native = Array.create('byte', buffer.byteLength);
                // const length = buffer.byteLength;
                //   for (let i = 0; i < length; i++) {
                //     native[i] = buffer[i];
                //   }

                const nativeBuffer = java.nio.ShortBuffer.wrap(Array.from(buffer));
                return this.native.decode(nativeBuffer.array());
            } else if (buffer instanceof Int32Array) {
                // const native = Array.create('byte', buffer.byteLength);
                // const length = buffer.byteLength;
                //   for (let i = 0; i < length; i++) {
                //     native[i] = buffer[i];
                //   }
                const nativeBuffer = java.nio.IntBuffer.wrap(Array.from(buffer));
                return this.native.decode(nativeBuffer.array());
            }

            const array = new Uint8Array(buffer);
            // const native = Array.create('byte', array.byteLength);
            // const length = array.byteLength;
            // for (let i = 0; i < length; i++) {
            //   native[i] = array[i];
            // }
            const nativeBuffer = java.nio.ByteBuffer.wrap(Array.from(array));
            return this.native.decode(nativeBuffer.array());
        } else {
            return '';
        }
    }
}
