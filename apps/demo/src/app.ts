//require('@nativescript/canvas-polyfill');
import { Application, Utils, path as filePath, knownFolders } from '@nativescript/core';
declare var __non_webpack_require__;
__non_webpack_require__('~/libcanvasnativev8.so');

try {
	java.lang.System.loadLibrary('canvasnativev8');
} catch (e) {}

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
