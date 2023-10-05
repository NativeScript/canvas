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

	const paths = new Array(10);
	const length = paths.length;
	console.time('data');
	for (let i = 0; i < length; i++) {
		const id = new ImageData(1000, 1000);
		const size = id.width + id.height;
		const length = id.data.length;
	}
	console.timeEnd('data');
});

Application.android.on('activityCreated', (args) => {
	const canvas = Canvas.createCustomView();
	const context = canvas.getContext('2d');
	console.log(canvas.width, canvas.height);

	const ctx = canvas.getContext('2d') as never as CanvasRenderingContext2D;

	console.time('fill');
	const fill = ctx.fillStyle;
	console.timeEnd('fill');

	//console.log(context.measureText('Osei Fortune'));
	//console.log(context.createImageData(100,100));

	// Create path
	let region = new Path2D() as any;
	region.moveTo(30, 90);
	region.lineTo(110, 20);
	region.lineTo(240, 130);
	region.lineTo(60, 130);
	region.lineTo(190, 20);
	region.lineTo(270, 90);
	region.closePath();

	let path = new Path2D() as any;
	path.addPath(region);

	console.log('new', path.__toSVG());

	/*
	  JS: CONSOLE LOG: M30 90L110 20L240 130L60 130L190 20L270 90L30 90Z
  JS: CONSOLE TIME: svg: 1.376ms
  */

	/*
    JS: CONSOLE LOG: 300 150
  JS: CONSOLE LOG: M30 90L110 20L240 130L60 130L190 20L270 90L30 90Z
  JS: CONSOLE TIME: svg: 0.819ms
  JS: CONSOLE LOG: M30 90L110 20L240 130L60 130L190 20L270 90L30 90Z
  JS: CONSOLE TIME: svg: 0.424ms
  */

	console.time('svg');
	console.log(region.__toSVG());
	console.timeEnd('svg');

	setTimeout(() => {
		const r = new Path2D(region) as any;
		console.time('svg1');
		console.log(r.__toSVG());
		console.timeEnd('svg1');
	}, 1000);

	setTimeout(() => {
		const r = new Path2D(region) as any;
		console.time('svg2');
		console.log(r.__toSVG());
		console.timeEnd('svg2');
	}, 1500);

	// Fill path
	ctx.fillStyle = 'green';
	ctx.fill(region, 'evenodd');
	//ctx.fillRect(0, 0, 100, 100);
	android.util.Log.d('JS', canvas.toDataURL());
});

Application.run({ moduleName: 'app-root' });
