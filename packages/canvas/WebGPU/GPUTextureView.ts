import { native_ } from './Constants';

export class GPUTextureView {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(view) {
		if (view) {
			const ret = new GPUTextureView();
			ret[native_] = view;
			return ret;
		}
		return null;
	}
}
