//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#import "v8runtime/V8Runtime.h"
#include "MatrixImpl.h"

using namespace facebook;

class JSI_EXPORT CanvasPattern: public jsi::HostObject {
public:
    CanvasPattern(rust::Box<PaintStyle> style);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

    PaintStyle& GetPaintStyle();

private:
    rust::Box<PaintStyle> style_;
};

