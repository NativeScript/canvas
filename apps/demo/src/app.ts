//require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';
declare const jp, GDPerformanceMonitor;
let monitor;

import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource, Trace } from '@nativescript/core';

Application.on('discardedError', (args) => {
	console.log(args.error);
});

Application.on('launch', (args) => {
	require('@nativescript/canvas-polyfill');
	if (global.isAndroid) {
		jp.wasabeef.takt.Takt.stock(Utils.android.getApplicationContext()).seat(jp.wasabeef.takt.Seat.TOP_CENTER).color(-65536);
	} else {
		monitor = GDPerformanceMonitor.new();
		monitor.startMonitoringWithConfiguration((label) => {
			label.backgroundColor = UIColor.blackColor;
			label.textColor = UIColor.whiteColor;
			label.layer.borderColor = UIColor.redColor;
		});
		monitor.appVersionHidden = true;
		monitor.deviceVersionHidden = true;
	}
});

Application.run({ moduleName: 'app-root' });
