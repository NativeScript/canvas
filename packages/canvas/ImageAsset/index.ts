import { File, knownFolders, path as filePath, Utils, Observable, EventData } from '@nativescript/core';
import { Helpers } from '../helpers';

let ctor;
// store ref if loading
const loaders = new Map<ImageAsset, number>();

export class ImageAsset extends Observable {
	static {
		Helpers.initialize();
	}

	_native;
	_android;
	get native() {
		return this._native;
	}
	constructor(native?) {
		super();
		this._native = native || new global.CanvasModule.ImageAsset();
		if (__ANDROID__) {
			const ref = long(this.native.__getRef());
			this._android = new (<any>org).nativescript.canvas.NSCImageAsset(ref);
		}
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

	private emitComplete(success: boolean, error) {
		this.notify({ eventName: 'complete', object: this, complete: success, error });
	}

	fromUrl(url: string) {
		return new Promise((resolve, reject) => {
			if (__ANDROID__) {
				const asset = this._android.getAsset();
				const ref = new WeakRef(this);
				(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromUrlAsync(
					asset,
					url,
					new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
						onComplete(success: boolean) {
							const owner = ref.get();
							if (!success) {
								const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
								if (owner) {
									owner.emitComplete(success, error);
								}
								reject(error);
							} else {
								if (owner) {
									owner.emitComplete(success, undefined);
								}
								resolve(success);
							}
						},
					})
				);
				return;
			}

			this._incrementStrongRef();
			this.native.fromUrlCb(url, (success, error) => {
				this.emitComplete(success, error);
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

		const ret = this.native.fromFileSync(realPath);

		return ret;
	}

	fromFile(path: string) {
		return new Promise((resolve, reject) => {
			if (typeof path === 'string') {
				if (path.startsWith('~/')) {
					path = filePath.join(knownFolders.currentApp().path, path.replace('~/', ''));
				}
			}

			if (__ANDROID__) {
				const asset = this._android.getAsset();
				const ref = new WeakRef(this);
				if (typeof path === 'string' && path.indexOf('.webp') > -1) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadWebPAsync(
						asset,
						path,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						})
					);
				} else {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromPathAsync(
						asset,
						path,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						})
					);
				}
				return;
			}

			this._incrementStrongRef();

			this.native.fromFileCb(path.toString(), (success, error) => {
				this.emitComplete(success, error);
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
			if (__ANDROID__) {
				const ref = new WeakRef(this);
				const asset = this._android.getAsset();

				if (!ArrayBuffer.isView(bytes)) {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBytesAsync(
						asset,
						bytes,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						})
					);
				} else {
					(<any>org).nativescript.canvas.NSCImageAsset.loadImageFromBufferAsync(
						asset,
						bytes,
						new (<any>org).nativescript.canvas.NSCImageAsset.Callback({
							onComplete(success) {
								const owner = ref.get();
								if (!success) {
									const error = (<any>org).nativescript.canvas.NSCImageAsset.getError(asset);
									if (owner) {
										owner.emitComplete(success, error);
									}
									reject(error);
								} else {
									if (owner) {
										owner.emitComplete(success, undefined);
									}
									resolve(success);
								}
							},
						})
					);
				}
				return;
			}

			this._incrementStrongRef();
			this.native.fromBytesCb(bytes, (success, error) => {
				this.emitComplete(success, error);
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
