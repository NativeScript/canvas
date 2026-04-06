import { createProgramFromScripts } from '../webgl/utils';

let texImage3DLAF = 0;

/**
 * Demonstrates texImage3D with TEXTURE_2D_ARRAY.
 *
 * Creates a 6-layer texture array where each layer holds a distinct
 * colour gradient. All layers are uploaded in a single texImage3D call
 * using a pre-filled Uint8Array, then the demo cycles through them.
 */
export function texImage3DDemo(canvas) {
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
		console.error('texImage3DDemo: WebGL 2 not available');
		return;
	}

	canvas.width = canvas.clientWidth || 300;
	canvas.height = canvas.clientHeight || 300;

	const program = createProgramFromScripts(gl, [
		{ type: 'vertex', src: vs },
		{ type: 'fragment', src: fs },
	]);
	gl.useProgram(program);

	// Full-screen quad (two triangles)
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

	// --- Build the texture array data ---
	const W = 64,
		H = 64,
		LAYERS = 6;

	// Each layer has a dominant hue + diagonal gradient
	const hues = [0, 60, 120, 180, 240, 300]; // red, yellow, green, cyan, blue, magenta

	function hsvToRgb(h: number, s: number, v: number): [number, number, number] {
		const c = v * s;
		const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
		const m = v - c;
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

	const data = new Uint8Array(W * H * LAYERS * 4);
	for (let layer = 0; layer < LAYERS; layer++) {
		const hue = hues[layer];
		for (let y = 0; y < H; y++) {
			for (let x = 0; x < W; x++) {
				const t = (x + y) / (W + H - 2); // 0..1 diagonal gradient
				const [r, g, b] = hsvToRgb(hue, 0.85, 0.2 + 0.8 * t);
				const idx = (layer * W * H + y * W + x) * 4;
				data[idx] = r;
				data[idx + 1] = g;
				data[idx + 2] = b;
				data[idx + 3] = 255;
			}
		}
	}

	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

	// Upload all 6 layers at once with a single texImage3D call
	gl.texImage3D(
		gl.TEXTURE_2D_ARRAY, // target
		0, // mip level
		gl.RGBA, // internal format
		W, // width
		H, // height
		LAYERS, // depth  (= number of array layers)
		0, // border
		gl.RGBA, // source format
		gl.UNSIGNED_BYTE, // source type
		data, // pixel data for every layer
	);

	gl.activeTexture(gl.TEXTURE0);
	gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
	gl.uniform1i(uTexture, 0);

	const startTime = Date.now();

	function render() {
		texImage3DLAF = requestAnimationFrame(render);

		const elapsed = (Date.now() - startTime) / 1000;
		// Cycle through layers, one new layer every 0.8 s
		const layer = Math.floor(elapsed / 0.8) % LAYERS;

		gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
		gl.clearColor(0.08, 0.08, 0.08, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);

		gl.useProgram(program);
		gl.activeTexture(gl.TEXTURE0);
		gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
		gl.uniform1i(uTexture, 0);
		gl.bindVertexArray(vao);
		gl.uniform1f(uLayer, layer);
		gl.drawArrays(gl.TRIANGLES, 0, 6);
	}

	render();
}

export function cancelTexImage3DDemo() {
	cancelAnimationFrame(texImage3DLAF);
	texImage3DLAF = 0;
}
