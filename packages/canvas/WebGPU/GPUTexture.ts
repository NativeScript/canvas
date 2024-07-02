import { native_ } from './Constants';
import { GPUTextureView } from './GPUTextureView';

export class GPUTexture {
	[native_];

	get depthOrArrayLayers() {
		return this[native_].depthOrArrayLayers;
	}
	get dimension() {
		return this[native_].dimension;
	}
	get format() {
		return this[native_].format;
	}
	get mipLevelCount() {
		return this[native_].mipLevelCount;
	}
	get sampleCount() {
		return this[native_].sampleCount;
	}
	get width() {
		return this[native_].width;
	}
	get height() {
		return this[native_].height;
	}
	get usage() {
		return this[native_].usage;
	}
	static fromNative(texture) {
		if (texture) {
			const ret = new GPUTexture();
			ret[native_] = texture;
			return ret;
		}
		return null;
	}

	createView(desc?) {
		const view = this[native_].createView(desc);
		console.log('createView', desc, view);
		return GPUTextureView.fromNative(view);
	}
}
