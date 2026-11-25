import type { View } from '@nativescript/core';
import { GLSL_BURN, GLSL_FX_VERTEX, GLSL_HAZE, GLSL_INK, GLSL_PIXELS, GLSL_QUAD, GLSL_QUAD_VERTEX, GLSL_RIPPLE, GLSL_VORTEX, GLSL_WARP, WGSL_FX, WGSL_QUAD } from './canvas-html-transitions-shaders';

/** A drawable's snapshot on the GPU; `sx`/`sy` is the part of the texture the snapshot covers. */
export interface Tex {
	handle: any;
	w: number;
	h: number;
	sx: number;
	sy: number;
}

export const FX_NAMES = ['ripple', 'warp', 'haze', 'vortex', 'ink', 'burn', 'pixels'] as const;
export type FxName = (typeof FX_NAMES)[number];
export type Vec4 = [number, number, number, number];

/** Floats per quad draw (256 bytes, WebGPU's dynamic offset alignment). */
export const QUAD_FLOATS = 64;

export interface Renderer {
	name: string;
	/** The list scene, drawn first so the effects can sample it. */
	scene: Tex;
	/** The scene's uv rect when drawn as a quad (GL framebuffers are stored bottom-up). */
	sceneUv: Vec4;
	upload(view: View, existing: Tex | undefined, width: number, height: number): Tex;
	resize(width: number, height: number): void;
	frame(width: number, height: number): void;
	begin(target: 'scene' | 'screen', clear: Vec4): void;
	quad(data: Float32Array, tex: Tex | null): void;
	effect(name: FxName, data: Float32Array, to: Tex): void;
	end(): void;
	submit(): void;
}

export async function webgpuRenderer(canvas: any): Promise<Renderer | null> {
	const adapter = await navigator.gpu?.requestAdapter();
	if (!adapter) {
		return null;
	}
	const device = await adapter.requestDevice();
	const TextureUsage = (global as any).GPUTextureUsage;
	const BufferUsage = (global as any).GPUBufferUsage;
	const ShaderStage = (global as any).GPUShaderStage;
	const context = canvas.getContext('webgpu') as GPUCanvasContext;
	const format = navigator.gpu.getPreferredCanvasFormat();
	context.configure({ device, format, alphaMode: 'opaque' });

	const SLOTS = 256;
	const SLOT_BYTES = QUAD_FLOATS * 4;
	const uniforms = device.createBuffer({ size: SLOTS * SLOT_BYTES, usage: BufferUsage.UNIFORM | BufferUsage.COPY_DST });
	const data = new Float32Array(SLOTS * QUAD_FLOATS);
	const sampler = device.createSampler({ magFilter: 'linear', minFilter: 'linear' });
	const uniformEntry = { binding: 0, visibility: ShaderStage.VERTEX | ShaderStage.FRAGMENT, buffer: { type: 'uniform', hasDynamicOffset: true, minBindingSize: SLOT_BYTES } };
	const samplerEntry = { binding: 1, visibility: ShaderStage.FRAGMENT, sampler: {} };
	const textureEntry = (binding: number) => ({ binding, visibility: ShaderStage.FRAGMENT, texture: {} });
	const quadLayout = device.createBindGroupLayout({ entries: [uniformEntry, samplerEntry, textureEntry(2)] as any });
	const fxLayout = device.createBindGroupLayout({ entries: [uniformEntry, samplerEntry, textureEntry(2), textureEntry(3)] as any });

	const premultiplied: GPUBlendComponent = { srcFactor: 'one', dstFactor: 'one-minus-src-alpha' };
	const quadModule = device.createShaderModule({ code: WGSL_QUAD });
	const quadPipeline = device.createRenderPipeline({
		layout: device.createPipelineLayout({ bindGroupLayouts: [quadLayout] }),
		vertex: { module: quadModule, entryPoint: 'quadVertex' },
		fragment: { module: quadModule, entryPoint: 'quadFragment', targets: [{ format, blend: { color: premultiplied, alpha: premultiplied } }] },
		primitive: { topology: 'triangle-strip' },
	});
	const fxModule = device.createShaderModule({ code: WGSL_FX });
	const fxPipelineLayout = device.createPipelineLayout({ bindGroupLayouts: [fxLayout] });
	const fxPipelines: Record<FxName, GPURenderPipeline> = {} as any;
	for (const name of FX_NAMES) {
		fxPipelines[name] = device.createRenderPipeline({
			layout: fxPipelineLayout,
			vertex: { module: fxModule, entryPoint: 'fxVertex' },
			fragment: { module: fxModule, entryPoint: `${name}Fragment`, targets: [{ format }] },
			primitive: { topology: 'triangle-list' },
		});
	}

	const blank = device.createTexture({ size: [1, 1], format: 'rgba8unorm', usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST });
	const quadGroups = new Map<GPUTexture, GPUBindGroup>();
	const quadGroup = (texture: GPUTexture) => {
		let group = quadGroups.get(texture);
		if (!group) {
			group = device.createBindGroup({
				layout: quadLayout,
				entries: [
					{ binding: 0, resource: { buffer: uniforms, size: SLOT_BYTES } },
					{ binding: 1, resource: sampler },
					{ binding: 2, resource: texture.createView() },
				],
			});
			quadGroups.set(texture, group);
		}
		return group;
	};
	const fxGroups = new Map<string, { group: GPUBindGroup; from: GPUTexture; to: GPUTexture }>();

	const scene: Tex = { handle: null, w: 0, h: 0, sx: 1, sy: 1 };
	let encoder: GPUCommandEncoder | null = null;
	let pass: GPURenderPassEncoder | null = null;
	let slot = 0;

	const take = (source: Float32Array) => {
		if (slot >= SLOTS) {
			return -1;
		}
		data.set(source.subarray(0, QUAD_FLOATS), slot * QUAD_FLOATS);
		return slot++ * SLOT_BYTES;
	};

	return {
		name: 'WebGPU',
		scene,
		sceneUv: [0, 0, 1, 1],
		upload(view, existing, width, height) {
			let tex = existing;
			if (!tex || tex.w !== width || tex.h !== height) {
				if (tex) {
					quadGroups.delete(tex.handle);
					tex.handle.destroy();
				}
				// RENDER_ATTACHMENT, and the snapshot's own size: the copy then stays on the GPU.
				const handle = device.createTexture({ size: [width, height], format: 'rgba8unorm', usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST | TextureUsage.RENDER_ATTACHMENT });
				tex = { handle, w: width, h: height, sx: 1, sy: 1 };
			}
			(device.queue as any).drawElementImageToTexture({ source: view }, { texture: tex.handle });
			return tex;
		},
		resize() {
			context.configure({ device, format, alphaMode: 'opaque' });
		},
		frame(width, height) {
			if (scene.w !== width || scene.h !== height) {
				if (scene.handle) {
					quadGroups.delete(scene.handle);
					scene.handle.destroy();
				}
				scene.handle = device.createTexture({ size: [width, height], format, usage: TextureUsage.TEXTURE_BINDING | TextureUsage.RENDER_ATTACHMENT });
				scene.w = width;
				scene.h = height;
			}
			encoder = device.createCommandEncoder();
			slot = 0;
		},
		begin(target, clear) {
			const view = target === 'scene' ? scene.handle.createView() : context.getCurrentTexture().createView();
			pass = encoder.beginRenderPass({ colorAttachments: [{ view, loadOp: 'clear', storeOp: 'store', clearValue: { r: clear[0], g: clear[1], b: clear[2], a: clear[3] } }] });
		},
		quad(source, tex) {
			const offset = take(source);
			if (offset < 0) {
				return;
			}
			pass.setPipeline(quadPipeline);
			pass.setBindGroup(0, quadGroup(tex?.handle ?? blank), [offset]);
			pass.draw(4);
		},
		effect(name, source, to) {
			const offset = take(source);
			if (offset < 0) {
				return;
			}
			let cached = fxGroups.get(name);
			if (!cached || cached.from !== scene.handle || cached.to !== to.handle) {
				const group = device.createBindGroup({
					layout: fxLayout,
					entries: [
						{ binding: 0, resource: { buffer: uniforms, size: SLOT_BYTES } },
						{ binding: 1, resource: sampler },
						{ binding: 2, resource: scene.handle.createView() },
						{ binding: 3, resource: to.handle.createView() },
					],
				});
				cached = { group, from: scene.handle, to: to.handle };
				fxGroups.set(name, cached);
			}
			pass.setPipeline(fxPipelines[name]);
			pass.setBindGroup(0, cached.group, [offset]);
			pass.draw(3);
		},
		end() {
			pass?.end();
			pass = null;
		},
		submit() {
			if (!encoder) {
				return;
			}
			if (slot > 0) {
				device.queue.writeBuffer(uniforms, 0, data, 0, slot * QUAD_FLOATS);
			}
			device.queue.submit([encoder.finish()]);
			encoder = null;
		},
	};
}

/** For devices with no usable WebGPU adapter. Snapshots reach WebGL through a CPU copy. */
export function webglRenderer(canvas: any): Renderer {
	const gl = canvas.getContext('webgl', { alpha: false, antialias: false }) as WebGLRenderingContext & any;
	const compile = (vertex: string, fragment: string) => {
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
		const u: Record<string, WebGLUniformLocation> = {};
		const count = gl.getProgramParameter(program, gl.ACTIVE_UNIFORMS);
		for (let i = 0; i < count; i++) {
			const name = gl.getActiveUniform(program, i).name;
			u[name] = gl.getUniformLocation(program, name);
		}
		return { program, u };
	};
	const quadProgram = compile(GLSL_QUAD_VERTEX, GLSL_QUAD);
	const sources: Record<FxName, string> = { ripple: GLSL_RIPPLE, warp: GLSL_WARP, haze: GLSL_HAZE, vortex: GLSL_VORTEX, ink: GLSL_INK, burn: GLSL_BURN, pixels: GLSL_PIXELS };
	const fxPrograms = {} as Record<FxName, ReturnType<typeof compile>>;
	for (const name of FX_NAMES) {
		fxPrograms[name] = compile(GLSL_FX_VERTEX, sources[name]);
	}
	const corners = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, corners);
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0, 1, 0, 0, 1, 1, 1]), gl.STATIC_DRAW);
	const newTexture = () => {
		const tex = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, tex);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		return tex;
	};
	const scene: Tex = { handle: newTexture(), w: 0, h: 0, sx: 1, sy: 1 };
	const fbo = gl.createFramebuffer();
	let height = 0;

	const prepare = (program: WebGLProgram) => {
		gl.useProgram(program);
		gl.bindBuffer(gl.ARRAY_BUFFER, corners);
		gl.enableVertexAttribArray(0);
		gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
	};

	return {
		name: 'WebGL',
		scene,
		sceneUv: [0, 1, 1, 0],
		upload(view, existing, width, h) {
			const tex = existing ?? { handle: newTexture(), w: 0, h: 0, sx: 1, sy: 1 };
			gl.bindTexture(gl.TEXTURE_2D, tex.handle);
			if (tex.w !== width || tex.h !== h) {
				gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
				tex.w = width;
				tex.h = h;
			}
			gl.texElementSubImage2D(gl.TEXTURE_2D, 0, 0, 0, view);
			return tex;
		},
		resize() {},
		frame(width, h) {
			if (scene.w !== width || scene.h !== h) {
				gl.bindTexture(gl.TEXTURE_2D, scene.handle);
				gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
				gl.bindFramebuffer(gl.FRAMEBUFFER, fbo);
				gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, scene.handle, 0);
				gl.bindFramebuffer(gl.FRAMEBUFFER, null);
				scene.w = width;
				scene.h = h;
			}
			height = h;
			gl.enable(gl.BLEND);
			gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
		},
		begin(target, clear) {
			gl.bindFramebuffer(gl.FRAMEBUFFER, target === 'scene' ? fbo : null);
			gl.viewport(0, 0, scene.w, scene.h);
			gl.clearColor(clear[0], clear[1], clear[2], clear[3]);
			gl.clear(gl.COLOR_BUFFER_BIT);
		},
		quad(d, tex) {
			const u = quadProgram.u;
			prepare(quadProgram.program);
			gl.uniformMatrix4fv(u.u_proj, false, d.subarray(0, 16));
			gl.uniformMatrix4fv(u.u_model, false, d.subarray(16, 32));
			gl.uniform4f(u.u_dst, d[32], d[33], d[34], d[35]);
			gl.uniform4f(u.u_uv, d[36], d[37], d[38], d[39]);
			gl.uniform4f(u.u_color, d[40], d[41], d[42], d[43]);
			gl.uniform4f(u.u_clip, d[44], d[45], d[46], d[47]);
			gl.uniform2f(u.u_size, d[48], d[49]);
			gl.uniform1f(u.u_radius, d[50]);
			gl.uniform1f(u.u_alpha, d[51]);
			gl.uniform1f(u.u_mode, d[52]);
			gl.uniform1f(u.u_soft, d[53]);
			gl.uniform1f(u.u_shade, d[54]);
			gl.uniform1f(u.u_clipRadius, d[55]);
			gl.uniform1f(u.u_height, height);
			if (tex) {
				gl.activeTexture(gl.TEXTURE0);
				gl.bindTexture(gl.TEXTURE_2D, tex.handle);
				gl.uniform1i(u.u_tex, 0);
			}
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
		},
		effect(name, d, to) {
			const { program, u } = fxPrograms[name];
			prepare(program);
			// Uniforms an effect never reads are compiled out; setting them is a no-op.
			gl.uniform2f(u.u_res, d[0], d[1]);
			gl.uniform2f(u.u_toScale, d[2], d[3]);
			gl.uniform2f(u.u_origin, d[4], d[5]);
			gl.uniform1f(u.u_p, d[6]);
			gl.uniform1f(u.u_time, d[7]);
			gl.uniform4f(u.u_tintA, d[8], d[9], d[10], d[11]);
			gl.uniform4f(u.u_tintB, d[12], d[13], d[14], d[15]);
			gl.uniform2f(u.u_shift, d[16], d[17]);
			gl.activeTexture(gl.TEXTURE0);
			gl.bindTexture(gl.TEXTURE_2D, scene.handle);
			gl.uniform1i(u.u_from, 0);
			gl.activeTexture(gl.TEXTURE1);
			gl.bindTexture(gl.TEXTURE_2D, to.handle);
			gl.uniform1i(u.u_to, 1);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
			gl.activeTexture(gl.TEXTURE0);
		},
		end() {},
		submit() {},
	};
}
