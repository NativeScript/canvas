import assert from 'node:assert/strict';
import test from 'node:test';

import { CanvasModule } from './addon.mjs';

test('installs globalThis.CanvasModule', () => {
	assert.equal(globalThis.CanvasModule, CanvasModule);
});

test('Path2D builds paths and coerces arguments like the V8 bindings', () => {
	const p = new CanvasModule.Path2D();
	p.moveTo(0, 0);
	p.lineTo('10', 10);
	assert.equal(p.__toSVG(), 'M0 0L10 10');
});

test('Path2D copies, parses SVG and adds paths with a transform', () => {
	const svg = new CanvasModule.Path2D('M0 0 L10 10');
	const copy = new CanvasModule.Path2D(svg);
	assert.equal(copy.__toSVG(), svg.__toSVG());
	copy.addPath(new CanvasModule.Path2D('M1 1 L2 2'), new CanvasModule.DOMMatrix());
	assert.match(copy.__toSVG(), /M1 1L2 2$/);
});

test('Path2D.roundRect accepts a radius or a list of radii', () => {
	const p = new CanvasModule.Path2D();
	p.roundRect(0, 0, 10, 10, 2);
	p.roundRect(0, 0, 10, 10, [1, 2]);
	assert.ok(p.__toSVG().length > 0);
});

test('methods reject foreign receivers', () => {
	assert.throws(() => CanvasModule.Path2D.prototype.lineTo.call({}, 1, 2), TypeError);
});
