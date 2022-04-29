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

    std::unique_ptr <v8::Persistent<v8::Function>> TextMetricsTmpl = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> CanvasRenderingContext2DTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> CanvasRenderingContext2DCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLRenderingContextTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLRenderingContextCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> CanvasGradientCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> CanvasPatternCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> ImageDataCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> Path2DCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> ImageAssetCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> MatrixCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::shared_ptr<ConcurrentMap<intptr_t, std::shared_ptr<OnImageAssetLoadCallbackHolder>>> OnImageAssetLoadCallbackHolder_ = std::make_shared<ConcurrentMap<intptr_t, std::shared_ptr<OnImageAssetLoadCallbackHolder>>>();

    std::unique_ptr <v8::Persistent<v8::Function>> TextEncoderCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> TextDecoderCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLBufferCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLActiveInfoCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLShaderPrecisionFormatCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLFramebufferCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLProgramCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLRenderbufferCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLShaderCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLTextureCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WebGLUniformLocationCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> EXT_blend_minmaxCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> EXT_color_buffer_half_floatImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> EXT_disjoint_timer_queryImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> ANGLE_instanced_arraysImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> EXT_sRGBImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> EXT_shader_texture_lodImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> EXT_texture_filter_anisotropicImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> OES_element_index_uintImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> OES_standard_derivativesImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> OES_texture_floatImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> OES_texture_float_linearImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> OES_texture_half_floatImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> OES_texture_half_float_linearImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);


    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_color_buffer_floatImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_compressed_texture_atcImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_compressed_texture_etc1ImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_compressed_texture_etcImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_draw_buffersImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_compressed_texture_pvrtcImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_compressed_texture_s3tcImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_depth_textureImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> WEBGL_compressed_texture_s3tc_srgbImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> OES_vertex_array_objectImplCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);


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
