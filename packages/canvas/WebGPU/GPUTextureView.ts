import { native_ } from './Constants';

export class GPUTextureView {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	// NOTE: deterministic release is not wired here. createView() runs for both
	// swapchain and persistent textures, and three caches views of persistent
	// render targets/depth textures across frames. Only swapchain-texture views
	// die at present, so that release is scoped in GPUTexture.createView() ->
	// GPUCanvasContext (see swapchainContext_).
	static fromNative(view) {
		if (view) {
			const ret = new GPUTextureView();
			ret[native_] = view;
			return ret;
		}
		return null;
	}
}
