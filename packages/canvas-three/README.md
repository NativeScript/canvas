# @nativescript/canvas-three

## Installation

```bash
npm i three @nativescript/canvas-three
```

## Usage

```js
import TNSTHREE from '@nativescript/canvas-three';
```

## Creating a Renderer

#### `TNSTHREE.Renderer({ gl: WebGLRenderingContext, width: number, height: number, pixelRatio: number, ...extras })`

Given a `gl (context)` from an
[`Canvas`](https://github.com/nativescript/canvas), return a
[`THREE.WebGLRenderer`](https://threejs.org/docs/#api/renderers/WebGLRenderer)
that draws into it.

```ts
import * as THREE from 'three';

var camera, scene, renderer;
var geometry, material, mesh;

canvas; // Canvas instance
init();
animate();

function init() {
  const context = canvas.getContext('webgl');

  const { drawingBufferWidth: width, drawingBufferHeight: height } = context;
  camera = new THREE.PerspectiveCamera(70, width / height, 0.01, 10);
  camera.position.z = 1;

  scene = new THREE.Scene();

  geometry = new THREE.BoxGeometry(0.2, 0.2, 0.2);
  material = new THREE.MeshNormalMaterial();

  mesh = new THREE.Mesh(geometry, material);
  scene.add(mesh);

  renderer = new THREE.WebGLRenderer({ context });
  renderer.setSize(width, height);
}

function animate() {
  requestAnimationFrame(animate);

  mesh.rotation.x += 0.01;
  mesh.rotation.y += 0.02;

  renderer.render(scene, camera);
}
```

## License

Apache License Version 2.0, January 2004
