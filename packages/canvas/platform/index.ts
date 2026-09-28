/**
 * Per-platform helpers for converting platform objects (images, fonts, views) into what the
 * native module takes. Android and iOS still convert inline (their bitmaps go to Kotlin/Swift
 * helpers); Node-API hosts implement these in index.<platform>.ts.
 */

/** A canvas ImageAsset holding a platform image (an ImageSource, a native bitmap), or null. */
export function imageAssetFor(image: any): any {
	return null;
}

/** Loads a platform image into `asset` (an ImageAsset's native object); false if unsupported. */
export function loadNativeImage(asset: any, image: any): boolean {
	return false;
}

/** Calls `registered(path, family)` for every font file the font manager finishes loading. */
export function onFontLoaded(registered: (path: string, family: string) => void): void {}

/** A native container view for the canvas DOM (`Dom`), or null for the default one. */
export function createContainerView(): any {
	return null;
}

/** Adds `child` (a native view) to a `createContainerView` container. */
export function addNativeChild(container: any, child: any): boolean {
	return false;
}
