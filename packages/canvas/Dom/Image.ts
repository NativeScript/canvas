import { ImageSource, LayoutBase, Property, View, ViewBase, Screen } from '@nativescript/core';
import { Canvas } from '../Canvas';
import { ImageAsset } from '../ImageAsset';

const xProperty = new Property<Image, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const yProperty = new Property<Image, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const imageProperty = new Property<Image, any>({
	name: 'image',
});

export class Image extends View {
	x: number;
	y: number;
	width: number;
	height: number;

	_canvas: Canvas;
	_addCanvas(canvas: Canvas) {
		this._canvas = canvas;
	}

	image;

	draw() {
		if (this.image === undefined) {
			return;
		}
		const context = this._canvas.getContext('2d') as any;
		const scale = Screen.mainScreen.scale;

		context.drawImage(this.image as any, this.x * scale, this.y * scale, this.width * scale, this.height * scale);
	}
}

imageProperty.register(Image);
xProperty.register(Image);
yProperty.register(Image);
