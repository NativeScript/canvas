//
// Created by Osei Fortune on 14/06/2025.
//

#include "GPUCompilationMessageImpl.h"
#include "Caches.h"

GPUCompilationMessageImpl::GPUCompilationMessageImpl(CanvasGPUCompilationMessage *message)
        : message_(
        message) {}

CanvasGPUCompilationMessage *GPUCompilationMessageImpl::GetMessage() {
    return this->message_.get();
}


void GPUCompilationMessageImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUCompilationMessage"),
                      func).FromJust();
}

GPUCompilationMessageImpl *
GPUCompilationMessageImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUCompilationMessageImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUCompilationMessageImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUCompilationMessageTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUCompilationMessage"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "length"),
            GetLength
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "lineNum"),
            GetLineNum
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "linePos"),
            GetLinePos
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "message"),
            GetMessage
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "offset"),
            GetOffset
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "type"),
            GetType
    );

    cache->GPUCompilationMessageTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
GPUCompilationMessageImpl::GetLength(v8::Local<v8::Name> name,
                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto len = canvas_native_webgpu_compilation_message_get_length(ptr->GetMessage());

        info.GetReturnValue().Set((double) len);

        return;
    }

    info.GetReturnValue().Set(0);
}


void
GPUCompilationMessageImpl::GetLineNum(v8::Local<v8::Name> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto num = canvas_native_webgpu_compilation_message_get_line_num(ptr->GetMessage());

        info.GetReturnValue().Set((double) num);

        return;
    }

    info.GetReturnValue().Set(0);
}


void
GPUCompilationMessageImpl::GetLinePos(v8::Local<v8::Name> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto pos = canvas_native_webgpu_compilation_message_get_line_pos(ptr->GetMessage());

        info.GetReturnValue().Set((double) pos);

        return;
    }

    info.GetReturnValue().Set(0);
}

void
GPUCompilationMessageImpl::GetMessage(v8::Local<v8::Name> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto isolate = info.GetIsolate();
        auto len = canvas_native_webgpu_compilation_message_get_message(ptr->GetMessage());

        info.GetReturnValue().Set(ConvertToV8String(isolate, len));

        return;
    }

    info.GetReturnValue().SetEmptyString();
}


void
GPUCompilationMessageImpl::GetOffset(v8::Local<v8::Name> name,
                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto offset = canvas_native_webgpu_compilation_message_get_offset(ptr->GetMessage());

        info.GetReturnValue().Set((double) offset);

        return;
    }

    info.GetReturnValue().Set(0);
}


void
GPUCompilationMessageImpl::GetType(v8::Local<v8::Name> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto message_type = canvas_native_webgpu_compilation_message_get_type(ptr->GetMessage());

        switch (message_type) {
            case CanvasGPUCompilationMessageType::CanvasGPUCompilationMessageTypeInfo:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "info"));
                break;
            case CanvasGPUCompilationMessageType::CanvasGPUCompilationMessageTypeWarning:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "warning"));
                break;
            case CanvasGPUCompilationMessageType::CanvasGPUCompilationMessageTypeError:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "error"));
                break;

        }

        return;
    }

    info.GetReturnValue().Set(ConvertToV8String(isolate, "error"));
}
