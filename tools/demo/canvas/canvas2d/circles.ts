import { Canvas } from '@nativescript/canvas';

class Circle {
	x: number;
	y: number;
	radius: any;
	angle: number;
	firstColor: string;
	secondColor: string;

	constructor({ width, height, minRadius, maxRadius }) {
		this.x = Math.random() * width;
		this.y = Math.random() * height;
		this.radius = Math.random() * (maxRadius - minRadius) + minRadius;
		this.angle = Math.random() * 2 * Math.PI;
		this.firstColor = `hsla(${Math.random() * 360}, 100%, 50%, ${Math.random()})`;
	}

	draw(context, speed) {
		this.angle += speed;

		const x = this.x + Math.cos(this.angle) * 200;
		const y = this.y + Math.sin(this.angle) * 200;

		context.fillStyle = this.firstColor;
		context.beginPath();
		context.arc(x, y, this.radius, 0, 2 * Math.PI);
		context.fill();
	}
}

let circlesNum = 500;
let minRadius = 100;
let maxRadius = 1200;
let speed = 0.01;
let circles = new Array(circlesNum);
let canvas;
let ctx;

let started = false;
let cancelled = false;
let raf;

export function circle_demo(view) {
	canvas = view;

	const width = canvas.clientWidth * window.devicePixelRatio;
	const height = canvas.clientHeight * window.devicePixelRatio;

	canvas.width = width;
	canvas.height = height;

	view.on('unloaded', (args) => {
		cancelled = true;
		if (raf) {
			cancelAnimationFrame(raf);
			raf = undefined;
		}
	});

	view.on('loaded', (args) => {
		cancelled = false;
		if (started) {
			//	canvas.nativeView.forceResize();
			drawAnimation();
		}
	});

	circles.forEach((circle) => {
		if (canvas) {
			circle.width = canvas.width * 0.25;
			circle.height = canvas.height * 0.25;
		}
	});

	// canvas.width = width;
	// canvas.height = height;

	canvas = canvas as Canvas;
	ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
	//canvas.width = canvas.clientWidth * window.devicePixelRatio;
	//canvas.height = canvas.clientHeight * window.devicePixelRatio;

	//const height = canvas.clientHeight * window.devicePixelRatio;
	// canvas.style.width = `${width}px`;
	// canvas.style.height = `${height}px`;

	minRadius = Math.min(width, height) * 0.05;
	maxRadius = Math.max(width, height) * 0.1;

	for (let i = 0; i < circlesNum; i++) {
		circles[i] = new Circle({
			width,
			height,
			minRadius: minRadius,
			maxRadius: maxRadius,
		});
	}

	drawAnimation();
	// _zone.runOutsideAngular(() => {
	//  drawAnimation();
	// });
}

function drawAnimation() {
	if (cancelled) {
		return;
	}
	started = true;
	ctx.clearRect(0, 0, canvas.width, canvas.height);

	for (const circle of circles) [circle.draw(ctx, speed)];

	raf = requestAnimationFrame(() => drawAnimation());
}
