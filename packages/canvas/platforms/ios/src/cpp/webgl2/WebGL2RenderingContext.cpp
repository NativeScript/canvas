//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGL2RenderingContext.h"

WebGL2RenderingContext::WebGL2RenderingContext(WebGLState* state) : WebGLRenderingContext(
        state, WebGLRenderingVersion::V2) {
}


WebGL2RenderingContext::WebGL2RenderingContext(WebGLState* state,
                                               WebGLRenderingVersion version)
        : WebGLRenderingContext(state, version) {
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

    WebGLRenderingContext::SetMethods(isolate, tmpl);
    SetMethods(isolate, tmpl);

    WebGLRenderingContext::SetProps(isolate, tmpl);
    SetProps(isolate, tmpl);
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

        if (query != nullptr) {
            canvas_native_webgl2_begin_query(
                    target,
                    query->GetQuery(),
                    ptr->GetState()
            );
        }
    }
}

void
WebGL2RenderingContext::BeginTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

void
WebGL2RenderingContext::BindTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        auto transformFeedback = WebGLTransformFeedback::GetPointer(
                transformFeedbackValue.As<v8::Object>());

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
            std::vector<float> buf;
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

            canvas_native_webgl2_clear_bufferfv(
                    buffer->GetBuffer(),
                    drawbuffer,
                                                buf.data(),
                                                buf.size(),
                    ptr->GetState()
            );

        } else if (values->IsFloat32Array()) {
            auto buf = values.As<v8::TypedArray>();
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<float *>((void*) data_ptr);
  
    
            canvas_native_webgl2_clear_bufferfv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    data,
                    size,
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
            std::vector<int32_t> buf;
            buf.reserve(len);
            for (int j = 0; j < len; ++j) {
                auto item = array->Get(
                        context, j).ToLocalChecked();
                buf.push_back(
                        static_cast<int32_t>(item->NumberValue(context).ToChecked())
                );
            }

            
            canvas_native_webgl2_clear_bufferiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                                                buf.data(),
                                                buf.size(),
                    ptr->GetState()
            );

        } else if (values->IsInt32Array()) {
            auto buf = values.As<v8::TypedArray>();
            
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<int32_t *>((void*) data_ptr);
            
            
            
            
            canvas_native_webgl2_clear_bufferiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    data,
                                                size,
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
            std::vector<uint32_t> buf;
            buf.reserve(len);
            for (int j = 0; j < len; ++j) {
                auto item = array->Get(
                        context, j).ToLocalChecked();
                buf.push_back(
                        static_cast<uint32_t>(item->NumberValue(context).ToChecked())
                );
            }

           
            canvas_native_webgl2_clear_bufferuiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                                                 buf.data(),
                                                 buf.size(),
                    ptr->GetState()
            );

        } else if (values->IsUint32Array()) {
            auto buf = values.As<v8::TypedArray>();
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint32_t *>((void*) data_ptr);
            
            
            
            canvas_native_webgl2_clear_bufferuiv(
                    buffer->GetBuffer(),
                    drawbuffer,
                    data, size,
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
    if (args.Length() > 2 && type == NativeType::WebGLSync) {
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

void
WebGL2RenderingContext::CompressedTexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
                auto buf = imageSizeOrBufValue.As<v8::TypedArray>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
                auto data = static_cast<uint8_t *>((void*) data_ptr);

                size_t srcOffset = 0;
                if (args[10]->IsNumber()) {
                    srcOffset = static_cast<size_t>(args[10]->NumberValue(context).ToChecked());
                }
                size_t srcLengthOverride = 0;
                if (args[11]->IsNumber()) {
                    srcLengthOverride = static_cast<size_t>(args[11]->NumberValue(
                            context).ToChecked());
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
                        data,
                        size,
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

void
WebGL2RenderingContext::CreateTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

    auto vertexArrayObject = WebGLVertexArrayObject::NewInstance(isolate,
                                                                 new WebGLVertexArrayObject(ret));

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

void
WebGL2RenderingContext::DeleteTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto transformFeedbackValue = args[0];
    auto type = GetNativeType(isolate, transformFeedbackValue);
    if (type == NativeType::WebGLTransformFeedback) {
        auto transformFeedback = WebGLTransformFeedback::GetPointer(
                transformFeedbackValue.As<v8::Object>());

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
        std::vector<uint32_t> buf;
        buf.reserve(len);
        for (int j = 0; j < len; ++j) {
            auto item = array->Get(
                    context, j).ToLocalChecked();
            buf.emplace_back(
                    (uint32_t) item->NumberValue(context).ToChecked());
        }
        
        canvas_native_webgl2_draw_buffers(
                                          buf.data(), buf.size(),
                ptr->GetState());
    }
}

void
WebGL2RenderingContext::DrawElementsInstanced(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

void
WebGL2RenderingContext::FramebufferTextureLayer(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

void
WebGL2RenderingContext::GetActiveUniformBlockName(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
            args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, (char*)name));
            return;
        }

    }

    args.GetReturnValue().SetEmptyString();
}

void WebGL2RenderingContext::GetActiveUniformBlockParameter(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
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
                            ret));
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                case GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: {
                    auto value = canvas_native_webgl_result_get_u32_array(
                            ret);

                    auto buf = (uint8_t*)canvas_native_u32_buffer_get_bytes(value);
                    auto size = canvas_native_u32_buffer_get_length(value);
                    auto bytes_size = size * sizeof(uint32_t);

                    auto store = v8::ArrayBuffer::NewBackingStore(buf, bytes_size,
                                                                  [](void *data, size_t length,
                                                                     void *deleter_data) {
                                                                      if (deleter_data != nullptr) {
                                                                          canvas_native_u32_buffer_destroy((U32Buffer *) deleter_data);
                                                                      }
                                                                  },
                                                                  value);

                    auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));
                    args.GetReturnValue().Set(v8::Uint32Array::New(arraybuffer, 0, size));
                    
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    
                    return;
                }
                case GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER:
                case GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                            ret));
                    
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                default:
                    args.GetReturnValue().SetNull();
                    canvas_native_webgl_WebGLResult_destroy(ret);
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
            std::vector<uint32_t> buf;
            buf.reserve(size);
            for (int j = 0; j < size; j++) {
                auto item = (uint32_t) uniformIndices->Get(
                        context, j).ToLocalChecked()->NumberValue(context).ToChecked();
                buf.emplace_back(item);
            }
            
            auto ret = canvas_native_webgl2_get_active_uniforms(
                    program->GetProgram(),
                                                                buf.data(), buf.size(),
                    pname,
                    ptr->GetState()
            );

            switch (pname) {
                case GL_UNIFORM_TYPE:
                case GL_UNIFORM_SIZE: {
                    auto value = canvas_native_webgl_result_get_u32_array(
                            ret);
                    auto len = canvas_native_u32_buffer_get_length(value);
                    auto buf = canvas_native_u32_buffer_get_bytes(value);
                    
                    auto array = v8::Array::New(
                            isolate, (int) len);

                    for (int j = 0;
                         j < len; ++j) {
                        auto item = buf[j];
                        array->Set(
                                context, j,
                                v8::Number::New(isolate, (double) item));
                    }
                    args.GetReturnValue().Set(array);
                    canvas_native_u32_buffer_destroy(value);
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                }
                case GL_UNIFORM_BLOCK_INDEX:
                case GL_UNIFORM_OFFSET:
                case GL_UNIFORM_ARRAY_STRIDE:
                case GL_UNIFORM_MATRIX_STRIDE: {
                    auto value = canvas_native_webgl_result_get_i32_array(
                            ret);
                    auto len = canvas_native_i32_buffer_get_length(value);
                    auto buf = canvas_native_i32_buffer_get_bytes(value);
                    auto array = v8::Array::New(
                            isolate, (int) len);
                    for (int j = 0;
                         j < len; ++j) {
                        auto item = buf[j];
                        array->Set(
                                context, j,
                                v8::Number::New(isolate,
                                                (double) item));
                    }
                    args.GetReturnValue().Set(array);
                    canvas_native_i32_buffer_destroy(value);
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                }
                case GL_UNIFORM_IS_ROW_MAJOR: {
                    auto value = canvas_native_webgl_result_get_bool_array(
                            ret);
                    auto len = canvas_native_u8_buffer_get_length(value);
                    auto buf = canvas_native_u8_buffer_get_bytes(value);
                    auto array = v8::Array::New(
                            isolate,
                            (int) len);
                    for (int j = 0;
                         j < len; ++j) {
                        bool item =
                                buf[j] == 1;
                        array->Set(
                                context, j,
                                v8::Boolean::New(isolate, item));
                    }
                    args.GetReturnValue().Set(array);
                    canvas_native_u8_buffer_destroy(value);
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                }
                default:
                    args.GetReturnValue().SetNull();
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
            }
        }


    }
}

void WebGL2RenderingContext::GetBufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto dstDataObject = args[2];
    if (args.Length() >= 3 &&
        dstDataObject->IsObject()) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto srcByteOffset = args[1]->NumberValue(context).ToChecked();

        if (dstDataObject->IsTypedArray()) {

            auto array = dstDataObject.As<v8::TypedArray>();
     
            auto buf = array->Buffer();
            auto offset = array->ByteOffset();
            auto size = buf->ByteLength();
            auto data = reinterpret_cast<uint8_t *>(buf->GetBackingStore()->Data()) + offset;
   
        

            auto BYTES_PER_ELEMENT = (ssize_t) array->Get(
                    context,
                    ConvertToV8String(isolate, "BYTES_PER_ELEMENT")).ToLocalChecked()->NumberValue(
                    context).ToChecked();

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
                    data, size,
                    dstOffsetValue,
                    lengthValue,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::GetFragDataLocation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto nameValue = args[1];
    auto type = GetNativeType(isolate, programValue);
    if (type == NativeType::WebGLProgram && nameValue->IsString()) {
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());

        if (program != nullptr) {
            auto name = ConvertFromV8String(isolate, nameValue);

            auto ret = canvas_native_webgl2_get_frag_data_location(
                    program->GetProgram(),
                    name.c_str(),
                    ptr->GetState()
            );

            args.GetReturnValue().Set(ret);

            return;
        }
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetIndexedParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
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
                        ret);

                args.GetReturnValue().Set(
                        WebGLBuffer::NewInstance(isolate, new WebGLBuffer(buffer)));
            }
                return;
            case GL_TRANSFORM_FEEDBACK_BUFFER_SIZE:
            case GL_TRANSFORM_FEEDBACK_BUFFER_START:
            case GL_UNIFORM_BUFFER_SIZE:
            case GL_UNIFORM_BUFFER_START: {
                auto value = canvas_native_webgl2_indexed_parameter_get_value(
                        ret);
                args.GetReturnValue().Set(static_cast<double>(value));
                return;
            }
                break;
            default:
                args.GetReturnValue().SetNull();
                return;
        }
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetInternalformatParameter(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
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
                auto arraybuffer = v8::ArrayBuffer::New(isolate, 0);
                
                args.GetReturnValue().Set(v8::Int32Array::New(arraybuffer, 0, 0));
                return;
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
                args.GetReturnValue().SetNull();
                return;
        }


        auto ret = canvas_native_webgl2_get_internalformat_parameter(
                target,
                internalformat,
                pname,
                ptr->GetState()
        );

        if (pname == GL_SAMPLES) {
            auto value = canvas_native_webgl_result_get_i32_array(ret);

            auto buf = (uint8_t*)canvas_native_i32_buffer_get_bytes(value);
            auto size = canvas_native_i32_buffer_get_length(value);
            auto bytes_size = size * sizeof(int32_t);

            auto store = v8::ArrayBuffer::NewBackingStore(buf, bytes_size,
                                                          [](void *data, size_t length,
                                                             void *deleter_data) {
                                                              if (deleter_data != nullptr) {
                                                                  canvas_native_i32_buffer_destroy((I32Buffer *) deleter_data);
                                                              }
                                                          },
                                                          value);

            auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));
            args.GetReturnValue().Set(v8::Int32Array::New(arraybuffer, 0, size));
            canvas_native_webgl_WebGLResult_destroy(ret);
            return;
        } else {
            args.GetReturnValue().SetNull();
            canvas_native_webgl_WebGLResult_destroy(ret);
            return;
        }
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto pname = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto result = canvas_native_webgl2_get_parameter(
                pname,
                ptr->GetState());
        switch (pname) {
            case GL_COPY_READ_BUFFER_BINDING:
            case GL_COPY_WRITE_BUFFER_BINDING:
            case GL_DRAW_FRAMEBUFFER_BINDING:
                args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                        result));
                
                canvas_native_webgl_WebGLResult_destroy(result);
                return;
            default: {
                auto ret = ptr->GetParameterInternal(
                        isolate, pname,result);
                
                canvas_native_webgl_WebGLResult_destroy(result);

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetQueryParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto queryValue = args[0];
        auto type = GetNativeType(isolate, queryValue);
        if (type == NativeType::WebGLQuery) {
            auto query = WebGLQuery::GetPointer(queryValue.As<v8::Object>());
            if (query != nullptr) {
                auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();


                auto ret = canvas_native_webgl2_get_query_parameter(
                        query->GetQuery(),
                        pname,
                        ptr->GetState());
                if (pname == GL_QUERY_RESULT) {
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                            ret));
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                } else if (pname ==
                           GL_QUERY_RESULT_AVAILABLE) {
                    args.GetReturnValue().Set((int32_t) canvas_native_webgl_result_get_u32(
                            ret));
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                }
                //clean up
                canvas_native_webgl_WebGLResult_destroy(ret);
            
            }
        }

    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();
        auto ret = canvas_native_webgl2_get_query(
                target,
                pname,
                ptr->GetState());
        if (pname == GL_CURRENT_QUERY) {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                    ret));
            canvas_native_webgl_WebGLResult_destroy(ret);
            return;
        }
        // clean up
        canvas_native_webgl_WebGLResult_destroy(ret);
    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetSamplerParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto samplerValue = args[0];
        auto type = GetNativeType(isolate, samplerValue);
        if (type == NativeType::WebGLSampler) {
            auto sampler = WebGLSampler::GetPointer(samplerValue.As<v8::Object>());
            if (sampler != nullptr) {
                auto pname = (uint32_t) args[1]->NumberValue(context).ToChecked();
                auto ret = canvas_native_webgl2_get_sampler_parameter(
                        sampler->GetSampler(),
                        pname,
                        ptr->GetState());

                switch (pname) {
                    case GL_TEXTURE_MAX_LOD:
                    case GL_TEXTURE_MIN_LOD:
                        args.GetReturnValue().Set(
                                static_cast<double>(canvas_native_webgl_result_get_f32(
                                        ret)));
                        canvas_native_webgl_WebGLResult_destroy(ret);
                        return;
                    case GL_TEXTURE_COMPARE_FUNC:
                    case GL_TEXTURE_COMPARE_MODE:
                    case GL_TEXTURE_MAG_FILTER:
                    case GL_TEXTURE_MIN_FILTER:
                    case GL_TEXTURE_WRAP_R:
                    case GL_TEXTURE_WRAP_S:
                    case GL_TEXTURE_WRAP_T:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                                ret));
                        canvas_native_webgl_WebGLResult_destroy(ret);
                        return;
                    default:
                        args.GetReturnValue().SetNull();
                        canvas_native_webgl_WebGLResult_destroy(ret);
                        return;
                }
            }
        }

    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetSyncParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto syncValue = args[0];
        auto sync = WebGLSyncImpl::GetPointer(syncValue.As<v8::Object>());
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
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                            ret));
                    
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                default:
                    args.GetReturnValue().SetNull();
                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
            }
        }
    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetTransformFeedbackVarying(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto programValue = args[0];
        auto type = GetNativeType(isolate, programValue);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
            auto index = (uint32_t) args[1]->NumberValue(context).ToChecked();
            if (program != nullptr) {
                auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                        program->GetProgram(),
                        index,
                        ptr->GetState()
                );

                if (canvas_native_webgl_active_info_get_is_empty(
                        ret)) {
                            canvas_native_webgl_active_info_destroy(ret);
                    args.GetReturnValue().SetNull();
                    return;
                }

                auto info = WebGLActiveInfoImpl::NewInstance(isolate, new WebGLActiveInfoImpl(ret));
                args.GetReturnValue().Set(info);
                return;
            }
        }
    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetUniformBlockIndex(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto programValue = args[0];
        auto type = GetNativeType(isolate, programValue);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
            auto index = ConvertFromV8String(isolate, args[1]);
            if (program != nullptr) {
                auto ret = canvas_native_webgl2_get_uniform_block_index(
                        program->GetProgram(),
                                                                        index.c_str(),
                        ptr->GetState());
                args.GetReturnValue().Set((int) ret);
                return;
            }
        }

    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetUniformIndices(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1 && args[1]->IsObject()) {
        auto programValue = args[0];
        auto type = GetNativeType(isolate, programValue);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
            auto uniformNamesObject = args[1];
            if (program != nullptr &&
                uniformNamesObject->IsArray()) {
                auto uniformNames = uniformNamesObject.As<v8::Array>();
                auto len = uniformNames->Length();
                
                std::vector<std::string> store;
                store.reserve(len);
                
                for (int j = 0; j < len; ++j) {
                    auto name = ConvertFromV8String(isolate, uniformNames->Get(
                            context, j).ToLocalChecked());
                    
                    store.push_back(name);
                }
                
                
                    const char* cStrings[store.size()];
                    for (size_t i = 0; i < store.size(); ++i) {
                        cStrings[i] = store[i].c_str();
                    }
                
                
                auto ret = canvas_native_webgl2_get_uniform_indices(
                        program->GetProgram(),
                               cStrings,
                                store.size(),
                        ptr->GetState());
            

                auto retSize = canvas_native_u32_buffer_get_length(ret);
                auto buf = canvas_native_u32_buffer_get_bytes(ret);
                auto result = v8::Array::New(isolate, (int) retSize);
                for (int j = 0; j < retSize; ++j) {
                    auto item = buf[j];
                    result->Set(context,
                                j,
                                v8::Number::New(isolate,
                                                (int32_t) item));
                }

                args.GetReturnValue().Set(result);
                canvas_native_u32_buffer_destroy(ret);
                return;
            }
        }

    }

    args.GetReturnValue().SetNull();
}

void
WebGL2RenderingContext::InvalidateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto attachments = args[1];
    if (args.Length() > 1 && attachments->IsObject()) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();

        if (attachments->IsArray()) {
            auto array = attachments.As<v8::Array>();
            auto len = array->Length();
            std::vector<uint32_t> buf;
            buf.reserve(len);
            for (int j = 0; j < len; ++j) {
                auto item = (uint32_t) array->Get(
                        context, j).ToLocalChecked()->NumberValue(context).ToChecked();
                buf.push_back(item);
            }

            canvas_native_webgl2_invalidate_framebuffer(
                    target, buf.data(), buf.size(),
                    ptr->GetState());
        }
    }
}

void
WebGL2RenderingContext::InvalidateSubFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 5 && args[1]->IsObject()) {
        auto attachments = args[1];
        if (attachments->IsArray()) {
            auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
            auto x = (int32_t) args[2]->NumberValue(context).ToChecked();
            auto y = (int32_t) args[3]->NumberValue(context).ToChecked();
            auto width = (int32_t) args[4]->NumberValue(context).ToChecked();
            auto height = (int32_t) args[5]->NumberValue(context).ToChecked();

            auto array = attachments.As<v8::Array>();
            auto len = array->Length();
            std::vector<uint32_t> buf;
            buf.reserve(len);
            for (int j = 0; j < len; ++j) {
                auto item = (uint32_t) array->Get(
                        context, j).ToLocalChecked()->NumberValue(context).ToChecked();
                buf.push_back(item);
            }
           
            canvas_native_webgl2_invalidate_sub_framebuffer(
                    target,
                                                            buf.data(), buf.size(),
                    x,
                    y,
                    width,
                    height,
                    ptr->GetState());
        }


    }
}

void WebGL2RenderingContext::IsQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLQuery) {
        auto query = WebGLQuery::GetPointer(value.As<v8::Object>());
        if (query != nullptr) {
            auto ret = canvas_native_webgl2_is_query(
                    query->GetQuery(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }
    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::IsSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLSampler) {
        auto query = WebGLSampler::GetPointer(value.As<v8::Object>());
        if (query != nullptr) {
            auto ret = canvas_native_webgl2_is_sampler(
                    query->GetSampler(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }
    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::IsSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLSync) {
        auto query = WebGLSyncImpl::GetPointer(value.As<v8::Object>());
        if (query != nullptr) {
            auto ret = canvas_native_webgl2_is_sync(
                    query->GetSync(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }
    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::IsTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLTransformFeedback) {
        auto query = WebGLTransformFeedback::GetPointer(value.As<v8::Object>());
        if (query != nullptr) {
            auto ret = canvas_native_webgl2_is_transform_feedback(
                    query->GetFeedback(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }
    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::IsVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLVertexArrayObject) {
        auto query = WebGLVertexArrayObject::GetPointer(value.As<v8::Object>());
        if (query != nullptr) {
            auto ret = canvas_native_webgl2_is_vertex_array(
                    query->GetVertexArrayObject(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }
    // todo check return
    args.GetReturnValue().Set(false);
}

void
WebGL2RenderingContext::PauseTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgl2_pause_transform_feedback(
            ptr->GetState());
}

void WebGL2RenderingContext::ReadBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto src = (uint32_t) args[0]->NumberValue(context).ToChecked();
        canvas_native_webgl2_read_buffer(
                src,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::RenderbufferStorageMultisample(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 4) {
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
}

void
WebGL2RenderingContext::ResumeTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgl2_resume_transform_feedback(
            ptr->GetState());
}

void WebGL2RenderingContext::SamplerParameterf(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
        auto samplerValue = args[0];
        auto type = GetNativeType(isolate, samplerValue);
        if (type == NativeType::WebGLSampler) {
            auto sampler = WebGLSampler::GetPointer(samplerValue.As<v8::Object>());
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

    }

}

void WebGL2RenderingContext::SamplerParameteri(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
        auto samplerValue = args[0];
        auto type = GetNativeType(isolate, samplerValue);
        if (type == NativeType::WebGLSampler) {
            auto sampler = WebGLSampler::GetPointer(samplerValue.As<v8::Object>());
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

    }

}

void WebGL2RenderingContext::TexImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();


    if (count == 10) {
        auto target = (int32_t) args[0]->NumberValue(
                context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto internalformat = (int32_t) args[2]->NumberValue(
                context).ToChecked();
        auto width = (int32_t) args[3]->NumberValue(
                context).ToChecked();
        auto height = (int32_t) args[4]->NumberValue(
                context).ToChecked();
        auto depth = (int32_t) args[5]->NumberValue(
                context).ToChecked();
        auto border = (int32_t) args[6]->NumberValue(
                context).ToChecked();
        auto format = (int32_t) args[7]->NumberValue(
                context).ToChecked();
        auto type = (uint32_t) args[8]->NumberValue(
                context).ToChecked();

        auto imageOrPixelsOrOffset = args[9];

        if (imageOrPixelsOrOffset->IsNumber()) {
            auto offset = imageOrPixelsOrOffset->NumberValue(
                    context).ToChecked();
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
                    static_cast<ssize_t>(offset),
                    ptr->GetState()
            );
            return;
        }


        if (imageOrPixelsOrOffset->IsTypedArray()) {
            auto buf = imageOrPixelsOrOffset.As<v8::TypedArray>();
            
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint8_t *>((void*) data_ptr);
            
            
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
                    data,
                    size,
                    ptr->GetState()
            );
            return;
        }

        auto imageType = GetNativeType(isolate, imageOrPixelsOrOffset);

        switch (imageType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(
                        imageOrPixelsOrOffset.As<v8::Object>());
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

                    return;
                }
                return;
            }
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(
                        imageOrPixelsOrOffset.As<v8::Object>());
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

                    return;
                }
                return;
            }
            default:
                return;
        }
    } else if (args.Length() > 10) {

        auto target = (int32_t) args[0]->NumberValue(
                context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto internalformat = (int32_t) args[2]->NumberValue(
                context).ToChecked();
        auto width = (int32_t) args[3]->NumberValue(
                context).ToChecked();
        auto height = (int32_t) args[4]->NumberValue(
                context).ToChecked();
        auto depth = (int32_t) args[5]->NumberValue(
                context).ToChecked();
        auto border = (int32_t) args[6]->NumberValue(
                context).ToChecked();
        auto format = (int32_t) args[7]->NumberValue(
                context).ToChecked();
        auto type = (uint32_t) args[8]->NumberValue(
                context).ToChecked();

        auto imageOrPixelsOrOffset = args[9];

        size_t srcOffsetValue = 0;
        if (args[9]->IsNumber()) {
            srcOffsetValue = static_cast<size_t>(args[9]->NumberValue(
                    context).ToChecked());
        }

        if (imageOrPixelsOrOffset->IsTypedArray()) {
            auto buf = imageOrPixelsOrOffset.As<v8::TypedArray>();
   
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint8_t *>((void*) data_ptr);
           
            srcOffsetValue =
                    srcOffsetValue * size;
            if (srcOffsetValue >
                size) {
                return;
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
                    data, size,
                    srcOffsetValue,
                    ptr->GetState()
            );
            return;
        }

    }
}

void WebGL2RenderingContext::TexStorage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 4) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto levels = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto internalFormat = (uint32_t) args[2]->NumberValue(
                context).ToChecked();
        auto width = (int32_t) args[3]->NumberValue(
                context).ToChecked();
        auto height = (int32_t) args[4]->NumberValue(
                context).ToChecked();
        canvas_native_webgl2_tex_storage2d(
                target,
                levels,
                internalFormat,
                width,
                height,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::TexStorage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 5) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto levels = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto internalFormat = (uint32_t) args[2]->NumberValue(
                context).ToChecked();
        auto width = (int32_t) args[3]->NumberValue(
                context).ToChecked();
        auto height = (int32_t) args[4]->NumberValue(
                context).ToChecked();
        auto depth = (int32_t) args[5]->NumberValue(
                context).ToChecked();
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
}

void WebGL2RenderingContext::TexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();
    if (count == 11) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto xoffset = (int32_t) args[2]->NumberValue(
                context).ToChecked();
        auto yoffset = (int32_t) args[3]->NumberValue(
                context).ToChecked();
        auto zoffset = (int32_t) args[4]->NumberValue(
                context).ToChecked();
        auto width = (int32_t) args[5]->NumberValue(
                context).ToChecked();
        auto height = (int32_t) args[6]->NumberValue(
                context).ToChecked();
        auto depth = (int32_t) args[7]->NumberValue(
                context).ToChecked();
        auto format = (uint32_t) args[8]->NumberValue(
                context).ToChecked();
        auto type = (int32_t) args[9]->NumberValue(
                context).ToChecked();

        if (args[10]->IsNumber()) {
            auto imageOrPixelsOrOffset = args[10]->NumberValue(
                    context).ToChecked();
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
            return;
        }

        auto imageOrPixelsOrOffsetObject = args[10];

        if (imageOrPixelsOrOffsetObject->IsTypedArray()) {
            auto buf = imageOrPixelsOrOffsetObject.As<v8::TypedArray>();
            
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint8_t *>((void*) data_ptr);
           
            
        
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
                    data, size,
                    ptr->GetState()
            );

            return;
        }


        auto imageType = GetNativeType(isolate, imageOrPixelsOrOffsetObject);

        if (imageType == NativeType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(imageOrPixelsOrOffsetObject.As<v8::Object>());
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
        }

    } else if (count > 11) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto xoffset = (int32_t) args[2]->NumberValue(
                context).ToChecked();
        auto yoffset = (int32_t) args[3]->NumberValue(
                context).ToChecked();
        auto zoffset = (int32_t) args[4]->NumberValue(
                context).ToChecked();
        auto width = (int32_t) args[5]->NumberValue(
                context).ToChecked();
        auto height = (int32_t) args[6]->NumberValue(
                context).ToChecked();
        auto depth = (int32_t) args[7]->NumberValue(
                context).ToChecked();
        auto format = (uint32_t) args[8]->NumberValue(
                context).ToChecked();
        auto type = (uint32_t) args[9]->NumberValue(
                context).ToChecked();

        size_t srcOffsetValue = 0;

        if (args[11]->IsNumber()) {
            srcOffsetValue = static_cast<size_t>(args[11]->NumberValue(
                    context).ToChecked());
        }

        auto imageOrPixelsOrOffsetObject = args[10];


        if (imageOrPixelsOrOffsetObject->IsTypedArray()) {
            auto buf = imageOrPixelsOrOffsetObject.As<v8::TypedArray>();
          
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint8_t *>((void*) data_ptr);
            
            
            srcOffsetValue =
                    srcOffsetValue * size;
            if (srcOffsetValue > size) {
                return;
            }


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
                    data, size,
                    srcOffsetValue,
                    ptr->GetState()
            );
        }
    }
}

void
WebGL2RenderingContext::TransformFeedbackVaryings(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);
    if (args.Length() > 2 && type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        auto varyingsObject = args[1];
        auto bufferMode = (uint32_t) args[2]->NumberValue(
                context).ToChecked();

        if (program != nullptr &&
            varyingsObject->IsArray()) {
            auto varyings = varyingsObject.As<v8::Array>();
            auto len = varyings->Length();
            
            
            std::vector<std::string> store;
            store.reserve(len);
            
            for (int j = 0; j < len; ++j) {
                auto name = ConvertFromV8String(isolate, varyings->Get(
                        context, j).ToLocalChecked());
                
                store.push_back(name);
            }
            
            
                const char* cStrings[store.size()];
                for (size_t i = 0; i < store.size(); ++i) {
                    cStrings[i] = store[i].c_str();
                }
            
            canvas_native_webgl2_transform_feedback_varyings(
                    program->GetProgram(),
                    cStrings, store.size(),
                    bufferMode,
                    ptr->GetState()
            );
        }
    }

}

void WebGL2RenderingContext::Uniform1ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);

    if (args.Length() > 1 &&
        args[1]->IsNumber() && type == NativeType::WebGLUniformLocation) {

        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto v0 = (uint32_t) args[1]->NumberValue(
                context).ToChecked();

        if (location != nullptr) {
            canvas_native_webgl2_uniform1ui(
                    location->GetUniformLocation(),
                    v0,
                    ptr->GetState()
            );
        }
    }

}

void WebGL2RenderingContext::Uniform2ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);

    if (args.Length() > 2 &&
        args[1]->IsNumber() && type == NativeType::WebGLUniformLocation) {

        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto v0 = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        auto v1 = (uint32_t) args[2]->NumberValue(
                context).ToChecked();

        if (location != nullptr) {
            canvas_native_webgl2_uniform2ui(
                    location->GetUniformLocation(),
                    v0,
                    v1,
                    ptr->GetState()
            );
        }
    }

}

void WebGL2RenderingContext::Uniform3ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);

    if (args.Length() > 3 &&
        args[1]->IsNumber() && type == NativeType::WebGLUniformLocation) {

        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto v0 = (uint32_t) args[1]->NumberValue(
                context).ToChecked();

        auto v1 = (uint32_t) args[2]->NumberValue(
                context).ToChecked();

        auto v2 = (uint32_t) args[3]->NumberValue(
                context).ToChecked();

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

}

void WebGL2RenderingContext::Uniform4ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);

    if (args.Length() > 1 &&
        args[1]->IsNumber() && type == NativeType::WebGLUniformLocation) {

        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto v0 = (uint32_t) args[1]->NumberValue(
                context).ToChecked();

        auto v1 = (uint32_t) args[2]->NumberValue(
                context).ToChecked();

        auto v2 = (uint32_t) args[3]->NumberValue(
                context).ToChecked();
        auto v3 = (uint32_t) args[4]->NumberValue(
                context).ToChecked();


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

}

void WebGL2RenderingContext::Uniform1uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);


    if (args.Length() > 1 &&
        args[1]->IsObject() && type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto data = args[1];
        if (location != nullptr &&
            data->IsUint32Array()) {
            auto buf = data.As<v8::TypedArray>();
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint32_t *>((void*) data_ptr);
            
            
            canvas_native_webgl2_uniform1uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState()
            );
        } else {
            std::vector<uint32_t> buf;
            auto array = data.As<v8::Array>();
            auto len = array->Length();
            buf.reserve(len);
            for (int i = 0; i < len; i++) {
                auto item = (uint32_t) array->Get(
                        context,
                        i).ToLocalChecked()->NumberValue(
                        context).ToChecked();
                buf.push_back(item);
            }
            
            canvas_native_webgl2_uniform1uiv(
                    location->GetUniformLocation(),
                    buf.data(), buf.size(),
                    ptr->GetState()
            );
        }


    }

}

void WebGL2RenderingContext::Uniform2uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);


    if (args.Length() > 1 &&
        args[1]->IsObject() && type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto data = args[1];
        if (location != nullptr &&
            data->IsUint32Array()) {
            auto buf = data.As<v8::TypedArray>();
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint32_t *>((void*) data_ptr);
            
            
            
            canvas_native_webgl2_uniform2uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState()
            );
        } else {
            std::vector<uint32_t> buf;
            auto array = data.As<v8::Array>();
            auto len = array->Length();
            buf.reserve(len);
            for (int i = 0; i < len; i++) {
                auto item = (uint32_t) array->Get(
                        context,
                        i).ToLocalChecked()->NumberValue(
                        context).ToChecked();
                buf.push_back(item);
            }
            canvas_native_webgl2_uniform2uiv(
                    location->GetUniformLocation(),
                    buf.data(), buf.size(),
                    ptr->GetState()
            );
        }


    }

}

void WebGL2RenderingContext::Uniform3uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);


    if (args.Length() > 1 &&
        args[1]->IsObject() && type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto data = args[1];
        if (location != nullptr &&
            data->IsUint32Array()) {
            auto buf = data.As<v8::TypedArray>();
            
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint32_t *>((void*) data_ptr);
            
            
            canvas_native_webgl2_uniform3uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState()
            );
        } else {
            std::vector<uint32_t> buf;
            auto array = data.As<v8::Array>();
            auto len = array->Length();
            buf.reserve(len);
            for (int i = 0; i < len; i++) {
                auto item = (uint32_t) array->Get(
                        context,
                        i).ToLocalChecked()->NumberValue(
                        context).ToChecked();
                buf.push_back(item);
            }
            canvas_native_webgl2_uniform3uiv(
                    location->GetUniformLocation(),
                    buf.data(), buf.size(),
                    ptr->GetState()
            );
        }


    }

}

void WebGL2RenderingContext::Uniform4uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);


    if (args.Length() > 1 &&
        args[1]->IsObject() && type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto data = args[1];
        if (location != nullptr &&
            data->IsUint32Array()) {
            auto buf = data.As<v8::TypedArray>();
            
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint32_t *>((void*) data_ptr);
            
            
            canvas_native_webgl2_uniform4uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState()
            );
        } else {
            std::vector<uint32_t> buf;
            auto array = data.As<v8::Array>();
            auto len = array->Length();
            buf.reserve(len);
            for (int i = 0; i < len; i++) {
                auto item = (uint32_t) array->Get(
                        context,
                        i).ToLocalChecked()->NumberValue(
                        context).ToChecked();
                buf.push_back(item);
            }
            
            canvas_native_webgl2_uniform4uiv(
                    location->GetUniformLocation(),
                    buf.data(), buf.size(),
                    ptr->GetState()
            );
        }

    }

}

void WebGL2RenderingContext::UniformBlockBinding(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);
    if (args.Length() > 2 && type == NativeType::WebGLProgram) {

        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        auto uniformBlockIndex = args[1]->NumberValue(
                context).ToChecked();
        auto uniformBlockBinding = args[2]->NumberValue(
                context).ToChecked();

        if (program != nullptr) {
            canvas_native_webgl2_uniform_block_binding(
                    program->GetProgram(),
                    (uint32_t) uniformBlockIndex,
                    (uint32_t) uniformBlockBinding,
                    ptr->GetState()
            );
        }

    }

}

void WebGL2RenderingContext::UniformMatrix2x3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 2 &&
        args[2]->IsObject()) {
        auto locationValue = args[0];
        auto type = GetNativeType(isolate, locationValue);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
            auto transpose = args[1]->BooleanValue(isolate);
            auto data = args[2];

            if (data->IsFloat32Array()) {
                auto buf = data.As<v8::TypedArray>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
                auto data = static_cast<float *>((void*) data_ptr);
                
                
                canvas_native_webgl2_uniform_matrix2x3fv(
                        location->GetUniformLocation(),
                        transpose,
                        data, size,
                        ptr->GetState()
                );
            } else if (data->IsArray()) {
                std::vector<float> buf;
                auto array = data.As<v8::Array>();
                auto len = array->Length();
                for (int i = 0; i < len; i++) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(""));
                    }
                }

            
                canvas_native_webgl2_uniform_matrix2x3fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

}

void WebGL2RenderingContext::UniformMatrix2x4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 2 &&
        args[2]->IsObject()) {
        auto locationValue = args[0];
        auto type = GetNativeType(isolate, locationValue);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
            auto transpose = args[1]->BooleanValue(isolate);
            auto data = args[2];

            if (data->IsFloat32Array()) {
                auto buf = data.As<v8::TypedArray>();
                
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
                auto data = static_cast<float *>((void*) data_ptr);
                
                
                canvas_native_webgl2_uniform_matrix2x4fv(
                        location->GetUniformLocation(),
                        transpose,
                        data, size,
                        ptr->GetState()
                );
            } else if (data->IsArray()) {
                std::vector<float> buf;
                auto array = data.As<v8::Array>();
                auto len = array->Length();
                for (int i = 0; i < len; i++) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(""));
                    }
                }

               

                canvas_native_webgl2_uniform_matrix2x4fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

}

void WebGL2RenderingContext::UniformMatrix3x2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 2 &&
        args[2]->IsObject()) {
        auto locationValue = args[0];
        auto type = GetNativeType(isolate, locationValue);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
            auto transpose = args[1]->BooleanValue(isolate);
            auto data = args[2];

            if (data->IsFloat32Array()) {
                auto buf = data.As<v8::TypedArray>();
               
                
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
                auto data = static_cast<float *>((void*) data_ptr);
                
                
                canvas_native_webgl2_uniform_matrix3x2fv(
                        location->GetUniformLocation(),
                        transpose,
                        data, size,
                        ptr->GetState()
                );
            } else if (data->IsArray()) {
                std::vector<float> buf;
                auto array = data.As<v8::Array>();
                auto len = array->Length();
                for (int i = 0; i < len; i++) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(""));
                    }
                }

               

                canvas_native_webgl2_uniform_matrix3x2fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

}

void WebGL2RenderingContext::UniformMatrix3x4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 2 &&
        args[2]->IsObject()) {
        auto locationValue = args[0];
        auto type = GetNativeType(isolate, locationValue);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
            auto transpose = args[1]->BooleanValue(isolate);
            auto data = args[2];

            if (data->IsFloat32Array()) {
                auto buf = data.As<v8::TypedArray>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
                auto data = static_cast<float *>((void*) data_ptr);
                
                
                canvas_native_webgl2_uniform_matrix3x4fv(
                        location->GetUniformLocation(),
                        transpose,
                        data, size,
                        ptr->GetState()
                );
            } else if (data->IsArray()) {
                std::vector<float> buf;
                auto array = data.As<v8::Array>();
                auto len = array->Length();
                for (int i = 0; i < len; i++) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(""));
                    }
                }

                

                canvas_native_webgl2_uniform_matrix3x4fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

}

void WebGL2RenderingContext::UniformMatrix4x2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 2 &&
        args[2]->IsObject()) {
        auto locationValue = args[0];
        auto type = GetNativeType(isolate, locationValue);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
            auto transpose = args[1]->BooleanValue(isolate);
            auto data = args[2];

            if (data->IsFloat32Array()) {
                auto buf = data.As<v8::TypedArray>();
                
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
                auto data = static_cast<float *>((void*) data_ptr);
                
                canvas_native_webgl2_uniform_matrix4x2fv(
                        location->GetUniformLocation(),
                        transpose,
                        data, size,
                        ptr->GetState()
                );
            } else if (data->IsArray()) {
                std::vector<float> buf;
                auto array = data.As<v8::Array>();
                auto len = array->Length();
                for (int i = 0; i < len; i++) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(""));
                    }
                }

            

                canvas_native_webgl2_uniform_matrix4x2fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

}

void WebGL2RenderingContext::UniformMatrix4x3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 2 &&
        args[2]->IsObject()) {
        auto locationValue = args[0];
        auto type = GetNativeType(isolate, locationValue);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
            auto transpose = args[1]->BooleanValue(isolate);
            auto data = args[2];

            if (data->IsFloat32Array()) {
                auto buf = data.As<v8::TypedArray>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
                auto data = static_cast<float *>((void*) data_ptr);
                
                canvas_native_webgl2_uniform_matrix4x3fv(
                        location->GetUniformLocation(),
                        transpose,
                        data, size,
                        ptr->GetState()
                );
            } else if (data->IsArray()) {
                std::vector<float> buf;
                auto array = data.As<v8::Array>();
                auto len = array->Length();
                for (int i = 0; i < len; i++) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(""));
                    }
                }

         
                canvas_native_webgl2_uniform_matrix4x3fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

}

void WebGL2RenderingContext::VertexAttribDivisor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 1) {
        auto index = args[0]->NumberValue(
                context).ToChecked();
        auto divisor = args[1]->NumberValue(
                context).ToChecked();
        canvas_native_webgl2_vertex_attrib_divisor(
                (uint32_t) index,
                (uint32_t) divisor,
                ptr->GetState()
        );
    }

}

void WebGL2RenderingContext::VertexAttribI4i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 4) {
        auto index = args[0]->NumberValue(
                context).ToChecked();
        auto v0 = args[1]->NumberValue(
                context).ToChecked();
        auto v1 = args[2]->NumberValue(
                context).ToChecked();
        auto v2 = args[3]->NumberValue(
                context).ToChecked();
        auto v3 = args[4]->NumberValue(
                context).ToChecked();
        canvas_native_webgl2_vertex_attrib_i4i(
                (uint32_t) index,
                (int32_t) v0,
                (int32_t) v1,
                (int32_t) v2,
                (int32_t) v3,
                ptr->GetState()
        );
    }

}

void WebGL2RenderingContext::VertexAttribI4iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 1 &&
        args[1]->IsObject()) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto value = args[1];
        if (value->IsInt32Array()) {
            auto buf = value.As<v8::TypedArray>();
            
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<int32_t *>((void*) data_ptr);
            
            
            canvas_native_webgl2_vertex_attrib_i4iv(
                    index,
                    data, size,
                    ptr->GetState()
            );
        } else if (value->IsArray()) {
            auto array = value.As<v8::Array>();
            auto len = array->Length();
            std::vector<int32_t> buf;
            buf.reserve(len);
            for (int i = 0; i < len; i++) {
                auto item = (int32_t) array->Get(
                        context,
                        i).ToLocalChecked()->NumberValue(
                        context).ToChecked();
                buf.push_back(item);
            }

           
            canvas_native_webgl2_vertex_attrib_i4iv(
                    index,
                    buf.data(), buf.size(),
                    ptr->GetState()
            );
        }
    }

}

void WebGL2RenderingContext::VertexAttribI4ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 4) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto v0 = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        auto v1 = (uint32_t) args[2]->NumberValue(
                context).ToChecked();
        auto v2 = (uint32_t) args[3]->NumberValue(
                context).ToChecked();
        auto v3 = (uint32_t) args[4]->NumberValue(
                context).ToChecked();

        canvas_native_webgl2_vertex_attrib_i4ui(
                index,
                v0,
                v1,
                v2,
                v3,
                ptr->GetState()
        );
    }

}

void WebGL2RenderingContext::VertexAttribI4uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGL2RenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 1 &&
        args[1]->IsObject()) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto value = args[1];
        if (value->IsUint32Array()) {
            auto buf = value.As<v8::TypedArray>();
            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = (uint8_t*) array->GetBackingStore()->Data() + offset;
            auto data = static_cast<uint32_t *>((void*) data_ptr);
            
            
            canvas_native_webgl2_vertex_attrib_i4uiv(
                    index,
                    data, size,
                    ptr->GetState()
            );
        } else if (value->IsArray()) {
            auto array = value.As<v8::Array>();
            auto len = array->Length();
            std::vector<uint32_t> buf;
            buf.reserve(len);
            for (int i = 0; i < len; i++) {
                auto item = (uint32_t) array->Get(
                        context,
                        i).ToLocalChecked()->NumberValue(
                        context).ToChecked();
                buf.push_back(item);
            }

            
            canvas_native_webgl2_vertex_attrib_i4uiv(
                    index,
                                                     buf.data(), buf.size(),
                    ptr->GetState()
            );
        }
    }

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


void WebGL2RenderingContext::SetMethods(v8::Isolate *isolate,
                                        const v8::Local<v8::ObjectTemplate> &tmpl) {

    tmpl->Set(
            ConvertToV8String(isolate, "beginQuery"),
            v8::FunctionTemplate::New(isolate, &BeginQuery)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "beginTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &BeginTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindBufferBase"),
            v8::FunctionTemplate::New(isolate, &BindBufferBase)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindBufferRange"),
            v8::FunctionTemplate::New(isolate, &BindBufferRange)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindSampler"),
            v8::FunctionTemplate::New(isolate, &BindSampler)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &BindTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindVertexArray"),
            v8::FunctionTemplate::New(isolate, &BindVertexArray)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "blitFramebuffer"),
            v8::FunctionTemplate::New(isolate, &BlitFramebuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clearBufferfi"),
            v8::FunctionTemplate::New(isolate, &ClearBufferfi)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clearBufferfv"),
            v8::FunctionTemplate::New(isolate, &ClearBufferfv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clearBufferiv"),
            v8::FunctionTemplate::New(isolate, &ClearBufferiv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clearBufferuiv"),
            v8::FunctionTemplate::New(isolate, &ClearBufferuiv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clientWaitSync"),
            v8::FunctionTemplate::New(isolate, &ClientWaitSync)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "compressedTexSubImage3D"),
            v8::FunctionTemplate::New(isolate, &CompressedTexSubImage3D)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "copyBufferSubData"),
            v8::FunctionTemplate::New(isolate, &CopyBufferSubData)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "copyTexSubImage3D"),
            v8::FunctionTemplate::New(isolate, &CopyTexSubImage3D)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createQuery"),
            v8::FunctionTemplate::New(isolate, &CreateQuery)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createSampler"),
            v8::FunctionTemplate::New(isolate, &CreateSampler)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &CreateTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createVertexArray"),
            v8::FunctionTemplate::New(isolate, &CreateVertexArray)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteQuery"),
            v8::FunctionTemplate::New(isolate, &DeleteQuery)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "deleteSampler"),
            v8::FunctionTemplate::New(isolate, &DeleteSampler)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteSync"),
            v8::FunctionTemplate::New(isolate, &DeleteSync)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &DeleteTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteVertexArray"),
            v8::FunctionTemplate::New(isolate, &DeleteVertexArray)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "drawArraysInstanced"),
            v8::FunctionTemplate::New(isolate, &DrawArraysInstanced)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "drawBuffers"),
            v8::FunctionTemplate::New(isolate, &DrawBuffers)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "drawElementsInstanced"),
            v8::FunctionTemplate::New(isolate, &DrawElementsInstanced)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "drawRangeElements"),
            v8::FunctionTemplate::New(isolate, &DrawRangeElements)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "endQuery"),
            v8::FunctionTemplate::New(isolate, &EndQuery)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "endTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &EndTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "fenceSync"),
            v8::FunctionTemplate::New(isolate, &FenceSync)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "framebufferTextureLayer"),
            v8::FunctionTemplate::New(isolate, &FramebufferTextureLayer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform1ui"),
            v8::FunctionTemplate::New(isolate, &Uniform1ui)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform1uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform1uiv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform2ui"),
            v8::FunctionTemplate::New(isolate, &Uniform2ui)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform2uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform2uiv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform3ui"),
            v8::FunctionTemplate::New(isolate, &Uniform3ui)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform3uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform3uiv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform4ui"),
            v8::FunctionTemplate::New(isolate, &Uniform4ui)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform4uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform4uiv)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniformBlockBinding"),
            v8::FunctionTemplate::New(isolate, &UniformBlockBinding)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix2x3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix2x3fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix2x4fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix2x4fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix3x2fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix3x2fv)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix3x4fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix3x4fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix4x2fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4x2fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix4x3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4x3fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix4x3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4x3fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttribDivisor"),
            v8::FunctionTemplate::New(isolate, &VertexAttribDivisor)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttribI4i"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4i)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttribI4iv"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4iv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttribI4ui"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4ui)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttribI4uiv"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4uiv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getActiveUniformBlockName"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniformBlockName)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getActiveUniformBlockParameter"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniformBlockParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getActiveUniforms"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniforms)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getBufferSubData"),
            v8::FunctionTemplate::New(isolate, &GetBufferSubData)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getFragDataLocation"),
            v8::FunctionTemplate::New(isolate, &GetFragDataLocation)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getIndexedParameter"),
            v8::FunctionTemplate::New(isolate, &GetIndexedParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getInternalformatParameter"),
            v8::FunctionTemplate::New(isolate, &GetInternalformatParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getParameter"),
            v8::FunctionTemplate::New(isolate, &GetParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getQueryParameter"),
            v8::FunctionTemplate::New(isolate, &GetQueryParameter)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getQuery"),
            v8::FunctionTemplate::New(isolate, &GetQuery)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getSamplerParameter"),
            v8::FunctionTemplate::New(isolate, &GetSamplerParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getSyncParameter"),
            v8::FunctionTemplate::New(isolate, &GetSyncParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getTransformFeedbackVarying"),
            v8::FunctionTemplate::New(isolate, &GetTransformFeedbackVarying)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getUniformBlockIndex"),
            v8::FunctionTemplate::New(isolate, &GetUniformBlockIndex)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getUniformIndices"),
            v8::FunctionTemplate::New(isolate, &GetUniformIndices)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "invalidateFramebuffer"),
            v8::FunctionTemplate::New(isolate, &InvalidateFramebuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "invalidateSubFramebuffer"),
            v8::FunctionTemplate::New(isolate, &InvalidateSubFramebuffer)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "isQuery"),
            v8::FunctionTemplate::New(isolate, &IsQuery)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "isSampler"),
            v8::FunctionTemplate::New(isolate, &IsSampler)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "isSync"),
            v8::FunctionTemplate::New(isolate, &IsSync)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "isTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &IsTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "isVertexArray"),
            v8::FunctionTemplate::New(isolate, &IsVertexArray)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "pauseTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &PauseTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "readBuffer"),
            v8::FunctionTemplate::New(isolate, &ReadBuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "renderbufferStorageMultisample"),
            v8::FunctionTemplate::New(isolate, &RenderbufferStorageMultisample)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "resumeTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &ResumeTransformFeedback)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "samplerParameterf"),
            v8::FunctionTemplate::New(isolate, &SamplerParameterf)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "samplerParameteri"),
            v8::FunctionTemplate::New(isolate, &SamplerParameteri)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "texImage3D"),
            v8::FunctionTemplate::New(isolate, &TexImage3D)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "texStorage2D"),
            v8::FunctionTemplate::New(isolate, &TexStorage2D)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "texStorage3D"),
            v8::FunctionTemplate::New(isolate, &TexStorage3D)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "texSubImage3D"),
            v8::FunctionTemplate::New(isolate, &TexSubImage3D)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "transformFeedbackVaryings"),
            v8::FunctionTemplate::New(isolate, &TransformFeedbackVaryings)
    );
}

void WebGL2RenderingContext::SetProps(v8::Isolate *isolate,
                                      const v8::Local<v8::ObjectTemplate> &webgl2RenderingContextTpl) {

}
