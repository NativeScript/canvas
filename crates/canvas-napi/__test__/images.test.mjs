// DOMMatrix, Path2D, CanvasPattern, TextEncoder/TextDecoder, ImageAsset, ImageBitmap and the
// module functions, called the way packages/canvas calls them.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { CanvasModule } from './addon.mjs';

const { CanvasRenderingContext2D, ImageData, Path2D, DOMMatrix, ImageAsset, ImageBitmap, TextEncoder, TextDecoder } = CanvasModule;

function context(width = 16, height = 16) {
	return CanvasRenderingContext2D.withCpu(width, height, 1, true, 0, 96, 0);
}

function pixel(ctx, x, y) {
	return Array.from(ctx.getImageData(x, y, 1, 1).data);
}

/** A `width`x`height` PNG of one colour. */
function png(width = 8, height = 4, color = '#ff0000') {
	const ctx = context(width, height);
	ctx.fillStyle = color;
	ctx.fillRect(0, 0, width, height);
	return new Uint8Array(Buffer.from(ctx.__toDataURL('image/png').split(',')[1], 'base64'));
}

/** Resolves with the callback's arguments. */
function callback(start) {
	return new Promise((resolve) => start((...args) => resolve(args)));
}

const close = (actual, expected) => assert.ok(Math.abs(actual - expected) < 1e-5, `${actual} != ${expected}`);

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'canvas-napi-'));
const pngPath = path.join(tmp, 'red.png');
fs.writeFileSync(pngPath, png());
process.on('exit', () => fs.rmSync(tmp, { recursive: true, force: true }));

// ---------------------------------------------------------------------------------------------

test('DOMMatrix: constructor forms and every accessor', () => {
	const identity = new DOMMatrix();
	assert.deepEqual(['a', 'b', 'c', 'd', 'e', 'f'].map((k) => identity[k]), [1, 0, 0, 1, 0, 0]);
	assert.equal(identity.m11, 1);
	assert.equal(identity.m44, 1);

	const affine = new DOMMatrix([1, 2, 3, 4, 5, 6]);
	assert.deepEqual(['a', 'b', 'c', 'd', 'e', 'f'].map((k) => affine[k]), [1, 2, 3, 4, 5, 6]);

	const names = ['m11', 'm12', 'm13', 'm14', 'm21', 'm22', 'm23', 'm24', 'm31', 'm32', 'm33', 'm34', 'm41', 'm42', 'm43', 'm44'];
	const full = new DOMMatrix(names.map((_, i) => i + 1));
	assert.deepEqual(
		names.map((k) => full[k]),
		names.map((_, i) => i + 1),
	);

	const m = new DOMMatrix();
	for (const [i, k] of [...'abcdef', ...names].entries()) {
		m[k] = i + 0.5;
		assert.equal(m[k], i + 0.5, k);
	}
});

test('DOMMatrix: methods, as packages/canvas calls them', () => {
	const m = new DOMMatrix();
	const moved = m.translate(10, 20, m);
	assert.ok(moved instanceof DOMMatrix);
	assert.deepEqual([moved.e, moved.f, m.e, m.f], [10, 20, 0, 0]);
	assert.equal(m.translate(1, 2).e, 1); // the source defaults to this matrix

	m.translateSelf(3, 4);
	assert.deepEqual([m.e, m.f], [3, 4]);

	const scaled = m.scaleNonUniform(2, 3, m);
	assert.deepEqual([scaled.a, scaled.d], [2, 3]);
	m.scaleNonUniformSelf(2, 2);
	assert.deepEqual([m.a, m.d], [2, 2]);

	const r = new DOMMatrix();
	const rotated = r.rotate(90, 0, 0, r);
	close(rotated.a, 0);
	close(rotated.b, 1);
	close(rotated.c, -1);
	r.rotateSelf(90); // cx, cy default to 0
	close(r.b, 1);

	const s = new DOMMatrix();
	close(s.skewX(45, s).c, 1);
	close(s.skewY(45, s).b, 1);
	s.skewXSelf(45);
	close(s.c, 1);
	s.skewYSelf(0);
});

test('DOMMatrix: multiplySelf / premultiplySelf take the native matrix or its wrapper', () => {
	const m = new DOMMatrix([2, 0, 0, 2, 0, 0]);
	m.multiplySelf(new DOMMatrix([1, 0, 0, 1, 5, 0]));
	assert.deepEqual([m.a, m.e], [2, 10]);

	const p = new DOMMatrix([2, 0, 0, 2, 0, 0]);
	p.premultiplySelf({ native: new DOMMatrix([1, 0, 0, 1, 5, 0]) }); // the TS DOMMatrix passes itself
	assert.deepEqual([p.a, p.e], [2, 5]);

	m.multiplySelf(m); // aliasing is fine
	assert.deepEqual([m.a, m.e], [4, 30]);
	m.multiplySelf({}); // non-matrices are ignored, as in the V8 bindings
	m.multiplySelf(null);
	assert.deepEqual([m.a, m.e], [4, 30]);
});

test('Path2D: constructors, addPath(path, matrix), roundRect forms', () => {
	const rect = new Path2D();
	rect.rect(0, 0, 4, 4);
	const copy = new Path2D(rect);
	assert.equal(copy.__toSVG(), rect.__toSVG());
	assert.match(new Path2D('M0 0 L10 10').__toSVG(), /M0 0L10 10/);

	const moved = new Path2D();
	moved.addPath(rect, new DOMMatrix([1, 0, 0, 1, 8, 8]));
	moved.addPath(rect, null);
	moved.addPath(moved); // itself
	const ctx = context();
	ctx.fillStyle = '#00ff00';
	ctx.fill(moved);
	assert.deepEqual(pixel(ctx, 9, 9), [0, 255, 0, 255]);
	assert.deepEqual(pixel(ctx, 1, 1), [0, 255, 0, 255]);
	assert.deepEqual(pixel(ctx, 6, 6), [0, 0, 0, 0]);

	const rounded = new Path2D();
	rounded.roundRect(0, 0, 10, 10, 2);
	rounded.roundRect(0, 0, 10, 10, [1, 2]);
	rounded.roundRect(0, 0, 10, 10, [1, 2, 3, 4]);
	rounded.roundRect(0, 0, 10, 10);
	assert.ok(rounded.__toSVG().length > 0);
});

test('CanvasPattern.setTransform(DOMMatrix)', () => {
	const source = context(4, 4);
	source.fillStyle = '#0000ff';
	source.fillRect(0, 0, 2, 2);
	const ctx = context();
	const pattern = ctx.createPattern(source, 'repeat');
	pattern.setTransform(new DOMMatrix([1, 0, 0, 1, 2, 2]));
	ctx.fillStyle = pattern;
	ctx.fillRect(0, 0, 16, 16);
	assert.deepEqual(pixel(ctx, 2, 2), [0, 0, 255, 255]);
	assert.deepEqual(pixel(ctx, 0, 0), [0, 0, 0, 0]);
	pattern.setTransform({}); // ignored
});

// ---------------------------------------------------------------------------------------------

test('TextEncoder', () => {
	const encoder = new TextEncoder('utf8');
	assert.equal(encoder.encoding, 'utf-8');
	const bytes = encoder.encode('héllo €');
	assert.ok(bytes instanceof Uint8Array);
	assert.deepEqual(Buffer.from(bytes), Buffer.from('héllo €'));
	assert.equal(encoder.encode().length, 0);
	assert.equal(new TextEncoder().encoding, 'utf-8');
});

test('TextDecoder: any buffer or view, read in place', async () => {
	const decoder = new TextDecoder('utf-8');
	assert.equal(decoder.encoding, 'utf-8');
	const bytes = new TextEncoder().encode('xxhéllo€yy');
	assert.equal(decoder.decode(bytes), 'xxhéllo€yy');
	assert.equal(decoder.decode(bytes.buffer), 'xxhéllo€yy');
	assert.equal(decoder.decode(bytes.subarray(2, bytes.length - 2)), 'héllo€');
	assert.equal(decoder.decode(new DataView(bytes.buffer, 2, bytes.length - 4)), 'héllo€');
	assert.equal(decoder.decode(new Uint16Array(new TextEncoder().encode('abcd').buffer)), 'abcd');
	assert.equal(decoder.decode(), '');
	assert.equal(decoder.decode(new Uint8Array(0)), '');
	assert.equal(decoder.decode(new Uint8Array([104, 0, 105])), 'h\0i');
	assert.throws(() => decoder.decode(42), /ArrayBuffer or ArrayBufferView/);

	assert.equal(await decoder.decodeAsync(bytes.subarray(2, bytes.length - 2)), 'héllo€');
	assert.equal(await decoder.decodeAsync(new Uint8Array(0)), '');
	await assert.rejects(decoder.decodeAsync(42), /ArrayBuffer or ArrayBufferView/);

	const utf16 = new TextDecoder('utf-16le');
	assert.equal(utf16.encoding, 'utf-16le');
	assert.equal(utf16.decode(new Uint8Array([104, 0, 105, 0])), 'hi');
});

// ---------------------------------------------------------------------------------------------

test('ImageAsset: sync loads', () => {
	const bytes = png();
	const asset = new ImageAsset();
	assert.equal(asset.fromEncodedBytesSync(bytes), true);
	assert.deepEqual([asset.width, asset.height], [8, 4]);
	assert.equal(new ImageAsset().fromEncodedBytesSync(bytes.buffer), true);
	assert.equal(new ImageAsset().fromEncodedBytesSync(new DataView(bytes.buffer)), true);

	const bad = new ImageAsset();
	assert.equal(bad.fromEncodedBytesSync(new Uint8Array([1, 2, 3])), false);
	assert.equal(typeof bad.error, 'string');

	const raw = new ImageAsset();
	assert.equal(raw.fromBytesSync(2, 2, new Uint8Array(16).fill(255)), true);
	assert.deepEqual([raw.width, raw.height], [2, 2]);
	assert.equal(new ImageAsset().fromBytesSync(2, 2, new Uint8ClampedArray(16).fill(128), true), true);

	const file = new ImageAsset();
	assert.equal(file.fromFileSync(pngPath), true);
	assert.equal(file.width, 8);
	assert.equal(new ImageAsset().fromFileSync(path.join(tmp, 'missing.png')), false);

	assert.match(asset.__getRef(), /^\d+$/);
	assert.equal(asset.__getRef(), asset.__addr);

	const ctx = context();
	ctx.drawImage(asset, 0, 0);
	assert.deepEqual(pixel(ctx, 1, 1), [255, 0, 0, 255]);
});

test('ImageAsset: *Cb loads call back once with (done)', async () => {
	const bytes = png();
	const encoded = new ImageAsset();
	let args = await callback((cb) => encoded.fromEncodedBytesCb(bytes, cb));
	assert.deepEqual(args, [true]);
	assert.equal(encoded.width, 8);

	args = await callback((cb) => new ImageAsset().fromEncodedBytesCb(new Uint8Array([1, 2, 3]), cb));
	assert.deepEqual(args, [false]);

	const raw = new ImageAsset();
	args = await callback((cb) => raw.fromBytesCb(2, 2, new Uint8Array(16).fill(255).buffer, cb));
	assert.deepEqual(args, [true]);
	assert.equal(raw.height, 2);

	const file = new ImageAsset();
	args = await callback((cb) => file.fromFileCb(pngPath, cb));
	assert.deepEqual(args, [true]);
	assert.equal(file.height, 4);
	args = await callback((cb) => new ImageAsset().fromFileCb(path.join(tmp, 'missing.png'), cb));
	assert.deepEqual(args, [false]);

	// Promise forms.
	assert.equal(await new ImageAsset().fromEncodedBytes(bytes), true);
	assert.equal(await new ImageAsset().fromFile(pngPath), true);
	assert.equal(await new ImageAsset().fromBytes(1, 1, new Uint8Array(4)), true);
});

// ---------------------------------------------------------------------------------------------

test('createImageBitmap: encoded bytes, all argument forms', async () => {
	const bytes = png();
	let [error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(bytes, cb));
	assert.equal(error, null);
	assert.ok(bitmap instanceof ImageBitmap);
	assert.deepEqual([bitmap.width, bitmap.height], [8, 4]);

	[error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(bytes.buffer, undefined, cb));
	assert.equal(error, null);
	assert.equal(bitmap.width, 8);

	[error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(bytes, 0, 0, 4, 2, cb));
	assert.equal(error, null);
	assert.deepEqual([bitmap.width, bitmap.height], [4, 2]);

	[error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(bytes, 0, 0, 4, 2, { resizeWidth: 2, resizeHeight: 1, resizeQuality: 'high', premultiplyAlpha: 'bogus' }, cb));
	assert.equal(error, null);
	assert.deepEqual([bitmap.width, bitmap.height], [2, 1]);

	[error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(bytes, { imageOrientation: 'flipY' }, cb));
	assert.equal(error, null);

	const ctx = context();
	ctx.drawImage(bitmap, 0, 0);
	assert.deepEqual(pixel(ctx, 1, 1), [255, 0, 0, 255]);
});

test('createImageBitmap: native sources', async () => {
	const asset = new ImageAsset();
	asset.fromEncodedBytesSync(png(6, 3, '#00ff00'));
	let [error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(asset, cb));
	assert.equal(error, null);
	assert.deepEqual([bitmap.width, bitmap.height], [6, 3]);

	let copy;
	[error, copy] = await callback((cb) => CanvasModule.createImageBitmap(bitmap, 0, 0, 2, 2, cb));
	assert.equal(error, null);
	assert.deepEqual([copy.width, copy.height], [2, 2]);

	const data = new ImageData(new Uint8ClampedArray(4 * 4 * 3).fill(255), 4, 3);
	[error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(data, null, cb));
	assert.equal(error, null);
	assert.deepEqual([bitmap.width, bitmap.height], [4, 3]);

	// A 2D context: snapshotted on this thread (pending drawing flushed first), reported synchronously.
	const source = context(5, 5);
	source.fillStyle = '#0000ff';
	source.fillRect(0, 0, 5, 5);
	let args;
	CanvasModule.createImageBitmap(source, (...a) => (args = a));
	assert.equal(args[0], null);
	assert.deepEqual([args[1].width, args[1].height], [5, 5]);
	const ctx = context();
	ctx.drawImage(args[1], 0, 0);
	assert.deepEqual(pixel(ctx, 2, 2), [0, 0, 255, 255]);
});

test('createImageBitmap: errors reach the callback as (message, null)', async () => {
	let [error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(new Uint8Array([1, 2, 3]), cb));
	assert.match(error, /could not be decoded/);
	assert.equal(bitmap, null);

	[error, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(png(), 0, 0, 0, 2, cb));
	assert.match(error, /crop rect width is 0/);
	[error] = await callback((cb) => CanvasModule.createImageBitmap(png(), 0, 0, 2, 0, undefined, cb));
	assert.match(error, /crop rect height is 0/);
	[error] = await callback((cb) => CanvasModule.createImageBitmap(null, cb));
	assert.equal(error, 'Failed to load image');
	[error] = await callback((cb) => CanvasModule.createImageBitmap({}, cb));
	assert.match(error, /could not be decoded/);
	[error] = await callback((cb) => CanvasModule.createImageBitmap(png(), 1, 2, cb));
	assert.match(error, /Invalid argument count/);

	assert.throws(() => CanvasModule.createImageBitmap(png()), /Illegal constructor/);
	assert.throws(() => CanvasModule.createImageBitmap(png(), {}), /Illegal constructor/);
});

test('ImageBitmap: close, fromAsset, __getRef', async () => {
	const asset = new ImageAsset();
	asset.fromEncodedBytesSync(png());
	const shared = ImageBitmap.fromAsset(asset);
	assert.ok(shared instanceof ImageBitmap);
	assert.deepEqual([shared.width, shared.height], [8, 4]);
	assert.equal(shared.__getRef(), asset.__getRef()); // the same image, not a copy
	assert.equal(ImageBitmap.fromAsset({}), null);

	const [, bitmap] = await callback((cb) => CanvasModule.createImageBitmap(png(), cb));
	assert.match(bitmap.__getRef(), /^\d+$/);
	assert.equal(bitmap.__addr, bitmap.__getRef());
	bitmap.close();
	assert.deepEqual([bitmap.width, bitmap.height], [0, 0]);
});

// ---------------------------------------------------------------------------------------------

test('readFile / getMime', async () => {
	const expected = fs.readFileSync(pngPath);
	let [error, result] = await callback((cb) => CanvasModule.readFile(pngPath, cb));
	assert.equal(error, null);
	assert.ok(result.buffer instanceof ArrayBuffer);
	assert.deepEqual(Buffer.from(result.buffer), expected);
	assert.equal(result.mime, 'image/png');
	assert.equal(result.extension, 'png');

	[error, result] = await callback((cb) => CanvasModule.readFile(path.join(tmp, 'missing.png'), cb));
	assert.ok(error instanceof Error);
	assert.equal(result, null);

	const text = path.join(tmp, 'empty.txt');
	fs.writeFileSync(text, '');
	[error, result] = await callback((cb) => CanvasModule.readFile(text, cb));
	assert.equal(error, null);
	assert.equal(result.buffer.byteLength, 0);

	[error, result] = await callback((cb) => CanvasModule.getMime(pngPath, cb));
	assert.equal(error, null);
	assert.ok(result instanceof ArrayBuffer);
	assert.equal(result.byteLength, expected.length);
});

test('__base64Encode / __base64Decode / __base64DecodeAsync', async () => {
	assert.equal(CanvasModule.__base64Encode('hello'), 'aGVsbG8=');
	assert.equal(CanvasModule.__base64Encode('é'), Buffer.from('é').toString('base64'));
	assert.equal(CanvasModule.__base64Encode(''), '');

	const [text, buffer] = CanvasModule.__base64Decode('aGVsbG8=');
	assert.equal(text, 'hello');
	assert.ok(buffer instanceof ArrayBuffer);
	assert.deepEqual(Buffer.from(buffer), Buffer.from('hello'));
	assert.equal(CanvasModule.__base64Decode(''), '');
	assert.equal(CanvasModule.__base64Decode('!!'), '');
	// One Latin-1 character per byte.
	assert.equal(CanvasModule.__base64Decode(Buffer.from([0xff, 0x00]).toString('base64'))[0], '\u00ff\u0000');

	const [asyncText, asyncBuffer] = await CanvasModule.__base64DecodeAsync('aGVsbG8=');
	assert.equal(asyncText, 'hello');
	assert.equal(asyncBuffer.byteLength, 5);
	assert.equal(await CanvasModule.__base64DecodeAsync('!!'), '');
});

test('__addFontFamily / __addFontData', () => {
	const fonts = process.platform === 'win32' ? 'C:\\Windows\\Fonts' : process.platform === 'darwin' ? '/System/Library/Fonts' : '/usr/share/fonts';
	const font = ['arial.ttf', 'segoeui.ttf', 'Helvetica.ttc'].map((f) => path.join(fonts, f)).find((f) => fs.existsSync(f));
	CanvasModule.__addFontFamily(null, []);
	CanvasModule.__addFontFamily('Nope', 'not-an-array');
	CanvasModule.__addFontData(null, 42);
	if (!font) return;
	CanvasModule.__addFontFamily(null, [font, 42]);
	CanvasModule.__addFontFamily('NapiAlias', [font]);
	const bytes = fs.readFileSync(font);
	CanvasModule.__addFontData('NapiData', new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength));
	const ctx = context();
	ctx.font = '12px NapiData';
	assert.ok(ctx.measureText('hello').width > 0);
});

test('async churn: many decodes in flight, buffers kept alive until done', async () => {
	const bytes = png(16, 16);
	const work = [];
	for (let i = 0; i < 64; i++) {
		work.push(callback((cb) => CanvasModule.createImageBitmap(bytes.slice(), cb)));
		work.push(callback((cb) => new ImageAsset().fromEncodedBytesCb(bytes.slice(), cb)));
		work.push(new TextDecoder().decodeAsync(new TextEncoder().encode(`text ${i}`)));
	}
	globalThis.gc?.();
	const results = await Promise.all(work);
	for (let i = 0; i < results.length; i += 3) {
		assert.equal(results[i][0], null);
		assert.equal(results[i][1].width, 16);
		assert.deepEqual(results[i + 1], [true]);
		assert.equal(results[i + 2], `text ${i / 3}`);
	}
	globalThis.gc?.();
});

test('ImageAsset: saveSync / saveCb write PNG and JPG that load back', async () => {
	const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'canvas-save-'));
	const asset = new ImageAsset();
	assert.equal(asset.fromBytesSync(2, 1, new Uint8Array([255, 0, 0, 255, 0, 0, 255, 255])), true);

	const png = path.join(dir, 'a.png');
	assert.equal(asset.saveSync(png, 1), true);
	assert.deepEqual([...fs.readFileSync(png).subarray(1, 4)].map((c) => String.fromCharCode(c)).join(''), 'PNG');
	const loaded = new ImageAsset();
	assert.equal(loaded.fromFileSync(png), true);
	assert.deepEqual([loaded.width, loaded.height], [2, 1]);

	const jpg = path.join(dir, 'a.jpg');
	const [success, error] = await new Promise((resolve) => asset.saveCb(jpg, 0, (...args) => resolve(args)));
	assert.equal(success, true, error);
	assert.deepEqual([...fs.readFileSync(jpg).subarray(0, 2)], [0xff, 0xd8]);

	// No encoder for TIFF; nothing to save in an empty asset.
	assert.equal(asset.saveSync(path.join(dir, 'a.tiff'), 4), false);
	const [emptySuccess, emptyError] = await new Promise((resolve) => new ImageAsset().saveCb(path.join(dir, 'b.png'), 1, (...args) => resolve(args)));
	assert.equal(emptySuccess, false);
	assert.match(emptyError, /No image/);
	fs.rmSync(dir, { recursive: true, force: true });
});
