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



	/// rendering logic
	// to render to your screen you will need to get the current texture from the WebGPU context.
	// Add the followig to the end of your render loop to ensure it gets displayed.

	const texture = context.getCurrentTexture();

	context.presentSurface(texture);
}
```




## API

- 2D Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
- WebGL Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
- WebGL2 Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext)
- WebGPU Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API)
