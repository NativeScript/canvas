import { native_ } from './Constants';

export class GPUBindGroupLayout {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(group) {
		if (group) {
			const ret = new GPUBindGroupLayout();
			ret[native_] = group;
			return ret;
		}
		return null;
	}
}
