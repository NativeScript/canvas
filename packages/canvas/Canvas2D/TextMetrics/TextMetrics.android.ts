import { TextMetricsBase } from './common';

export class TextMetrics extends TextMetricsBase {
	nativeInstance: com.github.triniwiz.canvas.TNSTextMetrics
	constructor(instance: any) {
		super(instance);
	}

	get width(): number {
		return this.nativeInstance.getWidth();
	}

	get actualBoundingBoxLeft(): number {
		return this.nativeInstance.getActualBoundingBoxLeft();
	}

	get actualBoundingBoxRight(): number {
		return this.nativeInstance.getActualBoundingBoxRight();
	}

	get actualBoundingBoxAscent(): number {
		return this.nativeInstance.getActualBoundingBoxAscent();
	}

	get actualBoundingBoxDescent(): number {
		return this.nativeInstance.getActualBoundingBoxDescent();
	}

	get fontBoundingBoxAscent(): number {
		return this.nativeInstance.getFontBoundingBoxAscent();
	}

	get fontBoundingBoxDescent(): number {
		return this.nativeInstance.getFontBoundingBoxDescent();
	}

	get emHeightAscent(): number {
		return this.nativeInstance.getEmHeightAscent();
	}

	get emHeightDescent(): number {
		return this.nativeInstance.getEmHeightDescent();
	}

	get hangingBaseline(): number {
		return this.nativeInstance.getHangingBaseline();
	}

	get alphabeticBaseline(): number {
		return this.nativeInstance.getAlphabeticBaseline();
	}

	get ideographicBaseline(): number {
		return this.nativeInstance.getIdeographicBaseline();
	}
}