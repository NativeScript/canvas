import Phaser from './phaser';

export default function Game({ canvas, width = null, height = null, renderer = 2, preventLoop = null, onRender = null, title = null, state = null, transparent = null, antialias = null, physicsConfig = null }) {
	const realWidth = canvas.width || 1;
	const realHeight = canvas.height || 1;

	(global as any).document.readyState = 'complete';

	const config = {
		width: width || realWidth,
		height: height || realHeight,
		renderer: renderer || 2,
		title: title || 'tns-phaser-game',
		state: state || null,
		transparent: transparent || false,
		antialias: antialias || true,
		physicsConfig: physicsConfig || null,
		canvas,
	};

	const game = new Phaser.Game(config);

	const render = () => {
		requestAnimationFrame(render);
		onRender && onRender();
	};
	if (!preventLoop) {
		render();
	}

	return game;
}
