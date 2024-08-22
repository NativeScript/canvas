import type { GPU } from './WebGPU';
declare global {
	interface Navigator {
		readonly gpu: GPU;
	}
}
