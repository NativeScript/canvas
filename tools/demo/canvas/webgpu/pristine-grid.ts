import { Canvas, GPUBufferUsage, GPUCanvasContext, GPUTextureUsage } from '@nativescript/canvas';

import { vec3, mat4 } from 'gl-matrix/esm';

const FRAME_BUFFER_SIZE = Float32Array.BYTES_PER_ELEMENT * 36;

export async function run(canvas: Canvas) {
	const devicePixelRatio = window.devicePixelRatio;
	canvas.width = canvas.clientWidth * devicePixelRatio;
	canvas.height = canvas.clientHeight * devicePixelRatio;

	// const instancedVertWGSL = File.fromPath(path + '/webgpu/shaders/instanced.vert.wgsl').readTextSync();
	// const vertexPositionColorWGSL = File.fromPath(path + '/webgpu/shaders/vertexPositionColor.frag.wgsl').readTextSync();

	// const adapter = await navigator.gpu.requestAdapter();
	// const device: GPUDevice = (await adapter!.requestDevice()) as never;

	// const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	// const context = canvas.getContext('webgpu') as never as GPUCanvasContext;

	// context.configure({
	// 	device,
	// 	format: presentationFormat,
	// });

	const demo = new TinyWebGpuDemo(canvas);
}

export class TinyWebGpuDemo {
	#frameArrayBuffer = new ArrayBuffer(FRAME_BUFFER_SIZE);
	#projectionMatrix = new Float32Array(this.#frameArrayBuffer, 0, 16);
	#viewMatrix = new Float32Array(this.#frameArrayBuffer, 16 * Float32Array.BYTES_PER_ELEMENT, 16);
	#cameraPosition = new Float32Array(this.#frameArrayBuffer, 32 * Float32Array.BYTES_PER_ELEMENT, 3);
	#timeArray = new Float32Array(this.#frameArrayBuffer, 35 * Float32Array.BYTES_PER_ELEMENT, 1);

	static CAMERA_UNIFORM_STRUCT = `
      struct CameraUniforms {
        projection: mat4x4f,
        view: mat4x4f,
        position: vec3f,
        time: f32,
      }
    `;

	#frameMs = new Array(20);
	#frameMsIndex = 0;

	// Configurable by extending classes
	colorFormat = navigator.gpu?.getPreferredCanvasFormat?.() || 'bgra8unorm';
	depthFormat = 'depth24plus';
	sampleCount = 4;
	clearColor = { r: 0, g: 0, b: 0, a: 1.0 };
	fov = Math.PI * 0.5;
	zNear = 0.01;
	zFar = 128;
	canvas: Canvas;
	context: GPUCanvasContext;
	camera: OrbitCamera;
	// resizeObserver: ResizeObserverHelper;
	device: any;
	frameBindGroupLayout: any;
	frameBindGroup: any;
	statsFolder: any;
	pane: any;
	msaaColorTexture: any;
	depthTexture: any;
	colorAttachment: {
		// Appropriate target will be populated in onFrame
		view: any;
		resolveTarget: any;
		clearValue: { r: number; g: number; b: number; a: number };
		loadOp: string;
		storeOp: string;
	};
	renderPassDescriptor: { colorAttachments: any[]; depthStencilAttachment: { view: any; depthClearValue: number; depthLoadOp: string; depthStoreOp: string } };
	constructor(canvas) {
		this.canvas = canvas;
		this.context = this.canvas.getContext('webgpu');

		this.camera = new OrbitCamera(this.canvas);

		/*
		this.resizeObserver = new ResizeObserverHelper(this.canvas, (width, height) => {
			if (width == 0 || height == 0) {
				return;
			}

			this.canvas.width = width;
			this.canvas.height = height;

			this.updateProjection();

			if (this.device) {
				const size = { width, height };
				this.#allocateRenderTargets(size);
				this.onResize(this.device, size);
			}
		});

        */

		const frameCallback = (t) => {
			requestAnimationFrame(frameCallback);

			const texture = this.context.getCurrentTexture();

			const frameStart = performance.now();

			// Update the frame uniforms
			this.#viewMatrix.set(this.camera.viewMatrix);
			this.#cameraPosition.set(this.camera.position);
			this.#timeArray[0] = t;

			this.device.queue.writeBuffer(this.frameUniformBuffer, 0, this.#frameArrayBuffer);

			this.context.presentSurface(texture);

			this.onFrame(this.device, this.context, t);

			this.#frameMs[this.#frameMsIndex++ % this.#frameMs.length] = performance.now() - frameStart;
		};

		this.#initWebGPU()
			.then(() => {
				// Make sure the resize callback has a chance to fire at least once now that the device is
				// initialized.
				//this.resizeObserver.callback(this.canvas.width, this.canvas.height);
				// Start the render loop.

				if (canvas.width == 0 || canvas.height == 0) {
					return;
				}

				this.canvas.width = canvas.width;
				this.canvas.height = canvas.height;

				this.updateProjection();

				if (this.device) {
					const size = { width: canvas.width, heigh: canvas.height };
					this.#allocateRenderTargets(size);
					this.onResize(this.device, size);
				}

				requestAnimationFrame(frameCallback);
			})
			.catch((error) => {
				console.log(error);
				// If something goes wrong during initialization, put up a really simple error message.
				//	this.setError(error, 'initializing WebGPU');
				throw error;
			});
	}
	frameUniformBuffer(frameUniformBuffer: any, arg1: number, arg2: ArrayBuffer) {
		throw new Error('Method not implemented.');
	}

	setError(error, contextString) {
		let prevError = document.querySelector('.error');
		while (prevError) {
			this.canvas.parentElement.removeChild(document.querySelector('.error'));
			prevError = document.querySelector('.error');
		}

		if (error) {
			const errorElement = document.createElement('p');
			errorElement.classList.add('error');
			errorElement.innerHTML = `
          <p style='font-weight: bold'>An error occured${contextString ? ' while ' + contextString : ''}:</p>
          <pre>${error?.message ? error.message : error}</pre>`;
			this.canvas.parentElement.appendChild(errorElement);
		}
	}

	updateProjection() {
		const aspect = (this.canvas.width as number) / (this.canvas.height as number);
		// Using mat4.perspectiveZO instead of mat4.perpective because WebGPU's
		// normalized device coordinates Z range is [0, 1], instead of WebGL's [-1, 1]
		mat4.perspectiveZO(this.#projectionMatrix, this.fov, aspect, this.zNear, this.zFar);
	}

	get frameMs() {
		let avg = 0;
		for (const value of this.#frameMs) {
			if (value === undefined) {
				return 0;
			} // Don't have enough sampled yet
			avg += value;
		}
		return avg / this.#frameMs.length;
	}

	async #initWebGPU() {
		const adapter = await navigator.gpu.requestAdapter();

		const requiredFeatures = [];
		const featureList = adapter.features;
		if (featureList.has('texture-compression-bc')) {
			requiredFeatures.push('texture-compression-bc');
		}
		if (featureList.has('texture-compression-etc2')) {
			requiredFeatures.push('texture-compression-etc2');
		}

		this.device = await adapter.requestDevice({
			requiredFeatures,
		});

		this.context.configure({
			device: this.device,
			format: this.colorFormat,
			alphaMode: 'opaque',
			viewFormats: [`${this.colorFormat}-srgb` as never],
		});

		this.frameUniformBuffer = this.device.createBuffer({
			size: FRAME_BUFFER_SIZE,
			usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
		});

		this.frameBindGroupLayout = this.device.createBindGroupLayout({
			label: `Frame BindGroupLayout`,
			entries: [
				{
					binding: 0, // Camera/Frame uniforms
					visibility: global.GPUShaderStage.VERTEX | global.GPUShaderStage.FRAGMENT,
					buffer: {},
				},
			],
		});

		this.frameBindGroup = this.device.createBindGroup({
			label: `Frame BindGroup`,
			layout: this.frameBindGroupLayout,
			entries: [
				{
					binding: 0, // Camera uniforms
					resource: { buffer: this.frameUniformBuffer },
				},
			],
		});

		// this.statsFolder = this.pane.addFolder({
		// 	title: 'Stats',
		// 	expanded: false,
		// });
		// this.statsFolder.addBinding(this, 'frameMs', {
		// 	readonly: true,
		// 	view: 'graph',
		// 	min: 0,
		// 	max: 2,
		// });

		await this.onInit(this.device);
	}

	#allocateRenderTargets(size) {
		if (this.msaaColorTexture) {
			this.msaaColorTexture.destroy();
		}

		if (this.sampleCount > 1) {
			this.msaaColorTexture = this.device.createTexture({
				size,
				sampleCount: this.sampleCount,
				format: `${this.colorFormat}-srgb`,
				usage: GPUTextureUsage.RENDER_ATTACHMENT,
			});
		}

		if (this.depthTexture) {
			this.depthTexture.destroy();
		}

		this.depthTexture = this.device.createTexture({
			size,
			sampleCount: this.sampleCount,
			format: this.depthFormat,
			usage: GPUTextureUsage.RENDER_ATTACHMENT,
		});

		this.colorAttachment = {
			// Appropriate target will be populated in onFrame
			view: this.sampleCount > 1 ? this.msaaColorTexture.createView() : undefined,
			resolveTarget: undefined,

			clearValue: this.clearColor,
			loadOp: 'clear',
			storeOp: this.sampleCount > 1 ? 'discard' : 'store',
		};

		this.renderPassDescriptor = {
			colorAttachments: [this.colorAttachment],
			depthStencilAttachment: {
				view: this.depthTexture.createView(),
				depthClearValue: 1.0,
				depthLoadOp: 'clear',
				depthStoreOp: 'discard',
			},
		};
	}

	get defaultRenderPassDescriptor() {
		const colorTexture = this.context.getCurrentTexture().createView({ format: `${this.colorFormat}-srgb` } as never);
		if (this.sampleCount > 1) {
			this.colorAttachment.resolveTarget = colorTexture;
		} else {
			this.colorAttachment.view = colorTexture;
		}
		return this.renderPassDescriptor;
	}

	async onInit(device) {
		// Override to handle initialization logic
	}

	onResize(device, size) {
		// Override to handle resizing logic
	}

	onFrame(device, context, timestamp) {
		// Override to handle frame logic
	}
}

/*

class ResizeObserverHelper extends ResizeObserver {
    element: any;
    callback: any;
	constructor(element, callback) {
		super((entries) => {
			for (let entry of entries) {
				if (entry.target != element) {
					continue;
				}

				if (entry.devicePixelContentBoxSize) {
					// Should give exact pixel dimensions, but only works on Chrome.
					const devicePixelSize = entry.devicePixelContentBoxSize[0];
					callback(devicePixelSize.inlineSize, devicePixelSize.blockSize);
				} else if (entry.contentBoxSize) {
					// Firefox implements `contentBoxSize` as a single content rect, rather than an array
					const contentBoxSize = Array.isArray(entry.contentBoxSize) ? entry.contentBoxSize[0] : entry.contentBoxSize;
					callback(contentBoxSize.inlineSize, contentBoxSize.blockSize);
				} else {
					callback(entry.contentRect.width, entry.contentRect.height);
				}
			}
		});

		this.element = element;
		this.callback = callback;

		this.observe(element);
	}
}
*/
export class OrbitCamera {
	orbitX = 0;
	orbitY = 0;
	maxOrbitX = Math.PI * 0.5;
	minOrbitX = -Math.PI * 0.5;
	maxOrbitY = Math.PI;
	minOrbitY = -Math.PI;
	constrainXOrbit = true;
	constrainYOrbit = false;

	maxDistance = 10;
	minDistance = 1;
	distanceStep = 0.005;
	constrainDistance = true;

	#distance = vec3.fromValues(0, 0, 1);
	#target = vec3.create();
	#viewMat = mat4.create();
	#cameraMat = mat4.create();
	#position = vec3.create();
	#dirty = true;

	#element;
	#registerElement;

	constructor(element = null) {
		let moving = false;
		let lastX, lastY;

		const downCallback = (event) => {
			if (event.isPrimary) {
				moving = true;
			}
			lastX = event.pageX;
			lastY = event.pageY;
		};
		const moveCallback = (event) => {
			let xDelta, yDelta;

			// if(document.pointerLockEnabled) {
			//     xDelta = event.movementX;
			//     yDelta = event.movementY;
			//     this.orbit(xDelta * 0.025, yDelta * 0.025);
			// } else if (moving) {
			//     xDelta = event.pageX - lastX;
			//     yDelta = event.pageY - lastY;
			//     lastX = event.pageX;
			//     lastY = event.pageY;
			//     this.orbit(xDelta * 0.025, yDelta * 0.025);
			// }

			if (moving) {
				xDelta = event.pageX - lastX;
				yDelta = event.pageY - lastY;
				lastX = event.pageX;
				lastY = event.pageY;
				this.orbit(xDelta * 0.025, yDelta * 0.025);
			}
		};
		const upCallback = (event) => {
			if (event.isPrimary) {
				moving = false;
			}
		};
		const wheelCallback = (event) => {
			this.distance = this.#distance[2] + -event.wheelDeltaY * this.distanceStep;
			event.preventDefault();
		};

		this.#registerElement = (value) => {
			if (this.#element && this.#element != value) {
				this.#element.removeEventListener('pointerdown', downCallback);
				this.#element.removeEventListener('pointermove', moveCallback);
				this.#element.removeEventListener('pointerup', upCallback);
				this.#element.removeEventListener('mousewheel', wheelCallback);
			}

			this.#element = value;
			if (this.#element) {
				this.#element.addEventListener('pointerdown', downCallback);
				this.#element.addEventListener('pointermove', moveCallback);
				this.#element.addEventListener('pointerup', upCallback);
				this.#element.addEventListener('mousewheel', wheelCallback);
			}
		};

		this.#element = element;
		this.#registerElement(element);
	}

	set element(value) {
		this.#registerElement(value);
	}

	get element() {
		return this.#element;
	}

	orbit(xDelta, yDelta) {
		if (xDelta || yDelta) {
			this.orbitY += xDelta;
			if (this.constrainYOrbit) {
				this.orbitY = Math.min(Math.max(this.orbitY, this.minOrbitY), this.maxOrbitY);
			} else {
				while (this.orbitY < -Math.PI) {
					this.orbitY += Math.PI * 2;
				}
				while (this.orbitY >= Math.PI) {
					this.orbitY -= Math.PI * 2;
				}
			}

			this.orbitX += yDelta;
			if (this.constrainXOrbit) {
				this.orbitX = Math.min(Math.max(this.orbitX, this.minOrbitX), this.maxOrbitX);
			} else {
				while (this.orbitX < -Math.PI) {
					this.orbitX += Math.PI * 2;
				}
				while (this.orbitX >= Math.PI) {
					this.orbitX -= Math.PI * 2;
				}
			}

			this.#dirty = true;
		}
	}

	get target() {
		return [this.#target[0], this.#target[1], this.#target[2]];
	}

	set target(value) {
		this.#target[0] = value[0];
		this.#target[1] = value[1];
		this.#target[2] = value[2];
		this.#dirty = true;
	}

	get distance() {
		return this.#distance[2];
	}

	set distance(value) {
		this.#distance[2] = value;
		if (this.constrainDistance) {
			this.#distance[2] = Math.min(Math.max(this.#distance[2], this.minDistance), this.maxDistance);
		}
		this.#dirty = true;
	}

	#updateMatrices() {
		if (this.#dirty) {
			var mv = this.#cameraMat;
			mat4.identity(mv);

			mat4.translate(mv, mv, this.#target);
			mat4.rotateY(mv, mv, -this.orbitY);
			mat4.rotateX(mv, mv, -this.orbitX);
			mat4.translate(mv, mv, this.#distance);
			mat4.invert(this.#viewMat, this.#cameraMat);

			this.#dirty = false;
		}
	}

	get position() {
		this.#updateMatrices();
		vec3.set(this.#position, 0, 0, 0);
		vec3.transformMat4(this.#position, this.#position, this.#cameraMat);
		return this.#position;
	}

	get viewMatrix() {
		this.#updateMatrices();
		return this.#viewMat;
	}
}
