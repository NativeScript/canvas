//
// Created by Osei Fortune on 30/04/2022.
//

#include "WebGLRenderingContextBase.h"

WebGLRenderingContextBase::WebGLRenderingContextBase(WebGLState* state,
                                                     WebGLRenderingVersion version)
: state_(state), version_(version) {


    auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(this));
    auto raf_callback = new OnRafCallback(
                                          ctx_ptr,
                                          version == WebGLRenderingVersion::V2 ? 2 : 1);
    auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
    auto raf = canvas_native_raf_create(raf_callback_ptr, OnRafCallbackOnFrame);
    this->SetRaf(
                 std::make_shared<RafImpl>(
                                           raf_callback,
                                           raf_callback_ptr, raf));

    auto _raf = this->GetRaf();

    if (_raf != nullptr) {
        canvas_native_raf_start(_raf->GetRaf());
    }

}


void WebGLRenderingContextBase::GetContinuousRenderMode(v8::Local<v8::String> property,
                                                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    WebGLRenderingContextBase *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(false);
        return;
    }
    info.GetReturnValue().Set(ptr->continuousRender_);
}

void WebGLRenderingContextBase::SetContinuousRenderMode(v8::Local<v8::String> property,
                                                           v8::Local<v8::Value> value,
                                                           const v8::PropertyCallbackInfo<void> &info) {
    WebGLRenderingContextBase *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto val = value->BooleanValue(isolate);
    if (val == ptr->continuousRender_) {
        return;
    }
    if (val) {
        ptr->StartRaf();
    } else {
        ptr->StopRaf();
    }
    ptr->continuousRender_ = val;
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
//    if (raf != nullptr) {
//        if (!canvas_native_raf_get_started(raf->GetRaf())) {
//            canvas_native_raf_start(raf->GetRaf());
//        }
//    }
    auto state = this->GetInvalidateState();
    this->SetInvalidateState(state | (int) InvalidateState::InvalidateStatePending);
}

void WebGLRenderingContextBase::Flush() {
    auto state = this->GetInvalidateState() & (int) InvalidateState::InvalidateStatePending;
    if (state == (int) InvalidateState::InvalidateStatePending) {
        this->SetInvalidateState((int) InvalidateState::InvalidateStateInvalidating);
        canvas_native_webgl_make_current_and_swap_buffers(this->GetState());
        this->SetInvalidateState((int) InvalidateState::InvalidateStateNone);
    }
}

void WebGLRenderingContextBase::Flush(intptr_t context) {
    auto ctx = reinterpret_cast<WebGLRenderingContextBase *>(reinterpret_cast<intptr_t *>(context));
    if (ctx != nullptr) {
        ctx->Flush();
    }
}

WebGLState* WebGLRenderingContextBase::GetState() {
    return this->state_;
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
    canvas_native_raf_release(_raf->GetRaf());
    this->raf_ = nullptr;
    canvas_native_webgl_state_destroy(this->GetState());
    this->state_ = nullptr;
}
