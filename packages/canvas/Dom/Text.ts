import { Property, Screen } from '@nativescript/core';
import { Paint } from './Paint';

const xProperty = new Property<Text, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const yProperty = new Property<Text, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const textProperty = new Property<Text, string>({
	name: 'text',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export class Text extends Paint {
	x: number;
	y: number;
	text: string;
	_font: string;
	set font(value) {
		this.style.font = value;
		this._font = value;
	}

	draw() {
		if (typeof this.text !== 'string') {
			return;
		}
		const context = this._canvas.getContext('2d') as any;

		if (typeof this._font === 'string') {
			context.font = this._font;
		}

		const style = this._getPaintStyle();
		const color = this._getColor();
		switch (style) {
			case 'fill':
				context.fillStyle = color;
				context.fillText(this.text, this.x, this.y);
				break;
			case 'stroke':
				context.strokeStyle = color;
				context.lineWidth = this.strokeWidth;
				context.strokeText(this.text, this.x, this.y);
				break;
		}
	}
}

textProperty.register(Text);
xProperty.register(Text);
yProperty.register(Text);
