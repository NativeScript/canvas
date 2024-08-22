import { native_ } from './Constants';

export class GPUBindGroup {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(group) {
		if (group) {
			const ret = new GPUBindGroup();
			ret[native_] = group;
			return ret;
		}
		return null;
	}
}
