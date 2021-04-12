import { Canvas } from '../Canvas';
import { ImageBitmapBase } from './common';
import { ImageData } from '../Canvas2D';
import { ImageAsset } from '../ImageAsset';
export class ImageBitmap extends ImageBitmapBase {
	private constructor(bitmap: any) {
		super(bitmap);
	}
	#width = -1;
	#height = -1;
	get width(): number {
		if (this.#width === -1) {
			this.#width = this.native.width;
		}
		return this.#width;
	}
	get height(): number {
		if (this.#height === -1) {
			this.#height = this.native.height;
		}
		return this.#height;
	}

	close() {
		this.native.close();
	}

	static fromNative(value) {
		if (value instanceof TNSImageBitmap) {
			return new ImageBitmap(value);
		}
		return null;
	}

	private static handleOptions(options, nativeOptions: TNSImageBitmapOptions) {
		if (options && typeof options === 'object') {
			Object.keys(options).forEach((key) => {
				const value = options[key];
				switch (key) {
					case 'flipY':
						if (typeof value === 'boolean') {
							nativeOptions.flipY = value;
						}
						break;
					case 'premultiplyAlpha':
						switch (value) {
							case 'default':
								nativeOptions.premultiplyAlpha = TNSImageBitmapPremultiplyAlpha.Default;
								break;
							case 'Premultiply':
								nativeOptions.premultiplyAlpha = TNSImageBitmapPremultiplyAlpha.Premultiply;
							case 'none':
								nativeOptions.premultiplyAlpha = TNSImageBitmapPremultiplyAlpha.None;
								break;
						}
						break;
					case 'colorSpaceConversion':
						switch (value) {
							case 'default':
								nativeOptions.colorSpaceConversion = TNSImageBitmapColorSpaceConversion.Default;
								break;
							case 'none':
								nativeOptions.colorSpaceConversion = TNSImageBitmapColorSpaceConversion.None;
								break;
						}
						break;
					case 'resizeQuality':
						switch (value) {
							case 'low':
								nativeOptions.resizeQuality = TNSImageBitmapResizeQuality.Low;
								break;
							case 'medium':
								nativeOptions.resizeQuality = TNSImageBitmapResizeQuality.Medium;
								break;
							case 'high':
								nativeOptions.resizeQuality = TNSImageBitmapResizeQuality.High;
								break;
							case 'pixelated':
								nativeOptions.resizeQuality = TNSImageBitmapResizeQuality.Pixelated;
								break;
						}
						break;
					case 'resizeWidth':
						if (typeof value === 'number') {
							nativeOptions.resizeWidth = value;
						}
						break;
					case 'resizeHeight':
						if (typeof value === 'number') {
							nativeOptions.resizeHeight = value;
						}
						break;
				}
			});
		}
	}

	static createFrom(source: any, options: any) {
		return new Promise((resolve, reject) => {
			const opts = new TNSImageBitmapOptions();
			ImageBitmap.handleOptions(options, opts);
			if (source instanceof Canvas) {
				TNSImageBitmap.createFromCanvas(source.ios, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ImageBitmap) {
				TNSImageBitmap.createFromImageBitmap(source.native, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ImageAsset) {
				TNSImageBitmap.createFromImageAsset(source.native, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ImageData) {
				TNSImageBitmap.createFromImageAsset(source.native, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof Blob) {
				const bytes = (Blob as any).InternalAccessor.getBuffer(source);
				TNSImageBitmap.createFromBytesEncoded(Array.from(bytes), opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
				TNSImageBitmap.createFromImageAsset(source._asset.native, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ArrayBuffer) {
				TNSImageBitmap.createFromDataEncoded(NSData.dataWithData(source as any), opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			}
		});
	}

	static createFromRect(source: any, sx: number, sy: number, sWidth: number, sHeight: number, options: any) {
		return new Promise((resolve, reject) => {
			const opts = TNSImageBitmapOptions.alloc().init();
			ImageBitmap.handleOptions(options, opts);
			if (source instanceof Canvas) {
				(TNSImageBitmap as any).createFromCanvas(source.ios, sx, sy, sWidth, sHeight, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ImageBitmap) {
				(TNSImageBitmap as any).createFromImageBitmap(source.native, sx, sy, sWidth, sHeight, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ImageAsset) {
				(TNSImageBitmap as any).createFromImageAsset(source.native, sx, sy, sWidth, sHeight, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ImageData) {
				(TNSImageBitmap as any).createFromImageAsset(source.native, sx, sy, sWidth, sHeight, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof Blob) {
				const bytes = (Blob as any).InternalAccessor.getBuffer(source);
				(TNSImageBitmap as any).createFromBytesEncoded(Array.from(bytes), sx, sy, sWidth, sHeight, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
				(TNSImageBitmap as any).createFromImageAsset(source._asset.native, sx, sy, sWidth, sHeight, opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			} else if (source instanceof ArrayBuffer) {
				(TNSImageBitmap as any).createFromDataEncoded(NSData.dataWithData(source as any), sx, sy, sWidth, sHeight,  opts, (bitmap, error) => {
					if (bitmap) {
						resolve(ImageBitmap.fromNative(bitmap));
					} else {
						reject(error);
					}
				});
			}
		});
	}
}
