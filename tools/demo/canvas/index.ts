import { DemoSharedBase } from '../utils';
import { ImageSource, ObservableArray, Screen, Color } from '@nativescript/core';
import Chart from 'chart.js';

let Matter;
import { Canvas, ImageAsset } from '@nativescript/canvas';
import {
	arc,
	arcTo,
	cancelParticlesColor,
	cancelParticlesLarge,
	cancelRain,
	cancelRainbowOctopus,
	cancelSwarm, clip,
	colorRain, createLinearGradient, createRadialGradient, ellipse, fillRule, imageBlock, imageSmoothingEnabled,
	imageSmoothingQuality,
	isPointInStrokeTouch,
	march, multiStrokeStyle,
	particlesColor,
	particlesLarge,
	patternWithCanvas,
	rainbowOctopus, shadowBlur,
	swarm,
	touchParticles
} from "./canvas2d";


declare var NSData, interop, NSString, malloc, TNSCanvas;
//const CanvasWorker = require('nativescript-worker-loader!./canvas.worker.js');
import Vex from 'vexflow';
import {
	cancelInteractiveCube,
	cancelMain, cubeRotation,
	cubeRotationRotation,
	drawElements,
	drawModes, imageFilter,
	interactiveCube,
	main,
	textures
} from "./webgl";
import { cancelEnvironmentMap, cancelFog, draw_image_space, draw_instanced, environmentMap, fog } from "./webgl2";
declare var com, java;
let zen3d

export class DemoSharedCanvas extends DemoSharedBase {
	private canvas: any;

	canvasLoaded(args) {
		this.canvas = args.object;
		this.draw();
	}

	draw() {
		// const worker = new CanvasWorker();
		// canvas.parent.on(GestureTypes.touch as any, (args: TouchGestureEventData) => {
		//     var x = args.getX() * Screen.mainScreen.scale,
		//         y = (args.getY() * Screen.mainScreen.scale);
		//     worker.postMessage({event: 'touch', x, y})
		// });
		// if (isAndroid) {
		//     canvas.android.setHandleInvalidationManually(true);
		//     (com.github.triniwiz.canvas.CanvasView as any).getViews().put(
		//         `${canvas._domId}`, new java.lang.ref.WeakReference(canvas.android)
		//     );
		// } else {
		//     canvas.ios.handleInvalidationManually = true;
		//     canvas.ios.moveOffMain();
		//     Canvas.getViews().setObjectForKey(canvas.ios, `${canvas._domId}`);
		// }
		// const w = canvas.getMeasuredWidth(),
		//     h = canvas.getMeasuredHeight();
		// worker.postMessage({id: `${canvas._domId}`, width: w, height: h});
		// worker.onerror = msg => {
		//     console.log('error', msg);
		// }

		// swarm(canvas);

		// touchParticles(canvas);

		// var map = L.map('map', {
		//     center: [51.505, -0.09],
		//     zoom: 13
		// });
		// this.vexFlow(this.canvas);
		// canvas.android.setHandleInvalidationManually(true);
		//const ctx = canvas.getContext('2d');
		//	fillRule(this.canvas);
		//const ctx = this.canvas.getContext('2d');
		//clip(this.canvas);
		//fillStyle(this.canvas);
		// font(this.canvas);
		// globalAlpha(this.canvas);
		//globalCompositeOperation(this.canvas);
		//imageSmoothingEnabled(this.canvas);
		//imageSmoothingQuality(this.canvas);
		//lineCap(this.canvas);
		//lineDashOffset(this.canvas);
		//lineJoin(this.canvas);
		//lineWidth(this.canvas);
		// miterLimit(this.canvas);
		//shadowBlur(this.canvas);
		//shadowColor(this.canvas);
		//shadowOffsetX(this.canvas);
		//shadowOffsetY(this.canvas);
		// strokeStyle(this.canvas);
		//multiStrokeStyle(this.canvas);
		//textAlign(this.canvas)

		//arc(this.canvas);
		//arcMultiple(this.canvas);

		//arcTo(this.canvas);
		// arcToAnimation(this.canvas);

		// ellipse(this.canvas);

		// fillPath(this.canvas);
		//imageBlock(this.canvas);
		// scale(this.canvas);
		//pattern(this.canvas);
		// patternWithCanvas(this.canvas);
		//isPointInStrokeTouch(this.canvas);
		//createLinearGradient(this.canvas);
		//createRadialGradient(this.canvas);
		//march(this.canvas);
		this.putImageDataDemo(this.canvas);
		//	this.drawImage(this.canvas);
		// ctx.fillStyle = 'blue';
		// ctx.fillRect(0,0,400,400)
		//ellipse(this.canvas);
		// drawPatternWithCanvas(this.canvas);
		//this.clock(this.canvas);
		//this.solar(this.canvas);
		//console.log('ready ??');
		//this.coloredParticles(this.canvas);
		//this.ball(this.canvas)
		//swarm(this.canvas);
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
		//swarm(this.canvas);
		//textures(this.canvas)
		//drawModes(this.canvas,'triangles')
		//drawElements(this.canvas)
		// ctx = canvas.getContext("2d") as any;
		//swarm(this.canvas);
		// canvas.nativeView.handleInvalidationManually = true;
		//  setTimeout(() => {
		//draw_instanced(this.canvas);
		// draw_image_space(this.canvas);
		//fog(this.canvas);
		//environmentMap(this.canvas);
		//cubeRotationRotation(this.canvas);
		//main(this.canvas);
		// imageFilter(this.canvas);
		// interactiveCube(this.canvas);
		//textures(this.canvas);
		//drawElements(this.canvas)
		//drawModes(this.canvas,'line_strip')
		//fog(this.canvas);
		// }, 1000);
		//cubeRotation(this.canvas);
		//},3000)
		// drawModes(this.canvas,'triangles')
		//cubeRotation(this.canvas);
		//main(this.canvas)
		//this.pointStyle(this.canvas);
		// this.matterJSExample(this.canvas);
		//this.matterJSCar(this.canvas);
		//this.multiCanvas(this.canvas);
		// triangle(this.canvas);
		//this.zen3dCube(this.canvas);
		//this.zen3dGeometryLoaderGltf(this.canvas);
		//this.playCanvas(this.canvas);
		//this.drawRandomFullscreenImage(this.canvas);
	}

	drawRandomFullscreenImage(canvas) {
		const width = Screen.mainScreen.widthPixels;
		const height = Screen.mainScreen.heightPixels;
		const ctx = canvas.getContext('2d');
		const image = new Image();
		image.onload = () => {
			ctx.drawImage(image, 0, 0);
		}
		image.src = `https://source.unsplash.com/random/${width}x${height}`;
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
			type: 'box'
		});
		app.root.addChild(box);

		// create camera entity
		const camera = new pc.Entity('camera');
		camera.addComponent('camera', {
			clearColor: new pc.Color(0.1, 0.1, 0.1)
		});
		app.root.addChild(camera);
		camera.setPosition(0, 0, 3);

		// create directional light entity
		const light = new pc.Entity('light');
		light.addComponent('light');
		app.root.addChild(light);
		light.setEulerAngles(45, 0, 0);

		// rotate the box according to the delta time since the last frame
		app.on('update', dt => box.rotate(10 * dt, 20 * dt, 30 * dt));

		app.start();
	}

	gridLoaded(args) {
		const grid = args.object;
		this.removeClipping(grid);
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
		var gl = canvas.getContext("webgl2", {
			antialias: true,
			alpha: false,
			stencil: true
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
		camera.setPerspective(45 / 180 * Math.PI, width / height, 1, 1000);
		scene.add(camera);

		function loop(count) {
			requestAnimationFrame(loop);

			mesh.euler.y = count / 1000 * .5; // rotate cube

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

			camera.setPerspective(45 / 180 * Math.PI, width / height, 1, 1000);

			backRenderTarget.resize(width, height);
		}

		window.addEventListener("resize", onWindowResize, false);
	}

	zen3dGeometryLoaderGltf(canvas) {

		if (zen3d === undefined) {
			zen3d = require('zen-3d');
			(global as any).zen3d = zen3d;
		}

		const zen3dRoot = '~/assets/file-assets/zen3d/';
		require("./zen3d/js/objects/SkyBox.js");
		require('./zen3d/js/loaders/GLTFLoader.js');
		require('./zen3d/js/controls/OrbitControls.js');

		var renderer = new zen3d.Renderer(canvas);
		let gl = canvas.getContext('webgl2');
		if (!gl) {
			gl = canvas.getContext('webgl');
		}

		const { drawingBufferWidth, drawingBufferHeight } = gl;
		let width = drawingBufferWidth;
		let height = drawingBufferHeight;
		var scene = new zen3d.Scene();

		var file = "~/assets/three/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf";
		var cube_texture = zen3d.TextureCube.fromSrc([
			zen3dRoot + "Bridge2/posx.jpg",
			zen3dRoot + "Bridge2/negx.jpg",
			zen3dRoot + "Bridge2/posy.jpg",
			zen3dRoot + "Bridge2/negy.jpg",
			zen3dRoot + "Bridge2/posz.jpg",
			zen3dRoot + "Bridge2/negz.jpg"
		]);
		var sky_box = new zen3d.SkyBox(cube_texture);
		sky_box.level = 4;

		let objectMaterial;


		// var nanobar = new Nanobar();
		// nanobar.el.style.background = "gray";

		var loadingManager = new zen3d.LoadingManager(function () {
			//  nanobar.go(100);
			//  nanobar.el.style.background = "transparent";
		}, function (url, itemsLoaded, itemsTotal) {
			if (itemsLoaded < itemsTotal) {
				// nanobar.go(itemsLoaded / itemsTotal * 100);
			}
		});


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
		camera.setPerspective(45 / 180 * Math.PI, width / height, 1, 8000);
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
			width = drawingBufferWidth;
			height = drawingBufferHeight;

			camera.setPerspective(45 / 180 * Math.PI, width / height, 1, 8000);

			renderer.backRenderTarget.resize(width, height);
		}

		window.addEventListener("resize", onWindowResize, false);
	}

	async drawImage(context) {
		var sun = await ImageSource.fromUrl(
			'https://mdn.mozillademos.org/files/1456/Canvas_sun.png'
		);
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

		console.log(utf8decoder.decode(u8arr));
		console.log(utf8decoder.decode(i8arr));
		console.log(utf8decoder.decode(u16arr));
		console.log(utf8decoder.decode(i16arr));
		console.log(utf8decoder.decode(i32arr));


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
				particles.push(new Factory);
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
			}
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
			World.add(
				world,
				Composites.car(150, 100, 150 * scale, 30 * scale, 30 * scale)
			);

			scale = 0.8;
			World.add(
				world,
				Composites.car(350, 300, 150 * scale, 30 * scale, 30 * scale)
			);

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
	}

	matterJSExample(canvas) {
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
	}

	multiCanvas(canvas) {
		if (canvas.id === 'canvas1') {
			swarm(canvas);
		}
		if (canvas.id === 'canvas2') {
			this.clock(canvas);
		}

		if (canvas.id === 'canvas3') {
			this.solar(canvas);
		}
		if (canvas.id === 'canvas4') {
			main(this.canvas);
		}
	}

	pointStyle(canvas) {
		const color = Chart.helpers.color;

		const createConfig = (colorName) => {
			return {
				type: 'line',
				data: {
					labels: [
						'January',
						'February',
						'March',
						'April',
						'May',
						'June',
						'July',
					],
					datasets: [
						{
							label: 'My First dataset',
							data: [
								this.randomScalingFactor(),
								this.randomScalingFactor(),
								this.randomScalingFactor(),
								this.randomScalingFactor(),
								this.randomScalingFactor(),
								this.randomScalingFactor(),
								this.randomScalingFactor(),
							],
							backgroundColor: color(this.chartColors[colorName])
								.alpha(0.5)
								.rgbString(),
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
		}

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
		}

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
		}

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
		}
	}


	drawPatternWithCanvas(canvas) {
		const patternCanvas = Canvas.createCustomView();

		const patternContext = patternCanvas.getContext('2d') as any;

		// Give the pattern a width and height of 50
		patternCanvas.width = 50;
		patternCanvas.height = 50;

		//  patternCanvas.getContext('2d') as any;

		const scale = Screen.mainScreen.scale;
		// Give the pattern a background color and draw an arc
		patternContext.fillStyle = '#fec';
		patternContext.fillRect(0, 0, patternCanvas.width, patternCanvas.height);
		patternContext.arc(0, 0, 50 * scale, 0, 0.5 * Math.PI);
		patternContext.stroke();

		// Create our primary canvas and fill it with the pattern
		const ctx = canvas.getContext('2d');
		ctx.fillStyle = ctx.createPattern(patternCanvas, 'repeat');
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
						data: [
							randomScalingFactor(),
							randomScalingFactor(),
							randomScalingFactor(),
							randomScalingFactor(),
							randomScalingFactor(),
						],
						backgroundColor: [
							this.chartColors.red,
							this.chartColors.orange,
							this.chartColors.yellow,
							this.chartColors.green,
							this.chartColors.blue,
						],
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
		}

		const addData = () => {
			if (config.data.datasets.length > 0) {
				config.data.labels.push('data #' + config.data.labels.length);

				var colorName =
					colorNames[
					config.data.datasets[0].data.length % colorNames.length
					];
				var newColor = this.chartColors[colorName];

				config.data.datasets.forEach(function (dataset) {
					dataset.data.push(randomScalingFactor());
					dataset.backgroundColor.push(newColor);
				});

				myDoughnut.update();
			}
		}

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
		ctx.scale(3, 3);

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
			ctx.rotate(
				hr * (Math.PI / 6) + (Math.PI / 360) * min + (Math.PI / 21600) * sec
			);
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

		var sun = new ImageAsset();
		var moon = new ImageAsset();
		var earth = new ImageAsset();

		try {
			await sun.loadFromUrlAsync(
				'https://mdn.mozillademos.org/files/1456/Canvas_sun.png'
			);
			await moon.loadFromUrlAsync(
				'https://mdn.mozillademos.org/files/1443/Canvas_moon.png'
			);
			await earth.loadFromUrlAsync(
				'https://mdn.mozillademos.org/files/1429/Canvas_earth.png'
			)

		} catch (e) {
			console.log('solar error:', e);
		}

		function init() {
			window.requestAnimationFrame(draw);
		}

		let didScale = false;

		function draw() {
			var ctx = canvas.getContext('2d');
			if(!ctx){
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
			ctx.rotate(
				((2 * Math.PI) / 60) * time.getSeconds() +
				((2 * Math.PI) / 60000) * time.getMilliseconds()
			);
			ctx.translate(105, 0);
			ctx.fillRect(0, -12, 40, 24); // Shadow
			ctx.drawImage(earth, -12, -12);

			// Moon
			ctx.save();
			ctx.rotate(
				((2 * Math.PI) / 6) * time.getSeconds() +
				((2 * Math.PI) / 6000) * time.getMilliseconds()
			);
			ctx.translate(0, 28.5);
			ctx.drawImage(moon, -3.5, -3.5);
			ctx.restore();

			ctx.restore();

			ctx.beginPath();
			ctx.arc(150, 150, 105, 0, Math.PI * 2, false); // Earth orbit
			ctx.stroke();

			ctx.drawImage(sun, 0, 0, 300, 300);

			// if (!didScale) {
			//     ctx.scale(canvas.clientWidth / 300, canvas.clientHeight / 300);
			//     didScale = true;
			// }

			window.requestAnimationFrame(draw);
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

	MONTHS = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December',
	];

	COLORS = [
		'#4dc9f6',
		'#f67019',
		'#f53794',
		'#537bc4',
		'#acc236',
		'#166a8f',
		'#00a950',
		'#58595b',
		'#8549ba',
	];

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
			return `rgba(${newColor.r},${newColor.g},${newColor.b},${newColor.a / 255
				})`;
		},
	}

	randomScalingFactor() {
		return Math.round(Math.random() * 100);
	}

	polarChart(canvas) {
		var DATA_COUNT = 7;

		this.utils.srand(110);

		function colorize(opaque, hover, ctx) {
			var v = ctx.dataset.data[ctx.dataIndex];
			var c =
				v < 35
					? '#D60000'
					: v < 55
						? '#F46300'
						: v < 75
							? '#0358B6'
							: '#44DE28';

			var opacity = hover
				? 1 - Math.abs(v / 150) - 0.2
				: 1 - Math.abs(v / 150);

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
		}

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
			labels: [
				'January',
				'February',
				'March',
				'April',
				'May',
				'June',
				'July',
			],
			datasets: [
				{
					label: 'Dataset 1',
					backgroundColor: this.utils.transparentize(this.chartColors.red, 0.5),
					borderColor: this.chartColors.red,
					borderWidth: 1,
					data: [
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
					],
				},
				{
					label: 'Dataset 2',
					backgroundColor: this.utils.transparentize(this.chartColors.blue, 0.5),
					borderColor: this.chartColors.blue,
					data: [
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
						this.randomScalingFactor(),
					],
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
		}

		var colorNames = Object.keys(this.chartColors);

		const addDataset = () => {
			var colorName =
				colorNames[
				horizontalBarChartData.datasets.length % colorNames.length
				];
			var dsColor = this.chartColors[colorName];
			var newDataset = {
				label: 'Dataset ' + (horizontalBarChartData.datasets.length + 1),
				backgroundColor: color(dsColor).alpha(0.5).rgbString(),
				borderColor: dsColor,
				data: [],
			};

			for (
				var index = 0;
				index < horizontalBarChartData.labels.length;
				++index
			) {
				newDataset.data.push(this.randomScalingFactor());
			}

			horizontalBarChartData.datasets.push(newDataset);
			myHorizontalBar.update();
		}

		const addData = () => {
			if (horizontalBarChartData.datasets.length > 0) {
				var month =
					this.MONTHS[horizontalBarChartData.labels.length % this.MONTHS.length];
				horizontalBarChartData.labels.push(month);

				for (
					var index = 0;
					index < horizontalBarChartData.datasets.length;
					++index
				) {
					horizontalBarChartData.datasets[index].data.push(
						this.randomScalingFactor()
					);
				}

				myHorizontalBar.update();
			}
		}

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
		}

		const generateLabels = () => {
			return this.utils.months({ count: inputs.count });
		}

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
						data: [
							this.randomScalingFactor(),
							this.randomScalingFactor(),
							this.randomScalingFactor(),
							this.randomScalingFactor(),
							this.randomScalingFactor(),
						],
						backgroundColor: [
							this.chartColors.red,
							this.chartColors.orange,
							this.chartColors.yellow,
							this.chartColors.green,
							this.chartColors.blue,
						],
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
							backgroundColor: [
								'rgba(255, 99, 132, 0.2)',
								'rgba(54, 162, 235, 0.2)',
								'rgba(255, 206, 86, 0.2)',
								'rgba(75, 192, 192, 0.2)',
								'rgba(153, 102, 255, 0.2)',
								'rgba(255, 159, 64, 0.2)',
							],
							borderColor: [
								'rgba(255, 99, 132, 1)',
								'rgba(54, 162, 235, 1)',
								'rgba(255, 206, 86, 1)',
								'rgba(75, 192, 192, 1)',
								'rgba(153, 102, 255, 1)',
								'rgba(255, 159, 64, 1)',
							],
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
		{ name: "2D Swarm", type: "swarm" },
		{ name: "2D ColorRain", type: "colorRain" },
		{ name: "2D Particles Large", type: "particlesLarge" },
		{ name: "2D Rainbow Octopus", type: "rainbowOctopus" },
		{ name: "2D Particles Color", type: "particlesColor" },
		{ name: "WEBGL textures", type: "textures" },
		{ name: "WEBGL Draw Elements", type: "drawElements" },
		{ name: "WEBGL Draw Modes", type: "drawModes" },
		{ name: "WEBGL InteractiveCube", type: "interactiveCube" },
		{ name: "WEBGL Cube Rotation With Image", type: "main" },
		{ name: "WEBGL2 Draw Instanced", type: "draw_instanced" },
		{ name: "WEBGL2 Draw ImageSpace", type: "draw_image_space" },
		{
			name: "WEBGL2 Cube Rotation With Cube Roating inside",
			type: "cubeRotationRotation",
		},
		{ name: "WEBGL2 Fog", type: "fog" },
		{
			name: "WEBGL2 Environment Map Roatating Cube",
			type: "environmentMap",
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

			ctx.clearRect(
				0,
				0,
				this.canvas.getMeasuredWidth(),
				this.canvas.getMeasuredHeight()
			);

			drawArc([p1, p2, p3], rr);
			drawPoints([p1, p2, p3]);
			requestAnimationFrame(loop);
		};

		loop(0);
	}

}
