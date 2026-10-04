# NativeScript Canvas

**Powered by**

- [CanvasNative](src-native/canvas-native) - Rust ([Skia](https://github.com/rust-skia/rust-skia), [WGPU](https://github.com/gfx-rs/wgpu))
- [CanvasNative](src-native/canvas-ios) - IOS
- [CanvasNative](src-native/canvas-android) - Android

## Installation

```bash
ns plugin add @nativescript/canvas
```

_Note_ min ios support 11 | min android support 21

IMPORTANT: ensure you include xmlns:canvas="@nativescript/canvas" on the Page element for core {N}

## Usage


```xml
<canvas:Canvas id="canvas" style="width:100%; height:100%"  width="100%" height="100%" ready="canvasReady"/>
```

### 2D

```typescript
let ctx;
let canvas;
export function canvasReady(args) {
	console.log('canvas ready');
	canvas = args.object;
	console.log(canvas);
	ctx = canvas.getContext('2d');
	ctx.fillStyle = 'green';
	ctx.fillRect(10, 10, 150, 100);
}
```

#### Reading pixels back: `willReadFrequently`

[`willReadFrequently`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)
is a standard 2D context attribute, and it means here what it means on the web:
it tells the canvas you intend to read pixels back often, so it should be backed
by CPU memory rather than the GPU.

```typescript
ctx = canvas.getContext('2d', { willReadFrequently: true });
```

It is worth knowing what the trade actually costs on a phone. Measured on a
Galaxy A53, for a 64x64 region:

| | default | `willReadFrequently: true` |
| --- | --- | --- |
| `getImageData` | 412 µs | 3.8 µs |
| `putImageData` | 156 µs | 0.5 µs |
| `fillText` | 1.1 µs | 6.3 µs |
| `strokeRect` | 0.65 µs | 2.7 µs |

Pixel access gets two orders of magnitude faster, and ordinary drawing gets
several times slower. Reach for it when you are doing image processing,
hit-testing against pixel data, or anything else that calls `getImageData` in a
loop — and leave it off for normal drawing.

### WEBGL

```typescript
let gl;
let canvas;
export function canvasReady(args) {
	console.log('canvas ready');
	canvas = args.object;
	gl = canvas.getContext('webgl'); // 'webgl' || 'webgl2'
	gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
	// Set the clear color to darkish green.
	gl.clearColor(0.0, 0.5, 0.0, 1.0);
	// Clear the context with the newly set color. This is
	// the function call that actually does the drawing.
	gl.clear(gl.COLOR_BUFFER_BIT);
}
```

### OffscreenCanvas

A canvas with no view: draw with any context, then use it as an image (`drawImage`,
`createPattern`, `createImageBitmap`, `texImage2D`, `copyExternalImageToTexture`), hand its
frame over with `transferToImageBitmap()`, or encode it with `convertToBlob()`.

```typescript
import { OffscreenCanvas } from '@nativescript/canvas';

const sprite = new OffscreenCanvas(64, 64);
const ctx = sprite.getContext('2d');
ctx.fillStyle = 'tomato';
ctx.fillRect(0, 0, 64, 64);

// Later, on a visible canvas:
visible.getContext('2d').drawImage(sprite, 0, 0);

const blob = await sprite.convertToBlob({ type: 'image/png' });
```

`canvas.transferControlToOffscreen()` returns an OffscreenCanvas that draws into that canvas's
surface, so its frames show in the view. With the polyfill, `OffscreenCanvas` is also a global.

#### In a Worker

An OffscreenCanvas can move to a Worker, which then draws into it on its own thread: a transferred
canvas keeps showing the Worker's frames in its view. Where the runtime can transfer it (NativeScript
Windows, with transfer support in `postMessage`), it moves as on the web:

```typescript
// Main thread
const offscreen = canvas.transferControlToOffscreen();
worker.postMessage({ canvas: offscreen }, [offscreen]);
```

```typescript
// The Worker
import '@nativescript/canvas/worker';

self.onmessage = ({ data }) => {
	const canvas: OffscreenCanvas = data.canvas;
	const ctx = canvas.getContext('2d');
	const draw = () => {
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		requestAnimationFrame(draw);
	};
	requestAnimationFrame(draw);
};
```

Elsewhere (the Android and iOS runtimes, for now), send a handle instead:

```typescript
// Main thread
worker.postMessage({ canvas: OffscreenCanvas._toHandle(offscreen) });

// The Worker
import { OffscreenCanvas } from '@nativescript/canvas/worker';
const canvas = OffscreenCanvas._fromHandle(data.canvas);
```

- `@nativescript/canvas/worker` is the canvas API without the `Canvas` view: import it first in the Worker.
- A transfer (or `_toHandle`) detaches the OffscreenCanvas it is given, and it can't have a context yet. A handle is received once; `OffscreenCanvas._releaseHandle(handle)` drops one nothing will receive.
- `2d`, `bitmaprenderer`, `webgl`, `webgl2` and `webgpu` all work there, as does `new OffscreenCanvas(w, h)` made in the Worker, which can be transferred back.
- Resizing it in the Worker resizes the view on the UI thread a moment later.

## WebGPU

_Note_ min ios support 11 | min android support 27

```typescript

// the webgpu type works as well but these exposes any non standard web api (native)

import type { GPUDevice, GPUAdapter } from '@nativescript/canvas';
import { Screen } from '@nativescript/core';

let canvas;
let device: GPUDevice;
export async function canvasReady(args) {
	console.log('canvas ready');
	canvas = args.object;

	const adapter: GPUAdapter = (await navigator.gpu.requestAdapter()) as never;
	device = (await adapter.requestDevice()) as never;
	// scaling the canvas to ensure everthing looks crisp
	const devicePixelRatio = Screen.mainScreen.scale;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	const context = canvas.getContext('webgpu');


	/// configureing the context
	// Passing in the following options will aollow the configure method to choose the best configs.
	// If unsure about what is supported try the following method

	const capabilities = this.getCapabilities(device);

	// cap.presentModes
	// cap.alphaModes
	// cap.format
	// cap.usages

	context.configure({
		device,
		format: presentationFormat,
	});

}
```




## API

- 2D Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
- WebGL Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
- WebGL2 Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext)
- WebGPU Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API)
