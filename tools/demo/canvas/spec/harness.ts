/**
 * Spec-test harness for the drawing contexts.
 *
 * Assertions are written against the HTML canvas spec (or, where it leaves
 * things open, against what Chrome does), so a failure means we drifted from
 * the web. One line per result so a run can be scraped off logcat:
 *
 *   SPEC|<suite>|<name>|PASS
 *   SPEC|<suite>|<name>|FAIL|<message>
 *   SPEC|summary|<passed>|<failed>|<total>
 *
 * Runner: `apps/demo/src/plugin-demos/canvas-spec.ts`.
 */

import { Canvas } from '@nativescript/canvas';

export interface SpecResult {
	suite: string;
	name: string;
	ok: boolean;
	message?: string;
}

export const results: SpecResult[] = [];

let currentSuite = '';

function report(name: string, ok: boolean, message?: string) {
	results.push({ suite: currentSuite, name, ok, message });
	if (ok) {
		console.log(`SPEC|${currentSuite}|${name}|PASS`);
	} else {
		console.log(`SPEC|${currentSuite}|${name}|FAIL|${message ?? ''}`);
	}
}

/** May return a promise. Anything thrown becomes one FAIL line, never a thrown run. */
export type TestBody = () => void | Promise<void>;

interface PendingTest {
	name: string;
	body: TestBody;
	skip?: string;
}

const pending: PendingTest[] = [];

export function suite(name: string, register: () => void) {
	const previous = currentSuite;
	currentSuite = name;
	// Registration is synchronous; the bodies run later, in `runAll`.
	const start = pending.length;
	register();
	for (let i = start; i < pending.length; i++) {
		(pending[i] as any).suite = name;
	}
	currentSuite = previous;
}

export function test(name: string, body: TestBody) {
	pending.push({ name, body });
}

/** Registered but not run -- records a PASS-shaped SKIP line with the reason. */
export function skip(name: string, reason: string) {
	pending.push({ name, body: () => {}, skip: reason });
}

/** `filter` is a suite-name prefix: `2d` runs every 2d.* suite, `2d.path2d` one. */
export async function runAll(filter?: string): Promise<{ passed: number; failed: number; total: number }> {
	let passed = 0;
	let failed = 0;

	const selected = filter && filter !== 'all' ? pending.filter((item) => ((item as any).suite ?? '').indexOf(filter) === 0) : pending;

	for (const item of selected) {
		currentSuite = (item as any).suite ?? '';
		if (item.skip) {
			results.push({ suite: currentSuite, name: item.name, ok: true, message: `skipped: ${item.skip}` });
			console.log(`SPEC|${currentSuite}|${item.name}|SKIP|${item.skip}`);
			continue;
		}
		// A native abort leaves no FAIL line, so name the test before running it.
		console.log(`SPEC|start|${currentSuite}|${item.name}`);
		try {
			await item.body();
			report(item.name, true);
			passed++;
		} catch (e) {
			report(item.name, false, describe(e));
			failed++;
		} finally {
			releaseCanvases();
		}
	}

	const total = passed + failed;
	console.log(`SPEC|summary|${passed}|${failed}|${total}`);
	for (const r of results) {
		if (!r.ok) {
			console.log(`SPEC|failure|${r.suite}|${r.name}|${r.message}`);
		}
	}
	pending.length = 0;
	return { passed, failed, total };
}

function describe(e: any): string {
	if (e == null) {
		return 'threw null';
	}
	if (typeof e === 'string') {
		return e;
	}
	const message = typeof e.message === 'string' ? e.message : String(e);
	return message.replace(/[\r\n]+/g, ' ');
}

// ------------------------------------------------------------------ assertions

export class AssertionError extends Error {}

function fail(message: string): never {
	throw new AssertionError(message);
}

export function ok(value: any, message = 'expected a truthy value') {
	if (!value) {
		fail(`${message} (got ${fmt(value)})`);
	}
}

export function equal(actual: any, expected: any, message = 'values differ') {
	// Object.is: NaN equals NaN, -0 does not equal 0.
	if (!Object.is(actual, expected)) {
		fail(`${message}: expected ${fmt(expected)}, got ${fmt(actual)}`);
	}
}

export function notEqual(actual: any, expected: any, message = 'values should differ') {
	if (Object.is(actual, expected)) {
		fail(`${message}: both are ${fmt(actual)}`);
	}
}

export function closeTo(actual: number, expected: number, epsilon = 1e-4, message = 'numbers differ') {
	if (typeof actual !== 'number' || !isFinite(actual) || Math.abs(actual - expected) > epsilon) {
		fail(`${message}: expected ${expected} +/- ${epsilon}, got ${fmt(actual)}`);
	}
}

/** For the places the spec allows latitude. */
export function oneOf(actual: any, expected: any[], message = 'value not in the allowed set') {
	for (const e of expected) {
		if (Object.is(actual, e)) {
			return;
		}
	}
	fail(`${message}: expected one of [${expected.map(fmt).join(', ')}], got ${fmt(actual)}`);
}

export function arrayEqual(actual: ArrayLike<number>, expected: ArrayLike<number>, message = 'arrays differ') {
	if (actual == null) {
		fail(`${message}: actual is ${fmt(actual)}`);
	}
	if (actual.length !== expected.length) {
		fail(`${message}: length ${actual.length} != ${expected.length}`);
	}
	for (let i = 0; i < expected.length; i++) {
		if (actual[i] !== expected[i]) {
			fail(`${message}: index ${i} is ${fmt(actual[i])}, expected ${fmt(expected[i])}`);
		}
	}
}

/**
 * `name` is the DOMException name the spec calls for; a plain Error whose message
 * mentions it also passes, since not every binding raises a real DOMException.
 */
export function throws(fn: () => void, name?: string, message = 'expected a throw') {
	let threw = false;
	let thrown: any;
	try {
		fn();
	} catch (e) {
		threw = true;
		thrown = e;
	}
	if (!threw) {
		fail(message);
	}
	if (name) {
		const actualName = thrown?.name ?? '';
		const text = `${actualName} ${thrown?.message ?? ''}`;
		if (actualName !== name && text.indexOf(name) === -1) {
			fail(`${message}: expected ${name}, got ${actualName}: ${thrown?.message}`);
		}
	}
	return thrown;
}

export async function rejects(promise: Promise<any>, message = 'expected a rejection') {
	try {
		await promise;
	} catch (e) {
		return e;
	}
	fail(message);
}

function fmt(value: any): string {
	if (typeof value === 'string') {
		return JSON.stringify(value);
	}
	if (value === undefined) {
		return 'undefined';
	}
	if (value === null) {
		return 'null';
	}
	if (typeof value === 'number') {
		return String(value);
	}
	if (ArrayBuffer.isView(value) || Array.isArray(value)) {
		const arr = Array.prototype.slice.call(value as any, 0, 8);
		return `[${arr.join(', ')}${(value as any).length > 8 ? ', …' : ''}]`;
	}
	try {
		return String(value);
	} catch (e) {
		return '<unprintable>';
	}
}

// ------------------------------------------------------------------- utilities

/**
 * The page's on-screen canvas. Only WebGPU needs it: a surface cannot be
 * configured against a canvas with no window.
 */
let pageCanvas: Canvas | null = null;

export function setPageCanvas(canvas: Canvas | null) {
	pageCanvas = canvas;
}

export function getPageCanvas(): Canvas | null {
	return pageCanvas;
}

/**
 * Released at the end of each test. Each canvas owns a GPU surface, and for a
 * WebGL test an EGL context; leaving a few hundred to the GC got the suite
 * killed by lowmemorykiller partway through the WebGL group.
 */
const liveCanvases: Canvas[] = [];

function releaseCanvases() {
	for (const canvas of liveCanvases) {
		try {
			(canvas as any).disposeNativeView();
		} catch (e) {
			// Never made it to a native view; nothing to release.
		}
	}
	liveCanvases.length = 0;
}

/**
 * Width and height are set back to back so the pair coalesces into one
 * synchronous setSurfaceSize -- see the width setter in Canvas/index.android.ts.
 */
export function makeCanvas(width = 100, height = 100): Canvas {
	const canvas = Canvas.createCustomView();
	(canvas as any).width = width;
	(canvas as any).height = height;
	liveCanvases.push(canvas);
	return canvas;
}

export function make2D(width = 100, height = 100, options?: any) {
	const canvas = makeCanvas(width, height);
	const ctx = canvas.getContext('2d', options) as any;
	if (!ctx) {
		fail('could not create a 2d context');
	}
	return { canvas, ctx };
}

export function pixelAt(ctx: any, x: number, y: number): [number, number, number, number] {
	const d = ctx.getImageData(x, y, 1, 1).data;
	return [d[0], d[1], d[2], d[3]];
}

/** Tolerant: a GPU surface will not reproduce a blend to the exact byte. */
export function pixelEqual(ctx: any, x: number, y: number, expected: [number, number, number, number], tolerance = 2, message = 'pixel differs') {
	const actual = pixelAt(ctx, x, y);
	for (let i = 0; i < 4; i++) {
		if (Math.abs(actual[i] - expected[i]) > tolerance) {
			fail(`${message} at (${x},${y}): expected [${expected.join(', ')}] +/-${tolerance}, got [${actual.join(', ')}]`);
		}
	}
}
