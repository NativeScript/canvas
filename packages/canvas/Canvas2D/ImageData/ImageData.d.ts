import { ImageDataBase } from './common';

export declare class ImageData extends ImageDataBase {
	constructor(width: number, height: number);

	constructor(arrayData: Uint8ClampedArray, width: number, height?: number);

	static fromNative(nativeInstance: any): ImageData;

	static from(instance: ImageData): ImageData;
}
