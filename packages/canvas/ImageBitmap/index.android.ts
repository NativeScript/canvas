import { ImageAsset } from '../ImageAsset';
import { ImageData } from '../Canvas2D';
import { ImageBitmapBase } from './common';
import { Canvas } from '../Canvas';
import lazy from '../utils';
import { ImageSource } from '@nativescript/core';
const ImageBitmapPremultiplyAlpha = {
	Default: lazy(() => org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha.Default),
	None: lazy(() => org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha.None),
	Premultiply: lazy(() => org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha.Premultiply),
};
const ImageBitmapColorSpaceConversion = {
	Default: lazy(() => org.nativescript.canvas.TNSImageBitmapColorSpaceConversion.Default),
	None: lazy(() => org.nativescript.canvas.TNSImageBitmapColorSpaceConversion.None),
};
const ImageBitmapResizeQuality = {
	Low: lazy(() => org.nativescript.canvas.TNSImageBitmapResizeQuality.Low),
	Medium: lazy(() => org.nativescript.canvas.TNSImageBitmapResizeQuality.Medium),
	High: lazy(() => org.nativescript.canvas.TNSImageBitmapResizeQuality.High),
	Pixelated: lazy(() => org.nativescript.canvas.TNSImageBitmapResizeQuality.Pixelated),
}
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
		if (value instanceof org.nativescript.canvas.TNSImageBitmap) {
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
								nativeOptions.setPremultiplyAlpha(ImageBitmapPremultiplyAlpha.Default());
								break;
							case 'Premultiply':
								nativeOptions.setPremultiplyAlpha(ImageBitmapPremultiplyAlpha.Premultiply());
							case 'none':
								nativeOptions.setPremultiplyAlpha(ImageBitmapPremultiplyAlpha.None());
								break;
						}
						break;
					case 'colorSpaceConversion':
						switch (value) {
							case 'default':
								nativeOptions.setColorSpaceConversion(ImageBitmapColorSpaceConversion.Default());
								break;
							case 'none':
								nativeOptions.setColorSpaceConversion(ImageBitmapColorSpaceConversion.None());
								break;
						}
						break;
					case 'resizeQuality':
						switch (value) {
							case 'low':
								nativeOptions.setResizeQuality(ImageBitmapResizeQuality.Low());
								break;
							case 'medium':
								nativeOptions.setResizeQuality(ImageBitmapResizeQuality.Medium());
								break;
							case 'high':
								nativeOptions.setResizeQuality(ImageBitmapResizeQuality.High());
								break;
							case 'pixelated':
								nativeOptions.setResizeQuality(ImageBitmapResizeQuality.Pixelated());
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
			const opts = new org.nativescript.canvas.TNSImageBitmap.Options();
			ImageBitmap.handleOptions(options, opts);
			if (source instanceof Canvas) {
				org.nativescript.canvas.TNSImageBitmap.createFromCanvas(
					source.android,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageBitmap) {
				org.nativescript.canvas.TNSImageBitmap.createFromImageBitmap(
					source.native,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageAsset) {
				org.nativescript.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageData) {
				org.nativescript.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
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
				org.nativescript.canvas.TNSImageBitmap.createFromBytesEncoded(
					Array.from(bytes),
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source && typeof source === 'object' && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
				org.nativescript.canvas.TNSImageBitmap.createFromImageAsset(
					source._asset.native,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			}else if (source instanceof ArrayBuffer){
				//@ts-ignore
				if(source.nativeObject){
					org.nativescript.canvas.TNSImageBitmap.createFromBufferEncoded(
						//@ts-ignore
						source.nativeObject,
						opts,
						new org.nativescript.canvas.TNSImageBitmap.Callback({
							onError(error) {
								reject(error);
							},
							onSuccess(bitmap) {
								resolve(ImageBitmap.fromNative(bitmap));
							},
						})
					);
				}else {
					org.nativescript.canvas.TNSImageBitmap.createFromBytesEncoded(
						Array.from(new Uint8Array(source as any) as any),
						opts,
						new org.nativescript.canvas.TNSImageBitmap.Callback({
							onError(error) {
								reject(error);
							},
							onSuccess(bitmap) {
								resolve(ImageBitmap.fromNative(bitmap));
							},
						})
					);
				}
			} else if (source instanceof ImageSource) {
				org.nativescript.canvas.TNSImageBitmap.createFromBitmap(
					source.android,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
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
			const opts = new org.nativescript.canvas.TNSImageBitmap.Options();
			ImageBitmap.handleOptions(options, opts);
			if (source instanceof Canvas) {
				org.nativescript.canvas.TNSImageBitmap.createFromCanvas(
					source.android,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageBitmap) {
				org.nativescript.canvas.TNSImageBitmap.createFromImageBitmap(
					source.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageAsset) {
				org.nativescript.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
						onError(error) {
							reject(error);
						},
						onSuccess(bitmap) {
							resolve(ImageBitmap.fromNative(bitmap));
						},
					})
				);
			} else if (source instanceof ImageData) {
				org.nativescript.canvas.TNSImageBitmap.createFromImageAsset(
					source.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
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
				org.nativescript.canvas.TNSImageBitmap.createFromBytesEncoded(
					Array.from(bytes),
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
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
				org.nativescript.canvas.TNSImageBitmap.createFromImageAsset(
					source._asset.native,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
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
			}else if (source instanceof ArrayBuffer){
				//@ts-ignore
				if(source.nativeObject){
					org.nativescript.canvas.TNSImageBitmap.createFromBufferEncoded(
						//@ts-ignore
						source.nativeObject,
						sx,
						sy,
						sWidth,
						sHeight,
						opts,
						new org.nativescript.canvas.TNSImageBitmap.Callback({
							onError(error) {
								reject(error);
							},
							onSuccess(bitmap) {
								resolve(ImageBitmap.fromNative(bitmap));
							},
						})
					);
				}else {
					org.nativescript.canvas.TNSImageBitmap.createFromBytesEncoded(
						Array.from(new Uint8Array(source as any) as any),
						sx,
						sy,
						sWidth,
						sHeight,
						opts,
						new org.nativescript.canvas.TNSImageBitmap.Callback({
							onError(error) {
								reject(error);
							},
							onSuccess(bitmap) {
								resolve(ImageBitmap.fromNative(bitmap));
							},
						})
					);
				}
			} else if (source instanceof ImageSource) {
				org.nativescript.canvas.TNSImageBitmap.createFromBitmap(
					source.android,
					sx,
					sy,
					sWidth,
					sHeight,
					opts,
					new org.nativescript.canvas.TNSImageBitmap.Callback({
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
}
