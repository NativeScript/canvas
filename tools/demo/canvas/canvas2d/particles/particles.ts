import { GestureTypes, TouchGestureEventData, Screen } from '@nativescript/core';

let LAF = 0;

var config = {
	particleNumber: 50,
	maxParticleSize: 10,
	maxSpeed: 40,
	colorVariation: 10,
};
var width = 0,
	height = 0;
var particles = [],
	drawParticle;

var colorPalette = {
	bg: { r: 12, g: 9, b: 29 },
	matter: [
		{ r: 36, g: 18, b: 42 }, // darkPRPL
		{ r: 78, g: 36, b: 42 }, // rockDust
		{ r: 252, g: 178, b: 96 }, // solorFlare
		{ r: 253, g: 238, b: 152 }, // totesASun
	],
};

// Provides some nice color variation
// Accepts an rgba object
// returns a modified rgba object or a rgba string if true is passed in for argument 2
var colorVariation = function (color, returnString) {
	var r, g, b, a, variation;
	r = Math.round(Math.random() * config.colorVariation - config.colorVariation / 2 + color.r);
	g = Math.round(Math.random() * config.colorVariation - config.colorVariation / 2 + color.g);
	b = Math.round(Math.random() * config.colorVariation - config.colorVariation / 2 + color.b);
	a = Math.random() + 0.5;
	if (returnString) {
		return 'rgba(' + r + ',' + g + ',' + b + ',' + a + ')';
	} else {
		return { r, g, b, a };
	}
};

// Particle Constructor
var Particle = function (x, y) {
	// X Coordinate
	this.x = x || Math.round(Math.random() * width);
	// Y Coordinate
	this.y = y || Math.round(Math.random() * height);
	// Radius of the space dust
	this.r = Math.ceil(Math.random() * config.maxParticleSize);
	// Color of the rock, given some randomness
	this.c = colorVariation(colorPalette.matter[Math.floor(Math.random() * colorPalette.matter.length)], true);
	// Speed of which the rock travels
	this.s = Math.pow(Math.ceil(Math.random() * config.maxSpeed), 0.7);
	// Direction the Rock flies
	this.d = Math.round(Math.random() * 360);
};

function touchParticles(canvas, w?, h?, nativeCanvas?) {
	canvas.width = canvas.clientWidth * window.devicePixelRatio;
	canvas.height = canvas.clientHeight * window.devicePixelRatio;
	width = w || canvas.width;
	height = h || canvas.height;

	const context = canvas.getContext ? canvas.getContext('2d') : canvas;

	//context.scale(window.devicePixelRatio, window.devicePixelRatio);

	// const ptr = context.canvas.nativeView.getNativeContext();
	// console.log('context.canvas.nativeView.getNativeContext()', ptr);
	//const v8Ctx = global.__getCanvasRenderingContext2DImpl(String(ptr));

	const ctx = context;

	// Set Canvas to be window size
	//  canvas.width = window.innerWidth;
	//canvas.height = window.innerHeight;

	// Configuration, Play with these

	// Colors

	// Some Variables hanging out
	var centerX = width / 2,
		//@ts-ignore
		centerY = height / 2,
		drawBg;

	// Draws the background for the canvas, because space
	drawBg = function (ctx, color) {
		ctx.fillStyle = 'rgb(' + color.r + ',' + color.g + ',' + color.b + ')';
		// ctx.clearRect(0, 0, width, height);
		ctx.fillRect(0, 0, width, height);
	};

	// Used to find the rocks next point in space, accounting for speed and direction
	var updateParticleModel = function (p) {
		var a = 180 - (p.d + 90); // find the 3rd angle
		p.d > 0 && p.d < 180 ? (p.x += (p.s * Math.sin(p.d)) / Math.sin(p.s)) : (p.x -= (p.s * Math.sin(p.d)) / Math.sin(p.s));
		p.d > 90 && p.d < 270 ? (p.y += (p.s * Math.sin(a)) / Math.sin(p.s)) : (p.y -= (p.s * Math.sin(a)) / Math.sin(p.s));
		return p;
	};

	// Just the function that physically draws the particles
	// Physically? sure why not, physically.
	drawParticle = function (x, y, r, c) {
		ctx.beginPath();
		ctx.fillStyle = c;
		ctx.arc(x, y, r, 0, 2 * Math.PI, false);
		ctx.fill();
		ctx.closePath();
	};

	// Remove particles that aren't on the canvas

	// That thing
	var requestAnimFrame = requestAnimationFrame;

	// Our Frame function
	var frame = function () {
		LAF = requestAnimFrame(frame);
		// Draw background first
		drawBg(ctx, colorPalette.bg);
		// Update Particle models to new position
		particles.map((p) => {
			return updateParticleModel(p);
		});
		// Draw em'
		for (const p of particles) {
			drawParticle(p.x, p.y, p.r, p.c);
		}
		// Play the same song? Ok!
	};

	// Click listener
	if (!nativeCanvas) {
		//canvas.parent.on(GestureTypes.tap as any, (args) => {});
		canvas.addEventListener('touchstart', (args: TouchEvent) => {
			const touches = args.touches.item(0);
			const first = touches;
			console.log('first', first.clientX, first.clientY);
			cleanUpArray();
			initParticles(first.clientX * window.devicePixelRatio, first.clientY * window.devicePixelRatio);
		});

		canvas.addEventListener('touchmove', (args) => {
			const touches = args.changedTouches;
			if (Array.isArray(touches)) {
				const first = touches[0];
				cleanUpArray();
				initParticles(first.clientX * window.devicePixelRatio, first.clientY * window.devicePixelRatio);
			}
		});

		// canvas.parent.on(GestureTypes.touch as any, (args: TouchGestureEventData) => {
		// 	var x = args.getX(),
		// 		y = (args.getY());
		// 	cleanUpArray();
		// 	initParticles(x, y);
		// });
	}

	// First Frame
	frame();

	// First particle explosion
	initParticles(config.particleNumber);
}

function cleanUpArray() {
	particles = particles.filter((p) => {
		return p.x > -100 && p.y > -100;
	});
}

function initParticles(x = 0, y = 0) {
	const numParticles = config.particleNumber;
	for (let i = 0; i < numParticles; i++) {
		particles.push(new Particle(x, y));
	}

	for (const p of particles) {
		drawParticle(p.x, p.y, p.r, p.c);
	}
}

function cancelTouchParticles() {
	cancelAnimationFrame(LAF);
	LAF = 0;
}

export { initParticles, touchParticles, cleanUpArray, cancelTouchParticles };
