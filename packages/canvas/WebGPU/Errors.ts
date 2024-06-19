export class GPUError extends Error {}
const GPUErrorPrototype = GPUError.prototype;

export class GPUValidationError extends GPUError {}
