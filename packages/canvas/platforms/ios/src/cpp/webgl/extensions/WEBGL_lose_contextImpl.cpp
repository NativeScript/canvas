//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_lose_contextImpl.h"


WEBGL_lose_contextImpl::WEBGL_lose_contextImpl(WEBGL_lose_context* context) : context_(context)) {}


void WEBGL_lose_contextImpl::LoseContext(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    WEBGL_lose_contextImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgl_lose_context_lose_context(
            ptr->GetContext());
}


void WEBGL_lose_contextImpl::RestoreContext(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    WEBGL_lose_contextImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgl_lose_context_restore_context(
            ptr->GetContext());
}
