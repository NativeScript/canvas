import * as THREE from 'three/webgpu';
import { RGBELoader } from 'three/examples/jsm/loaders/RGBELoader';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

/*
function getContentType(filePath) {
	const ext = filePath.split('.').pop();
	switch (ext) {
		case 'txt':
			return 'text/plain';
		case 'html':
			return 'text/html';
		case 'json':
			return 'application/json';
		case 'png':
			return 'image/png';
		case 'jpg':
		case 'jpeg':
			return 'image/jpeg';
		default:
			return 'application/octet-stream';
	}
}

	globalThis.__fetch = globalThis.fetch;
	globalThis.fetch = function(resource, options) {

		let url;

		// Normalize `resource` to a URL instance
		if (typeof resource === 'string' || resource instanceof URL) {
			url = new URL(resource.toString());
		} else if (resource instanceof Request) {
			url = new URL(resource.url);
		} else {
			throw new TypeError('Invalid resource type for fetch.');
		}

		if (url.protocol === 'file:') {
			const filePath = decodeURIComponent(url.pathname); // Decode URL-encoded characters
			return fs.readFile(filePath).then((buffer) => {
				// Create a Readable stream from the buffer
				const readable = Readable.from(buffer);
				const body = Readable.toWeb(readable);

				// Simulate a Response object
				const init = {
					status: 200,
					statusText: 'OK',
					headers: {
						'Content-Type': getContentType(filePath)
					}
				};

				console.log(init);

				return new Response(body, init);
			});
		}

		return globalThis.__fetch(resource, options);
	}

	*/

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

		const royal_esplanade_1k = import.meta.resolve('./textures/equirectangular/royal_esplanade_1k.hdr');
		const damaged_helmet = import.meta.resolve('./models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf');

		new RGBELoader().setPath('https://threejs.org/examples/textures/equirectangular/').load('royal_esplanade_1k.hdr', function (texture) {
			texture.mapping = THREE.EquirectangularReflectionMapping;

			scene.background = texture;
			scene.environment = texture;

			context?.presentSurface?.();

			var loader = new GLTFLoader().setPath('https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/').load('DamagedHelmet.gltf', (gltf) => {
				console.log('loaded');
				scene.add(gltf.scene);
				animate();
			});
		});

		/*	const path = import.meta.resolve('./models/gltf/DamagedHelmet/glTF/').replace('index.js', '');
			console.log(path);
			fs.readFile(damaged_helmet.replace('file://', '')).then(buffer => {
				loader.parse(buffer.buffer, path, function(gltf) {
					console.log('parse');
					scene.add(gltf.scene);
					animate();
				});
			});

			*/

		renderer = new THREE.WebGPURenderer({ canvas, antialias: true });
		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);
		await renderer.init();

		// renderer.toneMapping = THREE.ACESFilmicToneMapping;
		// renderer.toneMappingExposure = 1;

		// var pmremGenerator = new THREE.PMREMGenerator(renderer);
		// pmremGenerator.compileEquirectangularShader();

		controls = new OrbitControls(camera, canvas);

		canvas.addEventListener('change', render);
		controls.minDistance = 2;
		controls.maxDistance = 10;
		controls.target.set(0, 0, -0.2);
		controls.update();

		context = canvas.getContext('webgpu');

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
