import { EventData, Observable, Page, Screen, View } from '@nativescript/core';
import type { Canvas } from '@nativescript/canvas';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new WebViewInCanvasModel();
}

/**
 * The page inside the WebView. Where the engine has HTML-in-Canvas it draws its own card with
 * drawElementImage, like the outer canvas does with the whole WebView; elsewhere the same card
 * is a CSS 3D transform, so the page still says which one you are looking at.
 */
const PAGE = `<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width,initial-scale=1,user-scalable=no">
<style>
	:root { color-scheme: light; font-family: -apple-system, system-ui, Roboto, sans-serif; }
	body { margin: 0; padding: 16px; background: #fff; color: #111827; }
	h1 { font-size: 20px; margin: 0 0 4px; }
	.muted { color: #6b7280; font-size: 13px; margin: 0 0 12px; }
	button { font: inherit; font-weight: 600; width: 100%; padding: 10px; border: 0; border-radius: 10px; background: #4f46e5; color: #fff; }
	input { font: inherit; box-sizing: border-box; width: 100%; padding: 8px 10px; margin: 10px 0 4px; border: 1px solid #d1d5db; border-radius: 8px; }
	.bar { height: 6px; border-radius: 3px; margin: 12px 0; background: linear-gradient(90deg, #4f46e5, #ec4899, #f59e0b); }
	.stage { position: relative; height: 150px; border-radius: 12px; background: #f3f4f6; overflow: hidden; perspective: 400px; }
	.stage canvas { width: 100%; height: 100%; display: block; }
	.card { width: 170px; padding: 10px 12px; border-radius: 10px; background: #111827; color: #fff; font-size: 13px; }
	.card b { color: #f472b6; }
	.fallback .card { position: absolute; left: 50%; top: 50%; margin: -28px 0 0 -97px; transform: rotateY(-25deg) rotateZ(-4deg); transition: transform 0.6s ease-in-out; }
	.fallback .card.flipped { transform: rotateY(25deg) rotateZ(4deg); }
	.badge { display: inline-block; margin-top: 8px; padding: 2px 8px; border-radius: 99px; font-size: 12px; background: #e0e7ff; color: #3730a3; }
	h2 { font-size: 16px; margin: 20px 0 8px; }
	ol { margin: 0; padding: 0; list-style: none; }
	li { display: flex; align-items: center; gap: 10px; padding: 10px 0; border-bottom: 1px solid #e5e7eb; font-size: 14px; }
	li span { width: 28px; height: 28px; border-radius: 8px; flex: none; }
</style>
</head>
<body>
	<h1>A real web page</h1>
	<p class="muted" id="engine"></p>
	<button id="tap">Tap me</button>
	<input id="name" placeholder="Type here">
	<div id="echo" class="muted">Hello …</div>
	<div class="bar"></div>
	<div class="stage" id="stage"></div>
	<span class="badge" id="mode"></span>
	<h2>Scroll the page</h2>
	<ol id="list"></ol>
<script>
(function () {
	var list = document.getElementById('list');
	for (var i = 1; i <= 24; i++) {
		var li = document.createElement('li');
		li.innerHTML = '<span style="background:hsl(' + (i * 15) + ',70%,60%)"></span>Row ' + i + ' of a page that scrolls inside the canvas';
		list.appendChild(li);
	}

	var ua = navigator.userAgent;
	document.getElementById('engine').textContent = /Chrome\\//.test(ua) ? 'Android WebView (Chromium)' : /AppleWebKit/.test(ua) ? 'WKWebView (WebKit)' : ua;

	var taps = 0;
	document.getElementById('tap').addEventListener('click', function () {
		taps++;
		this.textContent = 'Tapped ' + taps + (taps === 1 ? ' time' : ' times');
	});
	document.getElementById('name').addEventListener('input', function () {
		document.getElementById('echo').textContent = 'Hello ' + (this.value || '…');
	});

	var stage = document.getElementById('stage');
	var card = '<div class="card" id="card">HTML inside a canvas, inside a <b>WebView</b>, inside a canvas.</div>';
	var ctx2d = window.CanvasRenderingContext2D && CanvasRenderingContext2D.prototype;
	var draw = ctx2d && (ctx2d.drawElementImage || ctx2d.drawElement);
	if (!draw) {
		stage.className = 'stage fallback';
		stage.innerHTML = card;
		document.getElementById('card').addEventListener('click', function () { this.classList.toggle('flipped'); });
		document.getElementById('mode').textContent = 'No drawElementImage here: CSS 3D fallback. Tap the card.';
		return;
	}

	// The WICG attribute names have changed while it was behind a flag: set both.
	var canvas = document.createElement('canvas');
	canvas.setAttribute('layoutsubtree', '');
	canvas.setAttribute('content', 'drawable');
	canvas.innerHTML = card;
	stage.appendChild(canvas);
	var el = canvas.firstElementChild;
	el.setAttribute('drawable', '');
	document.getElementById('mode').textContent = 'Drawn with the page\\'s own drawElementImage';

	var ctx = canvas.getContext('2d');
	(function frame(t) {
		var dpr = window.devicePixelRatio || 1;
		var w = canvas.clientWidth * dpr, h = canvas.clientHeight * dpr;
		if (canvas.width !== w || canvas.height !== h) { canvas.width = w; canvas.height = h; }
		ctx.setTransform(1, 0, 0, 1, 0, 0);
		ctx.clearRect(0, 0, w, h);
		ctx.translate(w / 2, h / 2);
		ctx.rotate(Math.sin(t / 900) * 0.2);
		ctx.scale(dpr, dpr);
		try { draw.call(ctx, el, -el.offsetWidth / 2, -el.offsetHeight / 2); } catch (e) {}
		requestAnimationFrame(frame);
	})(0);
})();
</script>
</body>
</html>`;

const SHADERS = /* wgsl */ `
struct Uniforms {
	matrix: mat4x4f,
	grid: vec2f,
	element: vec2f,
	reflection: f32,
};
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var pageSampler: sampler;
@group(0) @binding(2) var page: texture_2d<f32>;

struct PageOut {
	@builtin(position) position: vec4f,
	@location(0) uv: vec2f,
};

@vertex fn pageVertex(@builtin(vertex_index) i: u32) -> PageOut {
	let corner = vec2f(f32(i & 1u), f32(i >> 1u));
	let p = u.matrix * vec4f(corner * u.element, 0.0, 1.0);
	var out: PageOut;
	// Grid px to clip space, kept homogeneous so the texture stays perspective-correct.
	out.position = vec4f(2.0 * p.x / u.grid.x - p.w, p.w - 2.0 * p.y / u.grid.y, 0.0, p.w);
	out.uv = corner;
	return out;
}

@fragment fn pageFragment(in: PageOut) -> @location(0) vec4f {
	let color = textureSample(page, pageSampler, in.uv);
	if (u.reflection > 0.0) {
		return color * (u.reflection * in.uv.y * in.uv.y * in.uv.y);
	}
	return color;
}

struct BackgroundOut {
	@builtin(position) position: vec4f,
	@location(0) t: f32,
};

@vertex fn backgroundVertex(@builtin(vertex_index) i: u32) -> BackgroundOut {
	let xy = vec2f(f32((i << 1u) & 2u), f32(i & 2u)) * 2.0 - 1.0;
	var out: BackgroundOut;
	out.position = vec4f(xy, 0.0, 1.0);
	out.t = xy.y * 0.5 + 0.5;
	return out;
}

@fragment fn backgroundFragment(in: BackgroundOut) -> @location(0) vec4f {
	return vec4f(mix(vec3f(0.02, 0.02, 0.06), vec3f(0.16, 0.09, 0.32), clamp(in.t, 0.0, 1.0)), 1.0);
}
`;

type Mat4 = number[];

const mat4 = {
	/** Column-major, b applies first. */
	multiply(a: Mat4, b: Mat4): Mat4 {
		const out = new Array(16).fill(0);
		for (let c = 0; c < 4; c++) for (let r = 0; r < 4; r++) for (let k = 0; k < 4; k++) out[c * 4 + r] += a[k * 4 + r] * b[c * 4 + k];
		return out;
	},
	chain(...ms: Mat4[]): Mat4 {
		return ms.reduce((a, b) => mat4.multiply(a, b));
	},
	translate: (x: number, y: number): Mat4 => [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, x, y, 0, 1],
	scale: (x: number, y: number): Mat4 => [x, 0, 0, 0, 0, y, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
	perspective: (d: number): Mat4 => [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, -1 / d, 0, 0, 0, 1],
	rotateX(a: number): Mat4 {
		const c = Math.cos(a),
			s = Math.sin(a);
		return [1, 0, 0, 0, 0, c, s, 0, 0, -s, c, 0, 0, 0, 0, 1];
	},
	rotateY(a: number): Mat4 {
		const c = Math.cos(a),
			s = Math.sin(a);
		return [c, 0, -s, 0, 0, 1, 0, 0, s, 0, c, 0, 0, 0, 0, 1];
	},
};

const GLSL_PAGE_VERTEX = `
attribute vec2 a_corner;
uniform mat4 u_matrix;
uniform vec2 u_grid;
uniform vec2 u_element;
varying vec2 v_uv;
void main() {
	vec4 p = u_matrix * vec4(a_corner * u_element, 0.0, 1.0);
	gl_Position = vec4(2.0 * p.x / u_grid.x - p.w, p.w - 2.0 * p.y / u_grid.y, 0.0, p.w);
	v_uv = a_corner;
}`;

const GLSL_PAGE_FRAGMENT = `
precision mediump float;
uniform sampler2D u_page;
uniform float u_reflection;
varying vec2 v_uv;
void main() {
	vec4 color = texture2D(u_page, v_uv);
	gl_FragColor = u_reflection > 0.0 ? color * (u_reflection * v_uv.y * v_uv.y * v_uv.y) : color;
}`;

const GLSL_BACKGROUND_VERTEX = `
attribute vec2 a_corner;
varying float v_t;
void main() {
	vec2 xy = a_corner * 2.0 - 1.0;
	v_t = a_corner.y;
	gl_Position = vec4(xy, 0.0, 1.0);
}`;

const GLSL_BACKGROUND_FRAGMENT = `
precision mediump float;
varying float v_t;
void main() {
	gl_FragColor = vec4(mix(vec3(0.02, 0.02, 0.06), vec3(0.16, 0.09, 0.32), v_t), 1.0);
}`;

/** Where the page goes this frame: element CSS px to canvas grid px. */
interface Placement {
	matrix: Mat4;
	mirror: Mat4;
	gridW: number;
	gridH: number;
	cssW: number;
	cssH: number;
}

interface Renderer {
	name: string;
	/** Copies the WebView's latest snapshot, `width` x `height` px. */
	upload(width: number, height: number): void;
	draw(placement: Placement | null): void;
	dispose(): void;
}

async function webgpuRenderer(canvas: any, web: View): Promise<Renderer | null> {
	const adapter = await navigator.gpu?.requestAdapter();
	if (!adapter) {
		return null;
	}
	const device = await adapter.requestDevice();
	const GPUTextureUsage = (global as any).GPUTextureUsage;
	const GPUBufferUsage = (global as any).GPUBufferUsage;
	const context = canvas.getContext('webgpu') as GPUCanvasContext;
	const format = navigator.gpu.getPreferredCanvasFormat();
	context.configure({ device, format, alphaMode: 'opaque' });

	const module = device.createShaderModule({ code: SHADERS });
	const background = device.createRenderPipeline({
		layout: 'auto',
		vertex: { module, entryPoint: 'backgroundVertex' },
		fragment: { module, entryPoint: 'backgroundFragment', targets: [{ format }] },
	});
	const premultiplied: GPUBlendComponent = { srcFactor: 'one', dstFactor: 'one-minus-src-alpha' };
	const pagePipeline = device.createRenderPipeline({
		layout: 'auto',
		vertex: { module, entryPoint: 'pageVertex' },
		fragment: { module, entryPoint: 'pageFragment', targets: [{ format, blend: { color: premultiplied, alpha: premultiplied } }] },
		primitive: { topology: 'triangle-strip' },
	});
	const sampler = device.createSampler({ magFilter: 'linear', minFilter: 'linear' });
	// One buffer per draw: two writes to one buffer before a submit would both read the last.
	const uniforms = [0, 1].map(() => device.createBuffer({ size: 96, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }));
	const uniformData = new Float32Array(24);
	let texture: GPUTexture | null = null;
	let bindGroups: GPUBindGroup[] = [];

	const writeUniforms = (buffer: GPUBuffer, matrix: Mat4, p: Placement, reflection: number) => {
		uniformData.set(matrix, 0);
		uniformData.set([p.gridW, p.gridH, p.cssW, p.cssH, reflection], 16);
		device.queue.writeBuffer(buffer, 0, uniformData);
	};

	return {
		name: 'WebGPU',
		upload(width, height) {
			if (!texture || texture.width !== width || texture.height !== height) {
				texture?.destroy();
				// RENDER_ATTACHMENT, and the snapshot's own size: the copy then stays on the GPU.
				texture = device.createTexture({
					size: [width, height],
					format: 'rgba8unorm',
					usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT,
				});
				const view = texture.createView();
				bindGroups = uniforms.map((buffer) =>
					device.createBindGroup({
						layout: pagePipeline.getBindGroupLayout(0),
						entries: [
							{ binding: 0, resource: { buffer } },
							{ binding: 1, resource: sampler },
							{ binding: 2, resource: view },
						],
					}),
				);
			}
			(device.queue as any).drawElementImageToTexture({ source: web }, { texture });
		},
		draw(placement) {
			const encoder = device.createCommandEncoder();
			const pass = encoder.beginRenderPass({
				colorAttachments: [{ view: context.getCurrentTexture().createView(), loadOp: 'clear', storeOp: 'store', clearValue: [0, 0, 0, 1] }],
			});
			pass.setPipeline(background);
			pass.draw(3);
			if (placement && texture) {
				writeUniforms(uniforms[0], placement.mirror, placement, 0.3);
				writeUniforms(uniforms[1], placement.matrix, placement, 0);
				pass.setPipeline(pagePipeline);
				pass.setBindGroup(0, bindGroups[0]);
				pass.draw(4);
				pass.setBindGroup(0, bindGroups[1]);
				pass.draw(4);
			}
			pass.end();
			device.queue.submit([encoder.finish()]);
		},
		dispose() {
			texture?.destroy();
			device.destroy();
		},
	};
}

function compile(gl: WebGLRenderingContext, vertex: string, fragment: string) {
	const program = gl.createProgram();
	for (const [type, source] of [
		[gl.VERTEX_SHADER, vertex],
		[gl.FRAGMENT_SHADER, fragment],
	] as const) {
		const shader = gl.createShader(type);
		gl.shaderSource(shader, source);
		gl.compileShader(shader);
		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			throw new Error(gl.getShaderInfoLog(shader));
		}
		gl.attachShader(program, shader);
	}
	gl.bindAttribLocation(program, 0, 'a_corner');
	gl.linkProgram(program);
	return program;
}

/** For devices with no WebGPU adapter. Snapshots reach WebGL through a CPU copy. */
function webglRenderer(canvas: any, web: View): Renderer {
	const gl = canvas.getContext('webgl') as WebGLRenderingContext & any;
	const background = compile(gl, GLSL_BACKGROUND_VERTEX, GLSL_BACKGROUND_FRAGMENT);
	const page = compile(gl, GLSL_PAGE_VERTEX, GLSL_PAGE_FRAGMENT);
	const u = {
		matrix: gl.getUniformLocation(page, 'u_matrix'),
		grid: gl.getUniformLocation(page, 'u_grid'),
		element: gl.getUniformLocation(page, 'u_element'),
		page: gl.getUniformLocation(page, 'u_page'),
		reflection: gl.getUniformLocation(page, 'u_reflection'),
	};
	const corners = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, corners);
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0, 1, 0, 0, 1, 1, 1]), gl.STATIC_DRAW);
	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D, texture);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
	let textureW = 0;
	let textureH = 0;

	return {
		name: 'WebGL',
		upload(width, height) {
			gl.bindTexture(gl.TEXTURE_2D, texture);
			if (width !== textureW || height !== textureH) {
				gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
				textureW = width;
				textureH = height;
			}
			gl.texElementSubImage2D(gl.TEXTURE_2D, 0, 0, 0, web);
		},
		draw(placement) {
			gl.viewport(0, 0, canvas.width, canvas.height);
			gl.bindBuffer(gl.ARRAY_BUFFER, corners);
			gl.enableVertexAttribArray(0);
			gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
			gl.disable(gl.BLEND);
			gl.useProgram(background);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
			if (!placement || !textureW) {
				return;
			}
			gl.enable(gl.BLEND);
			gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
			gl.useProgram(page);
			gl.activeTexture(gl.TEXTURE0);
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.uniform1i(u.page, 0);
			gl.uniform2f(u.grid, placement.gridW, placement.gridH);
			gl.uniform2f(u.element, placement.cssW, placement.cssH);
			gl.uniformMatrix4fv(u.matrix, false, new Float32Array(placement.mirror));
			gl.uniform1f(u.reflection, 0.3);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
			gl.uniformMatrix4fv(u.matrix, false, new Float32Array(placement.matrix));
			gl.uniform1f(u.reflection, 0);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
		},
		dispose() {
			gl.deleteTexture(texture);
		},
	};
}

class WebViewInCanvasModel extends Observable {
	private raf = 0;
	private paints = 0;
	private alive = true;
	private dispose: (() => void) | null = null;

	constructor() {
		super();
		this.set('html', PAGE);
		this.set('status', 'waiting for a snapshot…');
	}

	async canvasLoaded(args: EventData) {
		const canvas = args.object as Canvas & any;
		const web = canvas.getViewById('web') as View;
		if (__APPLE__) {
			// It is never on screen where it is laid out, so the window's safe area is not its business.
			const scrollView = (web.nativeView as any).scrollView;
			scrollView.contentInsetAdjustmentBehavior = UIScrollViewContentInsetAdjustmentBehavior.Never;
			// iOS 26 blurs content under the top edge, for a bar this page does not have.
			if (scrollView.respondsToSelector('topEdgeEffect')) {
				scrollView.setValueForKeyPath(true, 'topEdgeEffect.hidden');
			}
		}
		const renderer = (await webgpuRenderer(canvas, web)) ?? webglRenderer(canvas, web);
		if (!this.alive) {
			renderer.dispose();
			return;
		}

		const onPaint = (event: any) => {
			if (!event.changedElements.includes(web)) {
				return;
			}
			const size = canvas.getElementImageDefaultSize(web);
			renderer.upload(Math.max(1, Math.ceil(size.width - 1e-3)), Math.max(1, Math.ceil(size.height - 1e-3)));
			this.paints++;
			this.set('status', `${renderer.name}: ${this.paints} snapshot${this.paints === 1 ? '' : 's'} of the page so far`);
		};
		canvas.on('paint', onPaint);
		this.dispose = () => {
			canvas.off('paint', onPaint);
			renderer.dispose();
		};

		const place = (): Placement | null => {
			const size = canvas.getElementImageDefaultSize(web);
			if (!this.paints || !size.width) {
				return null;
			}
			const gridW = canvas.width;
			const gridH = canvas.height;
			// Not web.getActualSize(): on iOS that is the frame, which follows the transform set below.
			const grid = gridW / (canvas.getMeasuredWidth() / Screen.mainScreen.scale || 1);
			const cssW = size.width / grid;
			const cssH = size.height / grid;
			const fit = Math.min(1, (gridW * 0.82) / size.width, (gridH * 0.7) / size.height);
			// A still pose: a moving view makes the platform re-raster it, so every frame would be a new capture.
			const tilt = mat4.chain(mat4.perspective(1400 * grid), mat4.rotateY(-0.3), mat4.rotateX(-0.12));
			const center = mat4.translate(gridW / 2, gridH * 0.44);
			const toGrid = mat4.scale(grid * fit, grid * fit);
			const matrix = mat4.chain(center, tilt, toGrid, mat4.translate(-cssW / 2, -cssH / 2));
			// Mirrored about the page's bottom edge, a small gap below it.
			const mirror = mat4.chain(center, tilt, toGrid, mat4.translate(0, cssH / 2 + 6), mat4.scale(1, -1), mat4.translate(-cssW / 2, -cssH));
			return { matrix, mirror, gridW, gridH, cssW, cssH };
		};

		const draw = () => {
			if (!this.alive) {
				return;
			}
			this.raf = requestAnimationFrame(draw);
			// As on the web: the drawing buffer follows the layout only when told to.
			const layoutW = Math.round(canvas.getMeasuredWidth());
			const layoutH = Math.round(canvas.getMeasuredHeight());
			if (layoutW > 0 && layoutH > 0 && (canvas.width !== layoutW || canvas.height !== layoutH)) {
				canvas.width = layoutW;
				canvas.height = layoutH;
			}
			const placement = place();
			if (placement) {
				// The drawn position is also where touches reach the WebView.
				canvas.updateElementGeometry(web, { canvasTransform: new Float64Array(placement.matrix) });
			}
			renderer.draw(placement);
		};
		this.raf = requestAnimationFrame(draw);
	}

	canvasUnloaded() {
		this.alive = false;
		cancelAnimationFrame(this.raf);
		this.dispose?.();
		this.dispose = null;
	}
}
