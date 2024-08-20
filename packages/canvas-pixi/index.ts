require('@nativescript/canvas-polyfill');
import { Canvas } from '@nativescript/canvas';
import { knownFolders } from '@nativescript/core';
import * as Pixii from 'pixi.js';

const NSCAdapter: Pixii.Adapter = {
	createCanvas(width?: number, height?: number) {
		const canvas = new Canvas();
		canvas.width = width;
		canvas.height = height;
		return canvas as never;
	},
	getCanvasRenderingContext2D() {
		return CanvasRenderingContext2D as never;
	},
	getWebGLRenderingContext() {
		return WebGLRenderingContext as never;
	},
	getNavigator() {
		return {
			userAgent: '',
			gpu: global.navigator.gpu,
		};
	},
	getBaseUrl() {
		return knownFolders.currentApp().path;
	},
	getFontFaceSet() {
		// todo FontFaceSet
		return null;
	},
	fetch(url: RequestInfo, options?: RequestInit) {
		return fetch(url, options);
	},
	parseXML(xml: string) {
		const parser = new DOMParser();
		return parser.parseFromString(xml, 'text/xml');
	},
};

Pixii.DOMAdapter.set(NSCAdapter);

let PIXI = Pixii;

PIXI.Assets.setPreferences({ preferWorkers: false });

(global as any).PIXI = (global as any).window.PIXI = (global as any).PIXI || PIXI;

export default PIXI;
