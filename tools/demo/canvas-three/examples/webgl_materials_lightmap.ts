import * as THREE from 'three';

//import Stats from 'three/addons/libs/stats.module.js';

import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

import { MeshBasicNodeMaterial, vec4, color, positionLocal, mix } from 'three/examples/jsm/nodes/Nodes';
import { nodeFrame } from 'three/examples/jsm/renderers/webgl/nodes/WebGLNodes.js';

let container; // stats;
let camera, scene, renderer;

const root = '~/assets/three/';

async function init(canvas) {
	const context = canvas.getContext('webgl2') as WebGLRenderingContext;

	const { drawingBufferWidth, drawingBufferHeight } = context;

	// CAMERA

	camera = new THREE.PerspectiveCamera(40, drawingBufferWidth / drawingBufferWidth, 1, 10000);
	camera.position.set(700, 200, -500);

	// SCENE

	scene = new THREE.Scene();

	// LIGHTS

	const light = new THREE.DirectionalLight(0xaabbff, 0.3);
	light.position.x = 300;
	light.position.y = 250;
	light.position.z = -500;
	scene.add(light);

	// SKYDOME

	const topColor = new THREE.Color().copy(light.color).convertSRGBToLinear();
	const bottomColor = new THREE.Color(0xffffff).convertSRGBToLinear();
	const offset = 400;
	const exponent = 0.6;

	const h = positionLocal.add(offset).normalize().y;

	const skyMat = new MeshBasicNodeMaterial();
	skyMat.colorNode = vec4(mix(color(bottomColor), color(topColor), h.max(0.0).pow(exponent)), 1.0);
	skyMat.side = THREE.BackSide;

	const sky = new THREE.Mesh(new THREE.SphereGeometry(4000, 32, 15), skyMat);
	scene.add(sky);

	// RENDERER

	renderer = new THREE.WebGLRenderer({ context, antialias: true });
	renderer.setPixelRatio(window.devicePixelRatio);
	renderer.setSize(drawingBufferWidth, drawingBufferHeight);
	//container.appendChild(renderer.domElement);
	renderer.outputEncoding = THREE.sRGBEncoding;

	// CONTROLS

	const controls = new OrbitControls(camera, canvas);
	controls.maxPolarAngle = (0.9 * Math.PI) / 2;
	controls.enableZoom = false;

	// STATS

	//stats = new Stats();
	//container.appendChild(stats.dom);

	// MODEL

	const loader = new THREE.ObjectLoader();
	const object = await loader.loadAsync(root + 'models/json/lightmap/lightmap.json');

	scene.add(object);

	//

	window.addEventListener('resize', onWindowResize);
}

function onWindowResize() {
	camera.aspect = window.innerWidth / window.innerHeight;
	camera.updateProjectionMatrix();

	renderer.setSize(window.innerWidth, window.innerHeight);
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
