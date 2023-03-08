import { ImageAssetBase, ImageAssetSaveFormat } from './common';
import { knownFolders, path as filePath } from '@nativescript/core';
import { Helpers } from '../helpers';

let ctor;
export class ImageAsset extends ImageAssetBase {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.ImageAsset;
	}

	constructor(native) {
		super(native || ctor());
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
		return this.native.loadImageFromUrl(url);
	}

	fromUrlAsync(url: string) {
		return new Promise((resolve, reject) => {
			this.native.fromUrlCb(url, (success, error) => {
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

        console.log(typeof realPath);

		return false;//this.native.fromFileSync(realPath);
	}

	fromFile(path: string) {
		return new Promise((resolve, reject) => {
			if (typeof path === 'string') {
				if (path.startsWith('~/')) {
					path = filePath.join(knownFolders.currentApp().path, path.replace('~/', ''));
				}
			}

			this.native.fromFileCb(path, (success, error) => {
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	/*

    loadFromNative(image: any): boolean {
        return this.native.loadImageFromImage(image);
    }

    loadFromNativeAsync(image: any) {
        return new Promise((resolve, reject) => {
            this.native.loadImageFromImageAsync(
                image,
                new org.nativescript.canvas.TNSImageAsset.Callback({
                    onError(error) {
                        reject(error);
                    },
                    onSuccess(success) {
                        resolve(success);
                    },
                })
            );
        });
    }

    */

	loadFromBytesSync(bytes: Uint8Array | Uint8ClampedArray) {
		return this.native.loadImageFromBytes(bytes);
	}

	loadFromBytes(bytes: Uint8Array | Uint8ClampedArray) {
		return new Promise((resolve, reject) => {
			const callback = new org.nativescript.canvas.TNSImageAsset.Callback({
				onError(error) {
					reject(error);
				},
				onSuccess(success) {
					resolve(success);
				},
			});

			this.native.fromBytesCb(bytes, (success, error) => {
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
		let realFormat;
		switch (format) {
			case ImageAssetSaveFormat.PNG:
				realFormat = org.nativescript.canvas.TNSImageAssetFormat.PNG;
				break;
			case ImageAssetSaveFormat.ICO:
				realFormat = org.nativescript.canvas.TNSImageAssetFormat.ICO;
				break;
			case ImageAssetSaveFormat.BMP:
				realFormat = org.nativescript.canvas.TNSImageAssetFormat.BMP;
				break;
			case ImageAssetSaveFormat.TIFF:
				realFormat = org.nativescript.canvas.TNSImageAssetFormat.TIFF;
				break;
			default:
				realFormat = org.nativescript.canvas.TNSImageAssetFormat.JPG;
				break;
		}
		return this.native.saveSync(path, realFormat);
	}

	saveAsync(path: string, format: ImageAssetSaveFormat): Promise<boolean> {
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
