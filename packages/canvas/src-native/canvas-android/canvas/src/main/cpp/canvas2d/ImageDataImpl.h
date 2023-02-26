//
// Created by Osei Fortune on 27/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#import "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT ImageDataImpl : public jsi::HostObject {
public:
    ImageDataImpl(rust::Box<ImageData> imageData);

    ~ImageDataImpl();

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    ImageData &GetImageData();

private:
    rust::Box<ImageData> imageData_;
};
