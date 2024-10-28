export * from './Elements';
import { SVGBase } from './common';

export class Svg extends SVGBase {
	static fromSrcSync(value: string): SvgData | null;
	static fromSrc(value: string): Promise<SvgData>;
}

export class SvgData {
	readonly native: any;

	readonly width: number;

	readonly height: number;

	readonly data: ArrayBuffer | null;
}
