import { native_, swapchainContext_ } from './Constants';
import { GPUTextureView } from './GPUTextureView';
import type { GPUTextureViewDescriptor } from './Interfaces';

export class GPUTexture {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

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

	destroy() {
		this[native_].destroy();
	}

	static fromNative(texture) {
		if (texture) {
			const ret = new GPUTexture();
			ret[native_] = texture;
			return ret;
		}
		return null;
	}

	createView(desc?: GPUTextureViewDescriptor) {
		const view = this[native_].createView(desc);
		const ret = GPUTextureView.fromNative(view);
		// A view of the swapchain current-texture dies at presentSurface(). Register
		// it with the owning context so that context releases it deterministically.
		// Views of persistent textures (render targets, depth) carry no context ref,
		// are cached by three across frames, and must NOT be released here. See
		// GPUCanvasContext.getCurrentTexture / presentSurface.
		const ctx = (this as any)[swapchainContext_];
		if (ret && ctx) {
			ctx._registerSwapchainView(ret);
		}
		return ret;
	}
}
