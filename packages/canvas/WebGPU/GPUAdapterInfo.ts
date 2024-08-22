import { native_ } from './Constants';

export class GPUAdapterInfo {
	[native_];

	get architecture() {
		return this[native_].architecture;
	}

	get description() {
		return this[native_].description;
	}

	get device() {
		return this[native_].device;
	}

	get vendor() {
		return this[native_].vendor;
	}

	static fromNative(info) {
		if (info) {
			const ret = new GPUAdapterInfo();
			ret[native_] = info;
			return ret;
		}
		return null;
	}
}
