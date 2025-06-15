import { native_ } from './Constants';
import { GPUCompilationInfo } from './GPUCompilationInfo';

export class GPUShaderModule {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	getCompilationInfo(): Promise<GPUCompilationInfo> {
		const ret = this[native_]?.getCompilationInfo();
		if (ret) {
			return ret.then((info) => {
				return GPUCompilationInfo.fromNative(info);
			});
		}
		return Promise.resolve(GPUCompilationInfo.fromNative(null));
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
