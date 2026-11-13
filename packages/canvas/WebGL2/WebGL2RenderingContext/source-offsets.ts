/**
 * WebGL2's `(srcData, srcOffset[, length])` overloads. The native calls take the whole array, so each is passed a
 * view of the requested range instead.
 */

type Viewable = ArrayBufferView & { subarray?(begin: number, end?: number): ArrayBufferView };

/** `data` from element `offset`, `length` elements long (0: to the end); a DataView counts bytes */
export function sourceView(data: Viewable, offset = 0, length = 0): ArrayBufferView {
	if (!offset && !length) return data;
	if (typeof data.subarray === 'function') return data.subarray(offset, length ? offset + length : undefined);

	return new Uint8Array(data.buffer, data.byteOffset + offset, length || data.byteLength - offset);
}

/** Method name, array argument index, offset argument index, whether a length follows */
type Overload = readonly [name: string, viewAt: number, offsetAt: number, hasLength: boolean];

const OVERLOADS: readonly Overload[] = [
	['texImage2D', 8, 9, false],
	['texSubImage2D', 8, 9, false],
	['texImage3D', 9, 10, false],
	['texSubImage3D', 10, 11, false],
	['compressedTexImage2D', 6, 7, true],
	['compressedTexSubImage2D', 7, 8, true],
	// bufferData(target, srcData, usage, srcOffset, length)
	['bufferData', 1, 3, true],
	['bufferSubData', 2, 3, true],
	['readPixels', 6, 7, false],
	...['1f', '2f', '3f', '4f', '1i', '2i', '3i', '4i', '1ui', '2ui', '3ui', '4ui'].map((kind) => [`uniform${kind}v`, 1, 2, true] as const),
	...['2', '3', '4', '2x3', '2x4', '3x2', '3x4', '4x2', '4x3'].map((shape) => [`uniformMatrix${shape}fv`, 2, 3, true] as const),
];

/** Wraps each overload on `proto` to pass a view of the requested range */
export function installSourceOffsets(proto: Record<string, any>): void {
	for (const [name, viewAt, offsetAt, hasLength] of OVERLOADS) {
		const original = proto[name];

		if (typeof original !== 'function') continue;
		proto[name] = function (this: unknown, ...args: any[]) {
			const data = args[viewAt];

			if (args.length > offsetAt && ArrayBuffer.isView(data)) {
				args[viewAt] = sourceView(data, args[offsetAt] ?? 0, hasLength ? (args[offsetAt + 1] ?? 0) : 0);
				args.length = offsetAt;
			}

			return original.apply(this, args);
		};
	}
}
