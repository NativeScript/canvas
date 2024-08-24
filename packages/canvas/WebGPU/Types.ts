import type { GPUExternalTexture } from './GPUExternalTexture';
import type { GPUSampler } from './GPUSampler';
import type { GPUTextureView } from './GPUTextureView';
import type { GPUBufferBinding, GPUColorDict } from './Interfaces';
// export type GPUImageCopyExternalImageSource = ImageBitmap | ImageData | HTMLImageElement | HTMLVideoElement | VideoFrame | HTMLCanvasElement | OffscreenCanvas;
export type GPUOrigin2D = [number, number] | { x?: number; y?: number };
export type GPUExtent3D = [number, number, number] | [number, number] | { depthOrArrayLayers?: number; height?: number; width: number };
export type GPUOrigin3D = [number, number, number] | { x: number; y: number; z: number };
export type GPUTextureAspect = 'all' | 'stencil-only' | 'depth-only';
export type GPULoadOp = 'load' | 'clear';
export type GPUStoreOp = 'store' | 'discard';
export type GPUColor = [number, number, number, number] | GPUColorDict;
export type GPUErrorFilter = 'validation' | 'out-of-memory' | 'internal';
export type GPUMipmapFilterMode = 'nearest' | 'linear';
export type GPUFilterMode = 'nearest' | 'linear';
export type GPUAddressMode = 'clamp-to-edge' | 'repeat' | 'mirror-repeat';
export type GPUVertexStepMode = 'vertex' | 'instance';
export type GPUPrimitiveTopology = 'point-list' | 'line-list' | 'line-strip' | 'triangle-list' | 'triangle-strip';
export type GPUIndexFormat = 'uint16' | 'uint32';
export type GPUFrontFace = 'ccw' | 'cw';
export type GPUCullMode = 'none' | 'front' | 'back';
export type GPUBlendOperation = 'add' | 'subtract' | 'reverse-subtract' | 'min' | 'max';
export type GPUBlendFactor = 'zero' | 'one' | 'src' | 'one-minus-src' | 'src-alpha' | 'one-minus-src-alpha' | 'dst' | 'one-minus-dst' | 'dst-alpha' | 'one-minus-dst-alpha' | 'src-alpha-saturated' | 'constant' | 'one-minus-constant';
export type GPUStencilOperation = 'keep' | 'zero' | 'replace' | 'invert' | 'increment-clamp' | 'decrement-clamp' | 'increment-wrap' | 'decrement-wrap';
export type GPUCompareFunction = 'never' | 'less' | 'equal' | 'less-equal' | 'greater' | 'not-equal' | 'greater-equal' | 'always';
export type GPUQueryType = 'occlusion' | 'timestamp';
export type GPUTextureViewDimension = '1d' | '2d' | '2d-array' | 'cube' | 'cube-array' | '3d';
export type GPUTextureSampleType = 'float' | 'unfilterable-float' | 'depth' | 'sint' | 'uint';
export type GPUTextureFormat =
	| 'r8unorm'
	| 'r8snorm'
	| 'r8uint'
	| 'r8sint'
	| 'r16uint'
	| 'r16sint'
	| 'r16float'
	| 'rg8unorm'
	| 'rg8snorm'
	| 'rg8uint'
	| 'rg8sint'
	| 'r32uint'
	| 'r32sint'
	| 'r32float'
	| 'rg16uint'
	| 'rg16sint'
	| 'rg16float'
	| 'rgba8unorm'
	| 'rgba8unorm-srgb'
	| 'rgba8snorm'
	| 'rgba8uint'
	| 'rgba8sint'
	| 'bgra8unorm'
	| 'bgra8unorm-srgb'
	| 'rgb9e5ufloat'
	| 'rgb10a2uint'
	| 'rgb10a2unorm'
	| 'rg11b10ufloat'
	| 'rg32uint'
	| 'rg32sint'
	| 'rg32float'
	| 'rgba16uint'
	| 'rgba16sint'
	| 'rgba16float'
	| 'rgba32uint'
	| 'rgba32sint'
	| 'rgba32float'
	| 'stencil8'
	| 'depth16unorm'
	| 'depth24plus'
	| 'depth24plus-stencil8'
	| 'depth32float'
	| 'depth32float-stencil8'
	| 'bc1-rgba-unorm'
	| 'bc1-rgba-unorm-srgb'
	| 'bc2-rgba-unorm'
	| 'bc2-rgba-unorm-srgb'
	| 'bc3-rgba-unorm'
	| 'bc3-rgba-unorm-srgb'
	| 'bc4-r-unorm'
	| 'bc4-r-snorm'
	| 'bc5-rg-unorm'
	| 'bc5-rg-snorm'
	| 'bc6h-rgb-ufloat'
	| 'bc6h-rgb-float'
	| 'bc7-rgba-unorm'
	| 'bc7-rgba-unorm-srgb'
	| 'etc2-rgb8unorm'
	| 'etc2-rgb8unorm-srgb'
	| 'etc2-rgb8a1unorm'
	| 'etc2-rgb8a1unorm-srgb'
	| 'etc2-rgba8unorm'
	| 'etc2-rgba8unorm-srgb'
	| 'eac-r11unorm'
	| 'eac-r11snorm'
	| 'eac-rg11unorm'
	| 'eac-rg11snorm'
	| 'astc-4x4-unorm'
	| 'astc-4x4-unorm-srgb'
	| 'astc-5x4-unorm'
	| 'astc-5x4-unorm-srgb'
	| 'astc-5x5-unorm'
	| 'astc-5x5-unorm-srgb'
	| 'astc-6x5-unorm'
	| 'astc-6x5-unorm-srgb'
	| 'astc-6x6-unorm'
	| 'astc-6x6-unorm-srgb'
	| 'astc-8x5-unorm'
	| 'astc-8x5-unorm-srgb'
	| 'astc-8x6-unorm'
	| 'astc-8x6-unorm-srgb'
	| 'astc-8x8-unorm'
	| 'astc-8x8-unorm-srgb'
	| 'astc-10x5-unorm'
	| 'astc-10x5-unorm-srgb'
	| 'astc-10x6-unorm'
	| 'astc-10x6-unorm-srgb'
	| 'astc-10x8-unorm'
	| 'astc-10x8-unorm-srgb'
	| 'astc-10x10-unorm'
	| 'astc-10x10-unorm-srgb'
	| 'astc-12x10-unorm'
	| 'astc-12x10-unorm-srgb'
	| 'astc-12x12-unorm'
	| 'astc-12x12-unorm-srgb';

export type GPUVertexFormat = 'uint8x2' | 'uint8x4' | 'sint8x2' | 'sint8x4' | 'unorm8x2' | 'unorm8x4' | 'snorm8x2' | 'snorm8x4' | 'uint16x2' | 'uint16x4' | 'sint16x2' | 'sint16x4' | 'unorm16x2' | 'unorm16x4' | 'snorm16x2' | 'snorm16x4' | 'float16x2' | 'float16x4' | 'float32' | 'float32x2' | 'float32x3' | 'float32x4' | 'uint32' | 'uint32x2' | 'uint32x3' | 'uint32x4' | 'sint32' | 'sint32x2' | 'sint32x3' | 'sint32x4' | 'unorm10-10-10-2';

export type GPUCanvasAlphaMode = 'opaque' | 'premultiplied' | 'postmultiplied' | 'inherit';

export type GPUCanvasPresentMode = 'autoVsync' | 'autoNoVsync' | 'fifo' | 'fifoRelaxed' | 'immediate' | 'mailbox';

export type GPUFeatureName = 'depth-clip-control' | 'depth32float-stencil8' | 'texture-compression-bc' | 'texture-compression-etc2' | 'texture-compression-astc' | 'timestamp-query' | 'indirect-first-instance' | 'shader-f16' | 'rg11b10ufloat-renderable' | 'bgra8unorm-storage' | 'float32-filterable';

export type GPUBindingResource = GPUSampler | GPUTextureView | GPUBufferBinding | GPUExternalTexture;

export type GPUBufferBindingType = 'uniform' | 'storage' | 'read-only-storage';

export type GPUSamplerBindingType = 'filtering' | 'non-filtering' | 'comparison';

export type GPUStorageTextureAccess = 'write-only' | 'read-only' | 'read-write';
