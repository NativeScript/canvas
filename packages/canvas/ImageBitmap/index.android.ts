import { ImageAsset } from '../ImageAsset';
import { ImageData } from '../Canvas2D';
import { ImageBitmapBase } from './common';
import { Canvas } from '../Canvas';
import { ImageSource } from '@nativescript/core';

import { Helpers } from '../helpers';

let ctor;

export class ImageBitmap extends ImageBitmapBase {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.createImageBitmap;
	}

	private constructor(bitmap: any) {
		super(bitmap);
	}

	_methodCache = new Map();

	_getMethod(name: string) {
		const cached = this._methodCache.get(name);
		if (cached === undefined) {
			const ret = this.native[name];
			this._methodCache.set(name, ret);
			return ret;
		}

		return cached;
	}

	get width(): number {
		return this.native.width;
	}
	get height(): number {
		return this.native.height;
	}

	close() {
		const close = this._getMethod('close');
		close();
	}

	static fromNative(value) {
		if (value) {
			return new ImageBitmap(value);
		}
		return null;
	}

	static createFrom(source: any, options: any) {
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
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
				realSource = source._asset.native;
			} else if (source instanceof ArrayBuffer) {
				realSource = source;
			} else if (source instanceof ImageSource) {
				realSource = source.android; // todo
			}

			ctor(realSource, options, (error, value) => {
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
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
				realSource = source._asset.native;
			} else if (source instanceof ArrayBuffer) {
				realSource = source;
			} else if (source instanceof ImageSource) {
				realSource = source.android; // todo
			}

			ctor(realSource, sx, sy, sWidth, sHeight, options, (error, value) => {
				if (value) {
					resolve(ImageBitmap.fromNative(value));
				} else {
					reject(new Error(error));
				}
			});
		});
	}
}
