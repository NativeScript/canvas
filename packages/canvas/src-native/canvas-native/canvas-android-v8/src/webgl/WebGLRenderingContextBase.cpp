//
// Created by Osei Fortune on 30/04/2022.
//

#include "WebGLRenderingContextBase.h"

WebGLRenderingContextBase::WebGLRenderingContextBase(rust::Box<WebGLState> state) : state_(std::move(state)) {

}

void WebGLRenderingContextBase::UpdateInvalidateState() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        if (!canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_start(raf->GetRaf());
        }
    }

    this->invalidateState_ = InvalidateState::PENDING;
}

InvalidateState WebGLRenderingContextBase::GetInvalidateState() const {
    return this->invalidateState_;
}

void WebGLRenderingContextBase::SetInvalidateState(InvalidateState state) {
    this->invalidateState_ = state;
}

void WebGLRenderingContextBase::Flush() {
    if (this->GetInvalidateState() == InvalidateState::PENDING) {
        this->SetInvalidateState(InvalidateState::INVALIDATING);
        auto current = canvas_native_webgl_make_current(*this->state_);
        auto swapped = canvas_native_webgl_swap_buffers(*this->state_);
        this->SetInvalidateState(InvalidateState::NONE);
    }
}

void WebGLRenderingContextBase::Flush(intptr_t context) {
    auto ctx = reinterpret_cast<WebGLRenderingContextBase *>(reinterpret_cast<intptr_t *>(context));
    if (ctx != nullptr) {
        ctx->Flush();
    }
}

WebGLState &WebGLRenderingContextBase::GetPointer() {
    return *this->state_;
}

void WebGLRenderingContextBase::SetRaf(std::shared_ptr <RafImpl> raf) {
    this->raf_ = raf;
}
