import { Canvas, GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUTexture, GPUTextureUsage, ImageAsset, GPUBuffer, GPUShaderStage, GPUBindGroup } from '@nativescript/canvas';

import { mat4, vec3 } from '../../../../wgpu-matrix/wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from '../../meshes/cube';

import { File, knownFolders } from '@nativescript/core';

import { ArcballCamera, WASDCamera } from './camera';
import { createInputHandler } from './input';

export async function run(canvas: Canvas) {
	const inputHandler = createInputHandler(canvas);

	// The camera types
	const initialCameraPosition = vec3.create(3, 2, 5);
	const cameras = {
		arcball: new ArcballCamera({ position: initialCameraPosition }),
		WASD: new WASDCamera({ position: initialCameraPosition }),
	};

	// const gui = new GUI();

	// GUI parameters
	const params: { type: 'arcball' | 'WASD' } = {
		type: 'arcball',
	};

	const adapter = await navigator.gpu?.requestAdapter();
	const device: GPUDevice = (await adapter?.requestDevice()) as never;

	const devicePixelRatio = window.devicePixelRatio;

	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	const context = canvas.getContext('webgpu') as never as GPUCanvasContext;

	const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	const path = knownFolders.currentApp().path;

	const cubeWGSL = await File.fromPath(path + '/webgpu/shaders/cube.wgsl').readText();

	context.configure({
		device,
		format: presentationFormat,
	});

	const width = canvas.width as number;
	const height = canvas.height as number;

	// Create a vertex buffer from the cube data.
	const verticesBuffer = device.createBuffer({
		size: cubeVertexArray.byteLength,
		usage: GPUBufferUsage.VERTEX,
		mappedAtCreation: true,
	});
	new Float32Array(verticesBuffer.getMappedRange()).set(cubeVertexArray);
	verticesBuffer.unmap();

	const pipeline = device.createRenderPipeline({
		layout: 'auto',
		vertex: {
			module: device.createShaderModule({
				code: cubeWGSL,
			}),
			entryPoint: 'vertex_main',
			buffers: [
				{
					arrayStride: cubeVertexSize,
					attributes: [
						{
							// position
							shaderLocation: 0,
							offset: cubePositionOffset,
							format: 'float32x4',
						},
						{
							// uv
							shaderLocation: 1,
							offset: cubeUVOffset,
							format: 'float32x2',
						},
					],
				},
			],
		},
		fragment: {
			module: device.createShaderModule({
				code: cubeWGSL,
			}),
			entryPoint: 'fragment_main',
			targets: [
				{
					format: presentationFormat,
				},
			],
		},
		primitive: {
			topology: 'triangle-list',
			cullMode: 'back',
		},
		depthStencil: {
			depthWriteEnabled: true,
			depthCompare: 'less',
			format: 'depth24plus',
		},
	});

	const depthTexture = device.createTexture({
		size: [width, height],
		format: 'depth24plus',
		usage: GPUTextureUsage.RENDER_ATTACHMENT,
	});

	const uniformBufferSize = 4 * 16; // 4x4 matrix
	const uniformBuffer = device.createBuffer({
		size: uniformBufferSize,
		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
	});

	// Fetch the image and upload it into a GPUTexture.
	let cubeTexture: GPUTexture;
	{
		// const imageBitmap = await createImageBitmap(require('../../assets/img/Di-3d.png'));

		const imageBitmap = new ImageAsset();
		await imageBitmap.fromFile('~/assets/file-assets/webgpu/Di-3d.png');

		cubeTexture = device.createTexture({
			size: [imageBitmap.width, imageBitmap.height, 1],
			format: 'rgba8unorm',
			usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT,
		});
		device.queue.copyExternalImageToTexture({ source: imageBitmap }, { texture: cubeTexture }, [imageBitmap.width, imageBitmap.height]);
	}

	// Create a sampler with linear filtering for smooth interpolation.
	const sampler = device.createSampler({
		magFilter: 'linear',
		minFilter: 'linear',
	});

	const uniformBindGroup = device.createBindGroup({
		layout: pipeline.getBindGroupLayout(0),
		entries: [
			{
				binding: 0,
				resource: {
					buffer: uniformBuffer,
				},
			},
			{
				binding: 1,
				resource: sampler,
			},
			{
				binding: 2,
				resource: cubeTexture.createView(),
			},
		],
	});

	const renderPassDescriptor = {
		colorAttachments: [
			{
				view: undefined, // Assigned later

				clearValue: [0.5, 0.5, 0.5, 1.0],
				loadOp: 'clear',
				storeOp: 'store',
			},
		],
		depthStencilAttachment: {
			view: depthTexture.createView(),

			depthClearValue: 1.0,
			depthLoadOp: 'clear',
			depthStoreOp: 'store',
		},
	};

	const aspect = width / height;
	const projectionMatrix = mat4.perspective((2 * Math.PI) / 5, aspect, 1, 100.0);
	const modelViewProjectionMatrix = mat4.create();

	function getModelViewProjectionMatrix(deltaTime: number) {
		const camera = cameras[params.type];
		const viewMatrix = camera.update(deltaTime, inputHandler);
		mat4.multiply(projectionMatrix, viewMatrix, modelViewProjectionMatrix);
		return modelViewProjectionMatrix;
	}

	let lastFrameMS = Date.now();

	function frame() {
		const framebuffer = context.getCurrentTexture();
		if (!framebuffer) {
			requestAnimationFrame(frame);
			return;
		}
		const now = Date.now();
		const deltaTime = (now - lastFrameMS) / 1000;
		lastFrameMS = now;

		const modelViewProjection = getModelViewProjectionMatrix(deltaTime);
		device.queue.writeBuffer(uniformBuffer, 0, modelViewProjection.buffer, modelViewProjection.byteOffset, modelViewProjection.byteLength);
		renderPassDescriptor.colorAttachments[0].view = framebuffer.createView();

		const commandEncoder = device.createCommandEncoder();
		const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor as never);
		passEncoder.setPipeline(pipeline);
		passEncoder.setBindGroup(0, uniformBindGroup);
		passEncoder.setVertexBuffer(0, verticesBuffer);
		passEncoder.draw(cubeVertexCount);
		passEncoder.end();
		device.queue.submit([commandEncoder.finish()]);

		context.presentSurface(framebuffer);
		requestAnimationFrame(frame);
	}

	requestAnimationFrame(frame);
}
