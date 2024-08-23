import { ImageAsset } from '../ImageAsset';
import { ImageData } from '../Canvas2D';
import { Canvas } from '../Canvas';
import { ImageSource } from '@nativescript/core';

import { Helpers } from '../helpers';

function parseOptions(options) {
	if (__ANDROID__) {
		const opts = new org.nativescript.canvas.NSCImageBitmap.Options();
		if (options?.imageOrientation === 'flipY') {
			opts.setImageOrientation(org.nativescript.canvas.ImageBitmapImageOrientation.FlipY);
		}

		switch (options?.premultiplyAlpha) {
			case 'premultiply':
				opts.setPremultiplyAlpha(org.nativescript.canvas.ImageBitmapPremultiplyAlpha.PremultiplyAlpha);
				break;
			case 'none':
				opts.setPremultiplyAlpha(org.nativescript.canvas.ImageBitmapPremultiplyAlpha.None);
				break;
		}

		if (options?.colorSpaceConversion === 'none') {
			opts.setColorSpaceConversion(org.nativescript.canvas.ImageBitmapColorSpaceConversion.None);
		}

		if (typeof options?.resizeWidth === 'number') {
			opts.setResizeWidth(options.resizeWidth);
		}

		if (typeof options?.resizeHeight === 'number') {
			opts.setResizeHeight(options.resizeWidth);
		}

		switch (options?.resizeQuality) {
			case 'medium':
				opts.setResizeQuality(org.nativescript.canvas.ImageBitmapResizeQuality.Medium);
				break;
			case 'high':
				opts.setResizeQuality(org.nativescript.canvas.ImageBitmapResizeQuality.High);
				break;
			case 'pixelated':
				opts.setResizeQuality(org.nativescript.canvas.ImageBitmapResizeQuality.Pixelated);
				break;
		}

		if (options?.premultiplyAlpha === 'flipY') {
			opts.setImageOrientation(org.nativescript.canvas.ImageBitmapImageOrientation.FlipY);
		}

		return opts;
	}
	return options;
}

export class ImageBitmap {
	static {
		Helpers.initialize();
	}

	_native;
	get native() {
		return this._native;
	}

	private constructor(bitmap: any) {
		this._native = bitmap;
	}

	get width(): number {
		return this.native.width;
	}
	get height(): number {
		return this.native.height;
	}

	close() {
		this.native.close();
	}

	static fromNative(value) {
		if (value) {
			return new ImageBitmap(value);
		}
		return null;
	}

	static createFrom(source: any, options: any) {
		return new Promise(function (resolve, reject) {
			let realSource;

			if (source instanceof Canvas) {
				realSource = (source as any).native;
			} else if (source instanceof ImageBitmap) {
				realSource = source.native;
			} else if (source instanceof ImageAsset) {
				realSource = source.native;
			} else if (source instanceof ImageData) {
				realSource = source.native;
			} else if (source instanceof Blob) {
				const bytes = (Blob as any).InternalAccessor.getBuffer(source) as Uint8Array;
				realSource = bytes;
				if (ArrayBuffer.isView(bytes)) {
					realSource = new Uint8Array(bytes.buffer);
				}
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string') {
				if (source.tagName === 'IMG' || source.tagName === 'IMAGE') {
					realSource = source._asset.native;
				} else if (source.tagName === 'CANVAS' && source._canvas instanceof Canvas) {
					realSource = source._canvas.native;
				}
			} else if (source instanceof ArrayBuffer) {
				// wrapping to create a ref
				realSource = new Uint8Array(source);
			} else if (source instanceof ImageSource) {
				if (global.isAndroid) {
					realSource = source.android; // todo
				}
				if (global.isIOS) {
					realSource = source.ios; // todo
				}
			}

			if (__ANDROID__) {
				if (ArrayBuffer.isView(realSource)) {
					const asset = new global.CanvasModule.ImageAsset();
					const ptr = long(asset.__getRef());
					const cb = new org.nativescript.canvas.NSCImageBitmap.Callback({
						onComplete(done) {
							if (done) {
								const value = global.CanvasModule.ImageBitmap.fromAsset(asset);
								resolve(ImageBitmap.fromNative(value));
							} else {
								reject(new Error('Failed to create ImageBitmap'));
							}
						},
					});
					if (options) {
						const opts = parseOptions(options);
						org.nativescript.canvas.NSCImageBitmap.createFromOptions(ptr, realSource as never, opts, cb);
					} else {
						org.nativescript.canvas.NSCImageBitmap.createFrom(ptr, realSource as never, cb);
					}
				}
				return;
			}

			global.CanvasModule.createImageBitmap(realSource, options, (error, value) => {
				if (value) {
					resolve(ImageBitmap.fromNative(value));
				} else {
					reject(new Error(error));
				}
			});
		});
	}

	static createFromRect(source: any, sx: number, sy: number, sWidth: number, sHeight: number, options: any) {
		return new Promise((resolve, reject) => {
			let realSource;
			if (source instanceof Canvas) {
				realSource = (source as any).native;
			} else if (source instanceof ImageBitmap) {
				realSource = source.native;
			} else if (source instanceof ImageAsset) {
				realSource = source.native;
			} else if (source instanceof ImageData) {
				realSource = source.native;
			} else if (source instanceof Blob) {
				const bytes = (Blob as any).InternalAccessor.getBuffer(source);
				realSource = bytes;
				if (ArrayBuffer.isView(bytes)) {
					realSource = new Uint8Array(bytes.buffer);
				}
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string') {
				if (source.tagName === 'IMG' || source.tagName === 'IMAGE') {
					realSource = source._asset.native;
				} else if (source.tagName === 'CANVAS' && source._canvas instanceof Canvas) {
					realSource = source._canvas.native;
				}
			} else if (source instanceof ArrayBuffer) {
				// wrapping to create a ref
				realSource = new Uint8Array(source);
			} else if (source instanceof ImageSource) {
				if (global.isAndroid) {
					realSource = source.android; // todo
				}

				if (global.isIOS) {
					realSource = source.ios; // todo
				}
			}

			if (__ANDROID__) {
				if (ArrayBuffer.isView(realSource)) {
					const asset = new global.CanvasModule.ImageAsset();
					const ptr = long(asset.__getRef());
					const cb = new org.nativescript.canvas.NSCImageBitmap.Callback({
						onComplete(done) {
							if (done) {
								const value = global.CanvasModule.ImageBitmap.fromAsset(asset);
								resolve(ImageBitmap.fromNative(value));
							} else {
								reject(new Error('Failed to create ImageBitmap'));
							}
						},
					});
					const opts = parseOptions(options ?? {});
					org.nativescript.canvas.NSCImageBitmap.createFromRectOptions(ptr, realSource as never, sx, sy, sWidth, sHeight, opts, cb);
				}
				return;
			}

			global.CanvasModule.createImageBitmap(realSource, sx, sy, sWidth, sHeight, options, (error, value) => {
				if (value) {
					resolve(ImageBitmap.fromNative(value));
				} else {
					reject(new Error(error));
				}
			});
		});
	}
}
