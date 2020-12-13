export abstract class TextMetricsBase {
	protected nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	public abstract readonly width: number;
	public abstract readonly actualBoundingBoxLeft: number;
	public abstract readonly actualBoundingBoxRight: number;

	public abstract readonly actualBoundingBoxAscent: number;
	public abstract readonly actualBoundingBoxDescent: number;

	public abstract readonly fontBoundingBoxAscent: number;
	public abstract readonly fontBoundingBoxDescent: number;

	public abstract readonly emHeightAscent: number;
	public abstract readonly emHeightDescent: number;
	public abstract readonly hangingBaseline: number;
	public abstract readonly alphabeticBaseline: number;
	public abstract readonly ideographicBaseline: number;
}
