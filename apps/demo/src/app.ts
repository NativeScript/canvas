/*import { Helpers } from '@nativescript/canvas/helpers';
Helpers.initialize();
require('@nativescript/canvas-polyfill');
*/
require('@nativescript/canvas-polyfill');
// import { CanvasRenderingContext2D } from '@nativescript/canvas';
declare const jp, GDPerformanceMonitor;
let monitor;
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

/*

let url = new URL("https://example.com?foo=1&bar=2");
let params = new URLSearchParams(url.search);

//Add a second foo parameter.
params.append("foo", 4 as any);
//Query string is now: 'foo=1&bar=2&foo=4'

console.log(url.search);

url = new URL('https://example.com/?name=Jonathan%20Smith&age=18');
params =  url.searchParams;

const name = params.get("name"); // is the string "Jonathan Smith".
const age = parseInt(params.get("age") as any); // is the number 18


*/
// url: 5804.647ms

// other url: 648.170ms

// try loading our custom font


const font = nsPath.join(knownFolders.currentApp().path, 'fonts/Creepster-Regular.ttf');

global.CanvasModule.__addFontFamily('creepster', [font]);




// console.time('url');
// for (let i = 0; i < 1_000_000; i++) {
// 	const url = new URL('https://example.com/?name=Jonathan%20Smith&age=18');
// }
// console.timeEnd('url');

// const url = new URL('https://example.com/?name=Jonathan%20Smith&age=18');

// console.log('canParse', URL.canParse('asdasd'), URL.canParse('https://example.com/?name=Jonathan%20Smith&age=18'));

Application.on('uncaughtError', (args) => {
	console.log('uncaughtError: error', args.error);
	console.log('uncaughtError: platform error', args.android ?? args.ios);
});
Application.on('launch', (args) => {
	//require('@nativescript/canvas-polyfill');
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
