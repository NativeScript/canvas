import { native_ } from './Constants';

export class GPUExternalTexture {
	[native_];

	/** @internal Keeps the decoded frame alive while this can be sampled. */
	_frame: unknown;

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(texture, frame?: unknown) {
		if (texture) {
			const ret = new GPUExternalTexture();
			ret[native_] = texture;
			ret._frame = frame;
			return ret;
		}
		return null;
	}
}
