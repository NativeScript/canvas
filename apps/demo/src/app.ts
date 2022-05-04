//require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';
import { Application, path as filePath, knownFolders, Utils } from '@nativescript/core';
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

__non_webpack_require__('~/libcanvasnativev8.so');

const ctx: CanvasRenderingContext2D = global.__getCanvasRenderingContext2DImpl(300, 300, 1, -16777216, ppi, direction, true);
ctx.lineDashOffset = 10;
const array = new Float32Array([1, 2, 3, 4]);
console.log(array);
ctx.setLineDash(array as any);

console.log(ctx.getLineDash());
// try {
// 	java.lang.System.loadLibrary('canvasnative');
// } catch (e) {}

// try {
// 	java.lang.System.loadLibrary('canvasnativev8');
// } catch (e) {}

//__non_webpack_require__(`${(Utils.android.getApplicationContext() as android.content.Context).getApplicationInfo().nativeLibraryDir}/libcanvasnativev8.so`)

// (() => {
// 	global.ImageAsset.prototype.loadFromUrlAsync = function (url) {
// 		return new Promise((resolve, reject) => {
// 			this.loadFromUrlCallback(url, (error, done) => {
// 				console.log('loadFromUrlCallback', url, done, error);
// 				if (error) {
// 					reject(error);
// 					return;
// 				}
// 				if (!error) {
// 					resolve(done);
// 				}
// 			});
// 		});
// 	};

// 	global.ImageAsset.prototype.loadFileAsync = function (path) {
// 		return new Promise((resolve, reject) => {
// 			global.ImageAsset.prototype.loadFileCallback(path, (error, done) => {
// 				if (error) {
// 					reject(error);
// 					return;
// 				}
// 				if (!error) {
// 					resolve(done);
// 				}
// 			});
// 		});
// 	};

// 	global.ImageAsset.prototype.loadFromBytesAsync = function (bytes) {
// 		return new Promise((resolve, reject) => {
// 			global.ImageAsset.prototype.loadFromBytesCallback(bytes, (error, done) => {
// 				if (error) {
// 					reject(error);
// 					return;
// 				}
// 				if (!error) {
// 					resolve(done);
// 				}
// 			});
// 		});
// 	};

// 	global.ImageAsset.prototype.saveAsyncAsync = function (path, format) {
// 		return new Promise((resolve, reject) => {
// 			global.ImageAsset.prototype.saveCallback(path, format, (done, error) => {
// 				if (error) {
// 					reject(error);
// 					return;
// 				}
// 				if (!error) {
// 					resolve(done);
// 				}
// 			});
// 		});
// 	};
// })();

Application.on('launch', (args) => {
	console.log(global.Path2D);

	const count = 1;

	for (let i = 0; i < count; i++) {
		const path = new global.Path2D();

		path.rect(100, 200, 300, 400);

		const path2 = new global.Path2D(path);

		console.log('path2');

		const path3 = new global.Path2D('M 10 10 H 90 V 90 H 10 Z');

		path3.addPath(path2);
		console.log(path);
		console.log(path2);
		console.log(path3);

		console.log(path.__toSVG());

		console.log('count', 1 + i);
	}

	const matrix = new DOMMatrix();

	console.log(matrix.a, matrix.b, matrix.c);
	console.log(matrix.d, matrix.e, matrix.f);

	console.log(matrix.is2D);

	let imageData = new global.ImageData(100, 100);
	global.imageData = imageData;
	console.log('imageData', imageData.data);

	try {
		console.log('TextMetrics', TextMetrics);
	} catch (e) {
		console.log('TextMetrics: error', e);
	}

	console.log('CanvasGradient', CanvasGradient);
	console.log('CanvasPattern', CanvasPattern);

	console.log('__getCanvasRenderingContext2D', global.__getCanvasRenderingContext2DImpl);

	console.log('CanvasRenderingContext2D', CanvasRenderingContext2D);

	const encoder = new TextEncoder();
	const decoder = new TextDecoder();

	const encoded = encoder.encode('Hi Osei');
	console.log('encoded', encoded);

	const decoded = decoder.decode(encoded);
	console.log('decoded', decoded);

	let utf8decoder = new TextDecoder(); // default 'utf-8' or 'utf8'

	let u8arr = new Uint8Array([240, 160, 174, 183]);
	let i8arr = new Int8Array([-16, -96, -82, -73]);
	let u16arr = new Uint16Array([41200, 47022]);
	let i16arr = new Int16Array([-24336, -18514]);
	let i32arr = new Int32Array([-1213292304]);

	console.log(utf8decoder.decode(u8arr));
	console.log(utf8decoder.decode(i8arr));
	console.log(utf8decoder.decode(u16arr));
	console.log(utf8decoder.decode(i16arr));
	console.log(utf8decoder.decode(i32arr));

	const asset = new global.ImageAsset();
	const asset2 = new global.ImageAsset();
	const asset3 = new global.ImageAsset();

	/*
global.thing = (async () => {


	console.log(asset);

	console.log(asset.width);
	console.log(asset.error);

	let realPath = '~/assets/file-assets/webgl/rick-and-morty-by-sawuinhaff-da64e7y.png';
	if (typeof realPath === 'string') {
		if (realPath.startsWith('~/')) {
			realPath = filePath.join(knownFolders.currentApp().path, realPath.replace('~/', ''));
		}
	}
	
	asset.loadFile(realPath).then(done =>{
		console.log(java.lang.Thread.currentThread())
	});
	console.log('1');

	asset2.loadFile(realPath).then(done =>{
		console.log(java.lang.Thread.currentThread())
	});
	console.log('2');

	asset3.loadFile(realPath).then(done =>{
		console.log(java.lang.Thread.currentThread())
	});
	
	console.log('3');

 	asset.loadFile(realPath)
	 .then(done =>{
		console.log(java.lang.Thread.currentThread())
	});
	console.log('4');

	asset2.loadFile(realPath)
	.then(done =>{
		console.log(java.lang.Thread.currentThread())
	});
	console.log('5');

	asset3.loadFile(realPath)
	.then(done =>{
		console.log(java.lang.Thread.currentThread())
	});
	console.log('6');


	// global.loading.then(done =>{
	// 	console.log('done ??');
	// })
	// let done3 = await asset3.loadFile(realPath);
	// console.log('1');
	// console.log(done3, asset3.width);
	// console.log('done loading assets');

	// console.log(asset.width, asset2.width , asset3.width);

	// const done2 = await asset.loadFromUrl('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');
	// console.log('loadFromUrlCallback', 'asset', done2);

	// const done3 = await asset2.loadFromUrl('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');
	// console.log('loadFromUrlCallback', 'asset', done3);


	// asset.loadFromUrlAsync('https://mdn.mozillademos.org/files/1456/Canvas_sun.png').then((done) => {
	// 	console.log('aadsadasdasdasda');
	// });

	// asset2.loadFromUrlAsync('https://interactive-examples.mdn.mozilla.net/media/examples/star.png').then((done) => {
	// 	console.log('asset2', 'asdasdasda');
	// });

	// asset.loadFileAsync(realPath).then(done =>{
	// 	console.log('loadFileAsync');
	// })

	console.log('asdasaaadasda', new Date());

	// await asset2
	// 	.loadFromUrlAsync('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');

	// await asset3.loadFromUrlAsync("https://interactive-examples.mdn.mozilla.net/media/examples/star.png");
	//console.log(asset.width, asset2.width, asset3.width);
})(); */

	// console.log(other_done, asset.width);
});

Application.run({ moduleName: 'app-root' });
