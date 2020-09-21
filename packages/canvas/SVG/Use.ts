import {G} from "./G";
import {SVG} from "./SVG";
import {SVGItem} from "./SVGItem";
import {Symbol} from './Symbol';
import {Circle} from "./Circle";
import {Rect} from './Rect';

export class Use extends SVGItem {
	href: string;
	xlink: { href?: string } = {};

	x: any;
	y: any;
	width: any = 0;
	height: any = 0;
	clone;

	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		let href;
		if (this.xlink && this.xlink.href) {
			href = this.xlink.href;
		}
		if (this.href) {
			href = this.href;
		}
		let svg = this.parent;
		while (!(svg instanceof SVG)) {
			svg = svg.parent;
		}
		if (href && svg instanceof SVG) {
			this.clone = svg._children.get(href.replace('#', ''));
			if (this.clone && this.clone.handleValues) {
				const v = Object.assign(Object.create(Object.getPrototypeOf(this.clone)), this.clone);
				if (v instanceof Circle) {
					if (this.x !== undefined) {
						v.cx = this.x;
					}
					if (this.y !== undefined) {
						v.cy = this.y;
					}
				} else if (v instanceof Rect) {
					if (this.x !== undefined) {
						v.x = this.x;
					}
					if (this.y !== undefined) {
						v.y = this.y;
					}
				} else if (v instanceof G || v instanceof Symbol) {
					v._views.forEach((view) => {
						if (view instanceof Circle) {
							if (this.x !== undefined) {
								view.cx = this.x;
							}
							if (this.y !== undefined) {
								view.cy = this.y;
							}
						} else if (view instanceof Rect) {
							if (this.x !== undefined) {
								view.x = this.x;
							}
							if (this.y !== undefined) {
								view.y = this.y;
							}
						}
					});
				}
				v.width = this.width;
				v.height = this.height;
				if (typeof this.opacity === 'number') {
					v.opacity = this.opacity;
				}
				if (this.fill !== undefined) {
					v.fill = this.fill;
				}
				if (this.stroke !== undefined) {
					v.stroke = this.stroke;
				}
				v.strokeWidth = this.strokeWidth;
				v.fillOpacity = this._realFillOpacity;
				v.handleValues();
			}
		}
	}
}
