import { EventData, Observable, Page } from '@nativescript/core';
import { runCanvasPerf, runWebGLPerf, runImageBitmapPerf, profileDemo, swarm, touchParticles, runBoundsProbe, runCallBound, runWebGPUSample } from '@demo/shared';
import { launchArgs } from '../launch-args';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new PerfModel();
}

/**
 * Demos that can be frame-profiled:
 *   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
 *     --es demo canvas-perf --es profile swarm
 *
 * The WebGL binding benchmark runs from the same page -- a canvas can only hand
 * out one kind of context, so which suite runs is a launch argument rather than
 * a second page:
 *   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
 *     --es demo canvas-perf --es suite webgl
 *
 * `--es suite cpu2d` runs the 2D suite against a willReadFrequently (CPU raster)
 * canvas, which is the comparison that makes the getImageData numbers readable.
 *
 * `--es suite imagebitmap` runs the createImageBitmap benchmark (IMGBM| lines),
 * which builds its own canvases and ignores the one on this page.
 *
 * `--es suite bounds` checks that getBoundingClientRect is really wired to the
 * native view on Android (BOUNDS| lines).
 *
 * `--es suite webgpu:<sample>` runs one of the WebGPU samples (`WEBGPU_SAMPLES`,
 * e.g. `webgpu:rotatingCube`) on this page's canvas.
 */
const PROFILABLE: Record<string, (canvas: any) => void> = {
	swarm: (canvas) => swarm(canvas),
	particles: (canvas) => touchParticles(canvas),
};

class PerfModel extends Observable {
	private ran = false;

	constructor() {
		super();
		this.set('status', 'waiting for canvas…');
	}

	canvasReady(args) {
		if (this.ran) {
			return;
		}
		this.ran = true;

		const canvas = args.object;
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;

		const profile = launchArgs.profile;
		const suite = launchArgs.suite;
		this.set('status', profile ? `profiling ${profile}…` : `running ${suite ?? '2d'}…`);

		// Let the first frame settle before timing anything.
		setTimeout(() => {
			try {
				if (profile) {
					const start = PROFILABLE[profile];
					if (!start) {
						console.log(`PROF|error|unknown demo ${profile}`);
						this.set('status', `unknown demo: ${profile}`);
						return;
					}
					profileDemo(canvas, start, profile, launchArgs.frames ?? 180, () => {
						this.set('status', `${profile} profiled — see logcat (PROF|…)`);
					});
				} else if (suite?.startsWith('webgpu:')) {
					const sample = suite.substring('webgpu:'.length);
					const started = runWebGPUSample(sample, canvas);
					console.log(started ? `WEBGPU|started|${sample}` : `WEBGPU|error|unknown sample ${sample}`);
					this.set('status', started ? sample : `unknown sample: ${sample}`);
				} else if (suite === 'alpha' || suite === 'alpha-webgl') {
					// A cleared canvas shows what is behind it (magenta here), as on the web.
					canvas.parent.backgroundColor = 'magenta';
					const w = canvas.width;
					const h = canvas.height;
					if (suite === 'alpha') {
						const ctx = canvas.getContext('2d');
						ctx.clearRect(0, 0, w, h);
						ctx.fillStyle = 'red';
						ctx.fillRect(w / 4, h / 4, w / 2, h / 2);
					} else {
						const gl = canvas.getContext('webgl');
						gl.clearColor(0, 0, 0, 0);
						gl.clear(gl.COLOR_BUFFER_BIT);
						gl.enable(gl.SCISSOR_TEST);
						gl.scissor(w / 4, h / 4, w / 2, h / 2);
						gl.clearColor(1, 0, 0, 1);
						gl.clear(gl.COLOR_BUFFER_BIT);
					}
					this.set('status', `${suite}: magenta around a red square`);
				} else if (suite === 'callbound') {
					runCallBound(canvas);
					this.set('status', 'done — see logcat (NVCALL|…)');
				} else if (suite === 'bounds') {
					runBoundsProbe(canvas);
					this.set('status', 'done — see logcat (BOUNDS|…)');
				} else if (suite === 'imagebitmap') {
					runImageBitmapPerf().then(() => {
						this.set('status', 'done — see logcat (IMGBM|…)');
					});
				} else if (suite === 'webgl' || suite === 'webgl2') {
					runWebGLPerf(canvas, suite);
					this.set('status', `${suite} done — see logcat (PERF|…)`);
				} else if (suite === 'cpu2d') {
					runCanvasPerf(canvas, true);
					this.set('status', 'done (willReadFrequently) — see logcat (PERF|…)');
				} else {
					runCanvasPerf(canvas);
					this.set('status', 'done — see logcat (PERF|…)');
				}
			} catch (e) {
				console.log('PERF|error|' + (e?.message ?? e));
				console.log(e?.stack ?? '');
				this.set('status', 'failed: ' + (e?.message ?? e));
			}
		}, 500);
	}
}
