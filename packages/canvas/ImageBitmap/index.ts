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
			opts.setResizeHeight(options.resizeHeight);
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

		return opts;
	}
	return options;
}

function invalidSource(source: any) {
	const error: any = new TypeError(`Failed to execute 'createImageBitmap' : The provided value is not of type '(HTMLImageElement or SVGImageElement or HTMLVideoElement or HTMLCanvasElement or ImageBitmap or OffscreenCanvas or VideoFrame or Blob or ImageData)'.`);
	error.__source = source;
	return error;
}

/** `undefined` for an unrecognised source, so the caller rejects instead of hanging. */
function resolveSource(source: any): any {
	if (source instanceof Canvas) {
		return (source as any).native;
	}
	if (source instanceof ImageBitmap) {
		return source.native;
	}
	if (source instanceof ImageAsset) {
		return source.native;
	}
	if (source instanceof ImageData) {
		return (source as any).native;
	}
	if (typeof Blob !== 'undefined' && source instanceof Blob) {
		const bytes = (Blob as any).InternalAccessor.getBuffer(source) as Uint8Array;
		if (ArrayBuffer.isView(bytes)) {
			// A Blob's buffer is often a slice of a larger store; keep the window.
			return new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength);
		}
		return bytes;
	}
	if (source && typeof source === 'object' && typeof source.tagName === 'string') {
		if (source.tagName === 'IMG' || source.tagName === 'IMAGE') {
			return source._asset?.native;
		}
		if (source.tagName === 'CANVAS' && source._canvas instanceof Canvas) {
			return source._canvas.native;
		}
		return undefined;
	}
	if (source instanceof ArrayBuffer) {
		// Normalise to a view: the native side reads byteOffset/byteLength.
		return new Uint8Array(source);
	}
	if (ArrayBuffer.isView(source)) {
		return source;
	}
	if (source instanceof ImageSource) {
		if (__ANDROID__) {
			return source.android; // todo
		}
		if (__APPLE__) {
			return source.ios; // todo
		}
	}
	return undefined;
}

export class ImageBitmap {
	static {
		Helpers.initialize();
	}

	_native;

	/** The spec's [[Detached]] slot: a detached bitmap reports 0x0. */
	private _detached = false;

	get native() {
		return this._native;
	}

	private constructor(bitmap: any) {
		this._native = bitmap;
	}

	get width(): number {
		return this._detached ? 0 : this.native.width;
	}

	get height(): number {
		return this._detached ? 0 : this.native.height;
	}

	get __detached(): boolean {
		return this._detached;
	}

	/** Detach without freeing, for a bitmaprenderer transfer still using the pixels. */
	__detach() {
		this._detached = true;
	}

	close() {
		if (this._detached) {
			return;
		}
		this._detached = true;
		this.native.close();
	}

	static fromNative(value) {
		if (value) {
			return new ImageBitmap(value);
		}
		return null;
	}

	private static _create(source: any, rect: [number, number, number, number] | null, options: any) {
		return new Promise<ImageBitmap>((resolve, reject) => {
			if (source === null || source === undefined) {
				reject(invalidSource(source));
				return;
			}

			if (source instanceof ImageBitmap && source.__detached) {
				const error: any = new Error(`Failed to execute 'createImageBitmap' : The image source is detached.`);
				error.name = 'InvalidStateError';
				reject(error);
				return;
			}

			const realSource = resolveSource(source);

			if (realSource === undefined || realSource === null) {
				reject(invalidSource(source));
				return;
			}

			if (__ANDROID__ && ArrayBuffer.isView(realSource)) {
				// Encoded bytes decode on the Java thread pool.
				const asset = new global.CanvasModule.ImageAsset();
				const ptr = long(asset.__getRef());
				const cb = new org.nativescript.canvas.NSCImageBitmap.Callback({
					onComplete(done) {
						if (done) {
							const value = global.CanvasModule.ImageBitmap.fromAsset(asset);
							const bitmap = ImageBitmap.fromNative(value);
							if (bitmap) {
								resolve(bitmap);
								return;
							}
						}
						reject(new Error('Failed to create ImageBitmap'));
					},
				});
				if (rect) {
					org.nativescript.canvas.NSCImageBitmap.createFromRectOptions(ptr, realSource as never, rect[0], rect[1], rect[2], rect[3], parseOptions(options ?? {}), cb);
				} else if (options) {
					org.nativescript.canvas.NSCImageBitmap.createFromOptions(ptr, realSource as never, parseOptions(options), cb);
				} else {
					org.nativescript.canvas.NSCImageBitmap.createFrom(ptr, realSource as never, cb);
				}
				return;
			}

			const done = (error, value) => {
				const bitmap = value ? ImageBitmap.fromNative(value) : null;
				if (bitmap) {
					resolve(bitmap);
				} else {
					reject(new Error(error ?? 'Failed to create ImageBitmap'));
				}
			};

			if (rect) {
				global.CanvasModule.createImageBitmap(realSource, rect[0], rect[1], rect[2], rect[3], options, done);
			} else {
				global.CanvasModule.createImageBitmap(realSource, options, done);
			}
		});
	}

	static createFrom(source: any, options?: any) {
		return ImageBitmap._create(source, null, options);
	}

	static createFromRect(source: any, sx: number, sy: number, sWidth: number, sHeight: number, options?: any) {
		return ImageBitmap._create(source, [sx, sy, sWidth, sHeight], options);
	}
}
