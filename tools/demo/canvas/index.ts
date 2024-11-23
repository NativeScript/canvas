import { DemoSharedBase } from '../utils';
import { ImageSource, ObservableArray, Screen, Color, Application, knownFolders, path as filePath } from '@nativescript/core';

let Matter;
declare const NSCCanvas;
import { Canvas, ImageAsset } from '@nativescript/canvas';
import {
	arcToAnimation,
	flappyBird,
	arc,
	arcTo,
	cancelParticlesColor,
	cancelParticlesLarge,
	cancelRain,
	cancelRainbowOctopus,
	cancelSwarm,
	clip,
	cloth,
	colorRain,
	createLinearGradient,
	createRadialGradient,
	ellipse,
	fillPath,
	fillRule,
	filterBlur,
	imageBlock,
	imageSmoothingEnabled,
	imageSmoothingQuality,
	isPointInStrokeTouch,
	lineWidth,
	march,
	multiStrokeStyle,
	particlesColor,
	particlesLarge,
	patternWithCanvas,
	rainbowOctopus,
	scale,
	shadowBlur,
	shadowColor,
	swarm,
	textAlign,
	touchParticles,
	globalCompositeOperation,
	pattern,
	font,
	fillStyle,
	globalAlpha,
	lineCap,
	lineDashOffset,
	shadowOffsetX,
	strokeStyle,
	circle_demo,
	createConicGradient,
	clip2,
	clip3,
	clearRect,
	skew,
} from './canvas2d';
const Chart = require('chart.js').Chart;
import { handleVideo, cancelInteractiveCube, cancelMain, cubeRotation, cubeRotationRotation, drawElements, drawModes, imageFilter, interactiveCube, main, textures, points, triangle, scaleTriangle, imageProcessing, createChaosLines } from './webgl';
import { cancelEnvironmentMap, cancelFog, draw_image_space, draw_instanced, environmentMap, fog } from './webgl2';
// declare var com, java;
let zen3d;
import { drawChart, issue127, issue54, issue93 } from './issues';
import { subTest } from './webgl/test';
import { rnSkiaPerf } from './canvas2d/rn-skia-perf';
import { breathe } from './canvas2d/breathe';
import { lines } from './canvas2d/lines';
import { Svg } from '@nativescript/canvas-svg';
var Vex;
export class DemoSharedCanvas extends DemoSharedBase {
	private canvas: any;

	constructor() {
		super();
		Vex = require('vexflow');
	}

	canvasLoaded(args) {
		this.canvas = args.object;
		console.log('canvas ready');
		this.draw();
	}

	textBaseLine(canvas) {
		const ctx = canvas.getContext('2d');

		const baselines = ['top', 'hanging', 'middle', 'alphabetic', 'ideographic', 'bottom'];
		ctx.font = '24px serif';
		ctx.strokeStyle = 'red';

		baselines.forEach((baseline, index) => {
			ctx.textBaseline = baseline;
			const y = 75 + index * 75;
			ctx.beginPath();
			ctx.moveTo(0, y + 0.5);
			ctx.lineTo(550, y + 0.5);
			ctx.stroke();
			ctx.fillText(`Abcdefghijklmnop (${baseline})`, 0, y);
		});
	}

	textBaseLine2(canvas) {
		const ctx = canvas.getContext('2d');

		const baselines = ['top', 'hanging', 'middle', 'alphabetic', 'ideographic', 'bottom'];
		ctx.font = '20px serif';
		ctx.strokeStyle = 'red';

		ctx.beginPath();
		ctx.moveTo(0, 100);
		ctx.lineTo(840, 100);
		ctx.moveTo(0, 55);
		ctx.stroke();

		baselines.forEach((baseline, index) => {
			ctx.save();
			ctx.textBaseline = baseline;
			let x = index * 120 + 10;
			ctx.fillText('Abcdefghijk', x, 100);
			ctx.restore();
			ctx.fillText(baseline, x + 5, 50);
		});
	}

	urlTests() {
		this.urlConstructor();
		this.urlHash();
		this.urlHost();
		this.urlHostname();
		this.urlHref();
		this.urlOrigin();
		this.urlPassword();
		this.urlPathname();
		this.urlProtocol();
		this.urlSearch();

		this.urlUsername();
	}

	urlConstructor() {
		let m = 'https://developer.mozilla.org';
		let a = new URL('/', m); // => 'https://developer.mozilla.org/'
		let b = new URL(m); // => 'https://developer.mozilla.org/'

		new URL('en-US/docs', b); // => 'https://developer.mozilla.org/en-US/docs'
		let d = new URL('/en-US/docs', b); // => 'https://developer.mozilla.org/en-US/docs'
		new URL('/en-US/docs', d); // => 'https://developer.mozilla.org/en-US/docs'
		new URL('/en-US/docs', a); // => 'https://developer.mozilla.org/en-US/docs'

		new URL('/en-US/docs', 'https://developer.mozilla.org/fr-FR/toto');
		// => 'https://developer.mozilla.org/en-US/docs'

		try {
			new URL('/en-US/docs', ''); // Raises a TypeError exception as '' is not a valid URL
		} catch (e) {
			console.log(e);
		}

		try {
			new URL('/en-US/docs'); // Raises a TypeError exception as '/en-US/docs' is not a valid URL
		} catch (e) {
			console.log(e);
		}
		new URL('http://www.example.com'); // => 'http://www.example.com/'
		new URL('http://www.example.com', b); // => 'http://www.example.com/'

		new URL('//foo.com', 'https://example.com'); // => 'https://foo.com' (see relative URLs)
	}
	urlHash() {
		const url = new URL('https://developer.mozilla.org/en-US/docs/Web/API/URL/href#Examples');
		console.log(url.hash); // Logs: '#Examples'
	}

	urlHost() {
		let url = new URL('https://developer.mozilla.org/en-US/docs/Web/API/URL/host');
		console.log(url.host); // "developer.mozilla.org"

		url = new URL('https://developer.mozilla.org:443/en-US/docs/Web/API/URL/host');
		console.log(url.host); // "developer.mozilla.org"
		// The port number is not included because 443 is the scheme's default port

		url = new URL('https://developer.mozilla.org:4097/en-US/docs/Web/API/URL/host');
		console.log(url.host); // "developer.mozilla.org:4097"
	}

	urlHostname() {
		const url = new URL('https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname');
		console.log(url.hostname); // Logs: 'developer.mozilla.org'
	}

	urlHref() {
		const url = new URL('https://developer.mozilla.org/en-US/docs/Web/API/URL/href');
		console.log(url.href); // Logs: 'https://developer.mozilla.org/en-US/docs/Web/API/URL/href'
	}

	urlOrigin() {
		const url = new URL('blob:https://mozilla.org:443/');
		console.log(url.origin); // Logs 'https://mozilla.org'
	}

	urlPassword() {
		const url = new URL('https://anonymous:flabada@developer.mozilla.org/en-US/docs/Web/API/URL/password');
		console.log(url.password); // Logs "flabada"
	}

	urlPathname() {
		const url = new URL('https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname?q=value');
		console.log(url.pathname); // Logs "/en-US/docs/Web/API/URL/pathname"
	}

	urlPort() {
		const url = new URL('https://mydomain.com:80/svn/Repos/');
		console.log(url.port); // Logs '80'
	}

	urlProtocol() {
		const url = new URL('https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol');
		console.log(url.protocol); // Logs "https:"
	}

	urlSearch() {
		const url = new URL('https://developer.mozilla.org/en-US/docs/Web/API/URL/search?q=123');
		console.log(url.search); // Logs "?q=123"
	}

	urlUsername() {
		const url = new URL('https://anonymous:flabada@developer.mozilla.org/en-US/docs/Web/API/URL/username');
		console.log(url.username); // Logs "anonymous"
	}

	drawHouse(canvas) {
		const ctx = canvas.getContext('2d');
		// Set line width
		ctx.lineWidth = 10;

		// Wall
		ctx.strokeRect(75, 140, 150, 110);

		// Door
		ctx.fillRect(130, 190, 40, 60);

		// Roof
		ctx.beginPath();
		ctx.moveTo(50, 140);
		ctx.lineTo(150, 60);
		ctx.lineTo(250, 140);
		ctx.closePath();
		ctx.stroke();
	}

	drawOnCanvasWithCanvas(canvas) {
		// const c2 = document.createElement('canvas');
		// c2.width = 512;
		// c2.height = 512;
		// const c2d = c2.getContext('2d');

		const image = new global.ImageAsset();

		image.fromUrl(`https://picsum.photos/${512 * window?.devicePixelRatio ?? 1}/${512 * window?.devicePixelRatio ?? 1}`).then(async () => {
			//	c2d.drawImage(image, 0, 0, c2.width, c2.height);
			const bm = await createImageBitmap(image);
			console.log('done', image.width, image.height);
			canvas.width = canvas.clientWidth * window?.devicePixelRatio;
			canvas.height = canvas.clientHeight * window?.devicePixelRatio;
			const ctx = canvas.getContext('2d');
			ctx.drawImage(bm, 0, 0, canvas.width, canvas.height);
			//ctx.drawImage(c2, 0, 0, canvas.width, canvas.height);
		});
	}

	pathIssue(canvas) {
		const d = [
			[100, 68],
			[109.40456403667957, 87.05572809000084],
			[130.43380852144492, 90.11145618000168],
			[115.21690426072246, 104.94427190999916],
			[118.80912807335915, 125.88854381999832],
			[100, 116],
			[81.19087192664087, 125.88854381999832],
			[84.78309573927754, 104.94427190999916],
			[69.56619147855508, 90.1114561800017],
			[90.59543596332043, 87.05572809000084],
		];
		function drawStarWithCoords(ctx) {
			ctx.beginPath();
			ctx.moveTo(d[0][0], d[0][1]);
			for (let i = 1; i < d.length; i++) {
				ctx.lineTo(d[i][0], d[i][1]);
			}
			ctx.closePath();
			ctx.fill();
		}

		function drawStar(ctx, fetti: { x: number; y: number; scalar: number }) {
			var rot = (Math.PI / 2) * 3;
			var innerRadius = 4 * fetti.scalar;
			var outerRadius = 8 * fetti.scalar;
			var x = fetti.x;
			var y = fetti.y;
			var spikes = 5;
			var step = Math.PI / spikes;
			ctx.beginPath();
			while (spikes--) {
				x = fetti.x + Math.cos(rot) * outerRadius;
				y = fetti.y + Math.sin(rot) * outerRadius;
				ctx.lineTo(x, y);

				rot += step;

				x = fetti.x + Math.cos(rot) * innerRadius;
				y = fetti.y + Math.sin(rot) * innerRadius;

				ctx.lineTo(x, y);
				rot += step;
			}

			ctx.closePath();
			ctx.fill();
		}

		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

		ctx.clearRect(0, 0, canvas.width, canvas.height);

		ctx.fillStyle = 'red';
		drawStar(ctx, { x: 100, y: 200, scalar: 4 });

		ctx.fillStyle = 'blue';
		drawStarWithCoords(ctx);
	}

	fillIssue(canvas) {
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		ctx.beginPath();
		ctx.fillStyle = 'rgba(0, 255, 0, 1)';
		ctx.arc(50, 50, 30, 0, Math.PI * 2, true);
		ctx.arc(50, 50, 15, 0, Math.PI * 2, true);
		ctx.fill('evenodd');
	}

	clearIssue(canvas) {
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		ctx.fillStyle = 'red';
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		ctx.resetTransform();
		ctx.clearRect(0, 0, canvas.width, canvas.height);
	}

	drawChart(canvas) {
		canvas.backgroundColor = 'black';
		let ctx = canvas.getContext('2d');

		if (!ctx) {
			console.error('missing draw context');
			return;
		}

		let chartList = [];
		console.log('*** drawChart: ' + canvas + ', #arcs=' + chartList.length);
		let { width, height } = canvas;
		let bgRadius = Math.floor(Math.min(width, height) / 2);
		let bgWidth = bgRadius * 0.5;
		let borderWidth = bgRadius * 0.02;
		let contentWidth = bgRadius - 2 * borderWidth;
		let outerContentRadius = bgRadius - borderWidth;
		let centerX = Math.round(width / 2);
		let centerY = Math.round(height / 2);

		ctx.clearRect(0, 0, width, height);

		const _2_PI = Math.PI * 2;

		// background donut
		ctx.strokeStyle = '#363636';
		ctx.lineWidth = bgWidth;
		console.log(bgWidth);
		ctx.beginPath();
		ctx.arc(centerX, centerY, bgRadius - bgWidth / 2, 0, Math.PI * 2);
		ctx.stroke();
		// background donut outer
		ctx.strokeStyle = 'white';
		ctx.lineWidth = borderWidth;
		ctx.beginPath();
		ctx.arc(centerX, centerY, bgRadius - borderWidth / 2, 0, _2_PI);
		ctx.stroke();
		// background donut inner
		ctx.beginPath();
		ctx.arc(centerX, centerY, bgRadius / 2 + borderWidth / 2, 0, _2_PI);
		ctx.stroke();

		//<TEMP>
		chartList = [
			{
				color: '#eeeeeeff',
				minAngle: -1.9634954084936207,
				maxAngle: -1.1780972450961724,
				outerRadius: 1.278232843017578,
				width: 0.768232843017578,
			},
			{
				color: '#b82433',
				minAngle: -1.9474526277449578,
				maxAngle: -1.1941400258448356,
				outerRadius: 1.268232843017578,
				width: 0.758232843017578,
			},
			{
				color: '#eeeeeeff',
				minAngle: 1.1780972450961724,
				maxAngle: 1.9634954084936207,
				outerRadius: 1.2404586791992187,
				width: 0.7304586791992187,
			},
			{
				color: '#b82433',
				minAngle: 1.1941400258448356,
				maxAngle: 1.9474526277449578,
				outerRadius: 1.2304586791992187,
				width: 0.7204586791992187,
			},
			{
				color: '#eeeeeeff',
				minAngle: -3.5342917352885173,
				maxAngle: -2.748893571891069,
				outerRadius: 1.2133154541015623,
				width: 0.7033154541015624,
			},
			{
				color: '#b82433',
				minAngle: -3.518248954539854,
				maxAngle: -2.764936352639732,
				outerRadius: 1.2033154541015623,
				width: 0.6933154541015624,
			},
			{
				color: '#eeeeeeff',
				minAngle: -0.39269908169872414,
				maxAngle: 0.39269908169872414,
				outerRadius: 0.9305414001464843,
				width: 0.42054140014648433,
			},
			{
				color: '#b82433',
				minAngle: -0.37665630095006103,
				maxAngle: 0.37665630095006103,
				outerRadius: 0.9205414001464843,
				width: 0.4105414001464843,
			},
		];

		for (let item of chartList) {
			let outerRadius = borderWidth + item.outerRadius * outerContentRadius;

			console.log('*** draw arc: ' + JSON.stringify(item, null, 4));
			ctx.strokeStyle = item.color;
			ctx.lineWidth = item.width * bgRadius; //contentWidth;
			ctx.beginPath();
			ctx.arc(centerX, centerY, outerRadius - ctx.lineWidth / 2, item.minAngle, item.maxAngle);
			ctx.stroke();
		}
	}

	async webgpuTriangle() {
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

		if (navigator.gpu) {
			const adapter = await navigator.gpu.requestAdapter();
			const device = await adapter.requestDevice();

			device.addEventListener('uncapturederror', (event: any) => {
				console.error('A WebGPU error was not captured:', event.error.message);
			});

			const devicePixelRatio = window.devicePixelRatio;
			this.canvas.width = this.canvas.clientWidth * devicePixelRatio;
			this.canvas.height = this.canvas.clientHeight * devicePixelRatio;

			const context = this.canvas.getContext('webgpu') as GPUCanvasContext;
			const thiz = this;

			const capabilities = (context as any).getCapabilities(adapter);
			const presentationFormat = navigator.gpu.getPreferredCanvasFormat(); //capabilities.format[0];
			const alphaMode = capabilities.alphaModes[0];

			context.configure({
				device,
				format: presentationFormat,
				alphaMode,
			});

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
				console.log('1', thiz.canvas.toDataURL());
				(<any>context).presentSurface();
				console.log('2', thiz.canvas.toDataURL());
				//	requestAnimationFrame(frame);
			}
			requestAnimationFrame(frame);
		}
	}

	draw() {
		//this.remoteFont(this.canvas);
		//this.webgpuTest();
		//this.webgpuTriangle();
		// const rc = require('./webgpu/rotatingCube');
		// rc.run(this.canvas);

		// const tests = require('./webgpu/tests');
		// tests.run(this.canvas);

		// const renderBundles = require('./webgpu/renderBundles');
		// renderBundles.run(this.canvas);

		// const occlusionQuery = require('./webgpu/occlusionQuery');
		// occlusionQuery.run(this.canvas);

		// const particles = require('./webgpu/particles');
		// particles.run(this.canvas);

		// const texturedCube = require('./webgpu/basicGraphics/texturedCube');
		// texturedCube.run(this.canvas);

		// const imageBlur = require('./webgpu/imageBlur');
		// imageBlur.run(this.canvas);

		// const cubeMap = require('./webgpu/cubeMap');
		// cubeMap.run(this.canvas);

		// const instancedCube = require('./webgpu/instancedCube');
		// instancedCube.run(this.canvas);

		// const computeBoids = require('./webgpu/gpgpu/computeBoids');
		// computeBoids.run(this.canvas);

		// const twoCubes = require('./webgpu/basicGraphics/twoCubes');
		// twoCubes.run(this.canvas);

		// const fractualCube = require('./webgpu/basicGraphics/fractalCube');
		// fractualCube.run(this.canvas);

		// const wireframe = require('./webgpu/graphicsTechniques/wireframe');
		// wireframe.run(this.canvas);

		// const cameras = require('./webgpu/graphicsTechniques/cameras/cameras.ts');
		// cameras.run(this.canvas);

		// const pristineGrid = require('./webgpu/pristine-grid');
		// pristineGrid.run(this.canvas);

		//this.drawChart(this.canvas);
		//this.drawSVG(this.canvas);
		//	const ctx = this.canvas.getContext('2d');
		/*

		const asset = new global.ImageAsset();
		asset.fromUrl('https://raw.githubusercontent.com/mdn/content/main/files/en-us/web/api/canvasrenderingcontext2d/drawimage/rhino.jpg').then((done) => {
			const x: {
				scos: number;
				ssin: number;
				tx: number;
				ty: number;
			}[] = [
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },

				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },

				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },

				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
				{ scos: 1, ssin: 0, tx: 0, ty: 0 },
			];

			const width = 50 //* Screen.mainScreen.scale;
			const height = 38// * Screen.mainScreen.scale;
			const t: { x: number; y: number; width: number; height: number }[] = [
				{ x: 0, y: 0, width: width, height: height },
				{ x: width, y: 0, width: width, height: height },
				{ x: width * 2, y: 0, width: width, height: height },

				{ x: 0, y: height, width: width, height: height },
				{ x: width, y: height, width: width, height: height },
				{ x: width * 2, y: height, width: width, height: height },

				{ x: 0, y: height * 2, width: width, height: height },
				{ x: width, y: height * 2, width: width, height: height },
				{ x: width * 2, y: height * 2, width: width, height: height },

				{ x: 0, y: height * 3, width: width, height: height },
				{ x: width, y: height * 3, width: width, height: height },
				{ x: width * 2, y: height * 3, width: width, height: height },
			];

			ctx.drawAtlas(asset, x, t, null, null);

			// for (var i = 0; i < 4; i++) {
			// 	for (var j = 0; j < 3; j++) {
			// 		ctx.drawImage(asset, j * 50 * Screen.mainScreen.scale, i * 38 * Screen.mainScreen.scale, 50 * Screen.mainScreen.scale, 38 * Screen.mainScreen.scale);
			// 	}
			// }
		});

		*/
		//this.pathIssue(this.canvas);
		//(this.canvas);
		//this.clearIssue(this.canvas);
		//this.fillIssue(this.canvas);
		//rnSkiaPerf(this.canvas);
		//breathe(this.canvas);
		//this.drawOnCanvasWithCanvas(this.canvas);
		//const ctx = this.canvas.getContext('2d');
		//this.urlTests();
		//const str = new java.lang.String()
		// ctx.font = '50px serif';
		// ctx.fillText('Hello world', 50, 90);
		/*	const ctx = this.canvas.getContext('2d');

	// Moved square
	ctx.translate(110, 30);
	ctx.fillStyle = 'red';
	ctx.fillRect(0, 0, 80, 80);

	// Reset current transformation matrix to the identity matrix
	ctx.setTransform(1, 0, 0, 1, 0, 0);

	// Unmoved square
	ctx.fillStyle = 'gray';
	ctx.fillRect(0, 0, 80, 80); */
		//filterBlur(this.canvas);
		//handleVideo(this.canvas);
		/*	const worker = new Worker('./canvas.worker.js');
		global.CanvasWorker = worker;
		// canvas.parent.on(GestureTypes.touch as any, (args: TouchGestureEventData) => {
		//     var x = args.getX() * Screen.mainScreen.scale,
		//         y = (args.getY() * Screen.mainScreen.scale);
		//     worker.postMessage({event: 'touch', x, y})
		// });

		this.canvas.addEventListener('touchstart', (event) => {
			const touches = event.touches.item(0);
			const first = touches;
			worker.postMessage({ event: 'touchstart', clientX: first.clientX, clientY: first.clientY });
		});

		this.canvas.addEventListener('touchmove', (event) => {
			const touches = event.changedTouches;
			if (Array.isArray(touches)) {
				const first = touches[0];
				worker.postMessage({ event: 'touchmove', clientX: first.clientX, clientY: first.clientY });
			}
		});


		if (global.isAndroid) {
			//    canvas.android.setHandleInvalidationManually(true);
			(org.nativescript as any).canvas.NSCCanvas.getViews().put(`${this.canvas._domId}`, new java.lang.ref.WeakReference(this.canvas.android));
		} else {
			//  this.canvas.ios.handleInvalidationManually = true;
			//this.canvas.ios.moveOffMain();
			NSCCanvas.getViews().setObjectForKey(this.canvas.ios, `${this.canvas._domId}`);
		}

		const w = this.canvas.width,
			h = this.canvas.height;

		worker.postMessage({ id: `${this.canvas._domId}`, width: w, height: h });

		*/
		// worker.onerror = msg => {
		//     console.log('error', msg);
		// }
		//swarm(this.canvas);
		//touchParticles(this.canvas);
		// var map = L.map('map', {
		//     center: [51.505, -0.09],
		//     zoom: 13
		// });
		//this.vexFlow(this.canvas);
		// canvas.android.setHandleInvalidationManually(true);
		//const ctx = canvas.getContext('2d');
		//clearRect(this.canvas);
		//fillRule(this.canvas);
		//font(this.canvas);
		//fillStyle(this.canvas);
		//ctx.setLineDash([1,2]);
		//console.log(ctx.getLineDash());
		//clip(this.canvas);
		//clip2(this.canvas);
		//clip3(this.canvas);
		//fillStyle(this.canvas);
		//font(this.canvas);
		//globalAlpha(this.canvas);
		//globalCompositeOperation(this.canvas);
		//imageSmoothingEnabled(this.canvas);
		//drawChart(this.canvas);
		//issue127(this.canvas);
		//circle_demo(this.canvas);
		//imageSmoothingQuality(this.canvas);
		//lineCap(this.canvas);
		//lineDashOffset(this.canvas);
		//lineJoin(this.canvas);
		//lineWidth(this.canvas);
		// miterLimit(this.canvas);
		//shadowBlur(this.canvas);
		//shadowColor(this.canvas);
		//this.vulkan(this.canvas);
		//shadowOffsetX(this.canvas);
		//shadowOffsetY(this.canvas);
		//strokeStyle(this.canvas);
		//multiStrokeStyle(this.canvas);
		//textAlign(this.canvas)
		//arc(this.canvas);
		//arcMultiple(this.canvas);
		//arcTo(this.canvas);
		//arcToAnimation(this.canvas);
		//ellipse(this.canvas);
		//fillPath(this.canvas);
		createChaosLines(this.canvas);
		//flappyBird(this.canvas);
		//imageBlock(this.canvas);
		//scale(this.canvas);
		//pattern(this.canvas);
		//patternWithCanvas(this.canvas);
		//isPointInStrokeTouch(this.canvas);
		//createLinearGradient(this.canvas);
		//createRadialGradient(this.canvas);
		//march(this.canvas);
		//skew(this.canvas);
		//this.putImageDataDemo(this.canvas);
		//	this.drawImage(this.canvas);
		// ctx.fillStyle = 'blue';
		// ctx.fillRect(0,0,400,400)
		//ellipse(this.canvas);
		//this.drawPatternWithCanvas(this.canvas);
		//this.clock(this.canvas);
		//this.solar(this.canvas);
		//console.log('ready ??');
		//this.coloredParticles(this.canvas);
		//this.ball(this.canvas)
		//swarm(this.canvas);
		//this.drawHouse(this.canvas);
		//this.bubbleChart(this.canvas);
		//this.donutChart(this.canvas);
		//canvas.page.actionBarHidden = true;
		//this.hBarChart(this.canvas);
		//this.bubbleChart(this.canvas);
		//this.dataSets(this.canvas);
		//this.chartJS(this.canvas);
		//clear(null)
		//points(this.canvas)
		//textures(this.canvas);
		//scaleTriangle(this.canvas);
		//setTimeout(()=>{
		//colorRain(this.canvas);
		//particlesLarge(this.canvas);
		//rainbowOctopus(this.canvas);
		//particlesColor(this.canvas);
		//cloth(this.canvas);
		//touchParticles(this.canvas);
		//createConicGradient(this.canvas);
		//swarm(this.canvas);
		//textures(this.canvas)
		//drawModes(this.canvas,'triangles');
		//drawElements(this.canvas)
		// ctx = canvas.getContext("2d") as any;
		//swarm(this.canvas);
		// canvas.nativeView.handleInvalidationManually = true;
		//  setTimeout(() => {
		//draw_instanced(this.canvas);
		//draw_image_space(this.canvas);
		//fog(this.canvas);
		//environmentMap(this.canvas);
		//cubeRotationRotation(this.canvas);
		//main(this.canvas);
		//this.letterSpacing(this.canvas);
		//this.wordSpacing(this.canvas);
		//imageProcessing(this.canvas);
		//subTest(this.canvas);
		//imageFilter(this.canvas);
		//interactiveCube(this.canvas);
		//textures(this.canvas);
		//drawElements(this.canvas)
		//drawModes(this.canvas,'triangles')
		//fog(this.canvas);
		// }, 1000);
		//cubeRotation(this.canvas);
		//},3000)
		//drawModes(this.canvas,'triangles');
		//cubeRotation(this.canvas);
		//main(this.canvas)
		//this.pointStyle(this.canvas);
		//this.matterJSExample(this.canvas);
		//this.matterJSCar(this.canvas);
		//this.multiCanvas(this.canvas);
		//triangle(this.canvas);
		//this.zen3dCube(this.canvas);
		//this.zen3dGeometryLoaderGltf(this.canvas);
		//this.playCanvas(this.canvas);
		//this.drawRandomFullscreenImage(this.canvas);
		//issue54(this.canvas);
		//this.decoder()
		//this.context2DTest(this.canvas);
		//issue93(this.canvas);
		// const canvas = this.canvas;
		// console.time('getBoundingClientRect');
		// for(let i = 0; i < 100000;i++){
		// 	canvas.getBoundingClientRect();
		// }
		// console.timeEnd('getBoundingClientRect');
		// this.textBaseLine(this.canvas);
		//this.textBaseLine2(this.canvas);
	}
	async remoteFont(canvas: any) {
		const font = new FontFace('ChaChicle', 'url(https://pixijs.com/assets/webfont-loader/ChaChicle.ttf)');
		document.fonts.add(font);
		await font.load();

		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

		ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
		ctx.font = '25px ChaChicle';
		const size = ctx.measureText('NativeScript FontFace');
		ctx.fillText('NativeScript FontFace', canvas.clientWidth / 2 - size.width / 2, 50);
	}
	vulkan(canvas: any) {
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
		ctx.font = '16px serif';
		const size = ctx.measureText('NativeScript Canvas 2D Powered by Skia & Vulkan');
		ctx.fillText('NativeScript Canvas 2D Powered by Skia & Vulkan', canvas.clientWidth / 2 - size.width / 2, 50);
		const vk = new ImageAsset();
		const ns = new ImageAsset();
		const skia = new ImageAsset();
		Promise.allSettled([skia.fromUrl('https://upload.wikimedia.org/wikipedia/en/thumb/3/33/Skia_Project_Logo.svg/1024px-Skia_Project_Logo.svg.png'), vk.fromUrl('https://upload.wikimedia.org/wikipedia/commons/thumb/f/fe/Vulkan_logo.svg/1024px-Vulkan_logo.svg.png'), ns.fromUrl('https://upload.wikimedia.org/wikipedia/commons/8/86/NativeScript_Logo.png')]).then((res) => {
			ctx.drawImage(ns as any, canvas.clientWidth / 2 - 50, 60, 100, 100);
			ctx.drawImage(skia as any, canvas.clientWidth / 2 - 50, 150 + 20, 100, 100);
			ctx.drawImage(vk as any, canvas.clientWidth / 2 - 50, 250 + 30, 100, 100);
		});
	}

	drawSVG(canvas) {
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		const data = `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
		<rect width="100" height="500" stroke="red" fill="none"></rect>
		<line x1="0" y1="80" x2="100" y2="20" stroke="black" />
	  </svg>
	  `;

		const svg = Svg.fromSrcSync(data) as any;
		const img = (<any>HTMLImageElement)._fromSvg(svg);
		ctx.drawImage(img, 0, 0, img.width, img.height);
	}

	letterSpacing(canvas) {
		const ctx = canvas.getContext('2d');
		ctx.font = '30px serif';

		// Default letter spacing
		ctx.fillText(`Hello world (default: ${ctx.letterSpacing})`, 10, 40);

		// Custom letter spacing: 10px
		ctx.letterSpacing = '10px';
		ctx.fillText(`Hello world (${ctx.letterSpacing})`, 10, 90);

		// Custom letter spacing: 20px
		ctx.letterSpacing = '20px';
		ctx.fillText(`Hello world (${ctx.letterSpacing})`, 10, 140);
	}

	wordSpacing(canvas) {
		const ctx = canvas.getContext('2d');
		ctx.font = '30px serif';

		// Default word spacing
		ctx.fillText(`Hello world (default: ${ctx.wordSpacing})`, 10, 40);

		// Custom word spacing: 10px
		ctx.wordSpacing = '10px';
		ctx.fillText(`Hello world (${ctx.wordSpacing})`, 10, 90);

		// Custom word spacing: 30px
		ctx.wordSpacing = '30px';
		ctx.fillText(`Hello world (${ctx.wordSpacing})`, 10, 140);
	}

	drawRandomFullscreenImage(canvas) {
		const width = Screen.mainScreen.widthPixels;
		const height = Screen.mainScreen.heightPixels;
		const ctx = canvas.getContext('2d');
		ctx.fillRect(300, 300, 300, 300);
		const asset = new global.ImageAsset();

		let realPath = '~/assets/file-assets/webgl/svh.jpeg';
		//let realPath = '~/assets/file-assets/webgl/Canvas_sun.png';
		if (typeof realPath === 'string') {
			if (realPath.startsWith('~/')) {
				realPath = filePath.join(knownFolders.currentApp().path, realPath.replace('~/', ''));
			}
		}

		asset.fromFileSync(realPath);
		//asset.loadFromUrlSync('https://i0.wp.com/www.printmag.com/wp-content/uploads/2021/02/4cbe8d_f1ed2800a49649848102c68fc5a66e53mv2.gif?fit=476%2C280&ssl=1');
		//asset.loadFromUrl('https://pbs.twimg.com/media/FQaPvSZXwAgfun7?format=png&name=large')
		//asset.loadFromUrlSync(`https://pbs.twimg.com/media/FQaPvSZXwAgfun7?format=jpg&name=large`);
		//asset.loadFromUrlSync('https://upload.wikimedia.org/wikipedia/en/0/00/Spider-Man_No_Way_Home_poster.jpg');
		//asset.loadFromUrlSync('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');
		function draw() {
			ctx.drawImage(asset, 0, 0, 300 * Screen.mainScreen.scale, 300 * Screen.mainScreen.scale);
			requestAnimationFrame(draw);
		}
		draw();

		// const image = new Image();
		// image.onload = () => {
		// 	ctx.drawImage(image, 0, 0);
		// };
		// image.src = `https://source.unsplash.com/random/${width}x${height}`;
	}

	playCanvas(canvas) {
		require('@nativescript/canvas-polyfill');
		const pc = require('playcanvas');
		const app = new pc.Application(canvas, {});

		// fill the available space at full resolution
		app.setCanvasFillMode(pc.FILLMODE_FILL_WINDOW);
		app.setCanvasResolution(pc.RESOLUTION_AUTO);

		// ensure canvas is resized when window changes size
		window.addEventListener('resize', () => app.resizeCanvas());

		// create box entity
		const box = new pc.Entity('cube');
		box.addComponent('model', {
			type: 'box',
		});
		app.root.addChild(box);

		// create camera entity
		const camera = new pc.Entity('camera');
		camera.addComponent('camera', {
			clearColor: new pc.Color(0.1, 0.1, 0.1),
		});
		app.root.addChild(camera);
		camera.setPosition(0, 0, 3);

		// create directional light entity
		const light = new pc.Entity('light');
		light.addComponent('light');
		app.root.addChild(light);
		light.setEulerAngles(45, 0, 0);

		// rotate the box according to the delta time since the last frame
		app.on('update', (dt) => box.rotate(10 * dt, 20 * dt, 30 * dt));

		app.start();
	}

	gridLoaded(args) {
		const grid = args.object;
		this.removeClipping(grid);

		// d3 example
		/*
		const d3 = require('d3');
		const svg = d3.create('svg')
		.attr("viewBox", [0, 0, 975, 610])
		.attr("stroke-linejoin", "round")
		.attr("stroke-linecap", "round");
		svg.append('circle').attr('cx', 2).attr('cy', 2).attr('r', 40).style('fill', 'blue');
		svg.append('circle').attr('cx', 140).attr('cy', 70).attr('r', 40).style('fill', 'red');
		svg.append('circle').attr('cx', 300).attr('cy', 100).attr('r', 40).style('fill', 'green');

		grid.addChild(svg['_groups'][0][0].nativeElement);

		*/
	}

	svgLoaded(args) {
		const svg = args.object;
		this.removeClipping(svg);
	}

	removeClipping(view) {
		if (view.android) {
			if (view.nativeView.setClipChildren) {
				view.nativeView.setClipChildren(false);
			}
			if (view.nativeView.setClipToPadding) {
				view.nativeView.setClipToPadding(false);
			}
		}
	}

	getWidth() {
		return Screen.mainScreen.widthPixels;
	}

	getHeight() {
		return Screen.mainScreen.heightPixels - 300;
	}

	ctx: CanvasRenderingContext2D;

	zen3dCube(canvas) {
		if (zen3d === undefined) {
			zen3d = require('zen-3d');
			(global as any).zen3d = zen3d;
		}
		var gl = canvas.getContext('webgl2', {
			antialias: true,
			alpha: false,
			stencil: true,
		});
		const { drawingBufferWidth, drawingBufferHeight } = gl;
		let width = drawingBufferWidth;
		let height = drawingBufferHeight;

		var glCore = new zen3d.WebGLCore(gl);
		glCore.state.colorBuffer.setClear(0.1, 0.1, 0.1, 1);
		var backRenderTarget = new zen3d.RenderTargetBack(canvas);

		var scene = new zen3d.Scene();

		var geometry = new zen3d.CubeGeometry(8, 8, 8);
		var material = new zen3d.PBRMaterial();
		var mesh = new zen3d.Mesh(geometry, material);
		scene.add(mesh);

		var ambientLight = new zen3d.AmbientLight(0xffffff);
		scene.add(ambientLight);

		var directionalLight = new zen3d.DirectionalLight(0xffffff);
		directionalLight.position.set(-5, 5, 5);
		directionalLight.lookAt(new zen3d.Vector3(), new zen3d.Vector3(0, 1, 0));
		scene.add(directionalLight);

		var camera = new zen3d.Camera();
		camera.position.set(0, 10, 30);
		camera.lookAt(new zen3d.Vector3(0, 0, 0), new zen3d.Vector3(0, 1, 0));
		camera.setPerspective((45 / 180) * Math.PI, width / height, 1, 1000);
		scene.add(camera);

		function loop(count) {
			requestAnimationFrame(loop);

			mesh.euler.y = (count / 1000) * 0.5; // rotate cube

			scene.updateMatrix();
			scene.updateLights();

			glCore.renderTarget.setRenderTarget(backRenderTarget);

			glCore.clear(true, true, false);

			glCore.render(scene, camera);
		}

		requestAnimationFrame(loop);

		function onWindowResize() {
			width = window.innerWidth || 2;
			height = window.innerHeight || 2;

			camera.setPerspective((45 / 180) * Math.PI, width / height, 1, 1000);

			backRenderTarget.resize(width, height);
		}

		window.addEventListener('resize', onWindowResize, false);
	}

	zen3dGeometryLoaderGltf(canvas) {
		if (zen3d === undefined) {
			zen3d = require('zen-3d');
			(global as any).zen3d = zen3d;
		}

		const zen3dRoot = '~/assets/file-assets/zen3d/';
		require('./zen3d/js/objects/SkyBox.js');
		require('./zen3d/js/loaders/GLTFLoader.js');
		require('./zen3d/js/controls/OrbitControls.js');

		var renderer = new zen3d.Renderer(canvas);
		let gl = canvas.getContext('webgl2');
		if (!gl) {
			gl = canvas.getContext('webgl');
		}

		const { drawingBufferWidth, drawingBufferHeight } = gl;
		let width = canvas.width;
		let height = canvas.height;
		var scene = new zen3d.Scene();

		var file = '~/assets/three/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf';
		var cube_texture = zen3d.TextureCube.fromSrc([zen3dRoot + 'Bridge2/posx.jpg', zen3dRoot + 'Bridge2/negx.jpg', zen3dRoot + 'Bridge2/posy.jpg', zen3dRoot + 'Bridge2/negy.jpg', zen3dRoot + 'Bridge2/posz.jpg', zen3dRoot + 'Bridge2/negz.jpg']);
		var sky_box = new zen3d.SkyBox(cube_texture);
		sky_box.level = 4;

		let objectMaterial;

		// var nanobar = new Nanobar();
		// nanobar.el.style.background = "gray";

		var loadingManager = new zen3d.LoadingManager(
			function () {
				//  nanobar.go(100);
				//  nanobar.el.style.background = "transparent";
			},
			function (url, itemsLoaded, itemsTotal) {
				if (itemsLoaded < itemsTotal) {
					// nanobar.go(itemsLoaded / itemsTotal * 100);
				}
			}
		);

		var loader = new zen3d.GLTFLoader(loadingManager);
		loader.load(file, function (result) {
			// add mesh to scene
			let object = result.scene.children[0];

			objectMaterial = object.material;

			objectMaterial.envMap = cube_texture;
			objectMaterial.envMapIntensity = 1;

			object.scale.set(10, 10, 10);
			object.euler.z = -Math.PI / 6;

			scene.add(object);
		});

		// top light
		var directionalLight = new zen3d.DirectionalLight(0xbbbbff, 0.5);
		directionalLight.euler.set(Math.PI / 2, 0, 0);
		scene.add(directionalLight);

		// bottom light
		var directionalLight = new zen3d.DirectionalLight(0x444422, 0.5);
		directionalLight.euler.set(-Math.PI / 2, 0, 0);
		scene.add(directionalLight);

		var camera = new zen3d.Camera();
		camera.outputEncoding = zen3d.TEXEL_ENCODING_TYPE.GAMMA;
		camera.position.set(-15, 10, 90);
		camera.lookAt(new zen3d.Vector3(0, 0, 0), new zen3d.Vector3(0, 1, 0));
		camera.setPerspective((45 / 180) * Math.PI, width / height, 1, 8000);
		camera.add(sky_box);
		scene.add(camera);

		var controller = new zen3d.OrbitControls(camera, canvas);
		controller.enablePan = false;

		function loop(count) {
			requestAnimationFrame(loop);

			controller.update();

			renderer.render(scene, camera);
		}

		loop(0);

		function onWindowResize() {
			width = canvas.width;
			height = canvas.height;

			camera.setPerspective((45 / 180) * Math.PI, width / height, 1, 8000);

			renderer.backRenderTarget.resize(width, height);
		}

		window.addEventListener('resize', onWindowResize, false);
	}

	async drawImage(context) {
		var sun = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');
		context.drawImage(sun, 0, 0);
	}

	decoder() {
		// let uint8Array = new Uint8Array([228, 189, 160, 229, 165, 189]);
		//
		// console.log( new TextDecoder().decode(uint8Array) ); // 你好

		let utf8decoder = new TextDecoder(); // default 'utf-8' or 'utf8'
		console.log(utf8decoder.encoding);

		let u8arr = new Uint8Array([240, 160, 174, 183]);
		let i8arr = new Int8Array([-16, -96, -82, -73]);
		let u16arr = new Uint16Array([41200, 47022]);
		let i16arr = new Int16Array([-24336, -18514]);
		let i32arr = new Int32Array([-1213292304]);

		console.log(utf8decoder.decode(u8arr)); // 𠮷
		console.log(utf8decoder.decode(i8arr)); // 𠮷
		console.log(utf8decoder.decode(u16arr)); // 𠮷
		console.log(utf8decoder.decode(i16arr)); // 𠮷
		console.log(utf8decoder.decode(i32arr)); // 𠮷

		let win1251decoder = new TextDecoder('windows-1251');
		let bytes = new Uint8Array([207, 240, 232, 226, 229, 242, 44, 32, 236, 232, 240, 33]);
		console.log(win1251decoder.decode(bytes)); // Привет, мир!
	}

	putImageDataDemo(canvas) {
		const ctx = canvas.getContext('2d');
		ctx.rect(10, 10, 100, 100);
		ctx.fill();

		let imageData = ctx.getImageData(60, 60, 200, 100);
		ctx.putImageData(imageData, 150, 10);
	}

	onLayout(args) {
		console.log('onLayout');
	}

	vexFlow(canvas) {
		const VF = Vex.Flow as any;
		const renderer = new VF.Renderer(canvas, VF.Renderer.Backends.CANVAS);

		// Configure the rendering context.
		renderer.resize(500, 500);
		const context = renderer.getContext();
		context.setFont('Arial', 10, '').setBackgroundFillStyle('#eed');

		// Create a stave of width 400 at position 10, 40 on the canvas.
		const stave = new VF.Stave(10, 40, 400);

		// Add a clef and time signature.
		stave.addClef('treble').addTimeSignature('4/4');

		// Connect it to the rendering context and draw!
		stave.setContext(context).draw();
	}

	coloredParticles(canvas) {
		canvas.width = canvas.clientWidth;
		canvas.height = canvas.clientHeight;
		var ctx = canvas.getContext('2d'),
			particles = [],
			patriclesNum = 100,
			w = canvas.width,
			h = canvas.height,
			colors = ['#f35d4f', '#f36849', '#c0d988', '#6ddaf1', '#f1e85b'];

		function Factory() {
			this.x = Math.round(Math.random() * w);
			this.y = Math.round(Math.random() * h);
			this.rad = Math.round(Math.random() * 1) + 1;
			this.rgba = colors[Math.round(Math.random() * 3)];
			this.vx = Math.round(Math.random() * 3) - 1.5;
			this.vy = Math.round(Math.random() * 3) - 1.5;
		}

		function draw() {
			ctx.clearRect(0, 0, w, h);
			ctx.globalCompositeOperation = 'lighter';
			for (var i = 0; i < patriclesNum; i++) {
				var temp = particles[i];
				var factor = 1;

				for (var j = 0; j < patriclesNum; j++) {
					var temp2 = particles[j];
					ctx.lineWidth = 0.5;

					if (temp.rgba == temp2.rgba && findDistance(temp, temp2) < 50) {
						ctx.strokeStyle = temp.rgba;
						ctx.beginPath();
						ctx.moveTo(temp.x, temp.y);
						ctx.lineTo(temp2.x, temp2.y);
						ctx.stroke();
						factor++;
					}
				}

				ctx.fillStyle = temp.rgba;
				ctx.strokeStyle = temp.rgba;

				ctx.beginPath();
				ctx.arc(temp.x, temp.y, temp.rad * factor, 0, Math.PI * 2, true);
				ctx.fill();
				ctx.closePath();

				ctx.beginPath();
				ctx.arc(temp.x, temp.y, (temp.rad + 5) * factor, 0, Math.PI * 2, true);
				ctx.stroke();
				ctx.closePath();

				temp.x += temp.vx;
				temp.y += temp.vy;

				if (temp.x > w) temp.x = 0;
				if (temp.x < 0) temp.x = w;
				if (temp.y > h) temp.y = 0;
				if (temp.y < 0) temp.y = h;
			}
		}

		function findDistance(p1, p2) {
			return Math.sqrt(Math.pow(p2.x - p1.x, 2) + Math.pow(p2.y - p1.y, 2));
		}

		function init() {
			for (var i = 0; i < patriclesNum; i++) {
				particles.push(new Factory());
			}
		}

		function loop() {
			draw();
			requestAnimationFrame(loop);
		}

		init();
		loop();
	}

	ball(canvas) {
		const ctx = canvas.getContext('2d');
		let raf;
		let running = false;

		const ball = {
			x: 100,
			y: 100,
			vx: 5,
			vy: 1,
			radius: 25,
			color: 'blue',
			draw: function () {
				ctx.beginPath();
				ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, true);
				ctx.closePath();
				ctx.fillStyle = this.color;
				ctx.fill();
			},
		};

		function clear() {
			ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
			ctx.fillRect(0, 0, canvas.width, canvas.height);
		}

		function draw() {
			clear();
			ball.draw();
			ball.x += ball.vx;
			ball.y += ball.vy;

			if (ball.y + ball.vy > canvas.height || ball.y + ball.vy < 0) {
				ball.vy = -ball.vy;
			}
			if (ball.x + ball.vx > canvas.width || ball.x + ball.vx < 0) {
				ball.vx = -ball.vx;
			}

			raf = window.requestAnimationFrame(draw);
		}

		ball.draw();

		raf = window.requestAnimationFrame(draw);
		running = true;
	}

	matterJSCar(canvas) {
		/*
		if (Matter === undefined) {
			Matter = require('matter-js');
		}
		const car = function () {
			const Engine = Matter.Engine,
				Render = Matter.Render,
				Runner = Matter.Runner,
				Composites = Matter.Composites,
				MouseConstraint = Matter.MouseConstraint,
				Mouse = Matter.Mouse,
				World = Matter.World,
				Bodies = Matter.Bodies;

			// create engine
			const engine = Engine.create(),
				world = engine.world;

			// create renderer
			const render = Render.create({
				canvas,
				engine: engine,
				options: {
					width: 800,
					height: 600,
					showAngleIndicator: true,
					showCollisions: true,
				},
			});

			Render.run(render);

			// create runner
			const runner = Runner.create();
			Runner.run(runner, engine);

			// add bodies
			World.add(world, [
				// walls
				Bodies.rectangle(400, 0, 800, 50, { isStatic: true }),
				Bodies.rectangle(400, 600, 800, 50, { isStatic: true }),
				Bodies.rectangle(800, 300, 50, 600, { isStatic: true }),
				Bodies.rectangle(0, 300, 50, 600, { isStatic: true }),
			]);

			let scale = 0.9;
			World.add(world, Composites.car(150, 100, 150 * scale, 30 * scale, 30 * scale));

			scale = 0.8;
			World.add(world, Composites.car(350, 300, 150 * scale, 30 * scale, 30 * scale));

			World.add(world, [
				Bodies.rectangle(200, 150, 400, 20, {
					isStatic: true,
					angle: Math.PI * 0.06,
				}),
				Bodies.rectangle(500, 350, 650, 20, {
					isStatic: true,
					angle: -Math.PI * 0.06,
				}),
				Bodies.rectangle(300, 560, 600, 20, {
					isStatic: true,
					angle: Math.PI * 0.04,
				}),
			]);

			// add mouse control
			const mouse = Mouse.create(render.canvas),
				mouseConstraint = MouseConstraint.create(engine, {
					mouse: mouse,
					constraint: {
						stiffness: 0.2,
						render: {
							visible: false,
						},
					},
				});

			World.add(world, mouseConstraint);

			// keep the mouse in sync with rendering
			render.mouse = mouse;

			// fit the render viewport to the scene
			Render.lookAt(render, {
				min: { x: 0, y: 0 },
				max: { x: 800, y: 600 },
			});

			// context for MatterTools.Demo
			return {
				engine: engine,
				runner: runner,
				render: render,
				canvas: render.canvas,
				stop: function () {
					Matter.Render.stop(render);
					Matter.Runner.stop(runner);
				},
			};
		};

		car();
		*/
	}

	matterJSExample(canvas) {
		/*
		if (Matter === undefined) {
			Matter = require('matter-js');
		}
		// module aliases
		const Engine = Matter.Engine,
			Render = Matter.Render,
			World = Matter.World,
			Bodies = Matter.Bodies;

		// create an engine
		const engine = Engine.create();

		// create a renderer
		const render = Render.create({
			canvas,
			engine: engine,
		});

		// create two boxes and a ground
		var boxA = Bodies.rectangle(400, 200, 80, 80);
		var boxB = Bodies.rectangle(450, 50, 80, 80);
		var ground = Bodies.rectangle(400, 610, 810, 60, { isStatic: true });

		// add all of the bodies to the world
		World.add(engine.world, [boxA, boxB, ground]);

		// run the engine
		Engine.run(engine);

		// run the renderer
		Render.run(render);

		*/
	}

	multiCanvas(canvas) {
		if (canvas.id === 'canvas1') {
			//swarm(canvas);
			this.zen3dCube(canvas);
		}
		if (canvas.id === 'canvas2') {
			this.clock(canvas);
		}

		if (canvas.id === 'canvas3') {
			this.solar(canvas);
		}
		if (canvas.id === 'canvas4') {
			main(canvas);
		}
	}

	pointStyle(canvas) {
		const color = Chart.helpers.color;

		const createConfig = (colorName) => {
			return {
				type: 'line',
				data: {
					labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
					datasets: [
						{
							label: 'My First dataset',
							data: [this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor()],
							backgroundColor: color(this.chartColors[colorName]).alpha(0.5).rgbString(),
							borderColor: this.chartColors[colorName],
							borderWidth: 1,
							pointStyle: 'rectRot',
							pointRadius: 5,
							pointBorderColor: 'rgb(0, 0, 0)',
						},
					],
				},
				options: {
					responsive: true,
					legend: {
						labels: {
							usePointStyle: false,
						},
					},
					scales: {
						xAxes: [
							{
								display: true,
								scaleLabel: {
									display: true,
									labelString: 'Month',
								},
							},
						],
						yAxes: [
							{
								display: true,
								scaleLabel: {
									display: true,
									labelString: 'Value',
								},
							},
						],
					},
					title: {
						display: true,
						text: 'Normal Legend',
					},
				},
			};
		};

		function createPointStyleConfig(colorName) {
			var config = createConfig(colorName);
			config.options.legend.labels.usePointStyle = true;
			config.options.title.text = 'Point Style Legend';
			return config;
		}

		[
			{
				id: 'chart-legend-normal',
				config: createConfig('red'),
			},
			{
				id: 'chart-legend-pointstyle',
				config: createPointStyleConfig('blue'),
			},
		].forEach(function (details) {
			var ctx = canvas.getContext('2d');
			new Chart(ctx, details.config);
		});
	}

	lineBoundaries(canvas) {
		var presets = this.chartColors;
		var inputs = {
			min: -100,
			max: 100,
			count: 8,
			decimals: 2,
			continuity: 1,
		};

		const generateData = (config?) => {
			return this.utils.numbers(Chart.helpers.merge(inputs, config || {}));
		};

		const generateLabels = (config?) => {
			return this.utils.months(
				Chart.helpers.merge(
					{
						count: inputs.count,
						section: 3,
					},
					config || {}
				)
			);
		};

		var options = {
			maintainAspectRatio: false,
			spanGaps: false,
			elements: {
				line: {
					tension: 0.000001,
				},
			},
			plugins: {
				filler: {
					propagate: false,
				},
			},
			scales: {
				xAxes: [
					{
						ticks: {
							autoSkip: false,
							maxRotation: 0,
						},
					},
				],
			},
		};

		[false, 'origin', 'start', 'end'].forEach((boundary, index) => {
			// reset the random seed to generate the same data for all charts
			this.utils.srand(8);

			new Chart(canvas, {
				type: 'line',
				data: {
					labels: generateLabels(),
					datasets: [
						{
							backgroundColor: this.utils.transparentize(presets.red),
							borderColor: presets.red,
							data: generateData(),
							label: 'Dataset',
							fill: boundary,
						},
					],
				},
				options: Chart.helpers.merge(options, {
					title: {
						text: 'fill: ' + boundary,
						display: true,
					},
				}),
			});
		});

		// eslint-disable-next-line no-unused-vars
		function toggleSmooth(btn) {
			var value = btn.classList.toggle('btn-on');
			Chart.helpers.each(Chart.instances, function (chart) {
				chart.options.elements.line.tension = value ? 0.4 : 0.000001;
				chart.update();
			});
		}

		const randomize = () => {
			var seed = this.utils.rand();
			Chart.helpers.each(Chart.instances, (chart) => {
				this.utils.srand(seed);

				chart.data.datasets.forEach(function (dataset) {
					dataset.data = generateData();
				});

				chart.update();
			});
		};
	}

	drawPatternWithCanvas(canvas) {
		const patternCanvas = Canvas.createCustomView() as any;

		//const patternContext = patternCanvas.getContext('webgl') as any;

		// if(patternContext instanceof WebGLRenderingContext){
		// 	patternContext.clearColor(0,1,0,1);
		// 	patternContext.clear(patternContext.COLOR_BUFFER_BIT);
		// }

		// const scale = Screen.mainScreen.scale;

		const size = 50;
		const patternContext = patternCanvas.getContext('2d') as any;

		//glViewport(0,0,50,50);
		// Give the pattern a width and height of 50
		patternCanvas.width = size;
		patternCanvas.height = size;

		//  patternCanvas.getContext('2d') as any;
		// Give the pattern a background color and draw an arc
		patternContext.fillStyle = '#fec';

		patternContext.fillRect(0, 0, size, size);
		patternContext.arc(0, 0, size, 0, 0.5 * Math.PI);
		patternContext.stroke();

		// Create our primary canvas and fill it with the pattern
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		ctx.fillStyle = ctx.createPattern(patternContext, 'repeat');
		ctx.fillRect(0, 0, canvas.width, canvas.height);
	}

	donutChart(canvas) {
		var randomScalingFactor = function () {
			return Math.round(Math.random() * 100);
		};

		var config = {
			type: 'doughnut',
			data: {
				datasets: [
					{
						data: [randomScalingFactor(), randomScalingFactor(), randomScalingFactor(), randomScalingFactor(), randomScalingFactor()],
						backgroundColor: [this.chartColors.red, this.chartColors.orange, this.chartColors.yellow, this.chartColors.green, this.chartColors.blue],
						label: 'Dataset 1',
					},
				],
				labels: ['Red', 'Orange', 'Yellow', 'Green', 'Blue'],
			},
			options: {
				responsive: true,
				legend: {
					position: 'top',
				},
				title: {
					display: true,
					text: 'Chart.js Doughnut Chart',
				},
				animation: {
					animateScale: true,
					animateRotate: true,
				},
			},
		};

		const myDoughnut = new Chart(canvas, config);

		function randomizeData() {
			config.data.datasets.forEach(function (dataset) {
				dataset.data = dataset.data.map(function () {
					return randomScalingFactor();
				});
			});

			myDoughnut.update();
		}

		var colorNames = Object.keys(this.chartColors);

		const addDataset = () => {
			var newDataset = {
				backgroundColor: [],
				data: [],
				label: 'New dataset ' + config.data.datasets.length,
			};

			for (var index = 0; index < config.data.labels.length; ++index) {
				newDataset.data.push(randomScalingFactor());

				var colorName = colorNames[index % colorNames.length];
				var newColor = this.chartColors[colorName];
				newDataset.backgroundColor.push(newColor);
			}

			config.data.datasets.push(newDataset);
			myDoughnut.update();
		};

		const addData = () => {
			if (config.data.datasets.length > 0) {
				config.data.labels.push('data #' + config.data.labels.length);

				var colorName = colorNames[config.data.datasets[0].data.length % colorNames.length];
				var newColor = this.chartColors[colorName];

				config.data.datasets.forEach(function (dataset) {
					dataset.data.push(randomScalingFactor());
					dataset.backgroundColor.push(newColor);
				});

				myDoughnut.update();
			}
		};

		function removeDataset() {
			config.data.datasets.splice(0, 1);
			myDoughnut.update();
		}

		function removeData() {
			config.data.labels.splice(-1, 1); // remove the label first

			config.data.datasets.forEach(function (dataset) {
				dataset.data.pop();
				dataset.backgroundColor.pop();
			});

			myDoughnut.update();
		}

		function changeCircleSize() {
			if (myDoughnut.options.circumference === Math.PI) {
				myDoughnut.options.circumference = 2 * Math.PI;
				myDoughnut.options.rotation = -Math.PI / 2;
			} else {
				myDoughnut.options.circumference = Math.PI;
				myDoughnut.options.rotation = -Math.PI;
			}

			myDoughnut.update();
		}

		setTimeout(() => {
			addData();
		}, 3000);
	}

	clock(canvas) {
		let scale = false;
		var ctx = canvas.getContext('2d');
		function clock() {
			var now = new Date();
			ctx.save();

			ctx.clearRect(0, 0, 150, 150);
			ctx.translate(75, 75);
			ctx.scale(0.4, 0.4);
			ctx.rotate(-Math.PI / 2);
			ctx.strokeStyle = 'black';
			ctx.fillStyle = 'white';
			ctx.lineWidth = 8;
			ctx.lineCap = 'round';

			// Hour marks
			ctx.save();
			for (var i = 0; i < 12; i++) {
				ctx.beginPath();
				ctx.rotate(Math.PI / 6);
				ctx.moveTo(100, 0);
				ctx.lineTo(120, 0);
				ctx.stroke();
			}
			ctx.restore();

			// Minute marks
			ctx.save();
			ctx.lineWidth = 5;
			for (i = 0; i < 60; i++) {
				if (i % 5 != 0) {
					ctx.beginPath();
					ctx.moveTo(117, 0);
					ctx.lineTo(120, 0);
					ctx.stroke();
				}
				ctx.rotate(Math.PI / 30);
			}
			ctx.restore();

			var sec = now.getSeconds();
			var min = now.getMinutes();
			var hr = now.getHours();
			hr = hr >= 12 ? hr - 12 : hr;

			ctx.fillStyle = 'black';

			// write Hours
			ctx.save();
			ctx.rotate(hr * (Math.PI / 6) + (Math.PI / 360) * min + (Math.PI / 21600) * sec);
			ctx.lineWidth = 14;
			ctx.beginPath();
			ctx.moveTo(-20, 0);
			ctx.lineTo(80, 0);
			ctx.stroke();
			ctx.restore();

			// write Minutes
			ctx.save();
			ctx.rotate((Math.PI / 30) * min + (Math.PI / 1800) * sec);
			ctx.lineWidth = 10;
			ctx.beginPath();
			ctx.moveTo(-28, 0);
			ctx.lineTo(112, 0);
			ctx.stroke();
			ctx.restore();

			// Write seconds
			ctx.save();
			ctx.rotate((sec * Math.PI) / 30);
			ctx.strokeStyle = '#D40000';
			ctx.fillStyle = '#D40000';
			ctx.lineWidth = 6;
			ctx.beginPath();
			ctx.moveTo(-30, 0);
			ctx.lineTo(83, 0);
			ctx.stroke();
			ctx.beginPath();
			ctx.arc(0, 0, 10, 0, Math.PI * 2, true);
			ctx.fill();
			ctx.beginPath();
			ctx.arc(95, 0, 10, 0, Math.PI * 2, true);
			ctx.stroke();
			ctx.fillStyle = 'rgba(0, 0, 0, 0)';
			ctx.arc(0, 0, 3, 0, Math.PI * 2, true);
			ctx.fill();
			ctx.restore();

			ctx.beginPath();
			ctx.lineWidth = 14;
			ctx.strokeStyle = '#325FA2';
			ctx.arc(0, 0, 142, 0, Math.PI * 2, true);
			ctx.stroke();

			ctx.restore();

			requestAnimationFrame(clock);
		}

		requestAnimationFrame(clock);
	}

	async solar(canvas) {
		var sun = new global.ImageAsset();
		var moon = new global.ImageAsset();
		var earth = new global.ImageAsset();

		await Promise.all([sun.fromUrl('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_sun.png'), moon.fromUrl('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_moon.png'), earth.fromUrl('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_earth.png')]);
		//	 sun.fromUrlSync('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_sun.png');
		//	 moon.fromUrlSync('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_moon.png');
		//	 earth.fromUrlSync('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_earth.png');

		// sun.fromUrl('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_sun.png')
		// .then(done =>{
		// 	console.log('sun', done);
		// 	return moon.fromUrl('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_moon.png')
		// }).then(done =>{
		// 	console.log('moon', done);
		// 	return earth.fromUrl('https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_earth.png');
		// }).then(done =>{
		// 	console.log('earth', done);
		// })

		//console.log(sun.width, moon.width, earth.width);
		var ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		//ctx.scale(Screen.mainScreen.scale, Screen.mainScreen.scale);

		//ctx.scale(3, 3);
		function init() {
			requestAnimationFrame(draw);
		}

		let didScale = false;

		function draw() {
			if (!ctx) {
				return;
			}

			ctx.globalCompositeOperation = 'destination-over';
			ctx.clearRect(0, 0, 300, 300); // clear canvas
			ctx.fillStyle = 'rgba(0, 0, 0, 0.4)';
			ctx.strokeStyle = 'rgba(0, 153, 255, 0.4)';
			ctx.save();
			ctx.translate(150, 150);

			// Earth
			var time = new Date();
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
			ctx.beginPath();
			ctx.arc(150, 150, 105, 0, Math.PI * 2, false); // Earth orbit
			ctx.stroke();
			ctx.drawImage(sun, 0, 0, 300, 300);

			// // if (!didScale) {
			// //     ctx.scale(canvas.clientWidth / 300, canvas.clientHeight / 300);
			// //     didScale = true;
			// // }
			requestAnimationFrame(draw);
		}

		init();
	}

	/* TODO after SVG
	import * as d3 from 'd3';
	d3Example(canvas){
			var mouse = [480, 250],
			count = 0;

	var svg = d3.select("body").append("svg")
			.attr("width", 960)
			.attr("height", 500);

	var g = svg.selectAll("g")
			.data(d3.range(25))
		.enter().append("g")
			.attr("transform", "translate(" + mouse + ")");

	g.append("rect")
			.attr("rx", 6)
			.attr("ry", 6)
			.attr("x", -12.5)
			.attr("y", -12.5)
			.attr("width", 25)
			.attr("height", 25)
			.attr("transform", function(d, i) { return "scale(" + (1 - d / 25) * 20 + ")"; })
			.style("fill", d3.scale.category20c());

	g.datum(function(d) {
		return {center: mouse.slice(), angle: 0};
	});

	svg.on("mousemove", function() {
		mouse = d3.mouse(this);
	});

	d3.timer(function() {
		count++;
		g.attr("transform", function(d, i) {
			d.center[0] += (mouse[0] - d.center[0]) / (i + 5);
			d.center[1] += (mouse[1] - d.center[1]) / (i + 5);
			d.angle += Math.sin((count + i) / 10) * 7;
			return "translate(" + d.center + ")rotate(" + d.angle + ")";
		});
	});
	}
	*/
	chartColors = {
		red: 'rgb(255, 99, 132)',
		orange: 'rgb(255, 159, 64)',
		yellow: 'rgb(255, 205, 86)',
		green: 'rgb(75, 192, 192)',
		blue: 'rgb(54, 162, 235)',
		purple: 'rgb(153, 102, 255)',
		grey: 'rgb(201, 203, 207)',
	};

	MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];

	COLORS = ['#4dc9f6', '#f67019', '#f53794', '#537bc4', '#acc236', '#166a8f', '#00a950', '#58595b', '#8549ba'];

	utils = {
		// Adapted from http://indiegamr.com/generate-repeatable-random-numbers-in-js/
		srand: function (seed) {
			this._seed = seed;
		},

		rand: function (min?, max?) {
			var seed = this._seed;
			min = min === undefined ? 0 : min;
			max = max === undefined ? 1 : max;
			this._seed = (seed * 9301 + 49297) % 233280;
			return min + (this._seed / 233280) * (max - min);
		},

		numbers: function (config) {
			var cfg = config || {};
			var min = cfg.min || 0;
			var max = cfg.max || 1;
			var from = cfg.from || [];
			var count = cfg.count || 8;
			var decimals = cfg.decimals || 8;
			var continuity = cfg.continuity || 1;
			var dfactor = Math.pow(10, decimals) || 0;
			var data = [];
			var i, value;

			for (i = 0; i < count; ++i) {
				value = (from[i] || 0) + this.rand(min, max);
				if (this.rand() <= continuity) {
					data.push(Math.round(dfactor * value) / dfactor);
				} else {
					data.push(null);
				}
			}

			return data;
		},

		labels: function (config) {
			var cfg = config || {};
			var min = cfg.min || 0;
			var max = cfg.max || 100;
			var count = cfg.count || 8;
			var step = (max - min) / count;
			var decimals = cfg.decimals || 8;
			var dfactor = Math.pow(10, decimals) || 0;
			var prefix = cfg.prefix || '';
			var values = [];
			var i;

			for (i = min; i < max; i += step) {
				values.push(prefix + Math.round(dfactor * i) / dfactor);
			}

			return values;
		},

		months: (config) => {
			var cfg = config || {};
			var count = cfg.count || 12;
			var section = cfg.section;
			var values = [];
			var i, value;

			for (i = 0; i < count; ++i) {
				value = this.MONTHS[Math.ceil(i) % 12];
				values.push(value.substring(0, section));
			}

			return values;
		},

		color: (index) => {
			return this.COLORS[index % this.COLORS.length];
		},

		transparentize: (color, opacity?) => {
			var alpha = (opacity === undefined ? 0.5 : 1 - opacity) * 255;
			const c = new Color(color);
			const newColor = new Color(alpha, c.r, c.g, c.b);
			return `rgba(${newColor.r},${newColor.g},${newColor.b},${newColor.a / 255})`;
		},
	};

	randomScalingFactor() {
		return Math.round(Math.random() * 100);
	}

	polarChart(canvas) {
		var DATA_COUNT = 7;

		this.utils.srand(110);

		function colorize(opaque, hover, ctx) {
			var v = ctx.dataset.data[ctx.dataIndex];
			var c = v < 35 ? '#D60000' : v < 55 ? '#F46300' : v < 75 ? '#0358B6' : '#44DE28';

			var opacity = hover ? 1 - Math.abs(v / 150) - 0.2 : 1 - Math.abs(v / 150);

			return opaque ? c : this.utils.transparentize(c, opacity);
		}

		function hoverColorize(ctx) {
			return colorize(false, true, ctx);
		}

		function generateData() {
			return this.utils.numbers({
				count: DATA_COUNT,
				min: 0,
				max: 100,
			});
		}

		var data = {
			datasets: [
				{
					data: generateData(),
				},
			],
		};

		var options = {
			legend: false,
			tooltips: false,
			elements: {
				arc: {
					backgroundColor: colorize.bind(null, false, false),
					hoverBackgroundColor: hoverColorize,
				},
			},
		};

		var chart = new Chart(canvas, {
			type: 'polarArea',
			data: data,
			options: options,
		});

		// eslint-disable-next-line no-unused-vars
		function randomize() {
			chart.data.datasets.forEach(function (dataset) {
				dataset.data = generateData();
			});
			chart.update();
		}

		// eslint-disable-next-line no-unused-vars
		var addData = function () {
			var newData = Math.round(Math.random() * 100);
			chart.data.datasets[0].data.push(newData);
			chart.update();
		};

		// eslint-disable-next-line no-unused-vars
		function removeData() {
			chart.data.datasets[0].data.pop();
			chart.update();
		}

		setTimeout(() => {
			addData();

			setTimeout(() => {
				randomize();

				setTimeout(() => {
					removeData();
				}, 3000);
			}, 3000);
		}, 3000);
	}

	bubbleChart(canvas) {
		var DATA_COUNT = 16;
		var MIN_XY = -150;
		var MAX_XY = 100;

		this.utils.srand(110);

		function colorize(opaque, context) {
			var value = context.dataset.data[context.dataIndex];
			var x = value.x / 100;
			var y = value.y / 100;
			var r = x < 0 && y < 0 ? 250 : x < 0 ? 150 : y < 0 ? 50 : 0;
			var g = x < 0 && y < 0 ? 0 : x < 0 ? 50 : y < 0 ? 150 : 250;
			var b = x < 0 && y < 0 ? 0 : x > 0 && y > 0 ? 250 : 150;
			var a = opaque ? 1 : (0.5 * value.v) / 1000;
			return 'rgba(' + r + ',' + g + ',' + b + ',' + a + ')';
		}

		const generateData = () => {
			var data = [];
			var i;

			for (i = 0; i < DATA_COUNT; ++i) {
				data.push({
					x: this.utils.rand(MIN_XY, MAX_XY),
					y: this.utils.rand(MIN_XY, MAX_XY),
					v: this.utils.rand(0, 1000),
				});
			}

			return data;
		};

		var data = {
			datasets: [
				{
					data: generateData(),
				},
				{
					data: generateData(),
				},
			],
		};

		var options = {
			aspectRatio: canvas.width / canvas.height,
			devicePixelRatio: 1,
			legend: false,
			tooltips: false,
			responsive: true,
			elements: {
				point: {
					backgroundColor: colorize.bind(null, false),

					borderColor: colorize.bind(null, true),

					borderWidth: function (context) {
						return Math.min(Math.max(1, context.datasetIndex + 1), 8);
					},

					hoverBackgroundColor: 'transparent',

					hoverBorderColor: (context) => {
						return this.utils.color(context.datasetIndex);
					},

					hoverBorderWidth: function (context) {
						var value = context.dataset.data[context.dataIndex];
						return Math.round((8 * value.v) / 1000);
					},

					radius: function (context) {
						var value = context.dataset.data[context.dataIndex];
						var size = context.chart.width;
						var base = Math.abs(value.v) / 1000;
						return (size / 24) * base;
					},
				},
			},
		};

		var chart = new Chart(canvas.getContext('2d'), {
			type: 'bubble',
			data: data,
			options: options,
		});

		// eslint-disable-next-line no-unused-vars
		function randomize() {
			chart.data.datasets.forEach(function (dataset) {
				dataset.data = generateData();
			});
			chart.update();
		}

		// eslint-disable-next-line no-unused-vars
		function addDataset() {
			chart.data.datasets.push({
				data: generateData(),
			});
			chart.update();
		}

		// eslint-disable-next-line no-unused-vars
		function removeDataset() {
			chart.data.datasets.shift();
			chart.update();
		}

		setTimeout(() => {
			addDataset();
		}, 5000);
	}

	hBarChart(canvas) {
		var color = Chart.helpers.color;
		var horizontalBarChartData = {
			labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
			datasets: [
				{
					label: 'Dataset 1',
					backgroundColor: this.utils.transparentize(this.chartColors.red, 0.5),
					borderColor: this.chartColors.red,
					borderWidth: 1,
					data: [this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor()],
				},
				{
					label: 'Dataset 2',
					backgroundColor: this.utils.transparentize(this.chartColors.blue, 0.5),
					borderColor: this.chartColors.blue,
					data: [this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor()],
				},
			],
		};

		const myHorizontalBar = new Chart(canvas, {
			type: 'horizontalBar',
			data: horizontalBarChartData,
			options: {
				// Elements options apply to all of the options unless overridden in a dataset
				// In this case, we are setting the border of each horizontal bar to be 2px wide
				elements: {
					rectangle: {
						borderWidth: 2,
					},
				},
				responsive: true,
				maintainAspectRatio: false,
				legend: {
					position: 'right',
				},
				title: {
					display: true,
					text: 'Chart.js Horizontal Bar Chart',
				},
			},
		});

		const randomizeData = () => {
			var zero = Math.random() < 0.2;
			horizontalBarChartData.datasets.forEach((dataset) => {
				dataset.data = dataset.data.map(() => {
					return zero ? 0.0 : this.randomScalingFactor();
				});
			});
			myHorizontalBar.update();
		};

		var colorNames = Object.keys(this.chartColors);

		const addDataset = () => {
			var colorName = colorNames[horizontalBarChartData.datasets.length % colorNames.length];
			var dsColor = this.chartColors[colorName];
			var newDataset = {
				label: 'Dataset ' + (horizontalBarChartData.datasets.length + 1),
				backgroundColor: color(dsColor).alpha(0.5).rgbString(),
				borderColor: dsColor,
				data: [],
			};

			for (var index = 0; index < horizontalBarChartData.labels.length; ++index) {
				newDataset.data.push(this.randomScalingFactor());
			}

			horizontalBarChartData.datasets.push(newDataset);
			myHorizontalBar.update();
		};

		const addData = () => {
			if (horizontalBarChartData.datasets.length > 0) {
				var month = this.MONTHS[horizontalBarChartData.labels.length % this.MONTHS.length];
				horizontalBarChartData.labels.push(month);

				for (var index = 0; index < horizontalBarChartData.datasets.length; ++index) {
					horizontalBarChartData.datasets[index].data.push(this.randomScalingFactor());
				}

				myHorizontalBar.update();
			}
		};

		function removeDataset() {
			horizontalBarChartData.datasets.pop();
			myHorizontalBar.update();
		}

		function removeData() {
			horizontalBarChartData.labels.splice(-1, 1); // remove the label first

			horizontalBarChartData.datasets.forEach(function (dataset) {
				dataset.data.pop();
			});

			myHorizontalBar.update();
		}
	}

	dataSets(canvas) {
		var presets = this.chartColors;
		var inputs = {
			min: 20,
			max: 80,
			count: 8,
			decimals: 2,
			continuity: 1,
		};

		const generateData = () => {
			return this.utils.numbers(inputs);
		};

		const generateLabels = () => {
			return this.utils.months({ count: inputs.count });
		};

		this.utils.srand(42);

		var data = {
			labels: generateLabels(),
			datasets: [
				{
					backgroundColor: this.utils.transparentize(presets.red),
					borderColor: presets.red,
					data: generateData(),
					hidden: true,
					label: 'D0',
				},
				{
					backgroundColor: this.utils.transparentize(presets.orange),
					borderColor: presets.orange,
					data: generateData(),
					label: 'D1',
					fill: '-1',
				},
				{
					backgroundColor: this.utils.transparentize(presets.yellow),
					borderColor: presets.yellow,
					data: generateData(),
					hidden: true,
					label: 'D2',
					fill: 1,
				},
				{
					backgroundColor: this.utils.transparentize(presets.green),
					borderColor: presets.green,
					data: generateData(),
					label: 'D3',
					fill: '-1',
				},
				{
					backgroundColor: this.utils.transparentize(presets.blue),
					borderColor: presets.blue,
					data: generateData(),
					label: 'D4',
					fill: '-1',
				},
				{
					backgroundColor: this.utils.transparentize(presets.grey),
					borderColor: presets.grey,
					data: generateData(),
					label: 'D5',
					fill: '+2',
				},
				{
					backgroundColor: this.utils.transparentize(presets.purple),
					borderColor: presets.purple,
					data: generateData(),
					label: 'D6',
					fill: false,
				},
				{
					backgroundColor: this.utils.transparentize(presets.red),
					borderColor: presets.red,
					data: generateData(),
					label: 'D7',
					fill: 8,
				},
				{
					backgroundColor: this.utils.transparentize(presets.orange),
					borderColor: presets.orange,
					data: generateData(),
					hidden: true,
					label: 'D8',
					fill: 'end',
				},
			],
		};

		var options = {
			responsive: true,
			maintainAspectRatio: false,
			spanGaps: false,
			elements: {
				line: {
					tension: 0.000001,
				},
			},
			scales: {
				yAxes: [
					{
						stacked: true,
					},
				],
			},
			plugins: {
				filler: {
					propagate: false,
				},
				'samples-filler-analyser': {
					target: 'chart-analyser',
				},
			},
		};

		var chart = new Chart(canvas, {
			type: 'line',
			data: data,
			options: options,
		});

		// eslint-disable-next-line no-unused-vars
		function togglePropagate(btn) {
			chart.options.plugins.filler.propagate = btn.classList.toggle('btn-on');
			chart.update();
		}

		// eslint-disable-next-line no-unused-vars
		function toggleSmooth(btn) {
			var value = btn.classList.toggle('btn-on');
			chart.options.elements.line.tension = value ? 0.4 : 0.000001;
			chart.update();
		}

		// eslint-disable-next-line no-unused-vars
		function randomize() {
			chart.data.datasets.forEach(function (dataset) {
				dataset.data = generateData();
			});
			chart.update();
		}
	}

	chartJSPie(canvas) {
		var config = {
			type: 'pie',
			data: {
				datasets: [
					{
						data: [this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor(), this.randomScalingFactor()],
						backgroundColor: [this.chartColors.red, this.chartColors.orange, this.chartColors.yellow, this.chartColors.green, this.chartColors.blue],
						label: 'Dataset 1',
					},
				],
				labels: ['Red', 'Orange', 'Yellow', 'Green', 'Blue'],
			},
			options: {
				responsive: true,
				maintainAspectRatio: false,
			},
		};

		const myPie = new Chart(canvas.getContext('2d'), config);
	}

	chart;

	chartJS(canvas) {
		var ctx = canvas.getContext('2d');
		if (this.chart) {
			this.chart.resize();
		} else {
			this.chart = new Chart(ctx, {
				type: 'bar',
				data: {
					labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
					datasets: [
						{
							label: '# of Votes',
							data: [12, 19, 3, 5, 2, 3],
							backgroundColor: ['rgba(255, 99, 132, 0.2)', 'rgba(54, 162, 235, 0.2)', 'rgba(255, 206, 86, 0.2)', 'rgba(75, 192, 192, 0.2)', 'rgba(153, 102, 255, 0.2)', 'rgba(255, 159, 64, 0.2)'],
							borderColor: ['rgba(255, 99, 132, 1)', 'rgba(54, 162, 235, 1)', 'rgba(255, 206, 86, 1)', 'rgba(75, 192, 192, 1)', 'rgba(153, 102, 255, 1)', 'rgba(255, 159, 64, 1)'],
							borderWidth: 1,
						},
					],
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					scales: {
						yAxes: [
							{
								ticks: {
									beginAtZero: true,
								},
							},
						],
					},
				},
			});
		}
	}

	toggle(args) {
		//vm.set("show", !vm.get("show"));
		//  drawPatternWithCanvas(canvas);
	}

	clear2D(canvas) {
		const ctx = canvas.getContext('2d');
		ctx.fillColor = 'black';
		ctx.strokeColor = 'black';
		ctx.clearRect(0, 0, canvas.getMeasuredWidth(), canvas.getMeasuredHeight());
	}

	clearGL(canvas) {
		const ctx = canvas.getContext('webgl');
		ctx.clearColor(1, 1, 1, 1);
		ctx.clear(ctx.COLOR_BUFFER_BIT);
	}

	clearGL2(canvas) {
		const ctx = canvas.getContext('webgl2');
		ctx.clearColor(1, 1, 1, 1);
		ctx.clear(ctx.COLOR_BUFFER_BIT);
	}

	changeTextColor(value) {
		this.set('textColor', value);
	}

	last;

	cleanup() {
		switch (this.last) {
			case 'swarm':
				cancelSwarm();
				break;
			case 'colorRain':
				cancelRain();
				break;
			case 'particlesLarge':
				cancelParticlesLarge();
				break;
			case 'rainbowOctopus':
				cancelRainbowOctopus();
				break;
			case 'particlesColor':
				cancelParticlesColor();
				break;
			case 'textures':
				break;
			case 'drawElements':
				break;
			case 'interactiveCube':
				cancelInteractiveCube();
				break;
			case 'main':
				cancelMain();
				break;
			case 'draw_instanced':
				break;
			case 'draw_image_space':
				break;
			case 'fog':
				cancelFog();
				break;
			case 'environmentMap':
				cancelEnvironmentMap();
				break;
			default:
				break;
		}
	}

	save2D() {
		if (!this.last) {
			this.canvas.getContext('2d').save();
		}
	}

	restore2D() {
		this.canvas.getContext('2d').restore();
	}

	tap(args) {
		const type = args.view.bindingContext.type || null;
		this.changeTextColor('white');
		this.cleanup();
		switch (type) {
			case 'swarm':
				this.save2D();
				this.clear2D(this.canvas);
				this.restore2D();
				swarm(this.canvas);
				this.changeTextColor('white');
				this.last = 'swarm';
				break;
			case 'colorRain':
				this.save2D();
				this.clear2D(this.canvas);
				this.restore2D();
				colorRain(this.canvas);
				this.changeTextColor('white');
				this.last = 'colorRain';
				break;
			case 'particlesLarge':
				this.save2D();
				this.clear2D(this.canvas);
				this.restore2D();
				particlesLarge(this.canvas);
				this.changeTextColor('black');
				this.last = 'particlesLarge';
				break;
			case 'rainbowOctopus':
				this.save2D();
				this.clear2D(this.canvas);
				this.restore2D();
				rainbowOctopus(this.canvas);
				this.changeTextColor('black');
				this.last = 'rainbowOctopus';
				break;
			case 'particlesColor':
				this.save2D();
				this.clear2D(this.canvas);
				this.restore2D();
				particlesColor(this.canvas);
				this.changeTextColor('white');
				this.last = 'particlesColor';
				break;
			case 'textures':
				this.clearGL(this.canvas);
				textures(this.canvas);
				this.changeTextColor('white');
				this.last = 'textures';
				break;
			case 'drawElements':
				this.clearGL(this.canvas);
				drawElements(this.canvas);
				this.changeTextColor('black');
				this.last = 'drawElements';
				break;
			case 'drawModes':
				this.clearGL(this.canvas);
				drawModes(this.canvas);
				this.changeTextColor('black');
				this.last = 'drawModes';
				break;
			case 'interactiveCube':
				this.clearGL(this.canvas);
				interactiveCube(this.canvas);
				this.last = 'interactiveCube';
				break;
			case 'main':
				this.clearGL(this.canvas);
				main(this.canvas);
				this.changeTextColor('white');
				break;
			case 'draw_instanced':
				this.clearGL2(this.canvas);
				draw_instanced(this.canvas);
				this.last = 'draw_instanced';
				break;
			case 'draw_image_space':
				this.clearGL2(this.canvas);
				draw_image_space(this.canvas);
				this.last = 'draw_image_space';
				break;
			case 'cubeRotationRotation':
				this.clearGL2(this.canvas);
				cubeRotationRotation(this.canvas);
				this.last = 'draw_image_space';
				this.changeTextColor('black');
				break;
			case 'fog':
				this.clearGL2(this.canvas);
				fog(this.canvas);
				this.last = 'fog';
				break;
			case 'environmentMap':
				this.clearGL2(this.canvas);
				environmentMap(this.canvas);
				this.last = 'environmentMap';
				break;
			default:
				break;
		}
	}

	clear(args) {
		// ctx.clearRect(0, 0, canvas.getMeasuredWidth(), canvas.getMeasuredHeight());
		// ctx.fillStyle = 'red'
		///ctx.fillRect(0, 0, canvas.getMeasuredWidth(), canvas.getMeasuredHeight());
		//const imageData = ctx.createImageData(100, 100);
		/*
		// Iterate through every pixel
		for (let i = 0; i < imageData.data.length; i += 4) {
				// Modify pixel data
				imageData.data[i] = 190; // R value
				imageData.data[i + 1] = 0; // G value
				imageData.data[i + 2] = 210; // B value
				imageData.data[i + 3] = 255; // A value
		}
		ctx.createImageData(imageData);

		// Draw image data to the canvas
		ctx.putImageData(imageData, 20, 20);

		*/
	}

	show: boolean = true;
	textColor: string = 'black';

	items = new ObservableArray([
		{ name: '2D Swarm', type: 'swarm' },
		{ name: '2D ColorRain', type: 'colorRain' },
		{ name: '2D Particles Large', type: 'particlesLarge' },
		{ name: '2D Rainbow Octopus', type: 'rainbowOctopus' },
		{ name: '2D Particles Color', type: 'particlesColor' },
		{ name: 'WEBGL textures', type: 'textures' },
		{ name: 'WEBGL Draw Elements', type: 'drawElements' },
		{ name: 'WEBGL Draw Modes', type: 'drawModes' },
		{ name: 'WEBGL InteractiveCube', type: 'interactiveCube' },
		{ name: 'WEBGL Cube Rotation With Image', type: 'main' },
		{ name: 'WEBGL2 Draw Instanced', type: 'draw_instanced' },
		{ name: 'WEBGL2 Draw ImageSpace', type: 'draw_image_space' },
		{
			name: 'WEBGL2 Cube Rotation With Cube Roating inside',
			type: 'cubeRotationRotation',
		},
		{ name: 'WEBGL2 Fog', type: 'fog' },
		{
			name: 'WEBGL2 Environment Map Roatating Cube',
			type: 'environmentMap',
		},
	]);

	arcAnimation(ctx) {
		ctx.scale(2, 2);
		const mouse = { x: 0, y: 0 };

		let r = 100; // Radius
		const p0 = { x: 0, y: 50 };

		const p1 = { x: 100, y: 100 };
		const p2 = { x: 150, y: 50 };
		const p3 = { x: 200, y: 100 };

		const labelPoint = function (p, offset, i = 0) {
			const { x, y } = offset;
			ctx.beginPath();
			ctx.arc(p.x, p.y, 2, 0, Math.PI * 2);
			ctx.fill('');
			ctx.fillText(`${i}:(${p.x}, ${p.y})`, p.x + x, p.y + y);
		};

		const drawPoints = function (points) {
			for (let i = 0; i < points.length; i++) {
				var p = points[i];
				labelPoint(p, { x: 0, y: -20 }, i);
			}
		};

		// Draw arc
		const drawArc = function ([p0, p1, p2], r) {
			ctx.beginPath();
			ctx.moveTo(p0.x, p0.y);
			ctx.arcTo(p1.x, p1.y, p2.x, p2.y, r);
			ctx.lineTo(p2.x, p2.y);
			ctx.stroke();
		};

		let t0 = 0;
		let rr = 0; // the radius that changes over time
		let a = 0; // angle
		let PI2 = Math.PI * 2;
		const loop = (t) => {
			t0 = t / 1000;
			a = t0 % PI2;
			rr = Math.abs(Math.cos(a) * r);

			ctx.clearRect(0, 0, this.canvas.getMeasuredWidth(), this.canvas.getMeasuredHeight());

			drawArc([p1, p2, p3], rr);
			drawPoints([p1, p2, p3]);
			requestAnimationFrame(loop);
		};

		loop(0);
	}

	context2DTest(canvas) {
		const ctx = canvas.getContext('2d');
		ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);
		let numCircles = 0;
		// Measure performance outside requestAnimationFrame
		const width = canvas.width;
		const height = canvas.height;

		let fillStyle = 0;
		let beginPath = 0;
		let arc = 0;
		let closePath = 0;
		let fill = 0;
		let nonRafStartTime = performance.now();
		for (let i = 0; i < 100000; i++) {
			const x = Math.random() * width;
			const y = Math.random() * height;
			const radius = Math.random() * 20;
			const color = `rgb(${Math.random() * 255}, ${Math.random() * 255}, ${Math.random() * 255})`;
			const a = Date.now();
			ctx.fillStyle = color;
			fillStyle += Date.now() - a;

			const b = Date.now();
			ctx.beginPath();
			beginPath += Date.now() - b;

			const c = Date.now();
			ctx.arc(x, y, radius, 0, 2 * Math.PI);
			arc += Date.now() - c;

			const d = Date.now();
			ctx.closePath();
			closePath += Date.now() - d;

			const e = Date.now();
			ctx.fill();
			fill += Date.now() - e;
		}
		const nonRafEndTime = performance.now();
		console.log('Drawing 100000 circles without RAF took', nonRafEndTime - nonRafStartTime, 'milliseconds');

		console.log('fillStyle', fillStyle, 'beginPath', beginPath, 'arc', arc, 'closePath', closePath, 'fill', fill);
		const draw = () => {
			const x = Math.random() * width;
			const y = Math.random() * height;
			const radius = Math.random() * 20;
			const color = `rgb(${Math.random() * 255}, ${Math.random() * 255}, ${Math.random() * 255})`;
			ctx.fillStyle = color;
			ctx.beginPath();
			ctx.arc(x, y, radius, 0, 2 * Math.PI);
			ctx.closePath();
			ctx.fill();

			numCircles++;

			if (numCircles < 200) {
				requestAnimationFrame(draw);
			} else {
				const rafEndTime = performance.now();
				console.log('Drawing 200 circles with RAF took', rafEndTime - rafStartTime, 'milliseconds');
			}
		};

		// Measure performance inside requestAnimationFrame
		ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);
		let rafStartTime = performance.now();
		requestAnimationFrame(draw);
	}
}
