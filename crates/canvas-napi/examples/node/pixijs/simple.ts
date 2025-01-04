import './adapter';
import type { Application } from 'pixi.js';
import * as PIXI from 'pixi.js';

export async function run(canvas) {
	const app = new PIXI.Application() as Application;
	canvas.width = canvas.clientWidth * window.devicePixelRatio;
	canvas.height = canvas.clientHeight * window.devicePixelRatio;

	await app.init({
		canvas,
		// preference: 'webgpu',
		width: canvas.width,
		height: canvas.height,
		background: '#1099bb',
	});

	// grab context to present
	// const ctx = canvas.getContext('webgpu');

	// app.ticker.add((delta) => {
	// 	if (ctx) {
	// 		ctx.presentSurface();
	// 	}
	// });

	const graphics = new PIXI.Graphics();

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

export async function simplePlane(canvas) {
	canvas.width = canvas.clientWidth * window.devicePixelRatio;
	canvas.height = canvas.clientHeight * window.devicePixelRatio;
	const app = new PIXI.Application();

	await app.init({
		background: '#1099bb',
		canvas,
		width: canvas.width,
		height: canvas.height,
		// preference: 'webgpu',
	});

	//app.loader.add('bg_grass', this.root + '/images/bg_grass.jpg').load(build);
	const texture = await PIXI.Assets.load('https://pixijs.com/assets/bg_grass.jpg');

	// const ctx = canvas.getContext('webgpu');
	//
	// app.ticker.add((delta) => {
	// 	if (ctx) {
	// 		ctx.presentSurface();
	// 	}
	// });

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
