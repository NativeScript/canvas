//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_draw_buffersImpl.h"

WEBGL_draw_buffersImpl::WEBGL_draw_buffersImpl(rust::Box<WEBGL_draw_buffers> buffers) : buffers_(
        std::move(buffers)) {

}

WEBGL_draw_buffersImpl *WEBGL_draw_buffersImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WEBGL_draw_buffersImpl *>(ptr);
}

v8::Local<v8::Object>
WEBGL_draw_buffersImpl::NewInstance(v8::Isolate *isolate, rust::Box<WEBGL_draw_buffers> buffers) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    WEBGL_draw_buffersImpl *buffersImpl = new WEBGL_draw_buffersImpl(std::move(buffers));
    auto result = ctorFunc->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WEBGL_draw_buffers");
    auto ext = v8::External::New(isolate, buffersImpl);
    result->SetInternalField(0, ext);

    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT0_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT0_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT1_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT1_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT2_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT2_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT3_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT3_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT4_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT4_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT5_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT5_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT6_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT6_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT7_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT7_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT8_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT8_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT9_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT9_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT10_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT10_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT11_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT11_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT12_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT12_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT13_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT13_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT14_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT14_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COLOR_ATTACHMENT15_WEBGL"),
                v8::Uint32::New(isolate, GL_COLOR_ATTACHMENT15_EXT));

    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER0_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER0_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER1_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER1_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER2_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER2_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER3_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER3_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER4_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER4_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER5_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER5_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER6_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER6_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER7_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER7_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER8_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER8_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER9_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER9_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER10_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER10_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER11_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER11_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER12_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER12_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER13_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER13_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER14_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER14_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "DRAW_BUFFER15_WEBGL"),
                v8::Uint32::New(isolate, GL_DRAW_BUFFER15_EXT));

    result->Set(context, Helpers::ConvertToV8String(isolate, "MAX_COLOR_ATTACHMENTS_WEBGL"),
                v8::Uint32::New(isolate, GL_MAX_COLOR_ATTACHMENTS_EXT));

    result->Set(context, Helpers::ConvertToV8String(isolate, "MAX_DRAW_BUFFERS_WEBGL"),
                v8::Uint32::New(isolate, GL_MAX_DRAW_BUFFERS_EXT));

    return handle_scope.Escape(result);
}

v8::Local<v8::Function> WEBGL_draw_buffersImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WEBGL_draw_buffersImplCtor.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WEBGL_draw_buffers"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();
    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->Set(Helpers::ConvertToV8String(isolate, "drawBuffersWEBGL"),
              v8::FunctionTemplate::New(isolate, &DrawBuffersWEBGL));

    cache->WEBGL_draw_buffersImplCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

void WEBGL_draw_buffersImpl::DrawBuffersWEBGL(const v8::FunctionCallbackInfo<v8::Value> &args)  {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto buffers = args[0];
    if(buffers->IsArray()){
        auto buffersVal = buffers.As<v8::Array>();
        auto len = buffersVal->Length();
        std::vector<uint32_t> buf;
        for (int j = 0; j < len; ++j) {
            auto item = buffersVal->Get(context,j);
            if(item.IsEmpty()){
                // todo verify
                buf.push_back(0);
            }else {
                buf.push_back(item.ToLocalChecked()->Uint32Value(context).ToChecked());
            }

        }
        rust::Slice<const uint32_t> slice(buf.data(), buf.size());
        canvas_native_webgl_draw_buffers_draw_buffers_webgl(slice,*ptr->buffers_);
    }

}
