import {ImageAssetBase, ImageAssetSaveFormat} from './common';
import {knownFolders, path as filePath} from '@nativescript/core';

declare var TNSImageAsset;

const main_queue = dispatch_get_current_queue();
const background_queue = dispatch_get_global_queue(21, 0);

export class ImageAsset extends ImageAssetBase {
	constructor() {
		super(TNSImageAsset.alloc().init());
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

	loadFromUrl(url: string): boolean {
		return this.native.loadImageFromUrlWithUrl(url);
	}

	loadFromUrlAsync(url: string) {
		return new Promise((resolve, reject) => {
			this.native.loadImageFromUrlAsyncWithUrlCallback(url, (error) => {
				if (error) {
					reject(error);
					return;
				}
				resolve(true);
			});
		});
	}

	loadFile(path: string): boolean {
		let realPath = path;
		if (typeof realPath === 'string') {
			if (realPath.startsWith('~/')) {
				realPath = filePath.join(
					knownFolders.currentApp().path,
					realPath.replace('~/', '')
				);
			}
		}
		return this.native.loadImageFromPathWithPath(realPath);
	}

	loadFileAsync(path: string) {
		return new Promise((resolve, reject) => {
			dispatch_async(background_queue, () => {
				const success = this.loadFile(path);
				dispatch_async(main_queue, () => {
					if (!success) {
						reject(this.error);
						return;
					}
					resolve(true);
				});
			});
		});
	}

	loadFromNative(image: any): boolean {
		return this.native.loadImageFromImageWithImage(image);
	}

	loadFromNativeAsync(image: any) {
		return new Promise((resolve, reject) => {
			dispatch_async(background_queue, () => {
				const success = this.loadFromNative(image);
				dispatch_async(main_queue, () => {
					if (!success) {
						reject(this.error);
						return;
					}
					resolve(true);
				});
			});
		});
	}

	loadFromBytes(bytes: Uint8Array | Uint8ClampedArray): boolean {
		return this.native.loadImageFromBytesWithArray(bytes as any);
	}

	loadFromBytesAsync(bytes: Uint8Array | Uint8ClampedArray) {
		return new Promise((resolve, reject) => {
			dispatch_async(background_queue, () => {
				const success = this.loadFromBytes(bytes);
				dispatch_async(main_queue, () => {
					if (!success) {
						reject(this.error);
						return;
					}
					resolve(true);
				});
			});
		});
	}

	scale(x: number, y: number) {
		this.native.scaleWithXY(x, y);
	}

	save(path: string, format: ImageAssetSaveFormat): boolean {
		return this.native.saveWithPathFormat(path, format.valueOf());
	}

	saveAsync(path: string, format: ImageAssetSaveFormat): Promise<boolean> {
		return new Promise((resolve, reject) => {
			const result = this.save(path, format);
			dispatch_async(main_queue, () => {
				if (!result) {
					reject(this.error);
					return;
				}
				resolve(result);
			});
		});
	}

	flipX() {
		this.native.flipX();
	}

	flipY() {
		this.native.flipY();
	}

	private _createQueue() {
		return dispatch_queue_create('TNSImageAsset', null);
	}
}
