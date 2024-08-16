import { native_ } from './Constants';

export class GPUPipelineLayout {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(layout) {
		if (layout) {
			const ret = new GPUPipelineLayout();
			ret[native_] = layout;
			return ret;
		}
		return null;
	}
}
