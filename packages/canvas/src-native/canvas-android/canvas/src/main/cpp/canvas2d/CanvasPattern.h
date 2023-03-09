//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"
#include "MatrixImpl.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT CanvasPattern : public jsi::HostObject {
public:
    CanvasPattern(rust::Box<PaintStyle> style);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    PaintStyle &GetPaintStyle();

private:
    rust::Box<PaintStyle> style_;
};

