import { Property } from '@nativescript/core';
import { Paint } from '../Paint';
import { Path2D } from '../../Canvas2D';

export const pathProperty = new Property<Path, string>({
	name: 'path',
});

export class Path extends Paint {
	_path: Path2D;
	path: string;

	[pathProperty.setNative](value) {
		if (value) {
			this._path = new Path2D(value);
		}
	}

	draw() {
		if (this._path === undefined) {
			return;
		}
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;

		const style = this.paintStyle;
		if (style === 'fill') {
			context.fillStyle = this.color.hex;
			context.fill(this._path as any);
		} else if (style === 'stroke') {
			context.lineWidth = this.strokeWidth;
			context.strokeStyle = this.color.hex;
			context.stroke(this._path as any);
		}
	}
}

pathProperty.register(Path);
