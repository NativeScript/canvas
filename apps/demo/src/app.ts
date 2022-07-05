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


const oldSave = global.ImageAsset.prototype.save;
const oldSaveSync = global.ImageAsset.prototype.saveSync;
global.ImageAsset.prototype.save = function (path, format, done) {
	oldSave(handlePath(path), format, done);
};

global.ImageAsset.prototype.saveSync = function (path, format) {
	return oldSaveSync(handlePath(path), format);
};

global.ImageAsset.prototype.saveAsync = function (path, format) {
	return new Promise((resolve, reject) => {
		global.ImageAsset.prototype.save(path, format, (done) => {
			resolve(done);
		});
	});
};

const oldLoadBytes = global.ImageAsset.prototype.loadBytes;
const oldLoadBytesSync = global.ImageAsset.prototype.loadBytesSync;
global.ImageAsset.prototype.loadBytes = function (bytes, done) {
	oldLoadBytes(bytes, done);
};

global.ImageAsset.prototype.loadBytesSync = function (bytes) {
	return oldLoadBytesSync(bytes);
};

global.ImageAsset.prototype.loadBytesAsync = function (bytes) {
	return new Promise((resolve, reject) => {
		global.ImageAsset.prototype.loadBytes(bytes, function (done) {
			resolve(done);
		});
	});
};


const oldLoadFile = global.ImageAsset.prototype.loadFile;
const oldLoadFileSync = global.ImageAsset.prototype.loadFileSync;

global.ImageAsset.prototype.loadFile = function (file, done) {
	oldLoadFile(handlePath(file), done);
};

global.ImageAsset.prototype.loadFileSync = function (file) {
	return oldLoadFileSync(handlePath(file));
};

global.ImageAsset.prototype.loadFileAsync = function (file) {
	return new Promise((resolve, reject) => {
		global.ImageAsset.prototype.loadFile(file, (done) => {
			resolve(done);
		});
	});
};

global.ImageAsset.prototype.loadUrlAsync = function (url) {
	console.log('loadUrlAsync');
	return new Promise((resolve, reject) => {
		console.log(global.ImageAsset.prototype.loadUrl);
		global.ImageAsset.prototype.loadUrl(url, (done) => {
			resolve(done);
		});
	});
};


console.log('asdasd', global.ImageAsset.prototype.loadUrlAsync);



// } catch (e) {
// 	console.log('__non_webpack_require__', e);
// }

// console.log(' global.__getCanvasRenderingContext2DImpl', global.__getCanvasRenderingContext2DImpl);

// console.log('CanvasRenderingContext2D', CanvasRenderingContext2D);

// const ctx: CanvasRenderingContext2D = global.__getCanvasRenderingContext2DImpl(300, 300, 1, -16777216, ppi, direction, true);

Application.run({ moduleName: 'app-root' });
