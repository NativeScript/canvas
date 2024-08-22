import { native_ } from './Constants';

export class GPUSampler {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(sampler) {
		if (sampler) {
			const ret = new GPUSampler();
			ret[native_] = sampler;
			return ret;
		}
		return null;
	}
}
