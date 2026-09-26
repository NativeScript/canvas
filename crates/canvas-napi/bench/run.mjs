// Per-call cost of the Node-API module's hot 2D and WebGL calls, measured on Node with the
// offscreen host. Scenario names match the demo's `native` group (tools/demo/canvas/canvas2d/perf.ts,
// webgl/perf.ts) so the numbers line up with the iOS/Android V8 baseline.
//
//   node --expose-gc bench/run.mjs [filter] [--save] [--json]
//
// Each scenario runs `reps` batches of `iters` calls after a warm-up; the result is ns per call,
// median and p95 over the batches. `--save` writes bench/results/<commit>.json for compare.mjs.
// CANVAS_BACKEND=cpu measures the CPU raster 2D context instead of D3D12 (Windows);
// CANVAS_FORCE_WARP=1 puts the D3D12 device on WARP; BENCH_LABEL suffixes the saved file;
// CANVAS_NAPI_FAST=0 measures napi-rs's own members instead of the raw fast ones (src/fast.rs).
import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';

const root = path.resolve(path.dirname(url.fileURLToPath(import.meta.url)), '../../..');
// Measure the optimized build unless told otherwise (the tests' loader prefers target/debug).
const release = path.join(root, 'target', 'release-napi', process.platform === 'win32' ? 'canvas_napi.dll' : 'libcanvas_napi.so');
if (!process.env.CANVAS_NAPI_ADDON && fs.existsSync(release)) {
	process.env.CANVAS_NAPI_ADDON = release;
}
const { CanvasModule } = await import('../__test__/addon.mjs');
console.log(`addon: ${process.env.CANVAS_NAPI_ADDON ?? 'target/debug (unoptimized: build release-napi for real numbers)'}`);

const here = path.dirname(url.fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const filter = args.find((a) => !a.startsWith('--')) ?? '';
const REPS = 15;

const results = [];

function bench(group, name, fn, { iters = 20_000, calls = 1 } = {}) {
	const label = `${group}.${name}`;
	if (filter && !label.includes(filter)) {
		return;
	}
	fn(Math.min(iters, 5_000));
	fn(Math.min(iters, 5_000));
	globalThis.gc?.();
	const samples = [];
	for (let r = 0; r < REPS; r++) {
		const start = process.hrtime.bigint();
		fn(iters);
		const elapsed = Number(process.hrtime.bigint() - start);
		samples.push(elapsed / (iters * calls));
	}
	samples.sort((a, b) => a - b);
	const median = samples[Math.floor(samples.length / 2)];
	const p95 = samples[Math.min(samples.length - 1, Math.round(samples.length * 0.95))];
	results.push({ group, name, median, p95 });
	if (!args.includes('--json')) {
		console.log(`${label.padEnd(44)} ${median.toFixed(1).padStart(9)} ns ${`p95 ${p95.toFixed(1)}`.padStart(14)}`);
	}
}

function context2D(width = 512, height = 512) {
	if (process.platform === 'win32' && process.env.CANVAS_BACKEND !== 'cpu') {
		const host = new CanvasModule.NSCCanvas();
		host.setSurfaceSize(width, height);
		const pointer = host.create2DContext(true, true, false, false, 0, true, false, false, false, false, 0, false, 0);
		return { host, ctx: CanvasModule.create2DContextWithPointer(BigInt(pointer)) };
	}
	return { ctx: CanvasModule.CanvasRenderingContext2D.withCpu(width, height, 1, true, 0, 96, 0) };
}

// ------------------------------------------------------------------------------------------ 2D
{
	const { ctx, host } = context2D();
	let sink = 0;

	bench('2d', 'save+restore', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.save();
			ctx.restore();
		}
	}, { calls: 2 });
	bench('2d', 'translate', (n) => {
		for (let i = 0; i < n; i++) ctx.translate(0.5, 0.5);
		ctx.resetTransform();
	});
	bench('2d', 'rotate', (n) => {
		for (let i = 0; i < n; i++) ctx.rotate(0.001);
		ctx.resetTransform();
	});
	bench('2d', 'setTransform', (n) => {
		for (let i = 0; i < n; i++) ctx.setTransform(1, 0, 0, 1, i & 7, 0);
		ctx.resetTransform();
	});
	bench('2d', 'beginPath', (n) => {
		for (let i = 0; i < n; i++) ctx.beginPath();
	});
	bench('2d', 'lineTo', (n) => {
		ctx.beginPath();
		ctx.moveTo(0, 0);
		for (let i = 0; i < n; i++) ctx.lineTo(i & 511, (i * 7) & 511);
		ctx.beginPath();
	});
	bench('2d', 'arc', (n) => {
		ctx.beginPath();
		for (let i = 0; i < n; i++) ctx.arc(256, 256, 10, 0, 6.283);
		ctx.beginPath();
	});
	bench('2d', 'fillRect', (n) => {
		for (let i = 0; i < n; i++) ctx.fillRect(i & 255, i & 127, 16, 16);
	});
	bench('2d', 'strokeRect', (n) => {
		for (let i = 0; i < n; i++) ctx.strokeRect(i & 255, i & 127, 16, 16);
	});
	bench('2d', 'clearRect', (n) => {
		for (let i = 0; i < n; i++) ctx.clearRect(i & 255, i & 127, 16, 16);
	});
	bench('2d', 'fillStyle (string)', (n) => {
		for (let i = 0; i < n; i++) ctx.fillStyle = i & 1 ? '#00ff00' : '#ff0000';
	});
	bench('2d', 'fillStyle (same string)', (n) => {
		for (let i = 0; i < n; i++) ctx.fillStyle = '#336699';
	});
	bench('2d', 'fillStyle (rgba string)', (n) => {
		for (let i = 0; i < n; i++) ctx.fillStyle = i & 1 ? 'rgba(10, 20, 30, 0.5)' : 'rgba(30, 20, 10, 0.25)';
	});
	bench('2d', 'fillStyle (get)', (n) => {
		for (let i = 0; i < n; i++) sink += ctx.fillStyle.length;
	});
	bench('2d', 'strokeStyle (string)', (n) => {
		for (let i = 0; i < n; i++) ctx.strokeStyle = i & 1 ? '#4060A3' : '#A36040';
	});
	bench('2d', 'lineWidth (number)', (n) => {
		for (let i = 0; i < n; i++) ctx.lineWidth = (i & 3) + 1;
	});
	bench('2d', 'globalAlpha (number)', (n) => {
		for (let i = 0; i < n; i++) ctx.globalAlpha = (i & 1) * 0.5 + 0.5;
		ctx.globalAlpha = 1;
	});
	bench('2d', 'font (string)', (n) => {
		for (let i = 0; i < n; i++) ctx.font = i & 1 ? '16px sans-serif' : 'bold 12px serif';
	}, { iters: 5_000 });
	bench('2d', 'fillText', (n) => {
		ctx.font = '16px sans-serif';
		for (let i = 0; i < n; i++) ctx.fillText('Score: 1234', i & 255, 64);
	}, { iters: 5_000 });
	bench('2d', 'measureText', (n) => {
		for (let i = 0; i < n; i++) sink += ctx.measureText('Score: 1234').width;
	}, { iters: 5_000 });

	const image = CanvasModule.CanvasRenderingContext2D.withCpu(64, 64, 1, true, 0, 96, 0);
	image.fillRect(0, 0, 64, 64);
	const asset = new CanvasModule.ImageAsset();
	asset.fromBytesSync(64, 64, new Uint8Array(64 * 64 * 4).fill(200));
	bench('2d', 'drawImage (asset, x, y)', (n) => {
		for (let i = 0; i < n; i++) ctx.drawImage(asset, i & 255, i & 127);
	}, { iters: 5_000 });
	bench('2d', 'drawImage (asset, 9 args)', (n) => {
		for (let i = 0; i < n; i++) ctx.drawImage(asset, 0, 0, 32, 32, i & 255, i & 127, 16, 16);
	}, { iters: 5_000 });
	bench('2d', 'drawImage (canvas)', (n) => {
		for (let i = 0; i < n; i++) ctx.drawImage(image, i & 255, i & 127);
	}, { iters: 2_000 });
	bench('2d', 'getImageData 16x16', (n) => {
		for (let i = 0; i < n; i++) sink += ctx.getImageData(i & 127, 0, 16, 16).width;
	}, { iters: 500 });
	const data = ctx.createImageData(16, 16);
	bench('2d', 'putImageData 16x16', (n) => {
		for (let i = 0; i < n; i++) ctx.putImageData(data, i & 127, 0);
	}, { iters: 2_000 });
	bench('2d', 'flush (__flushAll)', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.fillRect(0, 0, 1, 1);
			CanvasModule.__flushAll();
		}
	}, { iters: 200 });
	void host;
	void sink;
}

// --------------------------------------------------------------------------------------- WebGL
{
	const gl = CanvasModule.createWebGLContext({ version: 1 }, 256, 256);
	if (gl) {
		const vs = gl.createShader(0x8b31);
		gl.shaderSource(vs, 'attribute vec2 p; uniform vec4 u; uniform mat4 m; void main(){ gl_Position = m * vec4(p, 0.0, 1.0) + u * 0.0; }');
		gl.compileShader(vs);
		const fs = gl.createShader(0x8b30);
		gl.shaderSource(fs, 'precision highp float; uniform vec4 u; void main(){ gl_FragColor = u; }');
		gl.compileShader(fs);
		const program = gl.createProgram();
		gl.attachShader(program, vs);
		gl.attachShader(program, fs);
		gl.linkProgram(program);
		gl.useProgram(program);
		const u = gl.getUniformLocation(program, 'u');
		const m = gl.getUniformLocation(program, 'm');
		const buffer = gl.createBuffer();
		gl.bindBuffer(0x8892, buffer);
		const vertices = new Float32Array([-1, -1, 1, -1, 0, 1]);
		gl.bufferData(0x8892, vertices, 0x88e8);
		gl.enableVertexAttribArray(0);
		gl.vertexAttribPointer(0, 2, 0x1406, false, 0, 0);
		const matrix = new Float32Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);

		bench('webgl', 'uniform4f', (n) => {
			for (let i = 0; i < n; i++) gl.uniform4f(u, 1, 0, 0, 1);
		});
		bench('webgl', 'uniformMatrix4fv', (n) => {
			for (let i = 0; i < n; i++) gl.uniformMatrix4fv(m, false, matrix);
		});
		bench('webgl', 'bindBuffer', (n) => {
			for (let i = 0; i < n; i++) gl.bindBuffer(0x8892, buffer);
		});
		bench('webgl', 'useProgram', (n) => {
			for (let i = 0; i < n; i++) gl.useProgram(program);
		});
		bench('webgl', 'vertexAttribPointer', (n) => {
			for (let i = 0; i < n; i++) gl.vertexAttribPointer(0, 2, 0x1406, false, 0, 0);
		});
		bench('webgl', 'bufferSubData (24 B)', (n) => {
			for (let i = 0; i < n; i++) gl.bufferSubData(0x8892, 0, vertices);
		});
		bench('webgl', 'drawArrays', (n) => {
			for (let i = 0; i < n; i++) gl.drawArrays(0x0004, 0, 3);
		}, { iters: 5_000 });
		bench('webgl', 'getError', (n) => {
			for (let i = 0; i < n; i++) gl.getError();
		});
	}
}

if (args.includes('--json')) {
	console.log(JSON.stringify(results, null, 2));
}
if (args.includes('--save')) {
	let commit = 'working';
	try {
		commit = execSync('git rev-parse --short HEAD', { cwd: here }).toString().trim();
	} catch {}
	const dir = path.join(here, 'results');
	fs.mkdirSync(dir, { recursive: true });
	const label = [process.env.CANVAS_BACKEND, process.env.BENCH_LABEL].filter(Boolean).map((part) => `-${part}`).join('');
	const file = path.join(dir, `${commit}${label}.json`);
	fs.writeFileSync(file, JSON.stringify({ commit, platform: process.platform, arch: process.arch, node: process.version, results }, null, 2));
	console.log(`saved ${path.relative(process.cwd(), file)}`);
}
