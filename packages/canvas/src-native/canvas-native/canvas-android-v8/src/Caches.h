//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "Common.h"
#include "ConcurrentMap.h"
#include "OnImageAssetLoadCallbackHolder.h"
#include "TextEncoderImplEntry.h"
#include "ObjectCacheEntry.h"

class Caches {
public:
    Caches(v8::Isolate *isolate);

    ~Caches();

    static std::shared_ptr <Caches> Get(v8::Isolate *isolate);

    static void Remove(v8::Isolate *isolate);

    void SetContext(v8::Local<v8::Context> context);

    v8::Local<v8::Context> GetContext();


    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> TextMetricsTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> CanvasRenderingContext2DTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLRenderingContextTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> CanvasGradientTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> CanvasPatternTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> ImageDataTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> Path2DTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> ImageAssetTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> MatrixTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> TextEncoderTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> TextDecoderTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLBufferTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLActiveInfoTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLShaderPrecisionFormatTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLFramebufferTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLProgramTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLRenderbufferTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLShaderTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLTextureTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLUniformLocationTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_blend_minmaxTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_color_buffer_half_floatImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_disjoint_timer_queryImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> ANGLE_instanced_arraysImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_sRGBImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_shader_texture_lodImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_texture_filter_anisotropicImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> OES_element_index_uintImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> OES_standard_derivativesImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> OES_texture_floatImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> OES_texture_float_linearImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> OES_texture_half_floatImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> OES_texture_half_float_linearImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_color_buffer_floatImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_compressed_texture_atcImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_compressed_texture_etc1ImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_compressed_texture_etcImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_draw_buffersImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_compressed_texture_pvrtcImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_compressed_texture_s3tcImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_depth_textureImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WEBGL_compressed_texture_s3tc_srgbImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> OES_vertex_array_objectImplTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);


    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLQueryTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLSamplerTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLSyncTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLTransformFeedbackTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLVertexArrayObjectTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGL2RenderingContextTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);


    std::shared_ptr<ConcurrentMap<intptr_t, std::shared_ptr<OnImageAssetLoadCallbackHolder>>> OnImageAssetLoadCallbackHolder_ = std::make_shared<ConcurrentMap<intptr_t, std::shared_ptr<OnImageAssetLoadCallbackHolder>>>();


    std::shared_ptr<ConcurrentMap<TextEncoderImplEntry *,
            std::shared_ptr<v8::Persistent<v8::Object>>>> TextEncoderData = std::make_shared<ConcurrentMap<TextEncoderImplEntry *,
            std::shared_ptr<v8::Persistent<v8::Object>>>>();


    void SetPerformingMicrotaskCheckpoint(bool value);

    bool GetPerformingMicrotaskCheckpoint() const;

private:
    static std::shared_ptr <ConcurrentMap<v8::Isolate *,
            std::shared_ptr <Caches>>>
    perIsolateCaches_;
    v8::Isolate *isolate_;
    std::shared_ptr <v8::Persistent<v8::Context>> context_;
    bool performingMicrotaskCheckpoint_;
};
