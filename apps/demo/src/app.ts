import { Helpers } from '@nativescript/canvas/helpers';
Helpers.initialize();
require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';
declare const jp, GDPerformanceMonitor;
let monitor;
import { Canvas } from '@nativescript/canvas';
import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource, Trace } from '@nativescript/core';

Application.on('discardedError', (args) => {
	console.log(args.error);
});

// 0.253ms 1
// 0.438ms 10
// 17.375ms 100
// 27.237ms 1000

/// 35764.462ms 1_000_000

/// 2.068ms 1
// 0.354ms 10
// 0.600ms 100
// 3.155ms 1000

// 4243.135ms 1_000_000
// 3631.408ms 1_000_000 ... before ctor update

/*
JSI
 JS: CONSOLE TIME: TextEncoder: 2.423ms
  JS: CONSOLE TIME: TextDecoder: 0.646ms
*/

// const call = `
// function f(x, y) { return global.divide(x, y); }
// %PrepareFunctionForOptimization(f);
// f(100,50);
// %OptimizeFunctionOnNextCall(f);
// f(100,50);
// `;

// eval(call);

// : CONSOLE TIME: data: 4.250ms image data
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
