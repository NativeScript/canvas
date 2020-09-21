import {TextMetricsBase} from './common';

export class TextMetrics extends TextMetricsBase {
	constructor(instance: any) {
		super(instance);
	}

	get width(): number {
		return this.nativeInstance.width;
	}
}
