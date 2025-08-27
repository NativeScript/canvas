import type { GPU } from '@nativescript/canvas';

declare global {
	interface Navigator {
		readonly gpu: GPU;
	}
}

declare module '@nativescript/canvas' {
	export interface Canvas {
		toHTMLCanvas?(): HTMLCanvasElement;
	}
}
