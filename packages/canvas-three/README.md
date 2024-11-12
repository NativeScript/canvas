# @nativescript/canvas-three

## Installation

```bash
npm i three @nativescript/canvas-three
```

## Usage

```js
import * as THREE from '@nativescript/canvas-three';
```

## Creating a Renderer

#### Using WebGPU

Add the following to the webpackconfig.js

```js
const webpack = require('@nativescript/webpack');
const { resolve } = require('path');

module.exports = (env) => {
	....
  /// the path depends on the location of node_modules in this example it's in the root of the plugin repo in a simple project it would be './node_modules/three/build.three.webgpu.js' and not '../../node_modules/three/build.three.webgpu.js'
	webpack.chainWebpack((config) => {
    config.resolve.alias.set('three', resolve(__dirname, '..', '..', 'node_modules', 'three', 'build', 'three.webgpu.js'));
	});
```

```ts
const renderer = new THREE.WebGPURenderer({ canvas, antialias: false });
renderer.setPixelRatio(window.devicePixelRatio);
renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);
await renderer.init();
// grab the webgpu context as it's needed to present/render to the screen this is currently needed for now until it's removed in the future
const context = canvas.getContext('webgpu');
/// at the end of your render loop add the following
// context.presentSurface();
// e.g

function render() {
	renderer.render(scene, camera);
	context.presentSurface();
}
```

####

Given a `gl (context)` from a
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
	camera = new THREE.PerspectiveCamera(70, canvas.clientWidth / canvas.clientHeight, 0.01, 10);
	camera.position.z = 1;

	scene = new THREE.Scene();

	geometry = new THREE.BoxGeometry(0.2, 0.2, 0.2);
	material = new THREE.MeshNormalMaterial();

	mesh = new THREE.Mesh(geometry, material);
	scene.add(mesh);

	const renderer = new THREE.WebGLRenderer({ canvas, antialias: false });
	renderer.setPixelRatio(window.devicePixelRatio);
	renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);
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
