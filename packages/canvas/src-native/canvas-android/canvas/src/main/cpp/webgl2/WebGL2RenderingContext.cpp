//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGL2RenderingContext.h"

WebGL2RenderingContext::WebGL2RenderingContext(rust::Box<WebGLState> state) : WebGLRenderingContext(
        std::move(state), WebGLRenderingVersion::V2) {
}


WebGL2RenderingContext::WebGL2RenderingContext(rust::Box<WebGLState> state,
                                               WebGLRenderingVersion version)
        : WebGLRenderingContext(std::move(state), version) {
}


v8::Local<v8::FunctionTemplate> WebGL2RenderingContext::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGL2RenderingContextTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGL2RenderingContext"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);
    WebGLRenderingContext::SetConstants(isolate, tmpl);
    SetConstants(isolate, tmpl);
    cache->WebGL2RenderingContextTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

WebGL2RenderingContext *WebGL2RenderingContext::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WebGL2RenderingContext *>(ptr);
}

void WebGL2RenderingContext::BeginQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[1];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLQuery) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto query = WebGLQuery::GetPointer(value.As<v8::Object>());

        if(query != nullptr){
            canvas_native_webgl2_begin_query(
                    target,
                    query->GetQuery(),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::BeginTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    if (value->IsNumber()) {
        canvas_native_webgl2_begin_transform_feedback(
                (uint32_t) args[0]->NumberValue(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BindBufferBase(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[2];
    auto type = GetNativeType(isolate, bufferValue);

    if (type == NativeType::WebGLBuffer) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto index = (uint32_t) args[1]->NumberValue(context).ToChecked();

        auto buffer = WebGLBuffer::GetPointer(bufferValue.As<v8::Object>());

        canvas_native_webgl2_bind_buffer_base(
                target,
                index,
                buffer->GetBuffer(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BindBufferRange(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[2];
    auto type = GetNativeType(isolate, bufferValue);
    if (args.Length() > 4 && type == NativeType::WebGLBuffer) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto index = (uint32_t) args[1]->NumberValue(context).ToChecked();
        auto offset = args[3]->NumberValue(context).ToChecked();
        auto size = args[4]->NumberValue(context).ToChecked();
        auto buffer = WebGLBuffer::GetPointer(bufferValue.As<v8::Object>());
        canvas_native_webgl2_bind_buffer_range(
                target,
                index,
                buffer->GetBuffer(),
                static_cast<ssize_t>(offset),
                static_cast<ssize_t>(size),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BindSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto samplerValue = args[1];
    auto type = GetNativeType(isolate, samplerValue);
    if (type == NativeType::WebGLSampler) {
        auto unit = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto sampler = WebGLSampler::GetPointer(samplerValue.As<v8::Object>());

        canvas_native_webgl2_bind_sampler(
                unit,
                sampler->GetSampler(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BindTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto transformFeedbackValue = args[1];
    auto type = GetNativeType(isolate, transformFeedbackValue);

    if (type == NativeType::WebGLTransformFeedback) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto transformFeedback = WebGLTransformFeedback::GetPointer(transformFeedbackValue.As<v8::Object>());

        canvas_native_webgl2_bind_transform_feedback(
                target,
                transformFeedback->GetFeedback(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BindVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    if (value->IsNull()) {
        canvas_native_webgl2_bind_vertex_array(
                0,
                ptr->GetState()
        );
        return;
    }

    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLVertexArrayObject) {
        auto vertexArray = WebGLVertexArrayObject::GetPointer(value.As<v8::Object>());

        if (vertexArray != nullptr) {
            canvas_native_webgl2_bind_vertex_array(
                    vertexArray->GetVertexArrayObject(),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::BlitFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 9) {
        auto srcX0 = (int32_t) args[0]->NumberValue(context).ToChecked();
        auto srcY0 = (int32_t) args[1]->NumberValue(context).ToChecked();

        auto srcX1 = (int32_t) args[2]->NumberValue(context).ToChecked();
        auto srcY1 = (int32_t) args[3]->NumberValue(context).ToChecked();

        auto dstX0 = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto dstY0 = (int32_t) args[5]->NumberValue(context).ToChecked();

        auto dstX1 = (int32_t) args[6]->NumberValue(context).ToChecked();
        auto dstY1 = (int32_t) args[7]->NumberValue(context).ToChecked();

        auto mask = (uint32_t) args[8]->NumberValue(context).ToChecked();
        auto filter = (uint32_t) args[9]->NumberValue(context).ToChecked();
        canvas_native_webgl2_blit_framebuffer(
                srcX0,
                srcY0,
                srcX1,
                srcY1,
                dstX0,
                dstY0,
                dstX1,
                dstY1,
                mask,
                filter,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::ClearBufferfv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[0];
    auto type = GetNativeType(isolate, bufferValue);
    auto values = args[2];
    if (args.Length() > 2 && type == NativeType::WebGLBuffer &&
        args[1]->IsObject()) {
        auto drawbuffer = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto buffer = WebGLBuffer::GetPointer(bufferValue.As<v8::Object>());
        if (values->IsArray()) {
            auto array = values.As<v8::Array>();
            auto len = array->Length();
            rust::Vec<float> buf;
            buf.reserve(len);
            for (int j = 0; j < len; ++j) {
                auto item = array->Get(
                        context, j).ToLocalChecked();
                if (!item->IsNumber()) {
                    buf.push_back(
                            std::nanf(""));
                } else {
                    buf.push_back(
                            static_cast<float>(item->NumberValue(context).ToChecked())
                    );
                }
            }

            rust::Slice<const float> slice(
                    buf.data(),
                    buf.size());
            canvas_native_webgl2_clear_bufferfv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    slice,
                    ptr->GetState()
            );

        } else if (values->IsFloat32Array()) {
            auto buff = values.As<v8::TypedArray>();
            auto slice = GetTypedArrayData<const float>(buff);
            canvas_native_webgl2_clear_bufferfv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    slice,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::ClearBufferiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[0];
    auto type = GetNativeType(isolate, bufferValue);
    auto values = args[2];
    if (args.Length() > 2 && type == NativeType::WebGLBuffer &&
        args[1]->IsObject()) {
        auto drawbuffer = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto buffer = WebGLBuffer::GetPointer(bufferValue.As<v8::Object>());
        if (values->IsArray()) {
            auto array = values.As<v8::Array>();
            auto len = array->Length();
            rust::Vec<int32_t> buf;
            buf.reserve(len);
            for (int j = 0; j < len; ++j) {
                auto item = array->Get(
                        context, j).ToLocalChecked();
                buf.push_back(
                        static_cast<int32_t>(item->NumberValue(context).ToChecked())
                );
            }

            rust::Slice<const int32_t> slice(
                    buf.data(),
                    buf.size());
            canvas_native_webgl2_clear_bufferiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    slice,
                    ptr->GetState()
            );

        } else if (values->IsInt32Array()) {
            auto buff = values.As<v8::TypedArray>();
            auto slice = GetTypedArrayData<const int32_t>(buff);
            canvas_native_webgl2_clear_bufferiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    slice,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::ClearBufferfi(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[0];
    auto type = GetNativeType(isolate, bufferValue);
    if (args.Length() > 3 && type == NativeType::WebGLBuffer) {
        auto buffer = WebGLBuffer::GetPointer(bufferValue.As<v8::Object>());
        if (buffer != nullptr) {
            auto drawbuffer = (int32_t) args[1]->NumberValue(context).ToChecked();
            auto depth = args[2]->NumberValue(context).ToChecked();
            auto stencil = (int32_t) args[3]->NumberValue(context).ToChecked();
            canvas_native_webgl2_clear_bufferfi(
                    buffer->GetBuffer(),
                    drawbuffer,
                    static_cast<float>(depth),
                    stencil,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::ClearBufferuiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[0];
    auto type = GetNativeType(isolate, bufferValue);
    auto values = args[2];
    if (args.Length() > 2 && type == NativeType::WebGLBuffer &&
        args[1]->IsObject()) {
        auto drawbuffer = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto buffer = WebGLBuffer::GetPointer(bufferValue.As<v8::Object>());
        if (values->IsArray()) {
            auto array = values.As<v8::Array>();
            auto len = array->Length();
            rust::Vec<uint32_t> buf;
            buf.reserve(len);
            for (int j = 0; j < len; ++j) {
                auto item = array->Get(
                        context, j).ToLocalChecked();
                buf.push_back(
                        static_cast<uint32_t>(item->NumberValue(context).ToChecked())
                );
            }

            rust::Slice<const uint32_t> slice(
                    buf.data(),
                    buf.size());
            canvas_native_webgl2_clear_bufferuiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    slice,
                    ptr->GetState()
            );

        } else if (values->IsUint32Array()) {
            auto buff = values.As<v8::TypedArray>();
            auto slice = GetTypedArrayData<const uint32_t>(buff);
            canvas_native_webgl2_clear_bufferuiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    slice,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::ClientWaitSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto syncValue = args[0];
    auto type = GetNativeType(isolate, syncValue);
    if (args.Length() > 2 && type == NativeType::WebGLSync ) {
        auto sync = WebGLSyncImpl::GetPointer(syncValue.As<v8::Object>());
        if (sync != nullptr) {
            auto flags = (uint32_t) args[1]->NumberValue(context).ToChecked();
            auto timeout = args[2]->NumberValue(context).ToChecked();
            auto ret = canvas_native_webgl2_client_wait_sync(
                    sync->GetSync(),
                    flags,
                    static_cast<ssize_t>(timeout),
                    ptr->GetState()
            );

            args.GetReturnValue().Set((int32_t) ret);
            return;
        }
    }
    // todo decide if WAIT_FAILED should be returned here
    args.GetReturnValue().SetUndefined();
}

void WebGL2RenderingContext::CompressedTexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 8) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto xoffset = (int32_t) args[2]->NumberValue(context).ToChecked();
        auto yoffset = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto zoffset = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto width = (int32_t) args[5]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[6]->NumberValue(context).ToChecked();
        auto depth = (int32_t) args[7]->NumberValue(context).ToChecked();
        auto format = (uint32_t) args[8]->NumberValue(context).ToChecked();

        auto imageSizeOrBufValue = args[0];
        if (args[9]->IsObject()) {
            if (imageSizeOrBufValue->IsTypedArray()) {
                auto array = imageSizeOrBufValue.As<v8::TypedArray>();
                auto slice = GetTypedArrayData<const uint8_t>(array);

                size_t srcOffset = 0;
                if (args[10]->IsNumber()) {
                    srcOffset = static_cast<size_t>(args[10]->NumberValue(context).ToChecked());
                }
                size_t srcLengthOverride = 0;
                if (args[11]->IsNumber()) {
                    srcLengthOverride = static_cast<size_t>(args[11]->NumberValue(context).ToChecked());
                }


                canvas_native_webgl2_compressed_tex_sub_image3d(
                        target,
                        level,
                        xoffset,
                        yoffset,
                        zoffset,
                        width,
                        height,
                        depth,
                        format,
                        slice,
                        srcOffset,
                        srcLengthOverride,
                        ptr->GetState()
                );
            }
        } else {
            auto imageSizeOrBuf = (int32_t) imageSizeOrBufValue->NumberValue(context).ToChecked();
            auto offset = 0;
            if (args[10]->IsNumber()) {
                offset = (int32_t) args[10]->NumberValue(context).ToChecked();
            }
            canvas_native_webgl2_compressed_tex_sub_image3d_none(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    zoffset,
                    width,
                    height,
                    depth,
                    format,
                    imageSizeOrBuf,
                    offset,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::CopyBufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 4) {
        auto readTarget = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto writeTarget = (uint32_t) args[1]->NumberValue(context).ToChecked();
        auto readOffset = args[2]->NumberValue(context).ToChecked();
        auto writeOffset = args[3]->NumberValue(context).ToChecked();
        auto size = args[4]->NumberValue(context).ToChecked();
        canvas_native_webgl2_copy_buffer_sub_data(
                readTarget,
                writeTarget,
                static_cast<ssize_t>(readOffset),
                static_cast<ssize_t>(writeOffset),
                static_cast<ssize_t>(size),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::CopyTexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 8) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto xoffset = (int32_t) args[2]->NumberValue(context).ToChecked();
        auto yoffset = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto zoffset = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto x = (int32_t) args[5]->NumberValue(context).ToChecked();
        auto y = (int32_t) args[6]->NumberValue(context).ToChecked();
        auto width = (int32_t) args[7]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[8]->NumberValue(context).ToChecked();
        canvas_native_webgl2_copy_tex_sub_image3d(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                x,
                y,
                width,
                height,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::CreateQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto ret = canvas_native_webgl2_create_query(
            ptr->GetState());
    auto query = WebGLQuery::NewInstance(isolate, new WebGLQuery(ret));

    args.GetReturnValue().Set(query);
}

void WebGL2RenderingContext::CreateSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto ret = canvas_native_webgl2_create_sampler(
            ptr->GetState());
    auto sampler = WebGLSampler::NewInstance(isolate, new WebGLSampler(ret));

    args.GetReturnValue().Set(sampler);
}

void WebGL2RenderingContext::CreateTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto ret = canvas_native_webgl2_create_transform_feedback(
            ptr->GetState());

    auto feedback = WebGLTransformFeedback::NewInstance(isolate, new WebGLTransformFeedback(
            ret));

    args.GetReturnValue().Set(feedback);
}

void WebGL2RenderingContext::CreateVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto ret = canvas_native_webgl2_create_vertex_array(
            ptr->GetState());

    auto vertexArrayObject = WebGLVertexArrayObject::NewInstance(isolate, new WebGLVertexArrayObject(ret));

    args.GetReturnValue().Set(vertexArrayObject);
}

void WebGL2RenderingContext::DeleteQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto queryValue = args[0];
    auto type = GetNativeType(isolate, queryValue);
    if (type == NativeType::WebGLQuery) {
        auto query = WebGLQuery::GetPointer(queryValue.As<v8::Object>());

        if (query != nullptr) {
            canvas_native_webgl2_delete_query_with_query(
                    query->GetQuery(),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::DeleteSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto samplerValue = args[0];
    auto type = GetNativeType(isolate, samplerValue);

    if (type == NativeType::WebGLSampler) {
        auto sampler = WebGLSampler::GetPointer(samplerValue.As<v8::Object>());

        if (sampler != nullptr) {
            canvas_native_webgl2_delete_sampler_with_sampler(
                    sampler->GetSampler(),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::DeleteSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto syncValue = args[0];
    auto type = GetNativeType(isolate, syncValue);
    if (type == NativeType::WebGLSync) {
        auto sync = WebGLSyncImpl::GetPointer(syncValue.As<v8::Object>());

        if (sync != nullptr) {
            canvas_native_webgl2_delete_sync_with_sync(
                    sync->GetSync(),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::DeleteTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto transformFeedbackValue = args[0];
    auto type = GetNativeType(isolate, transformFeedbackValue);
    if (type == NativeType::WebGLTransformFeedback) {
        auto transformFeedback = WebGLTransformFeedback::GetPointer(transformFeedbackValue.As<v8::Object>());

        if (transformFeedback != nullptr) {
            canvas_native_webgl2_delete_transform_feedback(
                    transformFeedback->GetFeedback(),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::DeleteVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto vertexArrayValue = args[0];
    auto type = GetNativeType(isolate, vertexArrayValue);
    if (type == NativeType::WebGLVertexArrayObject) {

        auto vertexArray = WebGLVertexArrayObject::GetPointer(vertexArrayValue.As<v8::Object>());
        if (vertexArray != nullptr) {
            canvas_native_webgl2_delete_vertex_array_with_vertex_array(
                    vertexArray->GetVertexArrayObject(),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::DrawArraysInstanced(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
        auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto first = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto count_ = (int32_t) args[2]->NumberValue(context).ToChecked();
        auto instanceCount = (int32_t) args[3]->NumberValue(context).ToChecked();
        canvas_native_webgl2_draw_arrays_instanced(
                mode,
                first,
                count_,
                instanceCount,
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }
}

void WebGL2RenderingContext::DrawBuffers(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto buffersObject = args[0];
    if (buffersObject->IsArray()) {
        auto array = buffersObject.As<v8::Array>();
        auto len = array->Length();
        rust::Vec<uint32_t> buf;
        buf.reserve(len);
        for (int j = 0; j < len; ++j) {
            auto item = array->Get(
                    context, j).ToLocalChecked();
            buf.emplace_back(
                    (uint32_t) item->NumberValue(context).ToChecked());
        }
        rust::Slice<const uint32_t> slice(
                buf.data(), buf.size());
        canvas_native_webgl2_draw_buffers(
                slice,
                ptr->GetState());
    }
}

void WebGL2RenderingContext::DrawElementsInstanced(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 4) {
        auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto count_ = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto type = (uint32_t) args[2]->NumberValue(context).ToChecked();
        auto offset = args[3]->NumberValue(context).ToChecked();
        auto instanceCount = (int32_t) args[4]->NumberValue(context).ToChecked();
        canvas_native_webgl2_draw_elements_instanced(
                mode,
                count_,
                type,
                static_cast<ssize_t>(offset),
                instanceCount,
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }
}

void WebGL2RenderingContext::DrawRangeElements(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 5) {
        auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto start = (uint32_t) args[1]->NumberValue(context).ToChecked();
        auto end = (uint32_t) args[2]->NumberValue(context).ToChecked();
        auto count_ = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto type = (uint32_t) args[4]->NumberValue(context).ToChecked();
        auto offset = args[5]->NumberValue(context).ToChecked();
        canvas_native_webgl2_draw_range_elements(
                mode,
                start,
                end,
                count_,
                type,
                static_cast<ssize_t>(offset),
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }
}

void WebGL2RenderingContext::EndQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args[0]->IsNumber()) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        canvas_native_webgl2_end_query(target,
                                       ptr->GetState());
    }
}

void WebGL2RenderingContext::EndTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    canvas_native_webgl2_end_transform_feedback(
            ptr->GetState());
}

void WebGL2RenderingContext::FenceSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto condition = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto flags = (uint32_t) args[1]->NumberValue(context).ToChecked();
        auto sync = canvas_native_webgl2_fence_sync(
                condition,
                flags,
                ptr->GetState()
        );
        auto ret = WebGLSyncImpl::NewInstance(isolate, new WebGLSyncImpl(std::move(sync)));

        args.GetReturnValue().Set(ret);
    }
}

void WebGL2RenderingContext::FramebufferTextureLayer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto textureValue = args[2];
    auto type = GetNativeType(isolate, textureValue);
    if (args.Length() > 4 && type == NativeType::WebGLTexture) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto attachment = (uint32_t) args[1]->NumberValue(context).ToChecked();
        auto texture = WebGLTexture::GetPointer(textureValue.As<v8::Object>());
        auto level = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto layer = (int32_t) args[4]->NumberValue(context).ToChecked();
        if (texture != nullptr) {
            canvas_native_webgl2_framebuffer_texture_layer(
                    target,
                    attachment,
                    texture->GetTexture(),
                    level,
                    layer,
                    ptr->GetState()
            );
        }

    }
}

void WebGL2RenderingContext::GetActiveUniformBlockName(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetEmptyString();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);
    if (type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        if (program != nullptr) {
            auto uniformBlockIndex = (uint32_t) args[1]->NumberValue(context).ToChecked();
            auto name = canvas_native_webgl2_get_active_uniform_block_name(
                    program->GetProgram(),
                    uniformBlockIndex,
                    ptr->GetState()
            );
            args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, std::move(name)));
            return;
        }

    }

    args.GetReturnValue().SetEmptyString();
}

void WebGL2RenderingContext::GetActiveUniformBlockParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);

    if (args.Length() > 2 && type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());

        if (program != nullptr) {
            auto uniformBlockIndex = (uint32_t) args[1]->NumberValue(context).ToChecked();
            auto pname = (uint32_t) args[2]->NumberValue(context).ToChecked();
            auto ret = canvas_native_webgl2_get_active_uniform_block_parameter(
                    program->GetProgram(),
                    uniformBlockIndex,
                    pname,
                    ptr->GetState()
            );

            switch (pname) {
                case GL_UNIFORM_BLOCK_BINDING:
                case GL_UNIFORM_BLOCK_DATA_SIZE:
                case GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS:
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                            *ret));
                    return;
                case GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: {
                    auto value = canvas_native_webgl_result_get_u32_array(
                            *ret);

                    auto buf = new VecMutableBuffer<uint32_t>(
                            std::move(value));



                    auto store = v8::ArrayBuffer::NewBackingStore(buf->data(), buf->buffer_size(),
                                                                  [](void *data, size_t length,
                                                                     void *deleter_data) {
                                                                      if (deleter_data != nullptr) {
                                                                          delete (VecMutableBuffer<float> *) deleter_data;
                                                                      }
                                                                  },
                                                                  buf);

                    auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));
                    args.GetReturnValue().Set(v8::Uint32Array::New(arraybuffer, 0, buf->size()));
                    return;
                }
                case GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER:
                case GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                            *ret));
                    return;
                default:
                    args.GetReturnValue().SetNull();
                    return;
            }
        }
    }
}

void WebGL2RenderingContext::GetActiveUniforms(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);
    if (args.Length() > 2 && type == NativeType::WebGLProgram) {

        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        auto uniformIndicesObject = args[1];
        auto pname = (uint32_t) args[2]->NumberValue(context).ToChecked();

        if (uniformIndicesObject->IsArray()) {
            auto uniformIndices = uniformIndicesObject.As<v8::Array>();
            auto size = uniformIndices->Length();
            rust::Vec<uint32_t> buf;
            buf.reserve(size);
            for (int j = 0; j < size; j++) {
                auto item = (uint32_t) uniformIndices->Get(
                        context, j).ToLocalChecked()->NumberValue(context).ToChecked();
                buf.emplace_back(item);
            }
            rust::Slice<const uint32_t> slice(
                    buf.data(), buf.size());
            auto ret = canvas_native_webgl2_get_active_uniforms(
                    program->GetProgram(),
                    slice,
                    pname,
                    ptr->GetState()
            );

            switch (pname) {
                case GL_UNIFORM_TYPE:
                case GL_UNIFORM_SIZE: {
                    auto value = canvas_native_webgl_result_get_u32_array(
                            *ret);
                    auto len = value.size();
                    auto array = v8::Array::New(
                            isolate, (int)len);

                    for (int j = 0;
                         j < len; ++j) {
                        auto item = value[j];
                        array->Set(
                                context, j,
                                v8::Number::New(isolate, (double) item));
                    }
                    args.GetReturnValue().Set(array);
                    return;
                }
                case GL_UNIFORM_BLOCK_INDEX:
                case GL_UNIFORM_OFFSET:
                case GL_UNIFORM_ARRAY_STRIDE:
                case GL_UNIFORM_MATRIX_STRIDE: {
                    auto value = canvas_native_webgl_result_get_i32_array(
                            *ret);
                    auto len = value.size();
                    auto array = v8::Array::New(
                            isolate, (int) len);
                    for (int j = 0;
                         j < len; ++j) {
                        auto item = value[j];
                        array->Set(
                                context, j,
                                v8::Number::New(isolate,
                                                (double )item));
                    }
                    args.GetReturnValue().Set(array);
                    return;
                }
                case GL_UNIFORM_IS_ROW_MAJOR: {
                    auto value = canvas_native_webgl_result_get_bool_array(
                            *ret);
                    auto len = value.size();
                    auto array = v8::Array::New(
                            isolate,
                            (int)len);
                    for (int j = 0;
                         j < len; ++j) {
                        bool item =
                                value[j] == 1;
                        array->Set(
                                context, j,
                                v8::Boolean::New(isolate, item));
                    }
                    args.GetReturnValue().Set(array);
                    return;
                }
                default:
                    args.GetReturnValue().SetNull();
                    return;
            }
        }


    }
}

std::vector<jsi::PropNameID> WebGL2RenderingContext::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;

    // gl2 + gl1 props
    ret.reserve(353 + 434);

    // 351
    ret.push_back(jsi::PropNameID::forAscii(rt, "beginQuery"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "beginTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "bindBufferBase"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "bindBufferRange"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "bindSampler"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "bindTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "bindVertexArray"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "blitFramebuffer"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "clearBufferfi"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "clearBufferfv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "clearBufferiv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "clearBufferuiv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "clientWaitSync"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "compressedTexSubImage3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "copyBufferSubData"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "copyTexSubImage3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "createQuery"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "createSampler"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "createTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "createVertexArray"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "deleteQuery"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "deleteSampler"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "deleteSync"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "deleteTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "deleteVertexArray"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "drawArraysInstanced"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "drawBuffers"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "drawElementsInstanced"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "drawRangeElements"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "endQuery"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "endTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "fenceSync"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "framebufferTextureLayer"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform1ui"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform1uiv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform2ui"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform2uiv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform3ui"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform3uiv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform4ui"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniform4uiv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniformBlockBinding"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix2x3fv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix2x4fv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix3x2fv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix3x4fv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix4x2fv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix4x3fv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribDivisor"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4i"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4iv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4ui"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4uiv"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getActiveUniformBlockName"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getActiveUniformBlockParameter"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getActiveUniforms"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getBufferSubData"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getFragDataLocation"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getIndexedParameter"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getInternalformatParameter"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getParameter"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getQueryParameter"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getQuery"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getSamplerParameter"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getSyncParameter"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getTransformFeedbackVarying"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getUniformBlockIndex"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "getUniformIndices"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "invalidateFramebuffer"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "invalidateSubFramebuffer"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "isQuery"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "isSampler"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "isSync"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "isTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "isVertexArray"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "pauseTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "readBuffer"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "renderbufferStorageMultisample"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "resumeTransformFeedback"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "samplerParameterf"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "samplerParameteri"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "texImage3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "texStorage2D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "texStorage3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "texSubImage3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "transformFeedbackVaryings"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "READ_BUFFER"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_ROW_LENGTH"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_SKIP_ROWS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_SKIP_PIXELS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "PACK_ROW_LENGTH"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "PACK_SKIP_ROWS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "PACK_SKIP_PIXELS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_BINDING_3D"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_SKIP_IMAGES"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_IMAGE_HEIGHT"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_3D_TEXTURE_SIZE"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_ELEMENTS_VERTICES"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_ELEMENTS_INDICES"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_TEXTURE_LOD_BIAS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_FRAGMENT_UNIFORM_COMPONENTS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_VERTEX_UNIFORM_COMPONENTS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_ARRAY_TEXTURE_LAYERS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MIN_PROGRAM_TEXEL_OFFSET"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_PROGRAM_TEXEL_OFFSET"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_VARYING_COMPONENTS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAGMENT_SHADER_DERIVATIVE_HINT"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "RASTERIZER_DISCARD"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "VERTEX_ARRAY_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_VERTEX_OUTPUT_COMPONENTS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_FRAGMENT_INPUT_COMPONENTS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_SERVER_WAIT_TIMEOUT"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_ELEMENT_INDEX"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "RED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB10_A2"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_3D"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_WRAP_R"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_MIN_LOD"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_MAX_LOD"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_BASE_LEVEL"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_MAX_LEVEL"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_COMPARE_MODE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_COMPARE_FUNC"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SRGB"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SRGB8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SRGB8_ALPHA8"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "COMPARE_REF_TO_TEXTURE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA32F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB32F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA16F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB16F"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_2D_ARRAY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_BINDING_2D_ARRAY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R11F_G11F_B10F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB9_E5"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA32UI"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB32UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA16UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB16UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA8UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB8UI"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA32I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB32I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA16I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB16I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA8I"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB8I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RED_INTEGER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB_INTEGER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA_INTEGER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R8"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "RG8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R16F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R32F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG16F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG32F"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "R8I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R8UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R16I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R16UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R32I"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "R32UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG8I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG8UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG16I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG16UI"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "RG32I"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG32UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "R8_SNORM"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG8_SNORM"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB8_SNORM"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "RGBA8_SNORM"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RGB10_A2UI"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_IMMUTABLE_FORMAT"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_IMMUTABLE_LEVELS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_2_10_10_10_REV"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_10F_11F_11F_REV"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_5_9_9_9_REV"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_32_UNSIGNED_INT_24_8_REV"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_24_8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "HALF_FLOAT"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "RG"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RG_INTEGER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "INT_2_10_10_10_REV"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "QUERY_RESULT_AVAILABLE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "QUERY_RESULT"));


    ret.push_back(jsi::PropNameID::forAscii(rt, "CURRENT_QUERY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "ANY_SAMPLES_PASSED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "ANY_SAMPLES_PASSED_CONSERVATIVE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_DRAW_BUFFERS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER0"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER1"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER2"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER3)"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER4"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER5"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER6"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER7"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER9"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER10"));

    /* Getting GL parameter information */

    /* Textures */

    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER11"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER12"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER13"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER14"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER15"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_COLOR_ATTACHMENTS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT1"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT2"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT3"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT4"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT5"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT6"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT7"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT9"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT10"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT11"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT12"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT13"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT14"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT15"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_2D_SHADOW"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_2D_ARRAY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_2D_ARRAY_SHADOW"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_CUBE_SHADOW"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_2D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_CUBE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_2D_ARRAY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_2D"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_3D"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_CUBE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_2D_ARRAY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_SAMPLES"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_BINDING"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_PACK_BUFFER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_UNPACK_BUFFER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_PACK_BUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_UNPACK_BUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COPY_READ_BUFFER"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "COPY_WRITE_BUFFER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COPY_READ_BUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "COPY_WRITE_BUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT2x3"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT2x4"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT3x2"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT3x4"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT4x2"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT4x3"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_VEC2"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_VEC3"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_VEC4"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_NORMALIZED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SIGNED_NORMALIZED"));

    /* Vertex attributes */

    ret.push_back(jsi::PropNameID::forAscii(rt, "VERTEX_ATTRIB_ARRAY_INTEGER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "VERTEX_ATTRIB_ARRAY_DIVISOR"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_MODE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_VARYINGS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_START"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS"));

    /* Textures */

    /* Pixel types */

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "INTERLEAVED_ATTRIBS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SEPARATE_ATTRIBS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_BINDING"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_PAUSED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_ACTIVE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BINDING"));

    /* Pixel types */

    /* Queries */

    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_RED_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_GREEN_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_BLUE_SIZE"));

    /* Queries */

    /* Draw buffers */

    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_DEFAULT"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_STENCIL_ATTACHMENT"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_STENCIL"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DEPTH24_STENCIL8"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_FRAMEBUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "READ_FRAMEBUFFER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DRAW_FRAMEBUFFER"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "READ_FRAMEBUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "RENDERBUFFER_SAMPLES"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_START"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_VERTEX_UNIFORM_BLOCKS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_FRAGMENT_UNIFORM_BLOCKS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_COMBINED_UNIFORM_BLOCKS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_UNIFORM_BUFFER_BINDINGS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_UNIFORM_BLOCK_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_OFFSET_ALIGNMENT"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "ACTIVE_UNIFORM_BLOCKS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_TYPE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_INDEX"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_OFFSET"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_ARRAY_STRIDE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_MATRIX_STRIDE"));

    /* Draw buffers */

    /* Samplers */

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_IS_ROW_MAJOR"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_BINDING"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_DATA_SIZE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_ACTIVE_UNIFORMS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "OBJECT_TYPE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SYNC_CONDITION"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SYNC_STATUS"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "SYNC_FLAGS"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SYNC_FENCE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SYNC_GPU_COMMANDS_COMPLETE"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNALED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SIGNALED"));

    /* Samplers */

    /* Buffers */

    ret.push_back(jsi::PropNameID::forAscii(rt, "ALREADY_SIGNALED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TIMEOUT_EXPIRED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "CONDITION_SATISFIED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "WAIT_FAILED"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "SYNC_FLUSH_COMMANDS_BIT"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "COLOR"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DEPTH"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "STENCIL"));

    /* Buffers */

    /* Data types */

    ret.push_back(jsi::PropNameID::forAscii(rt, "MIN"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_COMPONENT24"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "STREAM_READ"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "STREAM_COPY"));

    ret.push_back(jsi::PropNameID::forAscii(rt, "STATIC_READ"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "STATIC_COPY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DYNAMIC_READ"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DYNAMIC_COPY"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_COMPONENT32F"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "DEPTH32F_STENCIL8"));

    /* Data types */

    ret.push_back(jsi::PropNameID::forAscii(rt, "INVALID_INDEX"));
    ret.push_back(jsi::PropNameID::forAscii(rt, "TIMEOUT_IGNORED"));

    /* Vertex attributes */

    /* Transform feedback */

    ret.push_back(jsi::PropNameID::forAscii(rt, "MAX_CLIENT_WAIT_TIMEOUT_WEBGL"));

    /* Transform feedback */


    /* GL 1 */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "__resized"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "__startRaf"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "__stopRaf"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("activeTexture")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("attachShader")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bindAttribLocation")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bindBuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bindFramebuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bindRenderbuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bindTexture")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("blendColor")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("blendEquationSeparate")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("blendEquation")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("blendFuncSeparate")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("blendFunc")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bufferData")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bufferSubData")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("checkFramebufferStatus")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("clearColor")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("clearDepth")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("clearStencil")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("clear")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("colorMask")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("commit")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("compileShader")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("compressedTexImage2D")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("compressedTexSubImage2D")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("copyTexImage2D")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("copyTexSubImage2D")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createBuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createFramebuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createProgram")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createRenderbuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createShader")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createTexture")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("cullFace")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("deleteBuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("deleteFramebuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("deleteProgram")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("deleteRenderbuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("deleteShader")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("deleteTexture")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("depthFunc")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("depthMask")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("depthRange")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("detachShader")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("disableVertexAttribArray")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("disable")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("drawArrays")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("drawElements")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("enableVertexAttribArray")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("enable")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("finish")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("flush")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("framebufferRenderbuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("framebufferTexture2D")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("frontFace")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("generateMipmap")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getActiveAttrib")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getActiveUniform")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getAttachedShaders")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getAttribLocation")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getBufferParameter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getContextAttributes")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getError")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getExtension")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getFramebufferAttachmentParameter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getParameter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getProgramInfoLog")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getProgramParameter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getRenderbufferParameter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getShaderInfoLog")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getShaderParameter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getShaderPrecisionFormat")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getShaderSource")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getSupportedExtensions")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getTexParameter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getUniformLocation")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getUniform")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getVertexAttribOffset")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getVertexAttrib")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("hint")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isBuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isContextLost")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isEnabled")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isFramebuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isProgram")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isRenderbuffer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isShader")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isTexture")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineWidth")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("linkProgram")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("pixelStorei")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("polygonOffset")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("readPixels")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("renderbufferStorage")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("sampleCoverage")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("scissor")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("shaderSource")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("stencilFuncSeparate")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("stencilFunc")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("stencilMaskSeparate")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("stencilMask")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("stencilOpSeparate")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("stencilOp")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("texImage2D")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("texParameterf")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("texParameteri")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("texSubImage2D")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform1f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform1iv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform1fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform1i")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform2f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform2iv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform2fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform2i")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform3f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform3iv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform3fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform3i")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform4f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform4iv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform4fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniform4i")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniformMatrix2fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniformMatrix3fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("uniformMatrix4fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("useProgram")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("validateProgram")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib1f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib1fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib2f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib2fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib3f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib3fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib4f")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib4fv")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttribPointer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("viewport")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("__toDataURL")));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_BUFFER_BIT"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BUFFER_BIT"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "COLOR_BUFFER_BIT"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "POINTS"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINES"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINE_LOOP"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINE_STRIP"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "TRIANGLES"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "TRIANGLE_STRIP"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "TRIANGLE_FAN"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ZERO"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ONE"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "SRC_COLOR"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ONE_MINUS_SRC_COLOR"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "SRC_ALPHA"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ONE_MINUS_SRC_ALPHA"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "DST_ALPHA"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ONE_MINUS_DST_ALPHA"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "DST_COLOR"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ONE_MINUS_DST_COLOR"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "SRC_ALPHA_SATURATE"));


    ret.push_back(jsi::PropNameID::forUtf8(rt, "CONSTANT_COLOR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ONE_MINUS_CONSTANT_COLOR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CONSTANT_ALPHA"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ONE_MINUS_CONSTANT_ALPHA"));


    /* Blending equations */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "FUNC_ADD"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FUNC_SUBTRACT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FUNC_REVERSE_SUBTRACT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_EQUATION"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_EQUATION_RGB"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_EQUATION_ALPHA"));


    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_DST_RGB"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_SRC_RGB"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_DST_ALPHA"));


    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_SRC_ALPHA"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND_COLOR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ARRAY_BUFFER_BINDING"));


    ret.push_back(jsi::PropNameID::forUtf8(rt, "ELEMENT_ARRAY_BUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINE_WIDTH"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ALIASED_POINT_SIZE_RANGE"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ALIASED_LINE_WIDTH_RANGE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CULL_FACE_MODE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRONT_FACE"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_RANGE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_WRITEMASK"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_CLEAR_VALUE"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_FUNC"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_CLEAR_VALUE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_FUNC"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_FAIL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_PASS_DEPTH_FAIL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_PASS_DEPTH_PASS"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_REF"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_VALUE_MASK"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_WRITEMASK"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_FUNC"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_FAIL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_PASS_DEPTH_FAIL"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_PASS_DEPTH_PASS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_REF"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_VALUE_MASK"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_WRITEMASK"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "VIEWPORT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SCISSOR_BOX"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "COLOR_CLEAR_VALUE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "COLOR_WRITEMASK"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNPACK_ALIGNMENT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "PACK_ALIGNMENT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_TEXTURE_SIZE"));


    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_VIEWPORT_DIMS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SUBPIXEL_BITS"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "RED_BITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "GREEN_BITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLUE_BITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ALPHA_BITS"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_BITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "POLYGON_OFFSET_UNITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "POLYGON_OFFSET_FACTOR"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_BINDING_2D"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLE_BUFFERS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLES"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLE_COVERAGE_VALUE"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLE_COVERAGE_INVERT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "COMPRESSED_TEXTURE_FORMATS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VENDOR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERER"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERSION"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "IMPLEMENTATION_COLOR_READ_TYPE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "IMPLEMENTATION_COLOR_READ_FORMAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BROWSER_DEFAULT_WEBGL"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "STATIC_DRAW"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STREAM_DRAW"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DYNAMIC_DRAW"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ARRAY_BUFFER"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ELEMENT_ARRAY_BUFFER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BUFFER_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BUFFER_USAGE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CURRENT_VERTEX_ATTRIB"));


    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_ENABLED"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_STRIDE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_TYPE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_NORMALIZED"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_POINTER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_BUFFER_BINDING"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "CULL_FACE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRONT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BACK"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRONT_AND_BACK"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "BLEND"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_TEST"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DITHER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "POLYGON_OFFSET_FILL"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLE_ALPHA_TO_COVERAGE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLE_COVERAGE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SCISSOR_TEST"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_TEST"));


    /* Errors */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "NO_ERROR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INVALID_ENUM"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INVALID_VALUE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INVALID_OPERATION"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "OUT_OF_MEMORY"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CONTEXT_LOST_WEBGL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CW"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CCW"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "DONT_CARE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FASTEST"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "NICEST"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "GENERATE_MIPMAP_HINT"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "BYTE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNSIGNED_BYTE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SHORT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "INT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNSIGNED_INT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FLOAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_COMPONENT"));

    ret.push_back(jsi::PropNameID::forUtf8(rt, "ALPHA"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RGB"));

    /* Clearing buffers */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "RGBA"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LUMINANCE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LUMINANCE_ALPHA"));

    /* Clearing buffers */

    /* Rendering primitives */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT_4_4_4_4"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT_5_5_5_1"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT_5_6_5"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAGMENT_SHADER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VERTEX_SHADER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "COMPILE_STATUS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DELETE_STATUS"));

    /* Rendering primitives */

    /* Blending modes */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINK_STATUS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "VALIDATE_STATUS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ATTACHED_SHADERS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ACTIVE_ATTRIBUTES"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ACTIVE_UNIFORMS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_VERTEX_ATTRIBS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_VERTEX_UNIFORM_VECTORS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_VARYING_VECTORS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_COMBINED_TEXTURE_IMAGE_UNITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_VERTEX_TEXTURE_IMAGE_UNITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_TEXTURE_IMAGE_UNITS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_FRAGMENT_UNIFORM_VECTORS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SHADER_TYPE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SHADING_LANGUAGE_VERSION"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CURRENT_PROGRAM"));

    /* Blending modes */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "NEVER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LESS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "EQUAL"));

    /* Blending equations */

    /* Getting GL parameter information */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "LEQUAL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "GREATER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "NOTEQUAL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "GEQUAL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ALWAYS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "KEEP"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "REPLACE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INCR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DECR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INVERT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INCR_WRAP"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DECR_WRAP"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "NEAREST"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINEAR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "NEAREST_MIPMAP_NEAREST"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINEAR_MIPMAP_NEAREST"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "NEAREST_MIPMAP_LINEAR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LINEAR_MIPMAP_LINEAR"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_MAG_FILTER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_MIN_FILTER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_WRAP_S"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_WRAP_T"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_2D"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_BINDING_CUBE_MAP"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_POSITIVE_X"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_NEGATIVE_X"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_POSITIVE_Y"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_NEGATIVE_Y"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_POSITIVE_Z"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_NEGATIVE_Z"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_CUBE_MAP_TEXTURE_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE0"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE1"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE2"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE3"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE4"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE5"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE6"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE7"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE8"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE9"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE10"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE11"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE12"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE13"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE14"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE15"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE16"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE17"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE18"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE19"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE20"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE21"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE22"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE23"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE24"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE25"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE26"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE27"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE28"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE29"));

    /* Getting GL parameter information */

    /* Buffers */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE30"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "TEXTURE31"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "ACTIVE_TEXTURE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "REPEAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "CLAMP_TO_EDGE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MIRRORED_REPEAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FLOAT_VEC2"));

    /* Buffers */

    /* Vertex attributes */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "FLOAT_VEC3"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FLOAT_VEC4"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INT_VEC2"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INT_VEC3"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INT_VEC4"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BOOL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BOOL_VEC2"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "BOOL_VEC3"));

    /* Vertex attributes */

    /* Culling */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "BOOL_VEC4"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FLOAT_MAT2"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FLOAT_MAT3"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FLOAT_MAT4"));

    /* Culling */

    /* Enabling and disabling */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLER_2D"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "SAMPLER_CUBE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LOW_FLOAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MEDIUM_FLOAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "HIGH_FLOAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "LOW_INT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MEDIUM_INT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "HIGH_INT"));

    /* Enabling and disabling */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RGBA4"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RGB5_A1"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RGB565"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_COMPONENT16"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_INDEX8"));

    /* Errors */

    /* Front face directions */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_STENCIL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_WIDTH"));

    /* Front face directions */

    /* Hints */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_HEIGHT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_INTERNAL_FORMAT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_RED_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_GREEN_SIZE"));

    /* Hints */

    /* Data types */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_BLUE_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_ALPHA_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_DEPTH_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_STENCIL_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_OBJECT_NAME"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL"));

    /* Data types */

    /* Pixel formats */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "COLOR_ATTACHMENT0"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_ATTACHMENT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "STENCIL_ATTACHMENT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "DEPTH_STENCIL_ATTACHMENT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "NONE"));

    /* Pixel formats */

    /* Pixel types */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_COMPLETE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_INCOMPLETE_ATTACHMENT"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT"));

    /* Pixel types */

    /* Shaders */

    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_INCOMPLETE_DIMENSIONS"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_UNSUPPORTED"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_BINDING"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "MAX_RENDERBUFFER_SIZE"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "INVALID_FRAMEBUFFER_OPERATION"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNPACK_FLIP_Y_WEBGL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNPACK_PREMULTIPLY_ALPHA_WEBGL"));
    ret.push_back(jsi::PropNameID::forUtf8(rt, "UNPACK_COLORSPACE_CONVERSION_WEBGL"));


    /* GL 1 */


    return ret;
}

jsi::Value WebGL2RenderingContext::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    auto prop = GetProperty(methodName);


   if (methodName == "getBufferSubData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count >= 3 &&
                                                             args[2]->IsObject()) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto srcByteOffset = args[1]->NumberValue(context).ToChecked();
                                                             auto dstDataObject = args[2].asObject(
                                                                     runtime);

                                                             if (dstDataObject.isTypedArray(
                                                                     runtime)) {

                                                                 auto array = dstDataObject.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<uint8_t>(
                                                                         runtime, array);

                                                                 auto BYTES_PER_ELEMENT = (ssize_t) array.getProperty(
                                                                         runtime,
                                                                         "BYTES_PER_ELEMENT")->NumberValue(context).ToChecked();

                                                                 ssize_t dstOffsetValue = 0;
                                                                 if (args[3]->IsNumber()) {
                                                                     dstOffsetValue =
                                                                             static_cast<ssize_t>(args[3]->NumberValue(context).ToChecked()) *
                                                                             BYTES_PER_ELEMENT;
                                                                 }

                                                                 ssize_t lengthValue = 0;
                                                                 if (args[4]->IsNumber()) {
                                                                     lengthValue =
                                                                             static_cast<ssize_t>(args[4]->NumberValue(context).ToChecked()) *
                                                                             BYTES_PER_ELEMENT;
                                                                 }

                                                                 canvas_native_webgl2_get_buffer_sub_data(
                                                                         target,
                                                                         static_cast<ssize_t>(srcByteOffset),
                                                                         slice,
                                                                         dstOffsetValue,
                                                                         lengthValue,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getFragDataLocation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1 && args[1].isString()) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, args[0]);

                                                             if (program != nullptr) {
                                                                 auto name = args[1].asString(
                                                                         runtime).utf8(runtime);

                                                                 auto ret = canvas_native_webgl2_get_frag_data_location(
                                                                         program->GetProgram(),
                                                                         rust::Str(name.c_str()),
                                                                         ptr->GetState()
                                                                 );

                                                                 return {ret};
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getIndexedParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto index = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto ret = canvas_native_webgl2_get_indexed_parameter(
                                                                     target,
                                                                     index,
                                                                     ptr->GetState()
                                                             );

                                                             switch (target) {
                                                                 case GL_UNIFORM_BUFFER_BINDING:
                                                                 case GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: {
                                                                     auto buffer = canvas_native_webgl2_indexed_parameter_get_buffer_value(
                                                                             *ret);
                                                                     return jsi::Object::createFromHostObject(
                                                                             runtime,
                                                                             std::make_shared<WebGLBuffer>(
                                                                                     buffer));
                                                                 }
                                                                     break;
                                                                 case GL_TRANSFORM_FEEDBACK_BUFFER_SIZE:
                                                                 case GL_TRANSFORM_FEEDBACK_BUFFER_START:
                                                                 case GL_UNIFORM_BUFFER_SIZE:
                                                                 case GL_UNIFORM_BUFFER_START: {
                                                                     auto value = canvas_native_webgl2_indexed_parameter_get_value(
                                                                             *ret);
                                                                     return {static_cast<double>(value)};
                                                                 }
                                                                     break;
                                                                 default:
                                                                     return jsi::Value::null();
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getInternalformatParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 2) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto internalformat = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto pname = (uint32_t) args[2]->NumberValue(context).ToChecked();
                                                             switch (internalformat) {
                                                                 case GL_RGB:
                                                                 case GL_RGBA:
                                                                 case GL_R8UI:
                                                                 case GL_R8I:
                                                                 case GL_R16UI:
                                                                 case GL_R16I:
                                                                 case GL_R32UI:
                                                                 case GL_R32I:
                                                                 case GL_RG8UI:
                                                                 case GL_RG8I:
                                                                 case GL_RG16UI:
                                                                 case GL_RG16I:
                                                                 case GL_RG32UI:
                                                                 case GL_RG32I:
                                                                 case GL_RGBA8UI:
                                                                 case GL_RGBA8I:
                                                                 case GL_RGB10_A2UI:
                                                                 case GL_RGBA16UI:
                                                                 case GL_RGBA16I:
                                                                 case GL_RGBA32UI:
                                                                 case GL_RGBA32I: {
                                                                     // empty

                                                                     auto value = rust::Vec<int32_t>();
                                                                     auto buffer = std::make_shared<VecMutableBuffer<int32_t>>(
                                                                             std::move(value));
                                                                     auto array = jsi::ArrayBuffer(
                                                                             runtime, buffer);

                                                                     auto Int32Array = runtime.global()
                                                                             .getProperty(runtime,
                                                                                          "Int32Array")
                                                                             .asObject(runtime)
                                                                             .asFunction(runtime);


                                                                     return Int32Array.callAsConstructor(
                                                                             runtime, array);
                                                                 }
                                                                 case GL_R8:
                                                                 case GL_RG8:
                                                                 case GL_RGB565:
                                                                 case GL_RGBA8:
                                                                 case GL_SRGB8_ALPHA8:
                                                                 case GL_RGB5_A1:
                                                                 case GL_RGBA4:
                                                                 case GL_RGB10_A2:
                                                                 case GL_DEPTH_COMPONENT16:
                                                                 case GL_DEPTH_COMPONENT24:
                                                                 case GL_DEPTH_COMPONENT32F:
                                                                 case GL_DEPTH24_STENCIL8:
                                                                 case GL_DEPTH32F_STENCIL8:
                                                                 case GL_STENCIL_INDEX8:
                                                                     // noop
                                                                     break;
                                                                 case GL_R16F:
                                                                 case GL_RG16F:
                                                                 case GL_R32F:
                                                                 case GL_RG32F:
                                                                 case GL_RGBA32F:
                                                                 case GL_R11F_G11F_B10F:
                                                                     // noop
                                                                     break;
                                                                 default:
                                                                     return jsi::Value::null();
                                                             }


                                                             auto ret = canvas_native_webgl2_get_internalformat_parameter(
                                                                     target,
                                                                     internalformat,
                                                                     pname,
                                                                     ptr->GetState()
                                                             );

                                                             if (pname == GL_SAMPLES) {
                                                                 auto value = canvas_native_webgl_result_get_i32_array(
                                                                         *ret);

                                                                 auto buffer = std::make_shared<VecMutableBuffer<int32_t>>(
                                                                         std::move(value));
                                                                 auto array = jsi::ArrayBuffer(
                                                                         runtime,
                                                                         buffer);

                                                                 auto Int32Array = runtime.global()
                                                                         .getProperty(runtime,
                                                                                      "Int32Array")
                                                                         .asObject(runtime)
                                                                         .asFunction(runtime);


                                                                 return Int32Array.callAsConstructor(
                                                                         runtime, array);
                                                             } else {
                                                                 return jsi::Value::null();
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto pname = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto result = canvas_native_webgl2_get_parameter(
                                                                     pname,
                                                                     ptr->GetState());
                                                             switch (pname) {
                                                                 case GL_COPY_READ_BUFFER_BINDING:
                                                                 case GL_COPY_WRITE_BUFFER_BINDING:
                                                                 case GL_DRAW_FRAMEBUFFER_BINDING:
                                                                     return {canvas_native_webgl_result_get_i32(
                                                                             *result)};
                                                                 default:
                                                                     return this->GetParameterInternal(
                                                                             runtime, pname,
                                                                             std::move(result));
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getQueryParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto query = getHostObject<WebGLQuery>(
                                                                     runtime, args[0]);
                                                             if (query != nullptr) {
                                                                 auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();


                                                                 auto ret = canvas_native_webgl2_get_query_parameter(
                                                                         query->GetQuery(),
                                                                         pname,
                                                                         ptr->GetState());
                                                                 if (pname == GL_QUERY_RESULT) {
                                                                     return {canvas_native_webgl_result_get_bool(
                                                                             *ret)};
                                                                 } else if (pname ==
                                                                            GL_QUERY_RESULT_AVAILABLE) {
                                                                     return {(int32_t) canvas_native_webgl_result_get_u32(
                                                                             *ret)};
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getQuery") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto ret = canvas_native_webgl2_get_query(
                                                                     target,
                                                                     pname,
                                                                     ptr->GetState());
                                                             if (pname == GL_CURRENT_QUERY) {
                                                                 return {canvas_native_webgl_result_get_i32(
                                                                         *ret)};
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getSamplerParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto sampler = getHostObject<WebGLSampler>(
                                                                     runtime, args[0]);
                                                             if (sampler != nullptr) {
                                                                 auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                                 auto ret = canvas_native_webgl2_get_sampler_parameter(
                                                                         sampler->GetSampler(),
                                                                         pname,
                                                                         ptr->GetState());

                                                                 switch (pname) {
                                                                     case GL_TEXTURE_MAX_LOD:
                                                                     case GL_TEXTURE_MIN_LOD:
                                                                         return {static_cast<double>(canvas_native_webgl_result_get_f32(
                                                                                 *ret))};
                                                                     case GL_TEXTURE_COMPARE_FUNC:
                                                                     case GL_TEXTURE_COMPARE_MODE:
                                                                     case GL_TEXTURE_MAG_FILTER:
                                                                     case GL_TEXTURE_MIN_FILTER:
                                                                     case GL_TEXTURE_WRAP_R:
                                                                     case GL_TEXTURE_WRAP_S:
                                                                     case GL_TEXTURE_WRAP_T:
                                                                         return {canvas_native_webgl_result_get_i32(
                                                                                 *ret)};
                                                                     default:
                                                                         return jsi::Value::null();
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getSyncParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto sync = getHostObject<WebGLSyncImpl>(
                                                                     runtime, args[0]);
                                                             if (sync != nullptr) {
                                                                 auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                                 auto ret = canvas_native_webgl2_get_sync_parameter(
                                                                         sync->GetSync(),
                                                                         pname,
                                                                         ptr->GetState());

                                                                 switch (pname) {
                                                                     case GL_OBJECT_TYPE:
                                                                     case GL_SYNC_STATUS:
                                                                     case GL_SYNC_CONDITION:
                                                                     case GL_SYNC_FLAGS:
                                                                         return {canvas_native_webgl_result_get_i32(
                                                                                 *ret)};
                                                                     default:
                                                                         return jsi::Value::null();
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getTransformFeedbackVarying") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 1) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, args[0]);
                                                             auto index = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             if (program != nullptr) {
                                                                 auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                                                                         program->GetProgram(),
                                                                         index,
                                                                         ptr->GetState()
                                                                 );

                                                                 if (canvas_native_webgl_active_info_get_is_empty(
                                                                         *ret)) {
                                                                     return jsi::Value::null();
                                                                 }

                                                                 auto info = std::make_shared<WebGLActiveInfoImpl>(
                                                                         std::move(ret));

                                                                 return jsi::Object::createFromHostObject(
                                                                         runtime, info);
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getUniformBlockIndex") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 1) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, args[0]);
                                                             auto index = args[1].asString(
                                                                     runtime).utf8(runtime);
                                                             if (program != nullptr) {
                                                                 auto ret = canvas_native_webgl2_get_uniform_block_index(
                                                                         program->GetProgram(),
                                                                         rust::Str(index.c_str()),
                                                                         ptr->GetState());

                                                                 return {(int) ret};
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getUniformIndices") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[1]->IsObject()) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, args[0]);
                                                             auto uniformNamesObject = args[1].asObject(
                                                                     runtime);
                                                             if (program != nullptr &&
                                                                 uniformNamesObject.isArray(
                                                                         runtime)) {
                                                                 auto uniformNames = uniformNamesObject.getArray(
                                                                         runtime);
                                                                 auto len = uniformNames.size(
                                                                         runtime);
                                                                 rust::Vec<rust::Str> store;
                                                                 store.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto name = uniformNames.getValueAtIndex(
                                                                             runtime, j).asString(
                                                                             runtime).utf8(runtime);
                                                                     rust::Str val(name.data(),
                                                                                   name.size());
                                                                     store.push_back(val);
                                                                 }
                                                                 rust::Slice<const rust::Str> slice(
                                                                         store.data(),
                                                                         store.size());
                                                                 auto ret = canvas_native_webgl2_get_uniform_indices(
                                                                         program->GetProgram(),
                                                                         slice,
                                                                         ptr->GetState());

                                                                 auto retSize = ret.size();
                                                                 auto result = jsi::Array(runtime,
                                                                                          retSize);
                                                                 for (int j = 0; j < retSize; ++j) {
                                                                     auto item = ret[j];
                                                                     result.setValueAtIndex(runtime,
                                                                                            j,
                                                                                            jsi::Value(
                                                                                                    (int32_t) item));
                                                                 }

                                                                 return result;
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "invalidateFramebuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1 && args[1]->IsObject()) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto attachments = args[1].asObject(
                                                                     runtime);

                                                             if (attachments.isArray(runtime)) {
                                                                 auto array = attachments.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, j)->NumberValue(context).ToChecked();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl2_invalidate_framebuffer(
                                                                         target, slice,
                                                                         ptr->GetState());
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "invalidateSubFramebuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 5 && args[1]->IsObject()) {
                                                             auto attachments = args[1].asObject(
                                                                     runtime);
                                                             if (attachments.isArray(runtime)) {
                                                                 auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                                 auto x = (int32_t) args[2]->NumberValue(context).ToChecked();
                                                                 auto y = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                                 auto width = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                                 auto height = (int32_t) args[5]->NumberValue(context).ToChecked();

                                                                 auto array = attachments.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto item = (uint) array.getValueAtIndex(
                                                                             runtime, j)->NumberValue(context).ToChecked();
                                                                     buf.push_back(item);
                                                                 }
                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_invalidate_sub_framebuffer(
                                                                         target,
                                                                         slice,
                                                                         x,
                                                                         y,
                                                                         width,
                                                                         height,
                                                                         ptr->GetState());
                                                             }


                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "isQuery") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 0) {
                                                             auto query = getHostObject<WebGLQuery>(
                                                                     runtime, args[0]);
                                                             if (query != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_query(
                                                                         query->GetQuery(),
                                                                         ptr->GetState());
                                                                 return {ret};
                                                             }
                                                         }
                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isSampler") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 0) {
                                                             auto sampler = getHostObject<WebGLSampler>(
                                                                     runtime, args[0]);
                                                             if (sampler != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_sampler(
                                                                         sampler->GetSampler(),
                                                                         ptr->GetState());
                                                                 return {ret};
                                                             }
                                                         }
                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 0) {
                                                             auto sync = getHostObject<WebGLSyncImpl>(
                                                                     runtime, args[0]);
                                                             if (sync != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_sync(
                                                                         sync->GetSync(),
                                                                         ptr->GetState());
                                                                 return {ret};
                                                             }
                                                         }
                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 0) {
                                                             auto transformFeedback = getHostObject<WebGLTransformFeedback>(
                                                                     runtime, args[0]);
                                                             if (transformFeedback != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_transform_feedback(
                                                                         transformFeedback->GetFeedback(),
                                                                         ptr->GetState());
                                                                 return {ret};
                                                             }
                                                         }
                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isVertexArray") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 0) {
                                                             auto vertexArray = getHostObject<WebGLVertexArrayObject>(
                                                                     runtime, args[0]);
                                                             if (vertexArray != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_vertex_array(
                                                                         vertexArray->GetVertexArrayObject(),
                                                                         ptr->GetState());
                                                                 return {ret};
                                                             }
                                                         }
                                                         // todo check return
                                                         return {false};


                                                     }
        );
    } else if (methodName == "pauseTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         canvas_native_webgl2_pause_transform_feedback(
                                                                 ptr->GetState());

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "readBuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 0) {
                                                             auto src = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             canvas_native_webgl2_read_buffer(
                                                                     src,
                                                                     ptr->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "renderbufferStorageMultisample") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto samples = (int32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto internalFormat = (uint32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto width = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto height = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                             canvas_native_webgl2_renderbuffer_storage_multisample(
                                                                     target,
                                                                     samples,
                                                                     internalFormat,
                                                                     width,
                                                                     height,
                                                                     ptr->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "resumeTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         canvas_native_webgl2_resume_transform_feedback(
                                                                 ptr->GetState());
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "samplerParameterf") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto sampler = getHostObject<WebGLSampler>(
                                                                     runtime, args[0]);
                                                             auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto param = args[2]->NumberValue(context).ToChecked();
                                                             if (sampler != nullptr) {
                                                                 canvas_native_webgl2_sampler_parameterf(
                                                                         sampler->GetSampler(),
                                                                         pname,
                                                                         static_cast<float>(param),
                                                                         ptr->GetState());
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "samplerParameteri") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto sampler = getHostObject<WebGLSampler>(
                                                                     runtime, args[0]);
                                                             auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto param = (int32_t) args[2]->NumberValue(context).ToChecked();
                                                             if (sampler != nullptr) {
                                                                 canvas_native_webgl2_sampler_parameteri(
                                                                         sampler->GetSampler(),
                                                                         pname,
                                                                         param,
                                                                         ptr->GetState());
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "texImage3D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 10) {
                                                             auto target = (int32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto internalformat = (int32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto width = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto height = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                             auto depth = (int32_t) args[5]->NumberValue(context).ToChecked();
                                                             auto border = (int32_t) args[6]->NumberValue(context).ToChecked();
                                                             auto format = (int32_t) args[7]->NumberValue(context).ToChecked();
                                                             auto type = (uint32_t) args[8]->NumberValue(context).ToChecked();


                                                             if (args[9]->IsNumber()) {
                                                                 auto imageOrPixelsOrOffset = args[9]->NumberValue(context).ToChecked();
                                                                 canvas_native_webgl2_tex_image3d_none(
                                                                         target,
                                                                         level,
                                                                         internalformat,
                                                                         width,
                                                                         height,
                                                                         depth,
                                                                         border,
                                                                         format,
                                                                         type,
                                                                         static_cast<ssize_t>(imageOrPixelsOrOffset),
                                                                         ptr->GetState()
                                                                 );
                                                                 return jsi::Value::undefined();
                                                             }

                                                             if (args[9]->IsObject()) {
                                                                 auto imageOrPixelsOrOffsetObject = args[9].asObject(
                                                                         runtime);

                                                                 if (imageOrPixelsOrOffsetObject.isTypedArray(
                                                                         runtime)) {
                                                                     auto buf = imageOrPixelsOrOffsetObject.getTypedArray(
                                                                             runtime);
                                                                     auto slice = GetTypedArrayData<const uint8_t>(
                                                                             runtime, buf);


                                                                     canvas_native_webgl2_tex_image3d(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             width,
                                                                             height,
                                                                             depth,
                                                                             border,
                                                                             format,
                                                                             type,
                                                                             slice,
                                                                             ptr->GetState()
                                                                     );
                                                                     return jsi::Value::undefined();
                                                                 }

                                                                 /*
                                                                 try {
                                                                     auto image_asset = getHostObject<ImageAssetImpl>(
                                                                             runtime, args[9]);
                                                                     if (image_asset != nullptr) {
                                                                         canvas_native_webgl2_tex_image3d_asset(
                                                                                 target,
                                                                                 level,
                                                                                 internalformat,
                                                                                 width,
                                                                                 height,
                                                                                 depth,
                                                                                 border,
                                                                                 format,
                                                                                 type,
                                                                                 image_asset->GetImageAsset(),
                                                                                 ptr->GetState()
                                                                         );

                                                                         return jsi::Value::undefined();
                                                                     }
                                                                 } catch (...) {}

                                                                 try {
                                                                     auto image_bitmap = getHostObject<ImageBitmapImpl>(
                                                                             runtime, args[9]);
                                                                     if (image_bitmap != nullptr) {
                                                                         canvas_native_webgl2_tex_image3d_asset(
                                                                                 target,
                                                                                 level,
                                                                                 internalformat,
                                                                                 width,
                                                                                 height,
                                                                                 depth,
                                                                                 border,
                                                                                 format,
                                                                                 type,
                                                                                 image_bitmap->GetImageAsset(),
                                                                                 ptr->GetState()
                                                                         );

                                                                         return jsi::Value::undefined();
                                                                     }
                                                                 } catch (...) {}
*/

                                                             }
                                                         } else if (count > 10) {

                                                             auto target = (int32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto internalformat = (int32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto width = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto height = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                             auto depth = (int32_t) args[5]->NumberValue(context).ToChecked();
                                                             auto border = (int32_t) args[6]->NumberValue(context).ToChecked();
                                                             auto format = (int32_t) args[7]->NumberValue(context).ToChecked();
                                                             auto type = (uint32_t) args[8]->NumberValue(context).ToChecked();
                                                             if (args[9]->IsObject()) {
                                                                 auto imageOrPixelsOrOffset = args[9].asObject(
                                                                         runtime);
                                                                 size_t srcOffsetValue = 0;
                                                                 if (args[9]->IsNumber()) {
                                                                     srcOffsetValue = static_cast<size_t>(args[9]->NumberValue(context).ToChecked());
                                                                 }

                                                                 if (imageOrPixelsOrOffset.isTypedArray(
                                                                         runtime)) {
                                                                     auto buf = imageOrPixelsOrOffset.getTypedArray(
                                                                             runtime);
                                                                     auto size = buf.size(runtime);
                                                                     auto array = GetTypedArrayData<const uint8_t>(
                                                                             runtime, buf);

                                                                     srcOffsetValue =
                                                                             srcOffsetValue * size;
                                                                     if (srcOffsetValue >
                                                                         size) {
                                                                         return jsi::Value::undefined();
                                                                     }

                                                                     canvas_native_webgl2_tex_image3d_offset(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             width,
                                                                             height,
                                                                             depth,
                                                                             border,
                                                                             format,
                                                                             type,
                                                                             array,
                                                                             srcOffsetValue,
                                                                             ptr->GetState()
                                                                     );
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             }

                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "texStorage2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 4) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto levels = (int32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto internalFormat = (uint32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto width = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto height = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                             canvas_native_webgl2_tex_storage2d(
                                                                     target,
                                                                     levels,
                                                                     internalFormat,
                                                                     width,
                                                                     height,
                                                                     ptr->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "texStorage3D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 5) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto levels = (int32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto internalFormat = (uint32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto width = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto height = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                             auto depth = (int32_t) args[5]->NumberValue(context).ToChecked();
                                                             canvas_native_webgl2_tex_storage3d(
                                                                     target,
                                                                     levels,
                                                                     internalFormat,
                                                                     width,
                                                                     height,
                                                                     depth,
                                                                     ptr->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "texSubImage3D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     12,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count == 11) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto xoffset = (int32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto yoffset = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto zoffset = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                             auto width = (int32_t) args[5]->NumberValue(context).ToChecked();
                                                             auto height = (int32_t) args[6]->NumberValue(context).ToChecked();
                                                             auto depth = (int32_t) args[7]->NumberValue(context).ToChecked();
                                                             auto format = (uint32_t) args[8]->NumberValue(context).ToChecked();
                                                             auto type = (int32_t) args[9]->NumberValue(context).ToChecked();

                                                             if (args[10]->IsNumber()) {
                                                                 auto imageOrPixelsOrOffset = args[10]->NumberValue(context).ToChecked();
                                                                 canvas_native_webgl2_tex_sub_image3d_none(
                                                                         target,
                                                                         level,
                                                                         xoffset,
                                                                         yoffset,
                                                                         zoffset,
                                                                         width,
                                                                         height,
                                                                         depth,
                                                                         format,
                                                                         type,
                                                                         static_cast<ssize_t>(imageOrPixelsOrOffset),
                                                                         ptr->GetState()
                                                                 );
                                                                 return jsi::Value::undefined();
                                                             }

                                                             if (args[10]->IsObject()) {
                                                                 auto imageOrPixelsOrOffsetObject = args[10].asObject(
                                                                         runtime);
                                                                 if (imageOrPixelsOrOffsetObject.isTypedArray(
                                                                         runtime)) {
                                                                     auto array = imageOrPixelsOrOffsetObject.getTypedArray(
                                                                             runtime);
                                                                     auto slice = GetTypedArrayData<const uint8_t>(
                                                                             runtime, array);

                                                                     canvas_native_webgl2_tex_sub_image3d(
                                                                             target,
                                                                             level,
                                                                             xoffset,
                                                                             yoffset,
                                                                             zoffset,
                                                                             width,
                                                                             height,
                                                                             depth,
                                                                             format,
                                                                             type,
                                                                             slice,
                                                                             ptr->GetState()
                                                                     );

                                                                     return jsi::Value::undefined();
                                                                 }

                                                                 /*
                                                                 auto asset = getHostObject<ImageAssetImpl>(
                                                                         runtime, args[10]);
                                                                 if (asset != nullptr) {

                                                                     canvas_native_webgl2_tex_sub_image3d_asset(
                                                                             target,
                                                                             level,
                                                                             xoffset,
                                                                             yoffset,
                                                                             zoffset,
                                                                             width,
                                                                             height,
                                                                             depth,
                                                                             format,
                                                                             type,
                                                                             asset->GetImageAsset(),
                                                                             ptr->GetState()
                                                                     );
                                                                 }

                                                                  */
                                                             }

                                                         } else if (count > 11) {
                                                             auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto xoffset = (int32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto yoffset = (int32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto zoffset = (int32_t) args[4]->NumberValue(context).ToChecked();
                                                             auto width = (int32_t) args[5]->NumberValue(context).ToChecked();
                                                             auto height = (int32_t) args[6]->NumberValue(context).ToChecked();
                                                             auto depth = (int32_t) args[7]->NumberValue(context).ToChecked();
                                                             auto format = (uint32_t) args[8]->NumberValue(context).ToChecked();
                                                             auto type = (uint32_t) args[9]->NumberValue(context).ToChecked();

                                                             size_t srcOffsetValue = 0;
                                                             if (args[11]->IsNumber()) {
                                                                 srcOffsetValue = static_cast<size_t>(args[11]->NumberValue(context).ToChecked());
                                                             }

                                                             if (args[10]->IsObject()) {
                                                                 auto imageOrPixelsOrOffsetObject = args[10].asObject(
                                                                         runtime);

                                                                 if (imageOrPixelsOrOffsetObject.isTypedArray(
                                                                         runtime)) {
                                                                     auto array = imageOrPixelsOrOffsetObject.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<uint8_t>(
                                                                             runtime, array);
                                                                     auto size = array.size(
                                                                             runtime);
                                                                     srcOffsetValue =
                                                                             srcOffsetValue * size;
                                                                     if (srcOffsetValue > size) {
                                                                         return jsi::Value::undefined();
                                                                     }

                                                                     rust::Slice<const uint8_t> slice(
                                                                             buf.data(),
                                                                             buf.size());

                                                                     canvas_native_webgl2_tex_sub_image3d_offset(
                                                                             target,
                                                                             level,
                                                                             xoffset,
                                                                             yoffset,
                                                                             zoffset,
                                                                             width,
                                                                             height,
                                                                             depth,
                                                                             format,
                                                                             type,
                                                                             slice,
                                                                             srcOffsetValue,
                                                                             ptr->GetState()
                                                                     );
                                                                 }

                                                             }
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "transformFeedbackVaryings") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 2 && args[1]->IsObject()) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, args[0]);
                                                             auto varyingsObject = args[1].asObject(
                                                                     runtime);
                                                             auto bufferMode = (uint32_t) args[2]->NumberValue(context).ToChecked();

                                                             if (program != nullptr &&
                                                                 varyingsObject.isArray(runtime)) {
                                                                 auto varyings = varyingsObject.getArray(
                                                                         runtime);
                                                                 auto len = varyings.size(runtime);
                                                                 rust::Vec<rust::Str> buf;
                                                                 buf.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto name = varyings.getValueAtIndex(
                                                                             runtime, j).asString(
                                                                             runtime).utf8(runtime);
                                                                     rust::Str val(name.data(),
                                                                                   name.size());
                                                                     buf.emplace_back(val);
                                                                 }

                                                                 rust::Slice<const rust::Str> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_transform_feedback_varyings(
                                                                         program->GetProgram(),
                                                                         slice,
                                                                         bufferMode,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform1ui") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[1]->IsNumber()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto v0 = (uint32_t) args[1]->NumberValue(context).ToChecked();

                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform1ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform1uiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[1]->IsObject()) {
                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto data = args[1].asObject(
                                                                     runtime);
                                                             if (location != nullptr &&
                                                                 data.isUint32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform1uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, i)->NumberValue(context).ToChecked();
                                                                     buf.push_back(item);
                                                                 }
                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl2_uniform1uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }


                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "uniform2ui") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto v0 = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto v1 = (uint32_t) args[2]->NumberValue(context).ToChecked();

                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform2ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         v1,
                                                                         ptr->GetState()
                                                                 );
                                                             }


                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform2uiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[1]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto data = args[1].asObject(
                                                                     runtime);

                                                             if (data.isUint32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform2uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, i)->NumberValue(context).ToChecked();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl2_uniform2uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "uniform3ui") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 3) {
                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto v0 = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto v1 = (uint32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto v2 = (uint32_t) args[3]->NumberValue(context).ToChecked();
                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform3ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         v1,
                                                                         v2,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform3uiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[1]->IsObject()) {
                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto data = args[1].asObject(
                                                                     runtime);

                                                             if (data.isUint32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform3uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i)->NumberValue(context).ToChecked();
                                                                     buf.push_back((uint32_t) item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform3uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "uniform4ui") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto v0 = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto v1 = (uint32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto v2 = (uint32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto v3 = (uint32_t) args[4]->NumberValue(context).ToChecked();
                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform4ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         v1,
                                                                         v2,
                                                                         v3,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform4uiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1 && args[1]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto data = args[1].asObject(
                                                                     runtime);

                                                             if (data.isUint32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform4uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i)->NumberValue(context).ToChecked();
                                                                     buf.push_back((uint32_t) item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform4uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "uniformBlockBinding") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 2) {

                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, args[0]);
                                                             auto uniformBlockIndex = args[1]->NumberValue(context).ToChecked();
                                                             auto uniformBlockBinding = args[2]->NumberValue(context).ToChecked();

                                                             if (program != nullptr) {
                                                                 canvas_native_webgl2_uniform_block_binding(
                                                                         program->GetProgram(),
                                                                         (uint32_t) uniformBlockIndex,
                                                                         (uint32_t) uniformBlockBinding,
                                                                         ptr->GetState()
                                                                 );
                                                             }

                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniformMatrix2x3fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && args[2]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto transpose = args[1].asBool();
                                                             auto data = args[2].asObject(
                                                                     runtime);

                                                             if (data.isFloat32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const float>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform_matrix2x3fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item->IsNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item->NumberValue(context).ToChecked()));
                                                                     } else {
                                                                         buf.push_back(
                                                                                 std::nanf(""));
                                                                     }
                                                                 }

                                                                 rust::Slice<const float> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform_matrix2x3fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "uniformMatrix2x4fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && args[2]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto transpose = args[1].asBool();
                                                             auto data = args[2].asObject(
                                                                     runtime);

                                                             if (data.isFloat32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const float>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform_matrix2x4fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item->IsNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item->NumberValue(context).ToChecked()));
                                                                     } else {
                                                                         buf.push_back(
                                                                                 std::nanf(""));
                                                                     }
                                                                 }
                                                                 rust::Slice<const float> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform_matrix2x4fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniformMatrix3x2fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && args[2]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto transpose = args[1].asBool();
                                                             auto data = args[2].asObject(
                                                                     runtime);

                                                             if (data.isFloat32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const float>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform_matrix3x2fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item->IsNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item->NumberValue(context).ToChecked()));
                                                                     } else {
                                                                         buf.push_back(
                                                                                 std::nanf(""));
                                                                     }
                                                                 }

                                                                 rust::Slice<const float> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform_matrix3x2fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniformMatrix3x4fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 2 && args[2]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto transpose = args[1].asBool();
                                                             auto data = args[2].asObject(
                                                                     runtime);

                                                             if (data.isFloat32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const float>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform_matrix3x4fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item->IsNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item->NumberValue(context).ToChecked()));
                                                                     } else {
                                                                         buf.push_back(
                                                                                 std::nanf(""));
                                                                     }
                                                                 }

                                                                 rust::Slice<const float> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform_matrix3x4fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniformMatrix4x2fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 2 && args[2]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto transpose = args[1].asBool();
                                                             auto data = args[2].asObject(
                                                                     runtime);

                                                             if (data.isFloat32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const float>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform_matrix4x2fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item->IsNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item->NumberValue(context).ToChecked()));
                                                                     } else {
                                                                         buf.push_back(
                                                                                 std::nanf(""));
                                                                     }
                                                                 }

                                                                 rust::Slice<const float> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform_matrix4x2fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "uniformMatrix4x3fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 2 && args[2]->IsObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             auto transpose = args[1].asBool();
                                                             auto data = args[2].asObject(
                                                                     runtime);

                                                             if (data.isFloat32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const float>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform_matrix4x3fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item->IsNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item->NumberValue(context).ToChecked()));
                                                                     } else {
                                                                         buf.push_back(
                                                                                 std::nanf(""));
                                                                     }
                                                                 }

                                                                 rust::Slice<const float> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform_matrix4x3fv(
                                                                         location->GetUniformLocation(),
                                                                         transpose,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttribDivisor") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 1) {
                                                             auto index = args[0]->NumberValue(context).ToChecked();
                                                             auto divisor = args[1]->NumberValue(context).ToChecked();
                                                             canvas_native_webgl2_vertex_attrib_divisor(
                                                                     (uint32_t) index,
                                                                     (uint32_t) divisor,
                                                                     ptr->GetState()
                                                             );
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttribI4i") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4) {
                                                             auto index = args[0]->NumberValue(context).ToChecked();
                                                             auto v0 = args[1]->NumberValue(context).ToChecked();
                                                             auto v1 = args[2]->NumberValue(context).ToChecked();
                                                             auto v2 = args[3]->NumberValue(context).ToChecked();
                                                             auto v3 = args[4]->NumberValue(context).ToChecked();
                                                             canvas_native_webgl2_vertex_attrib_i4i(
                                                                     (uint32_t) index,
                                                                     (int32_t) v0,
                                                                     (int32_t) v1,
                                                                     (int32_t) v2,
                                                                     (int32_t) v3,
                                                                     ptr->GetState()
                                                             );
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttribI4iv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[1]->IsObject()) {
                                                             auto index = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto value = args[1].asObject(
                                                                     runtime);
                                                             if (value.isInt32Array(runtime)) {
                                                                 auto array = value.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const int32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_vertex_attrib_i4iv(
                                                                         index,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (value.isArray(runtime)) {
                                                                 auto array = value.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<int32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (int32_t) array.getValueAtIndex(
                                                                             runtime, i)->NumberValue(context).ToChecked();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const int32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_vertex_attrib_i4iv(
                                                                         index,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttribI4ui") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4) {
                                                             auto index = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto v0 = (uint32_t) args[1]->NumberValue(context).ToChecked();
                                                             auto v1 = (uint32_t) args[2]->NumberValue(context).ToChecked();
                                                             auto v2 = (uint32_t) args[3]->NumberValue(context).ToChecked();
                                                             auto v3 = (uint32_t) args[4]->NumberValue(context).ToChecked();

                                                             canvas_native_webgl2_vertex_attrib_i4ui(
                                                                     index,
                                                                     v0,
                                                                     v1,
                                                                     v2,
                                                                     v3,
                                                                     ptr->GetState()
                                                             );
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttribI4uiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1 && args[1]->IsObject()) {
                                                             auto index = (uint32_t) args[0]->NumberValue(context).ToChecked();
                                                             auto value = args[1].asObject(
                                                                     runtime);
                                                             if (value.isUint32Array(runtime)) {
                                                                 auto array = value.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_vertex_attrib_i4uiv(
                                                                         index,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             } else if (value.isArray(runtime)) {
                                                                 auto array = value.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, i)->NumberValue(context).ToChecked();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_vertex_attrib_i4uiv(
                                                                         index,
                                                                         slice,
                                                                         ptr->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    }

    if (!prop.isUndefined()) {
        return prop;
    }
    auto gl_return = WebGLRenderingContext::get(runtime, name);
    if (!gl_return.isUndefined()) {
        return gl_return;
    }
    return jsi::Value::undefined();
}


void WebGL2RenderingContext::SetConstants(v8::Isolate *isolate,
                                          const v8::Local<v8::ObjectTemplate> &tmpl) {
    /* Getting GL parameter information */

    tmpl->Set(isolate, "READ_BUFFER", v8::Uint32::New(isolate, 0x0C02));

    tmpl->Set(isolate, "UNPACK_ROW_LENGTH", v8::Uint32::New(isolate, 0x0CF2));

    tmpl->Set(isolate, "UNPACK_SKIP_ROWS", v8::Uint32::New(isolate, 0x0CF3));

    tmpl->Set(isolate, "UNPACK_SKIP_PIXELS", v8::Uint32::New(isolate, 0x0CF4));

    tmpl->Set(isolate, "PACK_ROW_LENGTH", v8::Uint32::New(isolate, 0x0D02));

    tmpl->Set(isolate, "PACK_SKIP_ROWS", v8::Uint32::New(isolate, 0x0D03));

    tmpl->Set(isolate, "PACK_SKIP_PIXELS", v8::Uint32::New(isolate, 0x0D04));

    tmpl->Set(isolate, "TEXTURE_BINDING_3D", v8::Uint32::New(isolate, 0x806A));

    tmpl->Set(isolate, "UNPACK_SKIP_IMAGES", v8::Uint32::New(isolate, 0x806D));

    tmpl->Set(isolate, "UNPACK_IMAGE_HEIGHT", v8::Uint32::New(isolate, 0x806E));

    tmpl->Set(isolate, "MAX_3D_TEXTURE_SIZE", v8::Uint32::New(isolate, 0x8073));

    tmpl->Set(isolate, "MAX_ELEMENTS_VERTICES", v8::Uint32::New(isolate, 0x80E8));

    tmpl->Set(isolate, "MAX_ELEMENTS_INDICES", v8::Uint32::New(isolate, 0x80E9));

    tmpl->Set(isolate, "MAX_TEXTURE_LOD_BIAS", v8::Uint32::New(isolate, 0x84FD));

    tmpl->Set(isolate, "MAX_FRAGMENT_UNIFORM_COMPONENTS", v8::Uint32::New(isolate, 0x8B49));

    tmpl->Set(isolate, "MAX_VERTEX_UNIFORM_COMPONENTS", v8::Uint32::New(isolate, 0x8B4A));

    tmpl->Set(isolate, "MAX_ARRAY_TEXTURE_LAYERS", v8::Uint32::New(isolate, 0x88FF));

    tmpl->Set(isolate, "MIN_PROGRAM_TEXEL_OFFSET", v8::Uint32::New(isolate, 0x8904));

    tmpl->Set(isolate, "MAX_PROGRAM_TEXEL_OFFSET", v8::Uint32::New(isolate, 0x8905));

    tmpl->Set(isolate, "MAX_VARYING_COMPONENTS", v8::Uint32::New(isolate, 0x8B4B));

    tmpl->Set(isolate, "FRAGMENT_SHADER_DERIVATIVE_HINT", v8::Uint32::New(isolate, 0x8B8B));

    tmpl->Set(isolate, "RASTERIZER_DISCARD", v8::Uint32::New(isolate, 0x8C89));

    tmpl->Set(isolate, "VERTEX_ARRAY_BINDING", v8::Uint32::New(isolate, 0x85B5));
    tmpl->Set(isolate, "MAX_VERTEX_OUTPUT_COMPONENTS", v8::Uint32::New(isolate, 0x9122));
    tmpl->Set(isolate, "MAX_FRAGMENT_INPUT_COMPONENTS", v8::Uint32::New(isolate, 0x9125));
    tmpl->Set(isolate, "MAX_SERVER_WAIT_TIMEOUT", v8::Uint32::New(isolate, 0x9111));
    tmpl->Set(isolate, "MAX_ELEMENT_INDEX", v8::Uint32::New(isolate, 0x8D6B));

    tmpl->Set(isolate, "RED", v8::Uint32::New(isolate, 0x1903));
    tmpl->Set(isolate, "RGB8", v8::Uint32::New(isolate, 0x8051));
    tmpl->Set(isolate, "RGBA8", v8::Uint32::New(isolate, 0x8058));
    tmpl->Set(isolate, "RGB10_A2", v8::Uint32::New(isolate, 0x8059));
    tmpl->Set(isolate, "TEXTURE_3D", v8::Uint32::New(isolate, 0x806F));

    tmpl->Set(isolate, "TEXTURE_WRAP_R", v8::Uint32::New(isolate, 0x8072));
    tmpl->Set(isolate, "TEXTURE_MIN_LOD", v8::Uint32::New(isolate, 0x813A));
    tmpl->Set(isolate, "TEXTURE_MAX_LOD", v8::Uint32::New(isolate, 0x813B));
    tmpl->Set(isolate, "TEXTURE_BASE_LEVEL", v8::Uint32::New(isolate, 0x813C));
    tmpl->Set(isolate, "TEXTURE_MAX_LEVEL", v8::Uint32::New(isolate, 0x813D));


    tmpl->Set(isolate, "TEXTURE_COMPARE_MODE", v8::Uint32::New(isolate, 0x884C));
    tmpl->Set(isolate, "TEXTURE_COMPARE_FUNC", v8::Uint32::New(isolate, 0x884D));
    tmpl->Set(isolate, "SRGB", v8::Uint32::New(isolate, 0x8C40));
    tmpl->Set(isolate, "SRGB8", v8::Uint32::New(isolate, 0x8C41));
    tmpl->Set(isolate, "SRGB8_ALPHA8", v8::Uint32::New(isolate, 0x8C43));

    tmpl->Set(isolate, "COMPARE_REF_TO_TEXTURE", v8::Uint32::New(isolate, 0x884E));
    tmpl->Set(isolate, "RGBA32F", v8::Uint32::New(isolate, 0x8814));
    tmpl->Set(isolate, "RGB32F", v8::Uint32::New(isolate, 0x8815));
    tmpl->Set(isolate, "RGBA16F", v8::Uint32::New(isolate, 0x881A));
    tmpl->Set(isolate, "RGB16F", v8::Uint32::New(isolate, 0x881B));

    tmpl->Set(isolate, "TEXTURE_2D_ARRAY", v8::Uint32::New(isolate, 0x8C1A));
    tmpl->Set(isolate, "TEXTURE_BINDING_2D_ARRAY", v8::Uint32::New(isolate, 0x8C1D));
    tmpl->Set(isolate, "R11F_G11F_B10F", v8::Uint32::New(isolate, 0x8C3A));
    tmpl->Set(isolate, "RGB9_E5", v8::Uint32::New(isolate, 0x8C3D));
    tmpl->Set(isolate, "RGBA32UI", v8::Uint32::New(isolate, 0x8D70));


    tmpl->Set(isolate, "RGB32UI", v8::Uint32::New(isolate, 0x8D71));
    tmpl->Set(isolate, "RGBA16UI", v8::Uint32::New(isolate, 0x8D76));
    tmpl->Set(isolate, "RGB16UI", v8::Uint32::New(isolate, 0x8D77));
    tmpl->Set(isolate, "RGBA8UI", v8::Uint32::New(isolate, 0x8D7C));
    tmpl->Set(isolate, "RGB8UI", v8::Uint32::New(isolate, 0x8D7D));


    tmpl->Set(isolate, "RGBA32I", v8::Uint32::New(isolate, 0x8D82));
    tmpl->Set(isolate, "RGB32I", v8::Uint32::New(isolate, 0x8D83));
    tmpl->Set(isolate, "RGBA16I", v8::Uint32::New(isolate, 0x8D88));
    tmpl->Set(isolate, "RGB16I", v8::Uint32::New(isolate, 0x8D89));
    tmpl->Set(isolate, "RGBA8I", v8::Uint32::New(isolate, 0x8D8E));


    tmpl->Set(isolate, "RGB8I", v8::Uint32::New(isolate, 0x8D8F));
    tmpl->Set(isolate, "RED_INTEGER", v8::Uint32::New(isolate, 0x8D94));
    tmpl->Set(isolate, "RGB_INTEGER", v8::Uint32::New(isolate, 0x8D98));
    tmpl->Set(isolate, "RGBA_INTEGER", v8::Uint32::New(isolate, 0x8D99));
    tmpl->Set(isolate, "R8", v8::Uint32::New(isolate, 0x8229));


    tmpl->Set(isolate, "RG8", v8::Uint32::New(isolate, 0x822B));
    tmpl->Set(isolate, "R16F", v8::Uint32::New(isolate, 0x822D));
    tmpl->Set(isolate, "R32F", v8::Uint32::New(isolate, 0x822E));
    tmpl->Set(isolate, "RG16F", v8::Uint32::New(isolate, 0x822F));
    tmpl->Set(isolate, "RG32F", v8::Uint32::New(isolate, 0x8230));


    tmpl->Set(isolate, "R8I", v8::Uint32::New(isolate, 0x8231));
    tmpl->Set(isolate, "R8UI", v8::Uint32::New(isolate, 0x8232));
    tmpl->Set(isolate, "R16I", v8::Uint32::New(isolate, 0x8233));
    tmpl->Set(isolate, "R16UI", v8::Uint32::New(isolate, 0x8234));
    tmpl->Set(isolate, "R32I", v8::Uint32::New(isolate, 0x8235));


    tmpl->Set(isolate, "R32UI", v8::Uint32::New(isolate, 0x8236));
    tmpl->Set(isolate, "RG8I", v8::Uint32::New(isolate, 0x8237));
    tmpl->Set(isolate, "RG8UI", v8::Uint32::New(isolate, 0x8238));
    tmpl->Set(isolate, "RG16I", v8::Uint32::New(isolate, 0x8239));
    tmpl->Set(isolate, "RG16UI", v8::Uint32::New(isolate, 0x823A));

    tmpl->Set(isolate, "RG32I", v8::Uint32::New(isolate, 0x823B));
    tmpl->Set(isolate, "RG32UI", v8::Uint32::New(isolate, 0x823C));
    tmpl->Set(isolate, "R8_SNORM", v8::Uint32::New(isolate, 0x8F94));
    tmpl->Set(isolate, "RG8_SNORM", v8::Uint32::New(isolate, 0x8F95));
    tmpl->Set(isolate, "RGB8_SNORM", v8::Uint32::New(isolate, 0x8F96));


    tmpl->Set(isolate, "RGBA8_SNORM", v8::Uint32::New(isolate, 0x8F97));
    tmpl->Set(isolate, "RGB10_A2UI", v8::Uint32::New(isolate, 0x906F));
    tmpl->Set(isolate, "TEXTURE_IMMUTABLE_FORMAT", v8::Uint32::New(isolate, 0x912F));
    tmpl->Set(isolate, "TEXTURE_IMMUTABLE_LEVELS", v8::Uint32::New(isolate, 0x82DF));
    tmpl->Set(isolate, "UNSIGNED_INT_2_10_10_10_REV", v8::Uint32::New(isolate, 0x8368));


    tmpl->Set(isolate, "UNSIGNED_INT_10F_11F_11F_REV", v8::Uint32::New(isolate, 0x8C3B));
    tmpl->Set(isolate, "UNSIGNED_INT_5_9_9_9_REV", v8::Uint32::New(isolate, 0x8C3E));
    tmpl->Set(isolate, "FLOAT_32_UNSIGNED_INT_24_8_REV", v8::Uint32::New(isolate, 0x8DAD));
    tmpl->Set(isolate, "UNSIGNED_INT_24_8", v8::Uint32::New(isolate, 0x84FA));
    tmpl->Set(isolate, "HALF_FLOAT", v8::Uint32::New(isolate, 0x140B));


    tmpl->Set(isolate, "RG", v8::Uint32::New(isolate, 0x8227));
    tmpl->Set(isolate, "RG_INTEGER", v8::Uint32::New(isolate, 0x8228));
    tmpl->Set(isolate, "INT_2_10_10_10_REV", v8::Uint32::New(isolate, 0x8D9F));
    tmpl->Set(isolate, "QUERY_RESULT_AVAILABLE", v8::Uint32::New(isolate, 0x8865));
    tmpl->Set(isolate, "QUERY_RESULT", v8::Uint32::New(isolate, 0x8866));


    tmpl->Set(isolate, "CURRENT_QUERY", v8::Uint32::New(isolate, 0x8867));
    tmpl->Set(isolate, "ANY_SAMPLES_PASSED", v8::Uint32::New(isolate, 0x8C2F));
    tmpl->Set(isolate, "ANY_SAMPLES_PASSED_CONSERVATIVE", v8::Uint32::New(isolate, 0x8D6A));
    tmpl->Set(isolate, "MAX_DRAW_BUFFERS", v8::Uint32::New(isolate, 0x8824));

    tmpl->Set(isolate, "DRAW_BUFFER0", v8::Uint32::New(isolate, 0x8825));
    tmpl->Set(isolate, "DRAW_BUFFER1", v8::Uint32::New(isolate, 0x8826));
    tmpl->Set(isolate, "DRAW_BUFFER2", v8::Uint32::New(isolate, 0x8827));
    tmpl->Set(isolate, "DRAW_BUFFER3", v8::Uint32::New(isolate, 0x8828));
    tmpl->Set(isolate, "DRAW_BUFFER4", v8::Uint32::New(isolate, 0x8829));
    tmpl->Set(isolate, "DRAW_BUFFER5", v8::Uint32::New(isolate, 0x882A));
    tmpl->Set(isolate, "DRAW_BUFFER6", v8::Uint32::New(isolate, 0x882B));
    tmpl->Set(isolate, "DRAW_BUFFER7", v8::Uint32::New(isolate, 0x882C));
    tmpl->Set(isolate, "DRAW_BUFFER8", v8::Uint32::New(isolate, 0x882D));
    tmpl->Set(isolate, "DRAW_BUFFER9", v8::Uint32::New(isolate, 0x882E));
    tmpl->Set(isolate, "DRAW_BUFFER10", v8::Uint32::New(isolate, 0x882F));

    /* Getting GL parameter information */

    /* Textures */

    tmpl->Set(isolate, "DRAW_BUFFER11", v8::Uint32::New(isolate, 0x8830));
    tmpl->Set(isolate, "DRAW_BUFFER12", v8::Uint32::New(isolate, 0x8831));
    tmpl->Set(isolate, "DRAW_BUFFER13", v8::Uint32::New(isolate, 0x8832));
    tmpl->Set(isolate, "DRAW_BUFFER14", v8::Uint32::New(isolate, 0x8833));
    tmpl->Set(isolate, "DRAW_BUFFER15", v8::Uint32::New(isolate, 0x8834));

    tmpl->Set(isolate, "MAX_COLOR_ATTACHMENTS", v8::Uint32::New(isolate, 0x8CDF));
    tmpl->Set(isolate, "COLOR_ATTACHMENT1", v8::Uint32::New(isolate, 0x8CE1));
    tmpl->Set(isolate, "COLOR_ATTACHMENT2", v8::Uint32::New(isolate, 0x8CE2));
    tmpl->Set(isolate, "COLOR_ATTACHMENT3", v8::Uint32::New(isolate, 0x8CE3));
    tmpl->Set(isolate, "COLOR_ATTACHMENT4", v8::Uint32::New(isolate, 0x8CE4));
    tmpl->Set(isolate, "COLOR_ATTACHMENT5", v8::Uint32::New(isolate, 0x8CE5));
    tmpl->Set(isolate, "COLOR_ATTACHMENT6", v8::Uint32::New(isolate, 0x8CE6));
    tmpl->Set(isolate, "COLOR_ATTACHMENT7", v8::Uint32::New(isolate, 0x8CE7));
    tmpl->Set(isolate, "COLOR_ATTACHMENT8", v8::Uint32::New(isolate, 0x8CE8));
    tmpl->Set(isolate, "COLOR_ATTACHMENT9", v8::Uint32::New(isolate, 0x8CE9));
    tmpl->Set(isolate, "COLOR_ATTACHMENT10", v8::Uint32::New(isolate, 0x8CEA));
    tmpl->Set(isolate, "COLOR_ATTACHMENT11", v8::Uint32::New(isolate, 0x8CEB));
    tmpl->Set(isolate, "COLOR_ATTACHMENT12", v8::Uint32::New(isolate, 0x8CEC));
    tmpl->Set(isolate, "COLOR_ATTACHMENT13", v8::Uint32::New(isolate, 0x8CED));
    tmpl->Set(isolate, "COLOR_ATTACHMENT14", v8::Uint32::New(isolate, 0x8CEE));
    tmpl->Set(isolate, "COLOR_ATTACHMENT15", v8::Uint32::New(isolate, 0x8CEF));

    tmpl->Set(isolate, "SAMPLER_3D", v8::Uint32::New(isolate, 0x8B5F));
    tmpl->Set(isolate, "SAMPLER_2D_SHADOW", v8::Uint32::New(isolate, 0x8B62));
    tmpl->Set(isolate, "SAMPLER_2D_ARRAY", v8::Uint32::New(isolate, 0x8DC1));
    tmpl->Set(isolate, "SAMPLER_2D_ARRAY_SHADOW", v8::Uint32::New(isolate, 0x8DC4));
    tmpl->Set(isolate, "SAMPLER_CUBE_SHADOW", v8::Uint32::New(isolate, 0x8DC5));

    tmpl->Set(isolate, "INT_SAMPLER_2D", v8::Uint32::New(isolate, 0x8DCA));
    tmpl->Set(isolate, "INT_SAMPLER_3D", v8::Uint32::New(isolate, 0x8DCB));
    tmpl->Set(isolate, "INT_SAMPLER_CUBE", v8::Uint32::New(isolate, 0x8DCC));
    tmpl->Set(isolate, "INT_SAMPLER_2D_ARRAY", v8::Uint32::New(isolate, 0x8DCF));
    tmpl->Set(isolate, "UNSIGNED_INT_SAMPLER_2D", v8::Uint32::New(isolate, 0x8DD2));

    tmpl->Set(isolate, "UNSIGNED_INT_SAMPLER_3D", v8::Uint32::New(isolate, 0x8DD3));
    tmpl->Set(isolate, "UNSIGNED_INT_SAMPLER_CUBE", v8::Uint32::New(isolate, 0x8DD4));
    tmpl->Set(isolate, "UNSIGNED_INT_SAMPLER_2D_ARRAY", v8::Uint32::New(isolate, 0x8DD7));
    tmpl->Set(isolate, "MAX_SAMPLES", v8::Uint32::New(isolate, 0x8D57));
    tmpl->Set(isolate, "SAMPLER_BINDING", v8::Uint32::New(isolate, 0x8919));

    tmpl->Set(isolate, "PIXEL_PACK_BUFFER", v8::Uint32::New(isolate, 0x88EB));
    tmpl->Set(isolate, "PIXEL_UNPACK_BUFFER", v8::Uint32::New(isolate, 0x88EC));
    tmpl->Set(isolate, "PIXEL_PACK_BUFFER_BINDING", v8::Uint32::New(isolate, 0x88ED));
    tmpl->Set(isolate, "PIXEL_UNPACK_BUFFER_BINDING", v8::Uint32::New(isolate, 0x88EF));
    tmpl->Set(isolate, "COPY_READ_BUFFER", v8::Uint32::New(isolate, 0x8F36));

    tmpl->Set(isolate, "COPY_WRITE_BUFFER", v8::Uint32::New(isolate, 0x8F37));
    tmpl->Set(isolate, "COPY_READ_BUFFER_BINDING", v8::Uint32::New(isolate, 0x8F36));
    tmpl->Set(isolate, "COPY_WRITE_BUFFER_BINDING", v8::Uint32::New(isolate, 0x8F37));
    tmpl->Set(isolate, "FLOAT_MAT2x3", v8::Uint32::New(isolate, 0x8B65));
    tmpl->Set(isolate, "FLOAT_MAT2x4", v8::Uint32::New(isolate, 0x8B66));

    tmpl->Set(isolate, "FLOAT_MAT3x2", v8::Uint32::New(isolate, 0x8B67));
    tmpl->Set(isolate, "FLOAT_MAT3x4", v8::Uint32::New(isolate, 0x8B68));
    tmpl->Set(isolate, "FLOAT_MAT4x2", v8::Uint32::New(isolate, 0x8B69));
    tmpl->Set(isolate, "FLOAT_MAT4x3", v8::Uint32::New(isolate, 0x8B6A));
    tmpl->Set(isolate, "UNSIGNED_INT_VEC2", v8::Uint32::New(isolate, 0x8DC6));

    tmpl->Set(isolate, "UNSIGNED_INT_VEC3", v8::Uint32::New(isolate, 0x8DC7));
    tmpl->Set(isolate, "UNSIGNED_INT_VEC4", v8::Uint32::New(isolate, 0x8DC8));
    tmpl->Set(isolate, "UNSIGNED_NORMALIZED", v8::Uint32::New(isolate, 0x8C17));
    tmpl->Set(isolate, "SIGNED_NORMALIZED", v8::Uint32::New(isolate, 0x8F9C));

    /* Vertex attributes */

    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_INTEGER", v8::Uint32::New(isolate, 0x88FD));
    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_DIVISOR", v8::Uint32::New(isolate, 0x88FE));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BUFFER_MODE", v8::Uint32::New(isolate, 0x8C7F));
    tmpl->Set(isolate, "MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS",
              v8::Uint32::New(isolate, 0x8C80));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_VARYINGS", v8::Uint32::New(isolate, 0x8C83));

    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BUFFER_START", v8::Uint32::New(isolate, 0x8C84));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BUFFER_SIZE", v8::Uint32::New(isolate, 0x8C85));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN", v8::Uint32::New(isolate, 0x8C88));
    tmpl->Set(isolate, "MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS",
              v8::Uint32::New(isolate, 0x8C8A));

    /* Textures */

    /* Pixel types */

    tmpl->Set(isolate, "MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS", v8::Uint32::New(isolate, 0x8C8B));
    tmpl->Set(isolate, "INTERLEAVED_ATTRIBS", v8::Uint32::New(isolate, 0x8C8C));
    tmpl->Set(isolate, "SEPARATE_ATTRIBS", v8::Uint32::New(isolate, 0x8C8D));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BUFFER", v8::Uint32::New(isolate, 0x8C8E));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BUFFER_BINDING", v8::Uint32::New(isolate, 0x8C8F));

    tmpl->Set(isolate, "TRANSFORM_FEEDBACK", v8::Uint32::New(isolate, 0x8E22));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_PAUSED", v8::Uint32::New(isolate, 0x8E23));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_ACTIVE", v8::Uint32::New(isolate, 0x8E24));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BINDING", v8::Uint32::New(isolate, 0x8E25));

    /* Pixel types */

    /* Queries */

    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING", v8::Uint32::New(isolate, 0x8210));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE", v8::Uint32::New(isolate, 0x8211));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_RED_SIZE", v8::Uint32::New(isolate, 0x8212));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_GREEN_SIZE", v8::Uint32::New(isolate, 0x8213));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_BLUE_SIZE", v8::Uint32::New(isolate, 0x8214));

    /* Queries */

    /* Draw buffers */

    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE", v8::Uint32::New(isolate, 0x8215));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE", v8::Uint32::New(isolate, 0x8216));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE", v8::Uint32::New(isolate, 0x8217));
    tmpl->Set(isolate, "FRAMEBUFFER_DEFAULT", v8::Uint32::New(isolate, 0x8218));
    tmpl->Set(isolate, "DEPTH_STENCIL_ATTACHMENT", v8::Uint32::New(isolate, 0x821A));

    tmpl->Set(isolate, "DEPTH_STENCIL", v8::Uint32::New(isolate, 0x84F9));
    tmpl->Set(isolate, "DEPTH24_STENCIL8", v8::Uint32::New(isolate, 0x88F0));
    tmpl->Set(isolate, "DRAW_FRAMEBUFFER_BINDING", v8::Uint32::New(isolate, 0x8CA6));
    tmpl->Set(isolate, "READ_FRAMEBUFFER", v8::Uint32::New(isolate, 0x8CA8));
    tmpl->Set(isolate, "DRAW_FRAMEBUFFER", v8::Uint32::New(isolate, 0x8CA9));

    tmpl->Set(isolate, "READ_FRAMEBUFFER_BINDING", v8::Uint32::New(isolate, 0x8CAA));
    tmpl->Set(isolate, "RENDERBUFFER_SAMPLES", v8::Uint32::New(isolate, 0x8CAB));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER", v8::Uint32::New(isolate, 0x8CD4));
    tmpl->Set(isolate, "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE", v8::Uint32::New(isolate, 0x8D56));
    tmpl->Set(isolate, "UNIFORM_BUFFER", v8::Uint32::New(isolate, 0x8A11));

    tmpl->Set(isolate, "UNIFORM_BUFFER_BINDING", v8::Uint32::New(isolate, 0x8A28));
    tmpl->Set(isolate, "UNIFORM_BUFFER_START", v8::Uint32::New(isolate, 0x8A29));
    tmpl->Set(isolate, "UNIFORM_BUFFER_SIZE", v8::Uint32::New(isolate, 0x8A2A));
    tmpl->Set(isolate, "MAX_VERTEX_UNIFORM_BLOCKS", v8::Uint32::New(isolate, 0x8A2B));
    tmpl->Set(isolate, "MAX_FRAGMENT_UNIFORM_BLOCKS", v8::Uint32::New(isolate, 0x8A2D));

    tmpl->Set(isolate, "MAX_COMBINED_UNIFORM_BLOCKS", v8::Uint32::New(isolate, 0x8A2E));
    tmpl->Set(isolate, "MAX_UNIFORM_BUFFER_BINDINGS", v8::Uint32::New(isolate, 0x8A2F));
    tmpl->Set(isolate, "MAX_UNIFORM_BLOCK_SIZE", v8::Uint32::New(isolate, 0x8A30));
    tmpl->Set(isolate, "MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS", v8::Uint32::New(isolate, 0x8A31));
    tmpl->Set(isolate, "MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS",
              v8::Uint32::New(isolate, 0x8A33));

    tmpl->Set(isolate, "UNIFORM_BUFFER_OFFSET_ALIGNMENT", v8::Uint32::New(isolate, 0x8A34));
    tmpl->Set(isolate, "ACTIVE_UNIFORM_BLOCKS", v8::Uint32::New(isolate, 0x8A36));
    tmpl->Set(isolate, "UNIFORM_TYPE", v8::Uint32::New(isolate, 0x8A37));
    tmpl->Set(isolate, "UNIFORM_SIZE", v8::Uint32::New(isolate, 0x8A38));
    tmpl->Set(isolate, "UNIFORM_BLOCK_INDEX", v8::Uint32::New(isolate, 0x8A3A));

    tmpl->Set(isolate, "UNIFORM_OFFSET", v8::Uint32::New(isolate, 0x8A3B));
    tmpl->Set(isolate, "UNIFORM_ARRAY_STRIDE", v8::Uint32::New(isolate, 0x8A3C));
    tmpl->Set(isolate, "UNIFORM_MATRIX_STRIDE", v8::Uint32::New(isolate, 0x8A3D));

    /* Draw buffers */

    /* Samplers */

    tmpl->Set(isolate, "UNIFORM_IS_ROW_MAJOR", v8::Uint32::New(isolate, 0x8A3E));
    tmpl->Set(isolate, "UNIFORM_BLOCK_BINDING", v8::Uint32::New(isolate, 0x8A3F));
    tmpl->Set(isolate, "UNIFORM_BLOCK_DATA_SIZE", v8::Uint32::New(isolate, 0x8A40));
    tmpl->Set(isolate, "UNIFORM_BLOCK_ACTIVE_UNIFORMS", v8::Uint32::New(isolate, 0x8A42));
    tmpl->Set(isolate, "UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES", v8::Uint32::New(isolate, 0x8A43));

    tmpl->Set(isolate, "UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER",
              v8::Uint32::New(isolate, 0x8A44));
    tmpl->Set(isolate, "UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER",
              v8::Uint32::New(isolate, 0x8A46));
    tmpl->Set(isolate, "OBJECT_TYPE", v8::Uint32::New(isolate, 0x9112));
    tmpl->Set(isolate, "SYNC_CONDITION", v8::Uint32::New(isolate, 0x9113));
    tmpl->Set(isolate, "SYNC_STATUS", v8::Uint32::New(isolate, 0x9114));

    tmpl->Set(isolate, "SYNC_FLAGS", v8::Uint32::New(isolate, 0x9115));
    tmpl->Set(isolate, "SYNC_FENCE", v8::Uint32::New(isolate, 0x9116));
    tmpl->Set(isolate, "SYNC_GPU_COMMANDS_COMPLETE", v8::Uint32::New(isolate, 0x9117));
    tmpl->Set(isolate, "UNSIGNALED", v8::Uint32::New(isolate, 0x9118));
    tmpl->Set(isolate, "SIGNALED", v8::Uint32::New(isolate, 0x9119));

    /* Samplers */

    /* Buffers */

    tmpl->Set(isolate, "ALREADY_SIGNALED", v8::Uint32::New(isolate, 0x911A));
    tmpl->Set(isolate, "TIMEOUT_EXPIRED", v8::Uint32::New(isolate, 0x911B));
    tmpl->Set(isolate, "CONDITION_SATISFIED", v8::Uint32::New(isolate, 0x911C));
    tmpl->Set(isolate, "WAIT_FAILED", v8::Uint32::New(isolate, 0x911D));
    tmpl->Set(isolate, "SYNC_FLUSH_COMMANDS_BIT", v8::Uint32::New(isolate, 0x00000001));

    tmpl->Set(isolate, "COLOR", v8::Uint32::New(isolate, 0x1800));
    tmpl->Set(isolate, "DEPTH", v8::Uint32::New(isolate, 0x1801));
    tmpl->Set(isolate, "STENCIL", v8::Uint32::New(isolate, 0x1802));

    /* Buffers */

    /* Data types */

    tmpl->Set(isolate, "MIN", v8::Uint32::New(isolate, 0x8007));
    tmpl->Set(isolate, "MAX", v8::Uint32::New(isolate, 0x8008));
    tmpl->Set(isolate, "DEPTH_COMPONENT24", v8::Uint32::New(isolate, 0x81A6));
    tmpl->Set(isolate, "STREAM_READ", v8::Uint32::New(isolate, 0x88E1));
    tmpl->Set(isolate, "STREAM_COPY", v8::Uint32::New(isolate, 0x88E2));

    tmpl->Set(isolate, "STATIC_READ", v8::Uint32::New(isolate, 0x88E5));
    tmpl->Set(isolate, "STATIC_COPY", v8::Uint32::New(isolate, 0x88E6));
    tmpl->Set(isolate, "DYNAMIC_READ", v8::Uint32::New(isolate, 0x88E9));
    tmpl->Set(isolate, "DYNAMIC_COPY", v8::Uint32::New(isolate, 0x88EA));
    tmpl->Set(isolate, "DEPTH_COMPONENT32F", v8::Uint32::New(isolate, 0x8CAC));
    tmpl->Set(isolate, "DEPTH32F_STENCIL8", v8::Uint32::New(isolate, 0x8CAD));

    /* Data types */

    tmpl->Set(isolate, "INVALID_INDEX", v8::Number::New(isolate, (double) 0xFFFFFFFF));
    tmpl->Set(isolate, "TIMEOUT_IGNORED", v8::Int32::New(isolate, -1));

    /* Vertex attributes */

    /* Transform feedback */

    tmpl->Set(isolate, "MAX_CLIENT_WAIT_TIMEOUT_WEBGL", v8::Uint32::New(isolate, 0x9247));

    /* Transform feedback */

}
