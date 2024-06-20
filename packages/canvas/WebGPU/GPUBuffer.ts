import { off } from "@nativescript/core";

const native_ = Symbol('[[native]]');
const mapState_ = Symbol('[[mapState]]');
export class GPUBuffer {
	[native_];
	[mapState_]: 'unmapped' | 'mapped' | 'pending' = 'unmapped';

	get label() {
		return this[native_].label;
	}
	get size() {
		return this[native_]?.size ?? 0;
	}

	get usage() {
		return this[native_]?.usage ?? 0;
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
          rangeSize = Math.max(0, this[this.size] - offset);
        } else {
          rangeSize = size;
        }
        const ab = new ArrayBuffer(rangeSize);
        return this[native_].getMappedRange(offset, size);
    }

	mapAsync(mode: number, offset?: number, size?: number): Promise<void> {
        return this[native_].mapAsync(mode, offset, size);
    }

	unmap() {
        this[native_].unmap();
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
