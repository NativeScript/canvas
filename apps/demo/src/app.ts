/*import { Helpers } from '@nativescript/canvas/helpers';
Helpers.initialize();
require('@nativescript/canvas-polyfill');
*/
import '@nativescript/canvas-polyfill';
/*
import { Canvas, importFontsFromCSS } from '@nativescript/canvas';

const font = new FontFace('Roboto', 'url(https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu4mxP.ttf)');

importFontsFromCSS('https://fonts.googleapis.com/css2?family=Monsieur+La+Doulaise&family=Noto+Serif+TC:wght@200..900&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap')
	.then((fonts) => {
		for (const font of fonts) {
			console.log('status', font.status);
		}
	})
	.catch((err) => {
		console.log('error', err);
	});

document.fonts.add(font);

document.fonts.addEventListener('loading', (event) => {
	console.log('loading');
});
document.fonts.addEventListener('loadingerror', (event) => {
	console.log('loadingerror');
});
document.fonts.addEventListener('loadingdone', (event) => {
	console.log('loadingdone', event.fontfaces);
});

document.fonts.forEach((font) => {
	console.log('font', font);
});

document.fonts
	.load('12px Roboto')
	.then(() => {
		console.log('loaded', font.status);
	})
	.catch(() => {
		console.log('error');
	});

	*/

// import { DomainDispatcher, initDevtools, ProtocolWrapper } from '@nativescript/devtools';
// initDevtools();

// @DomainDispatcher('Inspector')
// class Inspector extends ProtocolWrapper {}

// @DomainDispatcher('Runtime')
// class Runtime extends ProtocolWrapper {
// 	evaluate() {
// 		console.log('evaluate', arguments);
// 	}

// 	runScript() {
// 		console.log('runScript', arguments);
// 	}
// }
/*

@DomainDispatcher('Page')
class CustomPage extends ProtocolWrapper {
	addScriptToEvaluateOnNewDocument(params) {
		if (params.worldName === 'DevTools Performance Metrics') {
			this.emit('Runtime.executionContextCreated', {
				context: {
					id: 2,
					name: params.worldName,
					origin: '',
					uniqueId: performance.now(),
				},
			});
			return;
		}

		this.emit('Page.addScriptToEvaluateOnNewDocument', {});
	}
}
*/

// require('@nativescript/canvas-polyfill');
// import { Svg } from '@nativescript/canvas-svg';
// import { ImageAsset } from '@nativescript/canvas';
declare const jp, GDPerformanceMonitor, android, java, UIColor;
let monitor;
import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource, Trace, Screen } from '@nativescript/core';

Canvas.useSurface = false;
Canvas.forceGL = false;
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
