/*import { Helpers } from '@nativescript/canvas/helpers';
Helpers.initialize();
require('@nativescript/canvas-polyfill');
*/
import '@nativescript/canvas-polyfill';
import { Canvas } from '@nativescript/canvas';

// for (let i = 0; i < 100_000; i++) {
// 	const canvas = new Canvas();
// }

const font = new FontFace('Serifa-Bold', 'url(~/fonts/Serifa-Bold.otf)', {
	weight: 'bold',
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

document.fonts
	.load('12px Serifa-Bold')
	.then((fonts) => {
		console.log(document.fonts.check('12px Serifa-Bold'));
	})
	.catch(() => {
		console.log('error');
	});

/*


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
import { Application, path as filePath, knownFolders, Utils, path as nsPath, ImageSource, Trace, Screen, Color } from '@nativescript/core';

function describeAppError(value: unknown): string {
	if (value == null) {
		return '';
	}

	try {
		if (typeof value === 'string') {
			return value;
		}

		const maybeError = value as { message?: unknown; stack?: unknown };
		const message = typeof maybeError?.message === 'string' ? maybeError.message : '';
		const stack = typeof maybeError?.stack === 'string' ? maybeError.stack : '';

		if (message && stack) {
			return `${message} ${stack}`;
		}

		if (message) {
			return message;
		}

		if (stack) {
			return stack;
		}

		return String(value);
	} catch {
		return 'unknown error';
	}
}

function logAppError(label: string, value?: unknown) {
	try {
		const detail = describeAppError(value);
		console.log(detail ? `${label} ${detail}` : label);
	} catch {}
}

// Canvas.useSurface = false;
// Canvas.forceGL = false;
Application.on('discardedError', (args) => {
	logAppError('discardedError', args.error);
	logAppError('discardedError platform', args.android ?? args.ios);
});

Application.on('uncaughtError', (args) => {
	logAppError('uncaughtError', args.error);
	logAppError('uncaughtError platform', args.android ?? args.ios);
});
// global.process = {} as any;
// global.process.env = {} as any;

Application.on('launch', (args) => {
	//require('@nativescript/canvas-polyfill');
	if (__ANDROID__) {
		jp.wasabeef.takt.Takt.stock(Utils.android.getApplicationContext()).seat(jp.wasabeef.takt.Seat.TOP_CENTER).color(-65536);
	} else if (__IOS__) {
		monitor = GDPerformanceMonitor.new();
		monitor.startMonitoringWithConfiguration((label) => {
			label.backgroundColor = UIColor.blackColor;
			label.textColor = UIColor.whiteColor;
			label.layer.borderColor = UIColor.redColor;
		});
		monitor.appVersionHidden = true;
		monitor.deviceVersionHidden = true;
	}

	/*
	const canvas = document.createElement('canvas');
	const ctx = canvas.getContext('2d')!;

	const texts = ['Hello', 'Hello world', 'The quick brown fox jumps over the lazy dog', 'Lorem ipsum dolor sit amet consectetur adipiscing elit'];

	const fonts = ['12px Arial', '16px Arial', '20px Arial', '16px Times New Roman'];

	const ITERATIONS = 50_000;

	const defaultFont = ctx.font;
	fonts.forEach((font) => {
		ctx.font = font;
	});
	ctx.font = defaultFont;

	let i = 0;
	let start = performance.now();

	for (let n = 0; n < ITERATIONS; n++) {
		ctx.font = fonts[n % fonts.length];
		ctx.measureText(texts[n % texts.length]);
	}

	let end = performance.now();

	if (__APPLE__) {
		//@ts-ignore
		__nslog('Total time: ' + (end - start) + ' ms');
		//@ts-ignore
		__nslog('Per call: ' + (end - start) / ITERATIONS + ' ms');
	} else {
		console.log('Total time:', end - start, 'ms');
		console.log('Per call:', (end - start) / ITERATIONS, 'ms');
	}

	// i = 0;
	// start = performance.now();

	// for (let n = 0; n < ITERATIONS; n++) {
	// 	ctx.font = fonts[n % fonts.length];
	// 	ctx.measureText(texts[n % texts.length]);
	// }

	// end = performance.now();

	// if (__APPLE__) {
	// 	//@ts-ignore
	// 	__nslog('Total time: ' + (end - start) + ' ms');
	// 	//@ts-ignore
	// 	__nslog('Per call: ' + (end - start) / ITERATIONS + ' ms');
	// } else {
	// 	console.log('Total time:', end - start, 'ms');
	// 	console.log('Per call:', (end - start) / ITERATIONS, 'ms');
	// }
	*/
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
