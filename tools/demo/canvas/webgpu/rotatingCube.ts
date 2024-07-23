import * as wgpuMatrix from '../../wgpu-matrix/wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from './meshes/cube';
import { File, knownFolders } from '@nativescript/core';

import { Canvas } from '@nativescript/canvas';
import type { GPUDevice } from '@nativescript/canvas';

export async function run(canvas: Canvas) {
	const adapter = await navigator.gpu.requestAdapter();
	const device: GPUDevice = (await adapter.requestDevice()) as never;

	const context = canvas.getContext('webgpu');

	const devicePixelRatio = window.devicePixelRatio;
	// canvas.width = canvas.clientWidth * devicePixelRatio;
	// canvas.height = canvas.clientHeight * devicePixelRatio;

	const capabilities = (context as any).getCapabilities(adapter);
	const presentationFormat = navigator.gpu.getPreferredCanvasFormat(); //capabilities.format[0];
	const alphaMode = capabilities.alphaModes[0];
	const presentModes = capabilities.presentModes[0];

	const appPath = knownFolders.currentApp().path;
	const basicVertWGSLFile = File.fromPath(appPath + '/webgpu/shaders/basic.vert.wgsl');

	// readText async fails on android
	const basicVertWGSL = basicVertWGSLFile.readTextSync();
	const vertexPositionColorWGSLFile = File.fromPath(appPath + '/webgpu/shaders/vertexPositionColor.frag.wgsl');
	const vertexPositionColorWGSL = vertexPositionColorWGSLFile.readTextSync();

	context.configure({
		device,
		format: presentationFormat
	});
	// Create a vertex buffer from the cube data.
	const verticesBuffer = device.createBuffer({
		size: cubeVertexArray.byteLength,
		usage: global.GPUBufferUsage.VERTEX,
		mappedAtCreation: true,
	});
	new Float32Array(verticesBuffer.getMappedRange()).set(cubeVertexArray);

	verticesBuffer.unmap();

	const pipeline = device.createRenderPipeline({
		layout: 'auto',
		vertex: {
			module: device.createShaderModule({
				code: basicVertWGSL,
			}),
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
			entryPoint: 'main',
		},
		fragment: {
			module: device.createShaderModule({
				code: vertexPositionColorWGSL,
			}),
			targets: [
				{
					format: presentationFormat,
				},
			],
			entryPoint: 'main',
		},
		primitive: {
			topology: 'triangle-list',

			// Backface culling since the cube is solid piece of geometry.
			// Faces pointing away from the camera will be occluded by faces
			// pointing toward the camera.
			cullMode: 'back',
		},

		// Enable depth testing so that the fragment closest to the camera
		// is rendered in front.
		depthStencil: {
			depthWriteEnabled: true,
			depthCompare: 'less',
			format: 'depth24plus',
		},
	});

	const depthTexture = device.createTexture({
		size: [(canvas.width as number) * devicePixelRatio, (canvas.height as number) * devicePixelRatio],
		format: 'depth24plus',
		usage: global.GPUTextureUsage.RENDER_ATTACHMENT,
	});

	const uniformBufferSize = 4 * 16; // 4x4 matrix
	const uniformBuffer = device.createBuffer({
		size: uniformBufferSize,
		usage: global.GPUBufferUsage.UNIFORM | global.GPUBufferUsage.COPY_DST,
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

	const aspect = (canvas.width as number) / (canvas.height as number);
	const projectionMatrix = wgpuMatrix.mat4.perspective((2 * Math.PI) / 5, aspect, 1, 100.0);
	const modelViewProjectionMatrix = wgpuMatrix.mat4.create();

	function getTransformationMatrix() {
		const viewMatrix = wgpuMatrix.mat4.identity();
		wgpuMatrix.mat4.translate(viewMatrix, wgpuMatrix.vec3.fromValues(0, 0, -4), viewMatrix);
		const now = Date.now() / 1000;
		wgpuMatrix.mat4.rotate(viewMatrix, wgpuMatrix.vec3.fromValues(Math.sin(now), Math.cos(now), 0), 1, viewMatrix);

		wgpuMatrix.mat4.multiply(projectionMatrix, viewMatrix, modelViewProjectionMatrix);

		return modelViewProjectionMatrix;
	}

	function frame() {
		const framebuffer = context.getCurrentTexture();
		const transformationMatrix = getTransformationMatrix();
		device.queue.writeBuffer(uniformBuffer, 0, transformationMatrix.buffer, transformationMatrix.byteOffset, transformationMatrix.byteLength);
		renderPassDescriptor.colorAttachments[0].view = framebuffer.createView();

		const commandEncoder = device.createCommandEncoder();
		const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor as never);
		passEncoder.setPipeline(pipeline);
		passEncoder.setBindGroup(0, uniformBindGroup);
		passEncoder.setVertexBuffer(0, verticesBuffer);
		passEncoder.draw(cubeVertexCount);
		passEncoder.end();
		device.queue.submit([commandEncoder.finish()]);
		(<any>context).presentSurface(framebuffer);

		requestAnimationFrame(frame);
	}
	requestAnimationFrame(frame);
}
