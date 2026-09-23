# @nativescript/canvas-svg

Renders SVG natively on iOS and Android with Skia, including SMIL and CSS animations and a live DOM you can script.

```bash
npm install @nativescript/canvas-svg
```

## Usage

### From a source

`src` accepts an app relative path, an absolute path, a URL, or inline markup.

```xml
<Page xmlns:svg="@nativescript/canvas-svg">
  <svg:Svg src="~/assets/rocket.svg" width="100%" height="100%" />
</Page>
```

```ts
import { Svg } from '@nativescript/canvas-svg';

const view = new Svg();
view.src = '~/assets/rocket.svg';
```

Animations in the source start as soon as the view is loaded and pause while it is off screen.

### Building it with the DOM

With `@nativescript/canvas-polyfill` installed you can build and change an SVG the same way you would in a browser. The `nativeElement` of the root `<svg>` is the view to add to your layout.

```ts
require('@nativescript/canvas-polyfill');

const NS = 'http://www.w3.org/2000/svg';
const svg = document.createElementNS(NS, 'svg');
svg.setAttribute('width', '150');
svg.setAttribute('height', '150');

const circle = document.createElementNS(NS, 'circle');
circle.setAttribute('cx', '75');
circle.setAttribute('cy', '75');
circle.setAttribute('r', '40');
circle.setAttribute('fill', 'gold');
svg.appendChild(circle);

const label = document.createElementNS(NS, 'text');
label.setAttribute('x', '10');
label.setAttribute('y', '140');
label.textContent = 'Hello';
svg.appendChild(label);

layout.addChild(svg.nativeElement);

// Later changes are batched into a single redraw on the next frame.
circle.setAttribute('fill', 'orange');
label.textContent = 'Updated';
```

Supported on elements: `setAttribute`, `getAttribute`, `appendChild`, `append`, `removeChild` and `textContent`. Animating by changing attributes in a `requestAnimationFrame` loop works too.

## Properties

| Property | Default | Description |
| --- | --- | --- |
| `src` | | Path, URL or inline markup to render. |
| `gpu` | `true` | Rasterize on the GPU (Metal on iOS, Vulkan or GL on Android). Falls back to the CPU automatically if no GPU context can be created. |
| `threaded` | `false` | Rasterize on a background thread. The frame is recorded on the UI thread and drawn elsewhere, so a heavy SVG no longer holds up the rest of the UI. |
| `shareSrc` | `true` | Views with the same `src` share one parsed document, one animation clock and one recording per frame, like `<img>` tags pointing at the same file on the web. Shared copies animate in step. Set it to `false` to give a view its own copy. |
| `backend` | `auto` | Force `gl`, `vulkan` or `metal`. Only needed to work around a driver problem. |
| `surfaceType` | `texture` | Android only. `texture` behaves like a normal view. `surface` is faster to composite but cannot be transformed or overlapped. |

## Events

| Event | When |
| --- | --- |
| `animationEnd` | Every animation in the document has finished. Never fires for one that repeats forever. |
| `contextLost` | The GPU context could not be rebuilt and drawing has moved to the CPU. |
| `contextRestored` | A lost GPU context was rebuilt and GPU drawing has resumed. |

```ts
view.on('animationEnd', () => console.log('done'));
```

## Animation

Skia's SVG renderer does not animate, so this plugin includes its own animation engine. Both common export formats work.

**SMIL:** `<animate>`, `<animateTransform>`, `<animateMotion>` (including `rotate="auto"`) and `<set>`, with `values`, `from`/`to`/`by`, `keyTimes`, `keySplines`, `calcMode`, `repeatCount`, `fill="freeze"`, `additive` and `accumulate`. A `begin` that waits for an event (such as `click`) does not start on its own.

**CSS:** `@keyframes` in a `<style>` block, applied through the `animation` shorthand or its longhands (duration, delay, iteration count, direction, fill mode and timing function). Keyword and `cubic-bezier()` easings are supported; `steps()` runs as linear. Only `#id` selectors are read, which is what SVG exporters produce.

Only changes that are actually visible trigger a redraw, so an animation that is holding still costs nothing.

## Rendering a source once

To get pixels without a view, use `Svg.fromSrc` (async) or `Svg.fromSrcSync`. Both return an `SvgData` with `width`, `height` and the RGBA `data`.

```ts
const data = await Svg.fromSrc('~/assets/icon.svg');
```

## Limitations

* A document loaded from `src` is not scriptable. That is what lets views share it.
* Text elements loaded from a source only report and replace text that was added through the DOM.

## License

Apache License Version 2.0
