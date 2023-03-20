import { ImageSource, LayoutBase, Property, View, ViewBase } from '@nativescript/core';
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

	constructor() {
		super();
		console.log('??');
	}

	_canvas: Canvas;
	_addCanvas(canvas: Canvas) {
		this._canvas = canvas;
	}

	image;

	draw() {
		console.log('draw', this.image);
		if (this.image === undefined) {
			return;
		}
		console.log(this.image);
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;

		context.drawImage(this.image as any, this.x, this.y, this.width, this.height);
	}
}

imageProperty.register(Image);
xProperty.register(Image);
yProperty.register(Image);
