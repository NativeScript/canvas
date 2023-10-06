//
// Created by Osei Fortune on 10/06/2022.
//

#pragma once

#include "Common.h"
#include "ConcurrentMap.h"

class Caches {
public:
    Caches(v8::Isolate *isolate);

    ~Caches();

    static std::shared_ptr <Caches> Get(v8::Isolate *isolate);

    static void Remove(v8::Isolate *isolate);

    void SetContext(v8::Local<v8::Context> context);

    v8::Local<v8::Context> GetContext();


    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> TextDecoderTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> TextEncoderTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> Path2DTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> ImageDataTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> ImageAssetTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> CanvasGradientTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> CanvasPatternTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> MatrixTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> TextMetricsTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> CanvasRenderingContext2DTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> ImageBitmapTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> ANGLE_instanced_arraysTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_blend_minmaxTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_color_buffer_half_floatTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLQueryTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_disjoint_timer_queryTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> EXT_shader_texture_lodTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLActiveInfoTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLBufferTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLSamplerTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLSyncTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLTransformFeedbackTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLVertexArrayObjectTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLRenderingContextTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGL2RenderingContextTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLProgramTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLShaderTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLFramebufferTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLRenderbufferTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> WebGLTextureTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);


private:
    static std::shared_ptr <ConcurrentMap<v8::Isolate *,
            std::shared_ptr <Caches>>>
    perIsolateCaches_;
    v8::Isolate *isolate_;
    std::shared_ptr <v8::Persistent<v8::Context>> context_;
};
