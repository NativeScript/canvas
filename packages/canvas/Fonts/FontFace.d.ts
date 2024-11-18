export type TypedArray = Int8Array | Uint8Array | Uint8ClampedArray | Int16Array | Uint16Array | Int32Array | Uint32Array | Float32Array | Float64Array;

type stretchName = 'ultra-condensed' | 'extra-condensed' | 'condensed' | 'semi-condensed' | 'normal' | 'semi-expanded' | 'expanded' | 'extra-expanded' | 'ultra-expanded';
type strechPercent = '50%' | '62.5%' | '75%' | '87.5%' | '100%' | '112.5%' | '125%' | '150%' | '200%' | '300%' | '400%';
type stretch = stretchName | strechPercent;
interface FontDescriptor {
	ascentOverride?: 'normal' | `${number}%`;
	descentOverride?: 'normal' | `${number}%`;
	display?: 'auto' | 'block' | 'swap' | 'fallback' | 'optional';
	featureSettings?: string;
	lineGapOverride?: 'normal' | `${number}%`;
	stretch?: stretch | `${stretch} ${stretch}`;
	style?: 'normal' | 'italic' | 'oblique' | `oblique ${number}deg`;
	unicodeRange?: string;
	variationSettings?: 'normal' | `${string} ${number}`;
	weight?: 'normal' | 'bold' | 'bolder' | 'lighter' | `${number}` | '100' | '200' | '300' | '400' | '500' | '600' | '700' | '800' | '900';
}

export function loadFontsFromCSS(url: string): Promise<FontFace[]>;

export function importFontsFromCSS(url: string): Promise<FontFace[]>;

export class FontFace {
	constructor(family: string, source?: string | TypedArray | ArrayBuffer, descriptors?: FontDescriptor);

	status: 'unloaded' | 'loading' | 'loaded' | 'error';

	load(): Promise<void>;

	ascentOverride?: 'normal' | `${number}%`;
	descentOverride?: 'normal' | `${number}%`;
	display?: 'auto' | 'block' | 'swap' | 'fallback' | 'optional';
	featureSettings?: string;
	lineGapOverride?: 'normal' | `${number}%`;
	stretch?: stretch | `${stretch} ${stretch}`;
	style?: 'normal' | 'italic' | 'oblique' | `oblique ${number}deg`;
	unicodeRange?: string;
	variationSettings?: 'normal' | `${string} ${number}`;
	weight?: 'normal' | 'bold' | 'bolder' | 'lighter' | `${number}` | '100' | '200' | '300' | '400' | '500' | '600' | '700' | '800' | '900';
}
