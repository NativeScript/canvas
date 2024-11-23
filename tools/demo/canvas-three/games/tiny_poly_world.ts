import * as THREE from 'three';

var Colors = {
	red: 0xf25346,
	yellow: 0xedeb27,
	white: 0xd8d0d1,
	brown: 0x59332e,
	pink: 0xf5986e,
	brownDark: 0x23190f,
	blue: 0x68c3c0,
	green: 0x458248,
	purple: 0x551a8b,
	lightgreen: 0x629265,
};

var scene, camera, fieldOfView, aspectRatio, nearPlane, farPlane, HEIGHT, WIDTH, renderer, container, hemisphereLight, petalCore, canvas;

function createScene(canvas) {
	// Get the width and height of the screen
	// and use them to setup the aspect ratio
	// of the camera and the size of the renderer.
	canvas.width = canvas.clientWidth * window.devicePixelRatio;
	canvas.width = canvas.clientHeight * window.devicePixelRatio;
	HEIGHT = canvas.clientWidth;
	WIDTH = canvas.clientHeight;

	const context = canvas.getContext('webgl2');
	// Create the scene.
	scene = new THREE.Scene();

	// Add FOV Fog effect to the scene. Same colour as the BG int he stylesheet.
	scene.fog = new THREE.Fog(0xf7d9aa, 100, 950);

	// Create the camera
	aspectRatio = WIDTH / HEIGHT;
	fieldOfView = 75;
	nearPlane = 0.1;
	farPlane = 1000;
	camera = new THREE.PerspectiveCamera(fieldOfView, aspectRatio, nearPlane, farPlane);
	// Position the camera
	camera.position.x = 0;
	camera.position.y = 150;
	camera.position.z = 100;

	// Create the renderer

	renderer = new THREE.WebGLRenderer({
		canvas,
		// Alpha makes the background transparent, antialias is performant heavy
		alpha: true,
		antialias: true,
	});
	renderer.outputColorSpace = THREE.SRGBColorSpace;
	//set the size of the renderer to fullscreen
	renderer.setPixelRatio(window.devicePixelRatio);
	renderer.setSize(WIDTH, HEIGHT, false);
	//enable shadow rendering
	renderer.shadowMap.enabled = true;

	// Add the Renderer to the DOM, in the world div.
	//container = document.getElementById('world');
	//container.appendChild(renderer.domElement);

	//RESPONSIVE LISTENER
	window.addEventListener('resize', handleWindowResize, false);
}

//RESPONSIVE FUNCTION
function handleWindowResize() {
	HEIGHT = canvas.height;
	WIDTH = canvas.width;
	renderer.setSize(WIDTH, HEIGHT);
	camera.aspect = WIDTH / HEIGHT;
	camera.updateProjectionMatrix();
}

var hemispshereLight, shadowLight;

function createLights() {
	// Gradient coloured light - Sky, Ground, Intensity
	hemisphereLight = new THREE.HemisphereLight(0xaaaaaa, 0x000000, 0.9);
	// Parallel rays
	shadowLight = new THREE.DirectionalLight(0xffffff, 0.9);

	shadowLight.position.set(0, 350, 350);
	shadowLight.castShadow = true;

	// define the visible area of the projected shadow
	shadowLight.shadow.camera.left = -650;
	shadowLight.shadow.camera.right = 650;
	shadowLight.shadow.camera.top = 650;
	shadowLight.shadow.camera.bottom = -650;
	shadowLight.shadow.camera.near = 1;
	shadowLight.shadow.camera.far = 1000;

	// Shadow map size
	shadowLight.shadow.mapSize.width = 2048;
	shadowLight.shadow.mapSize.height = 2048;

	// Add the lights to the scene
	scene.add(hemisphereLight);

	scene.add(shadowLight);
}

var Land = function () {
	const geom = new THREE.CylinderGeometry(600, 600, 1700, 40, 10);

	// Create a rotation matrix for the x-axis
	const rotationMatrix = new THREE.Matrix4().makeRotationX(-Math.PI / 2);

	// Apply the rotation matrix to the geometry's vertices
	geom.applyMatrix4(rotationMatrix);

	// Create a material
	const mat = new THREE.MeshPhongMaterial({
		color: 0x90ee90, // Use hexadecimal color values
		flatShading: true,
	});

	//create a mesh of the object
	this.mesh = new THREE.Mesh(geom, mat);
	//receive shadows
	this.mesh.receiveShadow = true;
};

var Orbit = function () {
	var geom = new THREE.Object3D();

	this.mesh = geom;
	//this.mesh.add(sun);
};

var Sun = function () {
	this.mesh = new THREE.Object3D();

	var sunGeom = new THREE.SphereGeometry(400, 20, 10);
	var sunMat = new THREE.MeshPhongMaterial({
		color: Colors.yellow,
		flatShading: true,
	});
	var sun = new THREE.Mesh(sunGeom, sunMat);
	//sun.applyMatrix(new THREE.Matrix4().makeRotationX(-Math.PI/2));
	sun.castShadow = false;
	sun.receiveShadow = false;
	this.mesh.add(sun);
};

var Cloud = function () {
	// Create an empty container for the cloud
	this.mesh = new THREE.Object3D();
	// Cube geometry and material
	var geom = new THREE.DodecahedronGeometry(20, 0);
	var mat = new THREE.MeshPhongMaterial({
		color: Colors.white,
	});

	var nBlocs = 3 + Math.floor(Math.random() * 3);

	for (var i = 0; i < nBlocs; i++) {
		//Clone mesh geometry
		var m = new THREE.Mesh(geom, mat);
		//Randomly position each cube
		m.position.x = i * 15;
		m.position.y = Math.random() * 10;
		m.position.z = Math.random() * 10;
		m.rotation.z = Math.random() * Math.PI * 2;
		m.rotation.y = Math.random() * Math.PI * 2;

		//Randomly scale the cubes
		var s = 0.1 + Math.random() * 0.9;
		m.scale.set(s, s, s);
		this.mesh.add(m);
	}
};

var Sky = function () {
	this.mesh = new THREE.Object3D();

	// Number of cloud groups
	this.nClouds = 25;

	// Space the consistenly
	var stepAngle = (Math.PI * 2) / this.nClouds;

	// Create the Clouds

	for (var i = 0; i < this.nClouds; i++) {
		var c = new Cloud();

		//set rotation and position using trigonometry
		var a = stepAngle * i;
		// this is the distance between the center of the axis and the cloud itself
		var h = 800 + Math.random() * 200;
		c.mesh.position.y = Math.sin(a) * h;
		c.mesh.position.x = Math.cos(a) * h;

		// rotate the cloud according to its position
		c.mesh.rotation.z = a + Math.PI / 2;

		// random depth for the clouds on the z-axis
		c.mesh.position.z = -400 - Math.random() * 400;

		// random scale for each cloud
		var s = 1 + Math.random() * 2;
		c.mesh.scale.set(s, s, s);

		this.mesh.add(c.mesh);
	}
};

var Tree = function () {
	this.mesh = new THREE.Object3D();

	var matTreeLeaves = new THREE.MeshPhongMaterial({ color: Colors.green, flatShading: true });

	var geonTreeBase = new THREE.BoxGeometry(10, 20, 10);
	var matTreeBase = new THREE.MeshBasicMaterial({ color: Colors.brown });
	var treeBase = new THREE.Mesh(geonTreeBase, matTreeBase);
	treeBase.castShadow = true;
	treeBase.receiveShadow = true;
	this.mesh.add(treeBase);

	var geomTreeLeaves1 = new THREE.CylinderGeometry(1, 12 * 3, 12 * 3, 4);
	var treeLeaves1 = new THREE.Mesh(geomTreeLeaves1, matTreeLeaves);
	treeLeaves1.castShadow = true;
	treeLeaves1.receiveShadow = true;
	treeLeaves1.position.y = 20;
	this.mesh.add(treeLeaves1);

	var geomTreeLeaves2 = new THREE.CylinderGeometry(1, 9 * 3, 9 * 3, 4);
	var treeLeaves2 = new THREE.Mesh(geomTreeLeaves2, matTreeLeaves);
	treeLeaves2.castShadow = true;
	treeLeaves2.position.y = 40;
	treeLeaves2.receiveShadow = true;
	this.mesh.add(treeLeaves2);

	var geomTreeLeaves3 = new THREE.CylinderGeometry(1, 6 * 3, 6 * 3, 4);
	var treeLeaves3 = new THREE.Mesh(geomTreeLeaves3, matTreeLeaves);
	treeLeaves3.castShadow = true;
	treeLeaves3.position.y = 55;
	treeLeaves3.receiveShadow = true;
	this.mesh.add(treeLeaves3);
};

var Flower = function () {
	this.mesh = new THREE.Object3D();

	var geomStem = new THREE.BoxGeometry(5, 50, 5, 1, 1, 1);
	var matStem = new THREE.MeshPhongMaterial({ color: Colors.green, flatShading: true });
	var stem = new THREE.Mesh(geomStem, matStem);
	stem.castShadow = false;
	stem.receiveShadow = true;
	this.mesh.add(stem);

	var geomPetalCore = new THREE.BoxGeometry(10, 10, 10, 1, 1, 1);
	var matPetalCore = new THREE.MeshPhongMaterial({ color: Colors.yellow, flatShading: true });
	petalCore = new THREE.Mesh(geomPetalCore, matPetalCore);
	petalCore.castShadow = false;
	petalCore.receiveShadow = true;

	var petalColor = petalColors[Math.floor(Math.random() * 3)];

	const geomPetal = new THREE.BoxGeometry(15, 20, 5, 1, 1, 1);
	const matPetal = new THREE.MeshBasicMaterial({ color: petalColor });

	// Modify vertices by accessing them through the geometry's attributes
	geomPetal.attributes.position.array[5] -= 4; // Vertex 5
	geomPetal.attributes.position.array[4] -= 4; // Vertex 4
	geomPetal.attributes.position.array[7] += 4; // Vertex 7
	geomPetal.attributes.position.array[6] += 4; // Vertex 6

	geomPetal.translate(12.5, 0, 3);

	geomPetal.attributes.position.needsUpdate = true; // Update the geometry after modifying positions

	const petal = new THREE.Mesh(geomPetal, matPetal);

	var petals = [];
	for (var i = 0; i < 4; i++) {
		petals[i] = new THREE.Mesh(geomPetal, matPetal);
		petals[i].rotation.z = (i * Math.PI) / 2;
		petals[i].castShadow = true;
		petals[i].receiveShadow = true;
	}

	petalCore.add(petals[0], petals[1], petals[2], petals[3]);
	petalCore.position.y = 25;
	petalCore.position.z = 3;
	this.mesh.add(petalCore);
};

var petalColors = [Colors.red, Colors.yellow, Colors.blue];

var Forest = function () {
	this.mesh = new THREE.Object3D();

	// Number of Trees
	this.nTrees = 300;

	// Space the consistenly
	var stepAngle = (Math.PI * 2) / this.nTrees;

	// Create the Trees

	for (var i = 0; i < this.nTrees; i++) {
		var t = new Tree();

		//set rotation and position using trigonometry
		var a = stepAngle * i;
		// this is the distance between the center of the axis and the tree itself
		var h = 605;
		t.mesh.position.y = Math.sin(a) * h;
		t.mesh.position.x = Math.cos(a) * h;

		// rotate the tree according to its position
		t.mesh.rotation.z = a + (Math.PI / 2) * 3;

		//Andreas Trigo funtime
		//t.mesh.rotation.z = Math.atan2(t.mesh.position.y, t.mesh.position.x)-Math.PI/2;

		// random depth for the tree on the z-axis
		t.mesh.position.z = 0 - Math.random() * 600;

		// random scale for each tree
		var s = 0.3 + Math.random() * 0.75;
		t.mesh.scale.set(s, s, s);

		this.mesh.add(t.mesh);
	}

	// Number of Trees
	this.nFlowers = 350;

	var stepAngle = (Math.PI * 2) / this.nFlowers;

	for (var i = 0; i < this.nFlowers; i++) {
		var f = new Flower();
		var a = stepAngle * i;

		var h = 605;
		f.mesh.position.y = Math.sin(a) * h;
		f.mesh.position.x = Math.cos(a) * h;

		f.mesh.rotation.z = a + (Math.PI / 2) * 3;

		f.mesh.position.z = 0 - Math.random() * 600;

		var s = 0.1 + Math.random() * 0.3;
		f.mesh.scale.set(s, s, s);

		this.mesh.add(f.mesh);
	}
};

var AirPlane = function () {
	this.mesh = new THREE.Object3D();

	// Create the cabin
	var geomCockpit = new THREE.BoxGeometry(80, 50, 50, 1, 1, 1);
	var matCockpit = new THREE.MeshPhongMaterial({ color: Colors.red, flatShading: true });
	// Access vertices through the geometry's attributes

	geomCockpit.attributes.position.array[1] -= 10; // Vertex 1
	geomCockpit.attributes.position.array[2] += 20;

	geomCockpit.attributes.position.array[4] -= 10; // Vertex 4
	geomCockpit.attributes.position.array[5] -= 20;

	geomCockpit.attributes.position.array[7] += 30; // Vertex 7
	geomCockpit.attributes.position.array[8] += 20;

	geomCockpit.attributes.position.array[10] += 30; // Vertex 10
	geomCockpit.attributes.position.array[11] -= 20;

	geomCockpit.attributes.position.needsUpdate = true; // Update the geometry after modifying positions

	var cockpit = new THREE.Mesh(geomCockpit, matCockpit);
	cockpit.castShadow = true;
	cockpit.receiveShadow = true;
	this.mesh.add(cockpit);

	// Create the engine
	var geomEngine = new THREE.BoxGeometry(20, 50, 50, 1, 1, 1);
	var matEngine = new THREE.MeshPhongMaterial({ color: Colors.white, flatShading: true });
	var engine = new THREE.Mesh(geomEngine, matEngine);
	engine.position.x = 40;
	engine.castShadow = true;
	engine.receiveShadow = true;
	this.mesh.add(engine);

	// Create the tail
	var geomTailPlane = new THREE.BoxGeometry(15, 20, 5, 1, 1, 1);
	var matTailPlane = new THREE.MeshPhongMaterial({ color: Colors.red, flatShading: true });
	var tailPlane = new THREE.Mesh(geomTailPlane, matTailPlane);
	tailPlane.position.set(-35, 25, 0);
	tailPlane.castShadow = true;
	tailPlane.receiveShadow = true;
	this.mesh.add(tailPlane);

	// Create the wing
	var geomSideWing = new THREE.BoxGeometry(40, 4, 150, 1, 1, 1);
	var matSideWing = new THREE.MeshPhongMaterial({ color: Colors.red, flatShading: true });

	var sideWingTop = new THREE.Mesh(geomSideWing, matSideWing);
	var sideWingBottom = new THREE.Mesh(geomSideWing, matSideWing);
	sideWingTop.castShadow = true;
	sideWingTop.receiveShadow = true;
	sideWingBottom.castShadow = true;
	sideWingBottom.receiveShadow = true;

	sideWingTop.position.set(20, 12, 0);
	sideWingBottom.position.set(20, -3, 0);
	this.mesh.add(sideWingTop);
	this.mesh.add(sideWingBottom);

	var geomWindshield = new THREE.BoxGeometry(3, 15, 20, 1, 1, 1);
	var matWindshield = new THREE.MeshPhongMaterial({ color: Colors.white, transparent: true, opacity: 0.3, flatShading: true });
	var windshield = new THREE.Mesh(geomWindshield, matWindshield);
	windshield.position.set(5, 27, 0);

	windshield.castShadow = true;
	windshield.receiveShadow = true;

	this.mesh.add(windshield);

	var geomPropeller = new THREE.BoxGeometry(20, 10, 10, 1, 1, 1);

	geomPropeller.attributes.position.array[1] -= 5; // Vertex 1
	geomPropeller.attributes.position.array[2] += 5;

	geomPropeller.attributes.position.array[4] -= 5; // Vertex 4
	geomPropeller.attributes.position.array[5] -= 5;

	geomPropeller.attributes.position.array[7] += 5; // Vertex 7
	geomPropeller.attributes.position.array[8] += 5;

	geomPropeller.attributes.position.array[10] += 5; // Vertex 10
	geomPropeller.attributes.position.array[11] -= 5;

	geomPropeller.attributes.position.needsUpdate = true; // Update the geometry after modifying positions

	var matPropeller = new THREE.MeshPhongMaterial({ color: Colors.brown, flatShading: true });
	this.propeller = new THREE.Mesh(geomPropeller, matPropeller);
	this.propeller.castShadow = true;
	this.propeller.receiveShadow = true;

	var geomBlade1 = new THREE.BoxGeometry(1, 100, 10, 1, 1, 1);
	var geomBlade2 = new THREE.BoxGeometry(1, 10, 100, 1, 1, 1);
	var matBlade = new THREE.MeshPhongMaterial({ color: Colors.brownDark, flatShading: true });

	var blade1 = new THREE.Mesh(geomBlade1, matBlade);
	blade1.position.set(8, 0, 0);
	blade1.castShadow = true;
	blade1.receiveShadow = true;

	var blade2 = new THREE.Mesh(geomBlade2, matBlade);
	blade2.position.set(8, 0, 0);
	blade2.castShadow = true;
	blade2.receiveShadow = true;
	this.propeller.add(blade1, blade2);
	this.propeller.position.set(50, 0, 0);
	this.mesh.add(this.propeller);

	var wheelProtecGeom = new THREE.BoxGeometry(30, 15, 10, 1, 1, 1);
	var wheelProtecMat = new THREE.MeshPhongMaterial({ color: Colors.white, flatShading: true });
	var wheelProtecR = new THREE.Mesh(wheelProtecGeom, wheelProtecMat);
	wheelProtecR.position.set(25, -20, 25);
	this.mesh.add(wheelProtecR);

	var wheelTireGeom = new THREE.BoxGeometry(24, 24, 4);
	var wheelTireMat = new THREE.MeshPhongMaterial({ color: Colors.brownDark, flatShading: true });
	var wheelTireR = new THREE.Mesh(wheelTireGeom, wheelTireMat);
	wheelTireR.position.set(25, -28, 25);

	var wheelAxisGeom = new THREE.BoxGeometry(10, 10, 6);
	var wheelAxisMat = new THREE.MeshPhongMaterial({ color: Colors.brown, flatShading: true });
	var wheelAxis = new THREE.Mesh(wheelAxisGeom, wheelAxisMat);
	wheelTireR.add(wheelAxis);

	this.mesh.add(wheelTireR);

	var wheelProtecL = wheelProtecR.clone();
	wheelProtecL.position.z = -wheelProtecR.position.z;
	this.mesh.add(wheelProtecL);

	var wheelTireL = wheelTireR.clone();
	wheelTireL.position.z = -wheelTireR.position.z;
	this.mesh.add(wheelTireL);

	var wheelTireB = wheelTireR.clone();
	wheelTireB.scale.set(0.5, 0.5, 0.5);
	wheelTireB.position.set(-35, -5, 0);
	this.mesh.add(wheelTireB);

	const suspensionGeom = new THREE.BoxGeometry(4, 20, 4);
	const suspensionMat = new THREE.MeshPhongMaterial({ color: 0xff0000, flatShading: true });

	// Create a matrix for translation
	const translationMatrix = new THREE.Matrix4().makeTranslation(0, 10, 0);

	// Apply the translation matrix to the geometry's vertices
	suspensionGeom.applyMatrix4(translationMatrix);

	const suspension = new THREE.Mesh(suspensionGeom, suspensionMat);

	// Your suspension object is now correctly positioned

	suspension.position.set(-35, -5, 0);
	suspension.rotation.z = -0.3;
	this.mesh.add(suspension);
};

var Fox = function () {
	this.mesh = new THREE.Object3D();

	var redFurMat = new THREE.MeshPhongMaterial({ color: Colors.red, flatShading: true });

	// Create the Body
	var geomBody = new THREE.BoxGeometry(100, 50, 50, 1, 1, 1);
	var body = new THREE.Mesh(geomBody, redFurMat);
	body.castShadow = true;
	body.receiveShadow = true;
	this.mesh.add(body);

	// Create the Chest
	var geomChest = new THREE.BoxGeometry(50, 60, 70, 1, 1, 1);
	var chest = new THREE.Mesh(geomChest, redFurMat);
	chest.position.x = 60;
	chest.castShadow = true;
	chest.receiveShadow = true;
	this.mesh.add(chest);

	// Create the Head
	var geomHead = new THREE.BoxGeometry(40, 55, 50, 1, 1, 1);
	this.head = new THREE.Mesh(geomHead, redFurMat);
	this.head.position.set(80, 35, 0);
	this.head.castShadow = true;
	this.head.receiveShadow = true;

	// Create the Snout
	var geomSnout = new THREE.BoxGeometry(40, 30, 30, 1, 1, 1);
	var snout = new THREE.Mesh(geomSnout, redFurMat);

	geomSnout.attributes.position.setXYZ(0, geomSnout.attributes.position.getX(0), geomSnout.attributes.position.getY(0) - 5, geomSnout.attributes.position.getZ(0) + 5);

	geomSnout.attributes.position.setXYZ(1, geomSnout.attributes.position.getX(1), geomSnout.attributes.position.getY(1) - 5, geomSnout.attributes.position.getZ(1) - 5);

	geomSnout.attributes.position.setXYZ(2, geomSnout.attributes.position.getX(2), geomSnout.attributes.position.getY(2) + 5, geomSnout.attributes.position.getZ(2) + 5);

	geomSnout.attributes.position.setXYZ(3, geomSnout.attributes.position.getX(3), geomSnout.attributes.position.getY(3) + 5, geomSnout.attributes.position.getZ(3) - 5);

	snout.castShadow = true;
	snout.receiveShadow = true;
	snout.position.set(30, 0, 0);
	this.head.add(snout);

	// Create the Nose
	var geomNose = new THREE.BoxGeometry(10, 15, 20, 1, 1, 1);
	var matNose = new THREE.MeshPhongMaterial({ color: Colors.brown, flatShading: true });
	var nose = new THREE.Mesh(geomNose, matNose);
	nose.position.set(55, 0, 0);
	this.head.add(nose);

	// Create the Ears
	var geomEar = new THREE.BoxGeometry(10, 40, 30, 1, 1, 1);
	var earL = new THREE.Mesh(geomEar, redFurMat);
	earL.position.set(-10, 40, -18);
	this.head.add(earL);
	earL.rotation.x = -Math.PI / 10;

	geomEar.attributes.position.setZ(1, geomEar.attributes.position.getZ(1) + 5);
	geomEar.attributes.position.setZ(4, geomEar.attributes.position.getZ(4) + 5);
	geomEar.attributes.position.setZ(0, geomEar.attributes.position.getZ(0) - 5);
	geomEar.attributes.position.setZ(5, geomEar.attributes.position.getZ(5) - 5);

	// Create the Ear Tips
	var geomEarTipL = new THREE.BoxGeometry(10, 10, 20, 1, 1, 1);
	var matEarTip = new THREE.MeshPhongMaterial({ color: Colors.white, flatShading: true });
	var earTipL = new THREE.Mesh(geomEarTipL, matEarTip);
	earTipL.position.set(0, 25, 0);
	earL.add(earTipL);

	var earR = earL.clone();
	earR.position.z = -earL.position.z;
	earR.rotation.x = -earL.rotation.x;
	this.head.add(earR);

	this.mesh.add(this.head);

	// Create the tail
	var geomTail = new THREE.BoxGeometry(80, 40, 40, 2, 1, 1);

	geomTail.attributes.position.setXYZ(4, geomTail.attributes.position.getX(4), geomTail.attributes.position.getY(4) - 10, geomTail.attributes.position.getZ(4) + 10);

	geomTail.attributes.position.setXYZ(5, geomTail.attributes.position.getX(5), geomTail.attributes.position.getY(5) - 10, geomTail.attributes.position.getZ(5) - 10);

	geomTail.attributes.position.setXYZ(6, geomTail.attributes.position.getX(6), geomTail.attributes.position.getY(6) + 10, geomTail.attributes.position.getZ(6) + 10);

	geomTail.attributes.position.setXYZ(7, geomTail.attributes.position.getX(7), geomTail.attributes.position.getY(7) + 10, geomTail.attributes.position.getZ(7) - 10);

	this.tail = new THREE.Mesh(geomTail, redFurMat);
	this.tail.castShadow = true;
	this.tail.receiveShadow = true;

	// Create the tail Tip
	var geomTailTip = new THREE.BoxGeometry(20, 40, 40, 1, 1, 1);
	var matTailTip = new THREE.MeshPhongMaterial({ color: Colors.white, flatShading: true });
	var tailTip = new THREE.Mesh(geomTailTip, matTailTip);
	tailTip.position.set(80, 0, 0);
	tailTip.castShadow = true;
	tailTip.receiveShadow = true;
	this.tail.add(tailTip);
	this.tail.position.set(-40, 10, 0);
	geomTail.translate(40, 0, 0);
	geomTailTip.translate(10, 0, 0);
	this.tail.rotation.z = Math.PI / 1.5;
	this.mesh.add(this.tail);

	// Create the Legs
	var geomLeg = new THREE.BoxGeometry(20, 60, 20, 1, 1, 1);
	this.legFR = new THREE.Mesh(geomLeg, redFurMat);
	this.legFR.castShadow = true;
	this.legFR.receiveShadow = true;

	// Create the feet
	var geomFeet = new THREE.BoxGeometry(20, 20, 20, 1, 1, 1);
	var matFeet = new THREE.MeshPhongMaterial({ color: Colors.white, flatShading: true });
	var feet = new THREE.Mesh(geomFeet, matFeet);
	feet.position.set(0, 0, 0);
	feet.castShadow = true;
	feet.receiveShadow = true;
	this.legFR.add(feet);
	this.legFR.position.set(70, -12, 25);
	geomLeg.translate(0, 40, 0);
	geomFeet.translate(0, 80, 0);
	this.legFR.rotation.z = 16;
	this.mesh.add(this.legFR);

	this.legFL = this.legFR.clone();
	this.legFL.position.z = -this.legFR.position.z;
	this.legFL.rotation.z = -this.legFR.rotation.z;
	this.mesh.add(this.legFL);

	this.legBR = this.legFR.clone();
	this.legBR.position.x = -this.legFR.position.x + 50;
	this.legBR.rotation.z = -this.legFR.rotation.z;
	this.mesh.add(this.legBR);

	this.legBL = this.legFL.clone();
	this.legBL.position.x = -this.legFL.position.x + 50;
	this.legBL.rotation.z = -this.legFL.rotation.z;
	this.mesh.add(this.legBL);
};

var sky;
var forest;
var land;
var orbit;
var airplane;
var sun;
var fox;

var mousePos = { x: 0, y: 0 };
var offSet = -600;

function createSky() {
	sky = new Sky();
	sky.mesh.position.y = offSet;
	scene.add(sky.mesh);
}

function createLand() {
	land = new Land();
	land.mesh.position.y = offSet;
	scene.add(land.mesh);
}

function createOrbit() {
	orbit = new Orbit();
	orbit.mesh.position.y = offSet;
	orbit.mesh.rotation.z = -Math.PI / 6;
	scene.add(orbit.mesh);
}

function createForest() {
	forest = new Forest();
	forest.mesh.position.y = offSet;
	scene.add(forest.mesh);
}

function createSun() {
	sun = new Sun();
	sun.mesh.scale.set(1, 1, 0.3);
	sun.mesh.position.set(0, -30, -850);
	scene.add(sun.mesh);
}

function createPlane() {
	airplane = new AirPlane();
	airplane.mesh.scale.set(0.35, 0.35, 0.35);
	airplane.mesh.position.set(-40, 110, -250);
	// airplane.mesh.rotation.z = Math.PI/15;
	scene.add(airplane.mesh);
}

function createFox() {
	fox = new Fox();
	fox.mesh.scale.set(0.35, 0.35, 0.35);
	fox.mesh.position.set(-40, 110, -250);
	scene.add(fox.mesh);
}

function updatePlane() {
	var targetY = normalize(mousePos.y, -0.75, 0.75, 50, 190);
	var targetX = normalize(mousePos.x, -0.75, 0.75, -100, -20);

	// Move the plane at each frame by adding a fraction of the remaining distance
	airplane.mesh.position.y += (targetY - airplane.mesh.position.y) * 0.1;

	airplane.mesh.position.x += (targetX - airplane.mesh.position.x) * 0.1;

	// Rotate the plane proportionally to the remaining distance
	airplane.mesh.rotation.z = (targetY - airplane.mesh.position.y) * 0.0128;
	airplane.mesh.rotation.x = (airplane.mesh.position.y - targetY) * 0.0064;
	airplane.mesh.rotation.y = (airplane.mesh.position.x - targetX) * 0.0064;

	airplane.propeller.rotation.x += 0.3;
}

function normalize(v, vmin, vmax, tmin, tmax) {
	var nv = Math.max(Math.min(v, vmax), vmin);
	var dv = vmax - vmin;
	var pc = (nv - vmin) / dv;
	var dt = tmax - tmin;
	var tv = tmin + pc * dt;
	return tv;
}

function loop() {
	land.mesh.rotation.z += 0.005;
	orbit.mesh.rotation.z += 0.001;
	sky.mesh.rotation.z += 0.003;
	forest.mesh.rotation.z += 0.005;
	updatePlane();

	renderer.render(scene, camera);
	requestAnimationFrame(loop);
}

function handleMouseMove(event) {
	var tx = -1 + (event.clientX / WIDTH) * 2;
	var ty = 1 - (event.clientY / HEIGHT) * 2;
	mousePos = { x: tx, y: ty };
}

function handleTouchMove(event) {
	var tx = -1 + (event.changedTouches[0].clientX / WIDTH) * 2;
	var ty = 1 - (event.changedTouches[0].clientY / HEIGHT) * 2;
	mousePos = { x: tx, y: ty };
}

export function tiny_poly_world(value) {
	canvas = value;
	createScene(canvas);
	createLights();
	createPlane();
	createOrbit();
	createSun();
	createLand();
	createForest();
	createSky();
	//createFox();

	//canvas.addEventListener('mousemove', handleMouseMove, false);
	canvas.addEventListener('touchmove', handleTouchMove, false);

	loop();
}

//window.addEventListener('load', init, false);
