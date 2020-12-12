import {TextMetricsBase} from './common';

export class TextMetrics extends TextMetricsBase {
	constructor(instance: any) {
		super(instance);
	}

	get width(): number {
		return this.nativeInstance.width;
	}

	get actualBoundingBoxLeft(): number {
		return this.nativeInstance.actualBoundingBoxLeft;
	}

	get actualBoundingBoxRight(): number {
		return this.nativeInstance.actualBoundingBoxRight;
	}

	get actualBoundingBoxAscent(): number {
		return this.nativeInstance.actualBoundingBoxAscent;
	}

	get actualBoundingBoxDescent(): number {
		return this.nativeInstance.actualBoundingBoxDescent;
	}

	get fontBoundingBoxAscent(): number {
		return this.nativeInstance.fontBoundingBoxAscent;
	}

	get fontBoundingBoxDescent(): number {
		return this.nativeInstance.fontBoundingBoxDescent;
	}

	get emHeightAscent(): number {
		return this.nativeInstance.emHeightAscent;
	}

	get emHeightDescent(): number {
		return this.nativeInstance.emHeightDescent;
	}

	get hangingBaseline(): number {
		return this.nativeInstance.hangingBaseline;
	}

	get alphabeticBaseline(): number {
		return this.nativeInstance.alphabeticBaseline;
	}

	get ideographicBaseline(): number {
		return this.nativeInstance.ideographicBaseline;
	}
}