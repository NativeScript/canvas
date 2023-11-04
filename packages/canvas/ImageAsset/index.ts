import { File, knownFolders, path as filePath, Utils } from '@nativescript/core';
import { Helpers } from '../helpers';

let ctor;
// store ref if loading
const loaders = new Map<ImageAsset, number>();
export class ImageAsset {
	static {
		Helpers.initialize();
	}

	_native;
	get native() {
		return this._native;
	}
	constructor(native?) {
		this._native = native || new global.CanvasModule.ImageAsset();
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
		const count = loaders.get(this) - 1;

		if (count <= 0) {
			loaders.delete(this);
		}
	}

	fromUrl(url: string) {
		return new Promise((resolve, reject) => {
			this._incrementStrongRef();
			this.native.fromUrlCb(url, (success, error) => {
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

		return this.native.fromFileSync(realPath);
	}

	fromFile(path: string) {
		return new Promise((resolve, reject) => {
			if (typeof path === 'string') {
				if (path.startsWith('~/')) {
					path = filePath.join(knownFolders.currentApp().path, path.replace('~/', ''));
				}
			}

			this._incrementStrongRef();

			this.native.fromFileCb(path, (success, error) => {
				if (error) {
					reject(error);
				} else {
					resolve(success);
				}

				this._decrementStrongRefAndRemove();
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
		return this.native.fromBytesSync(bytes);
	}

	loadFromBytes(bytes: Uint8Array | Uint8ClampedArray) {
		return new Promise((resolve, reject) => {
			this._incrementStrongRef();
			this.native.fromBytesCb(bytes, (success, error) => {
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
