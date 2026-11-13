import '@nativescript/canvas-polyfill';
import { Canvas } from '@nativescript/canvas';
import { knownFolders } from '@nativescript/core';
import { Adapter, Assets, CanvasSource, CanvasTextMetrics, DOMAdapter, EventSystem } from 'pixi.js';

const NSCAdapter: Adapter = {
	createCanvas(width?: number, height?: number) {
		const canvas = new Canvas();
		canvas.width = width;
		canvas.height = height;
		return canvas as never;
	},

	createImage() {
		return document.createElement('img');
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
		return document.fonts;
	},
	fetch(url: RequestInfo, options?: RequestInit) {
		return fetch(url, options);
	},
	parseXML(xml: string) {
		const parser = new DOMParser();
		return parser.parseFromString(xml, 'text/xml');
	},
};

DOMAdapter.set(NSCAdapter);

// Make `autoDensity`'s CSS-size half inert: NativeScript has no CSS layer, so
// `style.width = '<n>px'` means device pixels and Pixi ends up sizing the view by
// the device pixel ratio. Its backing-store scaling by `resolution` still applies.
const resizeCanvas = CanvasSource.prototype.resizeCanvas;
if (typeof resizeCanvas === 'function') {
	CanvasSource.prototype.resizeCanvas = function (this: CanvasSource) {
		if (!(this.resource instanceof Canvas)) {
			resizeCanvas.call(this);
			return;
		}
		const autoDensity = this.autoDensity;
		this.autoDensity = false;
		try {
			resizeCanvas.call(this);
		} finally {
			this.autoDensity = autoDensity;
		}
	};
}

Assets.setPreferences({ preferWorkers: false });

// PixiJS may have picked Intl.Segmenter before the polyfill hid it.
if (typeof (Intl as any).Segmenter !== 'function') {
	(CanvasTextMetrics as any).graphemeSegmenter = (text: string) => [...text];
}

// Pixi listens for pointermove/up on document and globalThis, which a native canvas never raises: listen on it too.
const events = EventSystem.prototype as any;
const addEvents = events._addEvents;
const removeEvents = events._removeEvents;

if (typeof addEvents === 'function' && typeof removeEvents === 'function' && !events.__nativeCanvasPointerBridge) {
	// Lets app-side shims of the same gap detect this one.
	events.__nativeCanvasPointerBridge = true;
	events._addEvents = function (this: any) {
		const added = this._eventsAdded;

		addEvents.call(this);
		if (!added && this.domElement instanceof Canvas && this.supportsPointerEvents) {
			this.domElement.addEventListener('pointermove', this._onPointerMove, true);
			this.domElement.addEventListener('pointerup', this._onPointerUp, true);
			this.domElement.addEventListener('pointercancel', this._onPointerUp, true);
		}
	};
	events._removeEvents = function (this: any) {
		if (this._eventsAdded && this.domElement instanceof Canvas && this.supportsPointerEvents) {
			this.domElement.removeEventListener('pointermove', this._onPointerMove, true);
			this.domElement.removeEventListener('pointerup', this._onPointerUp, true);
			this.domElement.removeEventListener('pointercancel', this._onPointerUp, true);
		}
		removeEvents.call(this);
	};
}
