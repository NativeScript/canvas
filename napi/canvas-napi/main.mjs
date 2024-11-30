import '@nativescript/macos-node-api';
import { CanvasRenderingContext2D, ImageAsset, TextDecoder, TextEncoder, WebGLRenderingContext } from './index.js';

objc.import('AppKit');
objc.import('OpenGL');
objc.import('QuartzCore');

let runLoop = 0;
//
//
// const enc = new TextEncoder();
// const dec = new TextDecoder();
//
// const encoded = enc.encode('Hello Osei');
// console.log('encoded', encoded);
// const decoded = dec.decode(encoded);
// console.log('decoded', decoded, decoded === 'Hello Osei');

class Application {
	/**
	 * @property {AppDelegate} delegate
	 */
	static delegate;

	/**
	 * @property {NSApplication} application
	 */

	static application;
	static rootView;

	/**
	 * @property {Window} window
	 */
	static window;

	/**
	 * @property {NSMenu} appMenu
	 */
	static appMenu;

	/**
	 * @property {boolean} ensure60FPS
	 */
	static ensure60FPS;

	/**
	 * @property {boolean} ensure1200FPS
	 */
	static ensure120FPS;

	/**
	 * @property {boolean} initEditMenu
	 */
	static initEditMenu;

	static launch() {
		// Application.rootView = document.body as unknown as HTMLViewElement;
		// Application.rootView?.connectedCallback();

		const controller = ViewController.new();
		const window = NSWindow.windowWithContentViewController(controller);

		window.title = 'NativeScript for macOS';
		window.styleMask = NSWindowStyleMask.Titled | NSWindowStyleMask.Closable | NSWindowStyleMask.Miniaturizable | NSWindowStyleMask.Resizable | NSWindowStyleMask.FullSizeContentView;

		window.titlebarAppearsTransparent = true;
		window.titleVisibility = NSWindowTitleVisibility.Hidden;

		NativeScriptApplication.window = window;


		window.becomeMainWindow();
		window.displayIfNeeded();
		window.makeKeyAndOrderFront(NSApp);


		Application.application = NSApplication.sharedApplication;
		Application.delegate = ApplicationDelegate.new();
		Application.delegate.window = NativeScriptApplication.window.nativeView;
		Application.createMenu();
		NSApp.delegate = Application.delegate;
		window.delegate = Application.delegate;
		NSApp.setActivationPolicy(NSApplicationActivationPolicy.Regular);
		NSApp.run();
	}

	static createMenu() {
		if (!Application.appMenu) {
			const menu = NSMenu.new();

			const appMenuItem = NSMenuItem.new();
			menu.addItem(appMenuItem);

			// appMenuItem.submenu = menu;

			menu.addItemWithTitleActionKeyEquivalent('Quit', 'terminate:', 'q');

			NSApp.mainMenu = menu;

			Application.appMenu = menu;
		}
	}

	static showMainWindow() {
		// override
	}
}

globalThis.NativeScriptApplication = Application;

function RunLoop() {
	let delay = 2;
	let lastEventTime = 0;

	const loop = () => {
		const event = NSApp.nextEventMatchingMaskUntilDateInModeDequeue(NSEventMask.Any, null, 'kCFRunLoopDefaultMode', true);

		const timeSinceLastEvent = Date.now() - lastEventTime;
		if (event != null) {
			NSApp.sendEvent(event);
			delay = timeSinceLastEvent < 32 ? 2 : 8;
			lastEventTime = Date.now();
		} else {
			delay = timeSinceLastEvent > 6000 ? 128 : timeSinceLastEvent > 4000 ? 64 : timeSinceLastEvent > 2000 ? 16 : 8;
		}

		if (NativeScriptApplication.delegate.running) {
			let timeOut = delay;
			if (NativeScriptApplication.ensure60FPS) {
				timeOut = 8;
			} else if (NativeScriptApplication.ensure120FPS) {
				timeOut = 4;
			}
			runLoop = setTimeout(loop, timeOut);
		}
	};
	runLoop = setTimeout(loop, 0);
}

export class ApplicationDelegate extends NSObject {
	running = true;
	isActive = true;
	static ObjCProtocols = [NSApplicationDelegate, NSWindowDelegate];

	static {
		NativeClass(this);
	}

	/**
	 * @param {NSNotification} _notification
	 */
	applicationDidFinishLaunching(_notification) {
		NSApp.activateIgnoringOtherApps(false);

		NSApp.stop(this);

		RunLoop();

		//doTheThing();

		doGL();
	}

	applicationWillTerminate(_notification) {
		this.running = false;
	}

	applicationShouldHandleReopenHasVisibleWindows(sender, hasVisibleWindows) {
		if (!hasVisibleWindows) {
			sender.windows.firstObject.makeKeyAndOrderFront(sender);
		}
		return true;
	}

	windowWillClose(_notification) {
		NSApp.terminate(this);
		clearTimeout(runLoop);
		runLoop = 0;
		process.exit(0);
	}
}

export class NSCMTLView extends NSView {
	static {
		NativeClass(this);
	}
	_device;
	_queue;
	_canvas;

	get queue() {
		return this._queue;
	}

	get device() {
		return this._device;
	}

	initWithFrame(frameRect) {
		super.initWithFrame(frameRect);
		this.wantsLayer = true;
		const layer = CAMetalLayer.layer();
		this._device = MTLCreateSystemDefaultDevice();
		this._queue = this._device.newCommandQueue();
		layer.device = this._device;
		layer.presentsWithTransaction = false;
		layer.framebufferOnly = false;
		layer.pixelFormat = MTLPixelFormat.BGRA8Unorm;

		this.layer = layer;
		return this;
	}

	/**
	 * @return {CGSize}
	 */
	get drawableSize() {
		return this.layer.drawableSize;
	}

	/**
	 * @param {CGSize} value
	 */
	set drawableSize(value) {
		this.layer.drawableSize = value;
	}

	present() {
		this._canvas?.getContext('2d').flush();
		this._canvas?.getContext('2d').present();
	}

	static ObjCExposedMethods = {
		present: { returns: interop.types.void, params: [] }
	};
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

	initWithFrame(frame) {
		super.initWithFrame(frame);
		this.wantsLayer = true;
		return this;
	}

	prepareOpenGL() {
		super.prepareOpenGL();
	}

	clearGLContext() {
		super.clearGLContext();
	}
}

export class ViewController extends NSViewController {
	static {
		NativeClass(this);
	}

	canvas;

	/**
	 * @param {NSCCanvas} canvas
	 */
	viewDidLoad() {
		super.viewDidLoad();
		this.canvas = NSCCanvas.alloc().initWithFrame(this.view.frame);

		glview.frame = this.view.frame;
		mtlview.frame = this.view.frame;
		mtlview.drawableSize = new CGSize({
			width: this.view.frame.size.width * NSScreen.mainScreen.backingScaleFactor,
			height: this.view.frame.size.height * NSScreen.mainScreen.backingScaleFactor
		});

		// this.canvas.addSubview(mtlview);

		glview.layer.backgroundColor = NSColor.blueColor;

		this.canvas.addSubview(glview);

		this.view.addSubview(this.canvas);

	}
}

const glview = CanvasGLView.alloc().initWithFrame({ x: 0, y: 0, width: 0, height: 0 });

const mtlview = NSCMTLView.alloc().initWithFrame({ x: 0, y: 0, width: 0, height: 0 });

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

function mdnRotate(ctx) {
	// Point of transform origin
	ctx.arc(0, 0, 5, 0, 2 * Math.PI);
	ctx.fillStyle = 'blue';
	ctx.fill();

	// Non-rotated rectangle
	ctx.fillStyle = 'gray';
	ctx.fillRect(100, 0, 80, 20);

	// Rotated rectangle
	ctx.rotate((45 * Math.PI) / 180);
	ctx.fillStyle = 'red';
	ctx.fillRect(100, 0, 80, 20);

	// Reset transformation matrix to the identity matrix
	ctx.setTransform(1, 0, 0, 1, 0, 0);
}

function mdnCreateConicGradient(ctx) {
	// Create a conic gradient
	// The start angle is 0
	// The center position is 100, 100
	const gradient = ctx.createConicGradient(0, 100, 100);

	// Add five color stops
	gradient.addColorStop(0, 'red');
	gradient.addColorStop(0.25, 'orange');
	gradient.addColorStop(0.5, 'yellow');
	gradient.addColorStop(0.75, 'green');
	gradient.addColorStop(1, 'blue');

	// Set the fill style and draw a rectangle
	ctx.fillStyle = gradient;
	ctx.fillRect(20, 20, 200, 200);
}

let requestAnimationFrameFunc;

class CADisplayLinkImpl extends NSObject {
	static ObjCProtocols = [NSApplicationDelegate, NSWindowDelegate];

	static {
		NativeClass(this);
	}

	handleFrame(link) {
		requestAnimationFrameFunc?.(link.timestamp);
		//requestAnimationFrameFunc = null;
	}

	static ObjCExposedMethods = {
		handleFrame: { returns: interop.types.void, params: [CADisplayLink] }
	};
}

const impl = CADisplayLinkImpl.new();

const displayLink = NSScreen.mainScreen.displayLinkWithTargetSelector(impl, 'handleFrame');
displayLink.paused = true;
displayLink.addToRunLoopForMode(NSRunLoop.currentRunLoop, NSDefaultRunLoopMode);

function requestAnimationFrame(func) {
	if (displayLink.paused) {
		displayLink.paused = false;
	}
	requestAnimationFrameFunc = func;
	return 1;
}

function flappyBird(canvas) {
	var ctx,
		width,
		height,
		fgpos = 0,
		frames = 0,
		score = 0,
		best = 0,
		currentstate,
		states = {
			Splash: 0,
			Game: 1,
			Score: 2
		},
		okbtn,
		bird = {
			x: 60,
			y: 0,
			frame: 0,
			animation: [0, 1, 2, 1],
			rotation: 0,
			velocity: 0,
			radius: 12,
			gravity: 0.25,
			_jump: 4.6,

			jump: function() {
				this.velocity = -this._jump;
			},

			update: function() {
				var n = currentstate === states.Splash ? 10 : 5;
				this.frame += frames % n === 0 ? 1 : 0;
				this.frame %= this.animation.length;

				if (currentstate === states.Splash) {
					this.y = height - 280 + 5 * Math.cos(frames / 10);
					this.rotation = 0;
				} else {
					this.velocity += this.gravity;
					this.y += this.velocity;

					if (this.y >= height - s_fg.height - 10) {
						this.y = height - s_fg.height - 10;
						if (currentstate === states.Game) {
							currentstate = states.Score;
						}
						this.velocity = this._jump;
					}

					if (this.velocity >= this._jump) {
						this.frame = 1;
						this.rotation = Math.min(Math.PI / 2, this.rotation + 0.3);
					} else {
						this.rotation = -0.3;
					}
				}
			},

			draw: function(ctx) {
				ctx.save();
				ctx.translate(this.x, this.y);
				ctx.rotate(this.rotation);

				var n = this.animation[this.frame];
				s_bird[n].draw(ctx, -s_bird[n].width / 2, -s_bird[n].height / 2);
				ctx.restore();
			}
		},
		pipes = {
			_pipes: [],

			reset: function() {
				this._pipes = [];
			},

			update: function() {
				if (frames % 100 === 0) {
					var _y = height - (s_pipeSouth.height + s_fg.height + 120 + 200 * Math.random());
					this._pipes.push({
						x: 500,
						y: _y,
						width: s_pipeSouth.width,
						height: s_pipeSouth.height
					});
				}
				for (var i = 0, len = this._pipes.length; i < len; i++) {
					var p = this._pipes[i];

					if (i === 0) {
						score += p.x === bird.x ? 1 : 0;

						var cx = Math.min(Math.max(bird.x, p.x), p.x + p.width);
						var cy1 = Math.min(Math.max(bird.y, p.y), p.y + p.height);
						var cy2 = Math.min(Math.max(bird.y, p.y + p.height + 80), p.y + 2 * p.height + 80);

						var dx = bird.x - cx;
						var dy1 = bird.y - cy1;
						var dy2 = bird.y - cy2;

						var d1 = dx * dx + dy1 * dy1;
						var d2 = dx * dx + dy2 * dy2;

						var r = bird.radius * bird.radius;

						if (r > d1 || r > d2) {
							currentstate = states.Score;
						}
					}

					p.x -= 2;
					if (p.x < -50) {
						this._pipes.splice(i, 1);
						i--;
						len--;
					}
				}
			},

			draw: function(...args) {
				for (var i = 0, len = this._pipes.length; i < len; i++) {
					var p = this._pipes[i];
					s_pipeSouth.draw(ctx, p.x, p.y);
					s_pipeNorth.draw(ctx, p.x, p.y + 80 + p.height);
				}
			}
		},
		img;

	function onpress(evt) {
		switch (currentstate) {
			case states.Splash:
				currentstate = states.Game;
				bird.jump();
				break;
			case states.Game:
				bird.jump();
				break;
			case states.Score:
				var mx = evt.offsetX,
					my = evt.offsetY;

				if (mx == null || my == null) {
					mx = evt.changedTouches[0].clientX;
					my = evt.changedTouches[0].clientY;
				}

				if (okbtn.x < mx && mx < okbtn.x + okbtn.width && okbtn.y < my && my < okbtn.y + okbtn.height) {
					pipes.reset();
					currentstate = states.Splash;
					score = 0;
				}
				break;
		}
	}

	function main() {
		width = canvas.clientWidth; //* window.devicePixelRatio;
		height = canvas.clientHeight; // * window.devicePixelRatio;

		canvas.addEventListener('touchstart', onpress);

		var evt = 'touchstart';
		if (width >= 500) {
			//	width = 320;
			//	height = 480;
			canvas.style.border = '1px solid #000';
			evt = 'mousedown';
		}

		canvas.width = width;
		canvas.height = height;
		ctx = canvas.getContext('2d');

		// ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

		currentstate = states.Splash;
		img = new ImageAsset();
		img.fromUrlSync('https://raw.githubusercontent.com/maxwihlborg/youtube-tutorials/master/flappy/starter/res/sheet.png');

		initSprites(img);
		ctx.fillStyle = s_bg.color;

		okbtn = {
			x: (width - s_buttons.Ok.width) / 2,
			y: height - 200,
			width: s_buttons.Ok.width,
			height: s_buttons.Ok.height
		};
		run();

		/*
		img = document.createElement('img');

		img.onload = (event) => {
			initSprites(img);
			ctx.fillStyle = s_bg.color;

			(okbtn = {
				x: (width - s_buttons.Ok.width) / 2,
				y: height - 200,
				width: s_buttons.Ok.width,
				height: s_buttons.Ok.height,
			}),
				run();
		};
		img.src = 'https://raw.githubusercontent.com/maxwihlborg/youtube-tutorials/master/flappy/starter/res/sheet.png';

		*/
	}

	function run() {
		var loop = function(ts) {
			render();
			update();
			requestAnimationFrame(loop);
		};
		requestAnimationFrame(loop);
	}

	function update() {
		frames++;

		if (currentstate !== states.Score) {
			fgpos = (fgpos - 2) % 14;
		} else {
			best = Math.max(best, score);
		}

		if (currentstate === states.Game) {
			pipes.update();
		}

		bird.update();
	}

	function render() {
		ctx.fillRect(0, 0, width, height);

		s_bg.draw(ctx, 0, height - s_bg.height);
		s_bg.draw(ctx, s_bg.width, height - s_bg.height);

		pipes.draw(ctx);
		bird.draw(ctx);

		s_fg.draw(ctx, fgpos, height - s_fg.height);
		s_fg.draw(ctx, fgpos + s_fg.width, height - s_fg.height);

		var width2 = width / 2;

		if (currentstate === states.Splash) {
			s_splash.draw(ctx, width2 - s_splash.width / 2, height - 300);
			s_text.GetReady.draw(ctx, width2 - s_text.GetReady.width / 2, height - 380);
		}

		if (currentstate === states.Score) {
			s_text.GameOver.draw(ctx, width2 - s_text.GameOver.width / 2, height - 400);
			s_score.draw(ctx, width2 - s_score.width / 2, height - 340);
			s_buttons.Ok.draw(ctx, okbtn.x, okbtn.y);

			s_numberS.draw(ctx, width2 - 47, height - 304, score, null, 10);
			s_numberS.draw(ctx, width2 - 47, height - 262, best, null, 10);
		} else {
			s_numberB.draw(ctx, null, 20, score, width2);
		}

		ctx.render();
	}

	var s_bird, s_bg, s_fg, s_pipeNorth, s_pipeSouth, s_text, s_score, s_splash, s_buttons, s_numberS, s_numberB;

	function Sprite(img, x, y, width, height) {
		this.img = img;
		this.x = x * 2;
		this.y = y * 2;
		this.width = width * 2;
		this.height = height * 2;
	}

	Sprite.prototype.draw = function(ctx, x, y) {
		ctx.drawImage(this.img, this.x, this.y, this.width, this.height, x, y, this.width, this.height);
	};

	function initSprites(img) {
		s_bird = [new Sprite(img, 156, 115, 17, 12), new Sprite(img, 156, 128, 17, 12), new Sprite(img, 156, 141, 17, 12)];

		s_bg = new Sprite(img, 0, 0, 138, 114);
		s_bg.color = '#70C5CF';
		s_fg = new Sprite(img, 138, 0, 112, 56);

		s_pipeNorth = new Sprite(img, 251, 0, 26, 200);
		s_pipeSouth = new Sprite(img, 277, 0, 26, 200);

		s_text = {
			FlappyBird: new Sprite(img, 59, 114, 96, 22),
			GameOver: new Sprite(img, 59, 136, 94, 19),
			GetReady: new Sprite(img, 59, 155, 87, 22)
		};
		s_buttons = {
			Rate: new Sprite(img, 79, 177, 40, 14),
			Menu: new Sprite(img, 119, 177, 40, 14),
			Share: new Sprite(img, 159, 177, 40, 14),
			Score: new Sprite(img, 79, 191, 40, 14),
			Ok: new Sprite(img, 119, 191, 40, 14),
			Start: new Sprite(img, 159, 191, 40, 14)
		};

		s_score = new Sprite(img, 138, 56, 113, 58);
		s_splash = new Sprite(img, 0, 114, 59, 49);

		s_numberS = new Sprite(img, 0, 177, 6, 7);
		s_numberB = new Sprite(img, 0, 188, 7, 10);

		s_numberS.draw = s_numberB.draw = function(ctx, x, y, num, center, offset) {
			num = num.toString();
			var step = this.width + 2;

			if (center) {
				x = center - (num.length * step - 2) / 2;
			}

			if (offset) {
				x += step * (offset - num.length);
			}

			for (var i = 0, len = num.length; i < len; i++) {
				var n = parseInt(num[i]);
				ctx.drawImage(img, step * n, this.y, this.width, this.height, x, y, this.width, this.height);
				x += step;
			}
		};
	}

	main();
}

function solarSystem(canvas) {
	const ctx = canvas.getContext('2d');

	let sun;
	let moon;
	let earth;

	const scale = NSScreen.mainScreen.backingScaleFactor;

	ctx.fillStyle = 'white';
	ctx.fillRect(0, 0, canvas.width, canvas.height);

	ctx.render();

	async function init() {
		// sun = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');
		// moon = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1443/Canvas_moon.png');
		// earth = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1429/Canvas_earth.png');

		sun = new ImageAsset();
		moon = new ImageAsset();
		earth = new ImageAsset();

		await sun.fromUrl('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_sun.png');
		await moon.fromUrl('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_moon.png');
		await earth.fromUrl('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_earth.png');

		// sun.fromUrlSync('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_sun.png');
		// moon.fromUrlSync('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_moon.png');
		// earth.fromUrlSync('https://raw.githubusercontent.com/NativeScript/canvas/refs/heads/feat/macos-napi/napi/canvas-napi/examples/assets/canvas_earth.png');
		LAF = requestAnimationFrame(draw);
	}

	//ctx.scale(scale, scale);

	function draw() {
		ctx.globalCompositeOperation = 'destination-over';
		ctx.clearRect(0, 0, canvas.width, canvas.height);

		ctx.fillStyle = 'rgba(0, 0, 0, 0.4)';
		ctx.strokeStyle = 'rgba(0, 153, 255, 0.4)';
		ctx.save();
		ctx.translate(150, 150);

		// Earth
		let time = new Date();
		ctx.rotate(((2 * Math.PI) / 60) * time.getSeconds() + ((2 * Math.PI) / 60000) * time.getMilliseconds());
		ctx.translate(105, 0);
		ctx.fillRect(0, -12, 40, 24); // Shadow
		ctx.drawImage(earth, -12, -12);

		// Moon
		ctx.save();
		ctx.rotate(((2 * Math.PI) / 6) * time.getSeconds() + ((2 * Math.PI) / 6000) * time.getMilliseconds());
		ctx.translate(0, 28.5);
		ctx.drawImage(moon, -3.5, -3.5);

		ctx.restore();
		ctx.restore();

		// Earth orbit arc
		ctx.beginPath();
		ctx.arc(150, 150, 105, 0, Math.PI * 2, false);
		ctx.stroke();

		//
		ctx.drawImage(sun, 0, 0, 300, 300);

		ctx.render();

		LAF = requestAnimationFrame(draw);
	}

	init();
}

let LAF = 0;

const window = {
	devicePixelRatio: NSScreen.mainScreen.backingScaleFactor
};

const Screen = {
	mainScreen: {
		scale: NSScreen.mainScreen.backingScaleFactor
	}
};

function swarm(canvas, width, height, nativeCanvas) {
	var requestAnimFrame = requestAnimationFrame;

	function init() {
		//canvas.nativeView.setHandleInvalidationManually(true);
		// Initialize the context of the canvas

		// canvas.nativeView.handleInvalidationManually = true

		// Set the canvas width and height to occupy full window
		var W = width || canvas.clientWidth * window.devicePixelRatio,
			H = height || canvas.clientHeight * window.devicePixelRatio;

		canvas.width = W;
		canvas.height = H;

		var ctx = canvas.getContext ? canvas.getContext('2d') : canvas;

		// ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

		// Some variables for later use
		var particleCount = 500,
			particles = [],
			minDist = 50,
			dist;

		// Function to paint the canvas black
		function paintCanvas() {
			// Set the fill color to black
			ctx.fillStyle = 'black';

			// This will create a rectangle of white color from the
			// top left (0,0) to the bottom right corner (W,H)
			//ctx.clearRect(0,0, W,H);
			ctx.fillRect(0, 0, W, H);
		}

		// Now the idea is to create some particles that will attract
		// each other when they come close. We will set a minimum
		// distance for it and also draw a line when they come
		// close to each other.

		// The attraction can be done by increasing their velocity as
		// they reach closer to each other

		// Let's make a function that will act as a class for
		// our particles.

		function Particle() {
			// Position them randomly on the canvas
			// Math.random() generates a random value between 0
			// and 1 so we will need to multiply that with the
			// canvas width and height.
			this.x = Math.random() * W;
			this.y = Math.random() * H;

			// We would also need some velocity for the particles
			// so that they can move freely across the space
			this.vx = -1 + Math.random() * 2;
			this.vy = -1 + Math.random() * 2;

			// Now the radius of the particles. I want all of
			// them to be equal in size so no Math.random() here..
			this.radius = 4;

			// This is the method that will draw the Particle on the
			// canvas. It is using the basic fillStyle, then we start
			// the path and after we use the `arc` function to
			// draw our circle. The `arc` function accepts four
			// parameters in which first two depicts the position
			// of the center point of our arc as x and y coordinates.
			// The third value is for radius, then start angle,
			// end angle and finally a boolean value which decides
			// whether the arc is to be drawn in counter clockwise or
			// in a clockwise direction. False for clockwise.
			this.draw = function() {
				ctx.fillStyle = 'white';
				ctx.beginPath();
				ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);

				// Fill the color to the arc that we just created
				ctx.fill();
			};
		}

		// Time to push the particles into an array
		for (var i = 0; i < particleCount; i++) {
			particles.push(new Particle());
		}

		// Function to draw everything on the canvas that we'll use when
		// animating the whole scene.
		var p = new Particle();

		function draw() {
			// Call the paintCanvas function here so that our canvas
			// will get re-painted in each next frame
			paintCanvas();

			// Call the function that will draw the balls using a loop
			for (var i = 0; i < particles.length; i++) {
				p = particles[i];
				p.draw();
			}

			//Finally call the update function
			update();

			ctx.render();
		}

		// Give every particle some life
		function update() {
			// In this function, we are first going to update every
			// particle's position according to their velocities
			for (var i = 0; i < particles.length; i++) {
				p = particles[i];

				// Change the velocities
				p.x += p.vx;
				p.y += p.vy;

				// We don't want to make the particles leave the
				// area, so just change their position when they
				// touch the walls of the window
				if (p.x + p.radius > W) p.x = p.radius;
				else if (p.x - p.radius < 0) {
					p.x = W - p.radius;
				}

				if (p.y + p.radius > H) p.y = p.radius;
				else if (p.y - p.radius < 0) {
					p.y = H - p.radius;
				}

				// Now we need to make them attract each other
				// so first, we'll check the distance between
				// them and compare it to the minDist we have
				// already set

				// We will need another loop so that each
				// particle can be compared to every other particle
				// except itself
				for (var j = i + 1; j < particles.length; j++) {
					let p2 = particles[j];
					distance(p, p2);
				}
			}
		}

		// Distance calculator between two particles
		function distance(p1, p2) {
			var dist,
				dx = p1.x - p2.x,
				dy = p1.y - p2.y;

			dist = Math.sqrt(dx * dx + dy * dy);

			// Draw the line when distance is smaller
			// then the minimum distance
			if (dist <= minDist) {
				// Draw the line
				ctx.beginPath();
				ctx.strokeStyle = 'rgba(255,255,255,' + (1.2 - dist / minDist) + ')';
				ctx.moveTo(p1.x, p1.y);
				ctx.lineTo(p2.x, p2.y);
				ctx.stroke();
				//ctx.closePath();

				// Some acceleration for the partcles
				// depending upon their distance
				var ax = dx / 2000,
					ay = dy / 2000;

				// Apply the acceleration on the particles
				p1.vx -= ax;
				p1.vy -= ay;

				p2.vx += ax;
				p2.vy += ay;
			}
		}

		// Start the main animation loop using requestAnimFrame
		function animloop() {
			LAF = requestAnimFrame(animloop);
			draw();
		}

		animloop();
	}

	init();
}

function breathe_demo(canvas) {
	const cartesian2Canvas = (v, center) => {
		return { x: v.x + center.x, y: -1 * v.y + center.y };
	};

	const cartesian2Polar = (v) => {
		return { x: Math.atan2(v.y, v.x), y: Math.sqrt(v.x * v.x + v.y * v.y) };
	};

	const polar2Cartesian = (p) => {
		return {
			x: p.radius * Math.cos(p.theta),
			y: p.radius * Math.sin(p.theta)
		};
	};

	const polar2Canvas = (p, center) => {
		return cartesian2Canvas(polar2Cartesian(p), center);
	};

	function breathe(canvas) {
		const context = canvas.getContext('2d');
		const { width, height } = canvas;

		let progress = 0;

		const c1 = '#61bea2';
		const c2 = '#529ca0';

		const mix = (value, x, y) => {
			return x * (1 - value) + y * value;
		};

		const Ring = ({ context, index, progress }) => {
			context.save();
			context.beginPath();
			const R = width / 4;
			const center = { x: width / 2, y: height / 2 - 64 };
			const theta = (index * (2 * Math.PI)) / 6;

			const { x, y } = polar2Canvas({ theta, radius: progress * R }, { x: 0, y: 0 });
			const scale = mix(progress, 0.3, 1);

			context.scale(scale / Screen.mainScreen.scale, scale / Screen.mainScreen.scale);

			//context.translate(x, y);

			context.arc(x, y, R, 0, 2 * Math.PI);

			context.fillStyle = index % 2 ? c1 : c2;
			context.fill();
			context.restore();
		};

		const center = { x: width / 2, y: height / 2 - 64 };

		const start = Date.now();
		const duration = 5000; // 3000ms or 3 seconds

		function breathingEase(t) {
			if (t <= 0.5) {
				// Inhale (first half of the cycle)
				return 0.5 - 0.5 * Math.cos(Math.PI * t * 2);
			} else {
				// Exhale (second half of the cycle)
				return 0.5 + 0.5 * Math.cos(Math.PI * (t - 0.5) * 2);
			}
		}

		const tick = () => {
			const elapsed = (Date.now() - start) % duration;
			const rawProgress = elapsed / duration; // Normalize progress to range [0, 1]
			progress = breathingEase(rawProgress); // Apply bounce easing to progress

			context.save();
			context.clearRect(0, 0, width, height);
			context.fillStyle = 'rgb(36,43,56)';
			context.fillRect(0, 0, width, height);
			context.filter = 'blur(1px)';
			context.globalCompositeOperation = 'screen';
			context.translate(center.x, center.y);

			// Rotate and draw rings
			context.rotate(mix(progress, -Math.PI, 0));
			for (let i = 0; i < 6; i++) {
				Ring({ context: context, index: i, progress });
			}

			context.restore();

			context.render();

			// Loop the animation
			requestAnimationFrame(tick);
		};

		tick();
	}

	breathe(canvas);
}

function mdnPattern(canvas) {
	const ctx = canvas.getContext('2d');
	const img = new ImageAsset();
	img.fromUrlSync('https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern/canvas_create_pattern.png');
	const pattern = ctx.createPattern(img, 'repeat');
	ctx.fillStyle = 'white';
	ctx.fillRect(0, 0, canvas.width, canvas.height);
	ctx.fillStyle = pattern;
	ctx.fillRect(0, 0, canvas.width, canvas.height);
}

function mdnPutImageData(canvas) {
	const ctx = canvas.getContext('2d');

	function putImageData(ctx, imageData, dx, dy, dirtyX, dirtyY, dirtyWidth, dirtyHeight) {
		const data = imageData.data;
		const height = imageData.height;
		const width = imageData.width;
		dirtyX = dirtyX || 0;
		dirtyY = dirtyY || 0;
		dirtyWidth = dirtyWidth !== undefined ? dirtyWidth : width;
		dirtyHeight = dirtyHeight !== undefined ? dirtyHeight : height;
		const limitBottom = dirtyY + dirtyHeight;
		const limitRight = dirtyX + dirtyWidth;
		for (let y = dirtyY; y < limitBottom; y++) {
			for (let x = dirtyX; x < limitRight; x++) {
				const pos = y * width + x;
				ctx.fillStyle = `rgb(${data[pos * 4 + 0]} ${data[pos * 4 + 1]}
      ${data[pos * 4 + 2]} / ${data[pos * 4 + 3] / 255})`;
				ctx.fillRect(x + dx, y + dy, 1, 1);
			}
		}
	}

	// Draw content onto the canvas
	ctx.fillRect(0, 0, 100, 100);
	// Create an ImageData object from it
	const imagedata = ctx.getImageData(0, 0, 100, 100);
	// use the putImageData function that illustrates how putImageData works
	putImageData(ctx, imagedata, 150, 0, 50, 50, 25, 25);
	ctx.render();
}

function mdnRadialGradient(canvas) {
	const ctx = canvas.getContext('2d');

	// Create a radial gradient
	// The inner circle is at x=110, y=90, with radius=30
	// The outer circle is at x=100, y=100, with radius=70
	const gradient = ctx.createRadialGradient(110, 90, 30, 100, 100, 70);

	// Add three color stops
	gradient.addColorStop(0, 'pink');
	gradient.addColorStop(0.9, 'white');
	gradient.addColorStop(1, 'green');

	// Set the fill style and draw a rectangle
	ctx.fillStyle = gradient;
	ctx.fillRect(20, 20, 160, 160);
}

function cancelSwarm() {
	// cancelAnimationFrame(LAF);
	// LAF = 0;
}

function webglTextures(canvas) {
	const vertexShaderSrc = `
// #version 120
precision highp float;
attribute vec2 position;
void main() {
  gl_Position = vec4(position, 0.0, 1.0);
  gl_PointSize = 128.0;
}`;

	const fragmentShaderSrc = `
// #version 120
precision mediump float;
void main() {
  vec2 fragmentPosition = 2.0*gl_PointCoord - 1.0;
  float distance = length(fragmentPosition);
  float distanceSqrd = distance * distance;
  gl_FragColor = vec4(
    0.2/distanceSqrd,
    0.1/distanceSqrd,
    0.0, 1.0 );
}`;

	var gl, program;

	function textures(canvas) {
		function setupWebGL() {
			if (!(gl = getRenderingContext())) return;
			var source = vertexShaderSrc;
			var vertexShader = gl.createShader(gl.VERTEX_SHADER);
			gl.shaderSource(vertexShader, source);
			gl.compileShader(vertexShader);
			source = fragmentShaderSrc;
			var fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
			gl.shaderSource(fragmentShader, source);
			gl.compileShader(fragmentShader);
			program = gl.createProgram();

			gl.attachShader(program, vertexShader);
			gl.attachShader(program, fragmentShader);
			gl.linkProgram(program);
			gl.detachShader(program, vertexShader);
			gl.detachShader(program, fragmentShader);
			gl.deleteShader(vertexShader);
			gl.deleteShader(fragmentShader);
			if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
				var linkErrLog = gl.getProgramInfoLog(program);
				cleanup();
				console.log('Shader program did not link successfully. ' + 'Error log: ' + linkErrLog);
				return;
			}
			console.log('1', gl.getError());
			initializeAttributes();
			console.log('2', gl.getError());
			gl.useProgram(program);
			console.log('3', gl.getError());
			gl.drawArrays(gl.POINTS, 0, 1);
			gl.render();

			cleanup();
		}

		var buffer;

		function initializeAttributes() {
			gl.enableVertexAttribArray(0);
			console.log('1.1', gl.getError());
			buffer = gl.createBuffer();
			console.log('1.2', gl.getError());
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			console.log('1.3', gl.getError());
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0]), gl.STATIC_DRAW);
			console.log('1.4', gl.getError());
			gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
			console.log('1.5', gl.getError());
		}

		function cleanup() {
			gl.useProgram(null);
			if (buffer) gl.deleteBuffer(buffer);
			if (program) gl.deleteProgram(program);
		}

		function getRenderingContext() {
			var gl = canvas.getContext('webgl2') || canvas.getContext('experimental-webgl');
			if (!gl) {
				console.log('Failed to get WebGL context.' + 'Your device may not support WebGL.');
				return null;
			}
			gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
			gl.clearColor(0, 0, 0, 1.0);
			gl.clear(gl.COLOR_BUFFER_BIT);
			return gl;
		}

		setupWebGL();
	}

	textures(canvas);

}

function createChaosLines(canvas) {

	function createShader(gl, type, source) {
		var shader = gl.createShader(type);
		gl.shaderSource(shader, source);
		gl.compileShader(shader);

		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			console.log('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
			gl.deleteShader(shader);
			return null;
		}

		return shader;
	}

	function initWebGL(canvas) {
		//	canvas.width = canvas.clientWidth;
		//	canvas.height = canvas.clientHeight;
		const gl = canvas.getContext('webgl2');

		if (!gl) {
			alert('Unable to initialize WebGL. Your browser may not support it.');
			return;
		}

		const vertexShaderSource = `
		#version 330 core
    attribute vec4 aVertexPosition;
    void main() {
        gl_Position = aVertexPosition;
    }`;

		const fragmentShaderSource = `
		#version 330 core
    precision mediump float;

    uniform float u_time;
    uniform vec2 u_resolution;

    float hash(float n) {
        return fract(sin(n) * 753.5453123);
    }

    float noise(vec2 p) {
        vec2 i = floor(p);
        vec2 f = fract(p);
        f = f * f * (3.0 - 2.0 * f);
        float n = i.x + i.y * 157.0;
        return mix(mix(hash(n + 0.0), hash(n + 1.0), f.x),
                   mix(hash(n + 157.0), hash(n + 158.0), f.x), f.y);
    }

    float fbm(vec2 p, vec3 a) {
        float v = 0.0;
        v += noise(p * a.x) * 0.50;
        v += noise(p * a.y) * 1.50;
        v += noise(p * a.z) * 0.125 * 0.1;
        return v;
    }

    vec3 drawLines(vec2 uv, vec3 fbmOffset, vec3 color1, vec3 colorSet[4], float secs) {
        float timeVal = secs * 0.05;
        vec3 finalColor = vec3(0.0);
        for (int i = 0; i < 4; ++i) {
            float indexAsFloat = float(i);
            float amp = 80.0 + indexAsFloat * 0.0;
            float period = 2.0 + indexAsFloat + 2.0;
            float thickness = mix(0.4, 0.2, noise(uv * 2.0));
            float t = abs(1.0 / (sin(uv.y + fbm(uv + timeVal * period, fbmOffset)) * amp) * thickness);
            finalColor += t * colorSet[i];
        }

        for (int i = 0; i < 4; ++i) {
            float indexAsFloat = float(i);
            float amp = 40.0 + indexAsFloat * 5.0;
            float period = 9.0 + indexAsFloat + 2.0;
            float thickness = mix(0.1, 0.1, noise(uv * 12.0));
            float t = abs(1.0 / (sin(uv.y + fbm(uv + timeVal * period, fbmOffset)) * amp) * thickness);
            finalColor += t * colorSet[i] * color1;
        }
        return finalColor;
    }

    void main() {
        vec2 uv = (gl_FragCoord.xy / u_resolution) * 0.75 - 2.0;
        uv.x *= u_resolution.x / u_resolution.y;

        vec3 lineColor1 = vec3(1.0, 0.0, 0.5);
        vec3 lineColor2 = vec3(0.3, 0.5, 1.5);

        float spread = 1.0;
        vec3 finalColor = vec3(0.0);
        vec3 colorSet[4];
        colorSet[0] = vec3(0.7, 0.05, 1.0);
        colorSet[1] = vec3(1.0, 0.19, 0.0);
        colorSet[2] = vec3(0.0, 1.0, 0.3);
        colorSet[3] = vec3(0.0, 0.38, 1.0);
        float t = sin(u_time) * 0.5 + 0.5;
        float pulse = mix(0.05, 0.20, t);

        finalColor = drawLines(uv, vec3(65.2, 40.0, 4.0), lineColor1, colorSet, u_time * 0.1) * pulse;
        finalColor += drawLines(uv, vec3(5.0 * spread / 2.0, 2.1 * spread, 1.0), lineColor2, colorSet, u_time);

        gl_FragColor = vec4(finalColor, 1.0);
    }
`;

		const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
		const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);
		if (!vertexShader || !fragmentShader) {
			console.error('Shader creation failed.');
			return;
		}

		const shaderProgram = gl.createProgram();
		gl.attachShader(shaderProgram, vertexShader);
		gl.attachShader(shaderProgram, fragmentShader);
		gl.linkProgram(shaderProgram);

		if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
			console.error('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
			return;
		}

		const positionBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
		console.log('positionBuffer', positionBuffer, gl.getError());
		const positions = [1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0];
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);
		console.log('bufferData', positionBuffer, gl.getError());

		const vertexPosition = gl.getAttribLocation(shaderProgram, 'aVertexPosition');
		console.log('vertexPosition', vertexPosition, gl.getError());
		gl.vertexAttribPointer(vertexPosition, 2, gl.FLOAT, false, 0, 0);
		gl.enableVertexAttribArray(vertexPosition);

		gl.useProgram(shaderProgram);
		const u_time = gl.getUniformLocation(shaderProgram, 'u_time');
		const u_resolution = gl.getUniformLocation(shaderProgram, 'u_resolution');

		console.log('u_time', u_time, 'u_resolution', u_resolution, gl.getError());

		const width = canvas.width;
		const height = canvas.height;
		let start = 0;

		function drawScene(now) {
			if (start === 0) {
				start = now;
			}
			// now *= 0.001;
			now = (now - start) * 0.0001;
			//	requestAnimationFrame(drawScene);
			console.log('1', gl.getError());
			gl.uniform1f(u_time, now);
			console.log('2', gl.getError());
			gl.uniform2f(u_resolution, width, height);
			console.log('3', gl.getError());
			gl.clearColor(0.0, 0.0, 0.0, 1.0);
			console.log('4', gl.getError());
			gl.clear(gl.COLOR_BUFFER_BIT);
			console.log('5', gl.getError());
			gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
			console.log('6', gl.getError());
			gl.vertexAttribPointer(vertexPosition, 2, gl.FLOAT, false, 0, 0);
			console.log('7', gl.getError());
			gl.enableVertexAttribArray(vertexPosition);
			console.log('8', gl.getError());
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
			console.log('9', gl.getError());
			gl.render();
			console.log(gl.getError());
		}

		drawScene();

		//requestAnimationFrame(drawScene);
	}

	initWebGL(canvas);
}


function cubeRotation(canvas) {
	let LAF = 0;

	function rotation(canvas) {
		var gl = canvas.getContext('webgl2');
		var width = gl.drawingBufferWidth;
		var height = gl.drawingBufferHeight;
		/*============ Defining and storing the geometry =========*/

		var vertices = [-1, -1, -1, 1, -1, -1, 1, 1, -1, -1, 1, -1, -1, -1, 1, 1, -1, 1, 1, 1, 1, -1, 1, 1, -1, -1, -1, -1, 1, -1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, 1, -1, 1, 1, 1, 1, -1, 1, -1, -1, -1, -1, -1, 1, 1, -1, 1, 1, -1, -1, -1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1];

		var colors = [5, 3, 7, 5, 3, 7, 5, 3, 7, 5, 3, 7, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0];

		var indices = [0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23];

		// Create and store data into vertex buffer
		var vertex_buffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);

		// Create and store data into color buffer
		var color_buffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, color_buffer);
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);

		// Create and store data into index buffer
		var index_buffer = gl.createBuffer();
		gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer);
		gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW);

		/*=================== Shaders =========================*/

		var vertCode =
			'#version 330 core\n' +
			'in vec3 position;' +
			'uniform mat4 Pmatrix;' +
			'uniform mat4 Vmatrix;' +
			'uniform mat4 Mmatrix;' +
			'in vec3 color;' + //the color of the point
			'out vec3 vColor;' +
			'void main(void) { ' + //pre-built function
			'gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.0);' +
			'vColor = color;' +
			'}';

		var fragCode =
			'#version 330 core\n' +
			'precision mediump float;' +
			'out vec3 vColor;' +
			'out vec4 fragColor;' +
			'void main(void) {' + 'fragColor = vec4(vColor, 1.0);' + '}';

		var vertShader = gl.createShader(gl.VERTEX_SHADER);
		gl.shaderSource(vertShader, vertCode);
		gl.compileShader(vertShader);

		let compiled = gl.getShaderParameter(vertShader, gl.COMPILE_STATUS);
		if (!compiled) {
			// Something went wrong during compilation; get the error
			const lastError = gl.getShaderInfoLog(vertShader);
			console.log('*** Error compiling shader \'' + vertShader + '\':' + lastError);
			gl.deleteShader(vertShader);
			return null;
		}


		var fragShader = gl.createShader(gl.FRAGMENT_SHADER);
		gl.shaderSource(fragShader, fragCode);
		gl.compileShader(fragShader);


		compiled = gl.getShaderParameter(fragShader, gl.COMPILE_STATUS);
		if (!compiled) {
			// Something went wrong during compilation; get the error
			const lastError = gl.getShaderInfoLog(fragShader);
			console.log('*** Error compiling shader \'' + fragShader + '\':' + lastError);
			gl.deleteShader(fragShader);
			return null;
		}


		var shaderProgram = gl.createProgram();
		gl.attachShader(shaderProgram, vertShader);
		gl.attachShader(shaderProgram, fragShader);
		gl.linkProgram(shaderProgram);

		if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
			var linkErrLog = gl.getProgramInfoLog(shaderProgram);
			console.log('Shader program did not link successfully. ' + 'Error log: ' + linkErrLog);
			return;
		}


		/* ====== Associating attributes to vertex shader =====*/
		var Pmatrix = gl.getUniformLocation(shaderProgram, 'Pmatrix');
		var Vmatrix = gl.getUniformLocation(shaderProgram, 'Vmatrix');
		var Mmatrix = gl.getUniformLocation(shaderProgram, 'Mmatrix');


		gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
		var position = gl.getAttribLocation(shaderProgram, 'position');
		gl.vertexAttribPointer(position, 3, gl.FLOAT, false, 0, 0);

		// Position
		gl.enableVertexAttribArray(position);
		gl.bindBuffer(gl.ARRAY_BUFFER, color_buffer);
		var color = gl.getAttribLocation(shaderProgram, 'color');
		gl.vertexAttribPointer(color, 3, gl.FLOAT, false, 0, 0);

		// Color
		gl.enableVertexAttribArray(color);
		gl.useProgram(shaderProgram);

		/*==================== MATRIX =====================*/

		function get_projection(angle, a, zMin, zMax) {
			var ang = Math.tan((angle * 0.5 * Math.PI) / 180); //angle*.5
			return [0.5 / ang, 0, 0, 0, 0, (0.5 * a) / ang, 0, 0, 0, 0, -(zMax + zMin) / (zMax - zMin), -1, 0, 0, (-2 * zMax * zMin) / (zMax - zMin), 0];
		}

		var proj_matrix = get_projection(40, width / height, 1, 100);

		var mov_matrix = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
		var view_matrix = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];

		// translating z
		view_matrix[14] = view_matrix[14] - 6; //zoom

		/*==================== Rotation ====================*/

		function rotateZ(m, angle) {
			var c = Math.cos(angle);
			var s = Math.sin(angle);
			var mv0 = m[0],
				mv4 = m[4],
				mv8 = m[8];

			m[0] = c * m[0] - s * m[1];
			m[4] = c * m[4] - s * m[5];
			m[8] = c * m[8] - s * m[9];

			m[1] = c * m[1] + s * mv0;
			m[5] = c * m[5] + s * mv4;
			m[9] = c * m[9] + s * mv8;
		}

		function rotateX(m, angle) {
			var c = Math.cos(angle);
			var s = Math.sin(angle);
			var mv1 = m[1],
				mv5 = m[5],
				mv9 = m[9];

			m[1] = m[1] * c - m[2] * s;
			m[5] = m[5] * c - m[6] * s;
			m[9] = m[9] * c - m[10] * s;

			m[2] = m[2] * c + mv1 * s;
			m[6] = m[6] * c + mv5 * s;
			m[10] = m[10] * c + mv9 * s;
		}

		function rotateY(m, angle) {
			var c = Math.cos(angle);
			var s = Math.sin(angle);
			var mv0 = m[0],
				mv4 = m[4],
				mv8 = m[8];

			m[0] = c * m[0] + s * m[2];
			m[4] = c * m[4] + s * m[6];
			m[8] = c * m[8] + s * m[10];

			m[2] = c * m[2] - s * mv0;
			m[6] = c * m[6] - s * mv4;
			m[10] = c * m[10] - s * mv8;
		}

		/*================= Drawing ===========================*/
		var time_old = 0;

		var animate = function(time) {
			var dt = time - time_old;
			rotateZ(mov_matrix, dt * 0.005); //time
			rotateY(mov_matrix, dt * 0.002);
			rotateX(mov_matrix, dt * 0.003);
			time_old = time;
			gl.clearColor(0.5, 0.5, 0.5, 0.9);
			gl.clearDepth(1.0);
			gl.enable(gl.DEPTH_TEST);
			gl.depthFunc(gl.LEQUAL);
			gl.viewport(0, 0, width, height);
			gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
			gl.uniformMatrix4fv(Pmatrix, false, proj_matrix);
			gl.uniformMatrix4fv(Vmatrix, false, view_matrix);
			gl.uniformMatrix4fv(Mmatrix, false, mov_matrix);
			gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer);
			gl.drawElements(gl.TRIANGLES, indices.length, gl.UNSIGNED_SHORT, 0);
			LAF = requestAnimationFrame(animate);
		};
		animate(0);
	}

	function cancelCubeRotation() {
		cancelAnimationFrame(LAF);
		LAF = 0;
	}

	rotation(canvas);

}

function doGL() {
	let loaded = false;
	// console.time('load1');
	// loaded = asset.fromUrlSync('https://www.superherotoystore.com/cdn/shop/articles/Website_Blog_creatives_29_1600x.jpg?v=1713945144');
	// console.timeEnd('load1');

	const scale = NSScreen.mainScreen.backingScaleFactor;

	const handle = interop.handleof(glview);
	const glContext = WebGLRenderingContext.withView(handle.toNumber(), 2, true, false, false, false, 1, true, false, false, false, false, false);
	// console.log(glContext, 'drawingBufferWidth', glContext.drawingBufferWidth, 'drawingBufferHeight', glContext.drawingBufferHeight);

	const glCanvas = {
		width: glview.frame.size.width,
		height: glview.frame.size.height,
		clientWidth: glview.frame.size.width,
		clientHeight: glview.frame.size.height,
		addEventListener: function() {
		},
		getContext: function() {
			return glContext;
		},
		style: {}
	};

	glview._canvas = glCanvas;


	//webglTextures(glCanvas);
	//createChaosLines(glCanvas);
	cubeRotation(glCanvas);

}

function doTheThing() {
	let loaded = false;
	// console.time('load1');
	// loaded = asset.fromUrlSync('https://www.superherotoystore.com/cdn/shop/articles/Website_Blog_creatives_29_1600x.jpg?v=1713945144');
	// console.timeEnd('load1');

	const scale = NSScreen.mainScreen.backingScaleFactor;

	console.log('doTheThing');

	const gl = WebGLRenderingContext.offscreen(600, 300, 1, true, false, false, false, 1, true, false, false, false, false, false);

	console.log('gl', gl);

	console.log(gl.getContextAttributes());


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

	//const ctx = CanvasRenderingContext2D.withView(handle.toNumber(), glview.frame.size.width * scale, glview.frame.size.height * scale, 1, true, 0, 90, 1);

	const mtlViewHandle = interop.handleof(mtlview);
	const deviceHandle = interop.handleof(mtlview.device);
	const queueHandle = interop.handleof(mtlview.queue);
	const ctx = CanvasRenderingContext2D.withMtlViewDeviceQueue(mtlViewHandle.toNumber(), deviceHandle.toNumber(), queueHandle.toNumber(), true, scale, 1, 0, 90, 1);

	const mtlCanvas = {
		width: mtlview.frame.size.width,
		height: mtlview.frame.size.height,
		clientWidth: mtlview.frame.size.width,
		clientHeight: mtlview.frame.size.height,
		addEventListener: function() {
		},
		getContext: function() {
			return ctx;
		},
		style: {}
	};

	mtlview._canvas = mtlCanvas;

	ctx.fillStyle = 'white';

	ctx.fillRect(0, 0, 1000, 1000);

	ctx.fillStyle = 'black';

	ctx.scale(scale, scale);

	const encoder = new TextEncoder();

	const decoder = new TextDecoder();

	//	ctx.translate(0, 100);

	//mdnShadowColor(ctx);
	//mdnRotate(ctx);
	// mdnCreateConicGradient(ctx);

	const canvas = {
		width: glview.frame.size.width,
		height: glview.frame.size.height,
		clientWidth: glview.frame.size.width,
		clientHeight: glview.frame.size.height,
		addEventListener: function() {
		},
		getContext: function() {
			return ctx;
		},
		style: {}
	};

	//flappyBird(mtlCanvas);

	solarSystem(canvas);

	// swarm(canvas);
	// breathe_demo(canvas);
	//mdnPattern(canvas);
	// mdnRadialGradient(canvas);
	//mdnPutImageData(canvas);

	//
	// ctx.font = '50px serif';
	// ctx.strokeText('Hello world', 50, 90);

	// ctx.render();

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

Application.launch();

// const NSApp = NSApplication.sharedApplication;
