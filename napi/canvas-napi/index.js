/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path');

const { platform, arch } = process;

let nativeBinding = null;
let localFileExisted = false;
let loadError = null;

function isMusl() {
	// For Node 10
	if (!process.report || typeof process.report.getReport !== 'function') {
		try {
			const lddPath = require('child_process').execSync('which ldd').toString().trim();
			return readFileSync(lddPath, 'utf8').includes('musl');
		} catch (e) {
			return true;
		}
	} else {
		const { glibcVersionRuntime } = process.report.getReport().header;
		return !glibcVersionRuntime;
	}
}

switch (platform) {
	case 'android':
		switch (arch) {
			case 'arm64':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.android-arm64.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.android-arm64.node');
					} else {
						nativeBinding = require('canvas-napi-android-arm64');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			case 'arm':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.android-arm-eabi.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.android-arm-eabi.node');
					} else {
						nativeBinding = require('canvas-napi-android-arm-eabi');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			default:
				throw new Error(`Unsupported architecture on Android ${arch}`);
		}
		break;
	case 'win32':
		switch (arch) {
			case 'x64':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.win32-x64-msvc.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.win32-x64-msvc.node');
					} else {
						nativeBinding = require('canvas-napi-win32-x64-msvc');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			case 'ia32':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.win32-ia32-msvc.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.win32-ia32-msvc.node');
					} else {
						nativeBinding = require('canvas-napi-win32-ia32-msvc');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			case 'arm64':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.win32-arm64-msvc.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.win32-arm64-msvc.node');
					} else {
						nativeBinding = require('canvas-napi-win32-arm64-msvc');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			default:
				throw new Error(`Unsupported architecture on Windows: ${arch}`);
		}
		break;
	case 'darwin':
		localFileExisted = existsSync(join(__dirname, 'canvas-napi.darwin-universal.node'));
		try {
			if (localFileExisted) {
				nativeBinding = require('./canvas-napi.darwin-universal.node');
			} else {
				nativeBinding = require('canvas-napi-darwin-universal');
			}
			break;
		} catch {}
		switch (arch) {
			case 'x64':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.darwin-x64.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.darwin-x64.node');
					} else {
						nativeBinding = require('canvas-napi-darwin-x64');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			case 'arm64':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.darwin-arm64.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.darwin-arm64.node');
					} else {
						nativeBinding = require('canvas-napi-darwin-arm64');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			default:
				throw new Error(`Unsupported architecture on macOS: ${arch}`);
		}
		break;
	case 'freebsd':
		if (arch !== 'x64') {
			throw new Error(`Unsupported architecture on FreeBSD: ${arch}`);
		}
		localFileExisted = existsSync(join(__dirname, 'canvas-napi.freebsd-x64.node'));
		try {
			if (localFileExisted) {
				nativeBinding = require('./canvas-napi.freebsd-x64.node');
			} else {
				nativeBinding = require('canvas-napi-freebsd-x64');
			}
		} catch (e) {
			loadError = e;
		}
		break;
	case 'linux':
		switch (arch) {
			case 'x64':
				if (isMusl()) {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-x64-musl.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-x64-musl.node');
						} else {
							nativeBinding = require('canvas-napi-linux-x64-musl');
						}
					} catch (e) {
						loadError = e;
					}
				} else {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-x64-gnu.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-x64-gnu.node');
						} else {
							nativeBinding = require('canvas-napi-linux-x64-gnu');
						}
					} catch (e) {
						loadError = e;
					}
				}
				break;
			case 'arm64':
				if (isMusl()) {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-arm64-musl.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-arm64-musl.node');
						} else {
							nativeBinding = require('canvas-napi-linux-arm64-musl');
						}
					} catch (e) {
						loadError = e;
					}
				} else {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-arm64-gnu.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-arm64-gnu.node');
						} else {
							nativeBinding = require('canvas-napi-linux-arm64-gnu');
						}
					} catch (e) {
						loadError = e;
					}
				}
				break;
			case 'arm':
				if (isMusl()) {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-arm-musleabihf.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-arm-musleabihf.node');
						} else {
							nativeBinding = require('canvas-napi-linux-arm-musleabihf');
						}
					} catch (e) {
						loadError = e;
					}
				} else {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-arm-gnueabihf.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-arm-gnueabihf.node');
						} else {
							nativeBinding = require('canvas-napi-linux-arm-gnueabihf');
						}
					} catch (e) {
						loadError = e;
					}
				}
				break;
			case 'riscv64':
				if (isMusl()) {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-riscv64-musl.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-riscv64-musl.node');
						} else {
							nativeBinding = require('canvas-napi-linux-riscv64-musl');
						}
					} catch (e) {
						loadError = e;
					}
				} else {
					localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-riscv64-gnu.node'));
					try {
						if (localFileExisted) {
							nativeBinding = require('./canvas-napi.linux-riscv64-gnu.node');
						} else {
							nativeBinding = require('canvas-napi-linux-riscv64-gnu');
						}
					} catch (e) {
						loadError = e;
					}
				}
				break;
			case 's390x':
				localFileExisted = existsSync(join(__dirname, 'canvas-napi.linux-s390x-gnu.node'));
				try {
					if (localFileExisted) {
						nativeBinding = require('./canvas-napi.linux-s390x-gnu.node');
					} else {
						nativeBinding = require('canvas-napi-linux-s390x-gnu');
					}
				} catch (e) {
					loadError = e;
				}
				break;
			default:
				throw new Error(`Unsupported architecture on Linux: ${arch}`);
		}
		break;
	default:
		throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`);
}

if (!nativeBinding) {
	if (loadError) {
		throw loadError;
	}
	throw new Error(`Failed to load native binding`);
}

const {
	GPUAdapter,
	GPUAdapterInfo,
	GPUBindGroup,
	GPUBindGroupLayout,
	GPUCommandBuffer,
	GPUCommandEncoder,
	GPUCanvasContext,
	GPUDevice,
	GPUFeatureName,
	GPUShaderStage,
	GPUQueryType,
	GPUMipmapFilterMode,
	GPUFilterMode,
	GPUAddressMode,
	GPUBufferUsage,
	GPUTextureUsage,
	GPUErrorFilter,
	GPUTextureSampleType,
	GPUTextureViewDimension,
	GPUStorageTextureAccess,
	GPUSamplerBindingType,
	GPUBufferBindingType,
	GPUTextureDimension,
	GPUTextureAspect,
	GPUMapMode,
	GPULoadOp,
	GPUStoreOp,
	GPUVertexFormat,
	GPUVertexStepMode,
	GPUPrimitiveTopology,
	GPUIndexFormat,
	GPUFrontFace,
	GPUCullMode,
	GPUBlendOperation,
	GPUBlendFactor,
	GPUPipelineLayoutAuto,
	GPUStencilOperation,
	GPUCompareFunction,
	PredefinedColorSpaceEnum,
	GPUCanvasAlphaMode,
	GPUCanvasPresentMode,
	GPUTextureFormat,
	GPUPipelineLayout,
	GPUQuerySet,
	GPUQueue,
	GPURenderPassEncoder,
	GPURenderPipeline,
	GPUShaderModule,
	GPUTexture,
	GPUTextureView,
	GPUMapState,
	GPUBuffer,
	GPUComputePassEncoder,
	GPUComputePipeline,
	GPUExternalTexture,
	GPURenderBundle,
	GPURenderBundleEncoder,
	GPUSampler,
	GPU,
	Path2D,
	CanvasPattern,
	CanvasGradient,
	ImageData,
	TextMetrics,
	CanvasRenderingContext2D,
	ImageAsset,
	DomMatrix,
	TextEncoder,
	TextDecoder,
	WebGLProgram,
	WebGLShader,
	WebGLBuffer,
	WebGLFramebuffer,
	WebGLRenderbuffer,
	WebGLTexture,
	WebGLActiveInfo,
	ANGLE_instanced_arrays,
	OES_fbo_render_mipmap,
	EXT_blend_minmax,
	EXT_color_buffer_half_float,
	EXT_disjoint_timer_query,
	EXT_sRGB,
	EXT_shader_texture_lod,
	EXT_texture_filter_anisotropic,
	OES_element_index_uint,
	OES_standard_derivatives,
	OES_texture_float,
	OES_texture_float_linear,
	OES_texture_half_float,
	OES_texture_half_float_linear,
	OES_vertex_array_object,
	WEBGL_color_buffer_float,
	WEBGL_compressed_texture_atc,
	WEBGL_compressed_texture_etc,
	WEBGL_compressed_texture_etc1,
	WEBGL_compressed_texture_pvrtc,
	WEBGL_compressed_texture_s3tc,
	WEBGL_lose_context,
	WEBGL_depth_texture,
	WEBGL_draw_buffers,
	WebGLShaderPrecisionFormat,
	WebGLUniformLocation,
	WebGLRenderingContext,
	WebGLQuery,
	WebGLSampler,
	WebGLSync,
	WebGLTransformFeedback,
	WebGLVertexArrayObject,
	WebGL2RenderingContext,
} = nativeBinding;

module.exports.GPUAdapter = GPUAdapter;
module.exports.GPUAdapterInfo = GPUAdapterInfo;
module.exports.GPUBindGroup = GPUBindGroup;
module.exports.GPUBindGroupLayout = GPUBindGroupLayout;
module.exports.GPUCommandBuffer = GPUCommandBuffer;
module.exports.GPUCommandEncoder = GPUCommandEncoder;
module.exports.GPUCanvasContext = GPUCanvasContext;
module.exports.GPUDevice = GPUDevice;
module.exports.GPUFeatureName = GPUFeatureName;
module.exports.GPUShaderStage = GPUShaderStage;
module.exports.GPUQueryType = GPUQueryType;
module.exports.GPUMipmapFilterMode = GPUMipmapFilterMode;
module.exports.GPUFilterMode = GPUFilterMode;
module.exports.GPUAddressMode = GPUAddressMode;
module.exports.GPUBufferUsage = GPUBufferUsage;
module.exports.GPUTextureUsage = GPUTextureUsage;
module.exports.GPUErrorFilter = GPUErrorFilter;
module.exports.GPUTextureSampleType = GPUTextureSampleType;
module.exports.GPUTextureViewDimension = GPUTextureViewDimension;
module.exports.GPUStorageTextureAccess = GPUStorageTextureAccess;
module.exports.GPUSamplerBindingType = GPUSamplerBindingType;
module.exports.GPUBufferBindingType = GPUBufferBindingType;
module.exports.GPUTextureDimension = GPUTextureDimension;
module.exports.GPUTextureAspect = GPUTextureAspect;
module.exports.GPUMapMode = GPUMapMode;
module.exports.GPULoadOp = GPULoadOp;
module.exports.GPUStoreOp = GPUStoreOp;
module.exports.GPUVertexFormat = GPUVertexFormat;
module.exports.GPUVertexStepMode = GPUVertexStepMode;
module.exports.GPUPrimitiveTopology = GPUPrimitiveTopology;
module.exports.GPUIndexFormat = GPUIndexFormat;
module.exports.GPUFrontFace = GPUFrontFace;
module.exports.GPUCullMode = GPUCullMode;
module.exports.GPUBlendOperation = GPUBlendOperation;
module.exports.GPUBlendFactor = GPUBlendFactor;
module.exports.GPUPipelineLayoutAuto = GPUPipelineLayoutAuto;
module.exports.GPUStencilOperation = GPUStencilOperation;
module.exports.GPUCompareFunction = GPUCompareFunction;
module.exports.PredefinedColorSpaceEnum = PredefinedColorSpaceEnum;
module.exports.GPUCanvasAlphaMode = GPUCanvasAlphaMode;
module.exports.GPUCanvasPresentMode = GPUCanvasPresentMode;
module.exports.GPUTextureFormat = GPUTextureFormat;
module.exports.GPUPipelineLayout = GPUPipelineLayout;
module.exports.GPUQuerySet = GPUQuerySet;
module.exports.GPUQueue = GPUQueue;
module.exports.GPURenderPassEncoder = GPURenderPassEncoder;
module.exports.GPURenderPipeline = GPURenderPipeline;
module.exports.GPUShaderModule = GPUShaderModule;
module.exports.GPUTexture = GPUTexture;
module.exports.GPUTextureView = GPUTextureView;
module.exports.GPUMapState = GPUMapState;
module.exports.GPUBuffer = GPUBuffer;
module.exports.GPUComputePassEncoder = GPUComputePassEncoder;
module.exports.GPUComputePipeline = GPUComputePipeline;
module.exports.GPUExternalTexture = GPUExternalTexture;
module.exports.GPURenderBundle = GPURenderBundle;
module.exports.GPURenderBundleEncoder = GPURenderBundleEncoder;
module.exports.GPUSampler = GPUSampler;
module.exports.GPU = GPU;
module.exports.Path2D = Path2D;
module.exports.CanvasPattern = CanvasPattern;
module.exports.CanvasGradient = CanvasGradient;
module.exports.ImageData = ImageData;
module.exports.TextMetrics = TextMetrics;
module.exports.CanvasRenderingContext2D = CanvasRenderingContext2D;
module.exports.ImageAsset = ImageAsset;
module.exports.DomMatrix = DomMatrix;
module.exports.TextEncoder = TextEncoder;
module.exports.TextDecoder = TextDecoder;
module.exports.WebGLProgram = WebGLProgram;
module.exports.WebGLShader = WebGLShader;
module.exports.WebGLBuffer = WebGLBuffer;
module.exports.WebGLFramebuffer = WebGLFramebuffer;
module.exports.WebGLRenderbuffer = WebGLRenderbuffer;
module.exports.WebGLTexture = WebGLTexture;
module.exports.WebGLActiveInfo = WebGLActiveInfo;
module.exports.ANGLE_instanced_arrays = ANGLE_instanced_arrays;
module.exports.OES_fbo_render_mipmap = OES_fbo_render_mipmap;
module.exports.EXT_blend_minmax = EXT_blend_minmax;
module.exports.EXT_color_buffer_half_float = EXT_color_buffer_half_float;
module.exports.EXT_disjoint_timer_query = EXT_disjoint_timer_query;
module.exports.EXT_sRGB = EXT_sRGB;
module.exports.EXT_shader_texture_lod = EXT_shader_texture_lod;
module.exports.EXT_texture_filter_anisotropic = EXT_texture_filter_anisotropic;
module.exports.OES_element_index_uint = OES_element_index_uint;
module.exports.OES_standard_derivatives = OES_standard_derivatives;
module.exports.OES_texture_float = OES_texture_float;
module.exports.OES_texture_float_linear = OES_texture_float_linear;
module.exports.OES_texture_half_float = OES_texture_half_float;
module.exports.OES_texture_half_float_linear = OES_texture_half_float_linear;
module.exports.OES_vertex_array_object = OES_vertex_array_object;
module.exports.WEBGL_color_buffer_float = WEBGL_color_buffer_float;
module.exports.WEBGL_compressed_texture_atc = WEBGL_compressed_texture_atc;
module.exports.WEBGL_compressed_texture_etc = WEBGL_compressed_texture_etc;
module.exports.WEBGL_compressed_texture_etc1 = WEBGL_compressed_texture_etc1;
module.exports.WEBGL_compressed_texture_pvrtc = WEBGL_compressed_texture_pvrtc;
module.exports.WEBGL_compressed_texture_s3tc = WEBGL_compressed_texture_s3tc;
module.exports.WEBGL_lose_context = WEBGL_lose_context;
module.exports.WEBGL_depth_texture = WEBGL_depth_texture;
module.exports.WEBGL_draw_buffers = WEBGL_draw_buffers;
module.exports.WebGLShaderPrecisionFormat = WebGLShaderPrecisionFormat;
module.exports.WebGLUniformLocation = WebGLUniformLocation;
module.exports.WebGLRenderingContext = WebGLRenderingContext;
module.exports.WebGLQuery = WebGLQuery;
module.exports.WebGLSampler = WebGLSampler;
module.exports.WebGLSync = WebGLSync;
module.exports.WebGLTransformFeedback = WebGLTransformFeedback;
module.exports.WebGLVertexArrayObject = WebGLVertexArrayObject;
module.exports.WebGL2RenderingContext = WebGL2RenderingContext;
