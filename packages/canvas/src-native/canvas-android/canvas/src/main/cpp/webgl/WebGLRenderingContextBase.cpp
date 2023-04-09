//
// Created by Osei Fortune on 30/04/2022.
//

#include "WebGLRenderingContextBase.h"

WebGLRenderingContextBase::WebGLRenderingContextBase(rust::Box<WebGLState> state,
                                                     WebGLRenderingVersion version)
        : state_(std::move(state)), version_(version) {


    auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(this));
    auto raf_callback = new OnRafCallback(
            ctx_ptr,
            version == WebGLRenderingVersion::V2 ? 2 : 1);
    auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
    auto raf = canvas_native_raf_create(
            raf_callback_ptr);
    this->SetRaf(
            std::make_shared<RafImpl>(
                    raf_callback,
                    raf_callback_ptr,
                    std::move(
                            raf)));

    auto _raf = this->GetRaf();

    if (_raf != nullptr) {
        canvas_native_raf_start(_raf->GetRaf());
    }

}


void WebGLRenderingContextBase::StartRaf() {
    auto raf = this->GetRaf();
    if (raf != nullptr) {
        if (!canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_start(raf->GetRaf());
        }
    }
}


void WebGLRenderingContextBase::StopRaf() {
    auto raf = this->GetRaf();
    if (raf != nullptr) {
        if (canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_stop(raf->GetRaf());
        }
    }
}


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
    auto state = this->GetInvalidateState() & (int) InvalidateState::PENDING;
    if (state == (int) InvalidateState::PENDING) {
        this->SetInvalidateState((int) InvalidateState::INVALIDATING);
        canvas_native_webgl_make_current(this->GetState());
        canvas_native_webgl_swap_buffers(this->GetState());
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

void WebGLRenderingContextBase::SetInvalidateState(InvalidateState state) {
    this->invalidateState_ = (int) state;
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
    auto _raf = this->GetRaf();
    if (_raf != nullptr) {
        canvas_native_raf_stop(
                _raf->GetRaf());
    }
}
