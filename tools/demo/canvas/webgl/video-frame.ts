let raf = 0;

// Shared video creation used by both demos
function createDemoVideo() {
	console.log('Creating demo video element');
	// @ts-ignore
	const video: any = document.createElement('video');
	video.crossOrigin = 'anonymous';
	video.loop = true;
	video.muted = true;
	video.playsInline = true;
	video.autoplay = false;
	const local = '~/assets/file-assets/webgl/apple.mp4';
	const fallback = 'https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4';
	video.src = local;
	video.addEventListener('error', () => {
		video.src = fallback;
		video.load();
	});

	video.load();

	video.play();

	return video;
}

export function videoFrame2DDemo(canvas: any) {
	const ctx = canvas.getContext('2d');
	const dpr = (window && window.devicePixelRatio) || 1;
	const video = createDemoVideo();

	function resizeTo(w: number, h: number) {
		canvas.width = Math.round(w * dpr);
		canvas.height = Math.round(h * dpr);
		ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
	}

	function handleFrameWithVideoFrame(frame: any) {
		try {
			const width = frame.codedWidth || Math.round(canvas.width / dpr);
			const height = frame.codedHeight || Math.round(canvas.height / dpr);
			if (canvas.width !== Math.round(width * dpr) || canvas.height !== Math.round(height * dpr)) {
				resizeTo(width, height);
			}

			const pixels: Uint8Array | null = frame.pixelData;
			if (!pixels) {
				frame.close();
				return;
			}

			// @ts-ignore
			const img = new ImageData(new Uint8ClampedArray(pixels.buffer, pixels.byteOffset, width * height * 4), width, height);
			ctx.putImageData(img, 0, 0);
			frame.close();
		} catch (err) {
			console.warn('VideoFrame handling error', err);
		}
	}

	function fallbackDraw() {
		const w = canvas.clientWidth || 320;
		const h = canvas.clientHeight || Math.round((w / 16) * 9);
		if (canvas.width !== Math.round(w * dpr) || canvas.height !== Math.round(h * dpr)) {
			resizeTo(w, h);
		}
		try {
			ctx.clearRect(0, 0, canvas.width / dpr, canvas.height / dpr);
			ctx.drawImage(video, 0, 0, canvas.width / dpr, canvas.height / dpr);
		} catch (e) {
			// ignore
		}
	}

	if (video.requestVideoFrameCallback) {
		const cb = async () => {
			try {
				const frame = typeof video.captureFrame === 'function' ? video.captureFrame() : null;
				if (frame) {
					handleFrameWithVideoFrame(frame);
				} else {
					fallbackDraw();
				}
			} catch (err) {
				console.warn('requestVideoFrameCallback handler error', err);
			} finally {
				try {
					video.requestVideoFrameCallback(cb);
				} catch {}
			}
		};
		video.requestVideoFrameCallback(cb);
	} else {
		let rafId = 0;
		function loop() {
			rafId = requestAnimationFrame(loop);
			if (typeof video.captureFrame === 'function') {
				const frame = video.captureFrame();
				if (frame) {
					handleFrameWithVideoFrame(frame);
					return;
				}
			}
			if (video.readyState >= 2) fallbackDraw();
		}
		video.addEventListener('playing', () => {
			if (!rafId) loop();
		});
		video.addEventListener('loadeddata', () => {
			if (!rafId) loop();
		});
		(canvas as any)._videoFrameRaf2d = () => cancelAnimationFrame(rafId);
	}

	(canvas as any)._videoFrameDemo2d = {
		video,
		stop: () => {
			try {
				if ((canvas as any)._videoFrameRaf2d) (canvas as any)._videoFrameRaf2d();
			} catch {}
			try {
				video.pause();
			} catch {}
		},
	};
}

export function cancelVideoFrame2DDemo(canvas: any) {
	const refs = (canvas as any)?._videoFrameDemo2d;
	if (refs) {
		try {
			refs.stop();
		} catch (e) {
			console.warn('cancelVideoFrame2DDemo error', e);
		}
		delete (canvas as any)._videoFrameDemo2d;
	}
}

export function videoFrameWebGLDemo(canvas: any) {
	const dpr = (window && window.devicePixelRatio) || 1;
	const video = createDemoVideo();
	const gl = (canvas.getContext ? canvas.getContext('webgl') : canvas) as WebGLRenderingContext | null;
	if (!gl) {
		console.error('WebGL not available');
		return;
	}

	function resizeTo(w: number, h: number) {
		canvas.width = Math.round(w * dpr);
		canvas.height = Math.round(h * dpr);
		gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
	}

	const vs = 'attribute vec2 a_pos; attribute vec2 a_uv; varying vec2 v_uv; void main(){ v_uv = a_uv; gl_Position = vec4(a_pos,0.0,1.0); }';
	const fs = 'precision mediump float; varying vec2 v_uv; uniform sampler2D u_tex; void main(){ gl_FragColor = texture2D(u_tex, v_uv); }';

	function compile(type: number, src: string) {
		const s = gl.createShader(type)!;
		gl.shaderSource(s, src);
		gl.compileShader(s);
		if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(s) || '');
		return s;
	}

	const prog = gl.createProgram()!;
	gl.attachShader(prog, compile(gl.VERTEX_SHADER, vs));
	gl.attachShader(prog, compile(gl.FRAGMENT_SHADER, fs));
	gl.linkProgram(prog);
	if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(prog) || '');
	gl.useProgram(prog);

	const posLoc = gl.getAttribLocation(prog, 'a_pos');
	const uvLoc = gl.getAttribLocation(prog, 'a_uv');
	const texLoc = gl.getUniformLocation(prog, 'u_tex');

	const quad = new Float32Array([-1, -1, 0, 1, 1, -1, 1, 1, -1, 1, 0, 0, 1, 1, 1, 0]);
	const buf = gl.createBuffer()!;
	gl.bindBuffer(gl.ARRAY_BUFFER, buf);
	gl.bufferData(gl.ARRAY_BUFFER, quad, gl.STATIC_DRAW);
	gl.enableVertexAttribArray(posLoc);
	gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 16, 0);
	gl.enableVertexAttribArray(uvLoc);
	gl.vertexAttribPointer(uvLoc, 2, gl.FLOAT, false, 16, 8);

	const tex = gl.createTexture()!;
	gl.bindTexture(gl.TEXTURE_2D, tex);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

	let frameBuf: Uint8Array | null = null;

	function handleVideoFrame(frame: any) {
		try {
			const width = frame.codedWidth || canvas.clientWidth;
			const height = frame.codedHeight || canvas.clientHeight;
			if (canvas.width !== Math.round(width * dpr) || canvas.height !== Math.round(height * dpr)) resizeTo(width, height);

			// Use the frame's pixel data directly — avoids the copyTo() intermediate
			// buffer and eliminates one full-frame copy on every tick.
			const pixels: Uint8Array | null = frame.pixelData;
			if (!pixels) {
				frame.close();
				return;
			}

			gl.bindTexture(gl.TEXTURE_2D, tex);
			gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			gl.clearColor(0, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			gl.uniform1i(texLoc, 0);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
			frame.close();
		} catch (e) {
			console.warn('handleVideoFrame error', e);
		}
	}

	function fallbackUpload() {
		if (video.readyState < 2) return;
		const width = video.videoWidth || canvas.clientWidth || 320;
		const height = video.videoHeight || canvas.clientHeight || Math.round((width / 16) * 9);
		if (canvas.width !== Math.round(width * dpr) || canvas.height !== Math.round(height * dpr)) resizeTo(width, height);
		gl.bindTexture(gl.TEXTURE_2D, tex);
		try {
			gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, video);
		} catch (e) {
			// some environments may not support video -> texImage2D
		}
		gl.clearColor(0, 0, 0, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);
		gl.uniform1i(texLoc, 0);
		gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
	}

	if (video.requestVideoFrameCallback) {
		const cb = async () => {
			try {
				const frame = typeof video.captureFrame === 'function' ? video.captureFrame() : null;
				if (frame) {
					await handleVideoFrame(frame);
				} else {
					fallbackUpload();
				}
			} catch (e) {
				console.warn('video frame cb error', e);
			} finally {
				try {
					video.requestVideoFrameCallback(cb);
				} catch {}
			}
		};
		video.requestVideoFrameCallback(cb);
	} else {
		let rafId = 0;
		function loop() {
			rafId = requestAnimationFrame(loop);
			if (typeof video.captureFrame === 'function') {
				const frame = video.captureFrame();
				if (frame) {
					handleVideoFrame(frame);
					return;
				}
			}
			fallbackUpload();
		}
		video.addEventListener('playing', () => {
			if (!rafId) loop();
		});
		video.addEventListener('loadeddata', () => {
			if (!rafId) loop();
		});
		(canvas as any)._videoFrameRafGL = () => cancelAnimationFrame(rafId);
	}

	(canvas as any)._videoFrameDemoGL = {
		video,
		stop: () => {
			try {
				if ((canvas as any)._videoFrameRafGL) (canvas as any)._videoFrameRafGL();
			} catch {}
			try {
				video.pause();
			} catch {}
		},
	};
}

export function cancelVideoFrameWebGLDemo(canvas: any) {
	const refs = (canvas as any)?._videoFrameDemoGL;
	if (refs) {
		try {
			refs.stop();
		} catch (e) {
			console.warn('cancelVideoFrameWebGLDemo error', e);
		}
		delete (canvas as any)._videoFrameDemoGL;
	}
}

// Backwards-compatible default
export const videoFrameDemo = videoFrame2DDemo;
