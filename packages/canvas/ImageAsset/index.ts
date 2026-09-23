import { File, knownFolders, path as filePath, Utils, Observable, EventData } from '@nativescript/core';
import { Helpers } from '../helpers';

let ctor;
// store ref if loading
const loaders = new Map<ImageAsset, number>();

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

	private _incrementStrongRef() {
		let count = 0;
		if (loaders.has(this)) {
			count = loaders.get(this);
		}
		count++;
		loaders.set(this, count);
	}

	private _decrementStrongRefAndRemove() {
		// Parenthesised: `??` binds looser than `-`, so the old form returned the
		// count undecremented and never released the strong ref.
		const count = (loaders.get(this) ?? 0) - 1;

		if (count <= 0) {
			loaders.delete(this);
		} else {
			loaders.set(this, count);
		}
	}

	private emitComplete(success: boolean, error) {
		this.notify({ eventName: 'complete', object: this, complete: success, error });
	}

	fromUrl(url: string) {
		return new Promise((resolve, reject) => {
			if (__ANDROID__) {
				const asset = this._android.getAsset();
				const ref = new WeakRef(this);
				(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromUrlAsync(
					asset,
					url,
					new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
						onComplete(success: boolean) {
							const owner = ref.get();
							if (!success) {
								const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
								if (owner) {
									owner.emitComplete(success, error);
								}
								reject(error);
							} else {
								if (owner) {
									owner.emitComplete(success, undefined);
								}
								resolve(success);
							}
						},
					}),
				);
				return;
			}

			this._incrementStrongRef();
			this.native.fromUrlCb(url, (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
				this._decrementStrongRefAndRemove();
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
				const ref = new WeakRef(this);
				if (typeof path === 'string' && path.indexOf('.webp') > -1) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadWebPAsync(
						asset,
						path,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						}),
					);
				} else {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromPathAsync(
						asset,
						path,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						}),
					);
				}
				return;
			}

			this._incrementStrongRef();

			this.native.fromFileCb(path.toString(), (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}

				this._decrementStrongRefAndRemove();
			});
		});
	}

	loadFromNativeSync(image: any): boolean {
		try {
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
			if (__ANDROID__) {
				const ref = new WeakRef(this);
				const asset = long(this.native.__getRef());
				try {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBitmapAsync(
						asset,
						image,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset) || 'Failed to load image from native source';
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						}),
					);
				} catch (e) {
					reject(e);
				}
				return;
			}

			if (__IOS__) {
				this._incrementStrongRef();
				const asset = NSString.stringWithString(this.native.__getRef());
				try {
					NSCImageAsset.loadImageFromImage(asset.longLongValue, image, (done) => {
						if (!done) {
							const error = this.error || 'Failed to load image from native source';
							this.emitComplete(done, error);
							this._decrementStrongRefAndRemove();
							reject(error);
						} else {
							this.emitComplete(done, undefined);
							this._decrementStrongRefAndRemove();
							resolve(done);
						}
					});
				} catch (e) {
					this._decrementStrongRefAndRemove();
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
				const ref = new WeakRef(this);
				const asset = this._android.getAsset();

				if (!ArrayBuffer.isView(bytes)) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromEncodedBytesAsync(
						asset,
						bytes,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						}),
					);
				} else {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromEncodedBufferAsync(
						asset,
						bytes,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						}),
					);
				}
				return;
			}

			this._incrementStrongRef();
			this.native.fromEncodedBytesCb(bytes, (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}

				this._decrementStrongRefAndRemove();
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
		this._incrementStrongRef();
		return provider
			.load(source, this, options)
			.then(
				(success) => {
					this.emitComplete(success, success ? undefined : this.error);
					return success;
				},
				(error) => {
					this.emitComplete(false, error);
					throw error;
				},
			)
			.finally(() => this._decrementStrongRefAndRemove());
	}

	loadFromBytes(width: number, height: number, bytes: Uint8Array | Uint8ClampedArray) {
		return new Promise((resolve, reject) => {
			if (__ANDROID__) {
				const ref = new WeakRef(this);
				const asset = this._android.getAsset();

				if (Array.isArray(bytes)) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBytesAsync(
						asset,
						width,
						height,
						bytes,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						}),
					);
				} else {
					let buffer: ArrayBuffer = bytes as never;
					if (ArrayBuffer.isView(bytes)) {
						buffer = bytes.buffer;
					} else if (buffer && 'nativeObject' in buffer) {
						buffer = buffer.nativeObject as never;
					}

					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBufferAsync(
						asset,
						width,
						height,
						buffer,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						}),
					);
				}
				return;
			}

			this._incrementStrongRef();
			this.native.fromBytesCb(width, height, bytes, (success, error) => {
				this.emitComplete(success, error);
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}

				this._decrementStrongRefAndRemove();
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
			this._incrementStrongRef();

			this.native.saveCb(path, format, (success, error) => {
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}

				this._decrementStrongRefAndRemove();
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
