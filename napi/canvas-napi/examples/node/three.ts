let isAvailable = typeof navigator !== 'undefined' && navigator.gpu !== undefined;
console.log('isAvailable', isAvailable);

import * as THREE from 'three/webgpu';
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

		console.log('animate');

		context.presentSurface();
	}

	async function init() {
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

		renderer = new THREE.WebGPURenderer({ canvas });

		await renderer.init();
		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);

		console.log(canvas.getContext('webgpu'), canvas.getContext('webgl'), canvas.getContext('webgl2'));

		context = canvas.getContext('webgpu');
		renderer.setAnimationLoop(animate);
	}

	init();
}

export default {
	webgpuCube,
};
