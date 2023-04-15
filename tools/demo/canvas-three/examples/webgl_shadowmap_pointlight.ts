import * as THREE from 'three';

import Stats from 'three/examples/jsm/libs/stats.module.js';

import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

let camera, scene, renderer; //stats;
let pointLight, pointLight2;

let context;

function init(canvas) {
	context = canvas.getContext('webgl2') as WebGLRenderingContext;

	const { drawingBufferWidth, drawingBufferHeight } = context;

	camera = new THREE.PerspectiveCamera(45, drawingBufferWidth / drawingBufferHeight, 1, 1000);
	camera.position.set(0, 10, 40);

	scene = new THREE.Scene();
	scene.add(new THREE.AmbientLight(0x111122));

	// lights

	function createLight(color) {
		const intensity = 2;

		const light = new THREE.PointLight(color, intensity, 20);
		light.castShadow = true;
		light.shadow.bias = -0.005; // reduces self-shadowing on double-sided objects

		let geometry = new THREE.SphereGeometry(0.3, 12, 6);
		let material = new THREE.MeshBasicMaterial({ color: color });
		material.color.multiplyScalar(intensity);
		let sphere = new THREE.Mesh(geometry, material);
		light.add(sphere);

		const texture = new THREE.CanvasTexture(generateTexture());
		texture.magFilter = THREE.NearestFilter;
		texture.wrapT = THREE.RepeatWrapping;
		texture.wrapS = THREE.RepeatWrapping;
		texture.repeat.set(1, 4.5);

		geometry = new THREE.SphereGeometry(2, 32, 8);
		material = new THREE.MeshPhongMaterial({
			side: THREE.DoubleSide,
			alphaMap: texture,
			alphaTest: 0.5,
		});

		sphere = new THREE.Mesh(geometry, material);
		sphere.castShadow = true;
		sphere.receiveShadow = true;
		light.add(sphere);

		return light;
	}

	pointLight = createLight(0x0088ff);
	scene.add(pointLight);

	pointLight2 = createLight(0xff8888);
	scene.add(pointLight2);
	//

	const geometry = new THREE.BoxGeometry(30, 30, 30);

	const material = new THREE.MeshPhongMaterial({
		color: 0xa0adaf,
		shininess: 10,
		specular: 0x111111,
		side: THREE.BackSide,
	});

	const mesh = new THREE.Mesh(geometry, material);
	mesh.position.y = 10;
	mesh.receiveShadow = true;
	scene.add(mesh);

	//

	renderer = new THREE.WebGLRenderer({ context, antialias: true });
	renderer.setPixelRatio(window.devicePixelRatio);
	renderer.setSize(window.innerWidth, window.innerHeight);
	renderer.shadowMap.enabled = true;
	renderer.shadowMap.type = THREE.BasicShadowMap;
	//document.body.appendChild(renderer.domElement);

	const controls = new OrbitControls(camera, canvas);
	controls.target.set(0, 10, 0);
	controls.update();

	//stats = new Stats();
	//document.body.appendChild(stats.dom);

	//

	window.addEventListener('resize', onWindowResize);
}

function onWindowResize() {
	const width = context.drawingBufferWidth; //window.innerWidth;
	const height = context.drawingBufferHeight; //window.innerHeight;

	camera.aspect = width / height;

	camera.updateProjectionMatrix();

	renderer.setSize(width, height);
}

function generateTexture() {
	const canvas = document.createElement('canvas');
	canvas.width = 2;
	canvas.height = 2;

	const context = canvas.getContext('2d');
	context.fillStyle = 'white';
	context.fillRect(0, 1, 2, 1);

	return canvas;
}

function animate() {
	requestAnimationFrame(animate);
	render();
}

function render() {
	let time = performance.now() * 0.001;

	pointLight.position.x = Math.sin(time * 0.6) * 9;
	pointLight.position.y = Math.sin(time * 0.7) * 9 + 6;
	pointLight.position.z = Math.sin(time * 0.8) * 9;

	pointLight.rotation.x = time;
	pointLight.rotation.z = time;

	time += 10000;

	pointLight2.position.x = Math.sin(time * 0.6) * 9;
	pointLight2.position.y = Math.sin(time * 0.7) * 9 + 6;
	pointLight2.position.z = Math.sin(time * 0.8) * 9;

	pointLight2.rotation.x = time;
	pointLight2.rotation.z = time;

	renderer.render(scene, camera);

	//  stats.update();
}

export function webgl_shadowmap_pointlight(canvas) {
	init(canvas);
	animate();
}
