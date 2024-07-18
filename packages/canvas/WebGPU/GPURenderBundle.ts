import { native_ } from './Constants';

export class GPURenderBundle {
	[native_];

	static fromNative(value) {
		if (value) {
			const ret = new GPURenderBundle();
			ret[native_] = value;
			return ret;
		}
		return null;
	}
}
