import { Application, Button, GridLayout, ItemSpec, Label } from '@nativescript/core';
import { Canvas } from '@nativescript/canvas';
import { AudioContext } from '@nativescript/audio-context';

function canvasIn(grid: GridLayout, row: number, col: number, onReady: (canvas: Canvas) => void) {
	const canvas = new Canvas();
	canvas.style.width = '100%' as any;
	canvas.style.height = '100%' as any;
	GridLayout.setRow(canvas, row);
	GridLayout.setColumn(canvas, col);
	grid.addChild(canvas);
	// Usable right away; the drawing buffer follows the view's size once laid out.
	onReady(canvas);
	return canvas;
}

/** A gradient, shapes, text and an orbiting dot (requestAnimationFrame). */
function draw2D(canvas: Canvas) {
	const ctx = canvas.getContext('2d') as any;
	let logged = '';
	const frame = (time: number) => {
		const { width, height } = canvas;
		if (logged !== `${width}x${height}`) {
			logged = `${width}x${height}`;
			console.log(`2d: ${logged}`);
		}
		const gradient = ctx.createLinearGradient(0, 0, width, height);
		gradient.addColorStop(0, '#1e3c72');
		gradient.addColorStop(1, '#2a5298');
		ctx.fillStyle = gradient;
		ctx.fillRect(0, 0, width, height);

		ctx.fillStyle = 'rgba(255, 200, 0, 0.9)';
		ctx.beginPath();
		ctx.arc(width * 0.3, height * 0.5, Math.min(width, height) * 0.25, 0, Math.PI * 2);
		ctx.fill();

		ctx.strokeStyle = '#ff4f6d';
		ctx.lineWidth = 8;
		ctx.strokeRect(width * 0.55, height * 0.25, width * 0.3, height * 0.5);

		const t = time / 1000;
		ctx.fillStyle = 'white';
		ctx.beginPath();
		ctx.arc(width * 0.3 + Math.cos(t * 2) * width * 0.15, height * 0.5 + Math.sin(t * 2) * height * 0.3, 10, 0, Math.PI * 2);
		ctx.fill();

		ctx.font = `bold ${Math.round(width * 0.07)}px sans-serif`;
		ctx.fillStyle = 'white';
		ctx.fillText('Canvas 2D on GTK 4', 16, height * 0.15);
		requestAnimationFrame(frame);
	};
	requestAnimationFrame(frame);
}

/** A clear colour and an RGB triangle. */
function drawWebGL(canvas: Canvas) {
	const gl = canvas.getContext('webgl2') as WebGL2RenderingContext;
	if (!gl) {
		console.log('webgl2: unavailable');
		return;
	}
	console.log(`webgl2: ${gl.getParameter(gl.VERSION)} / ${gl.getParameter(gl.RENDERER)}`);
	const shader = (type: number, source: string) => {
		const s = gl.createShader(type);
		gl.shaderSource(s, source);
		gl.compileShader(s);
		if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) {
			console.log('shader:', gl.getShaderInfoLog(s));
		}
		return s;
	};
	const program = gl.createProgram();
	gl.attachShader(
		program,
		shader(
			gl.VERTEX_SHADER,
			`#version 300 es
			in vec2 position; in vec3 color; out vec3 vColor;
			void main() { vColor = color; gl_Position = vec4(position, 0.0, 1.0); }`,
		),
	);
	gl.attachShader(
		program,
		shader(
			gl.FRAGMENT_SHADER,
			`#version 300 es
			precision mediump float; in vec3 vColor; out vec4 outColor;
			void main() { outColor = vec4(vColor, 1.0); }`,
		),
	);
	gl.linkProgram(program);
	gl.useProgram(program);
	const buffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0.8, 1, 0, 0, -0.8, -0.8, 0, 1, 0, 0.8, -0.8, 0, 0, 1]), gl.STATIC_DRAW);
	const position = gl.getAttribLocation(program, 'position');
	const color = gl.getAttribLocation(program, 'color');
	gl.enableVertexAttribArray(position);
	gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 20, 0);
	gl.enableVertexAttribArray(color);
	gl.vertexAttribPointer(color, 3, gl.FLOAT, false, 20, 8);
	const frame = (time: number) => {
		gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
		gl.clearColor(0.1, 0.1, 0.12 + 0.08 * Math.sin(time / 500), 1);
		gl.clear(gl.COLOR_BUFFER_BIT);
		gl.drawArrays(gl.TRIANGLES, 0, 3);
		requestAnimationFrame(frame);
	};
	requestAnimationFrame(frame);
}

let audio: AudioContext | null = null;

/** A 440 Hz tone, faded in and out over a second. */
function playTone(status: Label) {
	audio ??= new AudioContext();
	const now = audio.currentTime;
	const oscillator = audio.createOscillator();
	const gain = audio.createGain();
	oscillator.type = 'sine';
	oscillator.frequency.setValueAtTime(440, now);
	gain.gain.setValueAtTime(0, now);
	gain.gain.linearRampToValueAtTime(0.3, now + 0.05);
	gain.gain.linearRampToValueAtTime(0, now + 1);
	oscillator.connect(gain);
	gain.connect(audio.destination);
	oscillator.start(now);
	oscillator.stop(now + 1);
	status.text = `AudioContext: ${audio.state}, ${audio.sampleRate} Hz`;
	console.log(`audio: ${audio.state} ${audio.sampleRate} Hz, base latency ${audio.baseLatency}`);
}

Application.run({
	create: () => {
		const grid = new GridLayout();
		grid.addRow(new ItemSpec(1, 'star'));
		grid.addRow(new ItemSpec(1, 'auto'));
		grid.addColumn(new ItemSpec(1, 'star'));
		grid.addColumn(new ItemSpec(1, 'star'));
		canvasIn(grid, 0, 0, draw2D);
		canvasIn(grid, 0, 1, drawWebGL);

		const status = new Label();
		status.text = 'AudioContext: not started';
		status.margin = 12;
		GridLayout.setRow(status, 1);
		grid.addChild(status);

		const play = new Button();
		play.text = 'Play 440 Hz';
		play.margin = 8;
		play.on('tap', () => playTone(status));
		GridLayout.setRow(play, 1);
		GridLayout.setColumn(play, 1);
		grid.addChild(play);

		// Once on start, so a run without input still plays.
		setTimeout(() => playTone(status), 1500);
		return grid;
	},
});
