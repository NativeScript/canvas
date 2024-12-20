import { native_, mapState_ } from './Constants';

export class GPUBuffer {
	[native_];
	[mapState_]: 'unmapped' | 'mapped' | 'pending' = 'unmapped';

	get label() {
		return this[native_]?.label ?? '';
	}

	get size() {
		return this[native_]?.size ?? 0;
	}

	get usage() {
		return this[native_]?.usage ?? 0;
	}

	get native() {
		return this[native_];
	}

	destroy() {
		this[native_]?.destroy?.();
	}

	get mapState() {
		return this[mapState_];
	}

	getMappedRange(offset?: number, size?: number): ArrayBuffer {
		let rangeSize;
		if (size === undefined) {
			rangeSize = Math.max(0, this.size - (offset ?? 0));
		} else {
			rangeSize = size;
		}

		return this[native_].getMappedRange(offset, rangeSize);
	}

	mapAsync(mode: number, offset?: number, size?: number): Promise<void> {
		const previousSrc = this[mapState_];
		this[mapState_] = 'pending';
		return this[native_]
			.mapAsync(mode, offset, size)
			.then((value) => {
				this[mapState_] = 'mapped';
				return value;
			})
			.catch((e) => {
				this[mapState_] = previousSrc;
				throw e;
			});
	}

	unmap() {
		this[native_].unmap();
		this[mapState_] = 'unmapped';
	}

	static fromNative(buffer, mappedAtCreation: boolean = false) {
		if (buffer) {
			const ret = new GPUBuffer();
			ret[native_] = buffer;
			if (mappedAtCreation) {
				ret[mapState_] = 'mapped';
			}
			return ret;
		}
		return null;
	}
}
