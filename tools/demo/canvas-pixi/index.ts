import { DemoSharedBase } from '../utils';

let TNSPIXIApplication;
import { Screen } from '@nativescript/core';

let PIXI;

export class DemoSharedCanvasPixi extends DemoSharedBase {
	root = '~/assets/pixi';

	loaded(args) {
		console.log('loaded', args.object);
	}

	unloaded(args) {
		console.log('unloaded');
	}

	canvasLoaded(args) {
		const canvas = args.object;
		TNSPIXIApplication = require('@nativescript/canvas-pixi').TNSPIXIApplication;
		PIXI = require('pixi.js');
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
		//this.simple(canvas);
		//this.simplePlane(canvas);
		//this.advance(canvas);
		//this.container(canvas);
		//this.explosion(canvas);
		//this.bitmapFont(canvas);
		//this.dynamicGraphics(canvas);
		//this.meshBasic(canvas);
		//this.meshAdvance(canvas);
		this.renderTextureAdvance(canvas);
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
	}

	textureRotate(canvas) {
		const app = new TNSPIXIApplication({ canvas });
		// create a texture from an image path
		let texture;

		app.loader.add('flowerTop', this.root + '/images/flowerTop.png');
		app.loader.load((loader, resources) => {
			texture = resources.flowerTop.texture;
			init();
		});

		function init() {
			// create rotated textures
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
					rotatedTexture = new PIXI.Texture(texture.baseTexture, frame, crop, trim, rotate);
				} else {
					// HACK to avoid exception
					// PIXI doesnt like diamond-shaped UVs, because they are different in canvas and webgl
					rotatedTexture = new PIXI.Texture(texture.baseTexture, frame, crop, trim, rotate - 1);
					rotatedTexture.rotate++;
				}
				textures.push(rotatedTexture);
			}

			const offsetX = (app.screen.width / 16) | 0;
			const offsetY = (app.screen.height / 8) | 0;
			const gridW = (app.screen.width / 4) | 0;
			const gridH = (app.screen.height / 5) | 0;

			// normal rotations and mirrors
			for (let i = 0; i < 16; i++) {
				// create a new Sprite using rotated texture
				const dude = new PIXI.Sprite(textures[i < 8 ? i * 2 : (i - 8) * 2 + 1]);
				dude.scale.x = 0.5;
				dude.scale.y = 0.5;
				// show it in grid
				dude.x = offsetX + gridW * (i % 4);
				dude.y = offsetY + gridH * ((i / 4) | 0);
				app.stage.addChild(dude);
				const text = new PIXI.Text(`rotate = ${dude.texture.rotate}`, {
					fontFamily: 'Courier New',
					fontSize: '12px',
					fill: 'white',
					align: 'left',
				});
				text.x = dude.x;
				text.y = dude.y - 20;
				app.stage.addChild(text);
			}
		}
	}

	multiPassShaderGenMesh(canvas) {
		const app = new TNSPIXIApplication({ canvas });
		app.view.height = 640;
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
	}

	meshSharingGeo(canvas) {
		const app = new TNSPIXIApplication({ canvas });
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
		const app = new TNSPIXIApplication({ canvas });

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
	}

	renderTextureAdvance(canvas) {
		const app = new TNSPIXIApplication({ canvas });
		// create two render textures... these dynamic textures will be used to draw the scene into itself

		// create two render textures... these dynamic textures will be used to draw the scene into itself
		let renderTexture = PIXI.RenderTexture.create({
			width: app.screen.width,
			height: app.screen.height,
		});
		let renderTexture2 = PIXI.RenderTexture.create({
			width: app.screen.width,
			height: app.screen.height,
		});
		const currentTexture = renderTexture;

		// create a new sprite that uses the render texture we created above
		const outputSprite = new PIXI.Sprite(currentTexture);

		// align the sprite
		outputSprite.x = 400;
		outputSprite.y = 300;
		outputSprite.anchor.set(0.5);

		// add to stage
		app.stage.addChild(outputSprite);

		const stuffContainer = new PIXI.Container();

		stuffContainer.x = 400;
		stuffContainer.y = 300;

		app.stage.addChild(stuffContainer);

		// create an array of image ids..
		const fruits = [this.root + '/images/rt_object_01.png', this.root + '/images/rt_object_02.png', this.root + '/images/rt_object_03.png', this.root + '/images/rt_object_04.png', this.root + '/images/rt_object_05.png', this.root + '/images/rt_object_06.png', this.root + '/images/rt_object_07.png', this.root + '/images/rt_object_08.png'];

		// create an array of items
		const items = [];

		// now create some items and randomly position them in the stuff container
		for (let i = 0; i < 40; i++) {
			const item = PIXI.Sprite.from(fruits[i % fruits.length]);
			item.x = Math.random() * 400 - 200;
			item.y = Math.random() * 400 - 200;
			item.anchor.set(0.5);
			stuffContainer.addChild(item);
			items.push(item);
		}

		// used for spinning!
		let count = 0;

		app.ticker.add(() => {
			for (let i = 0; i < items.length; i++) {
				// rotate each item
				const item = items[i];
				item.rotation += 0.1;
			}

			count += 0.01;

			// swap the buffers ...
			const temp = renderTexture;
			renderTexture = renderTexture2;
			renderTexture2 = temp;

			// set the new texture
			outputSprite.texture = renderTexture;

			// twist this up!
			stuffContainer.rotation -= 0.01;
			outputSprite.scale.set(1 + Math.sin(count) * 0.2);

			// render the stage to the texture
			// the 'true' clears the texture before the content is rendered
			app.renderer.render(app.stage, renderTexture2, false);
		});
	}

	meshBasic(canvas) {
		// const app = new TNSPIXIApplication({canvas});
		const app = new TNSPIXIApplication({ canvas });
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
	}

	meshAdvance(canvas) {
		const app = new TNSPIXIApplication({ canvas });
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
	}

	dynamicGraphics(canvas) {
		const app = new TNSPIXIApplication({ canvas, antialias: true });

		app.stage.interactive = true;

		const graphics = new PIXI.Graphics();

		// set a fill and line style
		graphics.beginFill(0xff3300);
		graphics.lineStyle(10, 0xffd900, 1);

		// draw a shape
		graphics.moveTo(50, 50);
		graphics.lineTo(250, 50);
		graphics.lineTo(100, 100);
		graphics.lineTo(250, 220);
		graphics.lineTo(50, 220);
		graphics.lineTo(50, 50);
		graphics.closePath();
		graphics.endFill();

		// set a fill and line style again
		graphics.lineStyle(10, 0xff0000, 0.8);
		graphics.beginFill(0xff700b, 1);

		// draw a second shape
		graphics.moveTo(210, 300);
		graphics.lineTo(450, 320);
		graphics.lineTo(570, 350);
		graphics.quadraticCurveTo(600, 0, 480, 100);
		graphics.lineTo(330, 120);
		graphics.lineTo(410, 200);
		graphics.lineTo(210, 300);
		graphics.closePath();
		graphics.endFill();

		// draw a rectangle
		graphics.lineStyle(2, 0x0000ff, 1);
		graphics.drawRect(50, 250, 100, 100);

		// draw a circle
		graphics.lineStyle(0);
		graphics.beginFill(0xffff0b, 0.5);
		graphics.drawCircle(470, 200, 100);
		graphics.endFill();

		graphics.lineStyle(20, 0x33ff00);
		graphics.moveTo(30, 30);
		graphics.lineTo(600, 300);

		app.stage.addChild(graphics);

		// let's create a moving shape
		const thing = new PIXI.Graphics();
		app.stage.addChild(thing);
		thing.x = 800 / 2;
		thing.y = 600 / 2;

		let count = 0;

		// Just click on the stage to draw random lines
		(window as any).app = app;
		app.renderer.plugins.interaction.on('pointerdown', onPointerDown);

		function onPointerDown() {
			graphics.lineStyle(Math.random() * 30, Math.random() * 0xffffff, 1);
			graphics.moveTo(Math.random() * 800, Math.random() * 600);
			graphics.bezierCurveTo(Math.random() * 800, Math.random() * 600, Math.random() * 800, Math.random() * 600, Math.random() * 800, Math.random() * 600);
		}

		app.ticker.add(() => {
			count += 0.1;

			thing.clear();
			thing.lineStyle(10, 0xff0000, 1);
			thing.beginFill(0xffff00, 0.5);

			thing.moveTo(-120 + Math.sin(count) * 20, -100 + Math.cos(count) * 20);
			thing.lineTo(120 + Math.cos(count) * 20, -100 + Math.sin(count) * 20);
			thing.lineTo(120 + Math.sin(count) * 20, 100 + Math.cos(count) * 20);
			thing.lineTo(-120 + Math.cos(count) * 20, 100 + Math.sin(count) * 20);
			thing.lineTo(-120 + Math.sin(count) * 20, -100 + Math.cos(count) * 20);
			thing.closePath();

			thing.rotation = count * 0.1;
		});
	}

	bitmapFont(canvas) {
		const app = new TNSPIXIApplication({ canvas, backgroundColor: 0x1099bb });
		app.loader.add('desyrel', this.root + '/bitmap-font/desyrel.xml').load(onAssetsLoaded);

		function onAssetsLoaded() {
			const bitmapFontText = new (PIXI as any).BitmapText('bitmap fonts are supported!\nWoo yay!', { font: '55px Desyrel', align: 'left' });

			bitmapFontText.x = 50;
			bitmapFontText.y = 200;

			app.stage.addChild(bitmapFontText);
		}
	}

	explosion(canvas) {
		const app = new TNSPIXIApplication({ canvas, backgroundColor: 0x1099bb });

		app.stop();

		const onAssetsLoaded = () => {
			// create an array to store the textures
			const explosionTextures = [];
			let i;

			for (i = 0; i < 26; i++) {
				const texture = PIXI.Texture.from(`Explosion_Sequence_A ${i + 1}.png`);
				explosionTextures.push(texture);
			}

			for (i = 0; i < 50; i++) {
				// create an explosion AnimatedSprite
				const explosion = new PIXI.AnimatedSprite(explosionTextures);

				explosion.x = Math.random() * app.screen.width;
				explosion.y = Math.random() * app.screen.height;
				explosion.anchor.set(0.5);
				explosion.rotation = Math.random() * Math.PI;
				explosion.scale.set(0.75 + Math.random() * 0.5);
				explosion.gotoAndPlay(Math.random() * 27);
				app.stage.addChild(explosion);
			}
			// start animating
			app.start();
		};

		app.loader.add('spritesheet', this.root + '/spritesheet/mc.json').load(onAssetsLoaded);
	}

	starWarp(canvas) {
		const app = new TNSPIXIApplication({ canvas });

		// Get the texture for rope.
		const starTexture = PIXI.Texture.from(this.root + '/images/star.png');

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
		app.ticker.add((delta) => {
			// Simple easing. This should be changed to proper easing function when used for real.
			speed += (warpSpeed - speed) / 20;
			cameraZ += delta * 10 * (speed + baseSpeed);
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
				// Scale the star depending on how fast we are moving, what the stretchfactor is and depending on how far away it is from the center.
				star.sprite.scale.y = distanceScale * starBaseSize + (distanceScale * speed * starStretch * distanceCenter) / app.renderer.screen.width;
				star.sprite.rotation = Math.atan2(dyCenter, dxCenter) + Math.PI / 2;
			}
		});
	}

	container(canvas) {
		const app = new TNSPIXIApplication({ canvas, backgroundColor: 0x1099bb });
		const container = new PIXI.Container();
		app.stage.addChild(container);
		const loader = new PIXI.Loader();
		loader.add('bunny', this.root + '/images/bunny.png');
		loader.load((loader, resources) => {
			// Create a new texture
			const texture = new PIXI.Texture(resources.bunny.texture);

			// Create a 5x5 grid of bunnies
			for (let i = 0; i < 25; i++) {
				const bunny = new PIXI.Sprite(texture);
				bunny.anchor.set(0.5);
				bunny.x = (i % 5) * 40;
				bunny.y = Math.floor(i / 5) * 40;
				container.addChild(bunny);
			}

			// Move container to the center
			container.x = app.screen.width / 2;
			container.y = app.screen.height / 2;

			// Center bunny sprite in local container coordinates
			container.pivot.x = container.width / 2;
			container.pivot.y = container.height / 2;

			// Listen for animate update
			app.ticker.add((delta) => {
				// rotate the container!
				// use delta to create frame-independent transform
				container.rotation -= 0.01 * delta;
			});
		});
	}

	transparent(canvas) {
		const app = new TNSPIXIApplication({ canvas, transparent: true });

		// create a new Sprite from an image path.
		const bunny = PIXI.Sprite.from(this.root + '/images/bunny.png');

		// center the sprite's anchor point
		bunny.anchor.set(0.5);

		// move the sprite to the center of the screen
		bunny.x = app.screen.width / 2;
		bunny.y = app.screen.height / 2;

		app.stage.addChild(bunny);

		app.ticker.add(() => {
			// just for fun, let's rotate mr rabbit a little
			bunny.rotation += 0.1;
		});
	}

	eggHead(canvas) {
		const app = new TNSPIXIApplication({ canvas });
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
		const app = new TNSPIXIApplication({ canvas });

		app.stop();

		// load resources
		app.loader.add('spritesheet', this.root + '/spritesheet/monsters.json').load(onAssetsLoaded);

		// holder to store aliens
		const aliens = [];
		const alienFrames = ['eggHead.png', 'flowerTop.png', 'helmlok.png', 'skully.png'];

		let count = 0;

		// create an empty container
		const alienContainer = new PIXI.Container();
		alienContainer.x = 400;
		alienContainer.y = 300;

		// make the stage interactive
		app.stage.interactive = true;
		app.stage.addChild(alienContainer);

		function onAssetsLoaded() {
			// add a bunch of aliens with textures from image paths
			for (let i = 0; i < 100; i++) {
				const frameName = alienFrames[i % 4];

				// create an alien using the frame name..
				const alien = PIXI.Sprite.from(frameName);
				alien.tint = Math.random() * 0xffffff;

				/*
				 * fun fact for the day :)
				 * another way of doing the above would be
				 * var texture = PIXI.Texture.from(frameName);
				 * var alien = new PIXI.Sprite(texture);
				 */
				alien.x = Math.random() * 800 - 400;
				alien.y = Math.random() * 600 - 300;
				alien.anchor.x = 0.5;
				alien.anchor.y = 0.5;
				aliens.push(alien);
				alienContainer.addChild(alien);
			}
			app.start();
		}

		// Combines both mouse click + touch tap
		app.stage.on('pointertap', onClick);

		function onClick() {
			alienContainer.cacheAsBitmap = !alienContainer.cacheAsBitmap;

			// feel free to play with what's below
			// var sprite = new PIXI.Sprite(alienContainer.generateTexture());
			// app.stage.addChild(sprite);
			// sprite.x = Math.random() * 800;
			// sprite.y = Math.random() * 600;
		}

		app.ticker.add(() => {
			// let's rotate the aliens a little bit
			for (let i = 0; i < 100; i++) {
				const alien = aliens[i];
				alien.rotation += 0.1;
			}

			count += 0.01;

			alienContainer.scale.x = Math.sin(count);
			alienContainer.scale.y = Math.sin(count);
			alienContainer.rotation += 0.01;
		});
	}

	particleContainer(canvas) {
		const app = new TNSPIXIApplication({ canvas });
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
	}

	blendModes(canvas) {
		const app = new TNSPIXIApplication({ canvas });

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
	}

	simplePlane(canvas) {
		const app = new TNSPIXIApplication({
			canvas,
			backgroundColor: 0x1099bb,
		});
		app.loader.add('bg_grass', this.root + '/images/bg_grass.jpg').load(build);

		function build() {
			// Create a new texture
			const texture = app.loader.resources.bg_grass.texture;

			// Create the simple plane
			const verticesX = 10;
			const verticesY = 10;
			const plane = new PIXI.SimplePlane(texture, verticesX, verticesY);

			plane.x = 100;
			plane.y = 100;

			app.stage.addChild(plane);

			// Get the buffer for vertice positions.
			const buffer = plane.geometry.getBuffer('aVertexPosition') as any;
			// Listen for animate update
			app.ticker.add((delta) => {
				// Randomize the vertice positions a bit to create movement.
				for (let i = 0; i < buffer.data.length; i++) {
					buffer.data[i] += Math.random() - 0.5;
				}
				buffer.update();
			});
		}
	}

	animatedJet(canvas) {
		const app = new TNSPIXIApplication({
			canvas,
		});

		app.loader.add(this.root + '/spritesheet/fighter.json').load(onAssetsLoaded);

		function onAssetsLoaded() {
			// create an array of textures from an image path
			const frames = [];

			for (let i = 0; i < 30; i++) {
				const val = i < 10 ? `0${i}` : i;

				// magically works since the spritesheet was loaded with the pixi loader
				frames.push(PIXI.Texture.from(`rollSequence00${val}.png`));
			}

			// create an AnimatedSprite (brings back memories from the days of Flash, right ?)
			const anim = new PIXI.AnimatedSprite(frames);

			/*
			 * An AnimatedSprite inherits all the properties of a PIXI sprite
			 * so you can change its position, its anchor, mask it, etc
			 */
			anim.x = app.screen.width / 2;
			anim.y = app.screen.height / 2;
			anim.anchor.set(0.5);
			anim.animationSpeed = 0.5;
			anim.play();

			app.stage.addChild(anim);

			// Animate the rotation
			app.ticker.add(() => {
				anim.rotation += 0.01;
			});
		}
	}

	text(canvas) {
		const app = new TNSPIXIApplication({
			canvas,
			backgroundColor: 0x1099bb,
		});

		const basicText = new PIXI.Text('Basic text in pixi');
		basicText.x = 50;
		basicText.y = 100;

		app.stage.addChild(basicText);

		const style = new PIXI.TextStyle({
			fontFamily: 'Arial',
			fontSize: 300,
			fontStyle: 'italic',
			fontWeight: 'bold',
			fill: ['#ffffff', '#00ff99'], // gradient
			stroke: '#4a1850',
			strokeThickness: 5,
			dropShadow: true,
			dropShadowColor: '#000000',
			dropShadowBlur: 4,
			dropShadowAngle: Math.PI / 6,
			dropShadowDistance: 6,
			wordWrap: true,
			wordWrapWidth: 440,
		});

		const richText = new PIXI.Text('Rich text with a lot of options and across multiple lines', style);
		richText.x = 50;
		richText.y = 250;

		app.stage.addChild(richText);
	}

	drawPatternWithCanvas(canvas) {
		const scale = 1; //Screen.mainScreen.scale;
		const patternCanvas = document.createElement('canvas');
		// Give the pattern a width and height of 50
		patternCanvas.width = 50 * scale;
		patternCanvas.height = 50 * scale;

		const patternContext = patternCanvas.getContext('2d') as any;

		// Give the pattern a background color and draw an arc
		patternContext.fillStyle = '#fec';
		patternContext.fillRect(0, 0, patternCanvas.width, patternCanvas.height);
		patternContext.arc(0, 0, 50 * Screen.mainScreen.scale, 0, 0.5 * Math.PI);
		patternContext.stroke();

		// Create our primary canvas and fill it with the pattern
		const ctx = canvas.getContext('2d');
		const pattern = ctx.createPattern(patternCanvas, 'repeat');
		ctx.fillStyle = pattern;
		ctx.fillRect(0, 0, canvas.width, canvas.height);
	}

	/* Graphics */
	simple(canvas) {
		const app = new TNSPIXIApplication({
			canvas,
			backgroundColor: 0x1099bb,
		});
		const graphics = new PIXI.Graphics();

		// Rectangle
		graphics.beginFill(0xde3249);
		graphics.drawRect(50, 50, 100, 100);
		graphics.endFill();

		// Rectangle + line style 1
		graphics.lineStyle(2, 0xfeeb77, 1);
		graphics.beginFill(0x650a5a);
		graphics.drawRect(200, 50, 100, 100);
		graphics.endFill();

		// Rectangle + line style 2
		graphics.lineStyle(10, 0xffbd01, 1);
		graphics.beginFill(0xc34288);
		graphics.drawRect(350, 50, 100, 100);
		graphics.endFill();

		// Rectangle 2
		graphics.lineStyle(2, 0xffffff, 1);
		graphics.beginFill(0xaa4f08);
		graphics.drawRect(530, 50, 140, 100);
		graphics.endFill();

		// Circle
		graphics.lineStyle(0); // draw a circle, set the lineStyle to zero so the circle doesn't have an outline
		graphics.beginFill(0xde3249, 1);
		graphics.drawCircle(100, 250, 50);
		graphics.endFill();

		// Circle + line style 1
		graphics.lineStyle(2, 0xfeeb77, 1);
		graphics.beginFill(0x650a5a, 1);
		graphics.drawCircle(250, 250, 50);
		graphics.endFill();

		// Circle + line style 2
		graphics.lineStyle(10, 0xffbd01, 1);
		graphics.beginFill(0xc34288, 1);
		graphics.drawCircle(400, 250, 50);
		graphics.endFill();

		// Ellipse + line style 2
		graphics.lineStyle(2, 0xffffff, 1);
		graphics.beginFill(0xaa4f08, 1);
		graphics.drawEllipse(600, 250, 80, 50);
		graphics.endFill();

		// draw a shape
		graphics.beginFill(0xff3300);
		graphics.lineStyle(4, 0xffd900, 1);
		graphics.moveTo(50, 350);
		graphics.lineTo(250, 350);
		graphics.lineTo(100, 400);
		graphics.lineTo(50, 350);
		graphics.closePath();
		graphics.endFill();

		// draw a rounded rectangle
		graphics.lineStyle(2, 0xff00ff, 1);
		graphics.beginFill(0x650a5a, 0.25);
		graphics.drawRoundedRect(50, 440, 100, 100, 16);
		graphics.endFill();

		// draw star
		graphics.lineStyle(2, 0xffffff);
		graphics.beginFill(0x35cc5a, 1);
		graphics.drawStar(360, 370, 5, 50);
		graphics.endFill();

		// draw star 2
		graphics.lineStyle(2, 0xffffff);
		graphics.beginFill(0xffcc5a, 1);
		graphics.drawStar(280, 510, 7, 50);
		graphics.endFill();

		// draw star 3
		graphics.lineStyle(4, 0xffffff);
		graphics.beginFill(0x55335a, 1);
		graphics.drawStar(470, 450, 4, 50);
		graphics.endFill();

		// draw polygon
		const path = [600, 370, 700, 460, 780, 420, 730, 570, 590, 520];

		graphics.lineStyle(0);
		graphics.beginFill(0x3500fa, 1);
		graphics.drawPolygon(path);
		graphics.endFill();

		app.stage.addChild(graphics);
	}

	advance(canvas) {
		const app = new TNSPIXIApplication({
			canvas,
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
	}
}
