// global.SVGModule from the Node-API addon, called the way packages/canvas-svg's NativeNode.ts and
// canvas-image.ts call it.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import url from 'node:url';

const here = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../..');
const file = { win32: 'canvas_svg_napi.dll', darwin: 'libcanvas_svg_napi.dylib', linux: 'libcanvas_svg_napi.so' }[process.platform];
const addon = [process.env.CANVAS_SVG_NAPI_ADDON, path.join(root, 'target', 'debug', file), path.join(root, 'target', 'release-napi', file)].filter(Boolean).find((f) => fs.existsSync(f));
const module = { exports: {} };
process.dlopen(module, addon);
const SVGModule = globalThis.SVGModule;

const RED_SQUARE = '<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10"><rect id="r" width="10" height="10" fill="red"/><rect x="10" width="10" height="10" fill="blue"/></svg>';

function render(document, width, height, scale = 1, bgra = false) {
	const pixels = new Uint8Array(width * height * 4);
	document.setContainerSize(width / scale, height / scale);
	document.renderToBuffer(pixels, width, height, scale, bgra);
	return (x, y) => Array.from(pixels.subarray((y * width + x) * 4, (y * width + x) * 4 + 4));
}

test('installs global.SVGModule with the V8 bindings\' members', () => {
	assert.equal(SVGModule, module.exports);
	for (const name of ['SVGDocument', 'SVGNode', 'createSVGDocument', 'createElement', 'createTextNode']) {
		assert.equal(typeof SVGModule[name], 'function', name);
	}
	const document = SVGModule.createSVGDocument();
	for (const name of ['root', 'createElement', 'createTextNode', 'getElementById', 'registerId', 'unregisterId', 'setContainerSize', 'setLayer', 'invalidateBackdrop', 'setFrameSharing', 'invalidateFrames', 'nativePointer', 'renderToBuffer', 'hasAnimations', 'addStylesheet', 'animationDuration', 'currentTime', 'setCurrentTime']) {
		assert.equal(typeof document[name], 'function', `SVGDocument.${name}`);
	}
	for (const name of ['tagName', 'setAttribute', 'getAttribute', 'appendChild', 'removeChild', 'text', 'setText']) {
		assert.equal(typeof document.root()[name], 'function', `SVGNode.${name}`);
	}
});

test('parses a document and renders it as RGBA, or BGRA on request', () => {
	const document = SVGModule.createSVGDocument(RED_SQUARE);
	assert.equal(document.root().tagName(), 'svg');
	assert.ok(document.nativePointer() > 0);
	const rgba = render(document, 20, 10);
	assert.deepEqual(rgba(5, 5), [255, 0, 0, 255]);
	assert.deepEqual(rgba(15, 5), [0, 0, 255, 255]);
	const bgra = render(document, 20, 10, 1, true);
	assert.deepEqual(bgra(5, 5), [0, 0, 255, 255]);
	// Scaled 2x into 40x20 pixels.
	assert.deepEqual(render(document, 40, 20, 2)(30, 10), [0, 0, 255, 255]);
});

test('unparseable sources: createSVGDocument returns null, the constructor throws', () => {
	assert.equal(SVGModule.createSVGDocument('<not svg'), null);
	assert.throws(() => new SVGModule.SVGDocument('<not svg'));
	assert.ok(new SVGModule.SVGDocument(RED_SQUARE).root());
});

test('builds a tree with createElement / setAttribute / appendChild / removeChild', () => {
	const document = SVGModule.createSVGDocument('<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"/>');
	const rect = SVGModule.createElement('rect');
	assert.equal(rect.setAttribute('width', 10), true);
	rect.setAttribute('height', '10');
	rect.setAttribute('fill', '#00ff00');
	assert.equal(rect.getAttribute('fill'), '#00ff00');
	assert.equal(rect.getAttribute('missing'), null);
	// One handle per node, as NativeNode.ts keeps: a handle tracks the children added through it.
	const root = document.root();
	assert.equal(root.appendChild(rect), true);
	assert.deepEqual(render(document, 10, 10)(5, 5), [0, 255, 0, 255]);

	assert.equal(root.appendChild({}), false);
	const removed = root.removeChild(0);
	assert.equal(removed.tagName(), 'rect');
	assert.deepEqual(render(document, 10, 10)(5, 5), [0, 0, 0, 0]);
	assert.equal(root.removeChild(5), null);
});

test('ids, text nodes and layers', () => {
	const document = SVGModule.createSVGDocument(RED_SQUARE);
	// Paints come back normalised.
	assert.equal(document.getElementById('r').getAttribute('fill'), '#ff0000');
	assert.equal(document.getElementById('nope'), null);

	const circle = document.createElement('circle');
	document.registerId('c', circle);
	assert.equal(document.getElementById('c').tagName(), 'circle');
	document.unregisterId('c');
	assert.equal(document.getElementById('c'), null);

	const text = SVGModule.createTextNode('hello');
	assert.equal(text.text(), 'hello');
	assert.equal(text.setText('world'), true);
	assert.equal(text.text(), 'world');
	assert.equal(circle.text(), null);

	document.setLayer('r');
	document.setLayer(null);
	document.invalidateBackdrop();
	document.setFrameSharing(true);
	document.invalidateFrames();
});

test('SMIL animations drive setCurrentTime', () => {
	const document = SVGModule.createSVGDocument(
		'<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10" fill="red"><animate attributeName="fill" from="red" to="blue" dur="1s" fill="freeze"/></rect></svg>',
	);
	assert.equal(document.hasAnimations(), true);
	assert.equal(document.animationDuration(), 1);
	const state = document.setCurrentTime(2);
	assert.equal(state & 1, 0, 'finished');
	assert.equal(document.currentTime(), 2);
	assert.deepEqual(render(document, 10, 10)(5, 5), [0, 0, 255, 255]);
	assert.equal(SVGModule.createSVGDocument(RED_SQUARE).hasAnimations(), false);
});
