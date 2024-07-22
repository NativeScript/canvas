import { native_ } from './Constants';

class GPUDeviceLostInfo {
	[native_];

	get message() {
		return this[native_].message;
	}

	get reason() {
		return this[native_].reason;
	}
}
