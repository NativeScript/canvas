import { Observable } from '@nativescript/core';
import { FontFace } from '.';

export class FontFaceSet extends Observable {
	ready: Promise<void>;

	add(font: FontFace);

	entries(): IterableIterator<[FontFace, FontFace]>;

	keys(): IterableIterator<FontFace>;

	values(): IterableIterator<FontFace>;

	forEach(callback: (value: FontFace, key: FontFace, parent: FontFaceSet) => void, thisArg?: any);

	check(font: string, text?: string): boolean;

	clear();

	delete(font: FontFace);

	load(font: string, text?: string): Promise<FontFace[]>;
}
