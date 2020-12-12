import { DemoSharedBase } from '../utils';
import '@nativescript/canvas-babylon';
import * as BABYLON from 'babylonjs';
import * as MATERIALS from 'babylonjs-materials';
import { knownFolders } from '@nativescript/core';

export class DemoSharedCanvasBabylon extends DemoSharedBase {
	canvas: any;
	root = '~/assets/babylon';
	engine;

	loaded(args) {
		console.log('loaded', args.object);
	}

	unloaded(args) {
		console.log('unloaded');
	}

	canvasLoaded(args) {
		this.canvas = args.object;
		const gl = this.canvas.getContext('webgl2');

		const { drawingBufferWidth: width, drawingBufferHeight: height } = gl;
		var engine: BABYLON.Engine = null;
		var scene: BABYLON.Scene = null;
		var sceneToRender: BABYLON.Scene = null;
		var createDefaultEngine = function () {
			return new BABYLON.Engine(gl, true, { preserveDrawingBuffer: false, stencil: true, antialias: true });
		};
		// You have to create a function called createScene. This function must return a BABYLON.Scene object
		// You can reference the following variables: scene, canvas
		// You must at least define a camera
		engine = createDefaultEngine();
		if (!engine) throw 'engine should not be null.';
		 this.createMDN(engine)

		// sceneToRender = this.createChart(engine);
		// sceneToRender = this.createWaterScene(engine);
		//sceneToRender = this.createScene(engine);
		//sceneToRender = this.createSkullScene(engine);
		//sceneToRender = this.createLightTexture(engine);
		// sceneToRender = this.createParticleScene(engine);
		engine.runRenderLoop(function () {
			if (sceneToRender) {
				sceneToRender.render();
			}
		});

		// Resize
		window.addEventListener('resize', function () {
			engine.resize();
		});

		/*
    scene = createSkullScene(engine);
    sceneToRender = scene

    engine.runRenderLoop(function () {
    if (sceneToRender) {
        sceneToRender.render();
    }
    });


    // Resize
    window.addEventListener("resize", function () {
    engine.resize();
    });

    */
	}

	createChart(engine) {
		var scene = new BABYLON.Scene(engine);
		var light = new BABYLON.DirectionalLight('dir01', new BABYLON.Vector3(0, -0.5, 1.0), scene);
		var camera = new BABYLON.ArcRotateCamera('Camera', 0, 0, 10, BABYLON.Vector3.Zero(), scene);
		camera.setPosition(new BABYLON.Vector3(20, 70, -100));
		light.position = new BABYLON.Vector3(0, 25, -50);

		camera.attachControl(this.canvas, true);

		// Data
		var scale = 0.6;
		var operatingSystem_Series = [
			{ label: 'Macintosh', value: 12, color: new BABYLON.Color3(0, 1, 0) },
			{ label: 'Windows', value: 77, color: new BABYLON.Color3(1, 0, 0) },
			{ label: 'Linux', value: 4, color: new BABYLON.Color3(1, 0, 1) },
			{ label: 'iOS', value: 3, color: new BABYLON.Color3(1, 1, 0) },
			{ label: 'Android', value: 2, color: new BABYLON.Color3(0, 0, 1) },
			{ label: 'Win Phone', value: 1, color: new BABYLON.Color3(1, 1, 1) },
		];

		var browsers_Series = [
			{ label: 'IE', value: 32, color: new BABYLON.Color3(0, 0, 1) },
			{ label: 'Chrome', value: 28, color: new BABYLON.Color3(1, 0, 0) },
			{ label: 'Firefox', value: 16, color: new BABYLON.Color3(1, 0, 1) },
			{ label: 'Opera', value: 14, color: new BABYLON.Color3(1, 1, 0) },
			{ label: 'Safari', value: 10, color: new BABYLON.Color3(0, 1, 1) },
		];

		var playgroundSize = 100;
		// Background
		var background = BABYLON.Mesh.CreatePlane('background', playgroundSize, scene, false) as any;
		background.material = new BABYLON.StandardMaterial('background', scene);
		background.scaling.y = 0.5;
		background.position.z = playgroundSize / 2 - 0.5;
		background.position.y = playgroundSize / 4;
		background.receiveShadows = true;
		var backgroundTexture = new BABYLON.DynamicTexture('dynamic texture', 512, scene, true);
		background.material.diffuseTexture = backgroundTexture;
		background.material.specularColor = new BABYLON.Color3(0, 0, 0);
		background.material.backFaceCulling = false;

		backgroundTexture.drawText('Eternalcoding', null, 80, 'bold 70px Segoe UI', 'white', '#555555');
		backgroundTexture.drawText('- browsers statistics -', null, 250, '35px Segoe UI', 'white', null);

		// Ground
		var ground = BABYLON.Mesh.CreateGround('ground', playgroundSize, playgroundSize, 1, scene, false);
		var groundMaterial = new BABYLON.StandardMaterial('ground', scene);
		groundMaterial.diffuseColor = new BABYLON.Color3(0.5, 0.5, 0.5);
		groundMaterial.specularColor = new BABYLON.Color3(0, 0, 0);
		ground.material = groundMaterial;
		ground.receiveShadows = true;
		ground.position.y = -0.1;

		var shadowGenerator = new BABYLON.ShadowGenerator(1024, light);
		shadowGenerator.usePoissonSampling = true;

		var createSeries = function (series) {
			var margin = 2;
			var offset = playgroundSize / series.length - margin;
			var x = -playgroundSize / 2 + offset / 2;

			for (var index = 0; index < series.length; index++) {
				var data = series[index];

				var bar = BABYLON.Mesh.CreateBox(data.label, 1.0, scene, false) as any;
				bar.scaling = new BABYLON.Vector3(offset / 2.0, 0, offset / 2.0);
				bar.position.x = x;
				bar.position.y = 0;

				// Animate a bit
				var animation = new BABYLON.Animation('anim', 'scaling', 30, BABYLON.Animation.ANIMATIONTYPE_VECTOR3);
				animation.setKeys([
					{ frame: 0, value: new BABYLON.Vector3(offset / 2.0, 0, offset / 2.0) },
					{ frame: 100, value: new BABYLON.Vector3(offset / 2.0, data.value * scale, offset / 2.0) },
				]);
				bar.animations.push(animation);

				animation = new BABYLON.Animation('anim2', 'position.y', 30, BABYLON.Animation.ANIMATIONTYPE_FLOAT);
				animation.setKeys([
					{ frame: 0, value: 0 },
					{ frame: 100, value: (data.value * scale) / 2 },
				]);
				bar.animations.push(animation);
				scene.beginAnimation(bar, 0, 100, false, 2.0);

				// Material
				bar.material = new BABYLON.StandardMaterial(data.label + 'mat', scene);
				bar.material.diffuseColor = data.color;
				bar.material.emissiveColor = data.color.scale(0.3);
				bar.material.specularColor = new BABYLON.Color3(0, 0, 0);

				// Shadows
				shadowGenerator.getShadowMap().renderList.push(bar);

				// Legend
				var barLegend = BABYLON.Mesh.CreateGround(data.label + 'Legend', playgroundSize / 2, offset * 2, 1, scene, false) as any;
				barLegend.position.x = x;
				barLegend.position.z = -playgroundSize / 4;
				barLegend.rotation.y = Math.PI / 2;

				barLegend.material = new BABYLON.StandardMaterial(data.label + 'LegendMat', scene);
				var barLegendTexture = new BABYLON.DynamicTexture('dynamic texture', 512, scene, true);
				barLegendTexture.hasAlpha = true;
				barLegend.material.diffuseTexture = barLegendTexture;
				barLegend.material.emissiveColor = new BABYLON.Color3(0.4, 0.4, 0.4);

				var size = barLegendTexture.getSize();
				barLegendTexture.drawText(data.label + ' (' + data.value + '%)', 80, size.height / 2 + 30, 'bold 50px Segoe UI', 'white', 'transparent');

				// Going next
				x += offset + margin;
			}
		};

		createSeries(browsers_Series);

		// Limit camera
		camera.lowerAlphaLimit = Math.PI;
		camera.upperAlphaLimit = 2 * Math.PI;
		camera.lowerBetaLimit = 0.1;
		camera.upperBetaLimit = (Math.PI / 2) * 0.99;
		camera.lowerRadiusLimit = 5;
		camera.upperRadiusLimit = 150;

		return scene;
	}

	createMDN(engine: BABYLON.Engine) {
		var scene = new BABYLON.Scene(engine);
		scene.clearColor = new BABYLON.Color4(0.8, 0.8, 0.8, 1);
		var camera = new BABYLON.FreeCamera('camera', new BABYLON.Vector3(0, 0, -30), scene);
		var light = new BABYLON.PointLight('light', new BABYLON.Vector3(10, 10, 0), scene);

		var box = BABYLON.Mesh.CreateBox('box', 2, scene);
		box.rotation.x = -0.2;
		box.rotation.y = -0.4;

		var boxMaterial = new BABYLON.StandardMaterial('material', scene);
		boxMaterial.emissiveColor = new BABYLON.Color3(0, 0.58, 0.86);
		box.material = boxMaterial;

		var torus = BABYLON.Mesh.CreateTorus('torus', 2, 0.5, 15, scene);
		torus.position.x = -5;
		torus.rotation.x = 1.5;

		var torusMaterial = new BABYLON.StandardMaterial('material', scene);
		torusMaterial.emissiveColor = new BABYLON.Color3(0.4, 0.4, 0.4);
		torus.material = torusMaterial;

		var cylinder = BABYLON.Mesh.CreateCylinder('cylinder', 2, 2, 2, 12, 1, scene);
		cylinder.position.x = 5;
		cylinder.rotation.x = -0.2;

		var cylinderMaterial = new BABYLON.StandardMaterial('material', scene);
		cylinderMaterial.emissiveColor = new BABYLON.Color3(1, 0.58, 0);
		cylinder.material = cylinderMaterial;

		var t = 0;
		var renderLoop = function () {
			scene.render();
			t -= 0.01;
			box.rotation.y = t * 2;
			torus.scaling.z = Math.abs(Math.sin(t * 2)) + 0.5;
			cylinder.position.y = Math.sin(t * 3);
		};
		engine.runRenderLoop(renderLoop);
	}

	createParticleScene(engine) {
		var scene = new BABYLON.Scene(engine);

		// Setup environment
		var camera = new BABYLON.ArcRotateCamera('ArcRotateCamera', 1, 0.8, 20, new BABYLON.Vector3(0, 0, 0), scene);
		camera.attachControl(this.canvas, true);
		camera.wheelPrecision = 100;

		var fountain = BABYLON.Mesh.CreateBox('fountain', 0.1, scene);
		fountain.visibility = 0.1;

		// Create a particle system
		var particleSystem;
		var useGPUVersion = true;

		var createNewSystem = () => {
			if (particleSystem) {
				particleSystem.dispose();
			}

			if (useGPUVersion && BABYLON.GPUParticleSystem.IsSupported) {
				particleSystem = new BABYLON.GPUParticleSystem('particles', { capacity: 1000000 }, scene);
				particleSystem.activeParticleCount = 2000;
			} else {
				particleSystem = new BABYLON.ParticleSystem('particles', 10000, scene);
			}

			particleSystem.emitRate = 1000;
			particleSystem.particleEmitterType = new BABYLON.SphereParticleEmitter(1);
			particleSystem.particleTexture = new BABYLON.Texture(this.root + '/textures/flare.png', scene);
			particleSystem.maxLifeTime = 10;
			particleSystem.minSize = 0.01;
			particleSystem.maxSize = 0.1;
			particleSystem.emitter = fountain;

			particleSystem.start();
		};

		createNewSystem();

		var alpha = 0;
		var moveEmitter = false;
		var rotateEmitter = false;

		scene.registerBeforeRender(function () {
			if (moveEmitter) {
				fountain.position.x = 5 * Math.cos(alpha);
				fountain.position.z = 5 * Math.sin(alpha);
			}

			if (rotateEmitter) {
				fountain.rotation.x += 0.01;
			}

			alpha += 0.01;
		});
		/*
    // GUI
    var advancedTexture;

    var buildUI = function() {
        if (advancedTexture) {
            advancedTexture.dispose();
        }
        advancedTexture = (BABYLON as any).GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");

        var rightPanel = new (BABYLON as any).GUI.StackPanel();
        rightPanel.width = "300px";
        rightPanel.isVertical = true;
        rightPanel.paddingRight = "20px";
        rightPanel.horizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_RIGHT;
        rightPanel.verticalAlignment = (BABYLON as any).GUI.Control.VERTICAL_ALIGNMENT_CENTER;
        rightPanel.fontSize = 16;
        advancedTexture.addControl(rightPanel);

        var bottomPanel = new (BABYLON as any).GUI.StackPanel();
        bottomPanel.height = "100px";
        bottomPanel.paddingBottom = "20px";
        bottomPanel.isVertical = true;
        bottomPanel.horizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_STRETCH;
        bottomPanel.verticalAlignment = (BABYLON as any).GUI.Control.VERTICAL_ALIGNMENT_BOTTOM;
        bottomPanel.fontSize = 16;
        advancedTexture.addControl(bottomPanel);

        var upPanel = new (BABYLON as any).GUI.StackPanel();
        upPanel.height = "100px";
        upPanel.isVertical = true;
        upPanel.horizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_STRETCH;
        upPanel.verticalAlignment = (BABYLON as any).GUI.Control.VERTICAL_ALIGNMENT_TOP;
        upPanel.fontSize = 16;
        advancedTexture.addControl(upPanel);

        var leftPanel = new (BABYLON as any).GUI.StackPanel();
        leftPanel.width = "300px";
        leftPanel.isVertical = true;
        leftPanel.paddingLeft = "20px";
        leftPanel.horizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_LEFT;
        leftPanel.verticalAlignment = (BABYLON as any).GUI.Control.VERTICAL_ALIGNMENT_CENTER;
        leftPanel.fontSize = 16;
        advancedTexture.addControl(leftPanel);

        var getPropertyPath = function(property) {
            var dotIndex = property.indexOf(".");

            if (dotIndex === -1) {
                return particleSystem[property];
            }

            var splits = property.split(".");

            return particleSystem[splits[0]][splits[1]];
        }

        var setPropertyPath = function(property, value) {
            var dotIndex = property.indexOf(".");

            if (dotIndex === -1) {
                particleSystem[property] = value;
                return;
            }

            var splits = property.split(".");

            particleSystem[splits[0]][splits[1]] = value;
        }

        var addColorPicker = function(text, property, left, panel) {
            var header = new (BABYLON as any).GUI.TextBlock();
            header.text = text;
            header.height = "30px";
            header.color = "white";
            header.outlineWidth = "4px";
            header.outlineColor = "black";
            header.textHorizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_LEFT;
            panel.addControl(header);

            if (left) {
                header.left = left;
            }

            var colorPicker = new (BABYLON as any).GUI.ColorPicker();
            colorPicker.onSync = function() {
                colorPicker.value = particleSystem[property];
            }
            colorPicker.onSync();
            colorPicker.size = "100px";
            colorPicker.horizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_LEFT;
            colorPicker.onValueChangedObservable.add(function(value) {
                particleSystem[property] = value;
            });

            if (left) {
                colorPicker.left = left;
            }

            panel.addControl(colorPicker);

            return colorPicker;
        }

        var addCheckbox = function(text, initial, onCheck, panel) {
            var checkbox = new (BABYLON as any).GUI.Checkbox();
            checkbox.width = "20px";
            checkbox.height = "20px";
            checkbox.color = "green";
            checkbox.isChecked = initial;
            checkbox.onIsCheckedChangedObservable.add(function(value) {
                onCheck(value);
            });

            var header = (BABYLON as any).GUI.Control.AddHeader(checkbox, text, "180px", { isHorizontal: true, controlFirst: true});
            header.height = "30px";
            header.color = "white";
            header.outlineWidth = "4px";
            header.outlineColor = "black";
            header.horizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_LEFT;

            panel.addControl(header);
        }

        var addSlider = function(text, property, min, max, left, panel, top, fixedPoint) {
            var topPanel = new (BABYLON as any).GUI.StackPanel();
            topPanel.height = "30px";
            topPanel.isVertical = false;
            panel.addControl(topPanel);

            var header = new (BABYLON as any).GUI.TextBlock();
            header.text = text;
            header.width = "150px";
            header.color = "white";
            header.outlineWidth = "4px";
            header.outlineColor = "black";
            header.textHorizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_LEFT;
            topPanel.addControl(header);
            if (left) {
                header.left = left;
            }

            var valueHeader = new (BABYLON as any).GUI.TextBlock();
            valueHeader.text = getPropertyPath(property).toFixed(fixedPoint);
            valueHeader.width = "100px";
            valueHeader.color = "white";
            valueHeader.outlineWidth = "4px";
            valueHeader.outlineColor = "black";
            valueHeader.textHorizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_RIGHT;
            topPanel.addControl(valueHeader);

            var slider = new (BABYLON as any).GUI.Slider();
            slider.minimum = min;
            slider.maximum = max;
            slider.height = "20px";
            slider.color = "green";
            slider.background = "white";
            slider.onSync = function() {
                slider.value = getPropertyPath(property);
            }
            slider.onSync();
            slider.onValueChangedObservable.add(function(value) {
                valueHeader.text = value.toFixed(fixedPoint);
                setPropertyPath(property, value);
            });

            if (left) {
                slider.paddingLeft = left;
            }

            panel.addControl(slider);

            return slider;
        }

        var addSeparator = function(panel) {
            var rectangle = new (BABYLON as any).GUI.Rectangle();
            rectangle.height = "15px";
            rectangle.thickness = 0;
            panel.addControl(rectangle);
        }

        var addHeader = function(text, panel) {
            var header = new (BABYLON as any).GUI.TextBlock();
            header.text = text;
            header.height = "30px";
            header.color = "white";
            header.outlineWidth = "4px";
            header.outlineColor = "black";
            header.textHorizontalAlignment = (BABYLON as any).GUI.Control.HORIZONTAL_ALIGNMENT_CENTER;

            panel.addControl(header);
        }

        if (particleSystem.activeParticleCount) {
            addSlider("particles count:", "activeParticleCount", 0, particleSystem.getCapacity(), "20px", bottomPanel, 0, 0);
        } else {
            addHeader("particles count: " + particleSystem.getCapacity(), bottomPanel);
        }

        addSlider("updateSpeed:", "updateSpeed", 0, 0.1, "20px", rightPanel, 0, 2);
        addSeparator(rightPanel);
        addSlider("gravity:", "gravity.y", -5, 5, "20px", rightPanel, 0, 2);
        addSeparator(rightPanel);
        addSlider("minSize:", "minSize", 0.01, 1, "20px", rightPanel, 0, 2);
        addSlider("maxSize:", "maxSize", 0.01, 1, "20px", rightPanel, 0, 2);
        addSeparator(rightPanel);
        addSlider("minLifeTime:", "minLifeTime", 0.001, 10, "20px", rightPanel, 0, 2);
        addSlider("maxLifeTime:", "maxLifeTime", 0.001, 10, "20px", rightPanel, 0, 2);
        addSeparator(rightPanel);
        addSlider("minEmitPower:", "minEmitPower", 0, 10, "20px", rightPanel, 0, 2);
        addSlider("maxEmitPower:", "maxEmitPower", 0, 10, "20px", rightPanel, 0, 2);

        if (BABYLON.GPUParticleSystem.IsSupported) {
            addCheckbox("Use GPU particles", useGPUVersion, function(value){
                useGPUVersion = value;
                createNewSystem();
                buildUI();
            }, leftPanel);
        } else {
            addHeader("Browser does not support WebGL2", leftPanel);
            addHeader("Using CPU particles instead...", leftPanel);
        }
        addCheckbox("Rotate emitter", false, function(value){
            particleSystem.reset();
            rotateEmitter = value
        }, leftPanel);
        addCheckbox("Move emitter", false, function(value){
            particleSystem.reset();
            moveEmitter = value
        }, leftPanel);
        addSeparator(leftPanel);
        addColorPicker("color1:", "color1", "20px", leftPanel);
        addColorPicker("color2:", "color2", "20px", leftPanel);
        addColorPicker("colorDead:", "colorDead", "20px", leftPanel);
    }

    buildUI();
    */

		return scene;
	}

	createLightTexture(engine) {
		var scene = new BABYLON.Scene(engine);

		// Setup environment
		var camera = new BABYLON.ArcRotateCamera('Camera', 0, 0.8, 90, new BABYLON.Vector3(0, 0, 0), scene);
		camera.lowerBetaLimit = 0.1;
		camera.upperBetaLimit = (Math.PI / 2) * 0.9;
		camera.lowerRadiusLimit = 30;
		camera.upperRadiusLimit = 150;
		camera.attachControl(this.canvas, true);

		var light = new BABYLON.HemisphericLight('dir01', new BABYLON.Vector3(0, 1, 0), scene);
		light.intensity = 0.1;

		// spot
		var spotLight = new BABYLON.SpotLight('spot02', new BABYLON.Vector3(30, 40, 30), new BABYLON.Vector3(-1, -2, -1), 1.1, 16, scene);
		spotLight.projectionTexture = new BABYLON.Texture(this.root + '/textures/co.png', scene);
		spotLight.setDirectionToTarget(BABYLON.Vector3.Zero());
		spotLight.intensity = 1.5;

		// Ground
		var ground = BABYLON.Mesh.CreateGroundFromHeightMap('ground', this.root + '/textures/heightMap.png', 100, 100, 100, 0, 10, scene, false);
		var groundMaterial = new BABYLON.StandardMaterial('ground', scene);
		groundMaterial.diffuseTexture = new BABYLON.Texture(this.root + '/textures/ground.jpg', scene);
		// @ts-ignore
		groundMaterial.diffuseTexture.uScale = 6;
		// @ts-ignore
		groundMaterial.diffuseTexture.vScale = 6;
		groundMaterial.specularColor = new BABYLON.Color3(0, 0, 0);
		ground.position.y = -2.05;
		ground.material = groundMaterial;

		// Animations
		var alpha = 0;
		scene.registerBeforeRender(function () {
			spotLight.position = new BABYLON.Vector3(Math.cos(alpha) * 60, 40, Math.sin(alpha) * 60);
			spotLight.setDirectionToTarget(BABYLON.Vector3.Zero());
			alpha += 0.01;
		});

		return scene;
	}

	createScene(engine) {
		var scene = new BABYLON.Scene(engine);

		var light = new BABYLON.PointLight('Omni', new BABYLON.Vector3(0, 100, 100), scene);
		var camera = new BABYLON.ArcRotateCamera('Camera', 0, 0.8, 100, BABYLON.Vector3.Zero(), scene);
		camera.attachControl(this.canvas, true);

		//Boxes
		var box1 = BABYLON.Mesh.CreateBox('Box1', 10.0, scene);
		box1.position.x = -20;
		var box2 = BABYLON.Mesh.CreateBox('Box2', 10.0, scene);

		var materialBox = new BABYLON.StandardMaterial('texture1', scene);
		materialBox.diffuseColor = new BABYLON.Color3(0, 1, 0); //Green
		var materialBox2 = new BABYLON.StandardMaterial('texture2', scene);

		//Applying materials
		box1.material = materialBox;
		box2.material = materialBox2;

		//Positioning box
		box2.position.x = 20;

		// Creation of a basic animation with box 1
		//----------------------------------------

		//Create a scaling animation at 30 FPS
		var animationBox = new BABYLON.Animation('tutoAnimation', 'scaling.x', 30, BABYLON.Animation.ANIMATIONTYPE_FLOAT, BABYLON.Animation.ANIMATIONLOOPMODE_CYCLE);
		//Here we have chosen a loop mode, but you can change to :
		//  Use previous values and increment it (BABYLON.Animation.ANIMATIONLOOPMODE_RELATIVE)
		//  Restart from initial value (BABYLON.Animation.ANIMATIONLOOPMODE_CYCLE)
		//  Keep the final value (BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT)

		// Animation keys
		var keys = [];
		//At the animation key 0, the value of scaling is "1"
		keys.push({
			frame: 0,
			value: 1,
		});

		//At the animation key 20, the value of scaling is "0.2"
		keys.push({
			frame: 20,
			value: 0.2,
		});

		//At the animation key 100, the value of scaling is "1"
		keys.push({
			frame: 100,
			value: 1,
		});

		//Adding keys to the animation object
		animationBox.setKeys(keys);

		//Then add the animation object to box1
		box1.animations.push(animationBox);

		//Finally, launch animations on box1, from key 0 to key 100 with loop activated
		scene.beginAnimation(box1, 0, 100, true);

		// Creation of a manual animation with box 2
		//------------------------------------------
		setInterval(function () {
			//The color is defined at run time with random()
			materialBox2.diffuseColor = new BABYLON.Color3(Math.random(), Math.random(), Math.random());
		}, 1000);

		return scene;
	}

	createSkullScene(engine) {
		var scene = new BABYLON.Scene(engine);

		//Adding a light
		var light = new BABYLON.PointLight('Omni', new BABYLON.Vector3(20, 20, 100), scene);

		//Adding an Arc Rotate Camera
		var camera = new BABYLON.ArcRotateCamera('Camera', 0, 0.8, 100, BABYLON.Vector3.Zero(), scene);
		// camera.attachControl(canvas, false);

		// The first parameter can be used to specify which mesh to import. Here we import all meshes

		BABYLON.SceneLoader.ImportMesh(
			null,
			this.root + '/scenes/',
			'skull.babylon',
			scene,
			function (newMeshes) {
				console.log('newMeshes', newMeshes[0]);
				camera.target = newMeshes[0] as any;
			},
			null,
			(e) => {
				console.log('e', e);
			}
		);

		// Move the light with the camera
		scene.registerBeforeRender(function () {
			light.position = camera.position;
		});

		return scene;
	}

	createWaterScene(engine) {
		var scene = new BABYLON.Scene(engine);

		// Camera
		var camera = new BABYLON.ArcRotateCamera('Camera', (3 * Math.PI) / 2, Math.PI / 4, 100, BABYLON.Vector3.Zero(), scene);
		camera.attachControl(this.canvas, true);

		// Light
		var light = new BABYLON.HemisphericLight('light1', new BABYLON.Vector3(0, 1, 0), scene);

		// Skybox
		var skybox = BABYLON.Mesh.CreateBox('skyBox', 5000.0, scene);
		var skyboxMaterial = new BABYLON.StandardMaterial('skyBox', scene);
		skyboxMaterial.backFaceCulling = false;
		skyboxMaterial.reflectionTexture = new BABYLON.CubeTexture(this.root + '/images/skybox/TropicalSunnyDay', scene);
		skyboxMaterial.reflectionTexture.coordinatesMode = BABYLON.Texture.SKYBOX_MODE;
		skyboxMaterial.diffuseColor = new BABYLON.Color3(0, 0, 0);
		skyboxMaterial.specularColor = new BABYLON.Color3(0, 0, 0);
		skyboxMaterial.disableLighting = true;
		skybox.material = skyboxMaterial;
		// Water material

		var waterMaterial = new MATERIALS.WaterMaterial('waterMaterial', scene, new BABYLON.Vector2(512, 512));
		waterMaterial.bumpTexture = new BABYLON.Texture(this.root + '/images/waterbump.png', scene);
		waterMaterial.windForce = -10;
		waterMaterial.waveHeight = 0.5;
		waterMaterial.bumpHeight = 0.1;
		waterMaterial.waveLength = 0.1;
		waterMaterial.waveSpeed = 50.0;
		waterMaterial.colorBlendFactor = 0;
		waterMaterial.windDirection = new BABYLON.Vector2(1, 1);
		waterMaterial.colorBlendFactor = 0;

		// Ground
		var groundTexture = new BABYLON.Texture(this.root + '/images/sand.jpg', scene);
		groundTexture.vScale = groundTexture.uScale = 4.0;

		var groundMaterial = new BABYLON.StandardMaterial('groundMaterial', scene);
		groundMaterial.diffuseTexture = groundTexture;

		var ground = BABYLON.Mesh.CreateGround('ground', 512, 512, 32, scene, false);
		ground.position.y = -1;
		ground.material = groundMaterial;

		// Water mesh
		var waterMesh = BABYLON.Mesh.CreateGround('waterMesh', 512, 512, 32, scene, false);
		waterMesh.material = waterMaterial;

		// Sphere
		var sphereMaterial = new BABYLON.StandardMaterial('sphereMaterial', scene);
		sphereMaterial.diffuseTexture = new BABYLON.Texture(this.root + '/images/wood.jpg', scene);

		var sphere = BABYLON.Mesh.CreateSphere('sphere', 32, 24, scene);
		sphere.position.y = 20;
		sphere.material = sphereMaterial;

		// Configure water material
		waterMaterial.addToRenderList(ground);
		waterMaterial.addToRenderList(skybox);
		waterMaterial.addToRenderList(sphere);

		return scene;
	}

	/*
  var delayCreateScene = function (engine) {

      // Model by Mixamo

      engine.enableOfflineSupport = false;

      // This is really important to tell Babylon.js to use decomposeLerp and matrix interpolation
      BABYLON.Animation.AllowMatricesInterpolation = true;

      var scene = new BABYLON.Scene(engine);

      var camera = new BABYLON.ArcRotateCamera("camera1", Math.PI / 2, Math.PI / 4, 3, new BABYLON.Vector3(0, 1, 0), scene);
      camera.attachControl(canvas, true);

      camera.lowerRadiusLimit = 2;
      camera.upperRadiusLimit = 10;
      camera.wheelDeltaPercentage = 0.01;

    var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(0, 1, 0), scene);
    light.intensity = 0.6;
    light.specular = BABYLON.Color3.Black();

      var light2 = new BABYLON.DirectionalLight("dir01", new BABYLON.Vector3(0, -0.5, -1.0), scene);
      light2.position = new BABYLON.Vector3(0, 5, 5);

      // Shadows
      var shadowGenerator = new BABYLON.ShadowGenerator(1024, light2);
      shadowGenerator.useBlurExponentialShadowMap = true;
      shadowGenerator.blurKernel = 32;

      engine.displayLoadingUI();

    BABYLON.SceneLoader.ImportMesh("", "./scenes/", "dummy3.babylon", scene, function (newMeshes, particleSystems, skeletons) {
          var skeleton = skeletons[0];

          shadowGenerator.addShadowCaster(scene.meshes[0], true);
          for (var index = 0; index < newMeshes.length; index++) {
              newMeshes[index].receiveShadows = false;;
          }

          var helper = scene.createDefaultEnvironment({
              enableGroundShadow: true
          });
          helper.setMainColor(BABYLON.Color3.Gray());
          helper.ground.position.y += 0.01;

          // ROBOT
          skeleton.animationPropertiesOverride = new BABYLON.AnimationPropertiesOverride();
          skeleton.animationPropertiesOverride.enableBlending = true;
          skeleton.animationPropertiesOverride.blendingSpeed = 0.05;
          skeleton.animationPropertiesOverride.loopMode = 1;

          var idleRange = skeleton.getAnimationRange("YBot_Idle");
          var walkRange = skeleton.getAnimationRange("YBot_Walk");
          var runRange = skeleton.getAnimationRange("YBot_Run");
          var leftRange = skeleton.getAnimationRange("YBot_LeftStrafeWalk");
          var rightRange = skeleton.getAnimationRange("YBot_RightStrafeWalk");

          // IDLE
          if (idleRange) scene.beginAnimation(skeleton, idleRange.from, idleRange.to, true);

          // UI
          var advancedTexture = BABYLON.GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");
          var UiPanel = new BABYLON.GUI.StackPanel();
          UiPanel.width = "220px";
          UiPanel.fontSize = "14px";
          UiPanel.horizontalAlignment = BABYLON.GUI.Control.HORIZONTAL_ALIGNMENT_RIGHT;
          UiPanel.verticalAlignment = BABYLON.GUI.Control.VERTICAL_ALIGNMENT_CENTER;
          advancedTexture.addControl(UiPanel);
          // ..
          var button = BABYLON.GUI.Button.CreateSimpleButton("but1", "Play Idle");
          button.paddingTop = "10px";
          button.width = "100px";
          button.height = "50px";
          button.color = "white";
          button.background = "green";
          button.onPointerDownObservable.add(()=> {
              if (idleRange) scene.beginAnimation(skeleton, idleRange.from, idleRange.to, true);
          });
          UiPanel.addControl(button);
          // ..
          var button1 = BABYLON.GUI.Button.CreateSimpleButton("but2", "Play Walk");
          button1.paddingTop = "10px";
          button1.width = "100px";
          button1.height = "50px";
          button1.color = "white";
          button1.background = "green";
          button1.onPointerDownObservable.add(()=> {
              if (walkRange) scene.beginAnimation(skeleton, walkRange.from, walkRange.to, true);
          });
          UiPanel.addControl(button1);
          // ..
          var button1 = BABYLON.GUI.Button.CreateSimpleButton("but3", "Play Run");
          button1.paddingTop = "10px";
          button1.width = "100px";
          button1.height = "50px";
          button1.color = "white";
          button1.background = "green";
          button1.onPointerDownObservable.add(()=> {
              if (runRange) scene.beginAnimation(skeleton, runRange.from, runRange.to, true);
          });
          UiPanel.addControl(button1);
          // ..
          var button1 = BABYLON.GUI.Button.CreateSimpleButton("but4", "Play Left");
          button1.paddingTop = "10px";
          button1.width = "100px";
          button1.height = "50px";
          button1.color = "white";
          button1.background = "green";
          button1.onPointerDownObservable.add(()=> {
              if (leftRange) scene.beginAnimation(skeleton, leftRange.from, leftRange.to, true);
          });
          UiPanel.addControl(button1);
          // ..
          var button1 = BABYLON.GUI.Button.CreateSimpleButton("but5", "Play Right");
          button1.paddingTop = "10px";
          button1.width = "100px";
          button1.height = "50px";
          button1.color = "white";
          button1.background = "green";
          button1.onPointerDownObservable.add(()=> {
              if (rightRange) scene.beginAnimation(skeleton, rightRange.from, rightRange.to, true);
          });
          UiPanel.addControl(button1);
          // ..
          var button1 = BABYLON.GUI.Button.CreateSimpleButton("but6", "Play Blend");
          button1.paddingTop = "10px";
          button1.width = "100px";
          button1.height = "50px";
          button1.color = "white";
          button1.background = "green";
          button1.onPointerDownObservable.add(()=> {
              if (walkRange && leftRange) {
                  scene.stopAnimation(skeleton);
                  var walkAnim = scene.beginWeightedAnimation(skeleton, walkRange.from, walkRange.to, 0.5, true);
                  var leftAnim = scene.beginWeightedAnimation(skeleton, leftRange.from, leftRange.to, 0.5, true);

                  // Note: Sync Speed Ratio With Master Walk Animation
                  walkAnim.syncWith(null);
                  leftAnim.syncWith(walkAnim);
              }
          });
          UiPanel.addControl(button1);

          engine.hideLoadingUI();
      });

      return scene;
  };

  */
}
