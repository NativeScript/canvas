# NativeScript Canvas

## Introduction

This plugin provides a canvas view for NativeScript applications. It is powered by the following libraries:

- [CanvasNative](src-native/canvas-native) - Rust
- [CanvasNative](src-native/canvas-ios) - IOS
- [CanvasNative](src-native/canvas-android) - Android

>**Minimum version supported:** <br> - `iOS`: `11` <br>  - `Android`: `API 17`


## Installation

To install the plugin, run the following command from the root of your project:

```bash
ns plugin add @nativescript/canvas
```

You also need to install the following 2 plugins:

- `ns plugin add @nativescript/canvas-polyfill`

- `ns plugin add @nativescript/canvas-media`

## How to use @nativescript/canvas 

The following sections describe how to use the plugin in different frameworks.

### Using @nativescript/canvas in NativeScript Core

1. Register the plugin

2. Add the Canvas view to your page

### Using @nativescript/canvas in NativeScript Angular

1. Register the plugin

2. Add the Canvas view to your page
### Using @nativescript/canvas in NativeScript Vue

1. Register the plugin

2. Add the Canvas view to your page

### Using @nativescript/canvas in NativeScript Svelte

1. Register the plugin

2. Add the Canvas view to your page


IMPORTANT: ensure you include xmlns:canvas="@nativescript/canvas" on the Page element for core {N}

# Usage

```xml
<canvas:Canvas id="canvas" width="100%" height="100%" ready="canvasReady"/>
```
## Vanilla
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
## Vue
In `app.ts`, add:

```
Vue.registerElement("Canvas", ()=>require("@nativescript/canvas").Canvas)
```
Then in a `.vue` file:

```xml
<template>
  <Page>
    <ActionBar>
      <Label text="Home" />
    </ActionBar>

    <GridLayout>
      <Canvas id="canvas" width="100%" height="100%" @ready="canvasReady" />
    </GridLayout>
  </Page>
</template>
```
```ts
let ctx;
let canvas;
export default Vue.extend({
  methods: {

    canvasReady(args: EventData) {
      console.log('canvas ready');
      canvas = args.object as Canvas;
      console.log(canvas);
      ctx = (<any>canvas.getContext('2d')) as CanvasRenderingContext2D;

      ctx.fillStyle = 'green';
      ctx.fillRect(10, 10, 150, 100);
    }
  }
```

## API
### Canvas class

|Method| Return Type| Description|
|------|------------|------------|
|`getContext(type: string, options?: any)`|`CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null`||
|`toDataURL(type?: string, encoderOptions?: number)`| `any`||
|`getBoundingClientRect()`| `{x: number; y: number;width: number;height: number;top: number;right: number;bottom: number;left: number;}`||
- 2D Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
- WebGL Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
- WebGL2 Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext)
