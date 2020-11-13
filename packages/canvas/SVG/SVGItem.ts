import {PercentLength, View} from '@nativescript/core';
import {Canvas} from "../Canvas";
import {SVG} from "./SVG";

export class SVGItem extends View {
	set stroke(value) {
		(this.style as any).stroke = value;
	}

	get stroke(): any {
		return (this.style as any).stroke;
	}

	set strokeWidth(value) {
		(this.style as any).strokeWidth = value;
	}

	get strokeWidth(): any {
		return (this.style as any).strokeWidth;
	}

	set strokeLinecap(value) {
		(this.style as any).strokeLinecap = value;
	}

	get strokeLinecap(): any {
		return (this.style as any).strokeLinecap;
	}

	set strokeLinejoin(value) {
		(this.style as any).strokeLinejoin = value;
	}

	get strokeLinejoin(): any {
		return (this.style as any).strokeLinejoin;
	}

	set strokeMiterlimitProperty(value) {
		(this.style as any).strokeMiterlimitProperty = value;
	}

	get strokeMiterlimitProperty(): any {
		return (this.style as any).strokeMiterlimitProperty;
	}

	get fill(): any {
		return (this.style as any).fill;
	}

	set fill(value) {
		(this.style as any).fill = value;
	}

	set fillRule(value) {
		(this.style as any).fillRule = value;
	}

	get fillRule(): any {
		return (this.style as any).fillRule;
	}

	_parseOpacity(value) {
		if (typeof value === 'string') {
			if (value.indexOf('%') > -1) {
				const percent = parseInt(value.replace('%', ''), 10);
				value = isNaN(percent) ? 1 : percent / 100;
			} else {
				let percent = parseFloat(value);
				value = isNaN(percent) ? 1 : percent;
			}
		}
		return value;
	}

	set fillOpacity(value) {
		value = this._parseOpacity(value);
		(this.style as any).fillOpacity = value;
	}

	get fillOpacity() {
		return (this.style as any).fillOpacity;
	}

	opacity;

	_canvas: Canvas;


	_doStroke(): boolean {
		return this._realStroke !== undefined && this._realStroke !== 'none';
	}

	_doFill(): boolean {
		return this._realFill !== 'none';
	}

	_doFillOpacity(): boolean {
		return this._realFillOpacity !== undefined;
	}

	get _realFillOpacity() {
		// @ts-ignore
		if ((this.parent && this.parent.fillOpacity) !== undefined && this.fillOpacity !== undefined) {
			return this._parseOpacity(this.fillOpacity);
		}
		// @ts-ignore
		return this._parseOpacity((this.parent && this.parent.fillOpacity)) || this._parseOpacity(this.fillOpacity);
	}


	get _realStroke() {
		// @ts-ignore
		if ((this.parent && this.parent.stroke) !== undefined && this.stroke !== undefined) {
			return this.stroke;
		}
		// @ts-ignore
		return (this.parent && this.parent.stroke) || this.stroke;
	}

	get _realFill() {
		// @ts-ignore
		if ((this.parent && this.parent.fill) !== undefined && this.fill !== undefined) {
			return this.fill;
		}
		// @ts-ignore
		return (this.parent && this.parent.fill) || this.fill;
	}

	get _realOpacity() {
		// @ts-ignore
		if ((this.parent && this.parent.opacity) !== undefined && this.opacity !== undefined) {
			return this.opacity;
		}
		// @ts-ignore
		if (this.parent && this.parent.opacity !== undefined) {
			// @ts-ignore
			return this.parent.opacity;
		}
		if (this.opacity !== undefined) {
			return this.opacity;
		}
		return 1;
	}

	_getRealSize(value, parent?: any, type?: string) {
		if (!parent) {
			parent = this.parent;
			while (!(parent instanceof SVG)) {
				parent = parent.parent;
				if (parent === undefined || parent === null) {
					break;
				}
			}
			if (parent instanceof SVG && (type === 'width' || type === 'height')) {
				if (type === 'width') {
					parent = parent.getMeasuredWidth();
				}
				if (type === 'height') {
					parent = parent.getMeasuredHeight();
				}
			}
		}
		const base = parent || 0;
		let size;
		if (value && value.unit === '%') {
			size = PercentLength.convertToString(value);
		}

		if (typeof value === 'string' || typeof value === 'number') {
			size = value;
		}

		if (typeof size === 'string' && (size.indexOf('%') > -1)) {
			size = (base * (parseInt(size.replace('%', '')) / 100));
		}
		return size;
	}

	_appendChild(id, child) {
		let parent = this.parent;
		while (!(parent instanceof SVG)) {
			parent = parent.parent;
			if (parent === undefined || parent === null) {
				break;
			}
		}
		if (parent instanceof SVG) {
			parent._children.set(id, child);
		}
	}

	_removeChild(id) {
		let parent = this.parent;
		while (!(parent instanceof SVG)) {
			parent = parent.parent;
			if (parent === undefined || parent === null) {
				break;
			}
		}
		if (parent instanceof SVG) {
			parent._children.delete(id);
		}
	}

	_getViewById(name: string) {
		let parent = (this as any).parent;
		while (!(parent instanceof SVG)) {
			parent = parent.parent;
			if (parent === undefined || parent == null) {
				break;
			}
		}
		if (!parent) {
			return null;
		}
		return parent._getViewById(name);
	}
}
