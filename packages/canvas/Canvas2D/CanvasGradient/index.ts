/** Anything csscolorparser accepts; the native side silently ignores the rest. */
const COLOR_RE = /^(#[0-9a-f]{3,8}|(rgb|rgba|hsl|hsla|hwb|lab|lch|oklab|oklch|color)\(.*\)|[a-z]+)$/i;

export class CanvasGradient {
	private _native;

	constructor(nativeInstance?: any) {
		this._native = nativeInstance;
	}

	get native() {
		return this._native;
	}

	static fromNative(nativeInstance) {
		return new CanvasGradient(nativeInstance);
	}

	public addColorStop(offset: number, color: any): void {
		if (typeof offset !== 'number' || !isFinite(offset) || offset < 0 || offset > 1) {
			const error: any = new Error(`Failed to execute 'addColorStop' on 'CanvasGradient': The provided value (${offset}) is outside the range (0.0, 1.0).`);
			error.name = 'IndexSizeError';
			throw error;
		}
		if (typeof color !== 'string' || !COLOR_RE.test(color.trim())) {
			const error: any = new Error(`Failed to execute 'addColorStop' on 'CanvasGradient': The value provided ('${color}') could not be parsed as a color.`);
			error.name = 'SyntaxError';
			throw error;
		}
		this.native.addColorStop(offset, color);
	}
}
