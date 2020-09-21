import {ImageDataBase} from './common';

export declare class ImageData extends ImageDataBase {
	protected constructor(nativeInstance: any);

	static fromNative(nativeInstance: any): ImageData;

	static from(instance: ImageData): ImageData;
}
