import * as THREE from 'three/webgpu';
import { color, cos, float, mix, range, sin, timerLocal, uniform, uv, vec3, vec4, PI, PI2, Fn } from 'three/tsl';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
export async function run(canvas) {
	let camera, scene, renderer, controls, context;

	init();

	async function init() {
		canvas.width = canvas.clientWidth * window.devicePixelRatio;
		canvas.height = canvas.clientHeight * window.devicePixelRatio;

		camera = new THREE.PerspectiveCamera(75, canvas.clientWidth / canvas.clientHeight, 0.1, 1000);
		camera.position.set(4, 2, 5);
		//camera.position.set( 0,0, 5);

		scene = new THREE.Scene();
		scene.background = new THREE.Color(0x201919);

		// galaxy

		const material = new THREE.SpriteNodeMaterial({
			transparent: true,
			depthWrite: false,
			blending: THREE.AdditiveBlending,
		});

		const size = uniform(0.08);
		material.scaleNode = range(0, 1).mul(size);

		const time = timerLocal();

		const radiusRatio = range(0, 1);
		const radius = radiusRatio.pow(1.5).mul(5).toVar();

		const branches = 3;
		const branchAngle = range(0, branches).floor().mul(PI2.div(branches));
		const angle = branchAngle.add(time.mul(radiusRatio.oneMinus()));

		const position = vec3(cos(angle), 0, sin(angle)).mul(radius);

		const sphericalToVec3 = Fn(([phi, theta]) => {
			const sinPhiRadius = sin(phi);

			return vec3(sinPhiRadius.mul(sin(theta)), cos(phi), sinPhiRadius.mul(cos(theta)));
		});

		const phi = range(0, PI2);
		const theta = range(0, PI);
		const offsetRadius = range(0, 1).pow(2).mul(radiusRatio).mul(1.25);
		const randomOffset = sphericalToVec3(phi, theta).mul(offsetRadius);

		material.positionNode = position.add(randomOffset);

		const colorInside = uniform(color('#ffa575'));
		const colorOutside = uniform(color('#311599'));
		const colorFinal = mix(colorInside, colorOutside, radiusRatio.oneMinus().pow(2).oneMinus());
		const alpha = float(0.1).div(uv().sub(0.5).length()).sub(0.2);
		material.colorNode = vec4(colorFinal, alpha);

		const mesh = new THREE.InstancedMesh(new THREE.PlaneGeometry(1, 1), material, 20000);
		scene.add(mesh);

		// debug
		/*
			const gui = new GUI();

			gui.add( size, 'value', 0, 1, 0.001 ).name( 'size' );

			gui.addColor( { color: colorInside.value.getHex( THREE.SRGBColorSpace ) }, 'color' )
				.name( 'colorInside' )
				.onChange( function ( value ) {

					colorInside.value.set( value );

				} );

			gui.addColor( { color: colorOutside.value.getHex( THREE.SRGBColorSpace ) }, 'color' )
				.name( 'colorOutside' )
				.onChange( function ( value ) {

					colorOutside.value.set( value );

				} );

				*/

		// renderer

		renderer = new THREE.WebGPURenderer({ canvas, antialias: false });
		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);
		await renderer.init();
		context = canvas.getContext('webgpu');

		renderer.setAnimationLoop(animate);
		// document.body.appendChild( renderer.domElement );

		controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.minDistance = 0.1;
		controls.maxDistance = 50;

		window.addEventListener('resize', onWindowResize);
	}

	function onWindowResize() {
		camera.aspect = canvas.clientWidth / canvas.clientHeight;
		camera.updateProjectionMatrix();
		renderer.setSize(canvas.clientWidth, canvas.clientHeight, false);
	}

	async function animate() {
		controls.update();

		renderer.render(scene, camera);

		context.presentSurface();
	}
}
