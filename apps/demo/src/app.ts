//require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';
import { Application, path as filePath, knownFolders, Utils, path as nsPath } from '@nativescript/core';
declare var __non_webpack_require__;

(function () {
	// Providing Array Helpers since v8 crashing for now

	global.__Array_Get = (array, i) => {
		if (!Array.isArray(array)) {
			return undefined;
		}
		console.log('__Array_Get', array, i);
		console.log(array[i]);
		return array[i];
	};

	global.__Array_Set = (array, i, value) => {
		if (!Array.isArray(array)) {
			return;
		}
		array[i] = value;
	};
})();

let direction = 0;

if (androidx.core.text.TextUtilsCompat.getLayoutDirectionFromLocale(java.util.Locale.getDefault()) === androidx.core.view.ViewCompat.LAYOUT_DIRECTION_RTL) {
	direction = 1;
}

const ppi = (Utils.ad.getApplicationContext() as android.content.Context).getResources().getDisplayMetrics().density * 160;
// try {
__non_webpack_require__('~/libcanvasnativev8.so');

const handlePath = function (path) {
	if (typeof path === 'string' && path.startsWith('~/')) {
		return nsPath.join(knownFolders.currentApp().path, path.replace('~/', ''));
	}
	return path;
};

(global as any).__debug_browser_polyfill_image = true;

class ImageAssetImpl extends (global as any).ImageAsset {
	save(path, format, done) {
		super.save(handlePath(path), format, done);
	}

	saveSync(path, format) {
		return super.saveSync(handlePath(path), format);
	}

	saveAsync(path, format) {
		return new Promise((resolve, reject) => {
			this.save(path, format, (done) => {
				resolve(done);
			});
		});
	}

	loadBytesAsync(bytes) {
		return new Promise((resolve, reject) => {
			this.loadBytes(bytes, function (done) {
				resolve(done);
			});
		});
	}

	loadFile(file, done) {
		super.loadFile(handlePath(file), done);
	}

	loadFileSync(file) {
		return super.loadFileSync(handlePath(file));
	}

	loadFileAsync(file) {
		return new Promise((resolve, reject) => {
			this.loadFile(file, (done) => {
				resolve(done);
			});
		});
	}

	loadUrlAsync(url) {
		return new Promise((resolve, reject) => {
			this.loadUrl(url, (done) => {
				resolve(done);
			});
		});
	}
}

class WebGLRenderingContextImpl extends (global as any).WebGLRenderingContext {
	texImage2D(...args) {
		if (arguments.length === 6) {
			const image = arguments[5];
			if (image && image.android instanceof android.graphics.Bitmap) {
				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
				return;
			} else if(image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')){
				args[5] = image._asset;
			}
		}
		super.texImage2D(...args);
	}
}

class WebGL2RenderingContextImpl extends (global as any).WebGL2RenderingContext {
	texImage2D(...args) {
		if (arguments.length === 6) {
			const image = arguments[5];
			if (image && image.android instanceof android.graphics.Bitmap) {
				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
				return;
			} else if(image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')){
				args[5] = image._asset;
			}
		}
		super.texImage2D(...args);
	}
}

class CanvasRenderingContext2D extends (global as any).CanvasRenderingContext2D {
	drawImage(...args) {
		if (arguments.length === 3) {
			const image = arguments[0];
			if (image && image.android instanceof android.graphics.Bitmap) {
				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
				return;
			}
		}
	
		if (arguments.length === 5) {
			const image = arguments[0];
			if (image && image.android instanceof android.graphics.Bitmap) {
				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.height, 0, arguments[3], arguments[4], image.android, this.__flipY);
				return;
			}
		}
	
		if (arguments.length === 9) {
			const image = arguments[0];
			if (image && image.android instanceof android.graphics.Bitmap) {
				(org as any).nativescript.canvas.TNSWebGLRenderingContext.nativeTexImage2DBitmap(arguments[0], arguments[1], arguments[2], image.width, image.width, 0, arguments[3], arguments[4], image.android, this.__flipY);
				return;
			}
		}
		super.CanvasRenderingContext2D.prototype.drawImage(...arguments);
	}
}


global.WebGLRenderingContext = WebGLRenderingContextImpl as any;
global.WebGL2RenderingContext = WebGL2RenderingContextImpl as any;
global.CanvasRenderingContext2D = CanvasRenderingContext2D as any;
global.ImageAsset = ImageAssetImpl;

// } catch (e) {
// 	console.log('__non_webpack_require__', e);
// }

// console.log(' global.__getCanvasRenderingContext2DImpl', global.__getCanvasRenderingContext2DImpl);

// console.log('CanvasRenderingContext2D', CanvasRenderingContext2D);

// const ctx: CanvasRenderingContext2D = global.__getCanvasRenderingContext2DImpl(300, 300, 1, -16777216, ppi, direction, true);

Application.run({ moduleName: 'app-root' });
