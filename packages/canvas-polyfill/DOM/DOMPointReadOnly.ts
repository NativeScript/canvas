export class DOMPointReadOnly {
	x: number = 0;
	y: number = 0;
	z: number = 0;
	w: number = 1;
	constructor(x = 0, y = 0, z = 0, w = 1) {
		if (typeof x === 'number') {
			this.x = x;
		}
		if (typeof y === 'number') {
			this.y = y;
		}
		if (typeof z === 'number') {
			this.z = z;
		}
		if (typeof w === 'number') {
			this.w = w;
		}
	}

	static fromPoint(value: DOMPointReadOnly | { x?: number; y?: number; z?: number; w?: number }) {
		if (value instanceof DOMPointReadOnly) {
			return new DOMPointReadOnly(value.x, value.y, value.z, value.y);
		}
		return new DOMPointReadOnly(value?.x ?? 0, value?.y ?? 0, value?.z ?? 0, value?.w ?? 1);
	}

	toJSON() {
		return {
			x: this.x,
			y: this.y,
			z: this.z,
			w: this.w,
		};
	}
}

export class DOMPoint extends DOMPointReadOnly {
	static fromPoint(value: DOMPointReadOnly | DOMPoint | { x?: number; y?: number; z?: number; w?: number }) {
		if (value instanceof DOMPointReadOnly) {
			return new DOMPoint(value.x, value.y, value.z, value.y);
		}
		return new DOMPoint(value?.x ?? 0, value?.y ?? 0, value?.z ?? 0, value?.w ?? 1);
	}
}
