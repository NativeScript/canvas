import '@nativescript/macos-node-api';
import { createRequire } from 'node:module';

import utils from '../../../utils';

const { requestAnimationFrame } = utils;

// @ts-ignore
const require = createRequire(import.meta.url);

const { Canvas, GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUMapMode, GPUQuerySet, GPUTexture, GPUTextureUsage, ImageAsset, GPUBuffer } = require('../../../canvas-napi.darwin-arm64.node');

export async function run(canvas) {
	const adapter = await navigator.gpu?.requestAdapter();
	const hasTimestampQuery = adapter!.features.has('timestamp-query');
	const device = (await adapter?.requestDevice({
		requiredFeatures: hasTimestampQuery ? ['timestamp-query'] : [],
	})) as never;

	const context = canvas.getContext('webgpu');

	const devicePixelRatio = window.devicePixelRatio;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;
	const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	const spriteWGSLFile = import.meta.resolve('../../shaders/sprite.wgsl');
	const spriteWGSLData = NSData.dataWithContentsOfFile(spriteWGSLFile.replace('file://', ''));

	const spriteWGSL = NSString.alloc().initWithDataEncoding(spriteWGSLData, NSUTF8StringEncoding).toString();

	const updateSpritesWGSLFile = import.meta.resolve('../../shaders/updateSprites.wgsl');

	const updateSpritesWGSLData = NSData.dataWithContentsOfFile(updateSpritesWGSLFile.replace('file://', ''));

	const updateSpritesWGSL = NSString.alloc().initWithDataEncoding(updateSpritesWGSLData, NSUTF8StringEncoding).toString();

	let perfText = hasTimestampQuery ? 'Collecting samples...' : 'timestamp-query not supported on this device';

	context.configure({
		device,
		format: presentationFormat,
		presentMode: 'fifo',
	});

	const spriteShaderModule = device.createShaderModule({ code: spriteWGSL });

	const renderPipeline = device.createRenderPipeline({
		layout: 'auto',
		vertex: {
			module: spriteShaderModule,
			entryPoint: 'vert_main',
			buffers: [
				{
					// instanced particles buffer
					arrayStride: 4 * 4,
					stepMode: 'instance',
					attributes: [
						{
							// instance position
							shaderLocation: 0,
							offset: 0,
							format: 'float32x2',
						},
						{
							// instance velocity
							shaderLocation: 1,
							offset: 2 * 4,
							format: 'float32x2',
						},
					],
				},
				{
					// vertex buffer
					arrayStride: 2 * 4,
					stepMode: 'vertex',
					attributes: [
						{
							// vertex positions
							shaderLocation: 2,
							offset: 0,
							format: 'float32x2',
						},
					],
				},
			],
		},
		fragment: {
			module: spriteShaderModule,
			entryPoint: 'frag_main',
			targets: [
				{
					format: presentationFormat,
				},
			],
		},
		primitive: {
			topology: 'triangle-list',
		},
	});

	const module = device.createShaderModule({
		code: updateSpritesWGSL,
	});
	const computePipeline = device.createComputePipeline({
		layout: 'auto',
		compute: {
			module: module,
			entryPoint: 'main',
		},
	});

	const renderPassDescriptor = {
		colorAttachments: [
			{
				view: undefined, // Assigned later
				clearValue: [0, 0, 0, 1],
				loadOp: 'clear' as const,
				storeOp: 'store' as const,
			},
		],
	};

	const computePassDescriptor = {};

	/** Storage for timestamp query results */
	let querySet: typeof GPUQuerySet | undefined = undefined;
	/** Timestamps are resolved into this buffer */
	let resolveBuffer: typeof GPUBuffer | undefined = undefined;
	/** Pool of spare buffers for MAP_READing the timestamps back to CPU. A buffer
	 * is taken from the pool (if available) when a readback is needed, and placed
	 * back into the pool once the readback is done and it's unmapped. */
	const spareResultBuffers: (typeof GPUBuffer)[] = [];

	if (hasTimestampQuery) {
		querySet = device.createQuerySet({
			type: 'timestamp',
			count: 4,
		});
		resolveBuffer = device.createBuffer({
			size: 4 * BigInt64Array.BYTES_PER_ELEMENT,
			usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC,
		});
		(<any>computePassDescriptor).timestampWrites = {
			querySet: querySet as never,
			beginningOfPassWriteIndex: 0,
			endOfPassWriteIndex: 1,
		};
		(<any>renderPassDescriptor).timestampWrites = {
			querySet: querySet as never,
			beginningOfPassWriteIndex: 2,
			endOfPassWriteIndex: 3,
		};
	}

	// prettier-ignore
	const vertexBufferData = new Float32Array([
		-0.01, -0.02, 0.01,
		-0.02, 0.0, 0.02
	]);

	const spriteVertexBuffer = device.createBuffer({
		size: vertexBufferData.byteLength,
		usage: GPUBufferUsage.VERTEX,
		mappedAtCreation: true,
	});
	new Float32Array(spriteVertexBuffer.getMappedRange()).set(vertexBufferData);
	spriteVertexBuffer.unmap();

	const simParams = {
		deltaT: 0.04,
		rule1Distance: 0.1,
		rule2Distance: 0.025,
		rule3Distance: 0.025,
		rule1Scale: 0.02,
		rule2Scale: 0.05,
		rule3Scale: 0.005,
	};

	const simParamBufferSize = 7 * Float32Array.BYTES_PER_ELEMENT;
	const simParamBuffer = device.createBuffer({
		size: simParamBufferSize,
		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
	});

	function updateSimParams() {
		device.queue.writeBuffer(simParamBuffer, 0, new Float32Array([simParams.deltaT, simParams.rule1Distance, simParams.rule2Distance, simParams.rule3Distance, simParams.rule1Scale, simParams.rule2Scale, simParams.rule3Scale]));
	}

	// const gui = new GUI();
	updateSimParams();
	// Object.keys(simParams).forEach((k) => {
	//   const key = k as keyof typeof simParams;
	//   gui.add(simParams, key).onFinishChange(updateSimParams);
	// });

	const numParticles = global.isAndroid ? 750 : 1500;
	const initialParticleData = new Float32Array(numParticles * 4);
	for (let i = 0; i < numParticles; ++i) {
		initialParticleData[4 * i + 0] = 2 * (Math.random() - 0.5);
		initialParticleData[4 * i + 1] = 2 * (Math.random() - 0.5);
		initialParticleData[4 * i + 2] = 2 * (Math.random() - 0.5) * 0.1;
		initialParticleData[4 * i + 3] = 2 * (Math.random() - 0.5) * 0.1;
	}

	const particleBuffers: (typeof GPUBuffer)[] = new Array(2);
	const particleBindGroups: (typeof GPUBindGroup)[] = new Array(2);
	for (let i = 0; i < 2; ++i) {
		particleBuffers[i] = device.createBuffer({
			size: initialParticleData.byteLength,
			usage: GPUBufferUsage.VERTEX | GPUBufferUsage.STORAGE,
			mappedAtCreation: true,
		});
		new Float32Array(particleBuffers[i]!.getMappedRange()).set(initialParticleData);
		particleBuffers[i]!.unmap();
	}

	for (let i = 0; i < 2; ++i) {
		particleBindGroups[i] = device.createBindGroup({
			layout: computePipeline.getBindGroupLayout(0),
			entries: [
				{
					binding: 0,
					resource: {
						buffer: simParamBuffer,
					},
				},
				{
					binding: 1,
					resource: {
						buffer: particleBuffers[i] as never,
						offset: 0,
						size: initialParticleData.byteLength,
					},
				},
				{
					binding: 2,
					resource: {
						buffer: particleBuffers[(i + 1) % 2] as never,
						offset: 0,
						size: initialParticleData.byteLength,
					},
				},
			],
		}) as never;
	}

	let t = 0;
	let computePassDurationSum = 0;
	let renderPassDurationSum = 0;
	let timerSamples = 0;

	function frame() {
		const framebuffer = context.getCurrentTexture();

		renderPassDescriptor.colorAttachments[0].view = framebuffer.createView() as never;

		const commandEncoder = device.createCommandEncoder();
		{
			const passEncoder = commandEncoder.beginComputePass(computePassDescriptor as never);
			passEncoder.setPipeline(computePipeline);
			passEncoder.setBindGroup(0, particleBindGroups[t % 2] as never);
			passEncoder.dispatchWorkgroups(Math.ceil(numParticles / 64));
			passEncoder.end();
		}
		{
			const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor as never);
			passEncoder.setPipeline(renderPipeline);
			passEncoder.setVertexBuffer(0, particleBuffers[(t + 1) % 2] as never);
			passEncoder.setVertexBuffer(1, spriteVertexBuffer);
			passEncoder.draw(3, numParticles, 0, 0);
			passEncoder.end();
		}

		let resultBuffer: typeof GPUBuffer | undefined = undefined;
		if (hasTimestampQuery) {
			resultBuffer =
				spareResultBuffers.pop() ||
				device.createBuffer({
					size: 4 * BigInt64Array.BYTES_PER_ELEMENT,
					usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
				});
			commandEncoder.resolveQuerySet(querySet!, 0, 4, resolveBuffer as never, 0);
			commandEncoder.copyBufferToBuffer(resolveBuffer as never, 0, resultBuffer as never, 0, resultBuffer!.size);
		}

		device.queue.submit([commandEncoder.finish()]);

		context.presentSurface();

		if (hasTimestampQuery) {
			resultBuffer!.mapAsync(GPUMapMode.READ).then(() => {
				const times = new BigInt64Array(resultBuffer!.getMappedRange());
				const computePassDuration = Number(times[1]! - times[0]!);
				const renderPassDuration = Number(times[3]! - times[2]!);

				// In some cases the timestamps may wrap around and produce a negative
				// number as the GPU resets it's timings. These can safely be ignored.
				if (computePassDuration > 0 && renderPassDuration > 0) {
					computePassDurationSum += computePassDuration;
					renderPassDurationSum += renderPassDuration;
					timerSamples++;
				}
				resultBuffer!.unmap();

				// Periodically update the text for the timer stats
				const kNumTimerSamplesPerUpdate = 10;
				if (timerSamples >= kNumTimerSamplesPerUpdate) {
					const avgComputeMicroseconds = Math.round(computePassDurationSum / timerSamples / 1000);
					const avgRenderMicroseconds = Math.round(renderPassDurationSum / timerSamples / 1000);
					perfText = `\
avg compute pass duration: ${avgComputeMicroseconds}µs
avg render pass duration:  ${avgRenderMicroseconds}µs
spare readback buffers:    ${spareResultBuffers.length}`;
					computePassDurationSum = 0;
					renderPassDurationSum = 0;
					timerSamples = 0;
				}
				spareResultBuffers.push(resultBuffer!);
			});
		}

		++t;
		requestAnimationFrame(frame);
	}

	frame();
	// requestAnimationFrame(frame);
}
