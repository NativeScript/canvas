//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"
#include "../Helpers.h"
#include "../RafImpl.h"


enum class WebGLRenderingVersion : uint8_t {
    V1,
    V2
};

using namespace facebook;

class WebGLRenderingContextBase : public jsi::HostObject {
public:
    WebGLRenderingContextBase(rust::Box<WebGLState> state, WebGLRenderingVersion version);

    ~WebGLRenderingContextBase();

    void UpdateInvalidateState();

    int GetInvalidateState() const;

    void SetInvalidateState(int state);

    void Flush();

    static void Flush(intptr_t context);

    WebGLState &GetState();

    void SetRaf(std::shared_ptr<RafImpl> raf);

    RafImpl *GetRaf();

    WebGLRenderingVersion GetVersion() const;


private:
    rust::Box<WebGLState> state_;

    WebGLRenderingVersion version_;

    int invalidateState_ = static_cast<int>(InvalidateState::NONE);

    std::shared_ptr<RafImpl> raf_ = nullptr;
};

