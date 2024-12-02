//import { CanvasRenderingContext2D, WebGLRenderingContext } from './index.js';

import { createRequire } from 'node:module';

// @ts-ignore
const require = createRequire(import.meta.url);

import '@nativescript/macos-node-api';
import '@nativescript/foundation/dom/index.js';
import Application from './app.ts';
const { CanvasRenderingContext2D, ImageAsset, Path2D } = require('./canvas-napi.darwin-arm64.node');

const { requestAnimationFrame } = require('./utils/index.ts');

import './canvas.ts';

objc.import('OpenGL');

function mdnShadowColor(ctx: any) {
	// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor

	ctx.fillStyle = 'black';

	// Shadow
	ctx.shadowColor = 'red';
	ctx.shadowOffsetX = 10;
	ctx.shadowOffsetY = 10;

	// Filled rectangle
	ctx.fillRect(20, 20, 100, 100);

	// Stroked rectangle
	ctx.lineWidth = 6;

	ctx.strokeRect(170, 20, 100, 100);
}

function mdnStrokeText(ctx: any) {
	ctx.font = '50px serif';
	ctx.fillText('Hello world', 50, 90);
}

function mdnRoundRect(ctx: any) {
	// Rounded rectangle with zero radius (specified as a number)
	ctx.strokeStyle = 'red';
	ctx.beginPath();
	ctx.roundRect(10, 20, 150, 100, 0);
	ctx.stroke();

	// Rounded rectangle with 40px radius (single element list)
	ctx.strokeStyle = 'blue';
	ctx.beginPath();
	ctx.roundRect(10, 20, 150, 100, [40]);
	ctx.stroke();

	// Rounded rectangle with 2 different radii
	ctx.strokeStyle = 'orange';
	ctx.beginPath();
	ctx.roundRect(10, 150, 150, 100, [10, 40]);
	ctx.stroke();

	// Rounded rectangle with four different radii
	ctx.strokeStyle = 'green';
	ctx.beginPath();
	ctx.roundRect(400, 20, 200, 100, [0, 30, 50, 60]);
	ctx.stroke();

	// Same rectangle drawn backwards
	ctx.strokeStyle = 'magenta';
	ctx.beginPath();
	ctx.roundRect(400, 150, -200, 100, [0, 30, 50, 60]);
	ctx.stroke();
}

function doTheThing(canvas: any) {
	const asset = new ImageAsset();

	let loaded = false;
	// console.time('load1');
	// loaded = asset.fromUrlSync('https://www.superherotoystore.com/cdn/shop/articles/Website_Blog_creatives_29_1600x.jpg?v=1713945144');
	// console.timeEnd('load1');

	const scale = NSScreen.mainScreen.backingScaleFactor;

	const ctx = canvas.getContext('2d') as CanvasRenderingContext2D & { render: () => void };

	ctx.fillStyle = 'white';

	ctx.fillRect(0, 0, 1000, 1000);

	ctx.fillStyle = 'black';

	ctx.scale(scale, scale);

	//mdnShadowColor(ctx);
	mdnRoundRect(ctx);

	ctx.render();

	/*

	asset.fromUrl('https://picsum.photos/id/1/200/300')
		.then(function(done) {
			console.log('ftghjkl');
			console.log(NSThread.currentThread);
			console.log('fromUrl: done', done, NSThread.currentThread);
			ctx.drawImage(asset, 0, 0, glview.frame.size.width * scale, glview.frame.size.height * scale);

			ctx.render();
		}).catch(e => {
		console.log('fromUrl: error', e);
		console.log(NSThread.currentThread);
	})
		.catch(e => {
			console.log('e', e);
		}).finally(() => {
		console.log('?');
	});
	*/
}

const window = document.createElement('window');
window.style.width = `${NSScreen.mainScreen.frame.size.width / 2}`;
window.style.height = `${NSScreen.mainScreen.frame.size.height / 2}`;

const canvas = document.createElement('canvas');

canvas.addEventListener('ready', (event) => {
	doTheThing(canvas);
});

canvas.width = NSScreen.mainScreen.frame.size.width / 2;
canvas.height = NSScreen.mainScreen.frame.size.height / 2;

// canvas.style.width = `${NSScreen.mainScreen.frame.size.width / 2}`;
// canvas.style.height = `${NSScreen.mainScreen.frame.size.height / 2}`;

window.setAttribute('styleMask', (NSWindowStyleMask.Titled | NSWindowStyleMask.Closable | NSWindowStyleMask.Miniaturizable | NSWindowStyleMask.Resizable | NSWindowStyleMask.FullSizeContentView) as any);

const txt = document.createElement('text');
txt.textContent = 'Hello, World!';
txt.style.color = 'black';
txt.style.fontSize = '20px';
txt.style.textAlign = 'center';
txt.style.width = '100%';
txt.style.backgroundColor = 'red';
window.appendChild(txt);
window.appendChild(canvas);

canvas.style.width = '100%';
canvas.style.height = '100%';
canvas.style.backgroundColor = 'blue';

document.body.appendChild(window);

Application.launch();
