//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once
#pragma process_pending_includes

#import <NativeScript/JSIRuntime.h>
#include "rust/cxx.h"
#include <vector>
#include <cstdint>
#include <memory>
#include "canvas-cxx/src/lib.rs.h"

#include "Helpers.h"
#include "RafImpl.h"
#include "OnRafCallback.h"
#include "gl.h"

enum class WebGLRenderingVersion : uint8_t {
    V1,
    V2
};

using namespace facebook;
using namespace org::nativescript::canvas;

class WebGLRenderingContextBase : public jsi::HostObject {
public:
    WebGLRenderingContextBase(rust::Box<WebGLState> state, WebGLRenderingVersion version);

    ~WebGLRenderingContextBase();

    void UpdateInvalidateState();

    int GetInvalidateState() const;

    void SetInvalidateState(int state);

    void SetInvalidateState(InvalidateState state);

    void Flush();

    static void Flush(intptr_t context);

    WebGLState &GetState();

    void SetRaf(std::shared_ptr<RafImpl> raf);

    RafImpl *GetRaf();

    WebGLRenderingVersion GetVersion() const;
    
    int GetDisplayFramebuffer(){
        return displayFramebuffer_;
    }

    int GetDisplayRenderbuffer(){
        return displayRenderbuffer_;
    }
    
    void StartRaf();
    void StopRaf();
    
    void UpdateBindings(){
        auto displayFramebuffer = canvas_native_webgl_get_parameter(GL_FRAMEBUFFER_BINDING, *state_);
        
        auto displayRenderbuffer = canvas_native_webgl_get_parameter(GL_RENDERER, *state_);
        
        if(!canvas_native_webgl_result_get_is_none(*displayFramebuffer)){
            displayFramebuffer_ = canvas_native_webgl_result_get_i32(*displayFramebuffer);
        }
        
        if(!canvas_native_webgl_result_get_is_none(*displayRenderbuffer)){
            displayRenderbuffer_ = canvas_native_webgl_result_get_i32(*displayRenderbuffer);
        }
    }

private:
    rust::Box<WebGLState> state_;

    WebGLRenderingVersion version_;

    int invalidateState_ = static_cast<int>(InvalidateState::NONE);

    std::shared_ptr<RafImpl> raf_;
    int displayFramebuffer_;
    int displayRenderbuffer_;
};

