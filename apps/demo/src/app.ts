//require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';
import { Application, Utils, path as filePath, knownFolders } from '@nativescript/core';
declare var __non_webpack_require__;
__non_webpack_require__('~/libcanvasnativev8.so');

// try {
// 	java.lang.System.loadLibrary('canvasnative');
// } catch (e) {}

// try {
// 	java.lang.System.loadLibrary('canvasnativev8');
// } catch (e) {}

//__non_webpack_require__(`${(Utils.android.getApplicationContext() as android.content.Context).getApplicationInfo().nativeLibraryDir}/libcanvasnativev8.so`)

console.log(global.Path2D);

const count = 1;

for (let i = 0; i < count; i++) {
	const path = new global.Path2D();

	path.rect(100, 200, 300, 400);

	const path2 = new global.Path2D(path);

	console.log('path2');

	const path3 = new global.Path2D("M 10 10 H 90 V 90 H 10 Z");

	console.log('path3');

	path3.addPath(path2);

	console.log(path, path2, path3);

	console.dir(path.__toSVG());

	console.log('count', i);
}

const matrix = new DOMMatrix();

console.log(matrix.a, matrix.b, matrix.c);
console.log(matrix.d, matrix.e, matrix.f);

console.log(matrix.is2D);


let imageData = new global.ImageData(100,100);
global.imageData = imageData;
console.log('imageData', imageData.data);

try{
	console.log('TextMetrics',TextMetrics);
}catch(e){
	console.log('TextMetrics: error',e);
}

console.log('CanvasGradient',CanvasGradient);
console.log('CanvasPattern',CanvasPattern);

console.log('__getCanvasRenderingContext2D', global.__getCanvasRenderingContext2DImpl);

console.log('CanvasRenderingContext2D', CanvasRenderingContext2D);


const encoder = new TextEncoder();
const decoder = new TextDecoder();

const encoded = encoder.encode("Hi Osei");
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

console.log(asset);

console.log(asset.width);
console.log(asset.error);


let realPath = '~/assets/file-assets/webgl/rick-and-morty-by-sawuinhaff-da64e7y.png';
if (typeof realPath === 'string') {
	if (realPath.startsWith('~/')) {
		realPath = filePath.join(knownFolders.currentApp().path, realPath.replace('~/', ''));
	}
}
let done = asset.loadFile(realPath);
console.log(done, asset.width);

asset.loadFileAsync(realPath).then(done =>{
	console.log('loadFileAsync done', done);
	console.log('loadFileAsync', asset.width);
}).catch(e =>{
	console.log('loadFileAsync error', e)
})

// asset.loadFromUrlAsync("https://interactive-examples.mdn.mozilla.net/media/examples/star.png").then(done =>{
// 	console.log('loadFromUrlAsync done', done);
// 	console.log('loadFromUrlAsync', asset.width);
// }).catch(e =>{
// 	console.log('loadFromUrlAsync error', e)
// });

// let other_done = asset.loadFromUrl("https://interactive-examples.mdn.mozilla.net/media/examples/star.png");

// console.log(other_done, asset.width);

Application.run({ moduleName: 'app-root' });
