//import { CanvasRenderingContext2D, WebGLRenderingContext } from './index.js';

import { createRequire } from 'node:module';
import '@nativescript/macos-node-api';
import '@nativescript/foundation/dom/index.js';
import Application from './app.ts';
import './canvas.ts';

objc.import('AppKit');
objc.import('OpenGL');
objc.import('QuartzCore');

// @ts-ignore
const require = createRequire(import.meta.url);

const { CanvasRenderingContext2D, ImageAsset, Path2D, GPU, GPUCanvasContext } = require('../canvas-napi.darwin-arm64.node');

const { requestAnimationFrame } = require('../utils/index.ts');

function mdnShadowColor(ctx: any) {
	// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor

	ctx.fillStyle = 'black';

	// Shadow
	ctx.shadowColor = 'red';
	ctx.shadowOffsetX = 10;
	ctx.shadowOffsetY = 10;

	// Filled rectangle
	ctx.fillRect(20, 20, 100, 100);

	// Stroked rectangle
	ctx.lineWidth = 6;

	ctx.strokeRect(170, 20, 100, 100);
}

function mdnStrokeText(ctx: any) {
	ctx.font = '50px serif';
	ctx.fillText('Hello world', 50, 90);
}

function mdnRoundRect(ctx: any) {
	// Rounded rectangle with zero radius (specified as a number)
	ctx.strokeStyle = 'red';
	ctx.beginPath();
	ctx.roundRect(10, 20, 150, 100, 0);
	ctx.stroke();

	// Rounded rectangle with 40px radius (single element list)
	ctx.strokeStyle = 'blue';
	ctx.beginPath();
	ctx.roundRect(10, 20, 150, 100, [40]);
	ctx.stroke();

	// Rounded rectangle with 2 different radii
	ctx.strokeStyle = 'orange';
	ctx.beginPath();
	ctx.roundRect(10, 150, 150, 100, [10, 40]);
	ctx.stroke();

	// Rounded rectangle with four different radii
	ctx.strokeStyle = 'green';
	ctx.beginPath();
	ctx.roundRect(400, 20, 200, 100, [0, 30, 50, 60]);
	ctx.stroke();

	// Same rectangle drawn backwards
	ctx.strokeStyle = 'magenta';
	ctx.beginPath();
	ctx.roundRect(400, 150, -200, 100, [0, 30, 50, 60]);
	ctx.stroke();
}

function doTheThing(canvas: any) {
	const asset = new ImageAsset();

	let loaded = false;
	// console.time('load1');
	// loaded = asset.fromUrlSync('https://www.superherotoystore.com/cdn/shop/articles/Website_Blog_creatives_29_1600x.jpg?v=1713945144');
	// console.timeEnd('load1');

	const scale = NSScreen.mainScreen.backingScaleFactor;

	const ctx = canvas.getContext('2d') as CanvasRenderingContext2D & { render: () => void };

	ctx.fillStyle = 'white';

	ctx.fillRect(0, 0, canvas.width, canvas.height);

	ctx.fillStyle = 'black';

	ctx.scale(scale, scale);

	//mdnShadowColor(ctx);
	//mdnRoundRect(ctx);

	ctx.render();

	/*

	asset.fromUrl('https://picsum.photos/id/1/200/300')
		.then(function(done) {
			console.log('ftghjkl');
			console.log(NSThread.currentThread);
			console.log('fromUrl: done', done, NSThread.currentThread);
			ctx.drawImage(asset, 0, 0, glview.frame.size.width * scale, glview.frame.size.height * scale);

			ctx.render();
		}).catch(e => {
		console.log('fromUrl: error', e);
		console.log(NSThread.currentThread);
	})
		.catch(e => {
			console.log('e', e);
		}).finally(() => {
		console.log('?');
	});
	*/
}

function solarSystem(canvas: any) {
	const ctx = canvas.getContext('2d');

	let sun = new ImageAsset();
	let moon = new ImageAsset();
	let earth = new ImageAsset();

	let LAF;

	const scale = NSScreen.mainScreen.backingScaleFactor;

	ctx.fillStyle = 'white';
	ctx.fillRect(0, 0, canvas.width, canvas.height);

	ctx.render();

	async function init() {
		// sun = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');
		// moon = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1443/Canvas_moon.png');
		// earth = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1429/Canvas_earth.png');

		await sun.fromUrl('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_sun.png');
		await moon.fromUrl('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_moon.png');
		await earth.fromUrl('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_earth.png');

		//	sun.fromUrlSync('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_sun.png');
		//	moon.fromUrlSync('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_moon.png');
		//	earth.fromUrlSync('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_earth.png');
		LAF = requestAnimationFrame(draw);
	}

	ctx.scale(scale, scale);

	function draw() {
		ctx.globalCompositeOperation = 'destination-over';
		ctx.clearRect(0, 0, canvas.width, canvas.height);

		ctx.fillStyle = 'rgba(0, 0, 0, 0.4)';
		ctx.strokeStyle = 'rgba(0, 153, 255, 0.4)';
		ctx.save();
		ctx.translate(150, 150);

		// Earth
		let time = new Date();
		ctx.rotate(((2 * Math.PI) / 60) * time.getSeconds() + ((2 * Math.PI) / 60000) * time.getMilliseconds());
		ctx.translate(105, 0);
		ctx.fillRect(0, -12, 40, 24); // Shadow
		ctx.drawImage(earth, -12, -12);

		// Moon
		ctx.save();
		ctx.rotate(((2 * Math.PI) / 6) * time.getSeconds() + ((2 * Math.PI) / 6000) * time.getMilliseconds());
		ctx.translate(0, 28.5);
		ctx.drawImage(moon, -3.5, -3.5);

		ctx.restore();
		ctx.restore();

		// Earth orbit arc
		ctx.beginPath();
		ctx.arc(150, 150, 105, 0, Math.PI * 2, false);
		ctx.stroke();

		//
		ctx.drawImage(sun, 0, 0, 300, 300);

		ctx.render();

		LAF = requestAnimationFrame(draw);
	}

	init();
}

function webglTextures(canvas: any) {
	const vertexShaderSrc = `
#version 320
precision highp float;
attribute vec2 position;
void main() {
  gl_Position = vec4(position, 0.0, 1.0);
  gl_PointSize = 128.0;
}`;

	const fragmentShaderSrc = `
#version 320
precision mediump float;
void main() {
  vec2 fragmentPosition = 2.0*gl_PointCoord - 1.0;
  float distance = length(fragmentPosition);
  float distanceSqrd = distance * distance;
  gl_FragColor = vec4(
    0.2/distanceSqrd,
    0.1/distanceSqrd,
    0.0, 1.0 );
}`;

	var gl: WebGLRenderingContext, program: WebGLProgram;

	function textures(canvas: any) {
		function setupWebGL() {
			if (!(gl = getRenderingContext()!)) return;
			console.log(gl.getParameter(gl.SHADING_LANGUAGE_VERSION));
			var source = vertexShaderSrc;
			var vertexShader = gl.createShader(gl.VERTEX_SHADER)!;
			gl.shaderSource(vertexShader!, source);
			gl.compileShader(vertexShader!);
			source = fragmentShaderSrc;
			var fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
			gl.shaderSource(fragmentShader!, source);
			gl.compileShader(fragmentShader!);
			program = gl.createProgram()!;

			gl.attachShader(program, vertexShader);
			gl.attachShader(program, fragmentShader);
			gl.linkProgram(program);
			gl.detachShader(program, vertexShader);
			gl.detachShader(program, fragmentShader);
			gl.deleteShader(vertexShader);
			gl.deleteShader(fragmentShader);
			if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
				var linkErrLog = gl.getProgramInfoLog(program);
				cleanup();
				console.log('Shader program did not link successfully. ' + 'Error log: ' + linkErrLog);
				return;
			}
			console.log('1', gl.getError());
			initializeAttributes();
			console.log('2', gl.getError());
			gl.useProgram(program);
			console.log('3', gl.getError());
			gl.drawArrays(gl.POINTS, 0, 1);
			(<any>gl).render();

			cleanup();
		}

		var buffer: WebGLBuffer;

		function initializeAttributes() {
			gl.enableVertexAttribArray(0);
			console.log('1.1', gl.getError());
			buffer = gl.createBuffer()!;
			console.log('1.2', gl.getError());
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			console.log('1.3', gl.getError());
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0]), gl.STATIC_DRAW);
			console.log('1.4', gl.getError());
			gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
			console.log('1.5', gl.getError());
		}

		function cleanup() {
			gl.useProgram(null);
			if (buffer) gl.deleteBuffer(buffer);
			if (program) gl.deleteProgram(program);
		}

		function getRenderingContext() {
			var gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
			console.log('gl', gl);
			if (!gl) {
				console.log('Failed to get WebGL context.' + 'Your device may not support WebGL.');
				return null;
			}
			gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
			gl.clearColor(0, 0, 0, 1.0);
			gl.clear(gl.COLOR_BUFFER_BIT);
			return gl as WebGLRenderingContext;
		}

		setupWebGL();
	}

	textures(canvas);
}

const gpu = GPU.getInstance();

globalThis.gpu = gpu;

const window = document.createElement('window');

window.style.width = `${NSScreen.mainScreen.frame.size.width / 2}`;
window.style.height = `${NSScreen.mainScreen.frame.size.height / 2}`;

function installPolyfills(window: any) {
	Object.defineProperty(window, 'devicePixelRatio', {
		value: NSScreen.mainScreen.backingScaleFactor,
		writable: true,
	});
}

installPolyfills(window);

async function webgpuTest() {
	console.log(gpu.wgslLanguageFeatures);
	console.log(gpu.getPreferredCanvasFormat());

	const adapter = await gpu.requestAdapter();

	console.log(adapter.features);
	console.log(adapter.isFallbackAdapter);
	console.log(adapter.limits);

	const info = await adapter.requestAdapterInfo();
	console.log(info.architecture);
	console.log(info.description);
	console.log(info.device);
	console.log(info.vendor);
}

async function webgpuTriangle(canvas: HTMLCanvasElement) {
	const tri = `@vertex
fn main(
  @builtin(vertex_index) VertexIndex : u32
) -> @builtin(position) vec4f {
  var pos = array<vec2f, 3>(
    vec2(0.0, 0.5),
    vec2(-0.5, -0.5),
    vec2(0.5, -0.5)
  );

  return vec4f(pos[VertexIndex], 0.0, 1.0);
}`;

	const red = `@fragment
fn main() -> @location(0) vec4f {
  return vec4(1.0, 0.0, 0.0, 1.0);
}`;
	// todo

	if (gpu) {
		const adapter = await gpu.requestAdapter();
		let device = await adapter.requestDevice();
		// device.addEventListener('uncapturederror', (event: any) => {
		// 	console.error('A WebGPU error was not captured:', event.error.message);
		// });

		const devicePixelRatio = window.devicePixelRatio;
		canvas.width = canvas.clientWidth * devicePixelRatio;
		canvas.height = canvas.clientHeight * devicePixelRatio;

		const context = canvas.getContext('webgpu') as GPUCanvasContext;

		// console.log(context.toDataURL());

		//
		const capabilities = (context as any).getCapabilities(adapter);
		//
		console.log(capabilities);

		const presentationFormat = gpu.getPreferredCanvasFormat(); //capabilities.format[0];
		const alphaMode = 'postmultiplied'; //capabilities.alphaModes[0];

		context.configure({
			device,
			format: presentationFormat,
			alphaMode,
			presentMode: 'fifo',
		});

		console.log('toDataURL', context.toDataURL());

		const pipeline = device.createRenderPipeline({
			layout: 'auto',
			vertex: {
				module: device.createShaderModule({
					code: tri,
				}),
				entryPoint: 'main',
			},
			fragment: {
				module: device.createShaderModule({
					code: red,
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
			},
		});

		function frame() {
			const framebuffer = context.getCurrentTexture();
			if (!framebuffer) {
				requestAnimationFrame(frame);
				return;
			}

			const commandEncoder = device.createCommandEncoder();
			const textureView = framebuffer.createView();

			const passEncoder = commandEncoder.beginRenderPass({
				colorAttachments: [
					{
						view: textureView,
						clearValue: [0, 0, 0, 1],
						loadOp: 'clear',
						storeOp: 'store',
					},
				],
			});
			passEncoder.setPipeline(pipeline);
			passEncoder.draw(3);
			passEncoder.end();

			device.queue.submit([commandEncoder.finish()]);
			(<any>context).presentSurface();
			//	requestAnimationFrame(frame);
		}

		requestAnimationFrame(frame);
	}
}

const canvas = document.createElement('canvas');

canvas.addEventListener('ready', (event) => {
	webgpuTriangle(canvas);
});

canvas.width = NSScreen.mainScreen.frame.size.width;
canvas.height = NSScreen.mainScreen.frame.size.height;

window.setAttribute('styleMask', (NSWindowStyleMask.Titled | NSWindowStyleMask.Closable | NSWindowStyleMask.Miniaturizable | NSWindowStyleMask.Resizable | NSWindowStyleMask.FullSizeContentView) as never);

window.appendChild(canvas);

document.body.appendChild(window);

Application.launch();
