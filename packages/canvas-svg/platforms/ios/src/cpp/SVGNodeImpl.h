// V8-bound wrapper around one canvas-svg SvgNode (SvgElementHandle).
#pragma once

#include "SVGCommon.h"
#include "SVGHelpers.h"
#include "SVGObjectWrapperImpl.h"

class SVGNodeImpl : SVGObjectWrapperImpl {
public:
    explicit SVGNodeImpl(SvgNode *node) : node_(node) {}

    ~SVGNodeImpl() {
        canvas_native_svg_node_release(this->node_);
        this->node_ = nullptr;
    }

    SvgNode *GetNode() {
        return this->node_;
    }

    static void Init(v8::Local<v8::Object> module, v8::Isolate *isolate);

    static SVGNodeImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, SVGNodeImpl *node);

    static void TagName(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetAttribute(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetAttribute(const v8::FunctionCallbackInfo<v8::Value> &args);

    // Free functions: SVGModule.createElement/createTextNode (no owning document needed).
    static void CreateElement(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateTextNode(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AppendChild(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RemoveChild(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Text(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetText(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    SvgNode *node_;
};
