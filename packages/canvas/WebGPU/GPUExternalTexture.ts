import { native_ } from './Constants';

export class GPUExternalTexture {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(texture) {
		if (texture) {
			const ret = new GPUExternalTexture();
			ret[native_] = texture;
			return ret;
		}
		return null;
	}
}
