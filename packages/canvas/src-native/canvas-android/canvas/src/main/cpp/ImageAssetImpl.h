//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#include <unistd.h>
#include <thread>
#import "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT ImageAssetImpl : public jsi::HostObject {
public:
    ImageAssetImpl(rust::Box<ImageAsset> asset);

    ~ImageAssetImpl();

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    ImageAsset &GetImageAsset();

private:
    rust::Box<ImageAsset> asset_;
};
