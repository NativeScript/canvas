/**
 * WebGPU binding-layer benchmark: the calls a three.js or PixiJS frame repeats thousands of
 * times, timed through the public wrappers an app uses.
 *
 * Every result is one `PERF|...` line so a run can be scraped off logcat or the sim log:
 *
 *   PERF|<group>|<name>|<calls>|<best ms>|<ns per op>|<opt status>
 *
 * Groups:
 *   state  - render pass state calls: closest to pure call overhead.
 *   draw   - draw calls on a 4x4 target, so the GPU is not what is timed.
 *   upload - queue.writeBuffer of small payloads, as per-object uniforms are.
 *   query  - wrapper getters a hot path may touch per call.
 *
 * Only the JS side is timed: the queue thread encodes and submits after the loop.
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
}

const results: Array<{ group: string; name: string; nsPerOp: number }> = [];

function bench(group: string, name: string, fn: (n: number) => void, opts: BenchOptions = {}) {
	const iters = opts.iters ?? 50_000;
	const reps = opts.reps ?? 5;

	// Warm into the optimizing tier: fast-API calls are only emitted from optimized code.
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

	const nsPerOp = (best * 1e6) / iters;
	results.push({ group, name, nsPerOp });
	console.log(`PERF|${group}|${name}|${iters}|${best.toFixed(3)}|${nsPerOp.toFixed(1)}|${tier(fn)}`);
}

const SHADER = /* wgsl */ `
struct Uniforms { offset: vec4f };
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs(@location(0) position: vec2f) -> @builtin(position) vec4f {
  return vec4f(position + uniforms.offset.xy, 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4f {
  return vec4f(1.0, 0.0, 0.0, 1.0);
}`;

export async function runWebGPUPerf(canvas: any) {
	const gpu = (globalThis as any).navigator?.gpu;
	const adapter = await gpu?.requestAdapter?.();
	const device = await adapter?.requestDevice?.();
	if (!device) {
		console.log('PERF|error|no webgpu device');
		return;
	}

	console.log(`PERF|meta|context|webgpu`);
	console.log(`PERF|meta|canvas|${canvas.width}x${canvas.height}`);
	console.log(`PERF|meta|natives|${getOptimizationStatus ? 'available' : 'unavailable'}`);
	console.log(`PERF|header|group|name|calls|bestMs|nsPerOp|tier`);

	const GPUBufferUsage = (globalThis as any).GPUBufferUsage;
	const GPUTextureUsage = (globalThis as any).GPUTextureUsage;
	const GPUShaderStage = (globalThis as any).GPUShaderStage;

	const module = device.createShaderModule({ code: SHADER });
	const layout = device.createBindGroupLayout({
		entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX, buffer: { type: 'uniform', hasDynamicOffset: false } }],
	});
	const dynamicLayout = device.createBindGroupLayout({
		entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX, buffer: { type: 'uniform', hasDynamicOffset: true } }],
	});
	const vertexLayout = { arrayStride: 8, attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' }] };
	const pipeline = device.createRenderPipeline({
		layout: device.createPipelineLayout({ bindGroupLayouts: [layout] }),
		vertex: { module, entryPoint: 'vs', buffers: [vertexLayout] },
		fragment: { module, entryPoint: 'fs', targets: [{ format: 'rgba8unorm' }] },
		primitive: { topology: 'triangle-list' },
	});
	const dynamicPipeline = device.createRenderPipeline({
		layout: device.createPipelineLayout({ bindGroupLayouts: [dynamicLayout] }),
		vertex: { module, entryPoint: 'vs', buffers: [vertexLayout] },
		fragment: { module, entryPoint: 'fs', targets: [{ format: 'rgba8unorm' }] },
		primitive: { topology: 'triangle-list' },
	});

	// A single degenerate triangle: the draw benches are about the call, not pixels.
	const vertices = device.createBuffer({ size: 24, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST });
	device.queue.writeBuffer(vertices, 0, new Float32Array([0, 0, 0.001, 0, 0, 0.001]));
	const indices = device.createBuffer({ size: 8, usage: GPUBufferUsage.INDEX | GPUBufferUsage.COPY_DST });
	device.queue.writeBuffer(indices, 0, new Uint16Array([0, 1, 2, 0]));
	const uniforms = device.createBuffer({ size: 256 * 4, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
	const group = device.createBindGroup({ layout, entries: [{ binding: 0, resource: { buffer: uniforms, size: 16 } }] });
	const dynamicGroup = device.createBindGroup({ layout: dynamicLayout, entries: [{ binding: 0, resource: { buffer: uniforms, size: 16 } }] });
	const target = device.createTexture({ size: { width: 4, height: 4 }, format: 'rgba8unorm', usage: GPUTextureUsage.RENDER_ATTACHMENT });
	const view = target.createView();

	let pass: any = null;
	let encoder: any = null;
	function begin() {
		encoder = device.createCommandEncoder();
		pass = encoder.beginRenderPass({ colorAttachments: [{ view, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 1 } }] });
		pass.setPipeline(pipeline);
		pass.setBindGroup(0, group);
		pass.setVertexBuffer(0, vertices);
		pass.setIndexBuffer(indices, 'uint16');
	}
	function end() {
		pass.end();
		device.queue.submit([encoder.finish()]);
		pass = null;
		encoder = null;
	}
	/** Runs `op` n times inside one pass. */
	function inPass(op: (i: number) => void) {
		return (n: number) => {
			begin();
			for (let i = 0; i < n; i++) {
				op(i);
			}
			end();
		};
	}

	const offsets = new Uint32Array([0]);

	bench(
		'state',
		'setPipeline',
		inPass(() => pass.setPipeline(pipeline)),
	);
	bench(
		'state',
		'setBindGroup',
		inPass(() => pass.setBindGroup(0, group)),
	);
	bench(
		'state',
		'setBindGroup(dynamic offsets)',
		inPass((i) => {
			offsets[0] = (i & 3) * 256;
			pass.setBindGroup(0, dynamicGroup, offsets, 0, 1);
		}),
	);
	bench(
		'state',
		'setVertexBuffer(slot, buffer)',
		inPass(() => pass.setVertexBuffer(0, vertices)),
	);
	bench(
		'state',
		'setVertexBuffer(slot, buffer, offset, size)',
		inPass(() => pass.setVertexBuffer(0, vertices, 0, 24)),
	);
	bench(
		'state',
		'setIndexBuffer(buffer, format)',
		inPass(() => pass.setIndexBuffer(indices, 'uint16')),
	);
	bench(
		'state',
		'setScissorRect',
		inPass(() => pass.setScissorRect(0, 0, 4, 4)),
	);
	bench(
		'state',
		'setViewport',
		inPass(() => pass.setViewport(0, 0, 4, 4, 0, 1)),
	);

	bench(
		'draw',
		'draw(3)',
		inPass(() => pass.draw(3)),
	);
	bench(
		'draw',
		'drawIndexed(3)',
		inPass(() => pass.drawIndexed(3)),
	);
	bench(
		'draw',
		'setBindGroup+drawIndexed',
		inPass(() => {
			pass.setBindGroup(0, group);
			pass.drawIndexed(3);
		}),
	);

	const bytes = new Uint8Array(256);
	const floats = new Float32Array(16);
	bench('upload', 'writeBuffer(Uint8Array 256B)', (n) => {
		for (let i = 0; i < n; i++) {
			device.queue.writeBuffer(uniforms, 0, bytes);
		}
	});
	bench('upload', 'writeBuffer(Float32Array 64B)', (n) => {
		for (let i = 0; i < n; i++) {
			device.queue.writeBuffer(uniforms, 0, floats);
		}
	});
	bench('upload', 'writeBuffer(Float32Array 64B, offset)', (n) => {
		for (let i = 0; i < n; i++) {
			device.queue.writeBuffer(uniforms, (i & 3) * 256, floats, 0, 16);
		}
	});

	let sink = 0;
	bench('query', 'buffer.size', (n) => {
		for (let i = 0; i < n; i++) {
			sink += vertices.size;
		}
	});
	bench('query', 'buffer.usage', (n) => {
		for (let i = 0; i < n; i++) {
			sink += vertices.usage;
		}
	});
	void sink;

	// Drain the queue so a crash in encoding shows up here, not in the next demo.
	await device.queue.onSubmittedWorkDone?.();
	console.log('PERF|done|webgpu');
	return results;
}
