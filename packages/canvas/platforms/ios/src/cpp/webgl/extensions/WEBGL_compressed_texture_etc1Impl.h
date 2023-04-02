//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#import "rust/cxx.h"
#import "NativeScript/JSIRuntime.h"
#import "gl.h"
#import <vector>

using namespace facebook;

class JSI_EXPORT WEBGL_compressed_texture_etc1Impl : public jsi::HostObject {
    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;
};
