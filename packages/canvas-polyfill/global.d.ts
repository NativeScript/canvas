import type { GPU } from '@nativescript/canvas';
declare global {
	interface Navigator {
		readonly gpu: GPU;
	}
}
