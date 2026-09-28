import { DemoSharedBase } from '../utils';
import '@nativescript/canvas-polyfill';

const LABELS = ['A', 'B', 'X', 'Y', 'LB', 'RB', 'LT', 'RT', 'Sel', 'Start', 'L3', 'R3', 'Up', 'Down', 'Left', 'Right', 'Home'];

export class DemoSharedCanvasGamepad extends DemoSharedBase {
	private _raf = 0;

	canvasLoaded(args) {
		const canvas = args.object;
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

		window.addEventListener('gamepadconnected', (e: any) => console.log(`GAMEPAD|connected ${e.gamepad.index} ${e.gamepad.id}`));
		window.addEventListener('gamepaddisconnected', (e: any) => console.log(`GAMEPAD|disconnected ${e.gamepad.index}`));

		this.bench();

		const draw = () => {
			const width = canvas.width;
			const height = canvas.height;
			const scale = width / 400;
			ctx.fillStyle = '#111';
			ctx.fillRect(0, 0, width, height);
			ctx.font = `${14 * scale}px sans-serif`;

			const pads = navigator.getGamepads().filter(Boolean);
			if (pads.length === 0) {
				ctx.fillStyle = '#aaa';
				ctx.fillText('Connect a controller and press a button', 16 * scale, 32 * scale);
			}
			let y = 16 * scale;
			for (const pad of pads) {
				ctx.fillStyle = '#fff';
				ctx.fillText(`${pad.index}: ${pad.id}`, 16 * scale, (y += 18 * scale));
				ctx.fillText(`mapping=${pad.mapping} t=${pad.timestamp.toFixed(1)}`, 16 * scale, (y += 18 * scale));
				y += 12 * scale;
				for (let i = 0; i < pad.axes.length; i += 2) {
					const cx = (60 + i * 60) * scale;
					const cy = y + 40 * scale;
					ctx.strokeStyle = '#666';
					ctx.beginPath();
					ctx.arc(cx, cy, 36 * scale, 0, Math.PI * 2);
					ctx.stroke();
					ctx.fillStyle = '#4af';
					ctx.beginPath();
					ctx.arc(cx + pad.axes[i] * 30 * scale, cy + (pad.axes[i + 1] ?? 0) * 30 * scale, 8 * scale, 0, Math.PI * 2);
					ctx.fill();
				}
				y += 92 * scale;
				pad.buttons.forEach((button, i) => {
					const col = i % 6;
					const row = Math.floor(i / 6);
					const bx = (16 + col * 62) * scale;
					const by = y + row * 36 * scale;
					ctx.fillStyle = button.pressed ? '#4c4' : '#333';
					ctx.fillRect(bx, by, 56 * scale, 30 * scale);
					ctx.fillStyle = '#6a6';
					ctx.fillRect(bx, by + 26 * scale, 56 * scale * button.value, 4 * scale);
					ctx.fillStyle = '#fff';
					ctx.fillText(LABELS[i] ?? String(i), bx + 6 * scale, by + 20 * scale);
				});
				y += 120 * scale;
			}
			this._raf = requestAnimationFrame(draw);
		};
		this._raf = requestAnimationFrame(draw);
	}

	unloaded() {
		cancelAnimationFrame(this._raf);
	}

	private bench() {
		let sink = 0;
		const frame = () => {
			const pads = navigator.getGamepads();
			for (let p = 0; p < pads.length; p++) {
				const pad = pads[p];
				if (!pad) continue;
				for (let a = 0; a < pad.axes.length; a++) sink += pad.axes[a];
				for (let b = 0; b < pad.buttons.length; b++) sink += pad.buttons[b].value;
			}
		};
		for (let i = 0; i < 1000; i++) frame();
		const n = 100000;
		const start = performance.now();
		for (let i = 0; i < n; i++) frame();
		const ns = ((performance.now() - start) * 1e6) / n;
		const connected = navigator.getGamepads().filter(Boolean).length;
		console.log(`GAMEPAD|bench ${ns.toFixed(0)} ns/frame pads=${connected} (${sink === -1 ? '' : 'ok'})`);
	}
}
