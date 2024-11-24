//import { CanvasRenderingContext2D, WebGLRenderingContext } from './index.js';

import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);

const { CanvasRenderingContext2D, WebGLRenderingContext } = require('./canvas-napi.darwin-universal.node');

const ctx = CanvasRenderingContext2D.withCpu(300, 150, 1, true, 0, 90, 1);
ctx.fillRect(0, 0, 300, 150);
console.log('ctx', ctx.toDataURL('image/png'));
console.log(ctx.font);
ctx.font = '100px serif';
console.log(ctx.font);

ctx.fillStyle = 'red';
ctx.fillRect(0, 0, 100, 150);
ctx.fillStyle = 'blue';
ctx.fillRect(100, 0, 100, 150);
ctx.fillStyle = 'green';
ctx.fillRect(200, 0, 100, 150);

console.log('ctx', ctx.toDataURL('image/png'));

const gl = WebGLRenderingContext.offscreen(600, 300, 1, true, false, false, false, 1, true, false, false, false, false, false);

gl.clearColor(0, 0, 1, 1);
gl.clear(gl.COLOR_BUFFER_BIT);
gl.flush();

console.log('gl', gl.toDataURL('image/png'));
