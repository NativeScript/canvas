//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "../Common.h"
#include "../Helpers.h"
#include "../Caches.h"
#include "../RafImpl.h"


class WebGLRenderingContextBase {
public:
    WebGLRenderingContextBase(rust::Box<WebGLState> state);

    ~WebGLRenderingContextBase();

    static WebGLRenderingContextBase *GetPointerBase(const v8::Local<v8::Object> &object) {
        auto ptrValue = object->GetInternalField(0);

        if(ptrValue.IsEmpty()){
            return nullptr;
        }

        void* ptr = ptrValue.As<v8::External>()->Value();

        if (ptr == nullptr) {
            return nullptr;
        }

        return (WebGLRenderingContextBase *) (ptr);
    }

    void UpdateInvalidateState();

    InvalidateState GetInvalidateState() const;

    void SetInvalidateState(InvalidateState state);

    void Flush();

    static void Flush(intptr_t context);

    WebGLState &GetState();

    void SetRaf(std::shared_ptr <RafImpl> raf);

    RafImpl *GetRaf();


private:
    rust::Box<WebGLState> state_;

    InvalidateState invalidateState_ = InvalidateState::NONE;

    std::shared_ptr <RafImpl> raf_ = nullptr;
};

