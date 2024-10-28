import { DemoSharedBase } from '../utils';

import { Screen, knownFolders } from '@nativescript/core';

import { Application, Color, FillGradient, Graphics, Text, TextStyle } from 'pixi.js';
import '@nativescript/canvas-pixi';
import * as PIXI from 'pixi.js';
import { device } from '@nativescript/core/platform';
import { initDevtools } from '@pixi/devtools';
import { Canvas } from '@nativescript/canvas';

// import { Viewport } from 'pixi-viewport';

// import { SVGScene } from '@pixi-essentials/svg';
// import { Svg } from '@nativescript/canvas-svg';

// let PIXI;

interface Grid {
	width: number;
	height: number;
	color?: [number, number, number];
	lineThickness?: number;
	pitch?: { x: number; y: number };
}

const shaderCode = `
  precision mediump float;

  uniform float vpw;
  uniform float vph;
  uniform float thickness;

  uniform vec2 offset;
  uniform vec2 pitch;
  uniform vec4 color;

  void main() {
    float offX = ( offset[0]) + gl_FragCoord.x;
    float offY = ( offset[1]) + (vph-  gl_FragCoord.y);
    float rX = min(abs(pitch[0] - mod(offX, pitch[0])),
                   abs(mod(offX, pitch[0])));
    float rY = min(abs(pitch[1] - mod(offY, pitch[1])),
                   abs(mod(offY, pitch[1])));
    if ( int(rX) <= int(thickness/2.0) ||
         int(rY) <= int(thickness/2.0)  ) {
      gl_FragColor = color;
    } else {
      gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
    }
  }
`;
function makeGridFilter(grid: Grid) {
	const uniforms = {
		thickness: grid.lineThickness,
		color: [...(grid?.color ?? [1.0, 1.0, 1.0]), 1.0],
		vpw: grid.width * 2,
		vph: grid.height * 2,
		offset: [0, 0],
		pitch: [grid.pitch?.x ?? 50 * 2, grid.pitch?.y ?? 50 * 2],
	};

	return new global.PIXI.Filter(undefined, shaderCode, uniforms);
}

export class DemoSharedCanvasPixi extends DemoSharedBase {
	root = 'assets/pixi';

	loaded(args) {
		console.log('loaded', args.object);

		//this.root = knownFolders.currentApp().path + '/assets/pixi'
	}

	unloaded(args) {
		console.log('unloaded');
	}

	canvasLoaded(args) {
		const canvas = args.object;
		//	PIXI = require('@nativescript/canvas-pixi');
		// const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		// const canvas2 = document.createElement('canvas');
		// const ctx2 = canvas2.getContext('2d') as CanvasRenderingContext2D;

		// ctx.font = 'normal normal normal 150px times';
		// ctx2.font = 'normal normal normal 150px times';
		// ctx.shadowBlur = 10;
		// ctx2.shadowBlur = 10;
		// ctx.shadowColor = 'green';
		// ctx2.shadowColor = 'blue';
		// ctx.shadowOffsetX = 10;
		// ctx2.shadowOffsetX = 10;
		// ctx.shadowOffsetY = 10;
		// ctx2.shadowOffsetY = 10;
		// ctx.strokeText('Help!!!', 0, 150);
		// ctx2.strokeText('Help2!!!', 0, 150);
		// ctx.fillText('Help!!!', 0, 300);
		// ctx2.fillText('Help2!!!', 0, 300);
		// ctx.drawImage(canvas2, 0, 0);
		//this.text(canvas);

		//this.drawPatternWithCanvas(canvas);
		//this.simpleWebGPU(canvas);
		//this.simplePlane(canvas);
		//this.advance(canvas);
		//this.container(canvas);
		//this.explosion(canvas);
		//this.bitmapFont(canvas);

		//this.dynamicGraphics(canvas);
		//this.meshBasic(canvas);
		//this.meshAdvance(canvas);
		//this.renderTextureAdvance(canvas);
		//this.starWarp(canvas);
		//this.meshShader(canvas);
		//this.meshSharingGeo(canvas);
		//this.multiPassShaderGenMesh(canvas);
		//this.cacheAsBitmap(canvas);
		//this.blendModes(canvas);
		//this.particleContainer(canvas);
		//this.transparent(canvas);
		//this.textureRotate(canvas);
		//this.simplePlane(canvas);
		//this.animatedJet(canvas);
		//this.viewPort(canvas);
		this.svg(canvas);
	}

	/* Graphics */
	async simpleWebGPU(canvas) {
		const app = new PIXI.Application() as Application;
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;

		await app.init({
			canvas,
			preference: 'webgpu',
			width: canvas.width,
			height: canvas.height,
		});

		// grab context to present
		const ctx = canvas.getContext('webgpu');

		app.ticker.add((delta) => {
			if (ctx) {
				ctx.presentSurface();
			}
		});

		const graphics = new Graphics();

		// Rectangle
		graphics.rect(50, 50, 100, 100);
		graphics.fill(0xde3249);

		// Rectangle + line style 1
		graphics.rect(200, 50, 100, 100);
		graphics.fill(0x650a5a);
		graphics.stroke({ width: 2, color: 0xfeeb77 });

		// Rectangle + line style 2
		graphics.rect(350, 50, 100, 100);
		graphics.fill(0xc34288);
		graphics.stroke({ width: 10, color: 0xffbd01 });

		// Rectangle 2
		graphics.rect(530, 50, 140, 100);
		graphics.fill(0xaa4f08);
		graphics.stroke({ width: 2, color: 0xffffff });

		// Circle
		graphics.circle(100, 250, 50);
		graphics.fill(0xde3249, 1);

		// Circle + line style 1
		graphics.circle(250, 250, 50);
		graphics.fill(0x650a5a, 1);
		graphics.stroke({ width: 2, color: 0xfeeb77 });

		// Circle + line style 2
		graphics.circle(400, 250, 50);
		graphics.fill(0xc34288, 1);
		graphics.stroke({ width: 10, color: 0xffbd01 });

		// Ellipse + line style 2
		graphics.ellipse(600, 250, 80, 50);
		graphics.fill(0xaa4f08, 1);
		graphics.stroke({ width: 2, color: 0xffffff });

		// Draw a shape
		graphics.moveTo(50, 350);
		graphics.lineTo(250, 350);
		graphics.lineTo(100, 400);
		graphics.lineTo(50, 350);
		graphics.fill(0xff3300);
		graphics.stroke({ width: 4, color: 0xffd900 });

		// Draw a rounded rectangle
		graphics.roundRect(50, 440, 100, 100, 16);
		graphics.fill({
			color: 0x650a5a,
			alpha: 0.25,
		});
		graphics.stroke({ width: 2, color: 0xff00ff });

		// Draw star
		graphics.star(360, 370, 5, 50);
		graphics.fill(0x35cc5a);
		graphics.stroke({ width: 2, color: 0xffffff });

		// Draw star 2
		graphics.star(280, 510, 7, 50);
		graphics.fill(0xffcc5a);
		graphics.stroke({ width: 2, color: 0xfffffd });

		// Draw star 3
		graphics.star(470, 450, 4, 50);
		graphics.fill(0x55335a);
		graphics.stroke({ width: 4, color: 0xffffff });

		// Draw polygon
		const path = [600, 370, 700, 460, 780, 420, 730, 570, 590, 520];

		graphics.poly(path);
		graphics.fill(0x3500fa);

		app.stage.addChild(graphics);
	}

	async svg(canvas) {
		const app = new PIXI.Application();

		canvas.width = canvas.clientWidth; //* window.devicePixelRatio;
		canvas.height = canvas.clientHeight; //* window.devicePixelRatio;

		await app.init({
			//	antialias: true,
			backgroundColor: 'white',
			//resizeTo: window,
			canvas,
			//preference: 'webgpu',
			preferWebGLVersion: 2,
			width: canvas.width,
			height: canvas.height,
		});
		try {
			const encoder = new TextEncoder();
			const encoded = encoder.encode(`<svg height="400" width="450" xmlns="http://www.w3.org/2000/svg">
				<!-- Draw the paths -->
				<path id="lineAB" d="M 100 350 l 150 -300" stroke="red" stroke-width="4"/>
				<path id="lineBC" d="M 250 50 l 150 300" stroke="red" stroke-width="4"/>
				<path id="lineMID" d="M 175 200 l 150 0" stroke="green" stroke-width="4"/>
				<path id="lineAC" d="M 100 350 q 150 -300 300 0" stroke="blue" fill="none" stroke-width="4"/>

				<!-- Mark relevant points -->
				<g stroke="black" stroke-width="3" fill="black">
					<circle id="pointA" cx="100" cy="350" r="4" />
					<circle id="pointB" cx="250" cy="50" r="4" />
					<circle id="pointC" cx="400" cy="350" r="4" />
				</g>
			</svg>
		`);
			const blob = new Blob([encoded], { type: 'image/svg+xml' });
			const url = URL.createObjectURL(blob);
			const img = new Image();
			img.src = url;
			await img.decode();
			const c = document.createElement('canvas') as any;
			c.width = img.width;
			c.height = img.height;
			const cctx = c.getContext('2d');
			cctx.fillStyle = 'white';
			cctx.fillRect(0, 0, c.width, c.height);
			cctx.drawImage(img, 0, 0);

			//const source = PIXI.ImageSource.from(img);

			const source = new PIXI.CanvasSource({ resource: c as never });

			const texture = new PIXI.Texture(source);

			const sprite = new PIXI.Sprite(texture);

			const graphics = new Graphics().svg(`
				<svg height="400" width="450" xmlns="http://www.w3.org/2000/svg">
						<!-- Draw the paths -->
						<path id="lineAB" d="M 100 350 l 150 -300" stroke="red" stroke-width="4"/>
						<path id="lineBC" d="M 250 50 l 150 300" stroke="red" stroke-width="4"/>
						<path id="lineMID" d="M 175 200 l 150 0" stroke="green" stroke-width="4"/>
						<path id="lineAC" d="M 100 350 q 150 -300 300 0" stroke="blue" fill="none" stroke-width="4"/>

						<!-- Mark relevant points -->
						<g stroke="black" stroke-width="3" fill="black">
							<circle id="pointA" cx="100" cy="350" r="4" />
							<circle id="pointB" cx="250" cy="50" r="4" />
							<circle id="pointC" cx="400" cy="350" r="4" />
						</g>
					</svg>
				`);

			const other = new Graphics();

			other.rect(100, 100, 100, 100);
			other.fill(0xde3249);

			// const ctx = canvas.getContext('webgpu');

			// app.ticker.add((delta) => {
			// 	const texture = ctx.getCurrentTexture();
			// 	if (texture) {
			// 		ctx.presentSurface();
			// 	}
			// });

			//app.stage.addChild(graphics);

			app.stage.addChild(sprite);

			//	app.stage.addChild(other);
		} catch (error) {
			console.log(error);
		}
	}

	async viewPort(canvas) {
		/*	const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({
			context,
			// eventFeatures: {
			// 	globalMove: false,
			// 	move: false,
			// 	click: false,
			// 	wheel: false
			// }
		});

		// pixi attaches to the document for pointermove in {N} we want to use the canvas passed for this
		canvas.addEventListener('pointermove', (event) => {
			//@ts-ignore
			app.renderer.events.onPointerMove(event);
		});

		const width = canvas.width;
		const height = canvas.height;
		const worldWidth = width * 5;
		const worldHeight = height * 5;
		const world = new Viewport({
			screenWidth: width,
			screenHeight: height,
			worldWidth,
			worldHeight,
			events: app.renderer.events, // the interaction module is important for wheel to work properly when renderer.view is placed or scaled
		});

		// activate plugins
		world
			.drag({
				//	wheel: false
			})
			.pinch({
				//		noDrag: false
			})
			.decelerate();

		const grid = makeGridFilter({
			width,
			height,
		});

		// const g = new PIXI.Graphics();
		// g.width = worldWidth;
		// g.height = worldHeight;
		// g.filters = [this._grid];

		app.stage.addChild(world);

		world.fit();

		world.moveCenter(worldWidth / 2, worldHeight / 2);

		const graphics = new PIXI.Graphics();

		graphics.beginFill('red');
		graphics.drawRect(worldWidth / 2 - width / 2, worldHeight / 2 - height / 2, width, height);
		graphics.endFill();

		graphics.beginFill('blue');
		graphics.drawRect(worldWidth / 2 - width / 2 / 2, worldHeight / 2 - height / 2 / 2, width, height);
		graphics.endFill();

		world.addChild(graphics);

		*/
	}

	async textureRotate(canvas) {
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });
		// create a texture from an image path
		// let texture;

		// // app.loader.add('flowerTop', this.root + '/images/flowerTop.png');
		// // app.loader.load((loader, resources) => {
		// // 	texture = resources.flowerTop.texture;
		// // 	init();
		// // });

		// function init() {
		// 	// create rotated textures
		// 	const textures = [texture];
		// 	const D8 = PIXI.groupD8;
		// 	for (let rotate = 1; rotate < 16; rotate++) {
		// 		const h = D8.isVertical(rotate) ? texture.frame.width : texture.frame.height;
		// 		const w = D8.isVertical(rotate) ? texture.frame.height : texture.frame.width;

		// 		const { frame } = texture;
		// 		const crop = new PIXI.Rectangle(texture.frame.x, texture.frame.y, w, h);
		// 		const trim = crop;
		// 		let rotatedTexture;
		// 		if (rotate % 2 === 0) {
		// 			rotatedTexture = new PIXI.Texture(texture.baseTexture, frame, crop, trim, rotate);
		// 		} else {
		// 			// HACK to avoid exception
		// 			// PIXI doesnt like diamond-shaped UVs, because they are different in canvas and webgl
		// 			rotatedTexture = new PIXI.Texture(texture.baseTexture, frame, crop, trim, rotate - 1);
		// 			rotatedTexture.rotate++;
		// 		}
		// 		textures.push(rotatedTexture);
		// 	}

		// 	const offsetX = (app.screen.width / 16) | 0;
		// 	const offsetY = (app.screen.height / 8) | 0;
		// 	const gridW = (app.screen.width / 4) | 0;
		// 	const gridH = (app.screen.height / 5) | 0;

		// 	// normal rotations and mirrors
		// 	for (let i = 0; i < 16; i++) {
		// 		// create a new Sprite using rotated texture
		// 		const dude = new PIXI.Sprite(textures[i < 8 ? i * 2 : (i - 8) * 2 + 1]);
		// 		dude.scale.x = 0.5;
		// 		dude.scale.y = 0.5;
		// 		// show it in grid
		// 		dude.x = offsetX + gridW * (i % 4);
		// 		dude.y = offsetY + gridH * ((i / 4) | 0);
		// 		app.stage.addChild(dude);
		// 		const text = new PIXI.Text(`rotate = ${dude.texture.rotate}`, {
		// 			fontFamily: 'Courier New',
		// 			fontSize: '12px',
		// 			fill: 'white',
		// 			align: 'left',
		// 		});
		// 		text.x = dude.x;
		// 		text.y = dude.y - 20;
		// 		app.stage.addChild(text);
		// 	}
		// }

		const texture = await PIXI.Assets.load(this.root + '/images/flowerTop.png');

		// Create rotated textures
		const textures = [texture];
		const D8 = PIXI.groupD8;

		for (let rotate = 1; rotate < 16; rotate++) {
			const h = D8.isVertical(rotate) ? texture.frame.width : texture.frame.height;
			const w = D8.isVertical(rotate) ? texture.frame.height : texture.frame.width;

			const { frame } = texture;
			const crop = new PIXI.Rectangle(texture.frame.x, texture.frame.y, w, h);
			const trim = crop;
			let rotatedTexture;

			if (rotate % 2 === 0) {
				rotatedTexture = new PIXI.Texture({
					source: texture.baseTexture,
					frame,
					orig: crop,
					trim,
					rotate,
				} as any);
			} else {
				rotatedTexture = new PIXI.Texture({
					source: texture.baseTexture,
					frame,
					orig: crop,
					trim,
					rotate,
				} as any);
			}
			textures.push(rotatedTexture);
		}

		const offsetX = (app.screen.width / 16) | 0;
		const offsetY = (app.screen.height / 8) | 0;
		const gridW = (app.screen.width / 4) | 0;
		const gridH = (app.screen.height / 5) | 0;

		// Normal rotations and mirrors
		for (let i = 0; i < 16; i++) {
			// Create a new Sprite using rotated texture
			const dude = new PIXI.Sprite(textures[i < 8 ? i * 2 : (i - 8) * 2 + 1]);

			dude.scale.x = 0.5;
			dude.scale.y = 0.5;
			// Show it in grid
			dude.x = offsetX + gridW * (i % 4);
			dude.y = offsetY + gridH * ((i / 4) | 0);
			app.stage.addChild(dude);
			const text = new PIXI.Text({
				text: `rotate = ${dude.texture.rotate}`,
				style: {
					fontFamily: 'Courier New',
					fontSize: '12px',
					fill: 'white',
					align: 'left',
				},
			} as any);

			text.x = dude.x;
			text.y = dude.y - 20;
			app.stage.addChild(text);
		}
	}

	multiPassShaderGenMesh(canvas) {
		/*	const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });
		//app.view.height = 640;
		// Build geometry.
		const geometry = new PIXI.Geometry()
			.addAttribute(
				'aVertexPosition', // the attribute name
				[
					0,
					0, // x, y
					200,
					0, // x, y
					200,
					200,
					0,
					200,
				], // x, y
				2
			) // the size of the attribute
			.addAttribute(
				'aUvs', // the attribute name
				[
					0,
					0, // u, v
					1,
					0, // u, v
					1,
					1,
					0,
					1,
				], // u, v
				2
			) // the size of the attribute
			.addIndex([0, 1, 2, 0, 2, 3]);

		// Vertex shader. Use same shader for all passes.
		const vertexSrc = `

    precision mediump float;

    attribute vec2 aVertexPosition;
    attribute vec2 aUvs;

    uniform mat3 translationMatrix;
    uniform mat3 projectionMatrix;

    varying vec2 vUvs;

    void main() {

        vUvs = aUvs;
        gl_Position = vec4((projectionMatrix * translationMatrix * vec3(aVertexPosition, 1.0)).xy, 0.0, 1.0);

    }`;

		// Load a perlinnoise texture for one of the shaders.
		const perlinTexture = PIXI.Texture.from(this.root + '/images/perlin.jpg');

		// First pass, generates a grid.
		const fragmentGridSrc = `
precision mediump float;
varying vec2 vUvs;
uniform float zoom;

void main()
{
    //Generate a simple grid.
    //Offset uv so that center is 0,0 and edges are -1,1
    vec2 uv = (vUvs-vec2(0.5))*2.0;
    vec2 gUv = floor(uv*zoom);
    vec4 color1 = vec4(0.8, 0.8, 0.8, 1.0);
    vec4 color2 = vec4(0.4, 0.4, 0.4, 1.0);
    vec4 outColor = mod(gUv.x + gUv.y, 2.) < 0.5 ? color1 : color2;
    gl_FragColor = outColor;

}`;

		const gridUniforms = {
			zoom: 10,
		};
		const gridShader: any = PIXI.Shader.from(vertexSrc, fragmentGridSrc, gridUniforms);
		// Sharing textures and meshes is possible. But for simplicity each pass has it's own output texture and mesh in this example.
		const gridTexture = (PIXI as any).RenderTexture.create(200, 200);
		const gridQuad = new PIXI.Mesh(geometry, gridShader);
		const gridContainer = new PIXI.Container();
		gridContainer.addChild(gridQuad);

		// Second pass. Takes grid as input and makes it ripple.
		const fragmentRippleSrc = `
precision mediump float;
varying vec2 vUvs;
uniform float amount;
uniform float phase;
uniform sampler2D texIn;

void main()
{
    //Generate a simple grid.
    vec2 uv = vUvs;
    //Calculate distance from center
    float distance = length( uv - vec2(0.5));
    vec4 color = texture2D(texIn, uv);
    color.rgb *= sin(distance*25.0+phase) * amount+1.;
    gl_FragColor = color;
}`;
		const rippleUniforms = {
			amount: 0.5,
			phase: 0,
			texIn: gridTexture,
		};
		const rippleShader: any = PIXI.Shader.from(vertexSrc, fragmentRippleSrc, rippleUniforms);
		const rippleTexture = (PIXI as any).RenderTexture.create(200, 200);
		const rippleQuad = new PIXI.Mesh(geometry, rippleShader);
		const rippleContainer = new PIXI.Container();
		rippleContainer.addChild(rippleQuad);

		// Second effect. Generates a filtered noise.
		const fragmentNoiseSrc = `
precision mediump float;
varying vec2 vUvs;
uniform float limit;
uniform sampler2D noise;

void main()
{
    float color = texture2D(noise, vUvs).r;
    color = step(limit, color);
    gl_FragColor = vec4(color);
}`;
		const noiseUniforms = {
			limit: 0.5,
			noise: perlinTexture,
		};
		const noiseShader: any = PIXI.Shader.from(vertexSrc, fragmentNoiseSrc, noiseUniforms);
		const noiseTexture = (PIXI as any).RenderTexture.create(200, 200);
		const noiseQuad = new PIXI.Mesh(geometry, noiseShader);
		const noiseContainer = new PIXI.Container();
		noiseContainer.addChild(noiseQuad);

		// Third effect
		const fragmentWaveSrc = `
precision mediump float;
varying vec2 vUvs;
uniform float amplitude;
uniform float time;

void main()
{
    //Offset uv so that center is 0,0 and edges are -1,1
    vec2 uv = (vUvs-vec2(0.5))*2.0;

    vec3 outColor = vec3(0.);

    //Simple wavefunctions inversed and with small offsets.
    outColor += 5./length(uv.y*200. - 50.0*sin( uv.x*0.25+ time*0.25)*amplitude);
    outColor += 4./length(uv.y*300. - 100.0*sin(uv.x*0.5+time*0.5)*amplitude*1.2);
    outColor += 3./length(uv.y*400. - 150.0*sin(uv.x*0.75+time*0.75)*amplitude*1.4);
    outColor += 2./length(uv.y*500. - 200.0*sin(uv.x+time)*amplitude*1.6);

    gl_FragColor = vec4(outColor,1.0);
}`;
		const waveUniforms = {
			amplitude: 0.75,
			time: 0,
		};
		const waveShader: any = PIXI.Shader.from(vertexSrc, fragmentWaveSrc, waveUniforms);
		const waveTexture = (PIXI as any).RenderTexture.create(200, 200);
		const waveQuad = new PIXI.Mesh(geometry, waveShader);
		const waveContainer = new PIXI.Container();
		waveContainer.addChild(waveQuad);

		// Final combination pass
		const fragmentCombineSrc = `
precision mediump float;
varying vec2 vUvs;

uniform sampler2D texRipple;
uniform sampler2D texNoise;
uniform sampler2D texWave;

void main()
{
    //Read color from all
    vec4 ripple = texture2D(texRipple, vUvs);
    vec4 noise = texture2D(texNoise, vUvs);
    vec4 wave = texture2D(texWave, vUvs);

    gl_FragColor = mix(ripple, wave,noise.r);
}`;
		const combineUniforms = {
			texRipple: rippleTexture,
			texNoise: noiseTexture,
			texWave: waveTexture,
		};
		const combineShader: any = PIXI.Shader.from(vertexSrc, fragmentCombineSrc, combineUniforms);
		const combineQuad = new PIXI.Mesh(geometry, combineShader);

		gridContainer.position.set(10, 10);
		rippleContainer.position.set(220, 10);
		noiseContainer.position.set(10, 220);
		waveContainer.position.set(10, 430);
		combineQuad.position.set(430, 220);

		// Add all phases to stage so all the phases can be seen separately.
		app.stage.addChild(gridContainer);
		app.stage.addChild(rippleContainer);
		app.stage.addChild(noiseContainer);
		app.stage.addChild(waveContainer);
		app.stage.addChild(combineQuad);

		// start the animation..
		let time = 0;
		app.ticker.add((delta) => {
			time += 1 / 60;
			// gridQuad.shader.uniforms.zoom = Math.sin(time)*5+10;
			rippleQuad.shader.uniforms.phase = -time;
			waveQuad.shader.uniforms.time = time;
			noiseQuad.shader.uniforms.limit = Math.sin(time * 0.5) * 0.35 + 0.5;

			// Render the passes to get textures.
			app.renderer.render(gridQuad, gridTexture);
			app.renderer.render(rippleQuad, rippleTexture);
			app.renderer.render(noiseQuad, noiseTexture);
			app.renderer.render(waveQuad, waveTexture);
		});

		*/
	}

	meshSharingGeo(canvas) {
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });
		const geometry = new PIXI.Geometry()
			.addAttribute(
				'aVertexPosition', // the attribute name
				[
					-100,
					-100, // x, y
					100,
					-100, // x, y
					100,
					100,
				], // x, y
				2
			) // the size of the attribute

			.addAttribute(
				'aUvs', // the attribute name
				[
					0,
					0, // u, v
					1,
					0, // u, v
					1,
					1,
				], // u, v
				2
			); // the size of the attribute

		const program = PIXI.Program.from(
			`

    precision mediump float;

    attribute vec2 aVertexPosition;
    attribute vec2 aUvs;

    uniform mat3 translationMatrix;
    uniform mat3 projectionMatrix;

    varying vec2 vUvs;

    void main() {

        vUvs = aUvs;
        gl_Position = vec4((projectionMatrix * translationMatrix * vec3(aVertexPosition, 1.0)).xy, 0.0, 1.0);

    }`,

			`precision mediump float;

    varying vec2 vUvs;

    uniform sampler2D uSamplerTexture;

    void main() {

        gl_FragColor = texture2D(uSamplerTexture, vUvs);
    }

`
		);

		const triangle = new PIXI.Mesh(
			geometry,
			new (PIXI as any).Shader(program, {
				uSamplerTexture: PIXI.Texture.from(this.root + '/images/bg_scene_rotate.jpg'),
			})
		);

		const triangle2 = new PIXI.Mesh(
			geometry,
			new (PIXI as any).Shader(program, {
				uSamplerTexture: PIXI.Texture.from(this.root + '/images/bg_rotate.jpg'),
			})
		);

		const triangle3 = new PIXI.Mesh(
			geometry,
			new (PIXI as any).Shader(program, {
				uSamplerTexture: PIXI.Texture.from(this.root + '/images/bg_displacement.jpg'),
			})
		);

		triangle.position.set(400, 300);
		triangle.scale.set(2);

		triangle2.position.set(200, 100);

		triangle3.position.set(500, 400);
		triangle3.scale.set(3);

		app.stage.addChild(triangle3, triangle2, triangle);

		app.ticker.add((delta) => {
			triangle.rotation += 0.01;
			triangle2.rotation -= 0.01;
			triangle3.rotation -= 0.005;
		});
	}

	meshShader(canvas) {
		/*
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });

		const geometry = new PIXI.Geometry()
			.addAttribute(
				'aVertexPosition', // the attribute name
				[
					-100,
					-100, // x, y
					100,
					-100, // x, y
					100,
					100,
					-100,
					100,
				], // x, y
				2
			) // the size of the attribute
			.addAttribute(
				'aUvs', // the attribute name
				[
					0,
					0, // u, v
					1,
					0, // u, v
					1,
					1,
					0,
					1,
				], // u, v
				2
			) // the size of the attribute
			.addIndex([0, 1, 2, 0, 2, 3]);

		const vertexSrc = `

    precision mediump float;

    attribute vec2 aVertexPosition;
    attribute vec2 aUvs;

    uniform mat3 translationMatrix;
    uniform mat3 projectionMatrix;

    varying vec2 vUvs;

    void main() {

        vUvs = aUvs;
        gl_Position = vec4((projectionMatrix * translationMatrix * vec3(aVertexPosition, 1.0)).xy, 0.0, 1.0);

    }`;

		const fragmentSrc = `

    precision mediump float;

    varying vec2 vUvs;

    uniform sampler2D uSampler2;
    uniform float time;

    void main() {

        gl_FragColor = texture2D(uSampler2, vUvs + sin( (time + (vUvs.x) * 14.) ) * 0.1 );
    }`;

		const uniforms = {
			uSampler2: PIXI.Texture.from(this.root + '/images/bg_scene_rotate.jpg'),
			time: 0,
		};

		const shader: any = PIXI.Shader.from(vertexSrc, fragmentSrc, uniforms);

		const quad = new PIXI.Mesh(geometry, shader);

		quad.position.set(400, 300);
		quad.scale.set(2);

		app.stage.addChild(quad);

		// start the animation..
		// requestAnimationFrame(animate);

		app.ticker.add((delta) => {
			quad.rotation += 0.01;
			quad.shader.uniforms.time += 0.1;
		});
		*/
	}

	async renderTextureAdvance(canvas) {
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;
		const app = new PIXI.Application();
		await app.init({
			preference: 'webgpu',
			canvas,
			width: canvas.width,
			height: canvas.height,
			autoDensity: false,
			resolution: window.devicePixelRatio,
		});

		const stageSize = {
			width: app.screen.width,
			height: app.screen.height,
		};

		// Create two render textures... these dynamic textures will be used to draw the scene into itself
		let renderTexture = PIXI.RenderTexture.create(stageSize);
		let renderTexture2 = PIXI.RenderTexture.create(stageSize);
		const currentTexture = renderTexture;

		// Create a new sprite that uses the render texture we created above
		const outputSprite = new PIXI.Sprite(currentTexture);

		// Align the sprite
		outputSprite.x = 400;
		outputSprite.y = 300;
		outputSprite.anchor.set(0.5);

		// Add to stage
		app.stage.addChild(outputSprite);

		const stuffContainer = new PIXI.Container();

		stuffContainer.x = 400;
		stuffContainer.y = 300;

		app.stage.addChild(stuffContainer);

		// Create an array of image ids..
		// const fruits = ['https://pixijs.com/assets/rt_object_01.png', 'https://pixijs.com/assets/rt_object_02.png', 'https://pixijs.com/assets/rt_object_03.png', 'https://pixijs.com/assets/rt_object_04.png', 'https://pixijs.com/assets/rt_object_05.png', 'https://pixijs.com/assets/rt_object_06.png', 'https://pixijs.com/assets/rt_object_07.png', 'https://pixijs.com/assets/rt_object_08.png'];

		const fruits = [this.root + '/images/rt_object_01.png', this.root + '/images/rt_object_02.png', this.root + '/images/rt_object_03.png', this.root + '/images/rt_object_04.png', this.root + '/images/rt_object_05.png', this.root + '/images/rt_object_06.png', this.root + '/images/rt_object_07.png', this.root + '/images/rt_object_08.png'];

		// Load the textures
		await PIXI.Assets.load(fruits);

		// Create an array of items
		const items = [];

		// Now create some items and randomly position them in the stuff container
		for (let i = 0; i < 20; i++) {
			const item = PIXI.Sprite.from(fruits[i % fruits.length]);
			//item.scale.set(item.scale.x * window.devicePixelRatio, item.scale.y * window.devicePixelRatio);
			item.x = Math.random() * 400 - 200;
			item.y = Math.random() * 400 - 200;
			item.anchor.set(0.5);
			stuffContainer.addChild(item);
			items.push(item);
		}

		// Used for spinning!
		let count = 0;

		const ctx = canvas.getContext('webgpu');

		app.ticker.add(() => {
			for (let i = 0; i < items.length; i++) {
				// rotate each item
				const item = items[i];

				item.rotation += 0.1;
			}

			count += 0.01;

			// Swap the buffers ...
			const temp = renderTexture;

			renderTexture = renderTexture2;
			renderTexture2 = temp;

			// Set the new texture
			outputSprite.texture = renderTexture;

			// Twist this up!
			stuffContainer.rotation -= 0.01;
			outputSprite.scale.set(1 + Math.sin(count) * 0.2);

			// Render the stage to the texture
			// * The 'true' clears the texture before the content is rendered *
			app.renderer.render({
				container: app.stage,
				target: renderTexture2,
				clear: false,
			});

			ctx.presentSurface();
		});
	}

	meshBasic(canvas) {
		/*// const app = new PIXI.Application({canvas});
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });
		let count = 0;

		// build a rope!
		const ropeLength = 918 / 20;

		const points = [];

		for (let i = 0; i < 20; i++) {
			points.push(new PIXI.Point(i * ropeLength, 0));
		}

		const strip = new PIXI.SimpleRope(PIXI.Texture.from(this.root + '/images/snake.png'), points);

		strip.x = -459;

		const snakeContainer = new PIXI.Container();
		snakeContainer.x = 400;
		snakeContainer.y = 300;

		snakeContainer.scale.set(800 / 1100);
		app.stage.addChild(snakeContainer);

		snakeContainer.addChild(strip);

		app.ticker.add(() => {
			count += 0.1;

			// make the snake
			for (let i = 0; i < points.length; i++) {
				points[i].y = Math.sin(i * 0.5 + count) * 30;
				points[i].x = i * ropeLength + Math.cos(i * 0.3 + count) * 20;
			}
		});
		*/
	}

	meshAdvance(canvas) {
		/*const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });
		let count = 0;

		// build a rope!
		const ropeLength = 45;

		const points = [];

		for (let i = 0; i < 25; i++) {
			points.push(new PIXI.Point(i * ropeLength, 0));
		}

		const strip = new PIXI.SimpleRope(PIXI.Texture.from(this.root + '/images/snake.png'), points);

		strip.x = -40;
		strip.y = 300;

		app.stage.addChild(strip);

		const g = new PIXI.Graphics();
		g.x = strip.x;
		g.y = strip.y;
		app.stage.addChild(g);

		// start animating
		app.ticker.add(() => {
			count += 0.1;

			// make the snake
			for (let i = 0; i < points.length; i++) {
				points[i].y = Math.sin(i * 0.5 + count) * 30;
				points[i].x = i * ropeLength + Math.cos(i * 0.3 + count) * 20;
			}
			renderPoints();
		});

		function renderPoints() {
			g.clear();

			g.lineStyle(2, 0xffc2c2);
			g.moveTo(points[0].x, points[0].y);

			for (let i = 1; i < points.length; i++) {
				g.lineTo(points[i].x, points[i].y);
			}

			for (let i = 1; i < points.length; i++) {
				g.beginFill(0xff0022);
				g.drawCircle(points[i].x, points[i].y, 10);
				g.endFill();
			}
		}

		*/
	}

	async dynamicGraphics(canvas) {
		canvas.width = canvas.clientWidth; //* window.devicePixelRatio;
		canvas.height = canvas.clientHeight; // * window.devicePixelRatio;
		// canvas.width = canvas.clientWidth;
		// canvas.height = canvas.clientHeight;
		const app = new PIXI.Application();

		await app.init({
			backgroundColor: 0x1099bb,
			//preferWebGLVersion: 2,
			preference: 'webgpu',
			canvas,
			width: canvas.width,
			height: canvas.height,
		});

		app.stage.eventMode = 'static';
		app.stage.hitArea = app.screen;

		const graphics = new Graphics();

		// Draw a shape
		graphics.moveTo(50, 50).lineTo(250, 50).lineTo(100, 100).lineTo(250, 220).lineTo(50, 220).lineTo(50, 50).fill({ color: 0xff3300 }).stroke({ width: 10, color: 0xffd900 });

		// Draw a second shape
		graphics.moveTo(210, 300).lineTo(450, 320).lineTo(570, 350).quadraticCurveTo(600, 0, 480, 100).lineTo(330, 120).lineTo(410, 200).lineTo(210, 300).fill({ color: 0xff700b }).stroke({ width: 10, color: 0xff0000, alpha: 0.8 });

		// Draw a rectangle
		graphics.rect(50, 250, 100, 100);
		graphics.stroke({ width: 2, color: 0x0000ff });

		// Draw a circle
		graphics.circle(470, 200, 100);
		graphics.fill({ color: 0xffff0b, alpha: 0.5 });

		graphics.moveTo(30, 30);
		graphics.lineTo(600, 300);
		graphics.stroke({ width: 20, color: 0x33ff00 });

		app.stage.addChild(graphics);

		// Let's create a moving shape
		const thing = new Graphics();

		app.stage.addChild(thing);
		thing.x = 800 / 2;
		thing.y = 600 / 2;

		let count = 0;

		// Just click on the stage to draw random lines
		window.app = app;
		app.stage.on('pointerdown', () => {
			console.log('click');
			graphics.moveTo(Math.random() * 800, Math.random() * 600);
			graphics.bezierCurveTo(Math.random() * 800, Math.random() * 600, Math.random() * 800, Math.random() * 600, Math.random() * 800, Math.random() * 600);
			graphics.stroke({ width: Math.random() * 30, color: Math.random() * 0xffffff });
		});

		const ctx = canvas.getContext('webgpu');

		// Animate the moving shape
		app.ticker.add(() => {
			count += 0.1;

			thing.clear();

			thing
				.moveTo(-120 + Math.sin(count) * 20, -100 + Math.cos(count) * 20)
				.lineTo(120 + Math.cos(count) * 20, -100 + Math.sin(count) * 20)
				.lineTo(120 + Math.sin(count) * 20, 100 + Math.cos(count) * 20)
				.lineTo(-120 + Math.cos(count) * 20, 100 + Math.sin(count) * 20)
				.lineTo(-120 + Math.sin(count) * 20, -100 + Math.cos(count) * 20)
				.fill({ color: 0xffff00, alpha: 0.5 })
				.stroke({ width: 10, color: 0xff0000 });

			thing.rotation = count * 0.1;

			const texture = ctx.getCurrentTexture();
			if (texture) {
				ctx.presentSurface();
			}
		});
	}

	async bitmapFont(canvas) {
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context, backgroundColor: 0x1099bb });
		// app.loader.add('desyrel', this.root + '/bitmap-font/desyrel.xml').load(onAssetsLoaded);
		// function onAssetsLoaded() {
		// 	const bitmapFontText = new (PIXI as any).BitmapText('bitmap fonts are supported!\nWoo yay!', { font: '55px Desyrel', align: 'left' });
		// 	bitmapFontText.x = 50;
		// 	bitmapFontText.y = 200;
		// 	app.stage.addChild(bitmapFontText);
		// }

		await PIXI.Assets.load(this.root + '/bitmap-font/desyrel.xml');

		const bitmapFontText = new PIXI.BitmapText('bitmap fonts are supported!\nWoo yay!', {
			fontFamily: 'Desyrel',
			fontSize: 16,
			align: 'left',
		});

		// const nameText = new (PIXI.BitmapText as typeof BitmapText)(options.nameInfo?.text ?? '', {
		// 	fontName: 'Desyrel',
		// 	tint: info?.nameInfo?.textColor ?? 'black',
		// 	fontSize: 16,
		// 	align: (info?.nameInfo?.textAlignment as never) ?? 'left',
		//   });

		bitmapFontText.x = 50;
		bitmapFontText.y = 200;

		app.stage.addChild(bitmapFontText);
	}

	async explosion(canvas) {
		const app = new PIXI.Application();
		canvas.width = canvas.clientWidth; //* window.devicePixelRatio;
		canvas.height = canvas.clientHeight; //* window.devicePixelRatio;
		await app.init({
			backgroundColor: 0x1099bb,
			autoStart: false,
			canvas,
			width: canvas.width,
			height: canvas.height,
			preference: 'webgpu',
		});

		// app.stop();

		// const onAssetsLoaded = () => {
		// 	// create an array to store the textures
		// 	const explosionTextures = [];
		// 	let i;

		// 	for (i = 0; i < 26; i++) {
		// 		const texture = PIXI.Texture.from(`Explosion_Sequence_A ${i + 1}.png`);
		// 		explosionTextures.push(texture);
		// 	}

		// 	for (i = 0; i < 50; i++) {
		// 		// create an explosion AnimatedSprite
		// 		const explosion = new PIXI.AnimatedSprite(explosionTextures);

		// 		explosion.x = Math.random() * app.screen.width;
		// 		explosion.y = Math.random() * app.screen.height;
		// 		explosion.anchor.set(0.5);
		// 		explosion.rotation = Math.random() * Math.PI;
		// 		explosion.scale.set(0.75 + Math.random() * 0.5);
		// 		explosion.gotoAndPlay(Math.random() * 27);
		// 		app.stage.addChild(explosion);
		// 	}
		// 	// start animating
		// 	app.start();
		// };

		// app.loader.add('spritesheet', this.root + '/spritesheet/mc.json').load(onAssetsLoaded);

		//const texture = await PIXI.Assets.load('https://pixijs.com/assets/spritesheet/mc.json');
		try {
			await PIXI.Assets.load(this.root + '/spritesheet/mc.json');
		} catch (error) {
			console.log(error);
		}
		console.log('??');
		// Create an array to store the textures
		const explosionTextures = [];
		let i;

		for (i = 0; i < 26; i++) {
			const texture = PIXI.Texture.from(`Explosion_Sequence_A ${i + 1}.png`);

			explosionTextures.push(texture);
		}

		// Create and randomly place the animated explosion sprites on the stage
		for (i = 0; i < 50; i++) {
			// Create an explosion AnimatedSprite
			const explosion = new PIXI.AnimatedSprite(explosionTextures);

			explosion.x = Math.random() * app.screen.width;
			explosion.y = Math.random() * app.screen.height;
			explosion.anchor.set(0.5);
			explosion.rotation = Math.random() * Math.PI;
			explosion.scale.set(0.75 + Math.random() * 0.5);
			explosion.gotoAndPlay((Math.random() * 26) | 0);
			app.stage.addChild(explosion);
		}

		const ctx = canvas.getContext('webgpu');

		app.ticker.add(() => {
			const texture = ctx.getCurrentTexture();
			if (texture) {
				ctx.presentSurface();
			}
		});

		app.start();
	}

	async starWarp(canvas) {
		const app = new PIXI.Application();
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;

		await app.init({
			canvas,
			preference: 'webgpu',
			width: canvas.width,
			height: canvas.height,
		});

		const ctx = canvas.getContext('webgpu');

		// Get the texture for rope.
		const starTexture = await PIXI.Assets.load(this.root + '/images/star.png');

		const starAmount = 1000;
		let cameraZ = 0;
		const fov = 20;
		const baseSpeed = 0.025;
		let speed = 0;
		let warpSpeed = 0;
		const starStretch = 5;
		const starBaseSize = 0.05;

		// Create the stars
		const stars = [];

		for (let i = 0; i < starAmount; i++) {
			const star = {
				sprite: new PIXI.Sprite(starTexture),
				z: 0,
				x: 0,
				y: 0,
			};

			star.sprite.anchor.x = 0.5;
			star.sprite.anchor.y = 0.7;
			randomizeStar(star, true);
			app.stage.addChild(star.sprite);
			stars.push(star);
		}

		function randomizeStar(star, initial?) {
			star.z = initial ? Math.random() * 2000 : cameraZ + Math.random() * 1000 + 2000;

			// Calculate star positions with radial random coordinate so no star hits the camera.
			const deg = Math.random() * Math.PI * 2;
			const distance = Math.random() * 50 + 1;

			star.x = Math.cos(deg) * distance;
			star.y = Math.sin(deg) * distance;
		}

		// Change flight speed every 5 seconds
		setInterval(() => {
			warpSpeed = warpSpeed > 0 ? 0 : 1;
		}, 5000);

		// Listen for animate update
		app.ticker.add((time) => {
			// Simple easing. This should be changed to proper easing function when used for real.
			speed += (warpSpeed - speed) / 20;
			cameraZ += time.deltaTime * 10 * (speed + baseSpeed);
			for (let i = 0; i < starAmount; i++) {
				const star = stars[i];

				if (star.z < cameraZ) randomizeStar(star);

				// Map star 3d position to 2d with really simple projection
				const z = star.z - cameraZ;

				star.sprite.x = star.x * (fov / z) * app.renderer.screen.width + app.renderer.screen.width / 2;
				star.sprite.y = star.y * (fov / z) * app.renderer.screen.width + app.renderer.screen.height / 2;

				// Calculate star scale & rotation.
				const dxCenter = star.sprite.x - app.renderer.screen.width / 2;
				const dyCenter = star.sprite.y - app.renderer.screen.height / 2;
				const distanceCenter = Math.sqrt(dxCenter * dxCenter + dyCenter * dyCenter);
				const distanceScale = Math.max(0, (2000 - z) / 2000);

				star.sprite.scale.x = distanceScale * starBaseSize;
				// Star is looking towards center so that y axis is towards center.
				// Scale the star depending on how fast we are moving, what the stretchfactor is
				// and depending on how far away it is from the center.
				star.sprite.scale.y = distanceScale * starBaseSize + (distanceScale * speed * starStretch * distanceCenter) / app.renderer.screen.width;
				star.sprite.rotation = Math.atan2(dyCenter, dxCenter) + Math.PI / 2;
			}

			const texture = ctx.getCurrentTexture();

			if (texture != null) {
				ctx.presentSurface();
			}
		});
	}

	async container(canvas) {
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context, backgroundColor: 0x1099bb });

		// const loader = new PIXI.Loader();
		// loader.add('bunny', this.root + '/images/bunny.png');

		const container = new PIXI.Container();

		app.stage.addChild(container);

		// Load the bunny texture
		// const texture = await  PIXI.Assets.load('https://pixijs.com/assets/bunny.png');

		const texture = await PIXI.Assets.load(this.root + '/images/bunny.png');

		// Create a 5x5 grid of bunnies in the container
		for (let i = 0; i < 25; i++) {
			const bunny = new PIXI.Sprite(texture);

			bunny.x = (i % 5) * 40;
			bunny.y = Math.floor(i / 5) * 40;
			container.addChild(bunny);
		}

		// Move the container to the center
		container.x = app.screen.width / 2;
		container.y = app.screen.height / 2;

		// Center the bunny sprites in local container coordinates
		container.pivot.x = container.width / 2;
		container.pivot.y = container.height / 2;

		// Listen for animate update
		app.ticker.add((time: any) => {
			// Continuously rotate the container!
			// * use delta to create frame-independent transform *
			container.rotation -= 0.01 * time.deltaTime;
		});
	}

	transparent(canvas) {
		// const app = new PIXI.Application({ canvas, transparent: true });
		// // create a new Sprite from an image path.
		// const bunny = PIXI.Sprite.from(this.root + '/images/bunny.png');
		// // center the sprite's anchor point
		// bunny.anchor.set(0.5);
		// // move the sprite to the center of the screen
		// bunny.x = app.screen.width / 2;
		// bunny.y = app.screen.height / 2;
		// app.stage.addChild(bunny);
		// app.ticker.add(() => {
		// 	// just for fun, let's rotate mr rabbit a little
		// 	bunny.rotation += 0.1;
		// });
	}

	eggHead(canvas) {
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });
		// holder to store the aliens
		const aliens = [];

		const totalDudes = 20;

		for (let i = 0; i < totalDudes; i++) {
			// create a new Sprite that uses the image name that we just generated as its source
			const dude: any = PIXI.Sprite.from(this.root + '/images/eggHead.png');

			// set the anchor point so the texture is centerd on the sprite
			dude.anchor.set(0.5);

			// set a random scale for the dude - no point them all being the same size!
			dude.scale.set(0.8 + Math.random() * 0.3);

			// finally lets set the dude to be at a random position..
			dude.x = Math.random() * app.screen.width;
			dude.y = Math.random() * app.screen.height;

			dude.tint = Math.random() * 0xffffff;

			// create some extra properties that will control movement :
			// create a random direction in radians. This is a number between 0 and PI*2 which is the equivalent of 0 - 360 degrees
			dude.direction = Math.random() * Math.PI * 2;

			// this number will be used to modify the direction of the dude over time
			dude.turningSpeed = Math.random() - 0.8;

			// create a random speed for the dude between 2 - 4
			dude.speed = 2 + Math.random() * 2;

			// finally we push the dude into the aliens array so it it can be easily accessed later
			aliens.push(dude);

			app.stage.addChild(dude);
		}

		// create a bounding box for the little dudes
		const dudeBoundsPadding = 100;
		const dudeBounds = new PIXI.Rectangle(-dudeBoundsPadding, -dudeBoundsPadding, app.screen.width + dudeBoundsPadding * 2, app.screen.height + dudeBoundsPadding * 2);

		app.ticker.add(() => {
			// iterate through the dudes and update their position
			for (let i = 0; i < aliens.length; i++) {
				const dude = aliens[i];
				dude.direction += dude.turningSpeed * 0.01;
				dude.x += Math.sin(dude.direction) * dude.speed;
				dude.y += Math.cos(dude.direction) * dude.speed;
				dude.rotation = -dude.direction - Math.PI / 2;

				// wrap the dudes by testing their bounds...
				if (dude.x < dudeBounds.x) {
					dude.x += dudeBounds.width;
				} else if (dude.x > dudeBounds.x + dudeBounds.width) {
					dude.x -= dudeBounds.width;
				}

				if (dude.y < dudeBounds.y) {
					dude.y += dudeBounds.height;
				} else if (dude.y > dudeBounds.y + dudeBounds.height) {
					dude.y -= dudeBounds.height;
				}
			}
		});
	}

	cacheAsBitmap(canvas) {
		// const context = canvas.getContext('webgl2');
		// const app = new PIXI.Application({ context });
		// app.stop();
		// // load resources
		// app.loader.add('spritesheet', this.root + '/spritesheet/monsters.json').load(onAssetsLoaded);
		// // holder to store aliens
		// const aliens = [];
		// const alienFrames = ['eggHead.png', 'flowerTop.png', 'helmlok.png', 'skully.png'];
		// let count = 0;
		// // create an empty container
		// const alienContainer = new PIXI.Container();
		// alienContainer.x = 400;
		// alienContainer.y = 300;
		// // make the stage interactive
		// app.stage.interactive = true;
		// app.stage.addChild(alienContainer);
		// function onAssetsLoaded() {
		// 	// add a bunch of aliens with textures from image paths
		// 	for (let i = 0; i < 100; i++) {
		// 		const frameName = alienFrames[i % 4];
		// 		// create an alien using the frame name..
		// 		const alien = PIXI.Sprite.from(frameName);
		// 		alien.tint = Math.random() * 0xffffff;
		// 		/*
		// 		 * fun fact for the day :)
		// 		 * another way of doing the above would be
		// 		 * var texture = PIXI.Texture.from(frameName);
		// 		 * var alien = new PIXI.Sprite(texture);
		// 		 */
		// 		alien.x = Math.random() * 800 - 400;
		// 		alien.y = Math.random() * 600 - 300;
		// 		alien.anchor.x = 0.5;
		// 		alien.anchor.y = 0.5;
		// 		aliens.push(alien);
		// 		alienContainer.addChild(alien);
		// 	}
		// 	app.start();
		// }
		// // Combines both mouse click + touch tap
		// app.stage.on('pointertap', onClick);
		// function onClick() {
		// 	alienContainer.cacheAsBitmap = !alienContainer.cacheAsBitmap;
		// 	// feel free to play with what's below
		// 	// var sprite = new PIXI.Sprite(alienContainer.generateTexture());
		// 	// app.stage.addChild(sprite);
		// 	// sprite.x = Math.random() * 800;
		// 	// sprite.y = Math.random() * 600;
		// }
		// app.ticker.add(() => {
		// 	// let's rotate the aliens a little bit
		// 	for (let i = 0; i < 100; i++) {
		// 		const alien = aliens[i];
		// 		alien.rotation += 0.1;
		// 	}
		// 	count += 0.01;
		// 	alienContainer.scale.x = Math.sin(count);
		// 	alienContainer.scale.y = Math.sin(count);
		// 	alienContainer.rotation += 0.01;
		// });
	}

	particleContainer(canvas) {
		/*
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application({ context });
		const sprites = new (PIXI.ParticleContainer as any)(10000, {
			scale: true,
			position: true,
			rotation: true,
			uvs: true,
			alpha: true,
		});
		app.stage.addChild(sprites);

		// create an array to store all the sprites
		const maggots = [];

		const totalSprites = app.renderer instanceof PIXI.Renderer ? 10000 : 10;

		for (let i = 0; i < totalSprites; i++) {
			// create a new Sprite
			const dude = PIXI.Sprite.from(this.root + '/images/maggot_tiny.png') as any;

			dude.tint = Math.random() * 0xe8d4cd;

			// set the anchor point so the texture is centerd on the sprite
			dude.anchor.set(0.5);

			// different maggots, different sizes
			dude.scale.set(0.8 + Math.random() * 0.3);

			// scatter them all
			dude.x = Math.random() * app.screen.width;
			dude.y = Math.random() * app.screen.height;

			dude.tint = Math.random() * 0x808080;

			// create a random direction in radians
			dude.direction = Math.random() * Math.PI * 2;

			// this number will be used to modify the direction of the sprite over time
			dude.turningSpeed = Math.random() - 0.8;

			// create a random speed between 0 - 2, and these maggots are slooww
			dude.speed = (2 + Math.random() * 2) * 0.2;

			dude.offset = Math.random() * 100;

			// finally we push the dude into the maggots array so it it can be easily accessed later
			maggots.push(dude);

			sprites.addChild(dude);
		}

		// create a bounding box box for the little maggots
		const dudeBoundsPadding = 100;
		const dudeBounds = new PIXI.Rectangle(-dudeBoundsPadding, -dudeBoundsPadding, app.screen.width + dudeBoundsPadding * 2, app.screen.height + dudeBoundsPadding * 2);

		let tick = 0;

		app.ticker.add(() => {
			// iterate through the sprites and update their position
			for (let i = 0; i < maggots.length; i++) {
				const dude = maggots[i];
				dude.scale.y = 0.95 + Math.sin(tick + dude.offset) * 0.05;
				dude.direction += dude.turningSpeed * 0.01;
				dude.x += Math.sin(dude.direction) * (dude.speed * dude.scale.y);
				dude.y += Math.cos(dude.direction) * (dude.speed * dude.scale.y);
				dude.rotation = -dude.direction + Math.PI;

				// wrap the maggots
				if (dude.x < dudeBounds.x) {
					dude.x += dudeBounds.width;
				} else if (dude.x > dudeBounds.x + dudeBounds.width) {
					dude.x -= dudeBounds.width;
				}

				if (dude.y < dudeBounds.y) {
					dude.y += dudeBounds.height;
				} else if (dude.y > dudeBounds.y + dudeBounds.height) {
					dude.y -= dudeBounds.height;
				}
			}

			// increment the ticker
			tick += 0.1;
		});

		*/
	}

	async blendModes(canvas) {
		/*
		const context = canvas.getContext('webgl2');
		const app = new PIXI.Application();

		app.init({context});

		// create a new background sprite
		const background = PIXI.Sprite.from(this.root + '/images/bg_rotate.jpg');
		background.width = app.screen.width;
		background.height = app.screen.height;
		app.stage.addChild(background);

		// create an array to store a reference to the dudes
		const dudeArray = [];

		const totaldudes = 20;

		for (let i = 0; i < totaldudes; i++) {
			// create a new Sprite that uses the image name that we just generated as its source
			const dude: any = PIXI.Sprite.from(this.root + '/images/flowerTop.png');

			dude.anchor.set(0.5);

			// set a random scale for the dude
			dude.scale.set(0.8 + Math.random() * 0.3);

			// finally let's set the dude to be at a random position...
			dude.x = Math.floor(Math.random() * app.screen.width);
			dude.y = Math.floor(Math.random() * app.screen.height);

			// The important bit of this example, this is how you change the default blend mode of the sprite
			PIXI.BlendModeFilter
			dude.blendMode = PIXI.BLEND_MODES.ADD;

			// create some extra properties that will control movement
			dude.direction = Math.random() * Math.PI * 2;

			// this number will be used to modify the direction of the dude over time
			dude.turningSpeed = Math.random() - 0.8;

			// create a random speed for the dude between 0 - 2
			dude.speed = 2 + Math.random() * 2;

			// finally we push the dude into the dudeArray so it it can be easily accessed later
			dudeArray.push(dude);

			app.stage.addChild(dude);
		}

		// create a bounding box for the little dudes
		const dudeBoundsPadding = 100;

		const dudeBounds = new PIXI.Rectangle(-dudeBoundsPadding, -dudeBoundsPadding, app.screen.width + dudeBoundsPadding * 2, app.screen.height + dudeBoundsPadding * 2);

		app.ticker.add(() => {
			// iterate through the dudes and update the positions
			for (let i = 0; i < dudeArray.length; i++) {
				const dude = dudeArray[i];
				dude.direction += dude.turningSpeed * 0.01;
				dude.x += Math.sin(dude.direction) * dude.speed;
				dude.y += Math.cos(dude.direction) * dude.speed;
				dude.rotation = -dude.direction - Math.PI / 2;

				// wrap the dudes by testing their bounds...
				if (dude.x < dudeBounds.x) {
					dude.x += dudeBounds.width;
				} else if (dude.x > dudeBounds.x + dudeBounds.width) {
					dude.x -= dudeBounds.width;
				}

				if (dude.y < dudeBounds.y) {
					dude.y += dudeBounds.height;
				} else if (dude.y > dudeBounds.y + dudeBounds.height) {
					dude.y -= dudeBounds.height;
				}
			}
		});

		*/
	}

	async simplePlane(canvas) {
		canvas.width = canvas.parent.clientWidth * Screen.mainScreen.scale;
		canvas.height = canvas.parent.clientHeight * Screen.mainScreen.scale;
		const app = new PIXI.Application();

		await initDevtools({ app });

		// (<any>global).window.__PIXI_DEVTOOLS__ = {
		// 	app,
		// 	PIXI
		//   };

		//   (<any>global).__PIXI_DEVTOOLS__ = (<any>global).window.__PIXI_DEVTOOLS__;

		await app.init({
			background: '#1099bb',
			canvas,
			width: canvas.width,
			height: canvas.height,
			preference: 'webgpu',
		});

		//app.loader.add('bg_grass', this.root + '/images/bg_grass.jpg').load(build);
		const texture = await PIXI.Assets.load('https://pixijs.com/assets/bg_grass.jpg');

		const ctx = canvas.getContext('webgpu');

		app.ticker.add((delta) => {
			if (ctx) {
				ctx.presentSurface();
			}
		});

		const plane = new PIXI.MeshPlane({ texture, verticesX: 10, verticesY: 10 });
		plane.x = 100;
		plane.y = 100;

		app.stage.addChild(plane);

		// Get the buffer for vertice positions.
		const { buffer } = plane.geometry.getAttribute('aPosition');

		// Listen for animate update
		let timer = 0;

		app.ticker.add(() => {
			// Randomize the vertice positions a bit to create movement.
			for (let i = 0; i < buffer.data.length; i++) {
				buffer.data[i] += Math.sin(timer / 10 + i) * 0.5;
			}
			buffer.update();
			timer++;
		});
	}

	async animatedJet(canvas) {
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;
		let app = new PIXI.Application();
		await app.init({
			canvas,
			background: '#1099bb',
			width: canvas.width,
			height: canvas.height,
		});

		// canvas.parent.addChild(app.renderer.canvas);

		await PIXI.Assets.load(this.root + '/spritesheet/fighter.json');

		const frames = [];

		for (let i = 0; i < 30; i++) {
			const val = i < 10 ? `0${i}` : i;

			// magically works since the spritesheet was loaded with the pixi loader
			frames.push(PIXI.Texture.from(`rollSequence00${val}.png`));
		}

		// create an AnimatedSprite (brings back memories from the days of Flash, right ?)
		const anim = new PIXI.AnimatedSprite(frames);
		anim.x = app.screen.width / 2;
		anim.y = app.screen.height / 2;
		anim.anchor.set(0.5);
		anim.animationSpeed = 0.5;
		anim.play();

		app.stage.addChild(anim);

		/*


		const ctx = app.canvas.getContext('webgpu') as any;

		ctx.configure({
			device,
			format: navigator.gpu.getPreferredCanvasFormat(),
		});

		*/
		// Animate the rotation
		app.ticker.add(() => {
			anim.rotation += 0.01;
			// const texture = ctx.getCurrentTexture();
			// if (texture) {
			// 	ctx.presentSurface();
			// }
		});
	}

	async text(canvas) {
		const app = new PIXI.Application();
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;
		await app.init({
			background: '#1099bb',
			canvas,
			width: canvas.width,
			height: canvas.height,
		});
		const basicText = new Text({ text: 'Basic text in pixi' });

		basicText.x = 50;
		basicText.y = 100;

		basicText.style.fontSize = 100;

		app.stage.addChild(basicText);

		// Create gradient fill
		const fill = new FillGradient(0, 0, 0, 36 * 1.7 * 7);

		const colors = [0xffffff, 0x00ff99].map((color) => Color.shared.setValue(color).toNumber());

		colors.forEach((number, index) => {
			const ratio = index / colors.length;

			fill.addColorStop(ratio, number);
		});

		const style = new TextStyle({
			fontFamily: 'Arial',
			fontSize: 36,
			fontStyle: 'italic',
			fontWeight: 'bold',
			fill: { fill },
			stroke: { color: '#4a1850', width: 5, join: 'round' },
			dropShadow: {
				color: '#000000',
				blur: 4,
				angle: Math.PI / 6,
				distance: 6,
			},
			wordWrap: true,
			wordWrapWidth: 440,
		});

		const richText = new Text({
			text: 'Rich text with a lot of options and across multiple lines',
			style,
		});

		richText.x = 50;
		richText.y = 220;

		app.stage.addChild(richText);

		const skewStyle = new TextStyle({
			fontFamily: 'Arial',
			dropShadow: {
				alpha: 0.8,
				angle: 2.1,
				blur: 4,
				color: '0x111111',
				distance: 10,
			},
			fill: '#ffffff',
			stroke: { color: '#004620', width: 12, join: 'round' },
			fontSize: 60,
			fontWeight: 'lighter',
		});

		const skewText = new Text({
			text: 'SKEW IS COOL',
			style: skewStyle,
		});

		skewText.skew.set(0.65, -0.3);
		skewText.anchor.set(0.5, 0.5);
		skewText.x = 300;
		skewText.y = 480;

		app.stage.addChild(skewText);
	}

	drawPatternWithCanvas(canvas) {
		const scale = 1; //Screen.mainScreen.scale;
		const patternCanvas = document.createElement('canvas');
		// Give the pattern a width and height of 50
		patternCanvas.width = 50;
		patternCanvas.height = 50;

		const patternContext = patternCanvas.getContext('2d') as any;

		// Give the pattern a background color and draw an arc
		patternContext.fillStyle = '#fec';
		patternContext.fillRect(0, 0, patternCanvas.width, patternCanvas.height);
		patternContext.arc(0, 0, 50, 0, 0.5 * Math.PI);
		patternContext.stroke();

		// Create our primary canvas and fill it with the pattern
		const ctx = canvas.getContext('2d');
		const pattern = ctx.createPattern(patternCanvas, 'repeat');
		ctx.fillStyle = pattern;
		ctx.fillRect(0, 0, canvas.width, canvas.height);
	}

	/* Graphics */
	async simple(canvas) {
		const app = new PIXI.Application() as Application;

		await app.init({ canvas });

		const graphics = new Graphics();

		// Rectangle
		graphics.rect(50, 50, 100, 100);
		graphics.fill(0xde3249);

		// Rectangle + line style 1
		graphics.rect(200, 50, 100, 100);
		graphics.fill(0x650a5a);
		graphics.stroke({ width: 2, color: 0xfeeb77 });

		// Rectangle + line style 2
		graphics.rect(350, 50, 100, 100);
		graphics.fill(0xc34288);
		graphics.stroke({ width: 10, color: 0xffbd01 });

		// Rectangle 2
		graphics.rect(530, 50, 140, 100);
		graphics.fill(0xaa4f08);
		graphics.stroke({ width: 2, color: 0xffffff });

		// Circle
		graphics.circle(100, 250, 50);
		graphics.fill(0xde3249, 1);

		// Circle + line style 1
		graphics.circle(250, 250, 50);
		graphics.fill(0x650a5a, 1);
		graphics.stroke({ width: 2, color: 0xfeeb77 });

		// Circle + line style 2
		graphics.circle(400, 250, 50);
		graphics.fill(0xc34288, 1);
		graphics.stroke({ width: 10, color: 0xffbd01 });

		// Ellipse + line style 2
		graphics.ellipse(600, 250, 80, 50);
		graphics.fill(0xaa4f08, 1);
		graphics.stroke({ width: 2, color: 0xffffff });

		// Draw a shape
		graphics.moveTo(50, 350);
		graphics.lineTo(250, 350);
		graphics.lineTo(100, 400);
		graphics.lineTo(50, 350);
		graphics.fill(0xff3300);
		graphics.stroke({ width: 4, color: 0xffd900 });

		// Draw a rounded rectangle
		graphics.roundRect(50, 440, 100, 100, 16);
		graphics.fill({
			color: 0x650a5a,
			alpha: 0.25,
		});
		graphics.stroke({ width: 2, color: 0xff00ff });

		// Draw star
		graphics.star(360, 370, 5, 50);
		graphics.fill(0x35cc5a);
		graphics.stroke({ width: 2, color: 0xffffff });

		// Draw star 2
		graphics.star(280, 510, 7, 50);
		graphics.fill(0xffcc5a);
		graphics.stroke({ width: 2, color: 0xfffffd });

		// Draw star 3
		graphics.star(470, 450, 4, 50);
		graphics.fill(0x55335a);
		graphics.stroke({ width: 4, color: 0xffffff });

		// Draw polygon
		const path = [600, 370, 700, 460, 780, 420, 730, 570, 590, 520];

		graphics.poly(path);
		graphics.fill(0x3500fa);

		app.stage.addChild(graphics);
	}

	advance(canvas) {
		/*
		const app = new PIXI.Application({
			context: canvas.getContext('webgl2'),
		});
		const sprite = PIXI.Sprite.from(this.root + '/images/bg_rotate.jpg');

		// // BEZIER CURVE ////
		// information: https://en.wikipedia.org/wiki/Bézier_curve

		const realPath = new PIXI.Graphics();

		realPath.lineStyle(2, 0xffffff, 1);
		realPath.moveTo(0, 0);
		realPath.lineTo(100, 200);
		realPath.lineTo(200, 200);
		realPath.lineTo(240, 100);

		realPath.position.x = 50;
		realPath.position.y = 50;

		app.stage.addChild(realPath);

		const bezier = new PIXI.Graphics();

		bezier.lineStyle(5, 0xaa0000, 1);
		bezier.bezierCurveTo(100, 200, 200, 200, 240, 100);

		bezier.position.x = 50;
		bezier.position.y = 50;

		app.stage.addChild(bezier);

		// // BEZIER CURVE 2 ////
		const realPath2 = new PIXI.Graphics();

		realPath2.lineStyle(2, 0xffffff, 1);
		realPath2.moveTo(0, 0);
		realPath2.lineTo(0, -100);
		realPath2.lineTo(150, 150);
		realPath2.lineTo(240, 100);

		realPath2.position.x = 320;
		realPath2.position.y = 150;

		app.stage.addChild(realPath2);

		const bezier2 = new PIXI.Graphics();
		bezier2.lineTextureStyle({
			width: 10,
			texture: sprite.texture,
		});
		bezier2.bezierCurveTo(0, -100, 150, 150, 240, 100);

		bezier2.position.x = 320;
		bezier2.position.y = 150;

		app.stage.addChild(bezier2);

		// // ARC ////
		const arc = new PIXI.Graphics();

		arc.lineStyle(5, 0xaa00bb, 1);
		arc.arc(600, 100, 50, Math.PI, 2 * Math.PI);

		app.stage.addChild(arc);

		// // ARC 2 ////
		const arc2 = new PIXI.Graphics();

		arc2.lineStyle(6, 0x3333dd, 1);
		arc2.arc(650, 270, 60, 2 * Math.PI, (3 * Math.PI) / 2);

		app.stage.addChild(arc2);

		// // ARC 3 ////
		const arc3 = new PIXI.Graphics();

		arc3.lineTextureStyle({
			width: 10,
			texture: sprite.texture,
		});
		arc3.arc(650, 420, 60, 2 * Math.PI, (2.5 * Math.PI) / 2);

		app.stage.addChild(arc3);

		// / Hole ////
		const rectAndHole = new PIXI.Graphics();

		rectAndHole.beginFill(0x00ff00);
		rectAndHole.drawRect(350, 350, 150, 150);
		rectAndHole.beginHole();
		rectAndHole.drawCircle(375, 375, 25);
		rectAndHole.drawCircle(425, 425, 25);
		rectAndHole.drawCircle(475, 475, 25);
		rectAndHole.endHole();
		rectAndHole.endFill();

		app.stage.addChild(rectAndHole);

		// // Line Texture Style ////
		const beatifulRect = new PIXI.Graphics();

		beatifulRect.lineTextureStyle({
			width: 10,
			texture: sprite.texture,
		});
		beatifulRect.beginFill(0xff0000);
		beatifulRect.drawRect(80, 350, 150, 150);
		beatifulRect.endFill();

		app.stage.addChild(beatifulRect);

		*/
	}
}
