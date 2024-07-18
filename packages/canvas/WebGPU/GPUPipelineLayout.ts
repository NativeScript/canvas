import { native_ } from './Constants';

export class GPUPipelineLayout {
	[native_];

	static fromNative(layout) {
		if (layout) {
			const ret = new GPUPipelineLayout();
			ret[native_] = layout;
			return ret;
		}
		return null;
	}
}
