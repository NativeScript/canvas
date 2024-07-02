import { native_ } from './Constants';

export class GPURenderPipeline {
	[native_];

	static fromNative(pipeline) {
		if (pipeline) {
			const ret = new GPURenderPipeline();
			ret[native_] = pipeline;
			return ret;
		}
		return null;
	}
}
