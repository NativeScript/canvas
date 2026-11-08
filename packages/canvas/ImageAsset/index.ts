import { File, knownFolders, path as filePath, Utils, Observable, EventData, ImageSource } from '@nativescript/core';
import { Helpers } from '../helpers';
import { NAPI_HOST } from '../platform';
import { loadNativeImage, nativeImageOf, nativeImageOfAsync } from '../platform/index';

let ctor;

export interface SvgLoadOptions {
	/** CSS pixels; one alone keeps the aspect ratio. Defaults to the SVG's size. */
	width?: number;
	height?: number;
	/** Pixels per CSS pixel, default 1. */
	scale?: number;
	/** Animation time in seconds, default 0. */
	time?: number;
}

/** Registered by `@nativescript/canvas-svg` on import; canvas cannot import it. */
export interface SvgImageProvider {
	/** The Svg view behind `value`, else null. */
	resolve(value: unknown): object | null;
	/** In CSS pixels. */
	naturalSize(svg: object): { width: number; height: number };
	/** The current frame into `asset`; a no-op if unchanged. */
	rasterize(svg: object, scale: number, asset: ImageAsset): boolean;
	loadSync(source: string | object, asset: ImageAsset, options?: SvgLoadOptions): boolean;
	load(source: string | object, asset: ImageAsset, options?: SvgLoadOptions): Promise<boolean>;
}

const SVG_PROVIDER_KEY = '__canvasSvgImageProvider';

function svgProvider(): SvgImageProvider | undefined {
	return global[SVG_PROVIDER_KEY];
}

function requireSvgProvider(): SvgImageProvider {
	const provider = svgProvider();
	if (!provider) {
		throw new Error('Loading an SVG needs @nativescript/canvas-svg: install it and import it once before use.');
	}
	return provider;
}

/** Null for non-SVG sources, or when canvas-svg is absent. */
export function resolveSvgSource(value: unknown): object | null {
	if (!value || typeof value !== 'object') {
		return null;
	}
	return svgProvider()?.resolve(value) ?? null;
}

export function getSvgNaturalSize(svg: object): { width: number; height: number } | null {
	return svgProvider()?.naturalSize(svg) ?? null;
}

// Several sizes per view, for a view drawn at two sizes a frame.
const SVG_SIZES_PER_VIEW = 4;
const svgAssets = new WeakMap<object, Map<number, ImageAsset>>();

/** A view's current frame at `scale` pixels per CSS pixel. */
export function svgToImageAsset(svg: object, scale = 1): ImageAsset | null {
	const provider = svgProvider();
	if (!provider) {
		return null;
	}
	let sizes = svgAssets.get(svg);
	if (!sizes) {
		sizes = new Map();
		svgAssets.set(svg, sizes);
	}
	const key = Math.max(1, Math.round(scale * 100));
	let asset = sizes.get(key);
	if (asset) {
		// Re-inserted as newest; the first entry is evicted.
		sizes.delete(key);
	} else {
		asset = new ImageAsset();
		if (sizes.size >= SVG_SIZES_PER_VIEW) {
			sizes.delete(sizes.keys().next().value);
		}
	}
	sizes.set(key, asset);
	return provider.rasterize(svg, key / 100, asset) ? asset : null;
}

/** SVG sources become an ImageAsset at natural size; others pass through. */
export function fromSvgSource<T>(value: T): T | ImageAsset {
	const svg = resolveSvgSource(value);
	return svg ? (svgToImageAsset(svg) ?? value) : value;
}

// Assets whose pixels native code is reading off the main thread.
const converting = new Set<ImageAsset>();

let nextAndroidLoad = 0;
const androidLoads = new Map<number, (result: any) => void>();

function androidCallback<T>(type: 'Callback' | 'BitmapCallback', complete: (result: T) => void) {
	const id = nextAndroidLoad++;
	androidLoads.set(id, complete);
	return new (<any>org).nativescript.canvas.NSCImageAsset[type]({
		onComplete(result: T) {
			const done = androidLoads.get(id);
			androidLoads.delete(id);
			done?.(result);
		},
	});
}

function androidLoadCallback(complete: (success: boolean) => void) {
	return androidCallback('Callback', complete);
}

export class ImageAsset extends Observable {
	static {
		Helpers.initialize();
	}

	_native;
	_android;
	get native() {
		return this._native;
	}
	constructor(native?) {
		super();
		this._native = native || new global.CanvasModule.ImageAsset();
		if (__ANDROID__) {
			const ref = long(this.native.__getRef());
			// Kotlin must not release it when finalized.
			this._android = new (<any>org).nativescript.canvas.NSCImageAsset(ref, false);
		}
	}

	get width() {
		return this.native.width;
	}

	get height() {
		return this.native.height;
	}

	get error(): string {
		return this.native.error;
	}

	fromUrlSync(url: string): boolean {
		return this.native.fromUrlSync(url);
	}

	/** A `Bitmap` (Android), `UIImage` (iOS), `WriteableBitmap` (Windows) or `GdkTexture` (Linux); null without pixels. */
	toNativeImageSync(): any {
		if (NAPI_HOST) {
			return nativeImageOf(this.native);
		}
		if (__ANDROID__) {
			return (<any>org).nativescript.canvas.NSCImageAsset.toBitmap(this._android.getAsset());
		}
		if (__APPLE__) {
			return NSCImageAsset.toUIImage(NSString.stringWithString(this.native.__getRef()).longLongValue);
		}
		return null;
	}

	toNativeImage(): Promise<any> {
		if (NAPI_HOST) {
			return nativeImageOfAsync(this.native);
		}
		if (__ANDROID__) {
			converting.add(this);
			return new Promise((resolve) => {
				(<any>org).nativescript.canvas.NSCImageAsset.toBitmapAsync(
					this._android.getAsset(),
					androidCallback('BitmapCallback', (bitmap) => {
						converting.delete(this);
						resolve(bitmap);
					}),
				);
			});
		}
		if (__APPLE__) {
			converting.add(this);
			return new Promise((resolve) => {
				NSCImageAsset.toUIImageAsync(NSString.stringWithString(this.native.__getRef()).longLongValue, (image) => {
					converting.delete(this);
					resolve(image);
				});
			});
		}
		return Promise.resolve(this.toNativeImageSync());
	}

	toImageSourceSync(): ImageSource | null {
		const image = this.toNativeImageSync();
		return image ? new ImageSource(image) : null;
	}

	toImageSource(): Promise<ImageSource | null> {
		return this.toNativeImage().then((image) => (image ? new ImageSource(image) : null));
	}

	private _androidCallback(asset: number, resolve: (value: boolean) => void, reject: (reason?: any) => void, fallbackError?: string) {
		return androidLoadCallback((success) => {
			if (success) {
				this.emitComplete(success, undefined);
				resolve(success);
			} else {
				const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset) || fallbackError;
				this.emitComplete(success, error);
				reject(error);
			}
		});
	}

	private emitComplete(success: boolean, error) {
		// Not on the Node-API host.
		this.native.__syncExternalMemory?.();
		this.notify({ eventName: 'complete', object: this, complete: success, error });
	}

	fromUrl(url: string) {
		return new Promise((resolve, reject) => {
			if (__ANDROID__) {
				const asset = this._android.getAsset();
				(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromUrlAsync(asset, url, this._androidCallback(asset, resolve, reject));
				return;
			}

			this.native.fromUrlCb(url, (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	fromFileSync(path: string): boolean {
		let realPath = path;
		if (typeof realPath === 'string') {
			if (realPath.startsWith('~/')) {
				realPath = filePath.join(knownFolders.currentApp().path, realPath.replace('~/', ''));
			}
		}

		const ret = this.native.fromFileSync(realPath);

		return ret;
	}

	fromFile(path: string) {
		return new Promise((resolve, reject) => {
			if (typeof path === 'string') {
				if (path.startsWith('~/')) {
					path = filePath.join(knownFolders.currentApp().path, path.replace('~/', ''));
				}
			}

			if (__ANDROID__) {
				const asset = this._android.getAsset();
				if (typeof path === 'string' && path.indexOf('.webp') > -1) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadWebPAsync(asset, path, this._androidCallback(asset, resolve, reject));
				} else {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromPathAsync(asset, path, this._androidCallback(asset, resolve, reject));
				}
				return;
			}

			this.native.fromFileCb(path.toString(), (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	loadFromNativeSync(image: any): boolean {
		try {
			if (NAPI_HOST) {
				return loadNativeImage(this.native, image);
			}
			if (__ANDROID__) {
				const asset = long(this.native.__getRef());
				return (<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBitmap(asset, image);
			}

			if (__IOS__) {
				const asset = NSString.stringWithString(this.native.__getRef());
				return NSCImageAsset.loadImageFromImageSync(asset.longLongValue, image);
			}

			return false;
		} catch (e) {
			return false;
		}
	}

	loadFromNative(image: any): Promise<boolean> {
		return new Promise((resolve, reject) => {
			if (NAPI_HOST) {
				const loaded = loadNativeImage(this.native, image);
				this.emitComplete(loaded, loaded ? undefined : 'Failed to load image from native source');
				if (loaded) {
					resolve(true);
				} else {
					reject('Failed to load image from native source');
				}
				return;
			}
			if (__ANDROID__) {
				const asset = long(this.native.__getRef());
				try {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBitmapAsync(asset, image, this._androidCallback(asset, resolve, reject, 'Failed to load image from native source'));
				} catch (e) {
					reject(e);
				}
				return;
			}

			if (__IOS__) {
				const asset = NSString.stringWithString(this.native.__getRef());
				try {
					NSCImageAsset.loadImageFromImage(asset.longLongValue, image, (done) => {
						if (!done) {
							const error = this.error || 'Failed to load image from native source';
							this.emitComplete(done, error);
							reject(error);
						} else {
							this.emitComplete(done, undefined);
							resolve(done);
						}
					});
				} catch (e) {
					reject(e);
				}
				return;
			}

			reject(new Error('Unsupported platform'));
		});
	}

	loadFromEncodedBytesSync(bytes: Uint8Array | Uint8ClampedArray) {
		return this.native.fromEncodedBytesSync(bytes);
	}

	loadFromEncodedBytes(bytes: Uint8Array | Uint8ClampedArray) {
		return new Promise<boolean>((resolve, reject) => {
			if (__ANDROID__) {
				const asset = this._android.getAsset();

				if (!ArrayBuffer.isView(bytes)) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromEncodedBytesAsync(asset, bytes, this._androidCallback(asset, resolve, reject));
				} else {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromEncodedBufferAsync(asset, bytes, this._androidCallback(asset, resolve, reject));
				}
				return;
			}

			this.native.fromEncodedBytesCb(bytes, (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	/** Pass `premultiplied` for premultiplied bytes, or they are premultiplied twice. */
	loadFromBytesSync(width: number, height: number, bytes: Uint8Array | Uint8ClampedArray, premultiplied = false) {
		return this.native.fromBytesSync(width, height, bytes, premultiplied);
	}

	/** Markup, a `~/` or absolute path, or an Svg view. Needs `@nativescript/canvas-svg`. */
	loadSvgSync(source: string | object, options?: SvgLoadOptions): boolean {
		return requireSvgProvider().loadSync(source, this, options);
	}

	/** As `loadSvgSync`, and also takes a URL. */
	loadSvg(source: string | object, options?: SvgLoadOptions): Promise<boolean> {
		let provider: SvgImageProvider;
		try {
			provider = requireSvgProvider();
		} catch (error) {
			return Promise.reject(error);
		}
		return provider.load(source, this, options).then(
			(success) => {
				this.emitComplete(success, success ? undefined : this.error);
				return success;
			},
			(error) => {
				this.emitComplete(false, error);
				throw error;
			},
		);
	}

	loadFromBytes(width: number, height: number, bytes: Uint8Array | Uint8ClampedArray) {
		return new Promise((resolve, reject) => {
			if (__ANDROID__) {
				const asset = this._android.getAsset();

				if (Array.isArray(bytes)) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBytesAsync(asset, width, height, bytes, this._androidCallback(asset, resolve, reject));
				} else {
					let buffer: ArrayBuffer = bytes as never;
					if (ArrayBuffer.isView(bytes)) {
						buffer = bytes.buffer;
					} else if (buffer && 'nativeObject' in buffer) {
						buffer = buffer.nativeObject as never;
					}

					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBufferAsync(asset, width, height, buffer, this._androidCallback(asset, resolve, reject));
				}
				return;
			}

			this.native.fromBytesCb(width, height, bytes, (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	scale(x: number, y: number) {
		this.native.scale(x, y);
	}

	saveSync(path: string, format: ImageAssetSaveFormat): boolean {
		return this.native.saveSync(path, format);
	}

	save(path: string, format: ImageAssetSaveFormat): Promise<boolean> {
		return new Promise((resolve, reject) => {
			this.native.saveCb(path, format, (success, error) => {
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	flipX() {}

	flipY() {}
}

export enum ImageAssetSaveFormat {
	JPG,
	PNG,
	ICO,
	BMP,
	TIFF,
}
