import { native_ } from './Constants';

export class GPUShaderModule {
	label = '';
	[native_];

	static fromNative(module) {
		if (module) {
			const ret = new GPUShaderModule();
			ret[native_] = module;
			return ret;
		}

		return null;
	}
}
