export class TextMetrics {
	_native;
	constructor(nativeInstance?: any) {
		this._native = nativeInstance;
	}

	get native() {
		return this._native;
	}

	get width(): number {
		return this.native.width;
	}

	get actualBoundingBoxLeft(): number {
		return this.native.actualBoundingBoxLeft;
	}

	get actualBoundingBoxRight(): number {
		return this.native.actualBoundingBoxRight;
	}

	get actualBoundingBoxAscent(): number {
		return this.native.actualBoundingBoxAscent;
	}

	get actualBoundingBoxDescent(): number {
		return this.native.actualBoundingBoxDescent;
	}

	get fontBoundingBoxAscent(): number {
		return this.native.fontBoundingBoxAscent;
	}

	get fontBoundingBoxDescent(): number {
		return this.native.fontBoundingBoxDescent;
	}

	get emHeightAscent(): number {
		return this.native.emHeightAscent;
	}

	get emHeightDescent(): number {
		return this.native.emHeightDescent;
	}

	get hangingBaseline(): number {
		return this.native.hangingBaseline;
	}

	get alphabeticBaseline(): number {
		return this.native.alphabeticBaseline;
	}

	get ideographicBaseline(): number {
		return this.native.ideographicBaseline;
	}
}
