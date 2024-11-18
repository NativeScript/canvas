import { Observable, Utils } from '@nativescript/core';
import { FontFace } from '.';
declare const kotlin;
export class FontFaceSet extends Observable {
	native_: org.nativescript.canvas.NSCFontFaceSet;
	constructor() {
		super();
		this.native_ = org.nativescript.canvas.NSCFontFaceSet.getInstance();
	}

	ready: Promise<void> = Promise.resolve();

	add(font: FontFace) {
		this.native_.add((font as any).native_);
	}

	*entries() {
		const iter = this.native_.getIter();
		let done = false;
		return {
			next() {
				const object = iter.next();
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
		const iter = this.native_.getIter();
		let done = false;
		return {
			next() {
				const object = iter.next();
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
		const iter = this.native_.getIter();
		let done = false;
		return {
			next() {
				const object = iter.next();
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
		const array = this.native_.getArray();
		const count = array.length;
		for (let i = 0; i < count; i++) {
			const item = array[i];
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

			const ref = new WeakRef(this);
			const cb = new kotlin.jvm.functions.Function2({
				invoke(fonts: java.util.List<org.nativescript.canvas.NSCFontFace>, error) {
					if (error) {
						const item = fonts.get(0);
						const font = (FontFace as any).fromNative(item);
						const owner = ref.get();
						if (owner) {
							owner.notify({ eventName: 'loadingerror', object: owner, font, text, fontfaces: [font] });
						}
						reject(error);
					} else {
						const count = fonts.size();
						const ret = new Array<FontFace>(count);
						for (let i = 0; i < count; i++) {
							const item = fonts.get(i);
							const font = (FontFace as any).fromNative(item);
							ret.push(font);
						}
						const owner = ref.get();
						if (owner) {
							owner.notify({ eventName: 'loadingdone', object: owner, fonts: ret, fontfaces: ret });
						}

						resolve(ret);
					}
				},
			});

			this.native_.load(Utils.android.getApplicationContext(), font, text, cb);
		});
	}
}
