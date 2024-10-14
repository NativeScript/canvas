/*import { Helpers } from '@nativescript/canvas/helpers';
Helpers.initialize();
require('@nativescript/canvas-polyfill');
*/
import '@nativescript/canvas-polyfill';
// require('@nativescript/canvas-polyfill');
// import { Svg } from '@nativescript/canvas-svg';
// import { ImageAsset } from '@nativescript/canvas';
declare const jp, GDPerformanceMonitor, android, java, UIColor;
let monitor;
import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource, Trace, Screen } from '@nativescript/core';

Application.on('discardedError', (args) => {
	console.log('discardedError', args.error, args);
});

Application.on('uncaughtError', (args) => {
	console.log('uncaughtError', args.error, args);
});
// global.process = {} as any;
// global.process.env = {} as any;

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

// fetch('https://github.com/mrdoob/three.js/blob/dev/examples/models/gltf/DamagedHelmet/glTF/Default_metalRoughness.jpg?raw=true')

// .then(async (response) => {
// 	const blob = await response.blob();
// 	console.log('blob',blob);
// })
// .catch((err) => {
// 	console.log(err);
// });
// const image = new global.ImageAsset();
// image
// 	.fromUrl('https://github.com/mrdoob/three.js/blob/dev/examples/models/gltf/DamagedHelmet/glTF/Default_metalRoughness.jpg?raw=true')
// 	.then(function (done) {
// 		console.log('done 1', done);
// 	})
// 	.catch(function (err) {
// 		console.log('err', err);
// 	});

// const image2 = new global.ImageAsset();
// image
// 	.fromUrl('https://github.com/mrdoob/three.js/blob/dev/examples/models/gltf/DamagedHelmet/glTF/Default_metalRoughness.jpg?raw=true')
// 	.then(function (done) {
// 		console.log('done 2', done);
// 	})
// 	.catch(function (err) {
// 		console.log('err', err);
// 	});
Application.run({ moduleName: 'app-root' });
