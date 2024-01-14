import { Canvas } from '@nativescript/canvas';

export function rnSkiaPerf(canvas: Canvas) {
	const ctx = canvas.getContext('2d');

	let numberOfBoxes = 1200;
	const { width, height } = canvas as any;

	const Size = 25;
	const Increaser = 50;

	const SizeWidth = Size;
	const SizeHeight = Size * 0.45;

	const pos = {
		x: width / 2,
		y: height * 0.25,
	};

	const rects = new Array(numberOfBoxes).fill(0).map((_, i) => {
		return {
			x: 5 + ((i * Size) % width),
			y: 25 + Math.floor(i / (width / Size)) * Size,
			width: SizeWidth,
			height: SizeHeight,
		};
	});

	function draw() {
		ctx.clearRect(0, 0, width, height);
		for (const rect of rects) {
			ctx.save();
			ctx.translate(rect.x, rect.y);
			const p1 = { x: rect.x, y: rect.y };
			const p2 = pos;
			const r = Math.atan2(p2.y - p1.y, p2.x - p1.x);
			ctx.rotate(r);
			ctx.fillStyle = '#00ff00';
			ctx.fillRect(0, 0, rect.width, rect.height);
			ctx.strokeStyle = '#4060A3';
			ctx.lineWidth = 2;
			ctx.strokeRect(0, 0, rect.width, rect.height);
			ctx.restore();
		}
	}

	canvas.addEventListener('touchmove', (args: TouchEvent) => {
		const first = args.changedTouches[0];
		pos.x = first.clientX;
		pos.y = first.clientY;
		draw();
	});

	draw();
}
