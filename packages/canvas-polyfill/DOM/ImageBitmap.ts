import { ImageAsset } from '@nativescript/canvas';

export class ImageBitmap {
	_asset: ImageAsset;
	#width: number = 0;
	#height: number = 0;
	get width(): number {
		return this.#width;
	}
	get height(): number {
		return this.#height;
	}

    close(){}
}
