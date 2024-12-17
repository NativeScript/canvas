import '@nativescript/macos-node-api';
import { createRequire } from 'node:module';

import utils from '../../utils';
const { requestAnimationFrame, cancelAnimationFrame } = utils;

// @ts-ignore
const require = createRequire(import.meta.url);

const { GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUTextureUsage, GPUTexture, ImageAsset, GPUAdapter } = require('../../canvas-napi.darwin-arm64.node');

import { mat4, vec3 } from 'wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from '../meshes/cube';

export async function run(canvas) {
	const adapter = await navigator.gpu?.requestAdapter();
	const device = await adapter?.requestDevice();

	const devicePixelRatio = window.devicePixelRatio;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	const context = canvas.getContext('webgpu') as never;

	const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	const basicVertWGSLFile = import.meta.resolve('../shaders/basic.vert.wgsl');
	let url = NSURL.alloc().initWithString(basicVertWGSLFile);

	let data = NSData.dataWithContentsOfFile(url.path);
	const basicVertWGSL = NSString.alloc().initWithDataEncoding(data, NSUTF8StringEncoding).toString();

	const sampleTextureMixColorWGSLFile = import.meta.resolve('../shaders/sampleTextureMixColor.frag.wgsl');

	url = NSURL.alloc().initWithString(sampleTextureMixColorWGSLFile);

	data = NSData.dataWithContentsOfFile(url.path);
	const sampleTextureMixColorWGSL = NSString.alloc().initWithDataEncoding(data, NSUTF8StringEncoding).toString();

	context.configure({
		device,
		format: presentationFormat,
		presentMode: 'fifo',
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
				code: sampleTextureMixColorWGSL,
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

	const uniformBufferSize = 4 * 16; // 4x4 matrix
	const uniformBuffer = device.createBuffer({
		size: uniformBufferSize,
		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
	});

	// Fetch the image and upload it into a GPUTexture.
	let cubeTexture: GPUTexture;
	{
		// const response = await fetch('../../assets/img/Di-3d.png');
		// const imageBitmap = await createImageBitmap(await response.blob());

		const imageBitmap = new ImageAsset();
		const di_3d = import.meta.resolve('../assets/Di-3d.png');
		const url = new URL(di_3d);
		await imageBitmap.fromFile(url.pathname);

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

	const desc = {
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
	};

	const uniformBindGroup = device.createBindGroup(desc);

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
	const modelViewProjectionMatrix = mat4.create();

	function getTransformationMatrix() {
		const viewMatrix = mat4.identity();
		mat4.translate(viewMatrix, vec3.fromValues(0, 0, -4), viewMatrix);
		const now = Date.now() / 1000;
		mat4.rotate(viewMatrix, vec3.fromValues(Math.sin(now), Math.cos(now), 0), 1, viewMatrix);

		mat4.multiply(projectionMatrix, viewMatrix, modelViewProjectionMatrix);

		return modelViewProjectionMatrix;
	}

	function frame() {
		const transformationMatrix = getTransformationMatrix();
		device.queue.writeBuffer(uniformBuffer, 0, transformationMatrix.buffer, transformationMatrix.byteOffset, transformationMatrix.byteLength);
		const texture = context.getCurrentTexture();
		if (!texture) {
			requestAnimationFrame(frame);
			return;
		}

		renderPassDescriptor.colorAttachments[0].view = texture.createView();

		const commandEncoder = device.createCommandEncoder();
		const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor as never);
		passEncoder.setPipeline(pipeline);
		passEncoder.setBindGroup(0, uniformBindGroup);
		passEncoder.setVertexBuffer(0, verticesBuffer);
		passEncoder.draw(cubeVertexCount);
		passEncoder.end();
		device.queue.submit([commandEncoder.finish()]);

		context.presentSurface();

		requestAnimationFrame(frame);
	}
	requestAnimationFrame(frame);
}