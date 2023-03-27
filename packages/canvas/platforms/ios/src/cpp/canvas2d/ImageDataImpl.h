//
// Created by Osei Fortune on 27/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#import "NativeScript/JSIRuntime.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT ImageDataImpl : public jsi::HostObject {
public:
    ImageDataImpl(rust::Box<ImageData> imageData);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    ImageData &GetImageData() {
        return *this->imageData_;
    }

private:
    rust::Box<ImageData> imageData_;
};
