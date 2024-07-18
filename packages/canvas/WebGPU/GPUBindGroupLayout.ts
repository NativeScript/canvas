import { native_ } from './Constants';

export class GPUBindGroupLayout {
	[native_];

	static fromNative(group) {
		if (group) {
			const ret = new GPUBindGroupLayout();
			ret[native_] = group;
			return ret;
		}
		return null;
	}
}
