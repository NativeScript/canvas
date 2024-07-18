import { native_ } from './Constants';

export class GPUExternalTexture {
	[native_];

	static fromNative(texture) {
		if (texture) {
			const ret = new GPUExternalTexture();
			ret[native_] = texture;
			return ret;
		}
		return null;
	}
}
