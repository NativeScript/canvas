//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLBuffer.h"


v8::Local<v8::Object> WebGLBuffer::NewInstance(v8::Isolate *isolate, uint32_t buffer) {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetPrivate(isolate, result, "instance", v8::Uint32::New(isolate, buffer));
    Helpers::SetInstanceType(isolate, result, ObjectType::WebGLBuffer);
    return handle_scope.Escape(result);
}
