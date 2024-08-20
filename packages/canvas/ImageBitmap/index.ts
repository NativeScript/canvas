import { ImageAsset } from '../ImageAsset';
import { ImageData } from '../Canvas2D';
import { Canvas } from '../Canvas';
import { ImageSource } from '@nativescript/core';

import { Helpers } from '../helpers';

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
			let isBuffer = false;

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
				isBuffer = true;
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
