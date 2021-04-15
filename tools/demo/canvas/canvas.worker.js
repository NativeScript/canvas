require('@nativescript/core');
const TNSCanvas = require('@nativescript/canvas');
const canvas2D = require('./canvas2d');
const webgl = require('./webgl');
const webgl2 = require('./webgl2');
const swarm = canvas2D.swarm;
const particlesColor = canvas2D.particlesColor;
const touchParticles =  canvas2D.touchParticles();
const cubeExample = webgl.main;
const drawElements = webgl.drawElements;
const environmentMap = webgl2.environmentMap;
const fog = webgl2.fog;
//import {isAndroid} from '@nativescript/core';

onmessage = ev => {
    const data = ev.data;
    if (data.event === 'touch') {
        // const x = data.x;
        // const y = data.y;
        // particles.cleanUpArray();
        // particles.initParticles(x, y);
        // return;
    }
    let ref;
    let nativeCanvas;
    if (global.android) {
        const contexts = org.nativescript.canvas.TNSCanvas.getViews();
        ref = contexts.get(data.id);
        nativeCanvas = ref.get();
    } else {
        const contexts = Canvas.getViews();
        ref = contexts.objectForKey(data.id);
        nativeCanvas = ref;
        if(nativeCanvas){
            nativeCanvas.handleMoveOffMain();
        }
    }
    if (nativeCanvas) {
        let ctx;

        function doCtx(ctxType) {
            let nativeCtx
            const opts = {
                alpha: true,
                antialias: true,
                depth: true,
                failIfMajorPerformanceCaveat: false,
                powerPreference: "default",
                premultipliedAlpha: true,
                preserveDrawingBuffer: false,
                stencil: false,
                desynchronized: false
            };

            if (ctxType === '2d') {
                if (global.android) {
                    nativeCtx = nativeCanvas.getContext('2d');
                } else {
                    nativeCtx = nativeCanvas.getContextContextAttributes('2d', opts)
                }
                ctx = new TNSCanvas.TNSCanvasRenderingContext2D(nativeCtx);
            } else if (ctxType === 'webgl2') {
                if (global.android) {
                    nativeCtx = nativeCanvas.getContext('webgl2', opts);
                } else {
                    nativeCtx = nativeCanvas.getContextContextAttributes('webgl2', opts)
                }
                ctx = new TNSCanvas.TNSWebGL2RenderingContext(nativeCtx);
            } else {
                if (global.android) {
                    nativeCtx = nativeCanvas.getContext('webgl', opts);
                } else {
                    nativeCtx = nativeCanvas.getContextContextAttributes('webgl', opts)
                }
                ctx = new TNSCanvas.TNSWebGLRenderingContext(nativeCtx);
            }
        }

        doCtx('webgl2');
       // cubeExample(ctx, nativeCanvas);
       // swarm(ctx, data.width, data.height, nativeCanvas);
        // particlesColor(ctx, data.width, data.height, nativeCanvas);
       // particles.touchParticles(ctx, data.width, data.height, nativeCanvas);
        //drawElements(ctx, data.width, data.height, nativeCanvas);
       // environmentMap(ctx, data.width, data.height, nativeCanvas);
        fog(ctx, data.width, data.height, nativeCanvas);
    }


}


onerror = error =>{
    console.log('Worker: Error', error)
}
