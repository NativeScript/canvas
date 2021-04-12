import { ImageAsset } from '../ImageAsset';
import { ImageData } from '../Canvas2D';
import { ImageBitmapBase } from './common';
import { Canvas } from '../Canvas';

export class ImageBitmap extends ImageBitmapBase {
	private constructor(bitmap: any) {
		super(bitmap);
	}
	#width = -1;
	#height = -1;
	get width(): number {
		if (this.#width === -1) {
			this.#width = this.native.getWidth();
		}
		return this.#width;
	}
	get height(): number {
		if (this.#height === -1) {
			this.#height = this.native.getHeight();
		}
		return this.#height;
	}

	close() {
		this.native.close();
	}

	static fromNative(value) {
		if (value instanceof com.github.triniwiz.canvas.TNSImageBitmap) {
			return new ImageBitmap(value);
		}
		return null;
	}

	private static handleOptions(options, nativeOptions) {
		if (options && typeof options === 'object') {
			Object.keys(options).forEach((key) => {
				const value = options[key];
				switch (key) {
					case 'flipY':
						if (typeof value === 'boolean') {
							nativeOptions.setFlipY(value);
						}
						break;
					case 'premultiplyAlpha':
						switch (value) {
							case 'default':
								nativeOptions.setPremultiplyAlpha(com.github.triniwiz.canvas.TNSImageBitmapPremultiplyAlpha.Default);
								break;
							case 'Premultiply':
								nativeOptions.setPremultiplyAlpha(com.github.triniwiz.canvas.TNSImageBitmapPremultiplyAlpha.Premultiply);
							case 'none':
								nativeOptions.setPremultiplyAlpha(com.github.triniwiz.canvas.TNSImageBitmapPremultiplyAlpha.None);
								break;
						}
						break;
					case 'colorSpaceConversion':
						switch (value) {
							case 'default':
								nativeOptions.setColorSpaceConversion(com.github.triniwiz.canvas.TNSImageBitmapColorSpaceConversion.Default);
								break;
							case 'none':
								nativeOptions.setColorSpaceConversion(com.github.triniwiz.canvas.TNSImageBitmapColorSpaceConversion.None);
								break;
						}
						break;
					case 'resizeQuality':
						switch (value) {
							case 'low':
								nativeOptions.setResizeQuality(com.github.triniwiz.canvas.TNSImageBitmapResizeQuality.Low);
								break;
							case 'medium':
								nativeOptions.setResizeQuality(com.github.triniwiz.canvas.TNSImageBitmapResizeQuality.Medium);
								break;
							case 'high':
								nativeOptions.setResizeQuality(com.github.triniwiz.canvas.TNSImageBitmapResizeQuality.High);
								break;
							case 'pixelated':
								nativeOptions.setResizeQuality(com.github.triniwiz.canvas.TNSImageBitmapResizeQuality.Pixelated);
								break;
						}
						break;
					case 'resizeWidth':
						if (typeof value === 'number') {
							nativeOptions.setResizeWidth(value);
						}
						break;
					case 'resizeHeight':
						if (typeof value === 'number') {
							nativeOptions.setResizeHeight(value);
						}
						break;
				}
			});
		}
	}

	static createFrom(source: any, options: any) {
		return new Promise((resolve, reject) => {
			const opts = new com.github.triniwiz.canvas.TNSImageBitmap.Options();
			ImageBitmap.handleOptions(options, opts);
			if (source instanceof Canvas) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromCanvas(
					source.android,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageBitmap) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageBitmap(
					source.native,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageAsset) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageData) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof Blob) {
				const bytes = (Blob as any).InternalAccessor.getBuffer(source);
				com.github.triniwiz.canvas.TNSImageBitmap.createFromBytesEncoded(
					Array.from(bytes),
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageAsset(
					source._asset.native,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			}
		});
	}

	static createFromRect(source: any, sx: number, sy: number, sWidth: number, sHeight: number, options: any) {
		return new Promise((resolve, reject) => {
			const opts = new com.github.triniwiz.canvas.TNSImageBitmap.Options();
			ImageBitmap.handleOptions(options, opts);
			if (source instanceof Canvas) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromCanvas(
					source.android,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageBitmap) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageBitmap(
					source.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageAsset) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageData) {
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof Blob) {
				const bytes = (Blob as any).InternalAccessor.getBuffer(source);
				com.github.triniwiz.canvas.TNSImageBitmap.createFromBytesEncoded(
					Array.from(bytes),
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
				console.log(source._asset.native)
				com.github.triniwiz.canvas.TNSImageBitmap.createFromImageAsset(
					source._asset.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new com.github.triniwiz.canvas.TNSImageBitmap.Callback({
						onError(error) {
							console.log('error',error);
							reject(error);
						},
						onSuccess(bitmap) {
							console.log('bitmap',bitmap);
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			}
		});
	}
}
