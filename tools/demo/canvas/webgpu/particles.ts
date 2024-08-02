import { Canvas, GPUCanvasContext, GPUDevice, GPUTexture, ImageAsset } from '@nativescript/canvas';
import { mat4, vec3 } from '../../wgpu-matrix/wgpu-matrix';
import { File, knownFolders } from '@nativescript/core';
// import { GUI } from 'dat.gui';

export async function run(canvas: Canvas) {
	const path = knownFolders.currentApp().path;
	const particleWGSL = File.fromPath(path + '/webgpu/shaders/particle.wgsl').readTextSync();
	const probabilityMapWGSL = File.fromPath(path + '/webgpu/shaders/probabilityMap.wgsl').readTextSync();

	const numParticles = 50000;
	const particlePositionOffset = 0;
	const particleColorOffset = 4 * 4;
	const particleInstanceByteSize =
		3 * 4 + // position
		1 * 4 + // lifetime
		4 * 4 + // color
		3 * 4 + // velocity
		1 * 4 + // padding
		0;

	const adapter = await navigator.gpu.requestAdapter();
	const device: GPUDevice = (await adapter.requestDevice()) as never;

	device.addEventListener('uncapturederror', (event) => {
		console.log(event);
		console.log(event.error);
	});

	const context: GPUCanvasContext = canvas.getContext('webgpu') as never;

	const devicePixelRatio = window.devicePixelRatio;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	const capabilities = (context as any).getCapabilities(adapter);
	const presentationFormat = navigator.gpu.getPreferredCanvasFormat(); //capabilities.format[0];
	const alphaMode = capabilities.alphaModes[0];

	context.configure({
		device,
		format: presentationFormat,
	});

	const particlesBuffer = device.createBuffer({
		size: numParticles * particleInstanceByteSize,
		usage: global.GPUBufferUsage.VERTEX | global.GPUBufferUsage.STORAGE,
	});

	const renderPipeline = device.createRenderPipeline({
		layout: 'auto',
		vertex: {
			module: device.createShaderModule({
				code: particleWGSL,
			}),
			buffers: [
				{
					// instanced particles buffer
					arrayStride: particleInstanceByteSize,
					stepMode: 'instance',
					attributes: [
						{
							// position
							shaderLocation: 0,
							offset: particlePositionOffset,
							format: 'float32x3',
						},
						{
							// color
							shaderLocation: 1,
							offset: particleColorOffset,
							format: 'float32x4',
						},
					],
				},
				{
					// quad vertex buffer
					arrayStride: 2 * 4, // vec2f
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
			entryPoint: 'vs_main',
		},
		fragment: {
			module: device.createShaderModule({
				code: particleWGSL,
			}),
			targets: [
				{
					format: presentationFormat,
					blend: {
						color: {
							srcFactor: 'src-alpha',
							dstFactor: 'one',
							operation: 'add',
						},
						alpha: {
							srcFactor: 'zero',
							dstFactor: 'one',
							operation: 'add',
						},
					},
				},
			],
			entryPoint: 'fs_main',
		},
		primitive: {
			topology: 'triangle-list',
		},

		depthStencil: {
			depthWriteEnabled: false,
			depthCompare: 'less',
			format: 'depth24plus',
		},
	});

	const depthTexture = device.createTexture({
		size: [canvas.width as number, canvas.height as number],
		format: 'depth24plus',
		usage: global.GPUTextureUsage.RENDER_ATTACHMENT,
	});

	const uniformBufferSize =
		4 * 4 * 4 + // modelViewProjectionMatrix : mat4x4f
		3 * 4 + // right : vec3f
		4 + // padding
		3 * 4 + // up : vec3f
		4 + // padding
		0;
	const uniformBuffer = device.createBuffer({
		size: uniformBufferSize,
		usage: global.GPUBufferUsage.UNIFORM | global.GPUBufferUsage.COPY_DST,
	});

	const uniformBindGroup = device.createBindGroup({
		layout: renderPipeline.getBindGroupLayout(0),
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
				clearValue: [0, 0, 0, 1],
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

	//////////////////////////////////////////////////////////////////////////////
	// Quad vertex buffer
	//////////////////////////////////////////////////////////////////////////////
	const quadVertexBuffer = device.createBuffer({
		size: 6 * 2 * 4, // 6x vec2f
		usage: global.GPUBufferUsage.VERTEX,
		mappedAtCreation: true,
	});
	// prettier-ignore
	const vertexData = [
      -1.0, -1.0, +1.0, -1.0, -1.0, +1.0, -1.0, +1.0, +1.0, -1.0, +1.0, +1.0,
    ];
	new Float32Array(quadVertexBuffer.getMappedRange()).set(vertexData);
	quadVertexBuffer.unmap();

	//////////////////////////////////////////////////////////////////////////////
	// Texture
	//////////////////////////////////////////////////////////////////////////////
	let texture: GPUTexture;
	let textureWidth = 1;
	let textureHeight = 1;
	let numMipLevels = 1;
	{
		//   const response = await fetch('../../assets/img/webgpu.png');
		//   const imageBitmap = await createImageBitmap(await response.blob());

		const imageBitmap = new ImageAsset();
		imageBitmap.fromFileSync('~/assets/file-assets/webgpu/webgpu.png');

		// Calculate number of mip levels required to generate the probability map
		while (textureWidth < imageBitmap.width || textureHeight < imageBitmap.height) {
			textureWidth *= 2;
			textureHeight *= 2;
			numMipLevels++;
		}
		texture = device.createTexture({
			size: [imageBitmap.width, imageBitmap.height, 1],
			mipLevelCount: numMipLevels,
			format: 'rgba8unorm',
			usage: global.GPUTextureUsage.TEXTURE_BINDING | global.GPUTextureUsage.STORAGE_BINDING | global.GPUTextureUsage.COPY_DST | global.GPUTextureUsage.RENDER_ATTACHMENT,
		});

		device.queue.copyExternalImageToTexture({ source: imageBitmap }, { texture: texture }, [imageBitmap.width, imageBitmap.height]);
	}

	//////////////////////////////////////////////////////////////////////////////
	// Probability map generation
	// The 0'th mip level of texture holds the color data and spawn-probability in
	// the alpha channel. The mip levels 1..N are generated to hold spawn
	// probabilities up to the top 1x1 mip level.
	//////////////////////////////////////////////////////////////////////////////
	{
		const probabilityMapImportLevelPipeline = device.createComputePipeline({
			layout: 'auto',
			compute: {
				module: device.createShaderModule({ code: probabilityMapWGSL }),
				entryPoint: 'import_level',
			},
		});
		const probabilityMapExportLevelPipeline = device.createComputePipeline({
			layout: 'auto',
			compute: {
				module: device.createShaderModule({ code: probabilityMapWGSL }),
				entryPoint: 'export_level',
			},
		});

		const probabilityMapUBOBufferSize =
			1 * 4 + // stride
			3 * 4 + // padding
			0;
		const probabilityMapUBOBuffer = device.createBuffer({
			size: probabilityMapUBOBufferSize,
			usage: global.GPUBufferUsage.UNIFORM | global.GPUBufferUsage.COPY_DST,
		});
		const buffer_a = device.createBuffer({
			size: textureWidth * textureHeight * 4,
			usage: global.GPUBufferUsage.STORAGE,
		});
		const buffer_b = device.createBuffer({
			size: textureWidth * textureHeight * 4,
			usage: global.GPUBufferUsage.STORAGE,
		});

		device.queue.writeBuffer(probabilityMapUBOBuffer, 0, new Int32Array([textureWidth]));
		const commandEncoder = device.createCommandEncoder();

		for (let level = 0; level < numMipLevels; level++) {
			const levelWidth = textureWidth >> level;
			const levelHeight = textureHeight >> level;
			const pipeline = level == 0 ? probabilityMapImportLevelPipeline.getBindGroupLayout(0) : probabilityMapExportLevelPipeline.getBindGroupLayout(0);

			const probabilityMapBindGroup = device.createBindGroup({
				layout: pipeline,
				entries: [
					{
						// ubo
						binding: 0,
						resource: { buffer: probabilityMapUBOBuffer },
					},
					{
						// buf_in
						binding: 1,
						resource: { buffer: level & 1 ? buffer_a : buffer_b },
					},
					{
						// buf_out
						binding: 2,
						resource: { buffer: level & 1 ? buffer_b : buffer_a },
					},
					{
						// tex_in / tex_out
						binding: 3,
						resource: texture.createView({
							format: 'rgba8unorm',
							dimension: '2d',
							baseMipLevel: level,
							mipLevelCount: 1,
						}),
					},
				],
			});

			if (level == 0) {
				const passEncoder = commandEncoder.beginComputePass();
				passEncoder.setPipeline(probabilityMapImportLevelPipeline);
				passEncoder.setBindGroup(0, probabilityMapBindGroup);
				passEncoder.dispatchWorkgroups(Math.ceil(levelWidth / 64), levelHeight);
				passEncoder.end();
			} else {
				const passEncoder = commandEncoder.beginComputePass();
				passEncoder.setPipeline(probabilityMapExportLevelPipeline);
				passEncoder.setBindGroup(0, probabilityMapBindGroup);
				passEncoder.dispatchWorkgroups(Math.ceil(levelWidth / 64), levelHeight);
				passEncoder.end();
			}
		}
		device.queue.submit([commandEncoder.finish()]);
	}

	//////////////////////////////////////////////////////////////////////////////
	// Simulation compute pipeline
	//////////////////////////////////////////////////////////////////////////////
	const simulationParams = {
		simulate: true,
		deltaTime: 0.04,
	};

	const simulationUBOBufferSize =
		1 * 4 + // deltaTime
		3 * 4 + // padding
		4 * 4 + // seed
		0;
	const simulationUBOBuffer = device.createBuffer({
		size: simulationUBOBufferSize,
		usage: global.GPUBufferUsage.UNIFORM | global.GPUBufferUsage.COPY_DST,
	});

	// const gui = new GUI();
	// gui.add(simulationParams, 'simulate');
	// gui.add(simulationParams, 'deltaTime');

	const computePipeline = device.createComputePipeline({
		layout: 'auto',
		compute: {
			module: device.createShaderModule({
				code: particleWGSL,
			}),
			entryPoint: 'simulate',
		},
	});
	const computeBindGroup = device.createBindGroup({
		layout: computePipeline.getBindGroupLayout(0),
		entries: [
			{
				binding: 0,
				resource: {
					buffer: simulationUBOBuffer,
				},
			},
			{
				binding: 1,
				resource: {
					buffer: particlesBuffer,
					offset: 0,
					size: numParticles * particleInstanceByteSize,
				},
			},
			{
				binding: 2,
				resource: texture.createView(),
			},
		],
	});

	const aspect = (canvas.width as number) / (canvas.height as number);
	const projection = mat4.perspective((2 * Math.PI) / 5, aspect, 1, 100.0);
	const view = mat4.create();
	const mvp = mat4.create();

	function frame() {
		device.queue.writeBuffer(
			simulationUBOBuffer,
			0,
			new Float32Array([
				simulationParams.simulate ? simulationParams.deltaTime : 0.0,
				0.0,
				0.0,
				0.0, // padding
				Math.random() * 100,
				Math.random() * 100, // seed.xy
				1 + Math.random(),
				1 + Math.random(), // seed.zw
			])
		);

		mat4.identity(view);
		mat4.translate(view, vec3.fromValues(0, 0, -3), view);
		mat4.rotateX(view, Math.PI * -0.2, view);
		mat4.multiply(projection, view, mvp);

		// prettier-ignore
		device.queue.writeBuffer(
        uniformBuffer,
        0,
        new Float32Array([
          // modelViewProjectionMatrix
          mvp[0], mvp[1], mvp[2], mvp[3],
          mvp[4], mvp[5], mvp[6], mvp[7],
          mvp[8], mvp[9], mvp[10], mvp[11],
          mvp[12], mvp[13], mvp[14], mvp[15],
    
          view[0], view[4], view[8], // right
    
          0, // padding
    
          view[1], view[5], view[9], // up
    
          0, // padding
        ])
      );
		const swapChainTexture = context.getCurrentTexture();
		// prettier-ignore
		renderPassDescriptor.colorAttachments[0].view = swapChainTexture.createView();

		const commandEncoder = device.createCommandEncoder();
		{
			const passEncoder = commandEncoder.beginComputePass();
			passEncoder.setPipeline(computePipeline);
			passEncoder.setBindGroup(0, computeBindGroup);
			passEncoder.dispatchWorkgroups(Math.ceil(numParticles / 64));
			passEncoder.end();
		}
		{
			const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor as never);
			passEncoder.setPipeline(renderPipeline);
			passEncoder.setBindGroup(0, uniformBindGroup);
			passEncoder.setVertexBuffer(0, particlesBuffer);
			passEncoder.setVertexBuffer(1, quadVertexBuffer);
			passEncoder.draw(6, numParticles, 0, 0);
			passEncoder.end();
		}

		device.queue.submit([commandEncoder.finish()]);

		(<any>context).presentSurface(swapChainTexture);

		requestAnimationFrame(frame);
	}
	requestAnimationFrame(frame);
}
