import { Property, ViewBase } from '@nativescript/core';
import { Paint } from '../Paint';
import { CanvasRenderingContext2D } from '@nativescript/canvas';

const imageProperty = new Property<Atlas, any>({
	name: 'image',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const spritesProperty = new Property<Atlas, { x: number; y: number; width: number; height: number }[]>({
	name: 'sprites',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const transformsProperty = new Property<Atlas, { scos: number; ssin: number; tx: number; ty: number }[]>({
	name: 'transforms',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const colorsProperty = new Property<Atlas, string[]>({
	name: 'colors',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const blendModeProperty = new Property<Atlas, GlobalCompositeOperation>({
	name: 'blendMode',
	defaultValue: 'destination-over',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export class Atlas extends Paint {
	image;
	sprites: { x: number; y: number; width: number; height: number }[];
	transforms: { scos: number; ssin: number; tx: number; ty: number }[];
	colors: string[];
	blendMode: GlobalCompositeOperation;
	_children: Paint[] = [];

	draw() {
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		context.drawAtlas(this.image, this.transforms, this.sprites, this.colors ?? null, this.blendMode);
	}

	_addViewToNativeVisualTree(view: ViewBase, atIndex?: number): boolean {
		if (view === this._canvas) {
			this.nativeView.addView(this._canvas.nativeView);
			return true;
		} else if (view instanceof Paint) {
			view._canvas = this._canvas;
			this._children.push(view);
		}
		return false;
	}
}

imageProperty.register(Atlas);
spritesProperty.register(Atlas);
transformsProperty.register(Atlas);
colorsProperty.register(Atlas);
blendModeProperty.register(Atlas);
