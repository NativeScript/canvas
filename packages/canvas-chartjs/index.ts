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
	private chart?: Chart;
	private _layoutChangeListener?: () => void;
	acquireContext(canvas: HTMLCanvasElement, options?: CanvasRenderingContext2DSettings): CanvasRenderingContext2D | null {
		this._layoutChangeListener = () => {
			if (__APPLE__) {
				//	this.chart?.resize?.(canvas.clientWidth * Screen.mainScreen.scale, canvas.clientHeight * Screen.mainScreen.scale);
			}

			//	if (__ANDROID__) {
			this.chart?.resize?.(canvas.clientWidth, canvas.clientHeight);
			//	}
		};

		canvas.addEventListener('layoutChanged', this._layoutChangeListener as never);
		//canvas.style.width = `${canvas.clientWidth * Screen.mainScreen.scale}px`;
		//canvas.style.height = `${canvas.clientHeight * Screen.mainScreen.scale}px`;

		//@ts-ignore
		canvas.width = 600;
		//@ts-ignore
		canvas.height = 600;

		const ctx = canvas.getContext('2d', options);

		if (__ANDROID__) {
			//ctx?.setTransform(Screen.mainScreen.scale, 0, 0, Screen.mainScreen.scale, 0, 0);
		}

		return ctx;
	}

	releaseContext(context: CanvasRenderingContext2D): boolean {
		context.canvas.removeEventListener('layoutChanged', this._layoutChangeListener as never);
		return true;
	}

	getDevicePixelRatio(): number {
		return 1;
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
		const parent = (canvas as any).parent;

		// if (__APPLE__) {
		// 	return {
		// 		width: parent?.getMeasuredWidth(),
		// 		height: parent?.getMeasuredHeight(),
		// 	};
		// }

		return {
			width: canvas.clientWidth,
			height: canvas.clientHeight,
		};
	}
}
