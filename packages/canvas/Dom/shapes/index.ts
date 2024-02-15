import { ImageAsset } from '../../ImageAsset';
import { Canvas } from '../../Canvas';
import { Paint } from '../Paint';

export * from './Rect';
export * from './Path';
export * from './Circle';
export * from './RoundedRect';
export * from './Line';
export * from './Points';
export * from './Oval';
export * from './Atlas';

export function vec(x: number, y: number) {
	return { x, y };
}

export function rect(x: number, y: number, width: number, height: number) {
	return { x, y, width, height };
}

export function drawAsImage(elements: Paint[], size: { width: number; height: number } = { width: 300, height: 150 }) {
	const canvas = Canvas.createCustomView();

	if (size.width !== 300 && size.height !== 150) {
		(canvas as any)._isBatch = true;
		canvas.width = size.width;
		canvas.height = size.height;
		(canvas as any)._isBatch = false;
		(canvas as any)._layoutNative();
	}

	for (const element of elements) {
		element._addCanvas(canvas);
		element.draw();
	}
	// const asset = new ImageAsset() as any;

	const snapshot = canvas.snapshot();

	// if (global.isAndroid) {
	// 	const value = new java.lang.Long(asset.native.__addr);
	// 	console.log(value);
	// 	(org as any).nativescript.canvas.NSCImageAsset.loadImageFromBitmap(new java.lang.Long(asset.native.__addr), snapshot.android);
	// }

	return snapshot;
}
