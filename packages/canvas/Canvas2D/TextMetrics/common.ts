export class TextMetricsBase {
	protected nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	public readonly width: number;
	public readonly actualBoundingBoxLeft: number;
	public readonly actualBoundingBoxRight: number;

	public readonly actualBoundingBoxAscent: number;
	public readonly actualBoundingBoxDescent: number;

	public readonly fontBoundingBoxAscent: number;
	public readonly fontBoundingBoxDescent: number;

	public readonly emHeightAscent: number;
	public readonly emHeightDescent: number;
	public readonly hangingBaseline: number;
	public readonly alphabeticBaseline: number;
	public readonly ideographicBaseline: number;
}
