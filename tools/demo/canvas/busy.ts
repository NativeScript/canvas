/**
 * Frame pacing with canvases drawing next to animating views. A scenario is `name:count` parts,
 * e.g. `2d:2,views:24,js:4`:
 *   2d, webgl  canvases drawing a fixed scene each frame
 *   views      views spun by core's `view.animate`; js: views moved from the rAF loop
 *   only       2D scene part: `fill`, `stroke`, `text`, `trail` (never clears) or `video`
 *   rot        `rot:0` keeps the rects axis-aligned
 *   threaded   `1`/`0` passes `{ threaded }` to getContext; unset uses `Canvas.threaded2D`
 * Logs `BUSY|<scenario>|measuring`, `...|result|...` and `...|done`; see tools/scripts/busy-bench.sh.
 */
import { Application, ContentView, CoreTypes, GridLayout, ItemSpec, Label, StackLayout, View, WrapLayout } from '@nativescript/core';
import { Canvas } from '@nativescript/canvas';
import { createDemoVideo } from './webgl/video-frame';

const WARMUP_MS = 1500;
const MEASURE_MS = 8000;
const CANVAS_HEIGHT = 220;
const BOXES = 300;
const QUADS = 2000;
const DRAWS = 50;

export interface BusySpec {
	'2d': number;
	webgl: number;
	views: number;
	js: number;
	only: string;
	rot: number;
	threaded: number;
}

export const BUSY_PRESETS: Record<string, string> = {
	views: 'views:24,js:4',
	'2d': '2d:1',
	'2d+views': '2d:1,views:24,js:4',
	'2dx2+views': '2d:2,views:24,js:4',
	webgl: 'webgl:1',
	'webgl+views': 'webgl:1,views:24,js:4',
	'2d+webgl+views': '2d:1,webgl:1,views:24,js:4',
};

export function parseBusySpec(scenario: string): BusySpec {
	const spec: BusySpec = { '2d': 0, webgl: 0, views: 0, js: 0, only: '', rot: 1, threaded: -1 };
	for (const part of (BUSY_PRESETS[scenario] ?? scenario).split(',')) {
		const [name, value] = part.split(':');
		if (name === 'only') {
			spec.only = value ?? '';
		} else if (name in spec) {
			spec[name] = value === undefined ? 1 : parseInt(value, 10) || 0;
		}
	}
	return spec;
}

type Scene = (time: number) => void;

function refreshRate(): number {
	try {
		if (__ANDROID__) {
			return Application.android.foregroundActivity.getWindowManager().getDefaultDisplay().getRefreshRate();
		}
		if (__APPLE__) {
			return UIScreen.mainScreen.maximumFramesPerSecond;
		}
	} catch (e) {}
	return 60;
}

function percentile(sorted: number[], p: number): number {
	if (!sorted.length) {
		return 0;
	}
	return sorted[Math.min(sorted.length - 1, Math.max(0, Math.round((p / 100) * (sorted.length - 1))))];
}

function scene2D(canvas: Canvas, spec: BusySpec): Scene {
	const ctx = canvas.getContext('2d', spec.threaded < 0 ? undefined : { threaded: spec.threaded > 0 }) as any;
	const width = canvas.width as number;
	const height = canvas.height as number;
	const size = 22;
	const perRow = Math.max(1, Math.floor(width / size));
	const rects = new Array(BOXES).fill(0).map((_, i) => ({ x: 5 + (i % perRow) * size, y: 5 + Math.floor(i / perRow) * size }));
	if (spec.only === 'video') {
		const video = createDemoVideo();
		let n = 0;
		const probe = () => [0.25, 0.5, 0.75].map((f) => Array.from(ctx.getImageData(Math.round(width * f), Math.round(height / 2), 1, 1).data).join(',')).join(' ');
		return () => {
			ctx.drawImage(video, 0, 0, width, height);
			n++;
			if (n === 240 || n === 480) {
				console.log(`BUSY|video|probe|frame ${n}|${probe()}`);
			}
		};
	}
	if (spec.only === 'trail') {
		let n = 0;
		const at: Record<number, [number, number]> = {};
		return () => {
			const t = n++ / 60;
			const x = width * (0.1 + 0.8 * ((t / 8) % 1));
			const y = height / 2 + Math.sin(t * 3) * height * 0.35;
			at[n] = [x, y];
			if (n === 120) {
				for (const k of [1, 30, 100, 119]) {
					const px = ctx.getImageData(Math.round(at[k][0]), Math.round(at[k][1]), 1, 1).data;
					console.log(`BUSY|trail|probe|dot ${k} at frame 120|${Array.from(px).join(',')}`);
				}
			}
			ctx.fillStyle = `hsl(${(n * 3) % 360}, 90%, 50%)`;
			ctx.beginPath();
			ctx.arc(x, y, 6, 0, Math.PI * 2);
			ctx.fill();
		};
	}
	const fill = !spec.only || spec.only === 'fill';
	const stroke = !spec.only || spec.only === 'stroke';
	const text = !spec.only || spec.only === 'text';
	return (time) => {
		const cx = width / 2 + Math.cos(time / 700) * width * 0.3;
		const cy = height / 2 + Math.sin(time / 700) * height * 0.3;
		ctx.clearRect(0, 0, width, height);
		for (let i = 0; (fill || stroke) && i < rects.length; i++) {
			const r = rects[i];
			ctx.save();
			ctx.translate(r.x, r.y);
			if (spec.rot) {
				ctx.rotate(Math.atan2(cy - r.y, cx - r.x));
			}
			if (fill) {
				ctx.fillStyle = '#00ff00';
				ctx.fillRect(0, 0, size, size * 0.45);
			}
			if (stroke) {
				ctx.strokeStyle = '#4060A3';
				ctx.lineWidth = 2;
				ctx.strokeRect(0, 0, size, size * 0.45);
			}
			ctx.restore();
		}
		if (text) {
			ctx.fillStyle = '#000';
			ctx.font = '28px sans-serif';
			for (let i = 0; i < 5; i++) {
				ctx.fillText(`frame text line ${i}`, 10, 40 + i * 34);
			}
		}
	};
}

function sceneWebGL(canvas: Canvas): Scene {
	const gl = canvas.getContext('webgl') as WebGLRenderingContext;
	const vs = gl.createShader(gl.VERTEX_SHADER);
	gl.shaderSource(vs, 'attribute vec2 p; uniform float t; uniform vec2 o; void main(){ float c=cos(t),s=sin(t); gl_Position=vec4(mat2(c,-s,s,c)*p*0.5+o,0.,1.); }');
	gl.compileShader(vs);
	const fs = gl.createShader(gl.FRAGMENT_SHADER);
	gl.shaderSource(fs, 'precision mediump float; uniform vec3 col; void main(){ gl_FragColor=vec4(col,1.); }');
	gl.compileShader(fs);
	const program = gl.createProgram();
	gl.attachShader(program, vs);
	gl.attachShader(program, fs);
	gl.linkProgram(program);
	gl.useProgram(program);

	const data = new Float32Array(QUADS * 12);
	for (let q = 0; q < QUADS; q++) {
		const x = Math.random() * 2 - 1;
		const y = Math.random() * 2 - 1;
		const h = 0.02;
		data.set([x - h, y - h, x + h, y - h, x - h, y + h, x - h, y + h, x + h, y - h, x + h, y + h], q * 12);
	}
	const buffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
	gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW);
	const loc = gl.getAttribLocation(program, 'p');
	gl.enableVertexAttribArray(loc);
	gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);
	const uT = gl.getUniformLocation(program, 't');
	const uO = gl.getUniformLocation(program, 'o');
	const uCol = gl.getUniformLocation(program, 'col');
	const perDraw = Math.floor(QUADS / DRAWS) * 6;
	gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

	return (time) => {
		gl.clearColor(0.1, 0.1, 0.15, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);
		gl.uniform1f(uT, time / 1000);
		for (let d = 0; d < DRAWS; d++) {
			gl.uniform2f(uO, Math.sin(time / 900 + d) * 0.3, Math.cos(time / 1100 + d) * 0.3);
			gl.uniform3f(uCol, (d % 5) / 5, ((d * 3) % 7) / 7, 0.6);
			gl.drawArrays(gl.TRIANGLES, d * perDraw, perDraw);
		}
	};
}

function box(color: string): View {
	const v = new ContentView();
	v.width = 36;
	v.height = 36;
	v.margin = 4;
	v.backgroundColor = color;
	v.borderRadius = 6;
	return v;
}

function sized(canvas: Canvas): Promise<Canvas> {
	return new Promise((resolve) => {
		const done = () => {
			if (canvas.getMeasuredWidth() > 0) {
				canvas.off('layoutChanged', done);
				canvas.width = canvas.clientWidth * window.devicePixelRatio;
				canvas.height = canvas.clientHeight * window.devicePixelRatio;
				resolve(canvas);
			}
		};
		canvas.on('layoutChanged', done);
	});
}

export async function runBusyBench(host: GridLayout, scenario: string): Promise<void> {
	const spec = parseBusySpec(scenario);
	const tag = `BUSY|${scenario}`;
	console.log(`${tag}|spec|${JSON.stringify(spec)}`);

	host.removeChildren();
	host.addRow(new ItemSpec(1, 'auto'));
	host.addRow(new ItemSpec(1, 'star'));

	const canvasStack = new StackLayout();
	host.addChild(canvasStack);
	GridLayout.setRow(canvasStack, 0);

	const pending: Promise<{ canvas: Canvas; kind: '2d' | 'webgl' }>[] = [];
	for (const kind of ['2d', 'webgl'] as const) {
		for (let i = 0; i < spec[kind]; i++) {
			const canvas = new Canvas();
			canvas.setInlineStyle(`width: 100%; height: ${CANVAS_HEIGHT}; margin-bottom: 4;`);
			canvasStack.addChild(canvas);
			pending.push(sized(canvas).then((c) => ({ canvas: c, kind })));
		}
	}

	const others = new WrapLayout();
	host.addChild(others);
	GridLayout.setRow(others, 1);

	const colors = ['#e53935', '#8e24aa', '#1e88e5', '#43a047', '#fdd835', '#fb8c00'];
	for (let i = 0; i < spec.views; i++) {
		const v = box(colors[i % colors.length]);
		others.addChild(v);
		v.once('loaded', () => {
			v.animate({ rotate: 360, duration: 900 + (i % 5) * 150, iterations: Number.POSITIVE_INFINITY, curve: CoreTypes.AnimationCurve.linear }).catch(() => {});
		});
	}
	const jsViews: View[] = [];
	for (let i = 0; i < spec.js; i++) {
		const v = box('#546e7a');
		others.addChild(v);
		jsViews.push(v);
	}
	const counter = new Label();
	counter.margin = 4;
	if (spec.js > 0) {
		others.addChild(counter);
	}

	const canvases = await Promise.all(pending);
	const scenes = canvases.map(({ canvas, kind }) => (kind === '2d' ? scene2D(canvas, spec) : sceneWebGL(canvas)));

	const hz = refreshRate();
	const vsyncMs = 1000 / hz;
	const intervals: number[] = [];
	const drawMs: number[] = [];
	const startedAt = performance.now();
	let measuring = false;
	let last = 0;
	let frame = 0;

	return new Promise((resolve) => {
		const tick = () => {
			const now = performance.now();
			if (!measuring && now - startedAt >= WARMUP_MS) {
				measuring = true;
				console.log(`${tag}|measuring`);
			} else if (measuring) {
				intervals.push(now - last);
			}
			last = now;

			const drawStart = performance.now();
			for (let s = 0; s < scenes.length; s++) {
				scenes[s](now);
			}
			for (let j = 0; j < jsViews.length; j++) {
				jsViews[j].translateX = Math.sin(now / 300 + j) * 20;
			}
			if (spec.js > 0) {
				counter.text = `${frame}`;
			}
			frame++;
			if (measuring) {
				drawMs.push(performance.now() - drawStart);
			}

			if (measuring && now - startedAt >= WARMUP_MS + MEASURE_MS) {
				report();
				resolve();
				return;
			}
			requestAnimationFrame(tick);
		};

		const report = () => {
			const sorted = intervals.slice().sort((a, b) => a - b);
			const draws = drawMs.slice().sort((a, b) => a - b);
			const elapsed = intervals.reduce((s, v) => s + v, 0);
			// An interval of 2.1 vsyncs missed one.
			let missed = 0;
			let longFrames = 0;
			for (const v of intervals) {
				missed += Math.max(0, Math.round(v / vsyncMs) - 1);
				if (v > vsyncMs * 1.5) {
					longFrames++;
				}
			}
			const f = (n: number) => n.toFixed(2);
			console.log(
				`${tag}|result|hz|${f(hz)}|frames|${intervals.length}|fps|${f((intervals.length * 1000) / elapsed)}` +
					`|interval_p50|${f(percentile(sorted, 50))}|p95|${f(percentile(sorted, 95))}|p99|${f(percentile(sorted, 99))}|max|${f(percentile(sorted, 100))}` +
					`|long|${longFrames}|missed_vsyncs|${missed}|draw_p50|${f(percentile(draws, 50))}|draw_p95|${f(percentile(draws, 95))}`,
			);
			console.log(`${tag}|done`);
		};

		requestAnimationFrame(tick);
	});
}
