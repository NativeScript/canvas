import * as THREE from 'three/webgpu';
import { RGBELoader } from 'three/examples/jsm/loaders/RGBELoader';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

export async function run(canvas) {
	var container, controls, context, width, height;
	var camera, scene, renderer;
	var mouseX = 0,
		mouseY = 0,
		windowHalfX = 0,
		windowHalfY = 0;
	// THREE.RGBELoader: unsupported type:  1009
	const init = async () => {
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;
		width = canvas.width;
		height = canvas.height;
		camera = new THREE.PerspectiveCamera(45, canvas.clientWidth / canvas.clientHeight, 0.25, 20);
		camera.position.set(-1.8, 0.6, 2.7);
		scene = new THREE.Scene();
		const light = new THREE.SpotLight();
		light.position.set(-1.8, 0.6, 2.7);
		scene.add(light);

		// global.parent = window.parent = window;

		new RGBELoader().setPath('https://threejs.org/examples/textures/equirectangular/').load('royal_esplanade_1k.hdr', function (texture) {
			texture.mapping = THREE.EquirectangularReflectionMapping;
			scene.background = texture;
			scene.environment = texture;

			context?.presentSurface?.();

			new GLTFLoader().setPath('https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/').load('DamagedHelmet.gltf', (gltf) => {
				scene.add(gltf.scene);
				animate();
			});
		});

		renderer = new THREE.WebGPURenderer({ canvas, antialias: true });
		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);
		await renderer.init();

		renderer.toneMapping = THREE.ACESFilmicToneMapping;

		controls = new OrbitControls(camera, canvas);

		canvas.addEventListener('change', render);
		controls.minDistance = 2;
		controls.maxDistance = 10;
		controls.target.set(0, 0, -0.2);
		controls.update();

		context = canvas.getContext('webgpu');
		//	context = canvas.getContext('webgl2');

		//	onWindowResize();
		window.addEventListener('resize', onWindowResize, false);
	};

	function onWindowResize() {
		const width = canvas.width;
		const height = canvas.height;
		camera.aspect = width / height;
		camera.updateProjectionMatrix();
		renderer.setSize(width, height);
		render();
	}

	//

	function render() {
		renderer.render(scene, camera);
		context.presentSurface();
	}

	function animate() {
		render();
		//stats.update();
		requestAnimationFrame(animate);
	}

	init();
}
