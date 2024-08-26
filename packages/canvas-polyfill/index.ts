require('globals');

if (global.android && !(global as any).__canvasLoaded) {
	try {
		// load canvas lib if polyfill is called before
		java.lang.System.loadLibrary('canvasnative');
		(global as any).__canvasLoaded = true;
	} catch (e) {}
}
import { TNSXMLHttpRequest, FileReader, Blob } from './async/async';
import { Element } from './DOM/Element';
import { Document } from './DOM/Document';
import './window';
import './resize';
import './process';
import './localStorage';
import { TextDecoder, TextEncoder, ImageBitmap } from '@nativescript/canvas';
import './workerPatch';

if (!global.Document) {
	Object.defineProperty(global, 'Document', {
		value: Document,
		configurable: true,
		writable: true,
	});
}

(global as any).document = (global as any).window.document = (global as any).document || new Document();

(global as any).window.createImageBitmap = (global as any).createImageBitmap = (...args) => {
	const image = args[0];
	const sx_or_options = args[1];
	const sy = args[2];
	const sw = args[3];
	const sh = args[4];
	if ((typeof sw === 'number' && sw === 0) || (typeof sh === 'number' && sh === 0)) {
		return Promise.reject(new RangeError(`Failed to execute 'createImageBitmap' : The crop rect width is 0`));
	}

	if (args.length === 1) {
		//@ts-ignore
		return ImageBitmap.createFrom(image);
	} else if (args.length === 2) {
		//@ts-ignore
		return ImageBitmap.createFrom(image, sx_or_options);
	} else if (args.length === 5) {
		//@ts-ignore
		return ImageBitmap.createFromRect(image, sx_or_options, sy, sw, sh);
	} else if (args.length === 6) {
		//@ts-ignore
		return ImageBitmap.createFromRect(image, sx_or_options, sy, sw, sh, args[5]);
	}
};

if (!(global as any).Intl || (global as any).window.Intl) {
	(global as any).Intl = (global as any).window.Intl = (global as any).Intl || {}; // pixijs
}

import { MutationObserver } from './MutationObserver';

if (!global.MutationObserver) {
	Object.defineProperty(global, 'MutationObserver', {
		value: MutationObserver,
		configurable: true,
		writable: true,
	});
}

Object.defineProperty(global, 'Element', {
	value: Element,
	configurable: true,
	writable: true,
});
Object.defineProperty(global, 'XMLHttpRequest', {
	value: TNSXMLHttpRequest,
	configurable: true,
	writable: true,
});
Object.defineProperty(global, 'Blob', {
	value: Blob,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'FileReader', {
	value: FileReader,
	configurable: true,
	writable: true,
});

if (!((global as any).TextDecoder instanceof TextDecoder)) {
	Object.defineProperty(global, 'TextDecoder', {
		value: TextDecoder,
		configurable: true,
		writable: true,
	});
}

if (!((global as any).TextEncoder instanceof TextEncoder)) {
	Object.defineProperty(global, 'TextEncoder', {
		value: TextEncoder,
		configurable: true,
		writable: true,
	});
}

import './urlBlobPatch';

// require('./urlBlobPatch');
