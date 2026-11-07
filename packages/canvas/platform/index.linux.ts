/**
 * Linux: images from @nativescript/core are GdkTextures (`ImageSource.linux`); the native module
 * downloads their pixels straight into a canvas ImageAsset, once per source.
 */
import { ImageAsset } from '../ImageAsset';

const assets = new WeakMap<object, ImageAsset>();

/** The address of a GdkTexture: an ImageSource's, or a texture pointer itself. */
function textureAddress(image: any): number {
	const texture = image?.linux ?? image;
	return typeof texture?.toNumber === 'function' ? texture.toNumber() : 0;
}

export function loadNativeImage(asset: any, image: any): boolean {
	if (image instanceof Uint8Array) {
		return !!asset.fromEncodedBytesSync(image);
	}
	const address = textureAddress(image);
	return address ? !!asset.fromGdkTextureSync(address) : false;
}

export function imageAssetFor(image: any): ImageAsset | null {
	if (!image || typeof image !== 'object') {
		return null;
	}
	const cached = assets.get(image);
	if (cached) {
		return cached;
	}
	const asset = new ImageAsset();
	if (!loadNativeImage(asset.native, image)) {
		return null;
	}
	assets.set(image, asset);
	return asset;
}

/** Fonts: font-manager adds them to fontconfig, which Skia's font manager reads. */
export function onFontLoaded(registered: (path: string, family: string) => void): void {}

export function createContainerView(): any {
	return null;
}

export function addNativeChild(container: any, child: any): boolean {
	return false;
}
