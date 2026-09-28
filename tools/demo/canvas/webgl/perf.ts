/**
 * WebGL binding-layer benchmark.
 *
 * The 2D counterpart (`canvas2d/perf.ts`) showed that per-call overhead there
 * was already down to tens of nanoseconds; nothing equivalent existed for
 * WebGL, so "is the GL binding slow?" could only be answered by staring at a
 * whole demo. This measures the ops a real frame actually repeats.
 *
 * Every result is one `PERF|...` line so a run can be scraped off logcat:
 *
 *   PERF|<group>|<name>|<calls>|<best ms>|<ns per op>|<opt status>
 *
 * Groups:
 *   state  - calls that only set GL state. Closest to pure call overhead.
 *   upload - calls that push data across the boundary (uniforms, buffers).
 *   draw   - draw calls, deliberately on geometry small enough that the GPU is
 *            not the thing being timed.
 *   query  - calls that read back from GL. These cannot use the fast API
 *            because they return non-primitives, and some of them stall.
 *
 * Read `state` first: if a no-op state call is already expensive, everything
 * else inherits it.
 */

const MAGLEV = 1 << 5;
const TURBOFAN = 1 << 6;

let getOptimizationStatus: ((f: Function) => number) | null = null;
try {
	getOptimizationStatus = new Function('f', 'return %GetOptimizationStatus(f);') as (f: Function) => number;
	getOptimizationStatus(function () {});
} catch (e) {
	getOptimizationStatus = null;
}

function tier(fn: Function): string {
	if (!getOptimizationStatus) {
		return 'unknown';
	}
	try {
		const status = getOptimizationStatus(fn);
		if (status & TURBOFAN) {
			return 'turbofan';
		}
		if (status & MAGLEV) {
			return 'maglev';
		}
		return `tier:${status}`;
	} catch (e) {
		return 'unknown';
	}
}

interface BenchOptions {
	iters?: number;
	reps?: number;
	callsPerIter?: number;
}

const results: Array<{ group: string; name: string; nsPerOp: number }> = [];

function bench(group: string, name: string, fn: (n: number) => void, opts: BenchOptions = {}) {
	const iters = opts.iters ?? 50_000;
	const reps = opts.reps ?? 5;
	const callsPerIter = opts.callsPerIter ?? 1;

	// Warm into the optimizing tier: fast-API calls are only emitted from
	// optimized code, so timing before this measures the interpreter.
	fn(Math.min(iters, 20_000));
	fn(Math.min(iters, 20_000));

	let best = Infinity;
	for (let r = 0; r < reps; r++) {
		const start = performance.now();
		fn(iters);
		const elapsed = performance.now() - start;
		if (elapsed < best) {
			best = elapsed;
		}
	}

	const nsPerOp = (best * 1e6) / (iters * callsPerIter);
	results.push({ group, name, nsPerOp });
	console.log(`PERF|${group}|${name}|${iters * callsPerIter}|${best.toFixed(3)}|${nsPerOp.toFixed(1)}|${tier(fn)}`);
}

const VERTEX_SRC = `
attribute vec2 a_position;
uniform vec2 u_offset;
uniform mat4 u_matrix;
void main() {
  gl_Position = u_matrix * vec4(a_position + u_offset, 0.0, 1.0);
}`;

const FRAGMENT_SRC = `
precision mediump float;
uniform vec4 u_color;
void main() {
  gl_FragColor = u_color;
}`;

function compile(gl: any, type: number, src: string) {
	const shader = gl.createShader(type);
	gl.shaderSource(shader, src);
	gl.compileShader(shader);
	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		throw new Error('shader: ' + gl.getShaderInfoLog(shader));
	}
	return shader;
}

export function runWebGLPerf(canvas: any, version: 'webgl' | 'webgl2' = 'webgl') {
	const gl = canvas.getContext(version) as any;
	if (!gl) {
		console.log(`PERF|error|no ${version} context`);
		return;
	}

	console.log(`PERF|meta|context|${version}`);
	console.log(`PERF|meta|canvas|${canvas.width}x${canvas.height}`);
	console.log(`PERF|meta|renderer|${gl.getParameter(gl.RENDERER)}`);
	// Also a smoke test: the returned strings are owned by V8's external string
	// resource, and reading them back here catches a regression in that handoff.
	const extensions = gl.getSupportedExtensions() ?? [];
	console.log(`PERF|meta|extensions|${extensions.length}|${extensions.slice(0, 3).join(',')}`);
	console.log(`PERF|meta|natives|${getOptimizationStatus ? 'available' : 'unavailable'}`);
	console.log(`PERF|header|group|name|calls|bestMs|nsPerOp|tier`);

	const program = gl.createProgram();
	gl.attachShader(program, compile(gl, gl.VERTEX_SHADER, VERTEX_SRC));
	gl.attachShader(program, compile(gl, gl.FRAGMENT_SHADER, FRAGMENT_SRC));
	gl.linkProgram(program);
	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		throw new Error('link: ' + gl.getProgramInfoLog(program));
	}
	gl.useProgram(program);

	const aPosition = gl.getAttribLocation(program, 'a_position');
	const uOffset = gl.getUniformLocation(program, 'u_offset');
	const uColor = gl.getUniformLocation(program, 'u_color');
	const uMatrix = gl.getUniformLocation(program, 'u_matrix');

	// A single degenerate triangle: the draw benches are about the call, not
	// about filling pixels, so keep the rasteriser out of the measurement.
	const verts = new Float32Array([0, 0, 0.001, 0, 0, 0.001]);
	const buffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
	gl.bufferData(gl.ARRAY_BUFFER, verts, gl.STATIC_DRAW);
	gl.enableVertexAttribArray(aPosition);
	gl.vertexAttribPointer(aPosition, 2, gl.FLOAT, false, 0, 0);

	const indices = new Uint16Array([0, 1, 2]);
	const indexBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
	gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW);

	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D, texture);
	gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 2, 2, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array(16));

	const matrix = new Float32Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
	const color = new Float32Array([1, 0, 0, 1]);
	const sub = new Float32Array([0.5, 0.5, 0.5, 0.5, 0.5, 0.5]);

	// ------------------------------------------------------------------ state
	bench(
		'state',
		'enable/disable',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.enable(gl.BLEND);
				gl.disable(gl.BLEND);
			}
		},
		{ callsPerIter: 2 },
	);

	bench('state', 'activeTexture', (n) => {
		for (let i = 0; i < n; i++) {
			gl.activeTexture(gl.TEXTURE0);
		}
	});

	bench('state', 'bindTexture', (n) => {
		for (let i = 0; i < n; i++) {
			gl.bindTexture(gl.TEXTURE_2D, texture);
		}
	});

	bench('state', 'bindBuffer', (n) => {
		for (let i = 0; i < n; i++) {
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
		}
	});

	bench('state', 'useProgram', (n) => {
		for (let i = 0; i < n; i++) {
			gl.useProgram(program);
		}
	});

	bench('state', 'viewport', (n) => {
		for (let i = 0; i < n; i++) {
			gl.viewport(0, 0, 64, 64);
		}
	});

	bench('state', 'blendFunc', (n) => {
		for (let i = 0; i < n; i++) {
			gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
		}
	});

	bench('state', 'vertexAttribPointer', (n) => {
		for (let i = 0; i < n; i++) {
			gl.vertexAttribPointer(aPosition, 2, gl.FLOAT, false, 0, 0);
		}
	});

	// ----------------------------------------------------------------- upload
	bench('upload', 'uniform1f', (n) => {
		for (let i = 0; i < n; i++) {
			gl.uniform1f(uColor, 0.5);
		}
	});

	bench('upload', 'uniform2f', (n) => {
		for (let i = 0; i < n; i++) {
			gl.uniform2f(uOffset, 0.1, 0.2);
		}
	});

	bench('upload', 'uniform4fv', (n) => {
		for (let i = 0; i < n; i++) {
			gl.uniform4fv(uColor, color);
		}
	});

	bench('upload', 'uniformMatrix4fv', (n) => {
		for (let i = 0; i < n; i++) {
			gl.uniformMatrix4fv(uMatrix, false, matrix);
		}
	});

	bench(
		'upload',
		'bufferSubData (24B)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.bufferSubData(gl.ARRAY_BUFFER, 0, sub);
			}
		},
		{ iters: 20_000 },
	);

	// ------------------------------------------------------------------- draw
	gl.bindBuffer(gl.ARRAY_BUFFER, buffer);

	bench(
		'draw',
		'drawArrays (3 verts)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.drawArrays(gl.TRIANGLES, 0, 3);
			}
		},
		{ iters: 20_000 },
	);

	bench(
		'draw',
		'drawElements (3 idx)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.drawElements(gl.TRIANGLES, 3, gl.UNSIGNED_SHORT, 0);
			}
		},
		{ iters: 20_000 },
	);

	// What a sprite/particle renderer repeats: set a uniform, then draw.
	bench(
		'draw',
		'uniform4fv+drawArrays',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.uniform4fv(uColor, color);
				gl.drawArrays(gl.TRIANGLES, 0, 3);
			}
		},
		{ iters: 20_000, callsPerIter: 2 },
	);

	// ---------------------------------------------------------------- transfers
	// Bulk paths: a typed array crossing into GL. Anything here well above the
	// byte count divided by memory bandwidth is a copy we are making ourselves.
	const verts16k = new Float32Array(4096);
	const texels = new Uint8Array(64 * 64 * 4);

	bench(
		'upload',
		'bufferData (16KB)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.bufferData(gl.ARRAY_BUFFER, verts16k, gl.DYNAMIC_DRAW);
			}
		},
		{ iters: 5_000 },
	);

	bench(
		'upload',
		'bufferSubData (16KB)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.bufferSubData(gl.ARRAY_BUFFER, 0, verts16k);
			}
		},
		{ iters: 5_000 },
	);

	gl.bindTexture(gl.TEXTURE_2D, texture);
	gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 64, 64, 0, gl.RGBA, gl.UNSIGNED_BYTE, texels);

	bench(
		'upload',
		'texSubImage2D (64x64)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 64, 64, gl.RGBA, gl.UNSIGNED_BYTE, texels);
			}
		},
		{ iters: 2_000 },
	);

	bench(
		'upload',
		'texImage2D (64x64)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 64, 64, 0, gl.RGBA, gl.UNSIGNED_BYTE, texels);
			}
		},
		{ iters: 2_000 },
	);

	// The WebGL counterpart of getImageData: a GPU round trip, so expect it to
	// dwarf everything above.
	const readback = new Uint8Array(64 * 64 * 4);
	bench(
		'query',
		'readPixels (64x64)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.readPixels(0, 0, 64, 64, gl.RGBA, gl.UNSIGNED_BYTE, readback);
			}
		},
		{ iters: 300 },
	);

	// ------------------------------------------------------------------ query
	// Not on the fast API (non-primitive returns), and the first two are the
	// ones demo code tends to leave inside a frame loop by accident.
	bench(
		'query',
		'getError',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.getError();
			}
		},
		{ iters: 20_000 },
	);

	bench(
		'query',
		'getParameter (int)',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.getParameter(gl.MAX_TEXTURE_SIZE);
			}
		},
		{ iters: 10_000 },
	);

	bench(
		'query',
		'getUniformLocation',
		(n) => {
			for (let i = 0; i < n; i++) {
				gl.getUniformLocation(program, 'u_color');
			}
		},
		{ iters: 10_000 },
	);

	gl.finish();
	summary();
}

function summary() {
	console.log('PERF|--- webgl ns/op ---');
	const width = results.reduce((m, r) => Math.max(m, r.name.length), 0);
	for (const r of results) {
		console.log(`PERF|summary|${r.group.padEnd(6)} ${r.name.padEnd(width)}  ${r.nsPerOp.toFixed(0).padStart(7)} ns`);
	}
}
