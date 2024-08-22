import { Canvas, GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUTextureUsage } from '@nativescript/canvas';
import { mat4, vec3 } from '../../../wgpu-matrix/wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from '../meshes/cube';
import { knownFolders, File } from '@nativescript/core';

export async function run(canvas: Canvas) {
	const path = knownFolders.currentApp().path;

	const basicVertWGSL = File.fromPath(path + '/webgpu/shaders/basic.vert.wgsl').readTextSync();
	const vertexPositionColorWGSL = File.fromPath(path + '/webgpu/shaders/vertexPositionColor.frag.wgsl').readTextSync();

	const adapter = await navigator.gpu?.requestAdapter();
	const device = (await adapter?.requestDevice()) as never as GPUDevice;

	const devicePixelRatio = window.devicePixelRatio;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	const context = canvas.getContext('webgpu') as never as GPUCanvasContext;

	const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	context.configure({
		device,
		format: presentationFormat,
	});

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
		size: [canvas.width as number, canvas.height as number],
		format: 'depth24plus',
		usage: GPUTextureUsage.RENDER_ATTACHMENT,
	});

	const matrixSize = 4 * 16; // 4x4 matrix
	const offset = 256; // uniformBindGroup offset must be 256-byte aligned
	const uniformBufferSize = offset + matrixSize;

	const uniformBuffer = device.createBuffer({
		size: uniformBufferSize,
		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
	});

	const uniformBindGroup1 = device.createBindGroup({
		layout: pipeline.getBindGroupLayout(0),
		entries: [
			{
				binding: 0,
				resource: {
					buffer: uniformBuffer,
					offset: 0,
					size: matrixSize,
				},
			},
		],
	});

	const uniformBindGroup2 = device.createBindGroup({
		layout: pipeline.getBindGroupLayout(0),
		entries: [
			{
				binding: 0,
				resource: {
					buffer: uniformBuffer,
					offset: offset,
					size: matrixSize,
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
	const projectionMatrix = mat4.perspective((2 * Math.PI) / 5, aspect, 1, 100.0);

	const modelMatrix1 = mat4.translation(vec3.create(-2, 0, 0));
	const modelMatrix2 = mat4.translation(vec3.create(2, 0, 0));
	const modelViewProjectionMatrix1 = mat4.create();
	const modelViewProjectionMatrix2 = mat4.create();
	const viewMatrix = mat4.translation(vec3.fromValues(0, 0, -7));

	const tmpMat41 = mat4.create();
	const tmpMat42 = mat4.create();

	function updateTransformationMatrix() {
		const now = Date.now() / 1000;

		mat4.rotate(modelMatrix1, vec3.fromValues(Math.sin(now), Math.cos(now), 0), 1, tmpMat41);
		mat4.rotate(modelMatrix2, vec3.fromValues(Math.cos(now), Math.sin(now), 0), 1, tmpMat42);

		mat4.multiply(viewMatrix, tmpMat41, modelViewProjectionMatrix1);
		mat4.multiply(projectionMatrix, modelViewProjectionMatrix1, modelViewProjectionMatrix1);
		mat4.multiply(viewMatrix, tmpMat42, modelViewProjectionMatrix2);
		mat4.multiply(projectionMatrix, modelViewProjectionMatrix2, modelViewProjectionMatrix2);
	}

	function frame() {
		updateTransformationMatrix();
		device.queue.writeBuffer(uniformBuffer, 0, modelViewProjectionMatrix1.buffer, modelViewProjectionMatrix1.byteOffset, modelViewProjectionMatrix1.byteLength);
		device.queue.writeBuffer(uniformBuffer, offset, modelViewProjectionMatrix2.buffer, modelViewProjectionMatrix2.byteOffset, modelViewProjectionMatrix2.byteLength);

		const texture = context.getCurrentTexture();
		renderPassDescriptor.colorAttachments[0].view = texture.createView();

		const commandEncoder = device.createCommandEncoder();
		const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor as never);
		passEncoder.setPipeline(pipeline);
		passEncoder.setVertexBuffer(0, verticesBuffer);

		// Bind the bind group (with the transformation matrix) for
		// each cube, and draw.
		passEncoder.setBindGroup(0, uniformBindGroup1);
		passEncoder.draw(cubeVertexCount);

		passEncoder.setBindGroup(0, uniformBindGroup2);
		passEncoder.draw(cubeVertexCount);

		passEncoder.end();
		device.queue.submit([commandEncoder.finish()]);

		context.presentSurface();

		requestAnimationFrame(frame);
	}
	requestAnimationFrame(frame);
}
