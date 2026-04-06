import { createProgramFromScripts } from '../webgl/utils';

let videoTex3DLAF = 0;

/**
 * Demonstrates texImage3D and texSubImage3D with a video source.
 *
 * A 2-layer TEXTURE_2D_ARRAY is created:
 *   Layer 0 – live video frames, pushed every video frame via texSubImage3D.
 *   Layer 1 – an animated checker pattern updated via texSubImage3D with
 *             typed-array data.
 *
 * The texture array is first allocated/initialised with texImage3D (once),
 * then updated every frame with texSubImage3D for both layers, showcasing
 * how the two calls work together.
 *
 * The visible layer alternates every second so both layers can be observed.
 */
export function videoTex3DDemo(canvas) {
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
		console.error('videoTex3DDemo: WebGL 2 not available');
		return;
	}

	canvas.width = canvas.clientWidth || 300;
	canvas.height = canvas.clientHeight || 300;

	gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);

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

	// Texture dimensions – match the video or fall back to a sensible default.
	// We start with a placeholder size; the first video frame will determine
	// the real dimensions and reallocate if needed.
	const TEX_W = 512,
		TEX_H = 512,
		LAYERS = 2;

	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

	// Allocate storage for both layers without initialising pixel data
	gl.texImage3D(gl.TEXTURE_2D_ARRAY, 0, gl.RGBA, TEX_W, TEX_H, LAYERS, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);

	gl.activeTexture(gl.TEXTURE0);
	gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);
	gl.uniform1i(uTexture, 0);

	// @ts-ignore – document is provided by canvas-polyfill
	const video = document.createElement('video');
	video.width = TEX_W;
	video.height = TEX_H;
	video.loop = true;
	video.muted = true;

	let copyVideo = false;

	video.addEventListener(
		'playing',
		function () {
			checkVideoReady();
		},
		true,
	);
	video.addEventListener(
		'timeupdate',
		function () {
			checkVideoReady();
		},
		true,
	);

	let playing = false,
		timeupdate = false;

	function checkVideoReady() {
		if (!playing) {
			playing = true;
		}
		if (!timeupdate) {
			timeupdate = true;
		}
		if (playing && timeupdate) {
			copyVideo = true;
		}
	}

	video.src = '~/assets/file-assets/webgl/apple.mp4';
	video.play();

	// Checker pattern buffer for layer 1 (updated each frame to animate)
	const checkerBuf = new Uint8Array(TEX_W * TEX_H * 4);

	const startTime = Date.now();

	function render() {
		videoTex3DLAF = requestAnimationFrame(render);

		const elapsed = (Date.now() - startTime) / 1000;

		gl.bindVertexArray(null);
		gl.bindTexture(gl.TEXTURE_2D_ARRAY, texture);

		// Layer 0 – video frame via texSubImage3D
		if (copyVideo) {
			gl.texSubImage3D(
				gl.TEXTURE_2D_ARRAY, // target
				0, // mip level
				0, // xoffset
				0, // yoffset
				0, // zoffset  (layer 0 = video)
				TEX_W, // width
				TEX_H, // height
				1, // depth (1 layer)
				gl.RGBA, // format
				gl.UNSIGNED_BYTE, // type
				video, // video element as pixel source
			);
		}

		// Layer 1 – animated checker via texSubImage3D with typed-array data
		const cellSize = 32;
		const t = Math.floor(elapsed * 2); // flip every half-second
		for (let y = 0; y < TEX_H; y++) {
			for (let x = 0; x < TEX_W; x++) {
				const cx = Math.floor(x / cellSize);
				const cy = Math.floor(y / cellSize);
				const bright = ((cx + cy + t) & 1) === 0;
				const pulse = Math.sin(elapsed * 4 + (cx + cy) * 0.5) * 0.15 + 0.85;
				const val = bright ? Math.round(220 * pulse) : Math.round(40 * pulse);
				const idx = (y * TEX_W + x) * 4;
				checkerBuf[idx] = val;
				checkerBuf[idx + 1] = Math.round(val * 0.5);
				checkerBuf[idx + 2] = Math.round(255 * (1 - pulse) * 0.4 + val * 0.1);
				checkerBuf[idx + 3] = 255;
			}
		}

		gl.texSubImage3D(
			gl.TEXTURE_2D_ARRAY, // target
			0, // mip level
			0, // xoffset
			0, // yoffset
			1, // zoffset  (layer 1 = checker)
			TEX_W, // width
			TEX_H, // height
			1, // depth (1 layer)
			gl.RGBA, // format
			gl.UNSIGNED_BYTE, // type
			checkerBuf, // typed-array pixel data
		);

		// Alternate displayed layer every second
		const currentLayer = Math.floor(elapsed) & 1;

		gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
		gl.clearColor(0.0, 0.0, 0.0, 1);
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

export function cancelVideoTex3DDemo() {
	cancelAnimationFrame(videoTex3DLAF);
	videoTex3DLAF = 0;
}
