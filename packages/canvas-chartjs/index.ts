import { BasePlatform, Chart, ChartEvent } from 'chart.js';
import { Screen } from '@nativescript/core';

// Intl polyfills for engines without (full) Intl. Their feature probes can throw where Intl has
// no locale data (e.g. V8 without ICU data rejects 'und-x-private'); a failed probe must not keep
// the plugin from loading.
for (const load of [() => require('@formatjs/intl-getcanonicallocales/polyfill'), () => require('@formatjs/intl-locale/polyfill'), () => require('@formatjs/intl-pluralrules/polyfill'), () => require('@formatjs/intl-numberformat/polyfill'), () => require('@formatjs/intl-pluralrules/locale-data/en'), () => require('@formatjs/intl-numberformat/locale-data/en')]) {
	try {
		load();
	} catch (e) {
		console.warn('@nativescript/canvas-chartjs: an Intl polyfill failed to load:', e?.message ?? e);
	}
}

import { registerables } from 'chart.js';
Chart.register(...registerables);

export class NativeScriptPlatform extends BasePlatform {
	private chart?: Chart;
	private _layoutChangeListener?: () => void;
	acquireContext(canvas: HTMLCanvasElement, options?: CanvasRenderingContext2DSettings): CanvasRenderingContext2D | null {
		this._layoutChangeListener = () => {
			// CSS pixels: `_resize` multiplies by getDevicePixelRatio() itself.
			this.chart?.resize?.(canvas.clientWidth, canvas.clientHeight);
		};

		canvas.addEventListener('layoutChanged', this._layoutChangeListener as never);
		//canvas.style.width = `${canvas.clientWidth * Screen.mainScreen.scale}px`;
		//canvas.style.height = `${canvas.clientHeight * Screen.mainScreen.scale}px`;

		const ctx = canvas.getContext('2d', options);

		if (__ANDROID__) {
		}

		//ctx?.setTransform(Screen.mainScreen.scale, 0, 0, Screen.mainScreen.scale, 0, 0);

		return ctx;
	}

	releaseContext(context: CanvasRenderingContext2D): boolean {
		context.canvas.removeEventListener('layoutChanged', this._layoutChangeListener as never);
		return true;
	}

	getDevicePixelRatio(): number {
		return Screen.mainScreen.scale;
	}

	addEventListener(chart: Chart, type: string, listener: (e: ChartEvent) => void): void {
		this.chart = chart;
		chart.canvas.addEventListener(type, listener as never);
	}

	removeEventListener(chart: Chart, type: string, listener: (e: ChartEvent) => void): void {
		this.chart = chart;
		chart.canvas.removeEventListener(type, listener as never);
	}

	isAttached(canvas: HTMLCanvasElement): boolean {
		return canvas.isConnected;
	}

	getMaximumSize(canvas: HTMLCanvasElement, width?: number, height?: number, aspectRatio?: number): { width: number; height: number } {
		// CSS pixels, not device pixels: Chart.js applies the ratio itself in
		// `retinaScale`, so device pixels here get it applied twice.
		return {
			width: canvas.clientWidth,
			height: canvas.clientHeight,
		};
	}
}
