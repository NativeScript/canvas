import { ImageAssetBase, ImageAssetSaveFormat } from './common';

export declare class ImageAsset extends ImageAssetBase {
	width: number;
	height: number;
	error: string;

	constructor(native?: any);

	fromUrlSync(url: string): boolean;

	fromUrlAsync(url: string): Promise<boolean>;

	fromFileSync(path: string): boolean;

	fromFile(path: string): Promise<boolean>;

	loadFromBytesSync(bytes: Uint8Array | Uint8ClampedArray): boolean;

	loadFromBytes(bytes: Uint8Array | Uint8ClampedArray): Promise<boolean>;

	scale(x: number, y: number);

	saveSync(path: string, format: ImageAssetSaveFormat): boolean;

	saveAsync(path: string, format: ImageAssetSaveFormat): Promise<boolean>;

	flipX();

	flipY();
}
