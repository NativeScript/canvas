/**
 * Windows: images from @nativescript/core are XAML BitmapImages, which expose no pixels. An
 * ImageSource keeps the encoded bytes it was made from (`_rawBytes`, or `windows` itself for
 * fromDataSync); one loaded by URI (fromFileSync) is read back from its file. Either way the
 * canvas decodes it itself, once per source.
 */
import { FontFaceSet } from '@nativescript/font-manager';
import { ImageAsset } from '../ImageAsset';

declare const Windows: any, Microsoft: any;

const assets = new WeakMap<object, ImageAsset>();

function encodedBytes(image: any): Uint8Array | null {
	if (image instanceof Uint8Array) {
		return image;
	}
	const bytes = image?._rawBytes ?? image?.windows;
	return bytes instanceof Uint8Array ? bytes : null;
}

/** The file behind a BitmapImage's UriSource (ms-appx:/// is the app package, file:/// a path). */
function uriPath(image: any): string | null {
	const bitmap = image?.windows ?? image;
	return fileFromUri(bitmap?.UriSource?.AbsoluteUri);
}

function fileFromUri(uri: string | undefined): string | null {
	if (!uri) {
		return null;
	}
	if (uri.startsWith('ms-appx:///')) {
		const root = Windows.ApplicationModel.Package.Current.InstalledLocation.Path;
		return `${root}\\${decodeURIComponent(uri.substring('ms-appx:///'.length)).replace(/\//g, '\\')}`;
	}
	if (uri.startsWith('file:///')) {
		return decodeURIComponent(uri.substring('file:///'.length)).replace(/\//g, '\\');
	}
	return null;
}

export function loadNativeImage(asset: any, image: any): boolean {
	const bytes = encodedBytes(image);
	if (bytes) {
		return !!asset.fromEncodedBytesSync(bytes);
	}
	const path = uriPath(image);
	return path ? !!asset.fromFileSync(path) : false;
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

/** Font files: font-manager reports a loaded face's XAML font URI (`<file uri>#<family>`). */
export function onFontLoaded(registered: (path: string, family: string) => void): void {
	FontFaceSet.instance.on('loadingdone', (args: any) => {
		for (const face of args?.fontfaces ?? []) {
			const uri: string | undefined = face?.fontUri;
			const path = fileFromUri(uri?.split('#')[0]) ?? (uri && !uri.includes('://') ? uri.split('#')[0] : null);
			if (path) {
				registered(path, face.family);
			}
		}
	});
}

export function createContainerView(): any {
	return new Microsoft.UI.Xaml.Controls.Grid();
}

export function addNativeChild(container: any, child: any): boolean {
	if (!container?.Children || !child) {
		return false;
	}
	container.Children.Append(child);
	return true;
}
