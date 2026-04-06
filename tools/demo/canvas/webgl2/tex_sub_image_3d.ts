import { createProgramFromScripts } from '../webgl/utils';

let texSubImage3DLAF = 0;

/**
 * Demonstrates texSubImage3D with TEXTURE_2D_ARRAY.
 *
 * A 4-layer texture array is allocated once via texImage3D (with null data).
 * Each animation frame the demo generates a unique animated ripple/wave pattern
 * per layer and uploads it individually with texSubImage3D, then cycles which
 * layer is displayed.
 */
export function texSubImage3DDemo(canvas) {
	const vs = `#version 300 es
in vec2 a_position;
out vec2 v_texcoord;
void main() {
  gl_Position = vec4(a_position, 0.0, 1.0);
  v_texcoord  = a_position * 0.5 + 0.5;
}`;

	const fs = `#version 300 es
precision highp float;
in  vec2 v_texcoord;
uniform highp sampler2DArray u_texture;
uniform float                u_layer;
out vec4 outColor;
void main() {
  outColor = texture(u_texture, vec3(v_texcoord, u_layer));
}`;

	var gl = canvas.getContext ? canvas.getContext('webgl2') : canvas;
	if (!gl) {
		console.error('texSubImage3DDemo: WebGL 2 not available');
		return;
	}

	canvas.width = canvas.clientWidth || 300;
	canvas.height = canvas.clientHeight || 300;

	const program = createProgramFromScripts(gl, [
		{ type: 'vertex', src: vs },
		{ type: 'fragment', src: fs },
	]);
	gl.useProgram(program);

	const vao = gl.createVertexArray();
	gl.bindVertexArray(vao);

	const posBuf = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, posBuf);
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]), gl.STATIC_DRAW);

	const posLoc = gl.getAttribLocation(program, 'a_position');
	gl.enableVertexAttribArray(posLoc);
	gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 0, 0);
	gl.bindVertexArray(null);

	const uTexture = gl.getUniformLocation(program, 'u_texture');
	const uLayer = gl.getUniformLocation(program, 'u_layer');

	const W = 64,
		H = 64,
		LAYERS = 4;

	// --- Allocate storage for all layers in one texImage3D call (null data = zeroed) ---
	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

	// Allocate storage for all layers without initialising pixel data
	gl.texImage3D(gl.TEXTURE_2D_ARRAY, 0, gl.RGBA, W, H, LAYERS, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);

	gl.activeTexture(gl.TEXTURE0);
	gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
	gl.uniform1i(uTexture, 0);

	// Reusable per-layer pixel buffer (one layer at a time)
	const layerBuf = new Uint8Array(W * H * 4);

	// Phase offsets per layer so each one animates differently
	const phaseOffsets = [0, Math.PI * 0.5, Math.PI, Math.PI * 1.5];
	// Hue base per layer (degrees)
	const baseHues = [0, 90, 180, 270];

	function hsvToRgb(h: number, s: number, v: number): [number, number, number] {
		h = ((h % 360) + 360) % 360;
		const c = v * s,
			x = c * (1 - Math.abs(((h / 60) % 2) - 1)),
			m = v - c;
		let r = 0,
			g = 0,
			b = 0;
		if (h < 60) {
			r = c;
			g = x;
		} else if (h < 120) {
			r = x;
			g = c;
		} else if (h < 180) {
			g = c;
			b = x;
		} else if (h < 240) {
			g = x;
			b = c;
		} else if (h < 300) {
			r = x;
			b = c;
		} else {
			r = c;
			b = x;
		}
		return [Math.round((r + m) * 255), Math.round((g + m) * 255), Math.round((b + m) * 255)];
	}

	const startTime = Date.now();

	function render() {
		texSubImage3DLAF = requestAnimationFrame(render);

		const elapsed = (Date.now() - startTime) / 1000;

		// Update every layer individually with texSubImage3D
		for (let layer = 0; layer < LAYERS; layer++) {
			const phase = phaseOffsets[layer];
			const hue = baseHues[layer] + elapsed * 40;

			for (let y = 0; y < H; y++) {
				for (let x = 0; x < W; x++) {
					const dx = x - W / 2;
					const dy = y - H / 2;
					const dist = Math.sqrt(dx * dx + dy * dy);
					const wave = Math.sin(dist * 0.45 - elapsed * 3.5 + phase) * 0.5 + 0.5;
					const [r, g, b] = hsvToRgb(hue, 0.9, wave);
					const idx = (y * W + x) * 4;
					layerBuf[idx] = r;
					layerBuf[idx + 1] = g;
					layerBuf[idx + 2] = b;
					layerBuf[idx + 3] = 255;
				}
			}

			// Ensure the texture array is bound before each update
			gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);

			// Push just this one layer into the texture array
			gl.texSubImage3D(
				gl.TEXTURE_2D_ARRAY, // target
				0, // mip level
				0, // xoffset
				0, // yoffset
				layer, // zoffset  (which layer to update)
				W, // width
				H, // height
				1, // depth  (update exactly 1 layer)
				gl.RGBA, // format
				gl.UNSIGNED_BYTE, // type
				layerBuf, // pixel data for this layer
			);
		}

		// Cycle through layers, one every 0.8 s
		const currentLayer = Math.floor(elapsed / 0.8) % LAYERS;

		gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
		gl.clearColor(0.05, 0.05, 0.05, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);

		gl.useProgram(program);
		gl.activeTexture(gl.TEXTURE0);
		gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
		gl.uniform1i(uTexture, 0);
		gl.bindVertexArray(vao);
		gl.uniform1f(uLayer, currentLayer);
		gl.drawArrays(gl.TRIANGLES, 0, 6);
	}

	render();
}

export function cancelTexSubImage3DDemo() {
	cancelAnimationFrame(texSubImage3DLAF);
	texSubImage3DLAF = 0;
}
