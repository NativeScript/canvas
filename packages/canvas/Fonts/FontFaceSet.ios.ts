import { Observable } from '@nativescript/core';
import { FontFace } from '.';

export class FontFaceSet extends Observable {
	native_: NSCFontFaceSet;
	constructor() {
		super();
		this.native_ = NSCFontFaceSet.instance;
	}

	ready: Promise<void> = Promise.resolve();

	add(font: FontFace) {
		this.native_.add((font as any).native_);
	}

	*entries() {
		const iter: NSEnumerator<NSCFontFace> = this.native_.iter();
		let done = false;
		return {
			next() {
				const object = iter.nextObject();
				let value: [FontFace, FontFace] = null;
				if (object) {
					const font = (FontFace as any).fromNative(object);
					value = [font, font];
				} else {
					done = true;
				}
				return { value, done: done };
			},
		};
	}

	*keys() {
		const iter: NSEnumerator<NSCFontFace> = this.native_.iter();
		let done = false;
		return {
			next() {
				const object = iter.nextObject();
				let value: FontFace = null;
				if (object) {
					const font = (FontFace as any).fromNative(object);
					value = font;
				} else {
					done = true;
				}
				return { value, done: done };
			},
		};
	}

	*values() {
		const iter: NSEnumerator<NSCFontFace> = this.native_.iter();
		let done = false;
		return {
			next() {
				const object = iter.nextObject();
				let value: FontFace = null;
				if (object) {
					const font = (FontFace as any).fromNative(object);
					value = font;
				} else {
					done = true;
				}
				return { value, done: done };
			},
		};
	}

	forEach(callback: (value: FontFace, key: FontFace, parent: FontFaceSet) => void, thisArg?: any) {
		const array = this.native_.array();
		const count = array.count;
		for (let i = 0; i < count; i++) {
			const item = array.objectAtIndex(i);
			const font = (FontFace as any).fromNative(item);
			callback.call(thisArg, font, font, this);
		}
	}

	check(font: string, text?: string): boolean {
		return this.native_.check(font, text);
	}

	clear() {
		this.native_.clear();
	}

	delete(font: FontFace) {
		this.native_.delete((font as any).native_);
	}

	load(font: string, text?: string) {
		return new Promise<FontFace[]>((resolve, reject) => {
			this.notify({ eventName: 'loading', object: this, fontfaces: [] });
			this.native_.load(font, text, (fonts, error) => {
				if (error) {
					const item = fonts.objectAtIndex(0);
					const font = (FontFace as any).fromNative(item);
					this.notify({ eventName: 'loadingerror', object: this, font, text, fontfaces: [font] });
					reject(error);
				} else {
					const count = fonts.count;
					const ret = new Array<FontFace>(count);
					for (let i = 0; i < count; i++) {
						const item = fonts.objectAtIndex(i);
						const font = (FontFace as any).fromNative(item);
						ret.push(font);
					}
					this.notify({ eventName: 'loadingdone', object: this, fonts: ret, fontfaces: ret });
					resolve(ret);
				}
			});
		});
	}
}
