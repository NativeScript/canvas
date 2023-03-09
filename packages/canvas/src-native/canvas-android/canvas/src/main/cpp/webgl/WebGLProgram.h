//
// Created by Osei Fortune on 27/04/2022.
//

#pragma

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class WebGLProgram: public jsi::HostObject {
public:
    WebGLProgram(uint32_t program) : program_(program) {}

    uint32_t GetProgram() {
        return this->program_;
    }

private:
    uint32_t program_;
};

