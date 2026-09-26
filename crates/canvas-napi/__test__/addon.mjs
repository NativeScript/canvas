// Loads the canvas-napi addon for tests and benchmarks.
import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';

const here = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../..');

const names = { win32: 'canvas_napi.dll', darwin: 'libcanvas_napi.dylib', linux: 'libcanvas_napi.so' };
const file = names[process.platform];

const candidates = [
	process.env.CANVAS_NAPI_ADDON,
	path.join(root, 'target', 'debug', file),
	path.join(root, 'target', 'release-napi', file),
].filter(Boolean);

if (process.platform === 'win32' && !process.env.CANVAS_ANGLE_DIR) {
	const angle = path.join(root, '.angle-prebuilt', `angle-${process.arch}`, 'bin');
	if (fs.existsSync(angle)) process.env.CANVAS_ANGLE_DIR = angle;
}

let cached;

export function load() {
	if (cached) return cached;
	const found = candidates.find((f) => fs.existsSync(f));
	if (!found) throw new Error(`canvas-napi addon not built (looked in ${candidates.join(', ')})`);
	const module = { exports: {} };
	process.dlopen(module, found);
	cached = module.exports;
	return cached;
}

export const CanvasModule = load();
