import { ImageSource, LayoutBase, Property, View, ViewBase, Screen } from '@nativescript/core';
import { Canvas } from '../Canvas';
import { ImageAsset } from '../ImageAsset';
import { Dom } from './Dom';
import { Paint } from './Paint';

const xProperty = new Property<Image, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const yProperty = new Property<Image, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const imageProperty = new Property<Image, any>({
	name: 'image',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
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

	invalidate() {
		const parent = this.parent;
		if (parent instanceof Dom) {
			parent._dirty?.();
		} else if (parent instanceof Paint) {
			parent.invalidate();
		}
	}

	draw() {
		if (this.image === undefined) {
			return;
		}
		const context = this._canvas.getContext('2d') as any;

		context.drawImage(this.image as any, this.x, this.y, this.width, this.height);
	}
}

imageProperty.register(Image);
xProperty.register(Image);
yProperty.register(Image);
