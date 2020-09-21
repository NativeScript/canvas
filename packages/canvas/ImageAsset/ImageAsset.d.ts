import {ImageAssetBase, ImageAssetSaveFormat,} from './common';

export declare class ImageAsset extends ImageAssetBase {
	width: number;
	height: number;
	error: string;

	constructor();

	loadFile(path: string): boolean;

	loadFileAsync(path: string): Promise<boolean>;

	loadFromNative(image: any): boolean;

	loadFromNativeAsync(image: any): Promise<boolean>;

	loadFromBytes(bytes: Uint8Array | Uint8ClampedArray): boolean;

	loadFromBytesAsync(bytes: Uint8Array | Uint8ClampedArray): Promise<boolean>;

	scale(x: number, y: number);

	save(path: string, format: ImageAssetSaveFormat): boolean;

	saveAsync(path: string, format: ImageAssetSaveFormat): Promise<boolean>;

	flipX();

	flipY();
}
