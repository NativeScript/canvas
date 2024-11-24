import '@nativescript/macos-node-api';
import { CanvasRenderingContext2D, ImageAsset, Path2D } from './index.js';

objc.import('OpenGL');
objc.import('QuartzCore');

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

	canvas;

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

let requestAnimationFrameFunc;
class CADisplayLinkImpl extends NSObject {
	static ObjCProtocols = [NSApplicationDelegate, NSWindowDelegate];

	static {
		NativeClass(this);
	}

	handleFrame(link) {
		requestAnimationFrameFunc?.(link.timestamp);
		requestAnimationFrameFunc = null;
	}

	static ObjCExposedMethods = {
		handleFrame: { returns: interop.types.void, params: [CADisplayLink] },
	};
}

const impl = CADisplayLinkImpl.new();

const displayLink = NSScreen.mainScreen.displayLinkWithTargetSelector(impl, 'handleFrame');
displayLink.paused = true;
displayLink.addToRunLoopForMode(NSRunLoop.currentRunLoop, NSDefaultRunLoopMode);
// displayLink.addToRunLoopForMode(NSRunLoop.currentRunLoop, UITrackingRunLoopMode);

function requestAnimationFrame(func) {
	if (displayLink.paused) {
		displayLink.paused = true;
	}
	requestAnimationFrameFunc = func;
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
			Score: 2,
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

			jump: function () {
				this.velocity = -this._jump;
			},

			update: function () {
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

			draw: function (ctx) {
				ctx.save();
				ctx.translate(this.x, this.y);
				ctx.rotate(this.rotation);

				var n = this.animation[this.frame];
				s_bird[n].draw(ctx, -s_bird[n].width / 2, -s_bird[n].height / 2);
				ctx.restore();
			},
		},
		pipes = {
			_pipes: [],

			reset: function () {
				this._pipes = [];
			},

			update: function () {
				if (frames % 100 === 0) {
					var _y = height - (s_pipeSouth.height + s_fg.height + 120 + 200 * Math.random());
					this._pipes.push({
						x: 500,
						y: _y,
						width: s_pipeSouth.width,
						height: s_pipeSouth.height,
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

			draw: function (...args) {
				for (var i = 0, len = this._pipes.length; i < len; i++) {
					var p = this._pipes[i];
					s_pipeSouth.draw(ctx, p.x, p.y);
					s_pipeNorth.draw(ctx, p.x, p.y + 80 + p.height);
				}
			},
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
		width = canvas.clientWidth;
		height = canvas.clientHeight;

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

		//ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

		currentstate = states.Splash;
		img = new ImageAsset();
		img.fromUrlSync('https://raw.githubusercontent.com/maxwihlborg/youtube-tutorials/master/flappy/starter/res/sheet.png');

		initSprites(img);
		ctx.fillStyle = s_bg.color;

		(okbtn = {
			x: (width - s_buttons.Ok.width) / 2,
			y: height - 200,
			width: s_buttons.Ok.width,
			height: s_buttons.Ok.height,
		}),
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
		var loop = function (ts) {
			requestAnimationFrame(loop);
			try {
				update();
				render();
				console.log('ts', ts);
			} catch (error) {
				console.log('error', error);
			}
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
	}

	var s_bird, s_bg, s_fg, s_pipeNorth, s_pipeSouth, s_text, s_score, s_splash, s_buttons, s_numberS, s_numberB;

	function Sprite(img, x, y, width, height) {
		this.img = img;
		this.x = x * 2;
		this.y = y * 2;
		this.width = width * 2;
		this.height = height * 2;
	}
	Sprite.prototype.draw = function (ctx, x, y) {
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
			GetReady: new Sprite(img, 59, 155, 87, 22),
		};
		s_buttons = {
			Rate: new Sprite(img, 79, 177, 40, 14),
			Menu: new Sprite(img, 119, 177, 40, 14),
			Share: new Sprite(img, 159, 177, 40, 14),
			Score: new Sprite(img, 79, 191, 40, 14),
			Ok: new Sprite(img, 119, 191, 40, 14),
			Start: new Sprite(img, 159, 191, 40, 14),
		};

		s_score = new Sprite(img, 138, 56, 113, 58);
		s_splash = new Sprite(img, 0, 114, 59, 49);

		s_numberS = new Sprite(img, 0, 177, 6, 7);
		s_numberB = new Sprite(img, 0, 188, 7, 10);

		s_numberS.draw = s_numberB.draw = function (ctx, x, y, num, center, offset) {
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

function doTheThing() {
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

	ctx.fillRect(0, 0, 1000, 1000);

	ctx.fillStyle = 'black';

	ctx.translate(0, 100);

	//mdnShadowColor(ctx);
	//mdnRotate(ctx);

	flappyBird({
		clientWidth: glview.frame.size.width * scale,
		clientHeight: 480,
		addEventListener: function () {},
		getContext: function () {
			return ctx;
		},
		style: {},
	});

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
