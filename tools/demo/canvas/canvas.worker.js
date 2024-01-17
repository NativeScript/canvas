require('@nativescript/core');
const Canvas = require('@nativescript/canvas').Canvas;
const canvas2D = require('./canvas2d');
const webgl = require('./webgl');
const webgl2 = require('./webgl2');
const swarm = canvas2D.swarm;
const particlesColor = canvas2D.particlesColor;
const touchParticles = canvas2D.touchParticles;
const cubeExample = webgl.main;
const drawElements = webgl.drawElements;
const environmentMap = webgl2.environmentMap;
const fog = webgl2.fog;
//import {isAndroid} from '@nativescript/core';


let canvas;
onmessage = (ev) => {
	const data = ev.data;
	// if (data.event === 'touchstart' || data.event === 'touchmove') {
	// 	const x = data.clientX;
	// 	const y = data.clientY;
	// 	canvas2D.cleanUpArray();
	// 	canvas2D.initParticles(x, y);
	// 	 return;
	// }
 


    
	let ref;
	let nativeCanvas;
	if (global.android) {
        android.os.Process.setThreadPriority(android.os.Process.THREAD_PRIORITY_DISPLAY);
		const contexts = org.nativescript.canvas.NSCCanvas.getViews();
		ref = contexts.get(data.id);
		nativeCanvas = ref.get();
	} else {
		const contexts = NSCCanvas.getViews();
		ref = contexts.objectForKey(data.id);
		nativeCanvas = ref;
		// if(nativeCanvas){
		//     nativeCanvas.handleMoveOffMain();
		// }
	}
    canvas = new Canvas(nativeCanvas);


	if (nativeCanvas) {
		let ctx;

		function doCtx(ctxType) {
			let nativeCtx;
			const opts = {
				alpha: true,
				antialias: true,
				depth: true,
				failIfMajorPerformanceCaveat: false,
				powerPreference: 'default',
				premultipliedAlpha: true,
				preserveDrawingBuffer: false,
				stencil: false,
				desynchronized: false,
			};

			if (ctxType === '2d') {
				ctx = canvas.getContext('2d', opts);
			} else if (ctxType === 'webgl2') {
				ctx = canvas.getContext('webgl2', opts);
			} else {
				ctx = canvas.getContext('webgl', opts);
			}
		}

		doCtx('2d');
		//  doCtx('webgl2');
		// cubeExample(ctx, nativeCanvas);
		 swarm(ctx, data.width, data.height, nativeCanvas);
		// particlesColor(ctx, data.width, data.height, nativeCanvas);
		//touchParticles(ctx, data.width, data.height, nativeCanvas);
		//drawElements(ctx, data.width, data.height, nativeCanvas);
		// environmentMap(ctx, data.width, data.height, nativeCanvas);
		//  fog(ctx, data.width, data.height, nativeCanvas);
	}
    
};

onerror = (error) => {
	console.log('Worker: Error', error);
};
