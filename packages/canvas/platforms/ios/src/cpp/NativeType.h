//
// Created by Osei Fortune on 06/08/2024.
//

#ifndef CANVAS_ANDROID_NATIVETYPE_H
#define CANVAS_ANDROID_NATIVETYPE_H

enum class NativeType {
    None,
    CanvasGradient,
    CanvasPattern,
    ImageData,
    ImageAsset,
    CanvasRenderingContext2D,
    WebGLRenderingContextBase,
    Path2D,
    Matrix,
    ImageBitmap,
    TextMetrics,

    WebGLQuery,
    WebGLProgram,
    WebGLShader,
    WebGLBuffer,
    WebGLFramebuffer,
    WebGLRenderbuffer,
    WebGLTexture,
    WebGLActiveInfo,
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
    OES_texture_half_float_linear,
    OES_texture_half_float,
    WEBGL_color_buffer_float,
    OES_vertex_array_object,
    WebGLVertexArrayObject,
    WEBGL_compressed_texture_atc,
    WEBGL_compressed_texture_etc1,
    WEBGL_compressed_texture_s3tc,
    WEBGL_compressed_texture_s3tc_srgb,
    WEBGL_compressed_texture_etc,
    WEBGL_compressed_texture_pvrtc,
    WEBGL_lose_context,
    ANGLE_instanced_arrays,
    WEBGL_depth_texture,
    WEBGL_draw_buffers,
    WebGLShaderPrecisionFormat,
    WebGLUniformLocation,
    WebGLSampler,
    WebGLTransformFeedback,
    WebGLSync,

    GPUAdapter,
    GPUSupportedLimits,
    GPUDevice,
    GPUQueue,
    GPUBuffer,
    GPUInstance,
    GPUCanvasContext,
    GPUTexture,
    GPUAdapterInfo,
    GPUCommandEncoder,
    GPUComputePass,
    GPUQuerySet,
    GPUShaderModule,
    GPUPipelineLayout,
    GPURenderPipeline,
    GPUBindGroupLayout,
    GPUTextureView,
    GPURenderPassEncoder,
    GPUCommandBuffer,
    GPUBindGroup,
    GPUComputePipeline,
    GPUSampler,
    GPURenderBundleEncoder,
    GPURenderBundle
};

#endif //CANVAS_ANDROID_NATIVETYPE_H
