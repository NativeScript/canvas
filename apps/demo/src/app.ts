//require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';

const orginalEncoder = global.TextEncoder;
const orginalDecoder = global.TextDecoder;

import { Canvas } from '@nativescript/canvas';
import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource } from '@nativescript/core';
declare var __non_webpack_require__, __initAppConfiguration, _createImageBitmap;

let direction = 0;

if (androidx.core.text.TextUtilsCompat.getLayoutDirectionFromLocale(java.util.Locale.getDefault()) === androidx.core.view.ViewCompat.LAYOUT_DIRECTION_RTL) {
	direction = 1;
}

const ppi = (Utils.ad.getApplicationContext() as android.content.Context).getResources().getDisplayMetrics().density * 160;
// try {

// __non_webpack_require__('system_lib://libcanvasnativev8.so');

__non_webpack_require__('system_lib://libcanvasnativev8.so');

const module = global.CanvasJSIModule;

try {
	const matrix = module.DOMMatrix();
	console.dir(matrix);

	console.time('Path2D');
	const path = module.Path2D() as Path2D;
	console.timeEnd('Path2D');

	console.time('Path2D:loop');
	const rect = path.rect;
	for (let i = 0; i < 1000; i++) {
		rect(0, 0, 100, 100);
	}
	console.timeEnd('Path2D:loop');

	const encoder = module.TextEncoder();

	console.time('encode:1');
	const encode = encoder.encode;
	const encoded = encode('Hi Osei!!!');
	console.timeEnd('encode:1');

	console.time('encode:2');
	encode('Hi Osei!!!');
	console.timeEnd('encode:2');

	console.log(encoded);

	console.log(typeof encoded);

	const decoder = module.TextDecoder();

	console.time('decode:1');
	const decode = decoder.decode;
	const decoded = decode(encoded);
	console.timeEnd('decode:1');

	console.time('decode:2');
	decode(encoded);
	console.timeEnd('decode:2');

	console.log(decoded);
} catch (error) {
	console.log(error);
}

//__initAppConfiguration({ appBase: knownFolders.currentApp().path });

const handlePath = function (path) {
	if (typeof path === 'string' && path.startsWith('~/')) {
		return nsPath.join(knownFolders.currentApp().path, path.replace('~/', ''));
	}
	return path;
};

//java.lang.System.loadLibrary('canvasnativev8');

/*

global.WebGLRenderingContext.prototype.getSupportedExtensions = function () {
	const string = this.__getSupportedExtensions();
	if (!string) {
		return null;
	}
	return string.split(',');
};

const parent = global.WebGL2RenderingContext.prototype.uniformMatrix4fv;
global.WebGL2RenderingContext.prototype.uniformMatrix4fv = function (a, b, c) {
	if (Array.isArray(c)) {
		parent.call(this, a, b, Float32Array.from(c));
		return;
	}
	parent.call(this, a, b, c);
};

const parentUniform4fv = global.WebGL2RenderingContext.prototype.uniform4fv;
global.WebGL2RenderingContext.prototype.uniform4fv = function (a, b) {
	if (Array.isArray(b)) {
		parentUniform4fv.call(this, a, Float32Array.from(b));
		return;
	}
	parentUniform4fv.call(this, a, b);
};

global.WebGL2RenderingContext.prototype.getSupportedExtensions = function () {
	const string = this.__getSupportedExtensions() as string;
	if (!string) {
		return null;
	}

	return string.split(',');
};


global.CanvasRenderingContext2D.prototype.getContextAttributes = function () {
	const attrs = this.__contextAttributes;
	if (typeof attrs === 'string') {
		try {
			return JSON.parse(attrs);
		} catch (error) {}
	}
	return attrs;
};

global.CanvasRenderingContext2D.prototype.setLineDash = function (dash) {
	this.__setLineDashBuffer(Float32Array.from(dash));
};

global.CanvasRenderingContext2D.prototype.getLineDash = function () {
	return Array.from(this.__getLineDashBuffer());
};

global.ImageAsset.prototype.loadUrl = function (url, callback) {
	const that = this;
	const cb = new (org as any).nativescript.canvas.NSCImageAsset.Callback({
		onComplete: function (done) {
			callback(done);
			that.__isLoading = false;
			that.__dispose();
		},
	});

	this.__isLoading = true;
	(org as any).nativescript.canvas.NSCImageAsset.loadImageFromUrlAsync(long(this.__addr), url, cb);
};

global.ImageAsset.prototype.loadUrlAsync = function (path) {
	return new Promise((resolve, reject) => {
		this.loadUrl(path, (done) => {
			resolve(done);
		});
	});
};

global.ImageAsset.prototype.loadFile = function (path, callback) {
	const that = this;
	if (typeof path === 'string') {
		if (path.startsWith('~/')) {
			path = filePath.join(knownFolders.currentApp().path, path.replace('~/', ''));
		}
	}

	const cb = new (org as any).nativescript.canvas.NSCImageAsset.Callback({
		onComplete: function (done) {
			callback(done);
			that.__isLoading = false;
			that.__dispose();
		},
	});

	(org as any).nativescript.canvas.NSCImageAsset.loadImageFromPathAsync(long(this.__addr), path, cb);
};

global.ImageAsset.prototype.loadFileAsync = function (path) {
	return new Promise((resolve, reject) => {
		this.loadFile(path, (done) => {
			resolve(done);
		});
	});
};

global.ImageAsset.prototype.loadBytes = function (bytes, callback) {
	const that = this;
	const cb = new (org as any).nativescript.canvas.NSCImageAsset.Callback({
		onComplete: function (done) {
			callback(done);
			that.__isLoading = false;
			that.__dispose();
		},
	});

	(org as any).nativescript.canvas.NSCImageAsset.loadImageFromBytesAsync(long(this.__addr), bytes, cb);
};

global.ImageAsset.prototype.loadBytesAsync = function (path) {
	return new Promise((resolve, reject) => {
		this.loadBytes(path, (done) => {
			resolve(done);
		});
	});
};

global.ImageAsset.prototype.save = function (path, format, callback) {
	const that = this;
	const cb = new (org as any).nativescript.canvas.NSCImageAsset.Callback({
		onComplete: function (done) {
			callback(done);
			that.__isLoading = false;
			that.__dispose();
		},
	});

	(org as any).nativescript.canvas.NSCImageAsset.saveAsync(long(this.__addr), path, format, cb);
};

global.ImageAsset.prototype.saveAsync = function (path) {
	return new Promise((resolve, reject) => {
		this.save(path, (done) => {
			resolve(done);
		});
	});
};
*/

(global as any).__debug_browser_polyfill_image = false;

//require('@nativescript/canvas-polyfill');

// console.log('loadUrlAsync');

//console.log(img.width, img.height);

/*
class WebGLRenderingContextImpl extends (global as any).WebGLRenderingContext {
	texImage2D(...args) {
		if (arguments.length === 6) {
			const image = arguments[5];
			if (image && image.android instanceof android.graphics.Bitmap) {
				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
				return;
			} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
				args[5] = image._asset;
			}
		}
		super.texImage2D(...args);
	}
}

class WebGL2RenderingContextImpl exte//nds (global as any).WebGL2RenderingContext {
	texImage2D(...args) {
		console.log('WebGL2RenderingContextImpl');
		if (arguments.length === 6) {
			const image = arguments[5];
			if (image && image.android instanceof android.graphics.Bitmap) {
				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
				return;
			} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
				args[5] = image._asset;
			}
		}
		super.texImage2D(...args);
	}
}
*/
//const chroma = require('./chroma.min.js');
// class CanvasRenderingContext2DImpl extends (global as any).CanvasRenderingContext2D {
// 	drawImage(...args) {
// 		if (arguments.length === 3) {
// 			const image = arguments[0];
// 			if (image && image.android instanceof android.graphics.Bitmap) {
// 				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
// 				return;
// 			}
// 		}

// 		if (arguments.length === 5) {
// 			const image = arguments[0];
// 			if (image && image.android instanceof android.graphics.Bitmap) {
// 				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
// 				return;
// 			}
// 		}

// 		if (arguments.length === 9) {
// 			const image = arguments[0];
// 			if (image && image.android instanceof android.graphics.Bitmap) {
// 				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.width, 0, arguments[3], arguments[4], image.android, this.__flipY);
// 				return;
// 			}
// 		}
// 		super.drawImage(...arguments);
// 	}
// }

//global['images'] = images;

// } catch (e) {
// 	console.log('__non_webpack_require__', e);
// }

// console.log(' global.__getCanvasRenderingContext2DImpl', global.__getCanvasRenderingContext2DImpl);

// console.log('CanvasRenderingContext2D', CanvasRenderingContext2D);

// const ctx: CanvasRenderingContext2D = global.__getCanvasRenderingContext2DImpl(300, 300, 1, -16777216, ppi, direction, true);
declare const jp;

Application.on('launch', (args) => {
	//require('@nativescript/canvas-polyfill');
	if (global.isAndroid) {
		jp.wasabeef.takt.Takt.stock(Utils.android.getApplicationContext()).seat(jp.wasabeef.takt.Seat.TOP_CENTER).color(-65536);
	}
});

Application.run({ moduleName: 'app-root' });
