import { Application, BitmapText, Container, Graphics } from 'pixi.js';

const PILL_WIDTH = 60;
const PILL_HEIGHT = 65;
const PILL_RADIUS = 10;
const PILL_GAP = 10;
const NUM_PILLS = 512;
const COLUMNS = 32;

export async function runOnCanvas(canvas: any) {
	const app = new Application();

	// Initialize the application
	await app.init({
		background: '#1099bb',
		canvas: canvas as any,
		width: canvas.width,
		height: canvas.height,
		preference: 'webgpu',
	});

	// // grab context to present
	//   const ctx = canvas.getContext('webgpu');

	//   app.ticker.add((delta) => {
	//       if (ctx) {
	//           ctx.presentSurface();
	//       }
	//   });

	// Append the application canvas to the document body

	// Create and add a container to the stage
	const container = new Container();
	app.stage.addChild(container);

	const pills: {
		container: Container;
		bar: Graphics;
		label: BitmapText;
	}[] = [];

	for (let i = 0; i < NUM_PILLS; i++) {
		const x = (i % COLUMNS) * (PILL_WIDTH + PILL_GAP);
		const y = Math.floor(i / COLUMNS) * (PILL_HEIGHT + PILL_GAP);

		const pillContainer = new Container({ x, y });

		// Background (static)
		const background = new Graphics();
		background.roundRect(0, 0, PILL_WIDTH, PILL_HEIGHT, PILL_RADIUS).fill(0xf3f4f6);

		// Foreground bar (dynamic)
		const bar = new Graphics();
		bar.roundRect(0, 0, PILL_WIDTH, PILL_HEIGHT, PILL_RADIUS).fill(0xe12afb);
		bar.pivot.y = 0; // scale from top
		bar.scale.y = 0;

		// Label
		const label = new BitmapText({
			text: '0.00',
			style: {
				fontFamily: 'Inter',
				fontSize: 14,
				fontWeight: 'bold',
				fill: 0x000000,
			},
			x: PILL_WIDTH / 2,
			y: PILL_HEIGHT / 2,
			anchor: 0.5,
		});

		pillContainer.addChild(background, bar, label);
		container.addChild(pillContainer);

		pills[i] = { container: pillContainer, bar, label };
	}

	// Center the grid
	container.x = app.screen.width / 2;
	container.y = app.screen.height / 2;
	container.pivot.x = container.width / 2;
	container.pivot.y = container.height / 2;

	let waveLength = 32;
	const waveSpeed = 2;

	app.ticker.add(() => {
		const time = app.ticker.lastTime / 1000;

		for (let i = 0; i < NUM_PILLS; i++) {
			const phase = (i / waveLength) * Math.PI * 2 - time * waveSpeed * Math.PI * 2;
			const value = (Math.sin(phase) + 1) / 2;

			const { bar, label } = pills[i];

			// Scale from top (no y-shift needed)
			bar.scale.y = value;

			const newText = value.toFixed(2);
			if (label.text !== newText) {
				label.text = newText;
			}
		}
	});
}
