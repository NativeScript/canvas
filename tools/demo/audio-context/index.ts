import { DemoSharedBase } from '../utils';
import { AudioContext } from '@nativescript/audio-context';

export class DemoSharedAudioContext extends DemoSharedBase {
	ctx: AudioContext | null = null;
	source: any = null;
	gainNode: any = null;
	analyser: any = null;
	visualizerCanvas: any = null;
	visualizerCtx: any = null;
	rafId: number | null = null;
	dpr: number = 1;
	isPlaying = false;
	_visualizerLogged: boolean = false;

	async initAudio() {
		if (this.ctx) return;
		this.ctx = __ANDROID__ ? new AudioContext({ latencyHint: 'playback' }) : new AudioContext();
		this.gainNode = this.ctx.createGain({ gain: 0.8 });
		this.analyser = this.ctx.createAnalyser();
		this.analyser.fftSize = 2048;
		// route: gain -> analyser -> destination
		this.gainNode.connect(this.analyser);
		this.analyser.connect(this.ctx.destination);
	}

	async playUrl(url: string) {
		try {
			await this.initAudio();
		} catch (error) {
			console.log('playUrl: failed to initialize audio', error);
			return;
		}
		if (this.source) this.stop();
		try {
			const buffer = await this.ctx!.decodeAudioData(url);
			const src = this.ctx!.createBufferSource();
			src.buffer = buffer;
			src.connect(this.gainNode);

			src.loop = false;
			// start safely (resume/retry on native platform errors)
			const ok = await this._attachAndStartSource(src);
			if (!ok) {
				console.log('playUrl: failed to start source');
				return;
			}
		} catch (e) {
			console.log('playUrl failed', e);
		}
	}

	// play a short bundled sine sample generated at runtime (no external file)
	async playBundledSample(duration = 1.5, freq = 440) {
		await this.initAudio();
		if (this.source) this.stop();
		try {
			const sampleRate = this.ctx!.sampleRate || 44100;
			const length = Math.floor(sampleRate * duration);
			const buffer = this.ctx!.createBuffer({ length, numberOfChannels: 1, sampleRate });
			const data = buffer.getChannelData(0) as Float32Array;
			for (let i = 0; i < length; i++) {
				const t = i / sampleRate;
				// simple sine with a small decay
				data[i] = Math.sin(2 * Math.PI * freq * t) * 0.6 * Math.exp((-3 * t) / duration);
			}
			const src = this.ctx!.createBufferSource();
			src.buffer = buffer;
			src.connect(this.gainNode);
			const ok = await this._attachAndStartSource(src);
			if (!ok) {
				console.log('playBundledSample: failed to start source');
				return;
			}
		} catch (e) {
			console.log('playBundledSample failed', e);
		}
	}

	stop() {
		if (!this.source) return;
		try {
			if (this.source.stop) this.source.stop();
			this.source.disconnect && this.source.disconnect();
		} catch (e) {}
		this.source = null;
		this.isPlaying = false;
		this.stopVisualizer();
	}

	setVolume(v: number) {
		if (!this.gainNode || !this.ctx) return;
		const now = this.ctx.currentTime;
		this.gainNode.gain.setValueAtTime(v, now);
	}

	initVisualizer(canvas: any) {
		if (!canvas) return;
		this.visualizerCanvas = canvas;
		this.visualizerCtx = canvas.getContext('2d');
		const dpr = typeof (globalThis as any).devicePixelRatio === 'number' ? (globalThis as any).devicePixelRatio : 1;
		this.dpr = dpr;

		canvas.width = canvas.clientWidth * dpr;
		canvas.height = canvas.clientHeight * dpr;

		try {
			if (this.visualizerCtx && this.visualizerCtx.scale) this.visualizerCtx.scale(dpr, dpr);
		} catch (e) {}
		if (this.analyser && !this.rafId) this.startVisualizer();
	}

	startVisualizer() {
		console.log('startVisualizer: starting visualizer');
		if (!this.analyser || !this.visualizerCtx || !this.visualizerCanvas) return;
		const canvas = this.visualizerCanvas;
		const ctx2d = this.visualizerCtx;
		const bufferLength = this.analyser.frequencyBinCount;
		const dataArray = new Uint8Array(bufferLength);
		const raf = requestAnimationFrame;

		const width = canvas.clientWidth;
		const height = canvas.clientHeight;
		const draw = () => {
			this.analyser.getByteFrequencyData(dataArray);
			if (!this._visualizerLogged) {
				let max = 0;
				for (let k = 0; k < dataArray.length; k++) if (dataArray[k] > max) max = dataArray[k];
				this._visualizerLogged = true;
			}

			ctx2d.fillStyle = '#071026';
			ctx2d.fillRect(0, 0, width, height);

			const barCount = 64;
			const step = Math.max(1, Math.floor(bufferLength / barCount));
			const barWidth = (width / barCount) * 0.9;
			let x = 0;
			for (let i = 0; i < barCount; i++) {
				const v = dataArray[i * step] / 255;
				const barHeight = v * height;
				const r = Math.round(200 * v + 55);
				const g = Math.round(120 * (1 - v) + 80);
				ctx2d.fillStyle = `rgb(${r},${g},120)`;
				ctx2d.fillRect(x, height - barHeight, barWidth, barHeight);
				x += barWidth + 2;
			}

			this.rafId = raf(draw);
		};

		if (!this.rafId) draw();
	}

	stopVisualizer() {
		if (this.rafId) {
			globalThis.cancelAnimationFrame(this.rafId);
		}
		this.rafId = null;
		if (this.visualizerCtx && this.visualizerCanvas) {
			try {
				this.visualizerCtx.clearRect(0, 0, this.visualizerCanvas.width, this.visualizerCanvas.height);
			} catch (e) {}
		}
	}

	async startSourceSafe(src: any) {
		if (!this.ctx) return false;
		if (this.ctx && typeof this.ctx.resume === 'function') {
			try {
				await this.ctx.resume();
			} catch (e) {
				console.log('startSourceSafe: ctx.resume failed', e);
			}
		}
		for (let attempt = 0; attempt < 2; attempt++) {
			try {
				if (src.start) src.start();
				return true;
			} catch (err) {
				console.log('startSourceSafe: start failed attempt', attempt + 1, err);
				// try reconnect and resume then retry
				try {
					src.disconnect && src.disconnect();
					if (this.gainNode) src.connect(this.gainNode);
					else if (this.analyser) src.connect(this.analyser);
					else if (this.ctx && this.ctx.destination) src.connect(this.ctx.destination);
				} catch (e) {
					console.log('startSourceSafe: reconnect failed', e);
				}
				try {
					if (this.ctx && typeof this.ctx.resume === 'function') await this.ctx.resume();
				} catch (e) {
					console.log('startSourceSafe: ctx.resume failed', e);
				}
				// small delay before retry
				await new Promise((r) => setTimeout(r, 60));
			}
		}
		return false;
	}

	private async _attachAndStartSource(src: any) {
		const ok = await this.startSourceSafe(src);
		if (!ok) return false;
		this.source = src;
		this.isPlaying = true;
		(src as any).onended = () => {
			this.source = null;
			this.isPlaying = false;
			this.stopVisualizer();
		};
		if (this.analyser && this.visualizerCanvas && !this.rafId) this.startVisualizer();
		return true;
	}

	async testIt() {
		console.log('audio-context demo (with visualizer)');
		try {
			await this.initAudio();
		} catch (err) {
			console.log('audio-context test failed', err);
		}
	}
}
