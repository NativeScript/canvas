//require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';

const orginalEncoder = global.TextEncoder;
const orginalDecoder = global.TextDecoder;

import { ImageAsset, ImageData } from '@nativescript/canvas';
import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource } from '@nativescript/core';
declare var __non_webpack_require__, __initAppConfiguration, _createImageBitmap;

let direction = 0;

if (androidx.core.text.TextUtilsCompat.getLayoutDirectionFromLocale(java.util.Locale.getDefault()) === androidx.core.view.ViewCompat.LAYOUT_DIRECTION_RTL) {
	direction = 1;
}

const ppi = (Utils.ad.getApplicationContext() as android.content.Context).getResources().getDisplayMetrics().density * 160;

const asset = new ImageAsset();

console.log(asset.width, asset.height, asset.error);

const success = asset.fromFileSync('~/assets/file-assets/webgl/svh.jpeg');

console.log(success, asset.width, asset.height, asset.error);

// success.then(value =>{
// 	console.log(success, asset.width, asset.height, asset.error);
// });

//__initAppConfiguration({ appBase: knownFolders.currentApp().path });

const handlePath = function (path) {
	if (typeof path === 'string' && path.startsWith('~/')) {
		return nsPath.join(knownFolders.currentApp().path, path.replace('~/', ''));
	}
	return path;
};

// const ctx: CanvasRenderingContext2D = global.__getCanvasRenderingContext2DImpl(300, 300, 1, -16777216, ppi, direction, true);
declare const jp;

Application.on('launch', (args) => {
	require('@nativescript/canvas-polyfill');
	if (global.isAndroid) {
		jp.wasabeef.takt.Takt.stock(Utils.android.getApplicationContext()).seat(jp.wasabeef.takt.Seat.TOP_CENTER).color(-65536);
	}
});

Application.run({ moduleName: 'app-root' });
