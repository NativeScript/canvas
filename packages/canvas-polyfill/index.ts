import {TNSXMLHttpRequest, FileReader, Blob} from './async/async';
import {Document} from './DOM/Document';
import './window';
import './resize';
import './process';
import {TextDecoder, TextEncoder} from '@nativescript/canvas';

(global as any).document = (global as any).window.document = (global as any).document || new Document();
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

