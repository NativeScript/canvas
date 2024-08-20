import { Canvas, GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUTexture, GPUTextureUsage, ImageAsset, GPUBuffer, GPUShaderStage, GPUBindGroup } from '@nativescript/canvas';

import { mat4, mat3 } from '../../../wgpu-matrix/wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from '../meshes/cube';

import { File, knownFolders } from '@nativescript/core';

import { modelData } from '../models';
import { randElement, randColor } from '../utils';
import { GPUAddressMode, GPUFilterMode, GPUTextureFormat } from 'three/examples/jsm/renderers/webgpu/utils/WebGPUConstants';

import { HexWaterMesh } from './pointGen';
import { openSimplexNoise } from './simplexNoise';

const FRAC_PI_4 = Math.PI / 2;

///
/// Radius of the terrain.
///
/// Changing this value will change the size of the
/// water and terrain. Note however, that changes to
/// this value will require modification of the time
/// scale in the `render` method below.
///
const SIZE = 29.0;

///
/// Location of the camera.
/// Location of light is in terrain/water shaders.
///
const CAMERA = mat3.create(-200.0, 70.0, 200.0);

const Y = mat3.create(0.0, 1.0, 0.0);

const ZERO = mat3.create(0.0, 0.0, 0.0);

interface Matrices {
	view: Float32Array;
	flipped_view: Float32Array;
	projection: Float32Array;
}

interface TerrainUniforms {
	view_projection: Float32Array;
	clipping_plane: Float32Array;
}

interface WaterUniforms {
	view: Float32Array;
	projection: Float32Array;
	time_size_width: Float32Array;
	height: Float32Array;
}

interface Uniforms {
	terrain_normal: TerrainUniforms;
	terrain_flipped: TerrainUniforms;
	water: WaterUniforms;
}

class Example {
	water_vertex_buf: GPUBuffer;
	water_vertex_count: number;
	water_bind_group_layout: GPUBindGroupLayout;
	water_bind_group: GPUBindGroup;
	water_uniform_buf: GPUBuffer;
	water_pipeline: GPURenderPipeline;

	terrain_vertex_buf: GPUBuffer;
	terrain_vertex_count?: number;
	terrain_normal_bind_group: GPUBindGroup;
	terrain_normal_uniform_buf: GPUBuffer;
	///
	/// Contains uniform variables where the camera
	/// has been placed underwater.
	///
	terrain_flipped_uniform_buf: GPUBuffer;
	terrain_pipeline: GPURenderPipeline;

	/// A render bundle for drawing the terrain.
	///
	/// This isn't really necessary, but it does make sure we have at
	/// least one use of `RenderBundleEncoder::set_bind_group` among
	/// the examples.
	terrain_bundle: GPURenderBundle;

	reflect_view: GPUTextureView;

	depth_buffer: GPUTextureView;

	current_frame: number;

	///
	/// Used to prevent issues when rendering after
	/// minimizing the window.
	///
	active?: number;

	///
	/// Creates the view matrices, and the corrected projection matrix.
	///
	static generate_matrices(aspect_ratio: number): Matrices {
		let projection = mat4.perspective(FRAC_PI_4, aspect_ratio, 10.0, 400.0);
		let reg_view = mat4.lookAt(CAMERA, mat3.create(0, 0, 0), Y);
		let scale = mat4.scale(mat4.create(), mat3.create(8.0, 1.5, 8.0));

		reg_view = mat4.multiply(reg_view, scale);

		let flipped_view = mat4.lookAt(mat3.create(CAMERA[0], -CAMERA[1], CAMERA[2]), ZERO, Y);

		flipped_view = mat4.multiply(flipped_view, scale);

		return {
			view: reg_view,
			flipped_view,
			projection,
		};
	}

	static generate_uniforms(width: number, height: number): Uniforms {
		let { view, flipped_view, projection } = Example.generate_matrices(width / height);

		return {
			terrain_normal: {
				view_projection: mat4.multiply(projection, view),
				clipping_plane: new Float32Array(4),
			},
			terrain_flipped: {
				view_projection: mat4.multiply(projection, flipped_view),
				clipping_plane: new Float32Array([0, 1, 0, 0]),
			},
			water: {
				view: mat4.clone(view),
				projection: mat4.clone(projection),
				time_size_width: new Float32Array([0.0, 1.0, SIZE * 2.0, width]),
				height: new Float32Array([height, 0.0, 0.0, 0.0]),
			},
		};
	}

	///
	/// Initializes Uniforms and textures.
	///
	static initialize_resources(width: number, height: number, device: GPUDevice, queue: GPUQueue, water_uniforms: GPUBuffer, terrain_normal_uniforms: GPUBuffer, terrain_flipped_uniforms: GPUBuffer, water_bind_group_layout: GPUBindGroupLayout): { 0: GPUTextureView; 1: GPUTextureView; 2: GPUBindGroup } {
		// Matrices for our projection and view.
		// flipped_view is the view from under the water.
		let { terrain_normal, terrain_flipped, water } = Example.generate_uniforms(width, height);

		// Put the uniforms into buffers on the GPU
		queue.writeBuffer(terrain_normal_uniforms, 0, terrain_normal.view_projection);
		queue.writeBuffer(terrain_flipped_uniforms, 0, terrain_flipped.view_projection);
		queue.writeBuffer(water_uniforms, 0, water.projection);

		let texture_extent = {
			width,
			height,
			depth_or_array_layers: 1,
		};

		const format = navigator.gpu.getPreferredCanvasFormat();

		let reflection_texture = device.createTexture({
			label: 'Reflection Render Texture',
			size: texture_extent,
			mipLevelCount: 1,
			sampleCount: 1,
			dimension: '2d',
			format: format,
			usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT,
			viewFormats: [],
		});

		let draw_depth_buffer = device.createTexture({
			label: 'Depth Buffer',
			size: texture_extent,
			mipLevelCount: 1,
			sampleCount: 1,
			dimension: '2d',
			format: GPUTextureFormat.Depth32Float,
			usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT,
			viewFormats: [],
		});

		let color_sampler = device.createSampler({
			label: 'Color Sampler',
			addressModeU: GPUAddressMode.ClampToEdge,
			addressModeV: GPUAddressMode.ClampToEdge,
			addressModeW: GPUAddressMode.ClampToEdge,
			magFilter: GPUFilterMode.Nearest,
			minFilter: GPUFilterMode.Linear,
			mipmapFilter: GPUFilterMode.Nearest,
		});

		let depth_sampler = device.createSampler({
			label: 'Depth Sampler',
		});

		let depth_view = draw_depth_buffer.createView();

		let water_bind_group = device.createBindGroup({
			layout: water_bind_group_layout as never,
			entries: [
				{
					binding: 0,
					resource: {
						buffer: water_uniforms,
					},
				},
				{
					binding: 1,
					resource: reflection_texture.createView(),
				},
				{
					binding: 2,
					resource: depth_view,
				},
				{
					binding: 3,
					resource: color_sampler,
				},
				{
					binding: 4,
					resource: depth_sampler,
				},
			],
			label: 'Water Bind Group',
		});

		return {
			0: reflection_texture.createView(),
			1: depth_view,
			2: water_bind_group,
		};
	}

	static init(context: GPUCanvasContext, device: GPUDevice, queue: GPUQueue): Example {
		return null;
	}
}

export async function run(canvas: Canvas) {
	const adapter = await navigator.gpu?.requestAdapter();
	const device: GPUDevice = (await adapter?.requestDevice()) as never;

	const context = canvas.getContext('webgpu') as never as GPUCanvasContext;

	const devicePixelRatio = window.devicePixelRatio;
	const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	const path = knownFolders.currentApp().path;

	const solidColorLitWGSL = File.fromPath(path + '/webgpu/shaders/solidColorLit.wgsl').readTextSync();
	const wireframeWGSL = File.fromPath(path + '/webgpu/shaders/wireframe.wgsl').readTextSync();

	context.configure({
		device,
		format: presentationFormat,
	});
}
