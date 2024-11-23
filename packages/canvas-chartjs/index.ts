import { BasePlatform, Chart, ChartEvent } from 'chart.js';
import { Screen } from '@nativescript/core';

import '@formatjs/intl-getcanonicallocales/polyfill';
import '@formatjs/intl-locale/polyfill';
import '@formatjs/intl-pluralrules/polyfill';
import '@formatjs/intl-numberformat/polyfill';

import '@formatjs/intl-pluralrules/locale-data/en';
import '@formatjs/intl-numberformat/locale-data/en';

import { registerables } from 'chart.js';

Chart.register(...registerables);

export class NativeScriptPlatform extends BasePlatform {
	acquireContext(canvas: HTMLCanvasElement, options?: CanvasRenderingContext2DSettings): CanvasRenderingContext2D | null {
		return canvas.getContext('2d', options);
	}

	getDevicePixelRatio(): number {
		return 1;
	}

	addEventListener(chart: Chart, type: string, listener: (e: ChartEvent) => void): void {
		chart.canvas.addEventListener(type, listener as never);
	}

	removeEventListener(chart: Chart, type: string, listener: (e: ChartEvent) => void): void {
		chart.canvas.removeEventListener(type, listener as never);
	}

	isAttached(canvas: HTMLCanvasElement): boolean {
		return canvas.isConnected;
	}

	getMaximumSize(canvas: HTMLCanvasElement, width?: number, height?: number, aspectRatio?: number): { width: number; height: number } {
		const parent = (canvas as any).parent;
		width = Math.max(0, width ?? parent?.clientWidth ?? canvas.clientWidth);
		height = height || (parent?.clientHeight ?? canvas.clientHeight);
		return {
			width,
			height: Math.max(0, aspectRatio ? Math.floor(width / aspectRatio) : height),
		};
	}
}
