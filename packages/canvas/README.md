# NativeScript Canvas

**Powered by**

- [CanvasNative](src-native/canvas-native) - Rust
- [CanvasNative](src-native/canvas-ios) - IOS
- [CanvasNative](src-native/canvas-android) - Android

## Installation

```bash
ns plugin add @nativescript/canvas
```

_Note_ min ios support 11 | min android support 17

IMPORTANT: ensure you include xmlns:canvas="@nativescript/canvas" on the Page element for core {N}

## Usage

```xml
<canvas:Canvas id="canvas" width="100%" height="100%" ready="canvasReady"/>
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

## API

- 2D Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
- WebGL Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
- WebGL2 Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext)
