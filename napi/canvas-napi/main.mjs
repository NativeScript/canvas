import '@nativescript/macos-node-api';
import { CanvasRenderingContext2D, ImageAsset, Path2D } from './index.js';

objc.import('OpenGL');

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

		doTheThing();

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

		console.log('viewDidLoad');

	}
}

const glview = CanvasGLView.alloc().initWithFrame({ x: 0, y: 0, width: 0, height: 0 });

let isDoingOrDone = false;

async function doTheThing() {

	if (isDoingOrDone) {
		return;
	}

	isDoingOrDone = true;

	const asset = new ImageAsset();

	let loaded = false;
	// console.time('load1');
	// loaded = asset.fromUrlSync('https://www.superherotoystore.com/cdn/shop/articles/Website_Blog_creatives_29_1600x.jpg?v=1713945144');
	// console.timeEnd('load1');
	console.log('asset: loaded', loaded, asset.width, asset.height);

	const scale = NSScreen.mainScreen.backingScaleFactor;


	// glview.prepareOpenGL();

	console.log(scale);


	const path = new Path2D();
	path.roundRect(10, 10, 100, 100, [10, 10, 10, 10]);

	let ctx = CanvasRenderingContext2D.withCpu(300, 150, 1, true, 0, 90, 1);
	ctx.fillRect(0, 0, 300, 150);
	console.log('ctx', ctx.toDataURL('image/png'));
	console.log(ctx.font);
	ctx.font = '100px serif';
	console.log(ctx.font);

	ctx.fillStyle = 'red';
	ctx.fillRect(0, 0, 100, 150);
	ctx.fillStyle = 'blue';
	ctx.fillRect(100, 0, 100, 150);
	ctx.fillStyle = 'green';
	ctx.fillRect(200, 0, 100, 150);


	ctx.fillStyle = 'purple';
	ctx.fill(path);


	console.log('ctx', ctx.toDataURL('image/png'));

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

	ctx = CanvasRenderingContext2D.withView(handle.toNumber(), glview.frame.size.width * scale, glview.frame.size.height * scale, 1, true, 0, 90, 1);


	ctx.fillRect(0, 0, 300, 150);
	console.log('ctx', ctx.toDataURL('image/png'));
	console.log(ctx.font);
	ctx.font = '100px serif';
	console.log(ctx.font);

	ctx.fillStyle = 'red';
	ctx.fillRect(0, 0, 100, 150);
	ctx.fillStyle = 'blue';
	ctx.fillRect(100, 0, 100, 150);
	ctx.fillStyle = 'green';
	ctx.fillRect(200, 0, 100, 150);


	ctx.fillStyle = 'purple';
	// ctx.fill(path);
	console.log('osei');

	// asset.loadUrlCallback('https://picsum.photos/id/1/200/300', (done) => {
	// 	console.log('loadUrlCallback', done);
	// });

	// console.log(asset.width, asset.height);


	try {
		loaded = asset.fromUrlSync('https://www.superherotoystore.com/cdn/shop/articles/Website_Blog_creatives_29_1600x.jpg?v=1713945144');
		console.log('picsum');
	} catch (e) {
		console.log('picsum: e', e);
	}


	// ctx.render();

	ctx.drawImage(asset, 0, 0, glview.frame.size.width * scale, glview.frame.size.height * scale);

	ctx.render();


	console.log('??');


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

NSApp.finishLaunching();

NSApp.run();

// setTimeout(()=>{
// 	NSApplicationMain(0, null);
// }, 5000)


