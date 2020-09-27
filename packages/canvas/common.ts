(Set as any).prototype.remove = function (value) {
	this.delete(value);
};

export class TouchList extends Array {
	item(index: number) {
		return this[index];
	}
}

export interface CanvasRenderingContext {}
