import { native_ } from './Constants';

export class GPUBindGroup {
	[native_];

	static fromNative(group) {
		if (group) {
			const ret = new GPUBindGroup();
			ret[native_] = group;
			return ret;
		}
		return null;
	}
}
