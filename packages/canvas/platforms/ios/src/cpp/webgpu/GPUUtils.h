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

#endif //CANVAS_ANDROID_GPUUTILS_H
