import { DemoSharedBase } from '../utils';
import * as THREE from 'three';
import { TDSLoader } from 'three/examples/jsm/loaders/TDSLoader';
import { RGBELoader } from 'three/examples/jsm/loaders/RGBELoader';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader';
import { RoughnessMipmapper } from 'three/examples/jsm/utils/RoughnessMipmapper';
import { FBXLoader } from 'three/examples/jsm/loaders/FBXLoader';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
import { VertexNormalsHelper } from 'three/examples/jsm/helpers/VertexNormalsHelper';
import { VertexTangentsHelper } from 'three/examples/jsm/helpers/VertexTangentsHelper';
import { DRACOLoader } from './custom/DRACOLoader';
//import {DRACOLoader} from 'three/examples/jsm/loaders/DRACOLoader';
import { FirstPersonControls } from 'three/examples/jsm/controls/FirstPersonControls';
import { Points } from 'three';
import { Water } from 'three/examples/jsm/objects/Water';
import { Sky } from 'three/examples/jsm/objects/Sky';
//import { BufferGeometryUtils } from 'three/examples/jsm/utils/BufferGeometryUtils';
import { RoomEnvironment } from 'three/examples/jsm/environments/RoomEnvironment';
import { GPUComputationRenderer } from 'three/examples/jsm/misc/GPUComputationRenderer';
// import {ThreeMFLoader} from "three/examples/jsm/loaders/3MFLoader";
class IconMesh extends THREE.Mesh {
	constructor() {
		super(new THREE.BoxBufferGeometry(5.0, 5.0, 5.0), new THREE.MeshNormalMaterial());
	}
}

global.console.warn = () => {};

export class DemoSharedCanvasThree extends DemoSharedBase {
	canvas: any;
	root = '~/assets/three';

	loaded(args) {
		console.log('loaded', args.object);
	}

	unloaded(args) {
		console.log('unloaded');
	}

	canvasLoaded(args) {
		this.canvas = args.object;
		// (canvas as any).scaleX = -1;
		//this.group(this.canvas);
		//this.geoTextShapes(this.canvas);
		//this.geoColors(this.canvas);
		//this.threeDepth(this.canvas);
		//this.threeCrate(this.canvas);
		//this.skinningAndMorphing(this.canvas);
		//this.nearestNeighbour(this.canvas);
		//this.threeOcean(this.canvas);
		//this.threeCube(this.canvas);
		//this.threeCar(this.canvas);
		this.threeKeyframes(this.canvas);
		//this.webGLHelpers(this.canvas);
		//this.fbxLoader(this.canvas);
		//this.gtlfLoader(this.canvas);
		//this.rayCasting(this.canvas);
		//this.ThreeDS(this.canvas);
		//this.ThreeMF(this.canvas);
		//this.gtlfTonemapping(this.canvas);
		//this.bufferGeo(this.canvas);
		//this.birds(this.canvas);
		//this.renderVideo();
		//this.webgl_buffergeometry_drawrange(this.canvas);
	}

	renderVideo() {
		const ctx = this.canvas.getContext('webgl2');
		const video = document.createElement('video');
		video.loop = true;
		video.autoplay = true;
		video.src = 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4';
		function update() {
			//@ts-ignore
			video.requestVideoFrameCallback(update);
			//@ts-ignore
			video._video.getCurrentFrame(ctx.native);
		}
		//@ts-ignore
		video.requestVideoFrameCallback(update);
	}

	gtlfLoader(canvas) {
		var container, controls, context, width, height;
		var camera, scene, renderer;
		var mouseX = 0,
			mouseY = 0,
			windowHalfX = 0,
			windowHalfY = 0;

		const init = () => {
			context = canvas.getContext('webgl');
			width = context.drawingBufferWidth;
			height = context.drawingBufferHeight;
			camera = new THREE.PerspectiveCamera(75, width / height, 0.25, 20);
			camera.position.set(-1.8, 0.6, 2.7 * 1.2);
			scene = new THREE.Scene();
			const light = new THREE.SpotLight();
			light.position.set(-1.8, 0.6, 2.7 * 1.2);
			scene.add(light);
			new RGBELoader()
				.setDataType(THREE.UnsignedByteType)
				.setPath(this.root + '/textures/equirectangular/')
				.load('royal_esplanade_1k.hdr', (texture) => {
					var envMap = pmremGenerator.fromEquirectangular(texture).texture;

					scene.background = envMap;
					scene.environment = envMap;

					texture.dispose();
					pmremGenerator.dispose();

					render();

					// model

					// use of RoughnessMipmapper is optional
					var roughnessMipmapper = new RoughnessMipmapper(renderer);

					var loader = new GLTFLoader().setPath(this.root + '/models/gltf/DamagedHelmet/glTF/');
					loader.load('DamagedHelmet.gltf', function (gltf) {
						gltf.scene.traverse(function (child) {
							// @ts-ignore
							if (child.isMesh) {
								// TOFIX RoughnessMipmapper seems to be broken with WebGL 2.0
								// roughnessMipmapper.generateMipmaps( child.material );
							}
						});

						scene.add(gltf.scene);

						roughnessMipmapper.dispose();

						render();
					});
				});

			renderer = new THREE.WebGLRenderer({ context, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(width, height);
			renderer.toneMapping = THREE.ACESFilmicToneMapping;
			renderer.toneMappingExposure = 1;
			renderer.outputEncoding = THREE.sRGBEncoding;

			var pmremGenerator = new THREE.PMREMGenerator(renderer);
			pmremGenerator.compileEquirectangularShader();

			controls = new OrbitControls(camera, canvas);

			canvas.addEventListener('change', render);
			controls.minDistance = 2;
			controls.maxDistance = 10;
			controls.target.set(0, 0, -0.2);
			controls.update();

			onWindowResize();
			window.addEventListener('resize', onWindowResize, false);
		};

		function onWindowResize() {
			const width = context.drawingBufferWidth;
			const height = context.drawingBufferHeight;
			camera.aspect = width / height;
			camera.updateProjectionMatrix();

			renderer.setSize(width, height);

			render();
		}

		//

		function render() {
			renderer.render(scene, camera);
		}

		function animate() {
			requestAnimationFrame(animate);

			render();
			//stats.update();
		}

		init();
		animate();
	}

	gtlfTonemapping(canvas) {
		var mesh, renderer, scene, camera, controls;
		var gui,
			guiExposure = null;

		var params = {
			exposure: 1.0,
			toneMapping: 'ACESFilmic',
		};

		var toneMappingOptions = {
			None: THREE.NoToneMapping,
			Linear: THREE.LinearToneMapping,
			Reinhard: THREE.ReinhardToneMapping,
			Cineon: THREE.CineonToneMapping,
			ACESFilmic: THREE.ACESFilmicToneMapping,
			Custom: (THREE as any).CustomToneMapping,
		};

		const init = async () => {
			const context = canvas.getContext('webgl2');
			renderer = new THREE.WebGLRenderer({ context, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);

			renderer.toneMapping = toneMappingOptions[params.toneMapping];
			renderer.toneMappingExposure = params.exposure;

			renderer.outputEncoding = THREE.sRGBEncoding;

			// Set CustomToneMapping to Uncharted2
			// source: http://filmicworlds.com/blog/filmic-tonemapping-operators/

			THREE.ShaderChunk.tonemapping_pars_fragment = THREE.ShaderChunk.tonemapping_pars_fragment.replace(
				'vec3 CustomToneMapping( vec3 color ) { return color; }',
				`#define Uncharted2Helper( x ) max( ( ( x * ( 0.15 * x + 0.10 * 0.50 ) + 0.20 * 0.02 ) / ( x * ( 0.15 * x + 0.50 ) + 0.20 * 0.30 ) ) - 0.02 / 0.30, vec3( 0.0 ) )
					float toneMappingWhitePoint = 1.0;
					vec3 CustomToneMapping( vec3 color ) {
						color *= toneMappingExposure;
						return saturate( Uncharted2Helper( color ) / Uncharted2Helper( vec3( toneMappingWhitePoint ) ) );
					}`
			);

			scene = new THREE.Scene();

			camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.25, 20);
			camera.position.set(-1.8, 0.6, 10);

			controls = new OrbitControls(camera, canvas);
			controls.addEventListener('change', render); // use if there is no animation loop
			controls.enableZoom = false;
			controls.enablePan = false;
			controls.target.set(0, 0, -0.2);
			controls.update();

			var pmremGenerator = new THREE.PMREMGenerator(renderer);
			pmremGenerator.compileEquirectangularShader();

			var rgbeLoader = new RGBELoader().setDataType(THREE.UnsignedByteType).setPath(this.root + '/textures/equirectangular/');

			var gltfLoader = new GLTFLoader().setPath(this.root + '/models/gltf/DamagedHelmet/glTF/');

			var [texture, gltf] = await Promise.all([rgbeLoader.loadAsync('venice_sunset_1k.hdr'), gltfLoader.loadAsync('DamagedHelmet.gltf')]);

			// environment

			var envMap = pmremGenerator.fromEquirectangular(texture).texture;

			scene.background = envMap;
			scene.environment = envMap;

			texture.dispose();
			pmremGenerator.dispose();

			// model

			gltf.scene.traverse(function (child) {
				if (child.isMesh) {
					mesh = child;
					scene.add(mesh);
				}
			});

			render();

			window.addEventListener('resize', onWindowResize, false);
			/*
						gui = new GUI();

						gui.add( params, 'toneMapping', Object.keys( toneMappingOptions ) )

							.onChange( function () {

								updateGUI();

								renderer.toneMapping = toneMappingOptions[ params.toneMapping ];
								mesh.material.needsUpdate = true;
								render();

							} );

						updateGUI();

						gui.open();
						*/
		};

		function updateGUI() {
			if (guiExposure !== null) {
				gui.remove(guiExposure);
				guiExposure = null;
			}

			if (params.toneMapping !== 'None') {
				guiExposure = gui
					.add(params, 'exposure', 0, 2)

					.onChange(function () {
						renderer.toneMappingExposure = params.exposure;
						render();
					});
			}
		}

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;

			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);

			render();
		}

		function render() {
			renderer.render(scene, camera);
		}

		init().catch(function (err) {
			console.error(err);
		});
	}

	group(canvas) {
		let camera, scene, renderer, mesh;
		const context = canvas.getContext('webgl2');

		camera = new THREE.PerspectiveCamera(70, context.drawingBufferWidth / context.drawingBufferHeight, 0.01, 1000);
		camera.position.z = 50;

		scene = new THREE.Scene();

		const ring = new THREE.TorusBufferGeometry(5, 0.5, 64, 64);
		const ringMaterial = new THREE.MeshStandardMaterial({
			transparent: true,
			color: 'blue',
			metalness: 0.5,
			roughness: 0.5,
			depthTest: true,
			depthWrite: true,
		});
		const ringMesh = new THREE.Mesh(ring, ringMaterial);
		const cylinder = new THREE.CylinderBufferGeometry(5, 5, 1, 64);
		cylinder.rotateX(Math.PI / 2);
		cylinder.rotateZ(-Math.PI / 2);
		const cylinderMaterial = new THREE.MeshStandardMaterial({
			transparent: true,
			color: 'red',
			metalness: 0.5,
			roughness: 0.5,
			depthTest: true,
			depthWrite: true,
		});

		const frontMaterial = new THREE.MeshStandardMaterial({
			color: 'green',
			metalness: 0.5,
			roughness: 0.5,
			transparent: true,
			depthTest: true,
			depthWrite: true,
			side: THREE.FrontSide,
		});
		const backMaterial = new THREE.MeshStandardMaterial({
			color: 'yellow',
			metalness: 0.5,
			roughness: 0.5,
			transparent: true,
			depthTest: true,
			depthWrite: true,
			side: THREE.FrontSide,
		});
		const sideMaterial = new THREE.MeshStandardMaterial({
			color: 'black',
			metalness: 0.5,
			roughness: 0.5,
			transparent: false,
			depthTest: true,
			depthWrite: true,
		});

		const cyclinderMesh = new THREE.Mesh(cylinder, [sideMaterial, frontMaterial, backMaterial]);
		const group = new THREE.Group();
		group.add(ringMesh);
		group.add(cyclinderMesh);

		scene.add(group);

		const color = 0xffffff;
		const directionalLightFront = new THREE.SpotLight(color, 1);
		directionalLightFront.castShadow = true;
		directionalLightFront.position.y = 30;
		directionalLightFront.position.z = 30;
		directionalLightFront.position.x = 0;
		const light = new THREE.AmbientLight();
		scene.add(directionalLightFront);
		scene.add(light);

		renderer = new THREE.WebGLRenderer({ context, antialias: true });

		renderer.shadowMap.enabled = true;
		renderer.shadowMap.type = THREE.PCFSoftShadowMap;
		renderer.setSize(context.drawingBufferWidth, context.drawingBufferHeight);

		animate();

		function animate() {
			requestAnimationFrame(animate);

			group.rotation.x += 0.01;

			renderer.render(scene, camera);
		}
	}

	ThreeMF(canvas) {
		/*
		var camera, scene, renderer;


		const init = () => {

			scene = new THREE.Scene();
			scene.background = new THREE.Color(0xa0a0a0);
			scene.fog = new THREE.Fog(0xa0a0a0, 10, 500);

			camera = new THREE.PerspectiveCamera(35, window.innerWidth / window.innerHeight, 1, 500);
			camera.position.set(-50, 40, 50);
			scene.add(camera);

			//

			var hemiLight = new THREE.HemisphereLight(0xffffff, 0x444444);
			hemiLight.position.set(0, 100, 0);
			scene.add(hemiLight);

			var dirLight = new THREE.DirectionalLight(0xffffff);
			dirLight.position.set(-0, 40, 50);
			dirLight.castShadow = true;
			dirLight.shadow.camera.top = 50;
			dirLight.shadow.camera.bottom = -25;
			dirLight.shadow.camera.left = -25;
			dirLight.shadow.camera.right = 25;
			dirLight.shadow.camera.near = 0.1;
			dirLight.shadow.camera.far = 200;
			dirLight.shadow.mapSize.set(1024, 1024);
			scene.add(dirLight);

			// scene.add( new CameraHelper( dirLight.shadow.camera ) );

			//

			var manager = new THREE.LoadingManager();

			var loader = new ThreeMFLoader(manager);
			loader.load(this.root + '/models/3mf/truck.3mf', function (object) {

				object.quaternion.setFromEuler(new THREE.Euler(-Math.PI / 2, 0, 0)); 	// z-up conversion

				object.traverse(function (child) {

					child.castShadow = true;

				});

				scene.add(object);

			});

			//

			manager.onLoad = function () {

				render();

			};

			//

			var ground = new THREE.Mesh(new THREE.PlaneBufferGeometry(1000, 1000), new THREE.MeshPhongMaterial({
				color: 0x999999,
				depthWrite: false
			}));
			ground.rotation.x = -Math.PI / 2;
			ground.position.y = 11;
			ground.receiveShadow = true;
			scene.add(ground);

			//

			const context = canvas.getContext('webgl');
			renderer = new THREE.WebGLRenderer({context, antialias: true});
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);
			renderer.outputEncoding = THREE.sRGBEncoding;
			renderer.shadowMap.enabled = true;
			renderer.shadowMap.type = THREE.PCFSoftShadowMap;


			//

			window.addEventListener('resize', onWindowResize, false);

			render();

		}

		function onWindowResize() {

			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);

			render();

		}

		function render() {
			renderer.render(scene, camera);
		}

		init();
		render();
		*/
	}

	ThreeDS(canvas) {
		var camera, scene, renderer, width, height;

		const init = () => {
			const context = canvas.getContext('webgl2');
			width = context.drawingBufferWidth;
			height = context.drawingBufferHeight;
			camera = new THREE.PerspectiveCamera(100, width / height, 0.1, 10);
			camera.position.z = 2;

			scene = new THREE.Scene();
			scene.add(new THREE.HemisphereLight());

			var directionalLight = new THREE.DirectionalLight(0xffeedd);
			directionalLight.position.set(0, 0, 2);
			scene.add(directionalLight);

			{
				//3ds files dont store normal maps
				const loader = new THREE.TextureLoader();
				var normal = loader.load(this.root + '/models/3ds/portalgun/textures/normal.jpg');
			}

			const loader = new TDSLoader();
			loader.setResourcePath(this.root + '/models/3ds/portalgun/textures/');
			loader.load(this.root + '/models/3ds/portalgun/portalgun.3ds', function (object: any) {
				object.traverse(function (child) {
					if (child.isMesh) {
						child.material.normalMap = normal;
					}
				});

				scene.add(object);
			});
			renderer = new THREE.WebGLRenderer({ context });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(width, height);
			window.addEventListener('resize', resize, false);
		};

		function resize() {
			camera.aspect = width / height;
			camera.updateProjectionMatrix();
			renderer.setSize(width, height);
		}

		function animate() {
			renderer.render(scene, camera);

			requestAnimationFrame(animate);
		}

		init();
		animate();
	}

	rayCasting(canvas) {
		var renderer, scene, camera;
		var pointclouds;
		var raycaster;
		var mouse = new THREE.Vector2();
		var intersection = null;
		var spheres = [];
		var spheresIndex = 0;
		var clock;

		var threshold = 0.1;
		var pointSize = 0.05;
		var width = 80;
		var length = 160;
		var rotateY = new THREE.Matrix4().makeRotationY(0.005);

		init();
		animate();

		function generatePointCloudGeometry(color, width, length) {
			var geometry = new THREE.BufferGeometry();
			var numPoints = width * length;

			var positions = new Float32Array(numPoints * 3);
			var colors = new Float32Array(numPoints * 3);

			var k = 0;

			for (var i = 0; i < width; i++) {
				for (var j = 0; j < length; j++) {
					var u = i / width;
					var v = j / length;
					var x = u - 0.5;
					var y = (Math.cos(u * Math.PI * 4) + Math.sin(v * Math.PI * 8)) / 20;
					var z = v - 0.5;

					positions[3 * k] = x;
					positions[3 * k + 1] = y;
					positions[3 * k + 2] = z;

					var intensity = (y + 0.1) * 5;
					colors[3 * k] = color.r * intensity;
					colors[3 * k + 1] = color.g * intensity;
					colors[3 * k + 2] = color.b * intensity;

					k++;
				}
			}

			geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
			geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));
			geometry.computeBoundingBox();

			return geometry;
		}

		function generatePointcloud(color, width, length) {
			var geometry = generatePointCloudGeometry(color, width, length);
			var material = new THREE.PointsMaterial({ size: pointSize, vertexColors: true });

			return new THREE.Points(geometry, material);
		}

		function generateIndexedPointcloud(color, width, length) {
			var geometry = generatePointCloudGeometry(color, width, length);
			var numPoints = width * length;
			var indices = new Uint16Array(numPoints);

			var k = 0;

			for (var i = 0; i < width; i++) {
				for (var j = 0; j < length; j++) {
					indices[k] = k;
					k++;
				}
			}

			geometry.setIndex(new THREE.BufferAttribute(indices, 1));

			var material = new THREE.PointsMaterial({ size: pointSize, vertexColors: true });

			return new THREE.Points(geometry, material);
		}

		function generateIndexedWithOffsetPointcloud(color, width, length) {
			var geometry = generatePointCloudGeometry(color, width, length);
			var numPoints = width * length;
			var indices = new Uint16Array(numPoints);

			var k = 0;

			for (var i = 0; i < width; i++) {
				for (var j = 0; j < length; j++) {
					indices[k] = k;
					k++;
				}
			}

			geometry.setIndex(new THREE.BufferAttribute(indices, 1));
			geometry.addGroup(0, indices.length);

			var material = new THREE.PointsMaterial({ size: pointSize, vertexColors: true });

			return new THREE.Points(geometry, material);
		}

		function init() {
			var container = document.getElementById('container');

			scene = new THREE.Scene();

			clock = new THREE.Clock();

			camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 10000);
			camera.position.set(10, 10, 10);
			camera.lookAt(scene.position);
			camera.updateMatrix();

			//

			var pcBuffer = generatePointcloud(new THREE.Color(1, 0, 0), width, length);
			pcBuffer.scale.set(5, 10, 10);
			pcBuffer.position.set(-5, 0, 0);
			scene.add(pcBuffer);

			var pcIndexed = generateIndexedPointcloud(new THREE.Color(0, 1, 0), width, length);
			pcIndexed.scale.set(5, 10, 10);
			pcIndexed.position.set(0, 0, 0);
			scene.add(pcIndexed);

			var pcIndexedOffset = generateIndexedWithOffsetPointcloud(new THREE.Color(0, 1, 1), width, length);
			pcIndexedOffset.scale.set(5, 10, 10);
			pcIndexedOffset.position.set(5, 0, 0);
			scene.add(pcIndexedOffset);

			pointclouds = [pcBuffer, pcIndexed, pcIndexedOffset];

			//

			var sphereGeometry = new THREE.SphereBufferGeometry(0.1, 32, 32);
			var sphereMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });

			for (var i = 0; i < 40; i++) {
				var sphere = new THREE.Mesh(sphereGeometry, sphereMaterial);
				scene.add(sphere);
				spheres.push(sphere);
			}

			//

			const context = canvas.getContext('webgl2', { antialias: true, alpha: false });
			renderer = new THREE.WebGLRenderer({ context, antialias: true, alpha: false });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);
			container.appendChild(renderer.domElement);

			//

			raycaster = new THREE.Raycaster();
			raycaster.params.Points.threshold = threshold;

			//

			//

			window.addEventListener('resize', onWindowResize, false);
			document.addEventListener('mousemove', onDocumentMouseMove, false);
		}

		function onDocumentMouseMove(event) {
			event.preventDefault();

			mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
			mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;
		}

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		function animate() {
			requestAnimationFrame(animate);

			render();
		}

		var toggle = 0;

		function render() {
			camera.applyMatrix4(rotateY);
			camera.updateMatrixWorld();

			raycaster.setFromCamera(mouse, camera);

			var intersections = raycaster.intersectObjects(pointclouds);
			intersection = intersections.length > 0 ? intersections[0] : null;

			if (toggle > 0.02 && intersection !== null) {
				spheres[spheresIndex].position.copy(intersection.point);
				spheres[spheresIndex].scale.set(1, 1, 1);
				spheresIndex = (spheresIndex + 1) % spheres.length;

				toggle = 0;
			}

			for (var i = 0; i < spheres.length; i++) {
				var sphere = spheres[i];
				sphere.scale.multiplyScalar(0.98);
				sphere.scale.clampScalar(0.01, 1);
			}

			toggle += clock.getDelta();

			renderer.render(scene, camera);
		}
	}

	fbxLoader(canvas) {
		var controls;
		var camera, scene, renderer, light;

		var clock = new THREE.Clock();

		var mixer;

		const init = () => {
			camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 1, 2000);
			camera.position.set(100, 200, 300);

			scene = new THREE.Scene();
			scene.background = new THREE.Color(0xa0a0a0);
			scene.fog = new THREE.Fog(0xa0a0a0, 200, 1000);

			light = new THREE.HemisphereLight(0xffffff, 0x444444);
			light.position.set(0, 200, 0);
			scene.add(light);

			light = new THREE.DirectionalLight(0xffffff);
			light.position.set(0, 200, 100);
			light.castShadow = true;
			light.shadow.camera.top = 180;
			light.shadow.camera.bottom = -100;
			light.shadow.camera.left = -120;
			light.shadow.camera.right = 120;
			scene.add(light);

			// scene.add( new CameraHelper( light.shadow.camera ) );

			// ground
			var mesh = new THREE.Mesh(
				new THREE.PlaneBufferGeometry(2000, 2000),
				new THREE.MeshPhongMaterial({
					color: 0x999999,
					depthWrite: false,
				})
			);
			mesh.rotation.x = -Math.PI / 2;
			mesh.receiveShadow = true;
			scene.add(mesh);

			var grid = new THREE.GridHelper(2000, 20, 0x000000, 0x000000) as any;
			grid.material.opacity = 0.2;
			grid.material.transparent = true;
			scene.add(grid);

			// model
			var loader = new FBXLoader();
			loader.load(this.root + '/models/fbx/SambaDancing.fbx', function (object: any) {
				mixer = new THREE.AnimationMixer(object);

				var action = mixer.clipAction(object.animations[0]);
				action.play();

				object.traverse(function (child: any) {
					if (child.isMesh) {
						child.castShadow = true;
						child.receiveShadow = true;
					}
				});

				scene.add(object);
			});

			const context = canvas.getContext('webgl2');
			renderer = new THREE.WebGLRenderer({ context, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);
			renderer.shadowMap.enabled = true;

			controls = new OrbitControls(camera, canvas);
			controls.target.set(0, 100, 0);
			controls.update();

			window.addEventListener('resize', onWindowResize, false);
		};

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		//

		function animate() {
			requestAnimationFrame(animate);

			var delta = clock.getDelta();

			if (mixer) mixer.update(delta);

			renderer.render(scene, camera);
		}

		init();
		animate();
	}
	/*
		webGLHelpers(canvas) {
			var scene, renderer;
			var camera, light;
			var vnh;
			var vth;
	
			const init = () => {
				const context = canvas.getContext('webgl2');
				renderer = new THREE.WebGLRenderer({ context });
				renderer.setPixelRatio(window.devicePixelRatio);
				renderer.setSize(window.innerWidth, window.innerHeight);
	
				//
	
				camera = new THREE.PerspectiveCamera(100, window.innerWidth / window.innerHeight, 1, 1000);
				camera.position.z = 400;
	
				scene = new THREE.Scene();
	
				light = new THREE.PointLight();
				light.position.set(200, 100, 150);
				scene.add(light);
	
				scene.add(new THREE.PointLightHelper(light, 15));
	
				var gridHelper = new THREE.GridHelper(400, 40, 0x0000ff, 0x808080);
				gridHelper.position.y = -150;
				gridHelper.position.x = -150;
				scene.add(gridHelper);
	
				var polarGridHelper = new THREE.PolarGridHelper(200, 16, 8, 64, 0x0000ff, 0x808080);
				polarGridHelper.position.y = -150;
				polarGridHelper.position.x = 200;
				scene.add(polarGridHelper);
	
				var loader = new GLTFLoader();
				loader.load(this.root + '/models/gltf/LeePerrySmith/LeePerrySmith.glb', function (gltf) {
					var mesh: any = gltf.scene.children[0];
	
					//BufferGeometryUtils.computeTangents(mesh.geometry); // generates bad data due to degenerate UVs
	
					var group = new THREE.Group();
					group.scale.multiplyScalar(50);
					scene.add(group);
	
					// To make sure that the matrixWorld is up to date for the boxhelpers
					group.updateMatrixWorld(true);
	
					group.add(mesh);
	
					vnh = new VertexNormalsHelper(mesh, 5);
					scene.add(vnh);
	
					vth = new VertexTangentsHelper(mesh, 5);
					scene.add(vth);
	
					scene.add(new THREE.BoxHelper(mesh));
	
					var wireframe = new THREE.WireframeGeometry(mesh.geometry);
					var line = new THREE.LineSegments(wireframe) as any;
					line.material.depthTest = false;
					line.material.opacity = 0.25;
					line.material.transparent = true;
					line.position.x = 4;
					group.add(line);
					scene.add(new THREE.BoxHelper(line));
	
					var edges = new THREE.EdgesGeometry(mesh.geometry);
					var line = new THREE.LineSegments(edges) as any;
					line.material.depthTest = false;
					line.material.opacity = 0.25;
					line.material.transparent = true;
					line.position.x = -4;
					group.add(line);
					scene.add(new THREE.BoxHelper(line));
	
					scene.add(new THREE.BoxHelper(group));
					scene.add(new THREE.BoxHelper(scene));
				});
	
				//
	
				window.addEventListener('resize', onWindowResize, false);
			};
	
			function onWindowResize() {
				camera.aspect = window.innerWidth / window.innerHeight;
				camera.updateProjectionMatrix();
	
				renderer.setSize(window.innerWidth, window.innerHeight);
			}
	
			function animate() {
				requestAnimationFrame(animate);
	
				var time = -performance.now() * 0.0003;
	
				camera.position.x = 400 * Math.cos(time);
				camera.position.z = 400 * Math.sin(time);
				camera.lookAt(scene.position);
	
				light.position.x = Math.sin(time * 1.7) * 300;
				light.position.y = Math.cos(time * 1.5) * 400;
				light.position.z = Math.cos(time * 1.3) * 300;
	
				if (vnh) vnh.update();
				if (vth) vth.update();
	
				renderer.render(scene, camera);
			}
	
			init();
			animate();
		}
		*/

	geoTextShapes(canvas) {
		var camera, scene, renderer;
		const context = canvas.getContext('webgl2');

		const init = () => {
			camera = new THREE.PerspectiveCamera(120, window.innerWidth / window.innerHeight, 1, 10000);
			camera.position.set(0, -400, 600);

			scene = new THREE.Scene();
			scene.background = new THREE.Color(0xf0f0f0);

			var loader = new THREE.FontLoader();
			loader.load(this.root + '/fonts/helvetiker_regular.typeface.json', function (font) {
				var xMid, text;

				var color = 0x006699;

				var matDark = new THREE.LineBasicMaterial({
					color: color,
					side: THREE.DoubleSide,
				});

				var matLite = new THREE.MeshBasicMaterial({
					color: color,
					transparent: true,
					opacity: 0.4,
					side: THREE.DoubleSide,
				});

				var message = '   Three.js\nSimple text.';

				var shapes = font.generateShapes(message, 100);

				var geometry = new THREE.ShapeBufferGeometry(shapes);

				geometry.computeBoundingBox();

				xMid = -0.5 * (geometry.boundingBox.max.x - geometry.boundingBox.min.x);

				geometry.translate(xMid, 0, 0);

				// make shape ( N.B. edge view not visible )

				text = new THREE.Mesh(geometry, matLite);
				text.position.z = -150;
				scene.add(text);

				// make line shape ( N.B. edge view remains visible )

				var holeShapes = [];

				for (var i = 0; i < shapes.length; i++) {
					var shape = shapes[i];

					if (shape.holes && shape.holes.length > 0) {
						for (var j = 0; j < shape.holes.length; j++) {
							var hole = shape.holes[j];
							holeShapes.push(hole);
						}
					}
				}

				shapes.push.apply(shapes, holeShapes);

				var lineText = new THREE.Object3D();

				for (var i = 0; i < shapes.length; i++) {
					var shape = shapes[i];

					var points = shape.getPoints();
					var geometry = new THREE.BufferGeometry().setFromPoints(points);

					geometry.translate(xMid, 0, 0);

					var lineMesh = new THREE.Line(geometry, matDark);
					lineText.add(lineMesh);
				}

				scene.add(lineText);
			}); //end load function

			renderer = new THREE.WebGLRenderer({ context, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);

			window.addEventListener('resize', onWindowResize, false);
		}; // end init

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		function animate() {
			requestAnimationFrame(animate);

			render();
		}

		function render() {
			renderer.render(scene, camera);
		}

		init();
		animate();
	}

	geoColors(args) {
		var container, stats;

		var camera, scene, renderer;

		var mouseX = 0,
			mouseY = 0;

		var windowHalfX = window.innerWidth / 2;
		var windowHalfY = window.innerHeight / 2;

		init();
		animate();

		function init() {
			camera = new THREE.PerspectiveCamera(20, window.innerWidth / window.innerHeight, 1, 100000);
			camera.position.z = 8000;

			scene = new THREE.Scene();
			scene.background = new THREE.Color(0xffffff);

			var light = new THREE.DirectionalLight(0xffffff);
			light.position.set(0, 0, 1);
			scene.add(light);

			// shadow

			var canvas = document.createElement('canvas');
			canvas.width = 128;
			canvas.height = 128;

			var context = canvas.getContext('2d');
			var gradient = context.createRadialGradient(canvas.width / 2, canvas.height / 2, 0, canvas.width / 2, canvas.height / 2, canvas.width / 2);
			gradient.addColorStop(0.1, 'rgba(210,210,210,1)');
			gradient.addColorStop(1, 'rgba(255,255,255,1)');

			context.fillStyle = gradient;
			context.fillRect(0, 0, canvas.width, canvas.height);

			var shadowTexture = new THREE.CanvasTexture(canvas);

			var shadowMaterial = new THREE.MeshBasicMaterial({ map: shadowTexture });
			var shadowGeo = new THREE.PlaneBufferGeometry(300, 300, 1, 1);

			var shadowMesh;

			shadowMesh = new THREE.Mesh(shadowGeo, shadowMaterial);
			shadowMesh.position.y = -250;
			shadowMesh.rotation.x = -Math.PI / 2;
			scene.add(shadowMesh);

			shadowMesh = new THREE.Mesh(shadowGeo, shadowMaterial);
			shadowMesh.position.y = -250;
			shadowMesh.position.x = -400;
			shadowMesh.rotation.x = -Math.PI / 2;
			scene.add(shadowMesh);

			shadowMesh = new THREE.Mesh(shadowGeo, shadowMaterial);
			shadowMesh.position.y = -250;
			shadowMesh.position.x = 400;
			shadowMesh.rotation.x = -Math.PI / 2;
			scene.add(shadowMesh);

			var radius = 200;

			var geometry1 = new THREE.IcosahedronBufferGeometry(radius, 1);

			var count = geometry1.attributes.position.count;
			geometry1.setAttribute('color', new THREE.BufferAttribute(new Float32Array(count * 3), 3));

			var geometry2 = geometry1.clone();
			var geometry3 = geometry1.clone();

			var color = new THREE.Color();
			var positions1 = geometry1.attributes.position;
			var positions2 = geometry2.attributes.position;
			var positions3 = geometry3.attributes.position;
			var colors1 = geometry1.attributes.color;
			var colors2 = geometry2.attributes.color;
			var colors3 = geometry3.attributes.color;

			for (var i = 0; i < count; i++) {
				color.setHSL((positions1.getY(i) / radius + 1) / 2, 1.0, 0.5);
				colors1.setXYZ(i, color.r, color.g, color.b);

				color.setHSL(0, (positions2.getY(i) / radius + 1) / 2, 0.5);
				colors2.setXYZ(i, color.r, color.g, color.b);

				color.setRGB(1, 0.8 - (positions3.getY(i) / radius + 1) / 2, 0);
				colors3.setXYZ(i, color.r, color.g, color.b);
			}

			var material = new THREE.MeshPhongMaterial({
				color: 0xffffff,
				flatShading: true,
				vertexColors: true,
				shininess: 0,
			});

			var wireframeMaterial = new THREE.MeshBasicMaterial({ color: 0x000000, wireframe: true, transparent: true });

			var mesh = new THREE.Mesh(geometry1, material);
			var wireframe = new THREE.Mesh(geometry1, wireframeMaterial);
			mesh.add(wireframe);
			mesh.position.x = -400;
			mesh.rotation.x = -1.87;
			scene.add(mesh);

			var mesh = new THREE.Mesh(geometry2, material);
			var wireframe = new THREE.Mesh(geometry2, wireframeMaterial);
			mesh.add(wireframe);
			mesh.position.x = 400;
			scene.add(mesh);

			var mesh = new THREE.Mesh(geometry3, material);
			var wireframe = new THREE.Mesh(geometry3, wireframeMaterial);
			mesh.add(wireframe);
			scene.add(mesh);
			const gl = args.getContext('webgl2') as any;

			renderer = new THREE.WebGLRenderer({ context: gl, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);

			document.addEventListener('mousemove', onDocumentMouseMove, false);

			//

			window.addEventListener('resize', onWindowResize, false);
		}

		function onWindowResize() {
			windowHalfX = window.innerWidth / 2;
			windowHalfY = window.innerHeight / 2;

			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		function onDocumentMouseMove(event) {
			mouseX = event.clientX - windowHalfX;
			mouseY = event.clientY - windowHalfY;
		}

		//

		function animate() {
			requestAnimationFrame(animate);

			render();
		}

		function render() {
			camera.position.x += (mouseX - camera.position.x) * 0.05;
			camera.position.y += (-mouseY - camera.position.y) * 0.05;

			camera.lookAt(scene.position);

			renderer.render(scene, camera);
		}
	}

	// https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4

	threeCube(canvas) {
		var camera, scene, renderer;
		var geometry, material, mesh;
		var texture;
		const src = 'https://github.com/mdn/webgl-examples/blob/gh-pages/tutorial/sample8/Firefox.mp4?raw=true'; //this.root + '/textures/Firefox.mp4'
		init();
		animate();

		function init() {
			const context = canvas.getContext('webgl2');
			texture = document.createElement('video');
			texture.loop = true;
			texture.muted = true;
			texture.src = 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4';
			texture.play();
			camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.01, 1000);
			camera.position.z = 1;

			scene = new THREE.Scene();

			geometry = new THREE.BoxGeometry(0.2, 0.2, 0.2);
			material = new THREE.MeshBasicMaterial();
			material.map = new THREE.VideoTexture(texture);

			mesh = new THREE.Mesh(geometry, material);
			scene.add(mesh);

			renderer = new THREE.WebGLRenderer({ context, antialias: false });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		function animate() {
			requestAnimationFrame(animate);

			mesh.rotation.x += 0.01;
			mesh.rotation.y += 0.01;

			renderer.render(scene, camera);
		}
	}

	animationKkinningblending(canvas) {
		const context = canvas.getContext('webgl2') as any;
	}

	threeCar(canvas) {
		const context = canvas.getContext('webgl2') as any;
		let camera, scene, renderer;
		let stats;

		let grid;
		let controls;

		const wheels = [];
		let bodyColor = '#ff0000';
		let detailsColor = '#ffffff';
		let glassColor = '#ffffff';
		function init(root) {
			renderer = new THREE.WebGLRenderer({ context, antialias: false });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);

			grid = new THREE.GridHelper(100, 40, 0x000000, 0x000000);
			grid.material.opacity = 0.1;
			grid.material.depthWrite = false;
			grid.material.transparent = true;
			renderer.outputEncoding = THREE.sRGBEncoding;
			renderer.toneMapping = THREE.ACESFilmicToneMapping;
			renderer.toneMappingExposure = 0.85;

			window.addEventListener('resize', onWindowResize, false);

			//
			camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
			camera.position.set(4.25, 1.4, -7);

			controls = new OrbitControls(camera, canvas);
			controls.target.set(0, 0.5, 0);

			const environment = new RoomEnvironment();
			const pmremGenerator = new THREE.PMREMGenerator(renderer);

			scene = new THREE.Scene();
			scene.background = new THREE.Color(0xeeeeee);
			scene.environment = pmremGenerator.fromScene(environment).texture;
			scene.fog = new THREE.Fog(0xeeeeee, 10, 50);

			const light = new THREE.SpotLight('white', 1);
			light.position.set(4.25, 10, -10);
			scene.add(light);
			scene.add(grid);

			// materials

			const bodyMaterial = new THREE.MeshPhysicalMaterial({
				color: 0xff0000,
				metalness: 0.6,
				roughness: 0.4,
				clearcoat: 0.05,
				clearcoatRoughness: 0.05,
			});

			const detailsMaterial = new THREE.MeshStandardMaterial({
				color: 0xffffff,
				metalness: 1.0,
				roughness: 0.5,
			});

			const glassMaterial = new THREE.MeshPhysicalMaterial({
				color: 0xffffff,
				metalness: 0,
				roughness: 0.1,
				transmission: 0.9,
				transparent: true,
			});

			bodyMaterial.color.set(bodyColor);
			detailsMaterial.color.set(detailsColor);
			glassMaterial.color.set(glassColor);
			/*
			const bodyColorInput = document.getElementById( 'body-color' );
			bodyColorInput.addEventListener( 'input', function () {

				bodyMaterial.color.set( this.value );

			} );

			const detailsColorInput = document.getElementById( 'details-color' );
			detailsColorInput.addEventListener( 'input', function () {

				detailsMaterial.color.set( this.value );

			} );

			const glassColorInput = document.getElementById( 'glass-color' );
			glassColorInput.addEventListener( 'input', function () {

				glassMaterial.color.set( this.value );

			} );
			*/

			// Car

			const shadow = new THREE.TextureLoader().load(root + '/models/gltf/ferrari_ao.png');
			// almost working
			const dracoLoader = new DRACOLoader();
			dracoLoader.setDecoderPath(root + '/js/libs/draco/gltf/');
			dracoLoader.setDecoderConfig({ type: 'js' });

			const loader = new GLTFLoader();
			loader.setDRACOLoader(dracoLoader);

			loader.load(root + '/models/gltf/ferrari.glb', function (gltf) {
				console.log('ferrari.glb');
				const carModel: any = gltf.scene.children[0];

				carModel.getObjectByName('body').material = bodyMaterial;

				carModel.getObjectByName('rim_fl').material = detailsMaterial;
				carModel.getObjectByName('rim_fr').material = detailsMaterial;
				carModel.getObjectByName('rim_rr').material = detailsMaterial;
				carModel.getObjectByName('rim_rl').material = detailsMaterial;
				carModel.getObjectByName('trim').material = detailsMaterial;

				carModel.getObjectByName('glass').material = glassMaterial;

				wheels.push(carModel.getObjectByName('wheel_fl'), carModel.getObjectByName('wheel_fr'), carModel.getObjectByName('wheel_rl'), carModel.getObjectByName('wheel_rr'));

				// shadow
				const mesh = new THREE.Mesh(
					new THREE.PlaneBufferGeometry(0.655 * 4, 1.3 * 4),
					new THREE.MeshBasicMaterial({
						map: shadow,
						toneMapped: false,
						transparent: true,
					})
				);
				mesh.rotation.x = -Math.PI / 2;
				mesh.renderOrder = 2;
				carModel.add(mesh);

				scene.add(carModel);
			});

			renderer.setAnimationLoop(render);

			controls.update();
		}

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		function render() {
			const time = -performance.now() / 1000;

			for (let i = 0; i < wheels.length; i++) {
				wheels[i].rotation.x = time * Math.PI;
			}

			grid.position.z = -time % 5;

			renderer.render(scene, camera);

			//	stats.update();
		}

		init(this.root);
	}

	threeKeyframes(canvas) {
		let mixer;
		const context = canvas.getContext('webgl2') as any;
		const clock = new THREE.Clock();
		const renderer = new THREE.WebGLRenderer({ context, antialias: true });
		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(window.innerWidth, window.innerHeight);
		renderer.outputEncoding = THREE.sRGBEncoding;

		const pmremGenerator = new THREE.PMREMGenerator(renderer);

		const scene = new THREE.Scene();

		const light = new THREE.SpotLight();
		light.position.set(-1.8, 0.6, 2.7 * 1.2);
		scene.add(light);

		scene.background = new THREE.Color(0xbfe3dd);

		scene.environment = pmremGenerator.fromScene(new RoomEnvironment(), 0.04).texture;

		const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 3000);
		camera.position.set(5, 2, 8);

		const controls = new OrbitControls(camera, canvas);
		controls.target.set(0, 0.5, 0);
		controls.update();
		controls.enablePan = false;
		controls.enableDamping = true;

		const dracoLoader = new DRACOLoader();
		dracoLoader.setDecoderPath(this.root + '/js/libs/draco/gltf/');
		dracoLoader.setDecoderConfig({ type: 'js' });

		const loader = new GLTFLoader();
		loader.setDRACOLoader(dracoLoader);
		loader.load(
			this.root + '/models/gltf/LittlestTokyo.glb',
			function (gltf) {
				const model = gltf.scene;
				model.position.set(1, 1, 0);
				model.scale.set(0.01, 0.01, 0.01);
				scene.add(model);

				mixer = new THREE.AnimationMixer(model);
				mixer.clipAction(gltf.animations[0]).play();

				animate();
			},
			undefined,
			function (e) {
				console.error(e);
			}
		);

		window.onresize = function () {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		};

		function animate() {
			requestAnimationFrame(animate);

			const delta = clock.getDelta();

			mixer.update(delta);

			controls.update();

			renderer.render(scene, camera);
		}
	}
	birds(canvas) {
		const context = canvas.getContext('webgl2') as any;

		/* TEXTURE WIDTH FOR SIMULATION */
		const WIDTH = 20;
		const BIRDS = WIDTH * WIDTH;

		/* BAKE ANIMATION INTO TEXTURE and CREATE GEOMETRY FROM BASE MODEL */
		const BirdGeometry = new THREE.BufferGeometry();
		let textureAnimation, durationAnimation, birdMesh, materialShader, vertexPerBird;

		function nextPowerOf2(n) {
			return Math.pow(2, Math.ceil(Math.log(n) / Math.log(2)));
		}

		// @ts-ignore
		Math.lerp = function (value1, value2, amount) {
			amount = Math.max(Math.min(amount, 1), 0);
			return value1 + (value2 - value1) * amount;
		};

		const gltfs = ['/models/gltf/Parrot.glb', '/models/gltf/Flamingo.glb'];
		const colors = [0xccffff, 0xffdeff];
		const sizes = [0.2, 0.1];
		const selectModel = Math.floor(Math.random() * gltfs.length);
		new GLTFLoader().setPath(this.root).load(gltfs[selectModel], function (gltf: any) {
			const animations = gltf.animations;
			durationAnimation = Math.round(animations[0].duration * 60);
			const birdGeo = gltf.scene.children[0].geometry;
			const morphAttributes = birdGeo.morphAttributes.position;
			const tHeight = nextPowerOf2(durationAnimation);
			const tWidth = nextPowerOf2(birdGeo.getAttribute('position').count);
			vertexPerBird = birdGeo.getAttribute('position').count;
			const tData = new Float32Array(3 * tWidth * tHeight);

			for (let i = 0; i < tWidth; i++) {
				for (let j = 0; j < tHeight; j++) {
					const offset = j * tWidth * 3;

					const curMorph = Math.floor((j / durationAnimation) * morphAttributes.length);
					const nextMorph = (Math.floor((j / durationAnimation) * morphAttributes.length) + 1) % morphAttributes.length;
					const lerpAmount = ((j / durationAnimation) * morphAttributes.length) % 1;

					if (j < durationAnimation) {
						let d0, d1;

						d0 = morphAttributes[curMorph].array[i * 3];
						d1 = morphAttributes[nextMorph].array[i * 3];
						//@ts-ignore
						if (d0 !== undefined && d1 !== undefined) tData[offset + i * 3] = Math.lerp(d0, d1, lerpAmount);

						d0 = morphAttributes[curMorph].array[i * 3 + 1];
						d1 = morphAttributes[nextMorph].array[i * 3 + 1];

						//@ts-ignore
						if (d0 !== undefined && d1 !== undefined) tData[offset + i * 3 + 1] = Math.lerp(d0, d1, lerpAmount);

						d0 = morphAttributes[curMorph].array[i * 3 + 2];
						d1 = morphAttributes[nextMorph].array[i * 3 + 2];
						//@ts-ignore
						if (d0 !== undefined && d1 !== undefined) tData[offset + i * 3 + 2] = Math.lerp(d0, d1, lerpAmount);
					}
				}
			}

			textureAnimation = new THREE.DataTexture(tData, tWidth, tHeight, THREE.RGBFormat, THREE.FloatType);
			textureAnimation.needsUpdate = true;

			const vertices = [],
				color = [],
				reference = [],
				seeds = [],
				indices = [];
			const totalVertices = birdGeo.getAttribute('position').count * 3 * BIRDS;
			for (let i = 0; i < totalVertices; i++) {
				const bIndex = i % (birdGeo.getAttribute('position').count * 3);
				vertices.push(birdGeo.getAttribute('position').array[bIndex]);
				color.push(birdGeo.getAttribute('color').array[bIndex]);
			}

			let r = Math.random();
			for (let i = 0; i < birdGeo.getAttribute('position').count * BIRDS; i++) {
				const bIndex = i % birdGeo.getAttribute('position').count;
				const bird = Math.floor(i / birdGeo.getAttribute('position').count);
				if (bIndex == 0) r = Math.random();
				const j = ~~bird;
				const x = (j % WIDTH) / WIDTH;
				const y = ~~(j / WIDTH) / WIDTH;
				reference.push(x, y, bIndex / tWidth, durationAnimation / tHeight);
				seeds.push(bird, r, Math.random(), Math.random());
			}

			for (let i = 0; i < birdGeo.index.array.length * BIRDS; i++) {
				const offset = Math.floor(i / birdGeo.index.array.length) * birdGeo.getAttribute('position').count;
				indices.push(birdGeo.index.array[i % birdGeo.index.array.length] + offset);
			}

			BirdGeometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3));
			BirdGeometry.setAttribute('birdColor', new THREE.BufferAttribute(new Float32Array(color), 3));
			BirdGeometry.setAttribute('color', new THREE.BufferAttribute(new Float32Array(color), 3));
			BirdGeometry.setAttribute('reference', new THREE.BufferAttribute(new Float32Array(reference), 4));
			BirdGeometry.setAttribute('seeds', new THREE.BufferAttribute(new Float32Array(seeds), 4));

			BirdGeometry.setIndex(indices);

			init();
			animate();
		});

		let container, stats;
		let camera, scene, renderer;
		let mouseX = 0,
			mouseY = 0;

		let windowHalfX = window.innerWidth / 2;
		let windowHalfY = window.innerHeight / 2;

		const BOUNDS = 800,
			BOUNDS_HALF = BOUNDS / 2;

		let last = performance.now();

		let gpuCompute;
		let velocityVariable;
		let positionVariable;
		let positionUniforms;
		let velocityUniforms;

		function init() {
			camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 1, 3000);
			camera.position.z = 350;

			scene = new THREE.Scene();
			scene.background = new THREE.Color(colors[selectModel]);
			scene.fog = new THREE.Fog(colors[selectModel], 100, 1000);

			// LIGHTS

			const hemiLight = new THREE.HemisphereLight(colors[selectModel], 0xffffff, 1.6);
			hemiLight.color.setHSL(0.6, 1, 0.6);
			hemiLight.groundColor.setHSL(0.095, 1, 0.75);
			hemiLight.position.set(0, 50, 0);
			scene.add(hemiLight);

			const dirLight = new THREE.DirectionalLight(0x00ced1, 0.6);
			dirLight.color.setHSL(0.1, 1, 0.95);
			dirLight.position.set(-1, 1.75, 1);
			dirLight.position.multiplyScalar(30);
			scene.add(dirLight);

			renderer = new THREE.WebGLRenderer({ context, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);

			initComputeRenderer();

			window.addEventListener('resize', onWindowResize, false);

			const effectController = {
				separation: 20.0,
				alignment: 20.0,
				cohesion: 20.0,
				freedom: 0.75,
				size: sizes[selectModel],
				count: BIRDS,
			};

			const valuesChanger = function () {
				velocityUniforms['separationDistance'].value = effectController.separation;
				velocityUniforms['alignmentDistance'].value = effectController.alignment;
				velocityUniforms['cohesionDistance'].value = effectController.cohesion;
				velocityUniforms['freedomFactor'].value = effectController.freedom;
				if (materialShader) materialShader.uniforms['size'].value = effectController.size;
				BirdGeometry.setDrawRange(0, vertexPerBird * effectController.count);
			};

			valuesChanger();

			initBirds(effectController);
		}

		function initComputeRenderer() {
			gpuCompute = new GPUComputationRenderer(WIDTH, WIDTH, renderer);

			if (isSafari()) {
				gpuCompute.setDataType(THREE.HalfFloatType);
			}

			const dtPosition = gpuCompute.createTexture();
			const dtVelocity = gpuCompute.createTexture();
			fillPositionTexture(dtPosition);
			fillVelocityTexture(dtVelocity);
			const fragmentShaderVelocity = `
				uniform float time;
			uniform float testing;
			uniform float delta; // about 0.016
			uniform float separationDistance; // 20
			uniform float alignmentDistance; // 40
			uniform float cohesionDistance; //
			uniform float freedomFactor;
			uniform vec3 predator;

			const float width = resolution.x;
			const float height = resolution.y;

			const float PI = 3.141592653589793;
			const float PI_2 = PI * 2.0;
			// const float VISION = PI * 0.55;

			float zoneRadius = 40.0;
			float zoneRadiusSquared = 1600.0;

			float separationThresh = 0.45;
			float alignmentThresh = 0.65;

			const float UPPER_BOUNDS = BOUNDS;
			const float LOWER_BOUNDS = -UPPER_BOUNDS;

			const float SPEED_LIMIT = 9.0;

			float rand( vec2 co ){
				return fract( sin( dot( co.xy, vec2(12.9898,78.233) ) ) * 43758.5453 );
			}

			void main() {

				zoneRadius = separationDistance + alignmentDistance + cohesionDistance;
				separationThresh = separationDistance / zoneRadius;
				alignmentThresh = ( separationDistance + alignmentDistance ) / zoneRadius;
				zoneRadiusSquared = zoneRadius * zoneRadius;


				vec2 uv = gl_FragCoord.xy / resolution.xy;
				vec3 birdPosition, birdVelocity;

				vec3 selfPosition = texture2D( texturePosition, uv ).xyz;
				vec3 selfVelocity = texture2D( textureVelocity, uv ).xyz;

				float dist;
				vec3 dir; // direction
				float distSquared;

				float separationSquared = separationDistance * separationDistance;
				float cohesionSquared = cohesionDistance * cohesionDistance;

				float f;
				float percent;

				vec3 velocity = selfVelocity;

				float limit = SPEED_LIMIT;

				dir = predator * UPPER_BOUNDS - selfPosition;
				dir.z = 0.;
				// dir.z *= 0.6;
				dist = length( dir );
				distSquared = dist * dist;

				float preyRadius = 150.0;
				float preyRadiusSq = preyRadius * preyRadius;


				// move birds away from predator
				if ( dist < preyRadius ) {

					f = ( distSquared / preyRadiusSq - 1.0 ) * delta * 100.;
					velocity += normalize( dir ) * f;
					limit += 5.0;
				}


				// if (testing == 0.0) {}
				// if ( rand( uv + time ) < freedomFactor ) {}


				// Attract flocks to the center
				vec3 central = vec3( 0., 0., 0. );
				dir = selfPosition - central;
				dist = length( dir );

				dir.y *= 2.5;
				velocity -= normalize( dir ) * delta * 5.;

				for ( float y = 0.0; y < height; y++ ) {
					for ( float x = 0.0; x < width; x++ ) {

						vec2 ref = vec2( x + 0.5, y + 0.5 ) / resolution.xy;
						birdPosition = texture2D( texturePosition, ref ).xyz;

						dir = birdPosition - selfPosition;
						dist = length( dir );

						if ( dist < 0.0001 ) continue;

						distSquared = dist * dist;

						if ( distSquared > zoneRadiusSquared ) continue;

						percent = distSquared / zoneRadiusSquared;

						if ( percent < separationThresh ) { // low

							// Separation - Move apart for comfort
							f = ( separationThresh / percent - 1.0 ) * delta;
							velocity -= normalize( dir ) * f;

						} else if ( percent < alignmentThresh ) { // high

							// Alignment - fly the same direction
							float threshDelta = alignmentThresh - separationThresh;
							float adjustedPercent = ( percent - separationThresh ) / threshDelta;

							birdVelocity = texture2D( textureVelocity, ref ).xyz;

							f = ( 0.5 - cos( adjustedPercent * PI_2 ) * 0.5 + 0.5 ) * delta;
							velocity += normalize( birdVelocity ) * f;

						} else {

							// Attraction / Cohesion - move closer
							float threshDelta = 1.0 - alignmentThresh;
							float adjustedPercent;
							if( threshDelta == 0. ) adjustedPercent = 1.;
							else adjustedPercent = ( percent - alignmentThresh ) / threshDelta;

							f = ( 0.5 - ( cos( adjustedPercent * PI_2 ) * -0.5 + 0.5 ) ) * delta;

							velocity += normalize( dir ) * f;

						}

					}

				}

				// this make tends to fly around than down or up
				// if (velocity.y > 0.) velocity.y *= (1. - 0.2 * delta);

				// Speed Limits
				if ( length( velocity ) > limit ) {
					velocity = normalize( velocity ) * limit;
				}

				gl_FragColor = vec4( velocity, 1.0 );
			}
				`;

			const fragmentShaderPosition = `
				uniform float time;
			uniform float delta;

			void main()	{

				vec2 uv = gl_FragCoord.xy / resolution.xy;
				vec4 tmpPos = texture2D( texturePosition, uv );
				vec3 position = tmpPos.xyz;
				vec3 velocity = texture2D( textureVelocity, uv ).xyz;

				float phase = tmpPos.w;

				phase = mod( ( phase + delta +
					length( velocity.xz ) * delta * 3. +
					max( velocity.y, 0.0 ) * delta * 6. ), 62.83 );

				gl_FragColor = vec4( position + velocity * delta * 15. , phase );

			}
				`;
			velocityVariable = gpuCompute.addVariable('textureVelocity', fragmentShaderVelocity, dtVelocity);
			positionVariable = gpuCompute.addVariable('texturePosition', fragmentShaderPosition, dtPosition);

			gpuCompute.setVariableDependencies(velocityVariable, [positionVariable, velocityVariable]);
			gpuCompute.setVariableDependencies(positionVariable, [positionVariable, velocityVariable]);

			positionUniforms = positionVariable.material.uniforms;
			velocityUniforms = velocityVariable.material.uniforms;

			positionUniforms['time'] = { value: 0.0 };
			positionUniforms['delta'] = { value: 0.0 };
			velocityUniforms['time'] = { value: 1.0 };
			velocityUniforms['delta'] = { value: 0.0 };
			velocityUniforms['testing'] = { value: 1.0 };
			velocityUniforms['separationDistance'] = { value: 1.0 };
			velocityUniforms['alignmentDistance'] = { value: 1.0 };
			velocityUniforms['cohesionDistance'] = { value: 1.0 };
			velocityUniforms['freedomFactor'] = { value: 1.0 };
			velocityUniforms['predator'] = { value: new THREE.Vector3() };
			velocityVariable.material.defines.BOUNDS = BOUNDS.toFixed(2);

			velocityVariable.wrapS = THREE.RepeatWrapping;
			velocityVariable.wrapT = THREE.RepeatWrapping;
			positionVariable.wrapS = THREE.RepeatWrapping;
			positionVariable.wrapT = THREE.RepeatWrapping;

			const error = gpuCompute.init();

			if (error !== null) {
				console.error(error);
			}
		}

		function isSafari() {
			return !!navigator.userAgent.match(/Safari/i) && !navigator.userAgent.match(/Chrome/i);
		}

		function initBirds(effectController) {
			const geometry = BirdGeometry;

			const m = new THREE.MeshStandardMaterial({
				vertexColors: true,
				flatShading: true,
				roughness: 1,
				metalness: 0,
			});

			m.onBeforeCompile = (shader) => {
				shader.uniforms.texturePosition = { value: null };
				shader.uniforms.textureVelocity = { value: null };
				shader.uniforms.textureAnimation = { value: textureAnimation };
				shader.uniforms.time = { value: 1.0 };
				shader.uniforms.size = { value: effectController.size };
				shader.uniforms.delta = { value: 0.0 };

				let token = '#define STANDARD';

				let insert = /* glsl */ `
						attribute vec4 reference;
						attribute vec4 seeds;
						attribute vec3 birdColor;
						uniform sampler2D texturePosition;
						uniform sampler2D textureVelocity;
						uniform sampler2D textureAnimation;
						uniform float size;
						uniform float time;
					`;

				shader.vertexShader = shader.vertexShader.replace(token, token + insert);

				token = '#include <begin_vertex>';

				insert = /* glsl */ `
						vec4 tmpPos = texture2D( texturePosition, reference.xy );
						vec3 pos = tmpPos.xyz;
						vec3 velocity = normalize(texture2D( textureVelocity, reference.xy ).xyz);
						vec3 aniPos = texture2D( textureAnimation, vec2( reference.z, mod( time + ( seeds.x ) * ( ( 0.0004 + seeds.y / 10000.0) + normalize( velocity ) / 20000.0 ), reference.w ) ) ).xyz;
						vec3 newPosition = position;
						newPosition = mat3( modelMatrix ) * ( newPosition + aniPos );
						newPosition *= size + seeds.y * size * 0.2;
						velocity.z *= -1.;
						float xz = length( velocity.xz );
						float xyz = 1.;
						float x = sqrt( 1. - velocity.y * velocity.y );
						float cosry = velocity.x / xz;
						float sinry = velocity.z / xz;
						float cosrz = x / xyz;
						float sinrz = velocity.y / xyz;
						mat3 maty =  mat3( cosry, 0, -sinry, 0    , 1, 0     , sinry, 0, cosry );
						mat3 matz =  mat3( cosrz , sinrz, 0, -sinrz, cosrz, 0, 0     , 0    , 1 );
						newPosition =  maty * matz * newPosition;
						newPosition += pos;
						vec3 transformed = vec3( newPosition );
					`;

				shader.vertexShader = shader.vertexShader.replace(token, insert);

				materialShader = shader;
			};

			birdMesh = new THREE.Mesh(geometry, m);
			birdMesh.rotation.y = Math.PI / 2;

			birdMesh.castShadow = true;
			birdMesh.receiveShadow = true;

			scene.add(birdMesh);
		}

		function fillPositionTexture(texture) {
			const theArray = texture.image.data;

			for (let k = 0, kl = theArray.length; k < kl; k += 4) {
				const x = Math.random() * BOUNDS - BOUNDS_HALF;
				const y = Math.random() * BOUNDS - BOUNDS_HALF;
				const z = Math.random() * BOUNDS - BOUNDS_HALF;

				theArray[k + 0] = x;
				theArray[k + 1] = y;
				theArray[k + 2] = z;
				theArray[k + 3] = 1;
			}
		}

		function fillVelocityTexture(texture) {
			const theArray = texture.image.data;

			for (let k = 0, kl = theArray.length; k < kl; k += 4) {
				const x = Math.random() - 0.5;
				const y = Math.random() - 0.5;
				const z = Math.random() - 0.5;

				theArray[k + 0] = x * 10;
				theArray[k + 1] = y * 10;
				theArray[k + 2] = z * 10;
				theArray[k + 3] = 1;
			}
		}

		function onWindowResize() {
			windowHalfX = window.innerWidth / 2;
			windowHalfY = window.innerHeight / 2;

			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		function onPointerMove(event) {
			if (event.isPrimary === false) return;

			mouseX = event.clientX - windowHalfX;
			mouseY = event.clientY - windowHalfY;
		}

		//

		function animate() {
			requestAnimationFrame(animate);

			render();
		}

		function render() {
			const now = performance.now();
			let delta = (now - last) / 1000;

			if (delta > 1) delta = 1; // safety cap on large deltas
			last = now;

			positionUniforms['time'].value = now;
			positionUniforms['delta'].value = delta;
			velocityUniforms['time'].value = now;
			velocityUniforms['delta'].value = delta;
			if (materialShader) materialShader.uniforms['time'].value = now / 1000;
			if (materialShader) materialShader.uniforms['delta'].value = delta;

			velocityUniforms['predator'].value.set((0.5 * mouseX) / windowHalfX, (-0.5 * mouseY) / windowHalfY, 0);

			mouseX = 10000;
			mouseY = 10000;

			gpuCompute.compute();

			if (materialShader) materialShader.uniforms['texturePosition'].value = gpuCompute.getCurrentRenderTarget(positionVariable).texture;
			if (materialShader) materialShader.uniforms['textureVelocity'].value = gpuCompute.getCurrentRenderTarget(velocityVariable).texture;

			renderer.render(scene, camera);
		}
	}

	bufferGeo(canvas) {
		const context = canvas.getContext('webgl2', { antialias: false }) as any;

		let container, stats;

		let camera, scene, renderer;

		let points;

		const particles = 300000;
		let drawCount = 10000;

		init();
		animate();

		function init() {
			renderer = new THREE.WebGLRenderer({ context, antialias: false });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);

			//

			camera = new THREE.PerspectiveCamera(27, window.innerWidth / window.innerHeight, 5, 3500);
			camera.position.z = 2750;

			scene = new THREE.Scene();
			scene.background = new THREE.Color(0x050505);
			scene.fog = new THREE.Fog(0x050505, 2000, 3500);

			//

			const geometry = new THREE.BufferGeometry();

			const positions = [];
			const positions2 = [];
			const colors = [];

			const color = new THREE.Color();

			const n = 1000,
				n2 = n / 2; // particles spread in the cube

			for (let i = 0; i < particles; i++) {
				// positions

				const x = Math.random() * n - n2;
				const y = Math.random() * n - n2;
				const z = Math.random() * n - n2;

				positions.push(x, y, z);
				positions2.push(z * 0.3, x * 0.3, y * 0.3);

				// colors

				const vx = x / n + 0.5;
				const vy = y / n + 0.5;
				const vz = z / n + 0.5;

				color.setRGB(vx, vy, vz);

				colors.push(color.r, color.g, color.b);
			}

			const gl = renderer.getContext();

			const pos = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, pos);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

			const pos2 = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, pos2);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions2), gl.STATIC_DRAW);

			const rgb = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, rgb);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);

			// @ts-ignore
			const posAttr1 = new THREE.GLBufferAttribute(pos, gl.FLOAT, 3, 4, particles);
			// @ts-ignore
			const posAttr2 = new THREE.GLBufferAttribute(pos2, gl.FLOAT, 3, 4, particles);
			geometry.setAttribute('position', posAttr1);

			setInterval(function () {
				const attr = geometry.getAttribute('position');

				geometry.setAttribute('position', attr === posAttr1 ? posAttr2 : posAttr1);
			}, 2000);

			// @ts-ignore
			geometry.setAttribute('color', new THREE.GLBufferAttribute(rgb, gl.FLOAT, 3, 4, particles));

			//

			const material = new THREE.PointsMaterial({ size: 15, vertexColors: true });

			points = new THREE.Points(geometry, material);

			// Choose one:
			// geometry.boundingSphere = ( new THREE.Sphere() ).set( new THREE.Vector3(), Infinity );
			points.frustumCulled = false;

			scene.add(points);

			//

			//

			window.addEventListener('resize', onWindowResize, false);
		}

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		//

		function animate() {
			requestAnimationFrame(animate);

			render();
		}

		function render() {
			drawCount = (Math.max(5000, drawCount) + Math.floor(500 * Math.random())) % particles;
			points.geometry.setDrawRange(0, drawCount);

			const time = Date.now() * 0.001;

			points.rotation.x = time * 0.1;
			points.rotation.y = time * 0.2;

			renderer.render(scene, camera);
		}
	}

	/*
	nearestNeighbour(canvas) {
		const vertexShader = `//uniform float zoom;

	attribute float alpha;

	varying float vAlpha;

	void main() {

		vAlpha = 1.0 - alpha;

		vec4 mvPosition = modelViewMatrix * vec4( position, 1.0 );

		gl_PointSize = 4.0 * ( 300.0 / -mvPosition.z );

		gl_Position = projectionMatrix * mvPosition;

	}`;

		const fragmentShader = `uniform sampler2D tex1;

	varying float vAlpha;

	void main() {

		gl_FragColor = texture2D( tex1, gl_PointCoord );
		gl_FragColor.r = ( 1.0 - gl_FragColor.r ) * vAlpha + gl_FragColor.r;

	}`;

		const context = canvas.getContext('webgl2') as any;

		var camera, scene, renderer;
		var controls;

		var amountOfParticles = 5000,
			maxDistance = Math.pow(120, 2);
		var positions, alphas, particles, _particleGeom;
		var kdtree;

		var clock = new THREE.Clock();

		const init = () => {
			camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 1, 1000000);

			scene = new THREE.Scene();

			// add a skybox background
			var cubeTextureLoader = new THREE.CubeTextureLoader();

			cubeTextureLoader.setPath(this.root + '/textures/cube/skyboxsun25deg/');

			var cubeTexture = cubeTextureLoader.load(['px.jpg', 'nx.jpg', 'py.jpg', 'ny.jpg', 'pz.jpg', 'nz.jpg']);

			scene.background = cubeTexture;

			renderer = new THREE.WebGLRenderer({ context });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);
			controls = new FirstPersonControls(camera, canvas);
			controls.movementSpeed = 100;
			controls.lookSpeed = 0.1;

			controls.lookAt(500, 500, 500);

			// create the custom shader

			var textureLoader = new THREE.TextureLoader();

			var imagePreviewTexture = textureLoader.load(this.root + '/textures/crate.gif');

			imagePreviewTexture.minFilter = THREE.LinearMipmapLinearFilter;
			imagePreviewTexture.magFilter = THREE.LinearFilter;

			var pointShaderMaterial = new THREE.ShaderMaterial({
				uniforms: {
					tex1: { value: imagePreviewTexture },
					zoom: { value: 9.0 },
				},
				vertexShader,
				fragmentShader,
				transparent: true,
			});

			//create particles with buffer geometry
			var distanceFunction = function (a, b) {
				return Math.pow(a[0] - b[0], 2) + Math.pow(a[1] - b[1], 2) + Math.pow(a[2] - b[2], 2);
			};

			positions = new Float32Array(amountOfParticles * 3);
			alphas = new Float32Array(amountOfParticles);

			_particleGeom = new THREE.BufferGeometry();
			_particleGeom.setAttribute('position', new THREE.BufferAttribute(positions, 3));
			_particleGeom.setAttribute('alpha', new THREE.BufferAttribute(alphas, 1));

			particles = new Points(_particleGeom, pointShaderMaterial);

			for (var x = 0; x < amountOfParticles; x++) {
				positions[x * 3 + 0] = Math.random() * 1000;
				positions[x * 3 + 1] = Math.random() * 1000;
				positions[x * 3 + 2] = Math.random() * 1000;

				alphas[x] = 1.0;
			}

			var measureStart = new Date().getTime();

			// creating the kdtree takes a lot of time to execute, in turn the nearest neighbour search will be much faster
			kdtree = new TypedArrayUtils.Kdtree(positions, distanceFunction, 3);

			console.log('TIME building kdtree', new Date().getTime() - measureStart);

			// display particles after the kd-tree was generated and the sorting of the positions-array is done
			scene.add(particles);

			window.addEventListener('resize', onWindowResize, false);
		};

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);

			controls.handleResize();
		}

		function animate() {
			requestAnimationFrame(animate);

			//
			displayNearest(camera.position);

			controls.update(clock.getDelta());

			renderer.render(scene, camera);
		}

		function displayNearest(position) {
			// take the nearest 200 around him. distance^2 'cause we use the manhattan distance and no square is applied in the distance function
			var imagePositionsInRange = kdtree.nearest([position.x, position.y, position.z], 100, maxDistance);

			// We combine the nearest neighbour with a view frustum. Doesn't make sense if we change the sprites not in our view... well maybe it does. Whatever you want.
			var _frustum = new THREE.Frustum();
			var _projScreenMatrix = new THREE.Matrix4();

			_projScreenMatrix.multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse);
			_frustum.setFromProjectionMatrix(_projScreenMatrix);

			for (var i = 0, il = imagePositionsInRange.length; i < il; i++) {
				var object = imagePositionsInRange[i];
				var objectPoint = new THREE.Vector3().fromArray(object[0].obj);

				if (_frustum.containsPoint(objectPoint)) {
					var objectIndex = object[0].pos;

					// set the alpha according to distance
					alphas[objectIndex] = (1.0 / maxDistance) * object[1];

					// update the attribute
					_particleGeom.attributes.alpha.needsUpdate = true;
				}
			}
		}

		init();
		animate();
	}
	*/

	skinningAndMorphing(canvas) {
		const context = canvas.getContext('webgl2') as any;

		const { drawingBufferWidth: width, drawingBufferHeight: height } = context;
		var container, stats, clock, gui, mixer, actions, activeAction, previousAction;
		var camera, scene, renderer, model, face;

		var api = { state: 'Walking' };

		const init = () => {
			camera = new THREE.PerspectiveCamera(45, width / height, 0.25, 100);
			camera.position.set(-5, 3, 10);
			camera.lookAt(new THREE.Vector3(0, 2, 0));

			scene = new THREE.Scene();
			scene.background = new THREE.Color(0xe0e0e0);
			scene.fog = new THREE.Fog(0xe0e0e0, 20, 100);

			clock = new THREE.Clock();

			// lights

			var light = new THREE.HemisphereLight(0xffffff, 0x444444);
			light.position.set(0, 20, 0);
			scene.add(light);

			light = new THREE.DirectionalLight(0xffffff) as any;
			light.position.set(0, 20, 10);
			scene.add(light);

			// ground

			var mesh = new THREE.Mesh(
				new THREE.PlaneBufferGeometry(2000, 2000),
				new THREE.MeshPhongMaterial({
					color: 0x999999,
					depthWrite: false,
				})
			);
			mesh.rotation.x = -Math.PI / 2;
			scene.add(mesh);

			var grid = new THREE.GridHelper(200, 40, 0x000000, 0x000000) as any;
			grid.material.opacity = 0.2;
			grid.material.transparent = true;
			scene.add(grid);

			// model

			var loader = new GLTFLoader();
			loader.load(
				this.root + '/models/gltf/RobotExpressive/RobotExpressive.glb',
				function (gltf) {
					console.log('loaded');
					model = gltf.scene;
					scene.add(model);

					createGUI(model, gltf.animations);
				},
				undefined,
				function (e) {
					console.error(e);
				}
			);

			renderer = new THREE.WebGLRenderer({ context, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(width, height);
			renderer.outputEncoding = THREE.sRGBEncoding;
			//container.appendChild( renderer.domElement );

			window.addEventListener('resize', onWindowResize, false);

			const controls = new OrbitControls(camera, canvas);
			controls.update();

			// stats
			/*stats = new Stats();
			container.appendChild( stats.dom );*/
		};

		function createGUI(model, animations) {
			var states = ['Idle', 'Walking', 'Running', 'Dance', 'Death', 'Sitting', 'Standing'];
			var emotes = ['Jump', 'Yes', 'No', 'Wave', 'Punch', 'ThumbsUp'];

			//	gui = new GUI();

			mixer = new THREE.AnimationMixer(model);

			actions = {};

			for (var i = 0; i < animations.length; i++) {
				var clip = animations[i];
				var action = mixer.clipAction(clip);
				actions[clip.name] = action;

				if (emotes.indexOf(clip.name) >= 0 || states.indexOf(clip.name) >= 4) {
					action.clampWhenFinished = true;
					action.loop = THREE.LoopOnce;
				}
			}

			// states

			/*
			var statesFolder = gui.addFolder( 'States' );

			var clipCtrl = statesFolder.add( api, 'state' ).options( states );

			clipCtrl.onChange( function () {

				fadeToAction( api.state, 0.5 );

			} );

			statesFolder.open();

			// emotes

							var emoteFolder = gui.addFolder( 'Emotes' );

							*/

			function createEmoteCallback(name) {
				api[name] = function () {
					fadeToAction(name, 0.2);

					mixer.addEventListener('finished', restoreState);
				};

				//emoteFolder.add( api, name );
			}

			function restoreState() {
				mixer.removeEventListener('finished', restoreState);

				fadeToAction(api.state, 0.2);
			}

			for (var i = 0; i < emotes.length; i++) {
				createEmoteCallback(emotes[i]);
			}

			//	emoteFolder.open();

			// expressions

			face = model.getObjectByName('Head_2');

			/*	var expressions = Object.keys( face.morphTargetDictionary );
			var expressionFolder = gui.addFolder( 'Expressions' );

			for ( var i = 0; i < expressions.length; i ++ ) {

				expressionFolder.add( face.morphTargetInfluences, i, 0, 1, 0.01 ).name( expressions[ i ] );

			}*/

			activeAction = actions['Walking'];
			activeAction.play();

			//expressionFolder.open();

			setTimeout(() => {
				fadeToAction('Punch', 0.2);
				setTimeout(() => {
					fadeToAction('Jump', 0.2);
					setTimeout(() => {
						fadeToAction('Running', 0.2);
					}, 5000);
				}, 5000);
			}, 5000);
		}

		function fadeToAction(name, duration) {
			previousAction = activeAction;
			activeAction = actions[name];

			if (previousAction !== activeAction) {
				previousAction.fadeOut(duration);
			}

			activeAction.reset().setEffectiveTimeScale(1).setEffectiveWeight(1).fadeIn(duration).play();
		}

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		//

		var last = 0;

		function animate(d = 0) {
			var delta = 0;
			if (last != 0) {
				delta = (d - last) / 1000;
			}
			var dt = clock.getDelta();

			if (mixer) mixer.update(last == 0 ? 0 : delta);

			requestAnimationFrame(animate);

			renderer.render(scene, camera);

			//stats.update();

			last = d;
		}

		init();
		animate();
	}

	threeOcean(canvas) {
		var container, stats;
		var camera, scene, renderer;
		var controls, water, sun, mesh, mesh2, mesh3;
		const context = canvas.getContext('webgl2', { antialias: false }) as any;
		renderer = new THREE.WebGLRenderer({ context, antialias: false });
		renderer.setPixelRatio(1);
		renderer.setSize(context.drawingBufferWidth, context.drawingBufferHeight);
		scene = new THREE.Scene();
		camera = new THREE.PerspectiveCamera(55, context.drawingBufferWidth / context.drawingBufferHeight, 1, 20000);
		camera.position.set(30, 30, 100);

		//
		sun = new THREE.Vector3();

		// Water

		var waterGeometry = new THREE.PlaneBufferGeometry(10000, 10000);

		water = new Water(waterGeometry, {
			textureWidth: 512,
			textureHeight: 512,
			waterNormals: new THREE.TextureLoader().load(this.root + '/textures/waternormals.jpg', function (texture) {
				texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
			}),
			alpha: 1.0,
			sunDirection: new THREE.Vector3(),
			sunColor: 0xffffff,
			waterColor: 0x001e0f,
			distortionScale: 3.7,
			fog: scene.fog !== undefined,
		});

		water.renderOrder = 2;

		water.rotation.x = -Math.PI / 2;

		scene.add(water);

		// Skybox

		var sky = new Sky();
		sky.scale.setScalar(10000);
		scene.add(sky);

		var uniforms = sky.material.uniforms;

		uniforms['turbidity'].value = 10;
		uniforms['rayleigh'].value = 2;
		uniforms['mieCoefficient'].value = 0.005;
		uniforms['mieDirectionalG'].value = 0.8;

		var parameters = {
			inclination: 0.49,
			azimuth: 0.205,
		};

		var pmremGenerator = new THREE.PMREMGenerator(renderer);

		function updateSun() {
			var theta = Math.PI * (parameters.inclination - 0.5);
			var phi = 2 * Math.PI * (parameters.azimuth - 0.5);

			sun.x = Math.cos(phi);
			sun.y = Math.sin(phi) * Math.sin(theta);
			sun.z = Math.sin(phi) * Math.cos(theta);

			sky.material.uniforms['sunPosition'].value.copy(sun);
			water.material.uniforms['sunDirection'].value.copy(sun).normalize();

			scene.environment = pmremGenerator.fromScene(sky as any).texture;
		}

		updateSun();

		//
		/*
				const texture = document.createElement('video');
				texture.loop = true;
				texture.muted = true;
				texture.src = 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4';
				texture.play();
				*/

		var geometry = new THREE.BoxBufferGeometry(30, 30, 30);
		var material = new THREE.MeshStandardMaterial({
			transparent: true,
			roughness: 0,
		});

		//material.map = new THREE.VideoTexture(texture);

		//material.depthTest = true;
		//material.depthWrite = true;

		mesh = new THREE.Mesh(geometry, material);

		scene.add(mesh);

		//

		controls = new OrbitControls(camera, canvas);
		controls.maxPolarAngle = Math.PI * 0.495;
		controls.target.set(0, 10, 0);
		controls.minDistance = 40.0;
		controls.maxDistance = 200.0;
		controls.update();

		/*
		//

		stats = new Stats();
		container.appendChild( stats.dom );

		// GUI

		var gui = new GUI();

		var folder = gui.addFolder( 'Sky' );
		folder.add( parameters, 'inclination', 0, 0.5, 0.0001 ).onChange( updateSun );
		folder.add( parameters, 'azimuth', 0, 1, 0.0001 ).onChange( updateSun );
		folder.open();

		var uniforms = water.material.uniforms;

		var folder = gui.addFolder( 'Water' );
		folder.add( uniforms.distortionScale, 'value', 0, 8, 0.1 ).name( 'distortionScale' );
		folder.add( uniforms.size, 'value', 0.1, 10, 0.1 ).name( 'size' );
		folder.add( uniforms.alpha, 'value', 0.9, 1, .001 ).name( 'alpha' );
		folder.open();
		*/

		//

		function onWindowResize() {
			camera.aspect = context.drawingBufferWidth / context.drawingBufferHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(context.drawingBufferWidth, context.drawingBufferHeight);
		}

		function animate() {
			requestAnimationFrame(animate);
			render();
			// stats.update();
		}

		function render() {
			var time = performance.now() * 0.001;

			mesh.position.y = Math.sin(time) * 20 + 5;
			mesh.rotation.x = time * 0.5;
			mesh.rotation.z = time * 0.51;

			water.material.uniforms['time'].value += 1.0 / 60.0;

			renderer.render(scene, camera);
		}

		window.addEventListener('resize', onWindowResize, false);

		animate();
	}

	threeCrate(canvas) {
		var camera, scene, renderer;
		var mesh;
		const context = canvas.getContext('webgl2') as any;
		const { drawingBufferWidth: width, drawingBufferHeight: height } = context;

		const init = () => {
			camera = new THREE.PerspectiveCamera(70, width / height, 1, 1000);
			camera.position.z = 400;

			scene = new THREE.Scene();

			var texture = new THREE.TextureLoader().load(this.root + '/textures/crate.gif');
			// texture.flipY = false;
			var geometry = new THREE.BoxBufferGeometry(200, 200, 200);
			var material = new THREE.MeshBasicMaterial({ map: texture });

			mesh = new THREE.Mesh(geometry, material);
			scene.add(mesh);

			renderer = new THREE.WebGLRenderer({ context, alpha: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(width, height);

			window.addEventListener('resize', onWindowResize, false);
		};

		function onWindowResize() {
			camera.aspect = width / height;
			camera.updateProjectionMatrix();

			renderer.setSize(width, height);
		}

		function animate() {
			requestAnimationFrame(animate);

			mesh.rotation.x += 0.005;
			mesh.rotation.y += 0.01;

			renderer.render(scene, camera);
		}

		init();
		animate();
	}

	threeDepth(canvas) {
		const gl = canvas.getContext('webgl2') as any;
		const renderer = new THREE.WebGLRenderer({
			context: gl,
		});
		renderer.shadowMap.enabled = true;
		renderer.shadowMap.type = THREE.BasicShadowMap;

		renderer.setSize(gl.drawingBufferWidth, gl.drawingBufferHeight);
		renderer.setClearColor(0xffffff, 1.0);
		renderer.shadowMap.enabled = true;
		const vp = new THREE.Vector4();
		const size = new THREE.Vector2();

		// Standard Camera
		const camera = new THREE.PerspectiveCamera(100, gl.drawingBufferWidth / gl.drawingBufferHeight, 0.1, 1000);
		camera.position.set(10, 10, 0);
		camera.lookAt(0, 0, 0);

		const scene = new THREE.Scene();

		scene.add(new THREE.AmbientLight(0xffffff, 0.5));

		// Three's lights use depth and stencil buffers.
		const light = new THREE.DirectionalLight(0xffffff, 0.5);
		light.position.set(0, 6, 0);
		light.castShadow = true;
		light.shadow.camera.left = -1;
		light.shadow.camera.right = 1;
		light.shadow.camera.top = -1;
		light.shadow.camera.bottom = 1;
		scene.add(light);

		const shadowHelper = new THREE.DirectionalLightHelper(light, 2, 0x0000ff);
		scene.add(shadowHelper);

		// Create a plane that receives shadows (but does not cast them).
		const planeGeometry = new THREE.PlaneBufferGeometry(10, 10, 32, 32);
		const planeMaterial = new THREE.MeshStandardMaterial({
			color: 0x00ff00,
			side: THREE.DoubleSide,
		});

		const plane = new THREE.Mesh(planeGeometry, planeMaterial);
		plane.receiveShadow = true;
		plane.rotation.x = Math.PI / 2;
		plane.position.y = -2;
		scene.add(plane);

		const cube = new THREE.Mesh(
			new THREE.BoxGeometry(1.2, 1.2, 1.2),
			new THREE.MeshPhongMaterial({
				color: 0xffff00,
			})
		);
		cube.castShadow = true;
		cube.receiveShadow = true;
		cube.renderOrder = 3;
		scene.add(cube);

		const another = new THREE.Mesh(
			new THREE.BoxGeometry(1.4, 1.4, 1.4),
			new THREE.MeshPhongMaterial({
				color: 0xff0000,
			})
		);
		another.position.set(0, 2, 0);
		another.castShadow = true;
		another.receiveShadow = true;
		another.renderOrder = 1;
		scene.add(another);

		const helper = new THREE.CameraHelper(light.shadow.camera);
		scene.add(helper);

		const animate = () => {
			requestAnimationFrame(animate);
			cube.rotation.x += 0.01;
			cube.rotation.y += 0.01;
			renderer.render(scene, camera);
		};

		animate();
		renderer.render(scene, camera);
	}

	webgl_buffergeometry_drawrange(canvas) {
		let group;
		let container, stats;
		const particlesData = [];
		let camera, scene, renderer;
		let positions, colors;
		let particles;
		let pointCloud;
		let particlePositions;
		let linesMesh;

		const maxParticleCount = 1000;
		let particleCount = 500;
		const r = 800;
		const rHalf = r / 2;

		const effectController = {
			showDots: true,
			showLines: true,
			minDistance: 150,
			limitConnections: false,
			maxConnections: 20,
			particleCount: 500,
		};

		init();
		animate();

		function initGUI() {
			/*const gui = new GUI();

			gui.add( effectController, "showDots" ).onChange( function ( value ) {

				pointCloud.visible = value;

			} );
			gui.add( effectController, "showLines" ).onChange( function ( value ) {

				linesMesh.visible = value;

			} );
			gui.add( effectController, "minDistance", 10, 300 );
			gui.add( effectController, "limitConnections" );
			gui.add( effectController, "maxConnections", 0, 30, 1 );
			gui.add( effectController, "particleCount", 0, maxParticleCount, 1 ).onChange( function ( value ) {

				particleCount = parseInt( value );
				particles.setDrawRange( 0, particleCount );

			} );
			*/
		}

		function init() {
			initGUI();

			container = canvas;

			camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 4000);
			camera.position.z = 1750;

			const controls = new OrbitControls(camera, container);
			controls.minDistance = 1000;
			controls.maxDistance = 3000;

			scene = new THREE.Scene();

			group = new THREE.Group();
			scene.add(group);

			const helper = new THREE.BoxHelper(new THREE.Mesh(new THREE.BoxGeometry(r, r, r))) as any;
			helper.material.color.setHex(0x101010);
			helper.material.blending = THREE.AdditiveBlending;
			helper.material.transparent = true;
			group.add(helper);

			const segments = maxParticleCount * maxParticleCount;

			positions = new Float32Array(segments * 3);
			colors = new Float32Array(segments * 3);

			const pMaterial = new THREE.PointsMaterial({
				color: 0xffffff,
				size: 3,
				blending: THREE.AdditiveBlending,
				transparent: true,
				sizeAttenuation: false,
			});

			particles = new THREE.BufferGeometry();
			particlePositions = new Float32Array(maxParticleCount * 3);

			for (let i = 0; i < maxParticleCount; i++) {
				const x = Math.random() * r - r / 2;
				const y = Math.random() * r - r / 2;
				const z = Math.random() * r - r / 2;

				particlePositions[i * 3] = x;
				particlePositions[i * 3 + 1] = y;
				particlePositions[i * 3 + 2] = z;

				// add it to the geometry
				particlesData.push({
					velocity: new THREE.Vector3(-1 + Math.random() * 2, -1 + Math.random() * 2, -1 + Math.random() * 2),
					numConnections: 0,
				});
			}

			particles.setDrawRange(0, particleCount);
			particles.setAttribute('position', new THREE.BufferAttribute(particlePositions, 3).setUsage(THREE.DynamicDrawUsage));

			// create the particle system
			pointCloud = new THREE.Points(particles, pMaterial);
			group.add(pointCloud);

			const geometry = new THREE.BufferGeometry();

			geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3).setUsage(THREE.DynamicDrawUsage));
			geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3).setUsage(THREE.DynamicDrawUsage));

			geometry.computeBoundingSphere();

			geometry.setDrawRange(0, 0);

			const material = new THREE.LineBasicMaterial({
				vertexColors: true,
				blending: THREE.AdditiveBlending,
				transparent: true,
			});

			linesMesh = new THREE.LineSegments(geometry, material);
			group.add(linesMesh);

			//

			const ctx = canvas.getContext('webgl2');
			renderer = new THREE.WebGLRenderer({ context: ctx, antialias: true });
			renderer.setPixelRatio(window.devicePixelRatio);
			renderer.setSize(window.innerWidth, window.innerHeight);
			renderer.outputEncoding = THREE.sRGBEncoding;

			//

			//stats = new Stats();
			//container.appendChild( stats.dom );

			window.addEventListener('resize', onWindowResize);
		}

		function onWindowResize() {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();

			renderer.setSize(window.innerWidth, window.innerHeight);
		}

		function animate() {
			let vertexpos = 0;
			let colorpos = 0;
			let numConnected = 0;

			for (let i = 0; i < particleCount; i++) particlesData[i].numConnections = 0;

			for (let i = 0; i < particleCount; i++) {
				// get the particle
				const particleData = particlesData[i];

				particlePositions[i * 3] += particleData.velocity.x;
				particlePositions[i * 3 + 1] += particleData.velocity.y;
				particlePositions[i * 3 + 2] += particleData.velocity.z;

				if (particlePositions[i * 3 + 1] < -rHalf || particlePositions[i * 3 + 1] > rHalf) particleData.velocity.y = -particleData.velocity.y;

				if (particlePositions[i * 3] < -rHalf || particlePositions[i * 3] > rHalf) particleData.velocity.x = -particleData.velocity.x;

				if (particlePositions[i * 3 + 2] < -rHalf || particlePositions[i * 3 + 2] > rHalf) particleData.velocity.z = -particleData.velocity.z;

				if (effectController.limitConnections && particleData.numConnections >= effectController.maxConnections) continue;

				// Check collision
				for (let j = i + 1; j < particleCount; j++) {
					const particleDataB = particlesData[j];
					if (effectController.limitConnections && particleDataB.numConnections >= effectController.maxConnections) continue;

					const dx = particlePositions[i * 3] - particlePositions[j * 3];
					const dy = particlePositions[i * 3 + 1] - particlePositions[j * 3 + 1];
					const dz = particlePositions[i * 3 + 2] - particlePositions[j * 3 + 2];
					const dist = Math.sqrt(dx * dx + dy * dy + dz * dz);

					if (dist < effectController.minDistance) {
						particleData.numConnections++;
						particleDataB.numConnections++;

						const alpha = 1.0 - dist / effectController.minDistance;

						positions[vertexpos++] = particlePositions[i * 3];
						positions[vertexpos++] = particlePositions[i * 3 + 1];
						positions[vertexpos++] = particlePositions[i * 3 + 2];

						positions[vertexpos++] = particlePositions[j * 3];
						positions[vertexpos++] = particlePositions[j * 3 + 1];
						positions[vertexpos++] = particlePositions[j * 3 + 2];

						colors[colorpos++] = alpha;
						colors[colorpos++] = alpha;
						colors[colorpos++] = alpha;

						colors[colorpos++] = alpha;
						colors[colorpos++] = alpha;
						colors[colorpos++] = alpha;

						numConnected++;
					}
				}
			}

			linesMesh.geometry.setDrawRange(0, numConnected * 2);
			linesMesh.geometry.attributes.position.needsUpdate = true;
			linesMesh.geometry.attributes.color.needsUpdate = true;

			pointCloud.geometry.attributes.position.needsUpdate = true;

			requestAnimationFrame(animate);

			//stats.update();
			render();
		}

		function render() {
			const time = Date.now() * 0.001;

			group.rotation.y = time * 0.1;
			renderer.render(scene, camera);
		}
	}
}
