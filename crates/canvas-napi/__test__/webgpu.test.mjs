// WebGPU through the Node-API module, headless (no surface), with the call shapes
// packages/canvas/WebGPU uses: `new CanvasModule.GPU()`, callback-style requestAdapter /
// requestDevice / popErrorScope / onSubmittedWorkDone / create*PipelineAsync, promise-returning
// mapAsync / lost / getCompilationInfo, descriptors pre-parsed as Utils.ts parses them (vertex
// formats as ints, native objects in place of the wrappers, textures with a flattened size).
// On Windows wgpu picks the D3D12 adapter; without one the suite is skipped. "No object" results
// are null (napi-rs's None) where the V8 bindings return undefined; packages/canvas only tests
// them for truthiness.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';

import { CanvasModule } from './addon.mjs';

const { GPU, GPUSupportedLimits, GPUCompilationInfo, GPUCompilationMessage, GPUCanvasContext } = CanvasModule;

// packages/canvas/WebGPU/Constants.ts
const BufferUsage = { MAP_READ: 0x1, MAP_WRITE: 0x2, COPY_SRC: 0x4, COPY_DST: 0x8, INDEX: 0x10, VERTEX: 0x20, UNIFORM: 0x40, STORAGE: 0x80 };
const TextureUsage = { COPY_SRC: 0x1, COPY_DST: 0x2, TEXTURE_BINDING: 0x4, STORAGE_BINDING: 0x8, RENDER_ATTACHMENT: 0x10 };
const MapMode = { READ: 1, WRITE: 2 };
const ShaderStage = { VERTEX: 1, FRAGMENT: 2, COMPUTE: 4 };
// Utils.ts parseVertexFormat('float32x2')
const FLOAT32X2 = 28;

// ---------------------------------------------------------------------------------------------
// The shapes of packages/canvas/WebGPU/*.ts
// ---------------------------------------------------------------------------------------------

let gpu;
// GPU.ts
function requestAdapter(options = { powerPreference: undefined, isFallbackAdapter: false }) {
	return new Promise((resolve, reject) => {
		gpu.requestAdapter(options, (error, adapter) => {
			if (error) reject(error);
			else resolve(adapter ?? null);
		});
	});
}

// GPUAdapter.ts requestDevice, on a Node-API host: without requiredLimits it asks for the
// WebGPU defaults (a fresh GPUSupportedLimits) clamped to the adapter.
const MAX_LIMIT_KEYS = [
	'maxTextureDimension1d', 'maxTextureDimension2d', 'maxTextureDimension3d', 'maxTextureArrayLayers', 'maxBindGroups',
	'maxBindingsPerBindGroup', 'maxDynamicUniformBuffersPerPipelineLayout', 'maxDynamicStorageBuffersPerPipelineLayout',
	'maxSampledTexturesPerShaderStage', 'maxSamplersPerShaderStage', 'maxStorageBuffersPerShaderStage', 'maxStorageTexturesPerShaderStage',
	'maxUniformBuffersPerShaderStage', 'maxUniformBufferBindingSize', 'maxStorageBufferBindingSize', 'maxVertexBuffers', 'maxBufferSize',
	'maxVertexAttributes', 'maxVertexBufferArrayStride', 'maxInterStageShaderVariables', 'maxColorAttachments', 'maxColorAttachmentBytesPerSample',
	'maxComputeWorkgroupStorageSize', 'maxComputeInvocationsPerWorkgroup', 'maxComputeWorkgroupSizeX', 'maxComputeWorkgroupSizeY',
	'maxComputeWorkgroupSizeZ', 'maxComputeWorkgroupsPerDimension', 'maxNonSamplerBindings',
];
const MIN_LIMIT_KEYS = ['minUniformBufferOffsetAlignment', 'minStorageBufferOffsetAlignment'];

function requestDevice(adapter, desc) {
	return new Promise((resolve, reject) => {
		const options = desc ?? {};
		if (Array.isArray(options.requiredFeatures)) options.requiredFeatures.push('texture-adapter-specific-format-features');
		else options.requiredFeatures = ['texture-adapter-specific-format-features'];
		if (!options.requiredLimits) {
			const requiredLimits = new GPUSupportedLimits();
			const limits = adapter.limits;
			for (const key of MAX_LIMIT_KEYS) {
				const adapterVal = limits[key];
				if (typeof adapterVal === 'number' && typeof requiredLimits[key] === 'number' && requiredLimits[key] > adapterVal) requiredLimits[key] = adapterVal;
			}
			for (const key of MIN_LIMIT_KEYS) {
				const adapterVal = limits[key];
				if (typeof adapterVal === 'number' && typeof requiredLimits[key] === 'number' && requiredLimits[key] < adapterVal) requiredLimits[key] = adapterVal;
			}
			if (limits.maxSampledTexturesPerShaderStage >= 128) requiredLimits.maxSampledTexturesPerShaderStage = 128;
			if (limits.maxSamplersPerShaderStage >= 128) requiredLimits.maxSamplersPerShaderStage = 128;
			options.requiredLimits = requiredLimits;
		} else if (typeof options.requiredLimits === 'object' && options.requiredLimits?.constructor?.name !== 'GPUSupportedLimits') {
			const keys = Object.keys(options.requiredLimits);
			if (keys.length === 0) {
				delete options.requiredLimits;
			} else {
				const requiredLimits = new GPUSupportedLimits();
				for (const key of keys) requiredLimits[key] = options.requiredLimits[key];
				const adapterLimits = adapter.limits;
				for (const key of keys) {
					const adapterVal = adapterLimits[key];
					if (typeof adapterVal === 'number' && typeof requiredLimits[key] === 'number') {
						if (!key.startsWith('min') && requiredLimits[key] > adapterVal) requiredLimits[key] = adapterVal;
						else if (key.startsWith('min') && requiredLimits[key] < adapterVal) requiredLimits[key] = adapterVal;
					}
				}
				options.requiredLimits = requiredLimits;
			}
		}
		adapter.requestDevice(options, (error, device) => {
			if (error) reject(error);
			else resolve(device);
		});
	});
}

// GPUDevice.fromNative: uncaptured errors and `lost`.
function watch(device) {
	const state = { errors: [], waiters: [], lost: null };
	device.setuncapturederror((type, message) => {
		state.errors.push({ type, message });
		for (const waiter of state.waiters.splice(0)) waiter();
	});
	state.lostPromise = device.lost.then((info) => (state.lost = info));
	state.nextError = (ms = 5000) =>
		new Promise((resolve, reject) => {
			if (state.errors.length) return resolve(state.errors.shift());
			const timer = setTimeout(() => reject(new Error('no uncaptured error arrived')), ms);
			state.waiters.push(() => {
				clearTimeout(timer);
				resolve(state.errors.shift());
			});
		});
	return state;
}

// GPUDevice.ts popErrorScope
function popErrorScope(device) {
	return new Promise((resolve) => device.popErrorScope((type, message) => resolve({ type, message })));
}

// GPUDevice.ts createBuffer (size aligned to 4)
function createBuffer(device, descriptor) {
	return device.createBuffer({ ...descriptor, size: (descriptor.size + 3) & ~3 });
}

// GPUQueue.ts writeBuffer: bytes, normalised in JS.
function writeBuffer(queue, buffer, bufferOffset, data, dataOffset = 0, size) {
	const view = data instanceof ArrayBuffer ? new Uint8Array(data) : data;
	const elemSize = ArrayBuffer.isView(view) && !(view instanceof DataView) ? (view.BYTES_PER_ELEMENT ?? 1) : 1;
	const dataByteOffset = dataOffset * elemSize;
	const available = Math.max(0, view.byteLength - dataByteOffset);
	let writeBytes = size === undefined ? available : Math.min(size * elemSize, available);
	if (writeBytes === 0) return;
	const u8 = new Uint8Array(view.buffer, view.byteOffset + dataByteOffset, writeBytes);
	queue.writeBuffer(buffer, bufferOffset ?? 0, u8, 0, writeBytes);
}

// GPUQueue.ts submit: consumed command buffers are released right away.
function submit(queue, commands) {
	queue.submit(commands);
	for (const command of commands) command?.destroy?.();
}

// GPUQueue.ts onSubmittedWorkDone
function workDone(queue) {
	return new Promise((resolve) => queue.onSubmittedWorkDone(() => resolve()));
}

// GPUCommandEncoder.ts finish, pass end.
function finish(encoder, descriptor) {
	const buffer = encoder.finish(descriptor);
	encoder.destroy();
	return buffer;
}
function end(pass) {
	pass.end();
	pass.destroy();
}

// GPUDevice.ts createTexture: the size flattened.
function createTexture(device, descriptor) {
	const sizeIsArray = Array.isArray(descriptor.size);
	return device.createTexture({
		label: descriptor.label,
		mipLevelCount: descriptor.mipLevelCount ?? 1,
		sampleCount: descriptor.sampleCount ?? 1,
		dimension: descriptor.dimension ?? '2d',
		format: descriptor.format,
		usage: descriptor.usage,
		viewFormats: descriptor.viewFormats ?? [],
		width: sizeIsArray ? descriptor.size[0] : descriptor.size.width,
		height: sizeIsArray ? (descriptor.size[1] ?? 1) : (descriptor.size.height ?? 1),
		depthOrArrayLayers: sizeIsArray ? (descriptor.size[2] ?? 1) : (descriptor.size.depthOrArrayLayers ?? 1),
	});
}

async function readBuffer(device, source, size, offset = 0) {
	const readback = createBuffer(device, { size, usage: BufferUsage.MAP_READ | BufferUsage.COPY_DST });
	const encoder = device.createCommandEncoder(undefined);
	encoder.copyBufferToBuffer(source, offset, readback, 0, size);
	submit(device.queue, [finish(encoder)]);
	await readback.mapAsync(MapMode.READ, undefined, undefined);
	const bytes = new Uint8Array(readback.getMappedRange(undefined, size).slice(0));
	readback.unmap();
	readback.destroy();
	return bytes;
}

// ---------------------------------------------------------------------------------------------

let adapter;
let device;
let watched;
let skip = false;

before(async () => {
	gpu = new GPU();
	adapter = await requestAdapter();
	if (!adapter) {
		skip = 'no WebGPU adapter';
		return;
	}
	device = await requestDevice(adapter);
	watched = watch(device);
});

after(() => {
	device?.destroy();
});

test('new CanvasModule.GPU(), like packages/canvas', (t) => {
	assert.ok(gpu instanceof GPU);
	assert.equal(gpu.getPreferredCanvasFormat(), process.platform === 'linux' ? 'rgba8unorm' : 'bgra8unorm');
	assert.match(gpu.__getPointer(), /^[1-9][0-9]*$/);
	// one wgpu instance per process
	assert.equal(new GPU().__getPointer(), gpu.__getPointer());
	assert.equal(GPU.getInstance().__getPointer(), gpu.__getPointer());
	if (skip) t.skip(skip);
});

test('the adapter: features, limits, info', (t) => {
	if (skip) return t.skip(skip);
	assert.ok(adapter.features instanceof Set);
	for (const feature of adapter.features) assert.equal(typeof feature, 'string');
	assert.equal(typeof adapter.isFallbackAdapter, 'boolean');
	const limits = adapter.limits;
	assert.ok(limits instanceof GPUSupportedLimits);
	assert.ok(limits.maxTextureDimension2D >= 8192);
	assert.ok(limits.maxBindGroups >= 4);
	assert.ok(limits.maxBufferSize > 0);
	assert.equal(limits.maxInterStageShaderComponents, limits.maxInterStageShaderVariables);
	// requestAdapterInfo is synchronous (GPUAdapter.ts wraps it in a promise)
	const info = adapter.requestAdapterInfo();
	for (const key of ['vendor', 'architecture', 'device', 'description']) assert.equal(typeof info[key], 'string');
});

test('requestAdapter options: power preference (string or int) and feature level', async (t) => {
	if (skip) return t.skip(skip);
	for (const options of [{ powerPreference: 'high-performance' }, { powerPreference: 1 }, { featureLevel: 'core', isFallbackAdapter: false }, undefined, null]) {
		const found = await requestAdapter(options);
		assert.ok(found, JSON.stringify(options));
		assert.ok(found.features.has('core-features-and-limits'));
	}
	const compatibility = await requestAdapter({ featureLevel: 'compatibility' });
	if (compatibility) assert.equal(compatibility.features.has('core-features-and-limits'), false);
});

test('GPUSupportedLimits is constructible with settable limits', () => {
	const limits = new GPUSupportedLimits();
	assert.equal(limits.maxBindGroups, 4);
	assert.equal(limits.maxTextureDimension1D, 8192);
	limits.maxBindGroups = 2;
	assert.equal(limits.maxBindGroups, 2);
	limits.maxBindGroups = 'three'; // ignored
	assert.equal(limits.maxBindGroups, 2);
	limits.maxBufferSize = 2 ** 33;
	assert.equal(limits.maxBufferSize, 2 ** 33);
	assert.equal(limits.minSubgroupSize, 0);
});

test('requestDevice: the device, its limits, features and queue', async (t) => {
	if (skip) return t.skip(skip);
	assert.ok(device.features instanceof Set);
	assert.ok(device.limits instanceof GPUSupportedLimits);
	// the WebGPU defaults, as requested (with the 128 sampled textures bump where supported)
	assert.equal(device.limits.maxBindGroups, 4);
	assert.equal(device.limits.maxSampledTexturesPerShaderStage, adapter.limits.maxSampledTexturesPerShaderStage >= 128 ? 128 : 16);
	assert.equal(device.label, '');
	assert.equal(device.queue.label, '');
	// requiredLimits given as a plain object (GPUAdapter.ts turns it into GPUSupportedLimits)
	const limited = await requestDevice(adapter, { label: 'limited', requiredLimits: { maxBindGroups: 2, maxStorageBuffersPerShaderStage: 4 } });
	assert.equal(limited.label, 'limited');
	assert.equal(limited.limits.maxBindGroups, 2);
	assert.equal(limited.limits.maxStorageBuffersPerShaderStage, 4);
	limited.destroy();
});

test('requestDevice reports an unsupported feature through the callback', async (t) => {
	if (skip) return t.skip(skip);
	const missing = ['texture-compression-astc', 'texture-compression-etc2', 'shader-f16'].find((f) => !adapter.features.has(f));
	if (!missing) return t.skip('the adapter supports every probed feature');
	await assert.rejects(requestDevice(adapter, { requiredFeatures: [missing] }), /support/i);
});

test('createBuffer + writeBuffer + mapAsync readback', async (t) => {
	if (skip) return t.skip(skip);
	const source = createBuffer(device, { size: 16, usage: BufferUsage.COPY_SRC | BufferUsage.COPY_DST, label: 'source' });
	assert.equal(source.size, 16);
	assert.equal(source.usage, BufferUsage.COPY_SRC | BufferUsage.COPY_DST);
	assert.equal(source.label, 'source');
	const data = new Uint32Array([1, 2, 3, 0xdeadbeef]);
	writeBuffer(device.queue, source, 0, data);
	const bytes = await readBuffer(device, source, 16);
	assert.deepEqual(Array.from(new Uint32Array(bytes.buffer)), [1, 2, 3, 0xdeadbeef]);

	// a typed-array window with element offsets, as the TS normalises them
	const window = new Float32Array([9, 8, 7, 6, 5]);
	writeBuffer(device.queue, source, 4, window, 1, 3);
	const again = new Float32Array((await readBuffer(device, source, 16)).buffer);
	assert.deepEqual(Array.from(again.subarray(1)), [8, 7, 6]);

	// mappedAtCreation
	const mapped = createBuffer(device, { size: 8, usage: BufferUsage.COPY_SRC, mappedAtCreation: true });
	assert.equal(mapped.mapState, 'mapped');
	const range = mapped.getMappedRange(undefined, 8);
	assert.equal(range.byteLength, 8);
	new Uint8Array(range).set([10, 20, 30, 40, 50, 60, 70, 80]);
	mapped.unmap();
	assert.equal(range.byteLength, 0, 'unmap detaches the mapped range');
	assert.deepEqual(Array.from(await readBuffer(device, mapped, 8)), [10, 20, 30, 40, 50, 60, 70, 80]);

	// writing outside the data throws instead of reaching canvas-c
	assert.throws(() => device.queue.writeBuffer(source, 0, new Uint8Array(4), 2, 8), /outside/);
	source.destroy();
	mapped.destroy();
});

test('mapAsync rejects when the mapping fails', async (t) => {
	if (skip) return t.skip(skip);
	device.pushErrorScope('validation');
	const buffer = createBuffer(device, { size: 16, usage: BufferUsage.COPY_DST });
	await assert.rejects(buffer.mapAsync(MapMode.READ, undefined, undefined));
	await popErrorScope(device);
	buffer.destroy();
});

const doubleWGSL = /* wgsl */ `
override scale: f32 = 2.0;
@group(0) @binding(0) var<storage, read_write> data: array<f32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
  if (id.x < arrayLength(&data)) {
    data[id.x] = data[id.x] * scale;
  }
}`;

async function runCompute(pipeline, input) {
	const size = input.byteLength;
	const storage = createBuffer(device, { size, usage: BufferUsage.STORAGE | BufferUsage.COPY_SRC | BufferUsage.COPY_DST });
	writeBuffer(device.queue, storage, 0, input);
	// Utils.ts parseBindGroupDescriptor
	const group = device.createBindGroup({
		layout: pipeline.getBindGroupLayout(0),
		entries: [{ binding: 0, resource: { buffer: storage, offset: 0, size: storage.size } }],
	});
	const encoder = device.createCommandEncoder({ label: 'compute' });
	assert.equal(encoder.label, 'compute');
	const pass = encoder.beginComputePass(undefined);
	pass.setPipeline(pipeline);
	pass.setBindGroup(0, group);
	pass.dispatchWorkgroups(Math.ceil(input.length / 64), 1, 1);
	end(pass);
	submit(device.queue, [finish(encoder)]);
	await workDone(device.queue);
	const out = new Float32Array((await readBuffer(device, storage, size)).buffer);
	storage.destroy();
	return out;
}

test('a compute shader doubles numbers', async (t) => {
	if (skip) return t.skip(skip);
	const module = device.createShaderModule({ label: 'double', code: doubleWGSL, sourceMap: undefined, compilationHints: undefined });
	assert.equal(module.label, 'double');
	const info = await module.getCompilationInfo();
	assert.ok(info instanceof GPUCompilationInfo);
	assert.deepEqual(info.messages, []);

	// Utils.ts parseComputePipelineDescriptor
	const pipeline = device.createComputePipeline({ compute: { module, entryPoint: 'main' }, layout: 'auto' });
	const input = Float32Array.from({ length: 100 }, (_, i) => i + 0.5);
	const out = await runCompute(pipeline, input);
	assert.deepEqual(Array.from(out), Array.from(input, (v) => v * 2));

	// pipeline-overridable constants as a plain object (the web, packages/canvas) or a Map (what
	// the V8 bindings read)
	const tripled = device.createComputePipeline({ compute: { module, entryPoint: 'main', constants: { scale: 3 } }, layout: 'auto' });
	assert.deepEqual(Array.from(await runCompute(tripled, input)), Array.from(input, (v) => v * 3));
	const quadrupled = device.createComputePipeline({ compute: { module, entryPoint: 'main', constants: new Map([['scale', 4]]) }, layout: 'auto' });
	assert.deepEqual(Array.from(await runCompute(quadrupled, new Float32Array([1, 2]))), [4, 8]);
});

test('createComputePipelineAsync calls back (error, pipeline)', async (t) => {
	if (skip) return t.skip(skip);
	const module = device.createShaderModule({ code: doubleWGSL });
	// GPUDevice.ts createComputePipelineAsync
	const pipeline = await new Promise((resolve, reject) => {
		device.createComputePipelineAsync({ compute: { module, entryPoint: 'main' }, layout: 'auto' }, (error, pipeline) => {
			if (error) reject(error.error);
			else resolve(pipeline);
		});
	});
	const out = await runCompute(pipeline, new Float32Array([1, 2, 3, 4]));
	assert.deepEqual(Array.from(out), [2, 4, 6, 8]);
	// an explicit pipeline layout
	const bindGroupLayout = device.createBindGroupLayout({
		entries: [{ binding: 0, visibility: ShaderStage.COMPUTE, buffer: { type: 'storage' } }],
	});
	const layout = device.createPipelineLayout({ bindGroupLayouts: [bindGroupLayout] });
	const explicit = device.createComputePipeline({ compute: { module, entryPoint: 'main' }, layout });
	assert.deepEqual(Array.from(await runCompute(explicit, new Float32Array([5]))), [10]);
});

test('shader compilation errors: getCompilationInfo and error scopes', async (t) => {
	if (skip) return t.skip(skip);
	device.pushErrorScope('validation');
	const module = device.createShaderModule({ code: '@compute @workgroup_size(1) fn main() { let x: f32 = nope; }' });
	const scope = await popErrorScope(device);
	assert.equal(scope.type, 3);
	assert.equal(typeof scope.message, 'string');
	const info = await module.getCompilationInfo();
	assert.ok(info.messages.length > 0);
	const message = info.messages[0];
	assert.ok(message instanceof GPUCompilationMessage);
	assert.equal(message.type, 'error');
	assert.ok(message.message.length > 0);
	assert.ok(message.lineNum >= 1);
	for (const key of ['linePos', 'offset', 'length']) assert.equal(typeof message[key], 'number');
});

test('error scopes and the uncaptured-error path', async (t) => {
	if (skip) return t.skip(skip);
	// nothing to catch
	device.pushErrorScope('validation');
	assert.deepEqual(await popErrorScope(device), { type: 0, message: null });

	// MAP_READ may only go with COPY_DST: a validation error
	const invalid = { size: 16, usage: BufferUsage.MAP_READ | BufferUsage.STORAGE };
	device.pushErrorScope('validation');
	createBuffer(device, invalid);
	const caught = await popErrorScope(device);
	assert.equal(caught.type, 3);
	assert.match(caught.message, /usage/i);

	// an out-of-memory scope does not catch it: it reaches setuncapturederror
	watched.errors.length = 0;
	device.pushErrorScope('out-of-memory');
	createBuffer(device, invalid);
	assert.equal((await popErrorScope(device)).type, 0);
	const uncaptured = await watched.nextError();
	assert.equal(uncaptured.type, 3);
	assert.match(uncaptured.message, /usage/i);

	// an empty scope stack is not a crash
	assert.equal((await popErrorScope(device)).type, 0);
});

const triangleWGSL = /* wgsl */ `
@vertex fn vs(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
  return vec4<f32>(position, 0.0, 1.0);
}
@fragment fn fs() -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}`;

function trianglePipelineDescriptor(module) {
	// Utils.ts parseRenderPipelineDescriptor: vertex formats become parseVertexFormat ints.
	return {
		vertex: { module, entryPoint: 'vs', buffers: [{ arrayStride: 8, stepMode: 'vertex', attributes: [{ format: FLOAT32X2, offset: 0, shaderLocation: 0 }] }] },
		fragment: { module, entryPoint: 'fs', targets: [{ format: 'rgba8unorm' }] },
		primitive: { topology: 'triangle-list', cullMode: 'none' },
		multisample: { count: 1 },
		layout: 'auto',
	};
}

const SIZE = 64;

async function renderTriangle({ pipeline, attachmentView, bundle, indexed }) {
	const texture = attachmentView.texture;
	const vertices = createBuffer(device, { size: 24, usage: BufferUsage.VERTEX | BufferUsage.COPY_DST });
	writeBuffer(device.queue, vertices, 0, new Float32Array([-0.5, -0.5, 0.5, -0.5, 0, 0.5]));
	const indices = createBuffer(device, { size: 6, usage: BufferUsage.INDEX | BufferUsage.COPY_DST });
	writeBuffer(device.queue, indices, 0, new Uint16Array([0, 1, 2, 0]));

	const encoder = device.createCommandEncoder(undefined);
	// Utils.ts parseRenderPassDescriptor
	const pass = encoder.beginRenderPass({
		colorAttachments: [{ loadOp: 'clear', storeOp: 'store', view: attachmentView.view, clearValue: { r: 0, g: 0, b: 1, a: 1 } }],
	});
	if (bundle) {
		pass.executeBundles([bundle(vertices)]);
	} else {
		pass.setPipeline(pipeline);
		pass.setViewport(0, 0, SIZE, SIZE, 0, 1);
		pass.setScissorRect(0, 0, SIZE, SIZE);
		pass.setVertexBuffer(0, vertices, 0, vertices.size);
		if (indexed) {
			// GPURenderPassEncoder.ts: 0 = uint16
			pass.setIndexBuffer(indices, 0, 0, 6);
			pass.drawIndexed(3, 1, 0, 0, 0);
		} else {
			pass.draw(3, 1, 0, 0);
		}
	}
	end(pass);
	const readback = createBuffer(device, { size: 256 * SIZE, usage: BufferUsage.MAP_READ | BufferUsage.COPY_DST });
	// GPUCommandEncoder.ts copyTextureToBuffer
	encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow: 256 }, { width: SIZE, height: SIZE, depthOrArrayLayers: 1 });
	submit(device.queue, [finish(encoder)]);
	await readback.mapAsync(MapMode.READ, undefined, undefined);
	const pixels = new Uint8Array(readback.getMappedRange(undefined, 256 * SIZE).slice(0));
	readback.unmap();
	readback.destroy();
	vertices.destroy();
	indices.destroy();
	return (x, y) => Array.from(pixels.subarray(y * 256 + x * 4, y * 256 + x * 4 + 4));
}

function renderTarget() {
	const texture = createTexture(device, {
		label: 'target',
		size: [SIZE, SIZE],
		format: 'rgba8unorm',
		usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
	});
	assert.equal(texture.width, SIZE);
	assert.equal(texture.height, SIZE);
	assert.equal(texture.depthOrArrayLayers, 1);
	assert.equal(texture.format, 'rgba8unorm');
	assert.equal(texture.dimension, '2d');
	assert.equal(texture.mipLevelCount, 1);
	assert.equal(texture.sampleCount, 1);
	assert.equal(texture.usage, TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC);
	assert.equal(texture.label, 'target');
	return texture;
}

test('render a triangle into a texture and read the pixels back', async (t) => {
	if (skip) return t.skip(skip);
	const module = device.createShaderModule({ code: triangleWGSL });
	const pipeline = device.createRenderPipeline(trianglePipelineDescriptor(module));
	assert.ok(pipeline.getBindGroupLayout);

	const texture = renderTarget();
	const view = texture.createView(undefined);
	let pixel = await renderTriangle({ pipeline, attachmentView: { texture, view } });
	assert.deepEqual(pixel(SIZE / 2, SIZE / 2), [255, 0, 0, 255]);
	assert.deepEqual(pixel(1, 1), [0, 0, 255, 255]);
	assert.deepEqual(pixel(SIZE - 2, 1), [0, 0, 255, 255]);

	// indexed, and the texture itself as the attachment view (what parseRenderPassDescriptor
	// hands over for a GPUTexture)
	pixel = await renderTriangle({ pipeline, attachmentView: { texture, view: texture }, indexed: true });
	assert.deepEqual(pixel(SIZE / 2, SIZE / 2), [255, 0, 0, 255]);
	assert.deepEqual(pixel(1, SIZE - 2), [0, 0, 255, 255]);

	view.destroy();
	assert.equal(view.label, '');
	texture.destroy();
});

test('createRenderPipelineAsync and render bundles', async (t) => {
	if (skip) return t.skip(skip);
	const module = device.createShaderModule({ code: triangleWGSL });
	const pipeline = await new Promise((resolve, reject) =>
		device.createRenderPipelineAsync(trianglePipelineDescriptor(module), (error, pipeline) => (error ? reject(error.error) : resolve(pipeline))),
	);
	const texture = renderTarget();
	const view = texture.createView({ label: 'view', format: 'rgba8unorm', dimension: '2d', aspect: 'all', baseMipLevel: 0, mipLevelCount: 1 });
	assert.equal(view.label, 'view');
	const bundle = (vertices) => {
		const encoder = device.createRenderBundleEncoder({ colorFormats: ['rgba8unorm'], label: 'bundle' });
		assert.equal(encoder.label, 'bundle');
		encoder.setPipeline(pipeline);
		encoder.setVertexBuffer(0, vertices, 0, vertices.size);
		encoder.draw(3, 1, 0, 0);
		return encoder.finish(undefined);
	};
	const pixel = await renderTriangle({ attachmentView: { texture, view }, bundle });
	assert.deepEqual(pixel(SIZE / 2, SIZE / 2), [255, 0, 0, 255]);
	assert.deepEqual(pixel(1, 1), [0, 0, 255, 255]);
	texture.destroy();
});

test('writeTexture, copyBufferToTexture and copyTextureToTexture', async (t) => {
	if (skip) return t.skip(skip);
	const usage = TextureUsage.COPY_DST | TextureUsage.COPY_SRC;
	const a = createTexture(device, { size: { width: 4, height: 4 }, format: 'rgba8unorm', usage });
	const b = createTexture(device, { size: [4, 4, 1], format: 'rgba8unorm', usage });
	const pixels = Uint8Array.from({ length: 64 }, (_, i) => i);
	// GPUQueue.ts writeTexture
	device.queue.writeTexture({ texture: a, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } }, pixels, { offset: 0, bytesPerRow: 16, rowsPerImage: 4 }, { width: 4, height: 4, depthOrArrayLayers: 1 });
	const encoder = device.createCommandEncoder(undefined);
	encoder.copyTextureToTexture({ texture: a }, { texture: b }, { width: 4, height: 4, depthOrArrayLayers: 1 });
	const out = createBuffer(device, { size: 256 * 4, usage: BufferUsage.COPY_SRC | BufferUsage.COPY_DST });
	encoder.copyTextureToBuffer({ texture: b, origin: { x: 0, y: 0, z: 0 } }, { buffer: out, bytesPerRow: 256, rowsPerImage: 4 }, { width: 4, height: 4, depthOrArrayLayers: 1 });
	// and back through copyBufferToTexture, shifted by one row
	encoder.copyBufferToTexture({ buffer: out, bytesPerRow: 256 }, { texture: a, origin: { x: 0, y: 1, z: 0 } }, { width: 4, height: 3, depthOrArrayLayers: 1 });
	encoder.clearBuffer(out, 256 * 3, 256);
	submit(device.queue, [finish(encoder)]);
	const bytes = await readBuffer(device, out, 256 * 4);
	for (let row = 0; row < 3; row++) {
		assert.deepEqual(Array.from(bytes.subarray(row * 256, row * 256 + 16)), Array.from(pixels.subarray(row * 16, row * 16 + 16)));
	}
	assert.deepEqual(Array.from(bytes.subarray(256 * 3, 256 * 3 + 16)), new Array(16).fill(0));
	a.destroy();
	b.destroy();
	out.destroy();
});

test('samplers, bind group layouts with textures, query sets', (t) => {
	if (skip) return t.skip(skip);
	const sampler = device.createSampler({ label: 's', magFilter: 'linear', minFilter: 'linear', addressModeU: 'repeat', maxAnisotropy: 1 });
	assert.equal(sampler.label, 's');
	assert.ok(device.createSampler(undefined));
	const texture = createTexture(device, { size: [4, 4], format: 'rgba8unorm', usage: TextureUsage.TEXTURE_BINDING });
	const layout = device.createBindGroupLayout({
		entries: [
			{ binding: 0, visibility: ShaderStage.FRAGMENT, sampler: { type: 'filtering' } },
			{ binding: 1, visibility: ShaderStage.FRAGMENT, texture: { sampleType: 'float', viewDimension: '2d' } },
		],
	});
	const view = texture.createView(undefined);
	const group = device.createBindGroup({ layout, entries: [{ binding: 0, resource: sampler }, { binding: 1, resource: view }] });
	assert.ok(group);
	assert.equal(group.label, '');

	const querySet = device.createQuerySet({ type: 'occlusion', count: 4, label: 'q' });
	assert.equal(querySet.count, 4);
	assert.equal(querySet.type, 'occlusion');
	assert.equal(querySet.label, 'q');
	querySet.destroy();
	assert.equal(device.createQuerySet({ type: 'nope', count: 1 }) ?? undefined, undefined);
	view.destroy();
	texture.destroy();
});

test('released handles: destroy() on encoders, passes, buffers, views, textures', (t) => {
	if (skip) return t.skip(skip);
	const encoder = device.createCommandEncoder(undefined);
	const pass = encoder.beginComputePass({ label: 'p' });
	assert.equal(pass.label, 'p');
	pass.end();
	pass.destroy();
	pass.destroy();
	pass.end(); // a released pass is a no-op
	const commandBuffer = encoder.finish({ label: 'done' });
	assert.equal(commandBuffer.label, 'done');
	encoder.destroy();
	encoder.destroy();
	assert.equal(encoder.label, '');
	assert.equal(encoder.finish(undefined) ?? undefined, undefined, 'a released encoder finishes nothing');
	commandBuffer.destroy();
	// submitting released command buffers (and nulls) skips them
	device.queue.submit([commandBuffer, null, undefined]);

	const texture = createTexture(device, { size: [8, 8], format: 'rgba8unorm', usage: TextureUsage.RENDER_ATTACHMENT });
	const view = texture.createView(undefined);
	view.destroy();
	texture.__releaseHandle();
	assert.equal(texture.width, 0);
	assert.equal(texture.createView(undefined) ?? undefined, undefined);

	const buffer = createBuffer(device, { size: 4, usage: BufferUsage.COPY_DST });
	buffer.destroy();
	buffer.destroy();

	// dynamic offsets outside the data throw rather than aborting in canvas-c
	const layout = device.createBindGroupLayout({ entries: [{ binding: 0, visibility: ShaderStage.COMPUTE, buffer: { type: 'storage', hasDynamicOffset: true } }] });
	const storage = createBuffer(device, { size: 512, usage: BufferUsage.STORAGE });
	const group = device.createBindGroup({ layout, entries: [{ binding: 0, resource: { buffer: storage, offset: 0, size: 256 } }] });
	const encoder2 = device.createCommandEncoder(undefined);
	const pass2 = encoder2.beginComputePass(undefined);
	assert.throws(() => pass2.setBindGroup(0, group, new Uint32Array([0]), 1, 1), /out of range/);
	pass2.setBindGroup(0, group, new Uint32Array([256]), 0, 1);
	pass2.setBindGroup(0, group, [0]);
	end(pass2);
	finish(encoder2).destroy();
	storage.destroy();
});

test('device.destroy() resolves lost with reason 1', async (t) => {
	if (skip) return t.skip(skip);
	const doomed = await requestDevice(adapter);
	const state = watch(doomed);
	assert.equal(doomed.lost, doomed.lost, 'lost is one promise');
	doomed.destroy();
	const info = await state.lostPromise;
	assert.equal(info.reason, 1);
	assert.equal(typeof info.message, 'string');
	// lost read after destroy() is already settled
	const late = await requestDevice(adapter);
	late.destroy();
	assert.equal((await late.lost).reason, 1);
});

test('a device with an uncaptured-error handler and a lost reaction can be collected', { skip: typeof globalThis.gc !== 'function' && 'run with --expose-gc' }, async (t) => {
	if (skip) return t.skip(skip);
	let collected = false;
	const registry = new FinalizationRegistry(() => (collected = true));
	await (async () => {
		// GPUDevice.fromNative: the handler is bound to the wrapper that holds the native device.
		const owner = { native: await requestDevice(adapter), errors: 0 };
		owner.native.setuncapturederror(function () {
			this.errors++;
		}.bind(owner));
		owner.native.lost.then(() => {});
		registry.register(owner.native, 'device');
	})();
	for (let i = 0; i < 50 && !collected; i++) {
		globalThis.gc();
		await new Promise((resolve) => setTimeout(resolve, 10));
	}
	assert.ok(collected, 'the device was collected');
});

test('createWebGPUContextWithPointer and the GPUCanvasContext surface', () => {
	assert.equal(typeof CanvasModule.createWebGPUContextWithPointer, 'function');
	assert.equal(CanvasModule.createWebGPUContextWithPointer(0n) ?? undefined, undefined);
	for (const name of ['configure', 'unconfigure', 'getCurrentTexture', 'presentSurface', 'getCapabilities', '__toDataURL', '__startRaf', '__stopRaf']) {
		assert.equal(typeof GPUCanvasContext.prototype[name], 'function', name);
	}
	assert.ok('continuousRenderMode' in GPUCanvasContext.prototype);
});

test('GC churn: wrappers released by GC and by destroy()', { skip: typeof globalThis.gc !== 'function' && 'run with --expose-gc' }, async (t) => {
	if (skip) return t.skip(skip);
	const module = device.createShaderModule({ code: doubleWGSL });
	const pipeline = device.createComputePipeline({ compute: { module, entryPoint: 'main' }, layout: 'auto' });
	for (let round = 0; round < 10; round++) {
		for (let i = 0; i < 40; i++) {
			const storage = createBuffer(device, { size: 256, usage: BufferUsage.STORAGE | BufferUsage.COPY_DST });
			writeBuffer(device.queue, storage, 0, new Float32Array(64).fill(i));
			const group = device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: storage, offset: 0, size: 256 } }] });
			const texture = createTexture(device, { size: [16, 16], format: 'rgba8unorm', usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST });
			texture.createView(undefined);
			const encoder = device.createCommandEncoder(undefined);
			const pass = encoder.beginComputePass(undefined);
			pass.setPipeline(pipeline);
			pass.setBindGroup(0, group);
			pass.dispatchWorkgroups(1, 1, 1);
			if (i % 2) {
				end(pass);
				submit(device.queue, [finish(encoder)]);
				storage.destroy();
			} else {
				// left to the GC
				pass.end();
				device.queue.submit([encoder.finish(undefined)]);
			}
			void group;
		}
		globalThis.gc();
		await new Promise((resolve) => setImmediate(resolve));
	}
	await workDone(device.queue);
	globalThis.gc();
	// still working
	const out = await runCompute(pipeline, new Float32Array([21]));
	assert.deepEqual(Array.from(out), [42]);
});
