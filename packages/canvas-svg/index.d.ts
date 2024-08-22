export * from './Elements';
import { SVGBase } from './common';

export class Svg extends SVGBase {
	static fromSrcSync(value: string): Svg | null;
	static fromSrc(value: string): Promise<Svg>;
}
