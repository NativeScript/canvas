import {TextMetricsBase} from './common';

export class TextMetrics extends TextMetricsBase {
	constructor(nativeInstance: any) {
		super(nativeInstance);
	}

	get width() {
		return this.nativeInstance.getWidth();
	}
}
