export type TypedArray = Int8Array | Uint8Array | Uint8ClampedArray | Int16Array | Uint16Array | Int32Array | Uint32Array | Float32Array | Float64Array;

const url_ex = /url\(([^)]+?\.(?:woff2?|ttf|otf|eot))\)/;

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
		NSCFontFace.importFromRemoteWithUrlLoadCallback(url, false, (fonts, error) => {
			const count = fonts.count;
			const ret = new Array(count);
			if (error) {
				reject(error);
			} else {
				for (let i = 0; i < count; i++) {
					ret[i] = FontFace.fromNative(fonts.objectAtIndex(i));
				}
				resolve(ret);
			}
		});
	});
}

export function importFontsFromCSS(url: string) {
	return new Promise<any[]>((resolve, reject) => {
		NSCFontFace.importFromRemoteWithUrlLoadCallback(url, true, (fonts, error) => {
			const count = fonts.count;
			const ret = new Array(count);
			if (error) {
				reject(error);
			} else {
				for (let i = 0; i < count; i++) {
					ret[i] = FontFace.fromNative(fonts.objectAtIndex(i));
				}
				resolve(ret);
			}
		});
	});
}

export class FontFace {
	native_: NSCFontFace;
	constructor(family: string, source?: string | TypedArray | ArrayBuffer, descriptors?: FontDescriptor) {
		if (arguments.length === 4 && arguments[3]) {
			return;
		}
		if (source) {
			if (ArrayBuffer.isView(source) || source instanceof ArrayBuffer) {
				this.native_ = NSCFontFace.alloc().initWithFamilyData(family, NSData.dataWithData(source as never));
			} else if (typeof source === 'string') {
				const matches = source.match(url_ex);
				this.native_ = NSCFontFace.alloc().initWithFamilySource(family, matches[1] ?? source);
			}
		} else {
			this.native_ = NSCFontFace.alloc().initWithFamily(family);
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

			this.native_.updateDescriptorWithValue(descriptor);
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
			this.native_.load((error) => {
				if (error) {
					reject(error);
				} else {
					resolve();
				}
			});
		});
	}

	get ascentOverride() {
		return this.native_.ascentOverride;
	}

	set ascentOverride(value) {}

	get descentOverride() {
		return this.native_.descentOverride;
	}

	set descentOverride(value) {}

	get display() {
		switch (this.native_.display) {
			case NSCFontDisplay.Auto:
				return 'auto';
			case NSCFontDisplay.Block:
				return 'block';
			case NSCFontDisplay.Fallback:
				return 'fallback';
			case NSCFontDisplay.Optional:
				return 'optional';
			case NSCFontDisplay.Swap:
				return 'swap';
		}
	}

	set display(value: string) {
		this.native_.setFontDisplayWithValue(value);
	}

	get family() {
		return this.native_.family;
	}

	get status() {
		switch (this.native_.status) {
			case NSCFontFaceStatus.Loaded:
				return 'loaded';
			case NSCFontFaceStatus.Loading:
				return 'loading';
			case NSCFontFaceStatus.Unloaded:
				return 'unloaded';
		}
	}

	get style() {
		return this.native_.style;
	}

	get weight() {
		switch (this.native_.weight) {
			case NSCFontWeight.Thin:
				return 'thin';
			case NSCFontWeight.ExtraLight:
				return 'extra-light';
			case NSCFontWeight.Light:
				return 'light';
			case NSCFontWeight.Normal:
				return 'normal';
			case NSCFontWeight.Medium:
				return 'medium';
			case NSCFontWeight.SemiBold:
				return 'semi-bold';
			case NSCFontWeight.Bold:
				return 'bold';
			case NSCFontWeight.ExtraBold:
				return 'extra-bold';
			case NSCFontWeight.Black:
				return 'black';
		}
	}

	set weight(value: string) {
		this.native_.setFontWeightWithValue(value);
	}

	static fromNative(native: any): FontFace {
		if (native instanceof NSCFontFace) {
			const font = new (<any>FontFace)('', null, null, true);
			font.native_ = native;
			return font;
		}
		return null;
	}
}
