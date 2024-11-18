import { Utils } from '@nativescript/core';
type TypedArray = Int8Array | Uint8Array | Uint8ClampedArray | Int16Array | Uint16Array | Int32Array | Uint32Array | Float32Array | Float64Array;

const url_ex = /url\(([^)]+?)\.(woff2?|ttf|otf|eot)\)/;
declare const kotlin;
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

export function loadFontsFromCSS(url: string) {
	return new Promise<any[]>((resolve, reject) => {
		const cb = new kotlin.jvm.functions.Function2({
			invoke(fonts: java.util.List<org.nativescript.canvas.NSCFontFace>, error) {
				const count = fonts.size();
				const ret = new Array(count);
				if (error) {
					reject(error);
				} else {
					for (let i = 0; i < count; i++) {
						ret[i] = FontFace.fromNative(fonts.get(i));
					}
					resolve(ret);
				}
			},
		});
		org.nativescript.canvas.NSCFontFace.importFromRemote(Utils.android.getApplicationContext(), url, false, cb);
	});
}

export function importFontsFromCSS(url: string) {
	return new Promise<any[]>((resolve, reject) => {
		const cb = new kotlin.jvm.functions.Function2({
			invoke(fonts: java.util.List<org.nativescript.canvas.NSCFontFace>, error) {
				const count = fonts.size();
				const ret = new Array(count);
				if (error) {
					reject(error);
				} else {
					for (let i = 0; i < count; i++) {
						const font = fonts.get(i) as org.nativescript.canvas.NSCFontFace;
						ret[i] = FontFace.fromNative(font);
						const path = font.getFontPath();
						if (path) {
							global.CanvasModule.__addFontFamily(font.getFontFamily(), [path]);
						}
					}
					resolve(ret);
				}
			},
		});
		org.nativescript.canvas.NSCFontFace.importFromRemote(Utils.android.getApplicationContext(), url, true, cb);
	});
}

export class FontFace {
	native_: org.nativescript.canvas.NSCFontFace;
	private extension?: string;
	constructor(family: string, source?: string | TypedArray | ArrayBuffer, descriptors?: FontDescriptor) {
		if (arguments.length === 4 && arguments[3]) {
			return;
		}
		if (source) {
			if (ArrayBuffer.isView(source) || source instanceof ArrayBuffer) {
				this.native_ = new org.nativescript.canvas.NSCFontFace(family, source as never);
			} else if (typeof source === 'string') {
				const matches = source.match(url_ex);
				this.extension = matches[2];
				const url = `${matches[1]}.${matches[2]}`;
				this.native_ = new org.nativescript.canvas.NSCFontFace(family, url ?? source, null);
			}
		} else {
			this.native_ = new org.nativescript.canvas.NSCFontFace(family);
		}

		if (descriptors) {
			const descriptor = `
            @font-face {
             font-family: ${family};
             ascent-override: ${descriptors.ascentOverride};
             descent-override: ${descriptors.descentOverride};
             display: ${descriptors.display};
             feature-settings: ${descriptors.featureSettings};
             line-gap-override: ${descriptors.lineGapOverride};
             stretch: ${descriptors.stretch};
             style: ${descriptors.style};
             unicode-range: ${descriptors.unicodeRange};
             variation-settings: ${descriptors.variationSettings};
             weight: ${descriptors.weight};
        }`;

			this.native_.updateDescriptor(descriptor);
		}
	}

	toJSON() {
		return {
			ascentOverride: this.ascentOverride,
			descentOverride: this.descentOverride,
			display: this.display,
			family: this.family,
			status: this.status,
			style: this.style,
			weight: this.weight,
		};
	}

	load() {
		return new Promise<void>((resolve, reject) => {
			if (this.status === 'loaded') {
				resolve();
				return;
			}
			const font = this.native_;
			const useAlias = this.extension !== 'ttf';
			const cb = new kotlin.jvm.functions.Function1({
				invoke(error) {
					if (error) {
						reject(error);
					} else {
						const path = font.getFontPath();
						if (path) {
							const name = font.getFontFamily();
							global.CanvasModule.__addFontFamily(useAlias ? name : null, [path]);
						}
						resolve();
					}
				},
			});
			this.native_.load(Utils.android.getApplicationContext(), cb);
		});
	}

	get ascentOverride() {
		return 'normal';
	}

	set ascentOverride(value) {}

	get descentOverride() {
		return 'normal';
	}

	set descentOverride(value) {}

	get display() {
		switch (this.native_.getDisplay()) {
			case org.nativescript.canvas.NSCFontFace.NSCFontDisplay.Auto:
				return 'auto';
			case org.nativescript.canvas.NSCFontFace.NSCFontDisplay.Block:
				return 'block';
			case org.nativescript.canvas.NSCFontFace.NSCFontDisplay.Fallback:
				return 'fallback';
			case org.nativescript.canvas.NSCFontFace.NSCFontDisplay.Optional:
				return 'optional';
			case org.nativescript.canvas.NSCFontFace.NSCFontDisplay.Swap:
				return 'swap';
		}
	}

	set display(value: string) {
		this.native_.setFontDisplay(value);
	}

	get family() {
		return this.native_.getFontFamily();
	}

	get status() {
		switch (this.native_.getStatus()) {
			case org.nativescript.canvas.NSCFontFace.NSCFontFaceStatus.loaded:
				return 'loaded';
			case org.nativescript.canvas.NSCFontFace.NSCFontFaceStatus.loading:
				return 'loading';
			case org.nativescript.canvas.NSCFontFace.NSCFontFaceStatus.unloaded:
				return 'unloaded';
		}
	}

	get style() {
		return this.native_.getStyle().toString();
	}

	get weight() {
		switch (this.native_.getWeight()) {
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.Thin:
				return 'thin';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.ExtraLight:
				return 'extra-light';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.Light:
				return 'light';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.Normal:
				return 'normal';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.Medium:
				return 'medium';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.SemiBold:
				return 'semi-bold';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.Bold:
				return 'bold';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.ExtraBold:
				return 'extra-bold';
			case org.nativescript.canvas.NSCFontFace.NSCFontWeight.Black:
				return 'black';
		}
	}

	set weight(value: string) {
		this.native_.setFontWeight(value);
	}

	static fromNative(native: any): FontFace {
		if (native instanceof org.nativescript.canvas.NSCFontFace) {
			const font = new (<any>FontFace)('', null, null, true);
			font.native_ = native;
			return font;
		}
		return null;
	}
}
