import {TextMetricsBase} from './common';

export declare class TextMetrics extends TextMetricsBase {
	readonly width(): number;
	readonly actualBoundingBoxLeft(): number;
	readonly actualBoundingBoxRight(): number;
	readonly actualBoundingBoxAscent(): number;
	readonly actualBoundingBoxDescent(): number;
	readonly fontBoundingBoxAscent(): number;
	readonly fontBoundingBoxDescent(): number;
	readonly emHeightAscent(): number;
	readonly emHeightDescent(): number;
	readonly hangingBaseline(): number;
	readonly alphabeticBaseline(): number;
	readonly ideographicBaseline(): number;

	constructor(nativeInstance: any);
}
