import { Paint } from './Paint';

export class Fill extends Paint {
	draw() {
		const context = this._canvas.getContext('2d') as any;
		context.drawPaint(this.color.hex);
	}
}
