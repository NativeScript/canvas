import { native_ } from './Constants';

export class GPUShaderModule {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(module) {
		if (module) {
			const ret = new GPUShaderModule();
			ret[native_] = module;
			return ret;
		}

		return null;
	}
}
