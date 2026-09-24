// V8-bound wrapper around a live, mutable canvas-svg SvgDocument.
#pragma once

#include "SVGCommon.h"
#include "SVGHelpers.h"
#include "SVGObjectWrapperImpl.h"

class SVGDocumentImpl : SVGObjectWrapperImpl {
public:
    explicit SVGDocumentImpl(SvgDocument *document) : document_(document) {}

    ~SVGDocumentImpl() {
        canvas_native_svg_document_release(this->document_);
        this->document_ = nullptr;
    }

    SvgDocument *GetDocument() {
        return this->document_;
    }

    static void Init(v8::Local<v8::Object> module, v8::Isolate *isolate);

    static SVGDocumentImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, SVGDocumentImpl *document);

    // Registered as SVGModule.createSVGDocument(...), not a public constructor.
    static void CreateSVGDocument(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Root(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateElement(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateTextNode(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetElementById(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RegisterId(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UnregisterId(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetContainerSize(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetLayer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InvalidateBackdrop(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetFrameSharing(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InvalidateFrames(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void NativePointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    // SMIL. `hasAnimations` is what tells JS whether to drive a clock at all.
    static void HasAnimations(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AnimationDuration(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CurrentTime(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetCurrentTime(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RenderToBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    SvgDocument *document_;
};
