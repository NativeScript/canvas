// No Windows SVG renderer yet: importing is safe (canvas-polyfill's DOM references these classes
// at startup), and anything that creates an SVG document or element throws from
// Helpers.initialize().
import { SVGBase } from './common';

export * from './Elements';

export class SvgData {
	native: any;

	static fromNative(value) {
		return null;
	}

	get width(): number {
		return 0;
	}

	get height(): number {
		return 0;
	}

	get data(): ArrayBuffer | null {
		return null;
	}
}

const unsupported = () => new Error('@nativescript/canvas-svg is not supported on Windows yet');

export class Svg extends SVGBase {
	createNativeView() {
		return new (global as any).Microsoft.UI.Xaml.Controls.Grid();
	}

	setSvgData(value: SvgData) {}

	__redraw() {}

	static fromSrcSync(value: string): SvgData | null {
		throw unsupported();
	}

	static fromSrc(value: string): Promise<SvgData> {
		return Promise.reject(unsupported());
	}
}
