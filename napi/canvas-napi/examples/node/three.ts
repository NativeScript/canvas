import * as THREE from 'three';
import * as THREEWebGPU from 'three/webgpu';

async function webgpuCube(canvas) {
	// const adapter = await navigator.gpu?.requestAdapter();
	// const device: GPUDevice = (await adapter?.requestDevice()) as never;

	var camera, scene, renderer;
	var geometry, material, mesh;
	var context;

	function animate() {
		mesh.rotation.x += 0.01;
		mesh.rotation.y += 0.02;

		renderer.render(scene, camera);

		context.presentSurface();
	}

	async function init() {
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;

		const innerWidth = canvas.clientWidth;
		const innerHeight = canvas.clientHeight;

		camera = new THREEWebGPU.PerspectiveCamera(50, innerWidth / innerHeight, 0.1, 10);
		camera.position.z = 1;

		scene = new THREEWebGPU.Scene();

		geometry = new THREEWebGPU.BoxGeometry(0.2, 0.2, 0.2);
		material = new THREEWebGPU.MeshNormalMaterial();

		mesh = new THREEWebGPU.Mesh(geometry, material);
		scene.add(mesh);

		renderer = new THREEWebGPU.WebGPURenderer({ canvas });

		await renderer.init();
		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);

		context = canvas.getContext('webgpu');
		renderer.setAnimationLoop(animate);
	}

	init();
}

function cube(canvas) {
	// const adapter = await navigator.gpu?.requestAdapter();
	// const device: GPUDevice = (await adapter?.requestDevice()) as never;

	var camera, scene, renderer;
	var geometry, material, mesh;
	var context;

	function animate() {
		mesh.rotation.x += 0.01;
		mesh.rotation.y += 0.02;

		renderer.render(scene, camera);

		context.render();
	}

	function init() {
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;
		const innerWidth = canvas.clientWidth;
		const innerHeight = canvas.clientHeight;

		camera = new THREE.PerspectiveCamera(50, innerWidth / innerHeight, 0.1, 10);
		camera.position.z = 1;

		scene = new THREE.Scene();

		geometry = new THREE.BoxGeometry(0.2, 0.2, 0.2);
		material = new THREE.MeshNormalMaterial();

		mesh = new THREE.Mesh(geometry, material);
		scene.add(mesh);

		renderer = new THREE.WebGLRenderer({ canvas });

		context = canvas.getContext('webgl2');

		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);

		renderer.setAnimationLoop(animate);
	}

	init();
}

export default {
	webgpuCube,
	cube,
};
