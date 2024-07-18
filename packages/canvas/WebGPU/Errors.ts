import { native_ } from './Constants';

export class GPUError extends Error {}
const GPUErrorPrototype = GPUError.prototype;

export class GPUValidationError extends GPUError {}

export class GPUOutOfMemoryError extends GPUError {}

export class GPUInternalError extends GPUError {}

export class GPUUncapturedErrorEvent {
	[native_];
	constructor(type: 'internal' | 'out-of-memory' | 'validation', options: { error: GPUError }) {
		this[native_] = options.error;
	}

	get error() {
		return this[native_];
	}
}
