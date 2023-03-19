import * as THREE from 'three';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
import { Water } from './Water.js';
import { Sky } from 'three/examples/jsm/objects/Sky';

let camera, scene, renderer;
let renderTarget;
let pmremGenerator;
let sky;

let controls, water, sun, mesh;

let shipContainer = new THREE.Group();
let ship;
let drones = [];
let shotMesh;
const parameters = {
	elevation: 0,
	azimuth: 150,
};

var emitter = new THREE.Object3D();
shipContainer.add(emitter);

var shots = [];
let width = 0;
let height = 0;
const root = '~/assets/x-jet/';
let context;
export function init(canvas) {
	context = canvas.getContext('webgl2');
	width = context.drawingBufferWidth;
	height = context.drawingBufferHeight;
	renderer = new THREE.WebGLRenderer({ context, antialias: false });
	renderer.setPixelRatio(1); // reduce this value for better quality 1 is original too heavy and not needed for mobile
	renderer.setSize(width, height);
	renderer.toneMapping = THREE.ACESFilmicToneMapping;
	renderer.outputEncoding = THREE.sRGBEncoding;
	//document.body.appendChild(renderer.domElement);

	scene = new THREE.Scene();

	camera = new THREE.PerspectiveCamera(55, width / height, 1, 5000);
	camera.position.set(0, 30, 100);
	const dirLight = new THREE.DirectionalLight(0xffbb99, 1);
	dirLight.position.set(30, 80, 30);
	scene.add(dirLight);
	const ambLight = new THREE.AmbientLight(0xffffff, 1);
	scene.add(ambLight);

	sun = new THREE.Vector3();

	const waterGeometry = new THREE.PlaneGeometry(15000, 15000);
	water = new Water(waterGeometry, {
		textureWidth: 512,
		textureHeight: 512,
		waterNormals: new THREE.TextureLoader().load(root + 'waternormals.jpg', function (texture) {
			texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
		}),
		sunDirection: new THREE.Vector3(),
		sunColor: 0xffffff,
		waterColor: 0x001e0f,
		distortionScale: 8,
		fog: scene.fog !== undefined,
	});
	water.material.transparent = true;
	water.rotation.x = -Math.PI / 2;

	scene.add(water);

	sky = new Sky();
	sky.scale.setScalar(3500);
	scene.add(sky);

	const skyUniforms = sky.material.uniforms;

	skyUniforms['turbidity'].value = 2;
	skyUniforms['rayleigh'].value = 1;
	skyUniforms['mieCoefficient'].value = 0.005;
	skyUniforms['mieDirectionalG'].value = 0.82;

	pmremGenerator = new THREE.PMREMGenerator(renderer);

	const loader = new GLTFLoader();
	loader.setPath(root).load('TriniShip.glb', function (gltf) {
		ship = gltf.scene;

		shipContainer.add(ship);
		ship.scale.setScalar(10);
		ship.position.set(0, 30, -30);
		ship.rotation.y = Math.PI;
		scene.add(shipContainer);

		ship.children[1].children[0].material.roughness = 0.2;
		ship.children[1].children[0].material.metalness = 0.8;

		animate();
	});

	const droneLoader = new GLTFLoader();
	droneLoader.setPath(root).load('Drone.glb', function (gltf) {
		const droneContainer = new THREE.Group();
		const map = new THREE.TextureLoader().load(root + 'explosion.png');
		const material = new THREE.SpriteMaterial({ map: map, transparent: true });
		material.depthWrite = false;
		const explosion = new THREE.Sprite(material);
		explosion.visible = false;
		explosion.scale.set(300, 300);

		const drone = gltf.scene;
		droneContainer.add(drone);
		droneContainer.add(explosion);
		drone.scale.setScalar(7);
		drones.push(droneContainer.clone());
		drones.push(droneContainer.clone());
		drones.push(droneContainer.clone());
		drones.push(droneContainer.clone());
		drones.push(droneContainer.clone());
		drones.push(droneContainer.clone());
		drones.push(droneContainer.clone());
	});

	updateSun();

	controls = new OrbitControls(camera, canvas);
	controls.enableDamping = true;
	controls.maxPolarAngle = Math.PI * 0.495;
	controls.minDistance = 40.0;
	controls.maxDistance = 200.0;
	//

	const keyDown = (event) => {
		switch (event.keyCode) {
			case 65: // Left arrow key
				isLeftArrowKeyPressed = true;
				break;
			case 68: // Right arrow key
				isRightArrowKeyPressed = true;
				break;
			case 87: // Up
				isUpArrowKeyPressed = true;
				break;
			case 88: // Down
				isDownArrowKeyPressed = true;
				break;
		}
	};

	const keyUp = (event) => {
		switch (event.keyCode) {
			case 65: // Left arrow key
				isLeftArrowKeyPressed = false;
				break;
			case 68: // Right arrow key
				isRightArrowKeyPressed = false;
				break;
			case 87: // Up
				isUpArrowKeyPressed = false;
				break;
			case 88: // Down
				isDownArrowKeyPressed = false;
				break;
		}
	};

	shotMesh = new THREE.Mesh(
		new THREE.BoxGeometry(4, 70, 2),
		new THREE.MeshBasicMaterial({
			color: 'white',
			toneMapped: false,
		})
	);

	// window.addEventListener('mousedown', mouseDown);
	// window.addEventListener('keydown', keyDown);
	// window.addEventListener('keyup', keyUp);
	window.addEventListener('resize', onWindowResize);
}

function updateSun() {
	const phi = THREE.MathUtils.degToRad(90 - parameters.elevation);
	const theta = THREE.MathUtils.degToRad(parameters.azimuth);

	sun.setFromSphericalCoords(1, phi, theta);

	sky.material.uniforms['sunPosition'].value.copy(sun);
	water.material.uniforms['sunDirection'].value.copy(sun).normalize();

	if (renderTarget !== undefined) renderTarget.dispose();

	renderTarget = pmremGenerator.fromScene(sky);

	scene.environment = renderTarget.texture;
}
const speed = 500;

const vec = new THREE.Vector3();
function updateShots(delta) {
	shots.forEach((b) => {
		b.translateY(speed * delta);
		simpleCollisionCheck(b.getWorldPosition(vec));
		if (b.position.z < -1700) scene.remove(b);
	});
}
function simpleCollisionCheck(shot) {
	const dvec = new THREE.Vector3();
	drones.forEach((drone) => {
		drone.getWorldPosition(dvec);
		if (Math.abs(dvec.x - shot.x) < 100 && Math.abs(dvec.y - shot.y) < 100 && Math.abs(dvec.z - shot.z) < 100) drone.userData.mustExplode = true;
	});
}

let pingpong = 0;

function mouseDown(event) {
	const shot = shotMesh.clone();
	emitter.position.set(pingpong % 2 ? -28 : 28, 32, -55);
	emitter.getWorldPosition(shot.position);
	shot.quaternion.copy(shipContainer.quaternion);
	const destX = (event.pageX - window.innerWidth / 2) / window.innerWidth;
	shot.rotation.set(-Math.PI / 2, 0, -destX);
	scene.add(shot);
	shots.push(shot);
	pingpong++;
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

let rotationSpeedX = 0;
let rotationSpeedY = 0;
let maxRotation = 0.3;
let movementSpeed = 100;
let rotationEasing = 0.05;
let isLeftArrowKeyPressed = false;
let isRightArrowKeyPressed = false;
let isDownArrowKeyPressed = false;
let isUpArrowKeyPressed = false;

const updateShip = (delta) => {
	// Update the rotation speed based on user input and easing
	if (isLeftArrowKeyPressed) {
		rotationSpeedX = THREE.MathUtils.lerp(rotationSpeedX, -1, delta / rotationEasing);
	} else if (isRightArrowKeyPressed) {
		rotationSpeedX = THREE.MathUtils.lerp(rotationSpeedX, 1, delta / rotationEasing);
	} else if (isUpArrowKeyPressed) {
		rotationSpeedY = THREE.MathUtils.lerp(rotationSpeedY, 1, delta / rotationEasing);
	} else if (isDownArrowKeyPressed) {
		rotationSpeedY = THREE.MathUtils.lerp(rotationSpeedY, -1, delta / rotationEasing);
	} else {
		rotationSpeedX = THREE.MathUtils.lerp(rotationSpeedX, 0, delta / rotationEasing);
		rotationSpeedY = THREE.MathUtils.lerp(rotationSpeedY, 0, delta / rotationEasing);
	}

	// Limit the maximum rotation of the ship
	rotationSpeedX = THREE.MathUtils.clamp(rotationSpeedX, -1, 1);
	rotationSpeedY = THREE.MathUtils.clamp(rotationSpeedY, -1, 1);
	const targetRotationX = rotationSpeedX * maxRotation;
	const targetRotationY = rotationSpeedY * maxRotation;
	ship.rotation.z = THREE.MathUtils.lerp(ship.rotation.z, targetRotationX, delta / rotationEasing);
	ship.rotation.x = THREE.MathUtils.lerp(ship.rotation.x, targetRotationY, delta / rotationEasing);

	// Update the position of the ship
	shipContainer.position.x += rotationSpeedX * movementSpeed * delta;
	shipContainer.position.y += rotationSpeedY * movementSpeed * delta;
	controls.target.y = shipContainer.position.y + 50;
	controls.update();
};

const clock = new THREE.Clock();

const droneRotationEasing = 0.5;

function simulateKeyPress(delta, target) {
	const data = target.userData;
	if (data.timeLeft <= 0) {
		const keys = ['left', 'right', 'up', 'down'];
		data.currentKey = keys[Math.floor(Math.random() * keys.length)]; // Pick a random key

		data.timeLeft = Math.random() * (data.MAX_PRESS_TIME - data.MIN_PRESS_TIME) + data.MIN_PRESS_TIME;
	}

	let rotationSpeedX = 0;
	let rotationSpeedY = 0;

	switch (data.currentKey) {
		case 'left':
			rotationSpeedX = THREE.MathUtils.lerp(rotationSpeedX, -1, (data.MAX_PRESS_TIME - data.timeLeft) / droneRotationEasing);
			break;
		case 'right':
			rotationSpeedX = THREE.MathUtils.lerp(rotationSpeedX, 1, (data.MAX_PRESS_TIME - data.timeLeft) / droneRotationEasing);
			break;
		case 'up':
			rotationSpeedY = THREE.MathUtils.lerp(rotationSpeedY, 1, (data.MAX_PRESS_TIME - data.timeLeft) / droneRotationEasing);
			break;
		case 'down':
			rotationSpeedY = THREE.MathUtils.lerp(rotationSpeedY, -1, (data.MAX_PRESS_TIME - data.timeLeft) / droneRotationEasing);
			break;
	}

	data.timeLeft -= delta;

	rotationSpeedX = THREE.MathUtils.clamp(rotationSpeedX, -1, 1);
	rotationSpeedY = THREE.MathUtils.clamp(rotationSpeedY, -1, 1);
	const targetRotationX = rotationSpeedX * maxRotation;
	const targetRotationY = rotationSpeedY * maxRotation;
	target.rotation.z = THREE.MathUtils.lerp(target.rotation.z, targetRotationX, delta / droneRotationEasing);
	target.rotation.x = THREE.MathUtils.lerp(target.rotation.x, targetRotationY, delta / droneRotationEasing);

	if (target.position.x > -500 && target.position.y < 500) target.position.x += rotationSpeedX * data.droneMovementSpeed * delta;
	if (target.position.y > 0 && target.position.y < 500) target.position.y += rotationSpeedY * data.droneMovementSpeed * delta;

	if (data.nextKeyPressTime === null) {
		data.MAX_PRESS_TIME = 2.0 + Math.random() * 10;
		data.droneMovementSpeed = 150 + Math.random() * 200;
		data.nextKeyPressTime = data.MAX_PRESS_TIME - data.MIN_PRESS_TIME + data.MIN_PRESS_TIME;
		target.userData.speed = 1 + Math.random() * 30;
	} else {
		data.nextKeyPressTime -= delta;
	}
}

const updateDrones = (delta) => {
	drones.forEach((drone) => {
		if (!drone.parent) {
			drone.position.set(-300 + Math.random() * 600, Math.random() * 350, -10000 + Math.random() * 7000);
			drone.userData.timeLeft = 0;
			drone.userData.currentKey = null;
			drone.userData.nextKeyPressTime = null;
			drone.userData.MIN_PRESS_TIME = 2.0 + Math.random() * 2;
			drone.userData.MAX_PRESS_TIME = 2.0 + Math.random() * 10;
			drone.userData.droneMovementSpeed = 150 + Math.random() * 200;
			drone.userData.speed = 1 + Math.random() * 30;
			drone.rotation.y = Math.PI;
			scene.add(drone);
			drone.children[1].material.opacity = 1;
		} else {
			if (drone.position.z < 200) {
				drone.position.z += drone.userData.speed;
				simulateKeyPress(delta, drone);
			} else {
				scene.remove(drone);
			}
		}
	});
};

function collectExplosions() {
	drones.forEach((drone) => {
		if (drone.userData.mustExplode) {
			if (drone.children[1].material.opacity > 0.1) {
				drone.children[1].visible = true;
				drone.children[1].material.opacity -= 0.05;
			} else {
				drone.userData.mustExplode = false;
				drone.children[1].visible = false;
				scene.remove(drone);
			}
		}
	});
}

let frames = 0;
function render() {
	const delta = clock.getDelta();

	if (frames % 5 == 0) {
		const time = performance.now() * 0.0005;
		parameters.elevation = 3 + Math.cos(time / 5) * 5;
		parameters.azimuth = 180 - Math.cos(time / 8) * 20;
		updateSun();
	}

	updateDrones(delta);
	updateShip(delta);
	updateShots(delta);

	collectExplosions();

	water.material.uniforms['time'].value += 1.0 / 60.0;
	water.material.uniforms['time2'].value += 6;

	renderer.render(scene, camera);

	frames++;
}
