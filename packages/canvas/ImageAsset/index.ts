import { File, knownFolders, path as filePath, Utils } from '@nativescript/core';
import { Helpers } from '../helpers';

let ctor;
export class ImageAsset {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.ImageAsset;
	}

	_native;
	get native() {
		return this._native;
	}
	constructor(native?) {
		this._native = native || ctor();
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

	fromUrlSync(url: string): boolean {
		const fromUrlSync = this._getMethod('fromUrlSync');
		return fromUrlSync(url);
	}

	fromUrl(url: string) {
		const fromUrlCb = this._getMethod('fromUrlCb');
		return new Promise((resolve, reject) => {
			fromUrlCb(url, (success, error) => {
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	fromFileSync(path: string): boolean {
		const fromFileSync = this._getMethod('fromFileSync');
		let realPath = path;
		if (typeof realPath === 'string') {
			if (realPath.startsWith('~/')) {
				realPath = filePath.join(knownFolders.currentApp().path, realPath.replace('~/', ''));
			}
		}

		return fromFileSync(realPath);
	}

	fromFile(path: string) {
		const fromFileCb = this._getMethod('fromFileCb');
		return new Promise((resolve, reject) => {
			if (typeof path === 'string') {
				if (path.startsWith('~/')) {
					path = filePath.join(knownFolders.currentApp().path, path.replace('~/', ''));
				}
			}

			fromFileCb(path, (success, error) => {
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

    loadFromNative(image: any) {
        return new Promise((resolve, reject) => {
            this.native.loadImageFromImage(
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
		const fromBytesSync = this._getMethod('fromBytesSync');
		return fromBytesSync(bytes);
	}

	loadFromBytes(bytes: Uint8Array | Uint8ClampedArray) {
		const fromBytesCb = this._getMethod('fromBytesCb');
		return new Promise((resolve, reject) => {
			const callback = new org.nativescript.canvas.TNSImageAsset.Callback({
				onError(error) {
					reject(error);
				},
				onSuccess(success) {
					resolve(success);
				},
			});

			fromBytesCb(bytes, (success, error) => {
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}
			});
		});
	}

	scale(x: number, y: number) {
		const scale = this._getMethod('scale');
		scale(x, y);
	}

	saveSync(path: string, format: ImageAssetSaveFormat): boolean {
		const saveSync = this._getMethod('saveSync');
		return saveSync(path, format);
	}

	save(path: string, format: ImageAssetSaveFormat): Promise<boolean> {
		const saveCb = this._getMethod('saveCb');
		return new Promise((resolve, reject) => {
			saveCb(path, format, (success, error) => {
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
