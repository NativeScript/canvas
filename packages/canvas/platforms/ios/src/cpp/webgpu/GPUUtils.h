//
// Created by Osei Fortune on 18/07/2024.
//

#ifndef CANVAS_ANDROID_GPUUTILS_H
#define CANVAS_ANDROID_GPUUTILS_H

#include "Common.h"
#include "Helpers.h"

inline static CanvasCompareFunction
ParseCompareFunction(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                     CanvasCompareFunction defaultValue) {

    auto compareStr = ConvertFromV8String(isolate, obj);

    if (compareStr == "never") {
        return CanvasCompareFunctionNever;
    } else if (compareStr == "less") {
        return CanvasCompareFunctionLess;
    } else if (compareStr == "equal") {
        return CanvasCompareFunctionEqual;
    } else if (compareStr == "less-equal") {
        return CanvasCompareFunctionLessEqual;
    } else if (compareStr == "greater") {
        return CanvasCompareFunctionGreater;
    } else if (compareStr == "not-equal") {
        return CanvasCompareFunctionNotEqual;
    } else if (compareStr == "greater-equal") {
        return CanvasCompareFunctionGreaterEqual;
    } else if (compareStr == "always") {
        return CanvasCompareFunctionAlways;
    }

    return defaultValue;
}

inline static CanvasStencilOperation
ParseStencilOperation(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                      CanvasStencilOperation defaultValue) {

    auto op = ConvertFromV8String(isolate, obj);

    if (op == "decrement-clamp") {
        return CanvasStencilOperationDecrementClamp;
    } else if (op == "decrement-wrap") {
        return CanvasStencilOperationDecrementWrap;
    } else if (op == "invert") {
        return CanvasStencilOperationInvert;
    } else if (op == "increment-clamp") {
        return CanvasStencilOperationIncrementClamp;
    } else if (op == "increment-wrap") {
        return CanvasStencilOperationIncrementWrap;
    } else if (op == "keep") {
        return CanvasStencilOperationKeep;
    } else if (op == "replace") {
        return CanvasStencilOperationReplace;
    } else if (op == "zero") {
        return CanvasStencilOperationZero;
    }

    return defaultValue;
}


inline static CanvasBlendFactor
ParseBlendFactor(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                 CanvasBlendFactor defaultValue) {

    auto factor = ConvertFromV8String(isolate, obj);

    if (factor == "constant") {
        return CanvasBlendFactorConstant;
    } else if (factor == "dst") {
        return CanvasBlendFactorDst;
    } else if (factor == "dst-alpha") {
        return CanvasBlendFactorDstAlpha;
    } else if (factor == "one") {
        return CanvasBlendFactorOne;
    } else if (factor == "one-minus-dst") {
        return CanvasBlendFactorOneMinusDst;
    } else if (factor == "one-minus-src") {
        return CanvasBlendFactorOneMinusSrc;
    } else if (factor == "one-minus-src-alpha") {
        return CanvasBlendFactorOneMinusSrcAlpha;
    } else if (factor == "one-minus-dst-alpha") {
        return CanvasBlendFactorOneMinusDstAlpha;
    } else if (factor == "one-minus-constant") {
        return CanvasBlendFactorOneMinusConstant;
    } else if (factor == "src") {
        return CanvasBlendFactorSrc;
    } else if (factor == "src-alpha") {
        return CanvasBlendFactorSrcAlpha;
    } else if (factor == "src-alpha-saturated") {
        return CanvasBlendFactorSrcAlphaSaturated;
    } else if (factor == "zero") {
        return CanvasBlendFactorZero;
    }

    return defaultValue;
}


inline static CanvasBlendOperation
ParseBlendOperation(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                    CanvasBlendOperation defaultValue) {

    auto op = ConvertFromV8String(isolate, obj);

    if (op == "add") {
        return CanvasBlendOperationAdd;
    } else if (op == "max") {
        return CanvasBlendOperationMax;
    } else if (op == "min") {
        return CanvasBlendOperationMin;
    } else if (op == "reverse-subtract") {
        return CanvasBlendOperationReverseSubtract;
    } else if (op == "subtract") {
        return CanvasBlendOperationSubtract;
    }

    return defaultValue;
}

inline static CanvasExtent3d
ParseExtent3d(v8::Isolate *isolate, const v8::Local<v8::Value> &obj) {

    auto context = isolate->GetCurrentContext();
    CanvasExtent3d ret{
            0, 1, 1
    };

    if (!obj.IsEmpty()) {
        if (obj->IsArray()) {
            auto array = obj.As<v8::Array>();
            v8::Local<v8::Value> width;
            if (array->Get(context, 0).ToLocal(&width) &&
                width->IsUint32()) {
                ret.width = width->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> height;
            if (array->Get(context, 1).ToLocal(&height) &&
                height->IsUint32()) {
                ret.height = height->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> depthOrArrayLayers;
            if (array->Get(context, 2).ToLocal(
                    &depthOrArrayLayers) && depthOrArrayLayers->IsUint32()) {
                ret.depth_or_array_layers = depthOrArrayLayers->Uint32Value(context).FromJust();
            }
        } else if (obj->IsObject()) {
            auto extObj = obj.As<v8::Object>();
            v8::Local<v8::Value> width;
            if (extObj->Get(context, ConvertToV8String(isolate, "width")).ToLocal(&width) &&
                width->IsUint32()) {
                ret.width = width->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> height;
            if (extObj->Get(context, ConvertToV8String(isolate, "height")).ToLocal(&height) &&
                height->IsUint32()) {
                ret.height = height->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> depthOrArrayLayers;
            if (extObj->Get(context, ConvertToV8String(isolate, "depthOrArrayLayers")).ToLocal(
                    &depthOrArrayLayers) && depthOrArrayLayers->IsUint32()) {
                ret.depth_or_array_layers = depthOrArrayLayers->Uint32Value(context).FromJust();
            }
        }
    }

    return ret;
}


inline static CanvasColor
ParseColor(v8::Isolate *isolate, const v8::Local<v8::Value> &obj) {

    auto context = isolate->GetCurrentContext();
    CanvasColor ret{
            0, 0, 0, 0
    };

    if (!obj.IsEmpty()) {
        if (obj->IsArray()) {
            auto array = obj.As<v8::Array>();
            v8::Local<v8::Value> r;
            if (array->Get(context, 0).ToLocal(&r) &&
                r->IsNumber()) {
                ret.r = r->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> g;
            if (array->Get(context, 1).ToLocal(&g) &&
                g->IsNumber()) {
                ret.g = g->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> b;
            if (array->Get(context, 2).ToLocal(
                    &b) && b->IsNumber()) {
                ret.b = b->NumberValue(context).FromJust();
            }


            v8::Local<v8::Value> a;
            if (array->Get(context, 3).ToLocal(
                    &a) && a->IsNumber()) {
                ret.a = a->NumberValue(context).FromJust();
            }
        } else if (obj->IsObject()) {
            auto colorObj = obj.As<v8::Object>();
            v8::Local<v8::Value> r;
            if (colorObj->Get(context, ConvertToV8String(isolate, "r")).ToLocal(&r) &&
                r->IsNumber()) {
                ret.r = r->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> g;
            if (colorObj->Get(context, ConvertToV8String(isolate, "g")).ToLocal(&g) &&
                g->IsNumber()) {
                ret.g = g->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> b;
            if (colorObj->Get(context, ConvertToV8String(isolate, "b")).ToLocal(
                    &b) && b->IsNumber()) {
                ret.b = b->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> a;
            if (colorObj->Get(context, ConvertToV8String(isolate, "a")).ToLocal(
                    &a) && a->IsNumber()) {
                ret.a = a->NumberValue(context).FromJust();
            }
        }
    }

    return ret;
}

#endif //CANVAS_ANDROID_GPUUTILS_H
