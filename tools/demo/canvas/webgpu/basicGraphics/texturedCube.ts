import { Canvas, GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUTextureUsage, GPUTexture, ImageAsset, GPUAdapter } from '@nativescript/canvas';

import { mat4, vec3 } from '../../../wgpu-matrix/wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from '../meshes/cube';
import { knownFolders, File } from '@nativescript/core';

export async function run(canvas: Canvas) {
	const adapter = (await navigator.gpu?.requestAdapter()) as never as GPUAdapter;
	const device: GPUDevice = (await adapter?.requestDevice()) as never;

	const devicePixelRatio = window.devicePixelRatio;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	const context: GPUCanvasContext = canvas.getContext('webgpu') as never;

	const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
	const appPath = knownFolders.currentApp().path;
	const basicVertWGSLFile = File.fromPath(appPath + '/webgpu/shaders/basic.vert.wgsl');
	// readText async fails on android
	const basicVertWGSL = await basicVertWGSLFile.readText();

	const sampleTextureMixColorWGSLFile = File.fromPath(appPath + '/webgpu/shaders/sampleTextureMixColor.frag.wgsl');

	const sampleTextureMixColorWGSL = await sampleTextureMixColorWGSLFile.readText();

	context.configure({
		device,
		format: presentationFormat,
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

		(<any>context).presentSurface(texture);

		requestAnimationFrame(frame);
	}
	requestAnimationFrame(frame);
}
