import { native_ } from './Constants';

export class GPUQuerySet {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	destroy() {
		this[native_]?.destroy?.();
	}

	static fromNative(query) {
		if (query) {
			const ret = new GPUQuerySet();
			ret[native_] = query;
			return ret;
		}
		return null;
	}
}
