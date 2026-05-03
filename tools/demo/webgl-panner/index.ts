const vertexShaderSrc = `#version 300 es
in vec2 a_position;
uniform vec2 u_resolution;
void main() {
  vec2 zeroToOne = a_position / u_resolution;
  vec2 zeroToTwo = zeroToOne * 2.0;
  vec2 clipSpace = zeroToTwo - 1.0;
  gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
}
`;

const fragmentShaderSrc = `#version 300 es
precision mediump float;
out vec4 outColor;
uniform vec3 u_color;
void main() {
  outColor = vec4(u_color, 1.0);
}
`;
import { AudioContext } from '@nativescript/audio-context';
import { StackLayout, Label, Slider, Button } from '@nativescript/core';

function createShader(gl: WebGL2RenderingContext, type: number, src: string) {
	const s = gl.createShader(type)!;
	gl.shaderSource(s, src);
	gl.compileShader(s);
	if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) {
		console.error(gl.getShaderInfoLog(s));
		gl.deleteShader(s);
		return null;
	}
	return s;
}

function createProgram(gl: WebGL2RenderingContext, vsSrc: string, fsSrc: string) {
	const vs = createShader(gl, gl.VERTEX_SHADER, vsSrc)!;
	const fs = createShader(gl, gl.FRAGMENT_SHADER, fsSrc)!;
	const p = gl.createProgram()!;
	gl.attachShader(p, vs);
	gl.attachShader(p, fs);
	gl.linkProgram(p);
	if (!gl.getProgramParameter(p, gl.LINK_STATUS)) {
		console.error(gl.getProgramInfoLog(p));
		gl.deleteProgram(p);
		return null;
	}
	return p;
}

export function initDemo(canvas: any) {
	const container = canvas.parent;
	const gl = canvas.getContext('webgl2');
	if (!gl) {
		return;
	}

	function resize() {
		const w = canvas.clientWidth;
		const h = canvas.clientHeight;
		canvas.width = w;
		canvas.height = h;
		gl.viewport(0, 0, w, h);
	}

	window.addEventListener('resize', resize);
	resize();

	const program = createProgram(gl, vertexShaderSrc, fragmentShaderSrc)!;
	gl.useProgram(program);

	const posLoc = gl.getAttribLocation(program, 'a_position');
	const resLoc = gl.getUniformLocation(program, 'u_resolution');
	const colorLoc = gl.getUniformLocation(program, 'u_color');

	const buf = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, buf);

	gl.enableVertexAttribArray(posLoc);
	gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 0, 0);

	// Scene objects in 2D (x,z) plane. units are arbitrary screen space.
	const listener = { x: 300, z: 200 };
	const speaker = { x: 300, z: 50, orientation: 0 }; // orientation in degrees (facing +z)
	const wall = { x: 50, z: 120, w: 440, h: 20 }; // horizontal wall across x
	const testPath = __ANDROID__ ? '~/assets/file-assets/audio/gs-16b-1c-44100hz.wav' : '~/assets/file-assets/audio/sine441stereo.mp3';
	let resolvedTestPath = testPath;
	try {
		const core = require('@nativescript/core');
		const appPath = core?.knownFolders?.currentApp?.().path;
		if (typeof appPath === 'string' && testPath.startsWith('~/')) {
			resolvedTestPath = testPath.replace('~/', `${appPath}/`);
		}
	} catch (e) {}

	// NativeScript audio-context setup (use repo bindings)

	const ctx = new AudioContext();
	let audioBuffer: any = null;
	let srcNode: any = null;
	let toneOsc: any = null;
	let mediaEl: any = null;
	let mediaSourceNode: any = null;
	let activeMode: 'decoded' | 'media' | 'tone' | null = null;
	let isPlaying = false;

	const panner = ctx.createPanner();
	// Equal-power gives a clearer left/right demo on built-in speakers.
	panner.panningModel = 'equalpower';
	panner.distanceModel = 'inverse';
	panner.refDistance = 1;
	panner.maxDistance = 10000;
	panner.rolloffFactor = 1;
	panner.coneInnerAngle = 180;
	panner.coneOuterAngle = 230;
	panner.coneOuterGain = 0.1;

	const lowpass = __ANDROID__ ? null : ctx.createBiquadFilter({ type: 'lowpass', frequency: 22050, Q: 0.707, gain: 0 });
	if (lowpass) {
		panner.connect(lowpass);
		lowpass.connect(ctx.destination);
	}

	const listenerNode: any = (ctx as any).listener;
	if (listenerNode) {
		const now = ctx.currentTime;
		listenerNode.positionX?.setValueAtTime?.(0, now);
		listenerNode.positionY?.setValueAtTime?.(0, now);
		listenerNode.positionZ?.setValueAtTime?.(0, now);
		listenerNode.forwardX?.setValueAtTime?.(0, now);
		listenerNode.forwardY?.setValueAtTime?.(0, now);
		listenerNode.forwardZ?.setValueAtTime?.(-1, now);
		listenerNode.upX?.setValueAtTime?.(0, now);
		listenerNode.upY?.setValueAtTime?.(1, now);
		listenerNode.upZ?.setValueAtTime?.(0, now);
	}

	async function loadBuffer() {
		if (audioBuffer) return audioBuffer;
		const timeoutMs = __ANDROID__ ? 3000 : 8000;
		try {
			const decoded = await Promise.race([ctx.decodeAudioData(resolvedTestPath), new Promise<null>((resolve) => setTimeout(() => resolve(null), timeoutMs))]);
			if (!decoded) {
				console.warn('decodeAudioData timed out; will try media playback fallback');
				return null;
			}
			audioBuffer = decoded;
			console.log('Audio buffer loaded:', audioBuffer);
			return audioBuffer;
		} catch (e) {
			console.warn('Failed to decode audio buffer; will try media playback fallback', e);
			return null;
		}
	}

	async function ensureMediaSource(): Promise<boolean> {
		try {
			if (!mediaEl) {
				mediaEl = new Audio();
			}
			if (typeof mediaEl.src === 'string') mediaEl.src = resolvedTestPath;
			mediaEl.loop = true;
			if (!mediaSourceNode) {
				mediaSourceNode = ctx.createMediaElementSource(mediaEl);
				mediaSourceNode.connect(panner);
			}
			if (typeof mediaEl.load === 'function') mediaEl.load();
			return true;
		} catch (e) {
			console.warn('Failed to create media element source', e);
			return false;
		}
	}

	async function playBuffer() {
		try {
			await ctx.resume();
		} catch (e) {
			console.warn('AudioContext resume failed', e);
		}

		if (srcNode) {
			try {
				srcNode.stop && srcNode.stop();
				srcNode.disconnect && srcNode.disconnect();
			} catch (e) {}
			srcNode = null;
		}

		if (toneOsc) {
			try {
				toneOsc.stop && toneOsc.stop();
				toneOsc.disconnect && toneOsc.disconnect();
			} catch (e) {}
			toneOsc = null;
		}

		if (audioBuffer || (await loadBuffer())) {
			srcNode = ctx.createBufferSource();
			srcNode.buffer = audioBuffer;
			srcNode.connect(panner);
			srcNode.loop = true;
			srcNode.start && srcNode.start();
			activeMode = 'decoded';
			isPlaying = true;
			return;
		}

		if (await ensureMediaSource()) {
			try {
				const p = mediaEl.play && mediaEl.play();
				if (p && typeof p.then === 'function') await p;
				activeMode = 'media';
				isPlaying = true;
				return;
			} catch (e) {
				console.warn('mediaEl.play() failed', e);
			}
		}

		// Fallback so panning is still testable even if decode fails.
		toneOsc = ctx.createOscillator({ type: 'sine', frequency: 220 });
		toneOsc.connect(panner);
		toneOsc.start && toneOsc.start();
		activeMode = 'tone';
		isPlaying = true;
	}

	function stopBuffer() {
		if (srcNode) {
			try {
				srcNode.stop && srcNode.stop();
				srcNode.disconnect && srcNode.disconnect();
			} catch (e) {}
			srcNode = null;
		}

		if (toneOsc) {
			try {
				toneOsc.stop && toneOsc.stop();
				toneOsc.disconnect && toneOsc.disconnect();
			} catch (e) {}
			toneOsc = null;
		}

		if (mediaEl) {
			try {
				mediaEl.pause && mediaEl.pause();
				if (typeof mediaEl.currentTime === 'number') mediaEl.currentTime = 0;
			} catch (e) {}
		}

		activeMode = null;
		isPlaying = false;
	}

	function updatePanner() {
		const sx = (speaker.x - listener.x) / 100.0;
		const sz = (speaker.z - listener.z) / 100.0;
		panner.setPosition(sx, 0, sz);

		const ang = (speaker.orientation * Math.PI) / 180.0;
		const ox = Math.sin(ang);
		const oz = Math.cos(ang);
		panner.setOrientation(ox, 0, oz);

		let blocked = false;

		const tx = (listener.x + speaker.x) / 2;
		const tz = (listener.z + speaker.z) / 2;
		if (Math.abs(tx - wall.x) < wall.w / 2 && Math.abs(tz - wall.z) < wall.h / 2) blocked = true;

		const vx = listener.x - speaker.x;
		const vz = listener.z - speaker.z;
		const vlen = Math.hypot(vx, vz) || 1;
		const ndx = vx / vlen;
		const ndz = vz / vlen;
		const dot = ndx * ox + ndz * oz;
		let angDeg = (Math.acos(Math.max(-1, Math.min(1, dot))) * 180) / Math.PI;
		let coneGain = 1.0;
		const inner = panner.coneInnerAngle;
		const outer = panner.coneOuterAngle;
		const outGain = panner.coneOuterGain;
		if (angDeg <= inner) coneGain = 1.0;
		else if (angDeg >= outer) coneGain = outGain;
		else {
			const t = (angDeg - inner) / (outer - inner);
			coneGain = 1.0 - t + t * outGain;
		}

		const distance = vlen / 100.0; // meters
		let distanceAtt = 1.0 / (1.0 + distance);
		if (distanceAtt < 0) distanceAtt = 0;
		const combinedGain = coneGain * distanceAtt * (blocked ? 0.5 : 1.0);

		const lowFreq = 800.0;
		const highFreq = 22050.0;
		let g = combinedGain;
		if (g < 0) g = 0;
		if (g > 1) g = 1;
		if (lowpass) {
			const cutoff = lowFreq + (highFreq - lowFreq) * Math.pow(g, 0.5);
			lowpass.frequency.setTargetAtTime(cutoff, ctx.currentTime, 0.01);
		}

		requestAnimationFrame(render);
	}

	function render() {
		gl.clearColor(0.9, 0.9, 0.9, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);

		gl.uniform2f(resLoc, canvas.width, canvas.height);

		// draw wall
		drawRect(gl, program, buf, wall.x - wall.w / 2, wall.z - wall.h / 2, wall.w, wall.h, [1, 0, 0]);
		// draw speaker
		drawRect(gl, program, buf, speaker.x - 10, speaker.z - 10, 20, 20, [0, 0, 1]);
		// draw listener
		drawRect(gl, program, buf, listener.x - 8, listener.z - 8, 16, 16, [0, 1, 0]);

		updatePanner();
	}

	function drawRect(gl: WebGL2RenderingContext, program: WebGLProgram, buf: WebGLBuffer | null, x: number, y: number, w: number, h: number, color: [number, number, number]) {
		const x1 = x;
		const y1 = y;
		const x2 = x + w;
		const y2 = y + h;
		const verts = new Float32Array([x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]);
		gl.bindBuffer(gl.ARRAY_BUFFER, buf);
		gl.bufferData(gl.ARRAY_BUFFER, verts, gl.STREAM_DRAW);
		gl.uniform3fv(gl.getUniformLocation(program, 'u_color'), color);
		gl.drawArrays(gl.TRIANGLES, 0, 6);
	}

	const controls = new StackLayout();
	controls.marginTop = 8;

	const lxLabel = new Label();
	lxLabel.text = 'Speaker X';
	const sxSlider = new Slider();
	sxSlider.minValue = 50;
	sxSlider.maxValue = 550;
	sxSlider.value = speaker.x;
	sxSlider.on('valueChange', (args: any) => {
		speaker.x = Math.round((args.object as any).value);
	});

	const lzLabel = new Label();
	lzLabel.text = 'Speaker Z';
	const szSlider = new Slider();
	szSlider.minValue = 10;
	szSlider.maxValue = 350;
	szSlider.value = speaker.z;
	szSlider.on('valueChange', (args: any) => {
		speaker.z = Math.round((args.object as any).value);
	});

	const playBtn = new Button();
	playBtn.text = 'Play/Pause';
	playBtn.on('tap', async () => {
		if (!isPlaying) await playBuffer();
		else stopBuffer();
		console.log('webgl-panner mode:', activeMode);
	});

	controls.addChild(lxLabel);
	controls.addChild(sxSlider);
	controls.addChild(lzLabel);
	controls.addChild(szSlider);
	controls.addChild(playBtn);

	(container as any).addChild(controls);

	render();
}
