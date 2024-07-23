import { Canvas, GPUBufferUsage, GPUCanvasContext, GPUDevice, GPUTexture, GPUTextureUsage, ImageAsset, GPUBuffer, GPUShaderStage, GPUBindGroup } from '@nativescript/canvas';

import { mat4, mat3 } from '../../../wgpu-matrix/wgpu-matrix';

import { cubeVertexArray, cubeVertexSize, cubeUVOffset, cubePositionOffset, cubeVertexCount } from '../meshes/cube';

import { File, knownFolders } from '@nativescript/core';

import { modelData } from '../models';
import { randElement, randColor } from '../utils';

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

interface Matrices {
	view: Float32Array;
	flipped_view: Float32Array;
	projection: Float32Array;
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
