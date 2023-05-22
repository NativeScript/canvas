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

<!-- You also need to install the following 2 plugins:

- `ns plugin add @nativescript/canvas-polyfill`

- `ns plugin add @nativescript/canvas-media` -->

## How to use @nativescript/canvas 

The following sections describe how to use the plugin in different frameworks.

### Using @nativescript/canvas in NativeScript Core

1. Register the plugin

To register the plugin, use the Page view's `xmlns` attribute to declare the plugin namespace under a prefix, as follows:

```xml
<Page xmlns:canvas="@nativescript/canvas">
...
</Page>
```

2. Use the Canvas view

Next, use the prefix(`canvas`) to access the Canvas view in the page
```xml
<canvas:Canvas id="canvas" width="100%" height="100%" ready="canvasReady"/>
```

#### 2D rendering context

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

#### WEBGL rendering context

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

<!-- ### Using @nativescript/canvas in NativeScript Angular

1. Register the plugin

2. Add the Canvas view to your page -->
### Using @nativescript/canvas in NativeScript Vue

1. Register the plugin

In the `app.ts` file, add the following code:

```ts
import { registerElement } from 'nativescript-vue'

registerElement(
  'Canvas',
  () => require('@nativescript/canvas').Canvas
)

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
<script lang="ts" setup>
import { Canvas } from '@nativescript/canvas';
import { EventData } from '@nativescript/core';
let ctx;
let canvas;

function canvasReady(args: EventData) {
      console.log('canvas ready');
      canvas = args.object as Canvas;
      console.log(canvas);
      ctx = (<any>canvas.getContext('2d')) as CanvasRenderingContext2D;

      ctx.fillStyle = 'green';
      ctx.fillRect(10, 10, 150, 100);
    }
</script>
```

### Using @nativescript/canvas in NativeScript Svelte

1. Register the plugin

In the `app.ts` file, register the plugin as follows:

```ts
import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement('canvas', () => require('@nativescript/canvas').Canvas)
```

2. Then, in a `.svelte` file, add use the Canvas view as follows:


```svelte
<script lang="ts">
  let ctx;
  let canvas;
  function canvasReady(args: any) {
    console.log("canvas ready");
    canvas = args.object;
    console.log(canvas);
    ctx = canvas.getContext("2d");
    ctx.fillStyle = "green";
    ctx.fillRect(10, 10, 150, 100);
    ctx.fillStyle = "red";
    ctx.fillRect(200, 10, 150, 100);
  }
</script>

<page>
  <actionBar title="Canvas" />
  <gridLayout rows="auto, *">
    <canvas width="100%" height="100%" on:ready={canvasReady} />
  </gridLayout>
</page>
```
## API
### Canvas class

#### getContext()
```ts
context : CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null = canvas.getContext(type, options)
```

| Param | Type | Description |
| --- | --- | --- |
| `type` | `string` | The context type. Can be either `2d`, `webgl` or `webgl2` |
| `options?` | `any` | The context options. |

---
#### toDataURL()
```ts
data : any = canvas.toDataURL(type, encoderOptions)
```

| Param | Type | Description |
| --- | --- | --- |
| `type?` | `string` | 
| `encoderOptions?` | `number` |

---
#### getBoundingClientRect()
```ts
rect : {x: number; y: number;width: number;height: number;top: number;right: number;bottom: number;left: number;} = canvas.getBoundingClientRect()
```

---

#### flush()
```ts
canvas.flush()
```

---
- 2D Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
- WebGL Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
- WebGL2 Similar to -> the [Web Spec](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext)
