//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#include "v8runtime/V8Runtime.h"
#include <vector>

using namespace facebook;

class JSI_EXPORT CanvasGradient : public jsi::HostObject {
public:
    CanvasGradient(rust::Box<PaintStyle> style);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    PaintStyle &GetPaintStyle();

private:
    rust::Box<PaintStyle> style_;
};

