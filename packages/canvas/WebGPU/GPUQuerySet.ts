import { native_ } from './Constants';

export class GPUQuerySet {
	[native_];

	static fromNative(query) {
		if (query) {
			const ret = new GPUQuerySet();
			ret[native_] = query;
			return ret;
		}
		return null;
	}
}
