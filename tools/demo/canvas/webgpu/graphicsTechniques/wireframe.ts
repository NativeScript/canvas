import { Canvas, GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUTexture, GPUTextureUsage, ImageAsset, GPUBuffer, GPUShaderStage, GPUBindGroup } from '@nativescript/canvas';

import { mat4, mat3 } from '../../../wgpu-matrix/wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from '../meshes/cube';

import { File, knownFolders } from '@nativescript/core';

import { modelData } from '../models';
import { randElement, randColor } from '../utils';

export async function run(canvas: Canvas) {
	const adapter = await navigator.gpu?.requestAdapter();
	const device: GPUDevice = (await adapter?.requestDevice()) as never;

	const devicePixelRatio = window.devicePixelRatio;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	const context = canvas.getContext('webgpu') as never as GPUCanvasContext;

	const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	const path = knownFolders.currentApp().path;

	const solidColorLitWGSL = await File.fromPath(path + '/webgpu/shaders/solidColorLit.wgsl').readText();
	const wireframeWGSL = await File.fromPath(path + '/webgpu/shaders/wireframe.wgsl').readText();

	context.configure({
		device,
		format: presentationFormat,
	});

	const settings = {
		barycentricCoordinatesBased: true,
		thickness: 2,
		alphaThreshold: 0.5,
		animate: true,
		lines: true,
		depthBias: 1,
		depthBiasSlopeScale: 0.5,
		models: false,
	};

	type TypedArrayView = Float32Array | Uint32Array;

	function createBufferWithData(device: GPUDevice, data: TypedArrayView, usage: number) {
		const buffer = device.createBuffer({
			size: data.byteLength,
			usage,
		});
		device.queue.writeBuffer(buffer, 0, data);
		return buffer;
	}

	type Model = {
		vertexBuffer: GPUBuffer;
		indexBuffer: GPUBuffer;
		indexFormat: 'uint16' | 'uint32';
		vertexCount: number;
	};

	function createVertexAndIndexBuffer(device: GPUDevice, { vertices, indices }: { vertices: Float32Array; indices: Uint32Array }): Model {
		const vertexBuffer = createBufferWithData(device, vertices, global.GPUBufferUsage.VERTEX | global.GPUBufferUsage.STORAGE | global.GPUBufferUsage.COPY_DST);
		const indexBuffer = createBufferWithData(device, indices, global.GPUBufferUsage.INDEX | global.GPUBufferUsage.STORAGE | global.GPUBufferUsage.COPY_DST);
		return {
			vertexBuffer,
			indexBuffer,
			indexFormat: 'uint32',
			vertexCount: indices.length,
		};
	}

	const depthFormat = 'depth24plus';

	const models = Object.values(modelData).map((data) => createVertexAndIndexBuffer(device, data));

	const litModule = device.createShaderModule({
		code: solidColorLitWGSL,
	});

	const wireframeModule = device.createShaderModule({
		code: wireframeWGSL,
	});

	const litBindGroupLayout = device.createBindGroupLayout({
		label: 'lit bind group layout',
		entries: [
			{
				binding: 0,
				visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
				buffer: {},
			},
		],
	});

	let litPipeline;
	function rebuildLitPipeline() {
		litPipeline = device.createRenderPipeline({
			label: 'lit pipeline',
			layout: device.createPipelineLayout({
				bindGroupLayouts: [litBindGroupLayout],
			}),
			vertex: {
				module: litModule,
				buffers: [
					{
						arrayStride: 6 * 4, // position, normal
						attributes: [
							{
								// position
								shaderLocation: 0,
								offset: 0,
								format: 'float32x3',
							},
							{
								// normal
								shaderLocation: 1,
								offset: 3 * 4,
								format: 'float32x3',
							},
						],
					},
				],
				entryPoint: 'vs',
			},
			fragment: {
				module: litModule,
				targets: [{ format: presentationFormat }],
				entryPoint: 'fs',
			},
			primitive: {
				cullMode: 'back',
			},
			depthStencil: {
				depthWriteEnabled: true,
				depthCompare: 'less',
				// Applying a depth bias can prevent aliasing from z-fighting with the
				// wireframe lines. The depth bias has to be applied to the lit meshes
				// rather that the wireframe because depthBias isn't considered when
				// drawing line or point primitives.
				depthBias: settings.depthBias,
				depthBiasSlopeScale: settings.depthBiasSlopeScale,
				format: depthFormat,
			},
		});
	}
	rebuildLitPipeline();

	const wireframePipeline = device.createRenderPipeline({
		label: 'wireframe pipeline',
		layout: 'auto',
		vertex: {
			module: wireframeModule,
			entryPoint: 'vsIndexedU32',
		},
		fragment: {
			module: wireframeModule,
			entryPoint: 'fs',
			targets: [{ format: presentationFormat }],
		},
		primitive: {
			topology: 'line-list',
		},
		depthStencil: {
			depthWriteEnabled: true,
			depthCompare: 'less-equal',
			format: depthFormat,
		},
	});

	const barycentricCoordinatesBasedWireframePipeline = device.createRenderPipeline({
		label: 'barycentric coordinates based wireframe pipeline',
		layout: 'auto',
		vertex: {
			module: wireframeModule,
			entryPoint: 'vsIndexedU32BarycentricCoordinateBasedLines',
		},
		fragment: {
			module: wireframeModule,
			entryPoint: 'fsBarycentricCoordinateBasedLines',
			targets: [
				{
					format: presentationFormat,
					blend: {
						color: {
							srcFactor: 'one',
							dstFactor: 'one-minus-src-alpha',
						},
						alpha: {
							srcFactor: 'one',
							dstFactor: 'one-minus-src-alpha',
						},
					},
				},
			],
		},
		primitive: {
			topology: 'triangle-list',
		},
		depthStencil: {
			depthWriteEnabled: true,
			depthCompare: 'less-equal',
			format: depthFormat,
		},
	});

	type ObjectInfo = {
		worldViewProjectionMatrixValue: Float32Array;
		worldMatrixValue: Float32Array;
		uniformValues: Float32Array;
		uniformBuffer: GPUBuffer;
		lineUniformValues: Float32Array;
		lineUniformBuffer: GPUBuffer;
		litBindGroup: GPUBindGroup;
		wireframeBindGroups: GPUBindGroup[];
		model: Model;
	};

	const objectInfos: ObjectInfo[] = [];

	const numObjects = 200;
	for (let i = 0; i < numObjects; ++i) {
		// Make a uniform buffer and type array views
		// for our uniforms.
		const uniformValues = new Float32Array(16 + 16 + 4);
		const uniformBuffer = device.createBuffer({
			size: uniformValues.byteLength,
			usage: global.GPUBufferUsage.UNIFORM | global.GPUBufferUsage.COPY_DST,
		});
		const kWorldViewProjectionMatrixOffset = 0;
		const kWorldMatrixOffset = 16;
		const kColorOffset = 32;
		const worldViewProjectionMatrixValue = uniformValues.subarray(kWorldViewProjectionMatrixOffset, kWorldViewProjectionMatrixOffset + 16);
		const worldMatrixValue = uniformValues.subarray(kWorldMatrixOffset, kWorldMatrixOffset + 15);
		const colorValue = uniformValues.subarray(kColorOffset, kColorOffset + 4);
		colorValue.set(randColor());

		const model = randElement(models);

		// Make a bind group for this uniform
		const litBindGroup = device.createBindGroup({
			layout: litBindGroupLayout,
			entries: [{ binding: 0, resource: { buffer: uniformBuffer } }],
		});

		// Note: We're making one lineUniformBuffer per object.
		// This is only because stride might be different per object.
		// In this sample stride is the same across all objects so
		// we could have made just a single shared uniform buffer for
		// these settings.
		const lineUniformValues = new Float32Array(3 + 1);
		const lineUniformValuesAsU32 = new Uint32Array(lineUniformValues.buffer);
		const lineUniformBuffer = device.createBuffer({
			size: lineUniformValues.byteLength,
			usage: global.GPUBufferUsage.UNIFORM | global.GPUBufferUsage.COPY_DST,
		});
		lineUniformValuesAsU32[0] = 6; // the array stride for positions for this model.

		// We're creating 2 bindGroups, one for each pipeline.
		// We could create just one since they are identical. To do
		// so we'd have to manually create a bindGroupLayout.
		const wireframeBindGroup = device.createBindGroup({
			layout: wireframePipeline.getBindGroupLayout(0),
			entries: [
				{ binding: 0, resource: { buffer: uniformBuffer } },
				{ binding: 1, resource: { buffer: model.vertexBuffer } },
				{ binding: 2, resource: { buffer: model.indexBuffer } },
				{ binding: 3, resource: { buffer: lineUniformBuffer } },
			],
		});

		const barycentricCoordinatesBasedWireframeBindGroup = device.createBindGroup({
			layout: barycentricCoordinatesBasedWireframePipeline.getBindGroupLayout(0),
			entries: [
				{ binding: 0, resource: { buffer: uniformBuffer } },
				{ binding: 1, resource: { buffer: model.vertexBuffer } },
				{ binding: 2, resource: { buffer: model.indexBuffer } },
				{ binding: 3, resource: { buffer: lineUniformBuffer } },
			],
		});

		objectInfos.push({
			worldViewProjectionMatrixValue,
			worldMatrixValue,
			uniformValues,
			uniformBuffer,
			lineUniformValues,
			lineUniformBuffer,
			litBindGroup,
			wireframeBindGroups: [wireframeBindGroup, barycentricCoordinatesBasedWireframeBindGroup],
			model,
		});
	}

	const renderPassDescriptor = {
		label: 'our basic canvas renderPass',
		colorAttachments: [
			{
				view: undefined, // <- to be filled out when we render
				clearValue: [0.3, 0.3, 0.3, 1],
				loadOp: 'clear',
				storeOp: 'store',
			},
		],
		depthStencilAttachment: {
			view: undefined, // <- to be filled out when we render
			depthClearValue: 1.0,
			depthLoadOp: 'clear',
			depthStoreOp: 'store',
		},
	};

	function updateThickness() {
		objectInfos.forEach(({ lineUniformBuffer, lineUniformValues }) => {
			lineUniformValues[1] = settings.thickness;
			lineUniformValues[2] = settings.alphaThreshold;
			device.queue.writeBuffer(lineUniformBuffer, 0, lineUniformValues);
		});
	}
	updateThickness();

	let depthTexture: GPUTexture | undefined;

	let time = 0.0;
	function render(ts: number) {
		if (settings.animate) {
			time = ts * 0.001; // convert to seconds;
		}

		// Get the current texture from the canvas context and
		// set it as the texture to render to.
		const canvasTexture = context.getCurrentTexture();
		renderPassDescriptor.colorAttachments[0].view = canvasTexture.createView();

		// If we don't have a depth texture OR if its size is different
		// from the canvasTexture when make a new depth texture

		if (!depthTexture || depthTexture.width !== canvasTexture.width || depthTexture.height !== canvasTexture.height) {
			if (depthTexture) {
				depthTexture.destroy();
			}
			depthTexture = device.createTexture({
				size: [canvasTexture.width, canvasTexture.height],
				format: 'depth24plus',
				usage: GPUTextureUsage.RENDER_ATTACHMENT,
			});
		}

		renderPassDescriptor.depthStencilAttachment.view = depthTexture.createView();

		const fov = (60 * Math.PI) / 180;
		const aspect = (canvas.width as number) / (canvas.height as number);
		const projection = mat4.perspective(fov, aspect, 0.1, 1000);

		const view = mat4.lookAt(
			[-300, 0, 300], // eye
			[0, 0, 0], // target
			[0, 1, 0] // up
		);

		const viewProjection = mat4.multiply(projection, view);

		// make a command encoder to start encoding commands
		const encoder = device.createCommandEncoder();

		// make a render pass encoder to encode render specific commands
		const pass = encoder.beginRenderPass(renderPassDescriptor as never);
		pass.setPipeline(litPipeline);

		objectInfos.forEach(({ uniformBuffer, uniformValues, worldViewProjectionMatrixValue, worldMatrixValue, litBindGroup, model: { vertexBuffer, indexBuffer, indexFormat, vertexCount } }, i) => {
			const world = mat4.identity();
			mat4.translate(world, [0, 0, Math.sin(i * 3.721 + time * 0.1) * 200], world);
			mat4.rotateX(world, i * 4.567, world);
			mat4.rotateY(world, i * 2.967, world);
			mat4.translate(world, [0, 0, Math.sin(i * 9.721 + time * 0.1) * 200], world);
			mat4.rotateX(world, time * 0.53 + i, world);

			mat4.multiply(viewProjection, world, worldViewProjectionMatrixValue);
			mat3.fromMat4(world, worldMatrixValue);

			// Upload our uniform values.
			device.queue.writeBuffer(uniformBuffer, 0, uniformValues);

			if (settings.models) {
				pass.setVertexBuffer(0, vertexBuffer);
				pass.setIndexBuffer(indexBuffer, indexFormat);
				pass.setBindGroup(0, litBindGroup);
				pass.drawIndexed(vertexCount);
			}
		});

		if (settings.lines) {
			// Note: If we're using the line-list based pipeline then we need to
			// multiply the vertex count by 2 since we need to emit 6 vertices
			// for each triangle (3 edges).
			const [bindGroupNdx, countMult, pipeline] = settings.barycentricCoordinatesBased ? [1, 1, barycentricCoordinatesBasedWireframePipeline] : [0, 2, wireframePipeline];
			pass.setPipeline(pipeline);
			objectInfos.forEach(({ wireframeBindGroups, model: { vertexCount } }) => {
				pass.setBindGroup(0, wireframeBindGroups[bindGroupNdx]);
				pass.draw(vertexCount * countMult);
			});
		}

		pass.end();

		const commandBuffer = encoder.finish();
		device.queue.submit([commandBuffer]);

		context.presentSurface(canvasTexture);

		requestAnimationFrame(render);
	}

	requestAnimationFrame(render);
}
