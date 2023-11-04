import * as THREE from 'three';

//import Stats from 'three/addons/libs/stats.module.js';

import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

import { MeshBasicNodeMaterial, vec4, color, positionLocal, mix } from 'three/examples/jsm/nodes/Nodes';
import { nodeFrame } from 'three/examples/jsm/renderers/webgl-legacy/nodes/WebGLNodes.js';

let container; // stats;
let camera, scene, renderer;

const root = '~/assets/three/';
let context;
async function init(canvas) {
	context = canvas.getContext('webgl2') as WebGLRenderingContext;

	const { drawingBufferWidth, drawingBufferHeight } = context;

	camera = new THREE.PerspectiveCamera(40, drawingBufferWidth / drawingBufferHeight, 1, 10000);
	camera.position.set(700, 200, -500);

	// SCENE

	scene = new THREE.Scene();

	// LIGHTS

	const light = new THREE.DirectionalLight(0xd5deff);
	light.position.x = 300;
	light.position.y = 250;
	light.position.z = -500;
	scene.add(light);

	// SKYDOME

	const topColor = new THREE.Color().copy(light.color);
	const bottomColor = new THREE.Color(0xffffff);
	const offset = 400;
	const exponent = 0.6;

	const h = (positionLocal as any).add(offset).normalize().y;

	const skyMat = new MeshBasicNodeMaterial();
	skyMat.colorNode = vec4(mix(color(bottomColor), color(topColor), h.max(0.0).pow(exponent)), 1.0);
	skyMat.side = THREE.BackSide;

	const sky = new THREE.Mesh(new THREE.SphereGeometry(4000, 32, 15), skyMat);
	scene.add(sky);

	// RENDERER                                                                                                         

	renderer = new THREE.WebGLRenderer({context, antialias: true });
	renderer.setPixelRatio(window.devicePixelRatio);
	renderer.setSize(drawingBufferWidth, drawingBufferHeight);

	// CONTROLS

	const controls = new OrbitControls(camera, renderer.domElement);
	controls.maxPolarAngle = (0.9 * Math.PI) / 2;
	controls.enableZoom = false;

	// MODEL

	const loader = new THREE.ObjectLoader();
	const object = await loader.loadAsync('~/assets/three/models/json/lightmap/lightmap.json');
	scene.add(object);

	//

	window.addEventListener('resize', onWindowResize);
}

function onWindowResize() {
	const { drawingBufferWidth, drawingBufferHeight } = context;

	camera.aspect = drawingBufferWidth / drawingBufferHeight;
	camera.updateProjectionMatrix();

	renderer.setSize(drawingBufferWidth, drawingBufferHeight);
}

//

function animate() {
	requestAnimationFrame(animate);

	nodeFrame.update();

	renderer.render(scene, camera);
	//stats.update();
}

export function webgl_materials_lightmap(canvas) {
	init(canvas).then(animate);
}
