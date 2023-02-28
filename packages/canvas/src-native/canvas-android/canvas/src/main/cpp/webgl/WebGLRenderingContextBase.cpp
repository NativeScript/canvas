//
// Created by Osei Fortune on 30/04/2022.
//

#include "WebGLRenderingContextBase.h"

#include <utility>
#include "canvas-android/src/lib.rs.h"

WebGLRenderingContextBase::WebGLRenderingContextBase(rust::Box<WebGLState> state,
                                                     WebGLRenderingVersion version)
        : state_(std::move(state)), version_(version) {}

void WebGLRenderingContextBase::UpdateInvalidateState() {
    auto raf = this->GetRaf();
    if (raf != nullptr) {
        if (!canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_start(raf->GetRaf());
        }
    }
    auto state = this->GetInvalidateState();
    this->SetInvalidateState(state | (int) InvalidateState::PENDING);
}

void WebGLRenderingContextBase::Flush() {
    auto state = this->GetInvalidateState();
    if (state == (int) InvalidateState::PENDING) {
        this->SetInvalidateState((int) InvalidateState::INVALIDATING);
        canvas_native_webgl_make_current(*this->state_);
        canvas_native_webgl_swap_buffers(*this->state_);
        this->SetInvalidateState((int) InvalidateState::NONE);
    }
}

void WebGLRenderingContextBase::Flush(intptr_t context) {
    auto ctx = reinterpret_cast<WebGLRenderingContextBase *>(reinterpret_cast<intptr_t *>(context));
    if (ctx != nullptr) {
        ctx->Flush();
    }
}

WebGLState &WebGLRenderingContextBase::GetState() {
    return *this->state_;
}

void WebGLRenderingContextBase::SetRaf(std::shared_ptr<RafImpl> raf) {
    this->raf_ = std::move(raf);
}

RafImpl *WebGLRenderingContextBase::GetRaf() {
    return this->raf_.get();
}

void WebGLRenderingContextBase::SetInvalidateState(int state) {
    this->invalidateState_ = state;
}

int WebGLRenderingContextBase::GetInvalidateState() const {
    return this->invalidateState_;
}

WebGLRenderingVersion WebGLRenderingContextBase::GetVersion() const {
    return this->version_;
}

WebGLRenderingContextBase::~WebGLRenderingContextBase() {

}
