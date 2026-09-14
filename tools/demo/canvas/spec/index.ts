/**
 * Web-spec conformance for the drawing contexts. One command:
 *
 *   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
 *     --es demo canvas-spec
 *   adb logcat -d | grep 'SPEC|'
 *
 * `--es suite <prefix>` narrows the run: a group (`2d`, `webgl`, `webgpu`,
 * `imagebitmap`, `bitmaprenderer`) or a single suite (`2d.path2d`).
 */

import { runAll, results, setPageCanvas } from './harness';
import { registerContext2DSpec } from './context2d';
import { registerImageBitmapSpec } from './imagebitmap';
import { registerWebGLSpec } from './webgl';
import { registerWebGPUSpec } from './webgpu';
import { registerCanvasSourceSpec } from './canvassource';
import { registerScalingSpec } from './scaling';

export type SpecGroup = string;

export async function runSpecTests(group: SpecGroup = 'all') {
	console.log(`SPEC|run|${group}`);
	const started = Date.now();

	// Registration only pushes closures; the filter decides what runs.
	registerContext2DSpec();
	registerImageBitmapSpec();
	registerWebGLSpec();
	registerWebGPUSpec();
	registerCanvasSourceSpec();
	registerScalingSpec();

	const summary = await runAll(group);
	console.log(`SPEC|done|${group}|${Date.now() - started}ms`);
	return summary;
}

export { results as specResults };
export { setPageCanvas as setSpecPageCanvas };
