//import { CanvasRenderingContext2D, WebGLRenderingContext } from './index.js';

import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);

import '@nativescript/macos-node-api';
const { CanvasRenderingContext2D, ImageAsset, Path2D } = require('./canvas-napi.darwin-arm64.node');

const { requestAnimationFrame } = require('./utils');

objc.import('OpenGL');

requestAnimationFrame(function (ts) {
	console.log('ts', ts);
});

export class ApplicationDelegate extends NSObject {
	static ObjCProtocols = [NSApplicationDelegate, NSWindowDelegate];

	static {
		NativeClass(this);
	}

	/**
	 * @param {NSNotification} _notification
	 */
	applicationDidFinishLaunching(_notification) {
		const menu = NSMenu.new();
		NSApp.mainMenu = menu;

		const appMenuItem = NSMenuItem.new();
		menu.addItem(appMenuItem);

		const appMenu = NSMenu.new();
		appMenuItem.submenu = appMenu;

		appMenu.addItemWithTitleActionKeyEquivalent('Quit', 'terminate:', 'q');

		const controller = ViewController.new();
		const window = NSWindow.windowWithContentViewController(controller);

		window.title = 'NativeScript for macOS';
		window.delegate = this;
		window.styleMask = NSWindowStyleMask.Titled | NSWindowStyleMask.Closable | NSWindowStyleMask.Miniaturizable | NSWindowStyleMask.Resizable | NSWindowStyleMask.FullSizeContentView;

		window.titlebarAppearsTransparent = true;
		window.titleVisibility = NSWindowTitleVisibility.Hidden;

		window.makeKeyAndOrderFront(this);

		NSApp.activateIgnoringOtherApps(false);
	}

	/**
	 * @param {NSNotification} _notification
	 */
	windowWillClose(_notification) {
		NSApp.terminate(this);
	}
}

export class NSCMTLView extends NSView {
	static {
		NativeClass(this);
	}
}

export class NSCCanvas extends NSView {
	static {
		NativeClass(this);
	}
}

export class CanvasGLView extends NSOpenGLView {
	static {
		NativeClass(this);
	}
	isDirty = false;
	/**
	 * @param {NSCCanvas} canvas
	 */
	canvas = null;
	fbo = 0;

	initWithFrame(frame) {
		super.initWithFrame(frame);
		return this;
	}

	prepareOpenGL() {
		super.prepareOpenGL();
	}

	clearGLContext() {
		super.clearGLContext();
		this.fbo = 0;
	}
}

export class ViewController extends NSViewController {
	static {
		NativeClass(this);
	}

	canvas: NSCCanvas;

	/**
	 * @param {NSCCanvas} canvas
	 */
	viewDidLoad() {
		super.viewDidLoad();
		this.canvas = NSCCanvas.alloc().initWithFrame(this.view.frame);
		this.view.addSubview(this.canvas);

		glview.frame = this.view.frame;

		this.canvas.addSubview(glview);

		glview.wantsLayer = true;

		doTheThing();
	}
}

const glview = CanvasGLView.alloc().initWithFrame({ x: 0, y: 0, width: 0, height: 0 });

let isDoingOrDone = false;

function mdnShadowColor(ctx) {
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

function mdnStrokeText(ctx) {
	ctx.font = '50px serif';
	ctx.fillText('Hello world', 50, 90);
}

function mdnRoundRect(ctx) {
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

function doTheThing() {
	if (isDoingOrDone) {
		return;
	}

	isDoingOrDone = true;

	const asset = new ImageAsset();

	let loaded = false;
	// console.time('load1');
	// loaded = asset.fromUrlSync('https://www.superherotoystore.com/cdn/shop/articles/Website_Blog_creatives_29_1600x.jpg?v=1713945144');
	// console.timeEnd('load1');

	const scale = NSScreen.mainScreen.backingScaleFactor;

	//const gl = WebGLRenderingContext.offscreen(600, 300, 1, true, false, false, false, 1, true, false, false, false, false, false);

	// gl.clearColor(0, 0, 1, 1);
	// gl.clear(gl.COLOR_BUFFER_BIT);
	// gl.flush();

	//console.log('gl', gl.toDataURL('image/png'));

	// const glview = CanvasGLView.alloc().initWithFrame(NSMakeRect(0, 0, 300 / scale, 150 / scale));
	// glview.wantsLayer = true;
	// glview.prepareOpenGL();

	const handle = interop.handleof(glview);
	// const glContext = WebGLRenderingContext.withView(handle.toNumber(), 1, true, false, false, false, 1, true, false, false, false, false, false);
	// console.log(glContext, 'drawingBufferWidth', glContext.drawingBufferWidth, 'drawingBufferHeight', glContext.drawingBufferHeight);

	const ctx = CanvasRenderingContext2D.withView(handle.toNumber(), glview.frame.size.width * scale, glview.frame.size.height * scale, 1, true, 0, 90, 1);
	ctx.fillStyle = 'white';

	// ctx.fillRect(0, 0, 1000, 1000);

	ctx.fillStyle = 'black';

	ctx.translate(0, 100);

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

const NSApp = NSApplication.sharedApplication;

NSApp.delegate = ApplicationDelegate.new();

NSApp.setActivationPolicy(NSApplicationActivationPolicy.Regular);

NSApplicationMain(0, null);
