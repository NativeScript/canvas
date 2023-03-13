//require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';

import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource } from '@nativescript/core';


Application.on('discardedError', (args) => {
	console.log(args.error);
});

declare const jp;

Application.on('launch', (args) => {
	require('@nativescript/canvas-polyfill');
	if (global.isAndroid) {
		jp.wasabeef.takt.Takt.stock(Utils.android.getApplicationContext()).seat(jp.wasabeef.takt.Seat.TOP_CENTER).color(-65536);
	}
});

Application.run({ moduleName: 'app-root' });
