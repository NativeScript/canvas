//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGL2RenderingContext.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

WebGL2RenderingContext::WebGL2RenderingContext(rust::Box<WebGLState> state) : WebGLRenderingContextBase(
        std::move(state)) {}

WebGL2RenderingContext::~WebGL2RenderingContext() {}

WebGL2RenderingContext *WebGL2RenderingContext::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = GetPointerBase(object);
    if (ptr == nullptr) {
        return nullptr;
    }
    return (WebGL2RenderingContext *) (ptr);
}

void WebGL2RenderingContext::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    WebGLQuery::Init(isolate);
    WebGLSampler::Init(isolate);
    WebGLSyncImpl::Init(isolate);
    WebGLTransformFeedback::Init(isolate);
    WebGLVertexArrayObject::Init(isolate);

    auto ctorFunc = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGL2RenderingContext"),
                ctorFunc->GetFunction(context).ToLocalChecked());
    auto funcTpl = v8::FunctionTemplate::New(isolate, &InstanceFromPointer);
    global->Set(context, Helpers::ConvertToV8String(isolate, "__getWebGL2RenderingContext"),
                funcTpl->GetFunction(context).ToLocalChecked());
}

void WebGL2RenderingContext::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

void WebGL2RenderingContext::InstanceFromPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();

    if (Helpers::IsObject(args[0])) {
        auto config = args[0]->ToObject(context).ToLocalChecked();
        std::string version("none");
        auto alpha = true;
        auto antialias = true;
        auto depth = true;
        auto fail_if_major_performance_caveat = false;
        std::string power_preference("default");
        auto premultiplied_alpha = true;
        auto preserve_drawing_buffer = false;
        auto stencil = false;
        auto desynchronized = false;
        auto xr_compatible = false;

        auto versionMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "version"));
        if (!versionMaybe.IsEmpty()) {
            auto versionValue = versionMaybe.ToLocalChecked();
            version = Helpers::ConvertFromV8String(isolate, versionValue->ToString(context).ToLocalChecked());
        }


        auto alphaMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "alpha"));
        if (!alphaMaybe.IsEmpty()) {
            auto alphaLocal = alphaMaybe.ToLocalChecked();
            if (alphaLocal->IsBoolean()) {
                alpha = alphaLocal->BooleanValue(isolate);
            }
        }

        auto antialiasMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "antialias"));
        if (!antialiasMaybe.IsEmpty()) {
            auto antialiasLocal = antialiasMaybe.ToLocalChecked();
            if (antialiasLocal->IsBoolean()) {
                antialias = antialiasLocal->BooleanValue(isolate);
            }
        }

        auto failIfMajorPerformanceCaveatMaybe = config->Get(context, Helpers::ConvertToV8String(isolate,
                                                                                                 "failIfMajorPerformanceCaveat"));
        if (!failIfMajorPerformanceCaveatMaybe.IsEmpty()) {
            auto failIfMajorPerformanceCaveatLocal = failIfMajorPerformanceCaveatMaybe.ToLocalChecked();
            if (failIfMajorPerformanceCaveatLocal->IsBoolean()) {
                fail_if_major_performance_caveat = failIfMajorPerformanceCaveatLocal->BooleanValue(isolate);
            }
        }

        auto powerPreferenceMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "powerPreference"));
        if (!powerPreferenceMaybe.IsEmpty()) {
            auto powerPreferenceLocal = powerPreferenceMaybe.ToLocalChecked();
            if (powerPreferenceLocal->IsString()) {
                power_preference = Helpers::ConvertFromV8String(isolate, powerPreferenceLocal);
            }
        }

        auto premultipliedAlphaMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "premultipliedAlpha"));
        if (!premultipliedAlphaMaybe.IsEmpty()) {
            auto premultipliedAlphaLocal = premultipliedAlphaMaybe.ToLocalChecked();
            if (premultipliedAlphaLocal->IsBoolean()) {
                premultiplied_alpha = premultipliedAlphaLocal->BooleanValue(isolate);
            }
        }

        auto preserveDrawingBufferMaybe = config->Get(context,
                                                      Helpers::ConvertToV8String(isolate, "preserveDrawingBuffer"));
        if (!preserveDrawingBufferMaybe.IsEmpty()) {
            auto preserveDrawingBufferLocal = preserveDrawingBufferMaybe.ToLocalChecked();
            if (preserveDrawingBufferLocal->IsBoolean()) {
                preserve_drawing_buffer = preserveDrawingBufferLocal->BooleanValue(isolate);
            }
        }

        auto stencilMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "stencil"));
        if (!stencilMaybe.IsEmpty()) {
            auto stencilLocal = stencilMaybe.ToLocalChecked();
            if (stencilLocal->IsBoolean()) {
                stencil = stencilLocal->BooleanValue(isolate);
            }
        }

        auto desynchronizedMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "desynchronized"));
        if (!desynchronizedMaybe.IsEmpty()) {
            auto desynchronizedLocal = desynchronizedMaybe.ToLocalChecked();
            if (desynchronizedLocal->IsBoolean()) {
                desynchronized = desynchronizedLocal->BooleanValue(isolate);
            }
        }

        auto xrCompatibleMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "xrCompatible"));
        if (!xrCompatibleMaybe.IsEmpty()) {
            auto xrCompatibleLocal = xrCompatibleMaybe.ToLocalChecked();
            if (xrCompatibleLocal->IsBoolean()) {
                xr_compatible = xrCompatibleLocal->BooleanValue(isolate);
            }
        }

        if (version != "v1" && version != "v2") {
            args.GetReturnValue().SetUndefined();
            return;
        } else {
            auto cache = Caches::Get(isolate);

            WebGL2RenderingContext *renderingContext;

            if (args.Length() == 7) {
                auto width = args[1];
                auto height = args[2];
                auto density = args[3];
                auto fontColor = args[4];
                auto ppi = args[5];
                auto direction = args[6];
                auto ctx = canvas_native_webgl_create_no_window(
                        width->Int32Value(context).ToChecked(),
                        height->Int32Value(context).ToChecked(),
                        rust::Str(version.c_str(), version.size()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(power_preference.c_str(), power_preference.size()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible,
                        false
                );

                renderingContext = new WebGL2RenderingContext(std::move(ctx));
            } else {
                auto ctx = canvas_native_webgl_create(
                        rust::Str(version.c_str(), version.size()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(power_preference.c_str(), power_preference.size()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible
                );
                renderingContext = new WebGL2RenderingContext(std::move(ctx));
            }


            auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(renderingContext));
            auto raf_callback = new OnRafCallback(ctx_ptr, 2);
            auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
            auto raf = canvas_native_raf_create(raf_callback_ptr);
            renderingContext->SetRaf(std::make_shared<RafImpl>(raf_callback, raf_callback_ptr, std::move(raf)));

            auto _raf = renderingContext->GetRaf();
            canvas_native_raf_start(_raf->GetRaf());

            auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
            Helpers::SetInstanceType(isolate, ret, ObjectType::WebGL2RenderingContext);
            auto ext = v8::External::New(isolate, renderingContext);
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
        }
        return;
    }
    args.GetReturnValue().SetUndefined();
}

v8::Local<v8::FunctionTemplate> WebGL2RenderingContext::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->WebGL2RenderingContextTmpl.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }

    auto context = isolate->GetCurrentContext();
    auto webgl2RenderingContextFunc = v8::FunctionTemplate::New(isolate, &Create);
    webgl2RenderingContextFunc->SetClassName(Helpers::ConvertToV8String(isolate, "WebGL2RenderingContext"));
    webgl2RenderingContextFunc->InstanceTemplate()->SetInternalFieldCount(1);

    auto webgl2RenderingContextTpl = webgl2RenderingContextFunc->PrototypeTemplate();

    WebGLRenderingContext::SetConstants(isolate, webgl2RenderingContextTpl);
    SetConstants(isolate, webgl2RenderingContextTpl);

    WebGLRenderingContext::SetProps(isolate, webgl2RenderingContextTpl);
    SetProps(isolate, webgl2RenderingContextTpl);

    WebGLRenderingContext::SetMethods(isolate, webgl2RenderingContextTpl);
    SetMethods(isolate, webgl2RenderingContextTpl);


    cache->WebGL2RenderingContextTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate,
                                                                                               webgl2RenderingContextFunc);
    return webgl2RenderingContextFunc;
}

void WebGL2RenderingContext::BeginQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto query = args[1];

    if (target->IsNumber() && Helpers::GetInstanceType(isolate, query) == ObjectType::WebGLQuery) {
        auto queryValue = Helpers::GetPrivate(isolate, query.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_begin_query(
                target->Uint32Value(context).ToChecked(),
                queryValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BeginTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    canvas_native_webgl2_begin_transform_feedback(
            args[0]->Uint32Value(context).ToChecked(),
            ptr->GetState()
    );
}

void WebGL2RenderingContext::BindBufferBase(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto index = args[1];
    auto buffer = args[2];
    if (target->IsNumber() && index->IsNumber() &&
        Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
        auto bufferValue = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_bind_buffer_base(
                target->Uint32Value(context).ToChecked(),
                index->Uint32Value(context).ToChecked(),
                bufferValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }

}

void WebGL2RenderingContext::BindBufferRange(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto index = args[1];
    auto buffer = args[2];
    auto offset = args[3];
    auto size = args[4];
    if (args.Length() > 4) {
        if (target->IsNumber() && index->IsNumber() &&
            Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
            auto bufferValue = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance")->ToUint32(context);
            canvas_native_webgl2_bind_buffer_range(
                    target->Uint32Value(context).ToChecked(),
                    index->Uint32Value(context).ToChecked(),
                    bufferValue.ToLocalChecked()->Value(),
                    static_cast<ssize_t>(offset->IntegerValue(context).ToChecked()),
                    static_cast<ssize_t>(size->IntegerValue(context).ToChecked()),
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::BindSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto unit = args[0];
    auto sampler = args[1];

    if (unit->IsNumber() && Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSampler) {
        auto samplerValue = Helpers::GetPrivate(isolate, sampler.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_bind_sampler(
                unit->Uint32Value(context).ToChecked(),
                samplerValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BindTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto transformFeedback = args[1];

    if (target->IsNumber() &&
        Helpers::GetInstanceType(isolate, transformFeedback) == ObjectType::WebGLTransformFeedback) {
        auto transformFeedbackValue = Helpers::GetPrivate(isolate, transformFeedback.As<v8::Object>(),
                                                          "instance")->ToUint32(context);
        canvas_native_webgl2_bind_transform_feedback(
                target->Uint32Value(context).ToChecked(),
                transformFeedbackValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }

}

void WebGL2RenderingContext::BindVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto vertexArray = args[0];

    if (vertexArray->IsNull()) {
        canvas_native_webgl2_bind_vertex_array(
                0,
                ptr->GetState()
        );
        return;
    }

    if (Helpers::GetInstanceType(isolate, vertexArray) == ObjectType::WebGLVertexArrayObject) {
        auto vertexArrayValue = Helpers::GetPrivate(isolate, vertexArray.As<v8::Object>(), "instance")->ToUint32(
                context);
        canvas_native_webgl2_bind_vertex_array(
                vertexArrayValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::BlitFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    auto srcX0 = args[0];
    auto srcY0 = args[1];

    auto srcX1 = args[2];
    auto srcY1 = args[3];

    auto dstX0 = args[4];
    auto dstY0 = args[5];

    auto dstX1 = args[6];
    auto dstY1 = args[7];

    auto mask = args[8];
    auto filter = args[9];

    if (args.Length() > 9) {
        canvas_native_webgl2_blit_framebuffer(
                srcX0->Int32Value(context).ToChecked(),
                srcY0->Int32Value(context).ToChecked(),
                srcX1->Int32Value(context).ToChecked(),
                srcY1->Int32Value(context).ToChecked(),
                dstX0->Int32Value(context).ToChecked(),
                dstY0->Int32Value(context).ToChecked(),
                dstX1->Int32Value(context).ToChecked(),
                dstY1->Int32Value(context).ToChecked(),
                mask->Uint32Value(context).ToChecked(),
                filter->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::ClearBufferfi(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto buffer = args[0];
    auto drawbuffer = args[1];
    auto depth = args[2];
    auto stencil = args[3];

    if (args.Length() > 3 && Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
        auto bufferValue = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_clear_bufferfi(
                bufferValue.ToLocalChecked()->Value(),
                drawbuffer->Int32Value(context).ToChecked(),
                static_cast<float>(depth->NumberValue(context).ToChecked()),
                stencil->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::ClearBufferfv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto buffer = args[0];
    auto drawbuffer = args[1];
    auto values = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
        auto bufferValue = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance")->ToUint32(context);
        if (values->IsArray()) {
            auto array = values.As<v8::Array>();
            std::vector<float> buf;
            auto len = array->Length();
            for (int j = 0; j < len; ++j) {
                auto item = Helpers::ObjectGet(context, array, j);
                if (item.IsEmpty()) {
                    buf.push_back(std::nanf(""));
                } else {
                    buf.push_back(
                            static_cast<float>(item->NumberValue(context).ToChecked())
                    );
                }
            }

            rust::Slice<const float> slice(buf.data(), buf.size());
            canvas_native_webgl2_clear_bufferfv(
                    bufferValue.ToLocalChecked()->Value(),
                    drawbuffer->Int32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );

        } else if (values->IsFloat32Array()) {
            auto buff = values.As<v8::TypedArray>();
            auto array = buff->Buffer();
            auto store = array->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data()) + buff->ByteOffset();
            rust::Slice<const float> slice(reinterpret_cast<float *>(data), buff->Length());
            canvas_native_webgl2_clear_bufferfv(
                    bufferValue.ToLocalChecked()->Value(),
                    drawbuffer->Int32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::ClearBufferiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto buffer = args[0];
    auto drawbuffer = args[1];
    auto values = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
        auto bufferValue = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance")->ToUint32(context);
        if (values->IsArray()) {
            auto array = values.As<v8::Array>();
            std::vector <int32_t> buf;
            auto len = array->Length();
            for (int j = 0; j < len; ++j) {
                auto item = Helpers::ObjectGet(context, array, j);
                buf.push_back(
                        item->Int32Value(context).ToChecked()
                );
            }

            rust::Slice<const int32_t> slice(buf.data(), buf.size());
            canvas_native_webgl2_clear_bufferiv(
                    bufferValue.ToLocalChecked()->Value(),
                    drawbuffer->Int32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );

        } else if (values->IsInt32Array()) {
            auto slice = Helpers::GetTypedArrayData<const int32_t>(values.As<v8::TypedArray>());
            canvas_native_webgl2_clear_bufferiv(
                    bufferValue.ToLocalChecked()->Value(),
                    drawbuffer->Int32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::ClearBufferuiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto buffer = args[0];
    auto drawbuffer = args[1];
    auto values = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
        auto bufferValue = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance")->ToUint32(context);
        if (values->IsArray()) {
            auto array = values.As<v8::Array>();
            std::vector <uint32_t> buf;
            auto len = array->Length();
            for (int j = 0; j < len; ++j) {
                auto item = Helpers::ObjectGet(context, array, j);
                buf.push_back(
                        item->Uint32Value(context).ToChecked()
                );
            }

            rust::Slice<const uint32_t> slice(buf.data(), buf.size());
            canvas_native_webgl2_clear_bufferuiv(
                    bufferValue.ToLocalChecked()->Value(),
                    drawbuffer->Int32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );

        } else if (values->IsUint32Array()) {
            auto slice = Helpers::GetTypedArrayData<const uint32_t>(values.As<v8::TypedArray>());
            canvas_native_webgl2_clear_bufferuiv(
                    bufferValue.ToLocalChecked()->Value(),
                    drawbuffer->Int32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );
        }
    }
}

void WebGL2RenderingContext::ClientWaitSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    auto sync = args[0];
    auto flags = args[1];
    auto timeout = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, sync) == ObjectType::WebGLSync) {
        auto syncValue = WebGLSyncImpl::GetPointer(sync->ToObject(context).ToLocalChecked());
        auto ret = canvas_native_webgl2_client_wait_sync(
                syncValue->GetSync(),
                flags->Uint32Value(context).ToChecked(),
                static_cast<ssize_t>(timeout->IntegerValue(context).ToChecked()),
                ptr->GetState()
        );
        args.GetReturnValue().Set(ret);
        return;
    }
    // todo decide if WAIT_FAILED should be returned here
    args.GetReturnValue().SetUndefined();
}

void WebGL2RenderingContext::CompressedTexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    auto target = args[0];
    auto level = args[1];
    auto xoffset = args[2];
    auto yoffset = args[3];
    auto zoffset = args[4];
    auto width = args[5];
    auto height = args[6];
    auto depth = args[7];
    auto format = args[8];
    auto imageSizeOrBuf = args[9];

    if (imageSizeOrBuf->IsArrayBufferView()) {
        auto buf = imageSizeOrBuf.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        size_t srcOffset = 0;
        auto srcOffsetVal = args[10];
        if (srcOffsetVal->IsNumber()) {
            srcOffset = srcOffsetVal->IntegerValue(context).ToChecked();
        }
        size_t srcLengthOverride = 0;
        auto srcLengthOverrideVal = args[11];
        if (srcLengthOverrideVal->IsNumber()) {
            srcLengthOverride = srcLengthOverrideVal->IntegerValue(context).ToChecked();
        }

        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const uint8_t> slice(data, store->ByteLength());

        canvas_native_webgl2_compressed_tex_sub_image3d(
                target->Uint32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                xoffset->Int32Value(context).ToChecked(),
                yoffset->Int32Value(context).ToChecked(),
                zoffset->Int32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                depth->Int32Value(context).ToChecked(),
                format->Uint32Value(context).ToChecked(),
                slice,
                srcOffset,
                srcLengthOverride,
                ptr->GetState()
        );
    } else {
        auto offset = args[10];
        canvas_native_webgl2_compressed_tex_sub_image3d_none(
                target->Uint32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                xoffset->Int32Value(context).ToChecked(),
                yoffset->Int32Value(context).ToChecked(),
                zoffset->Int32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                depth->Int32Value(context).ToChecked(),
                format->Uint32Value(context).ToChecked(),
                imageSizeOrBuf->Int32Value(context).ToChecked(),
                offset->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::CopyBufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto readTarget = args[0];
    auto writeTarget = args[1];
    auto readOffset = args[2];
    auto writeOffset = args[3];
    auto size = args[4];
    if (args.Length() > 4) {
        canvas_native_webgl2_copy_buffer_sub_data(
                readTarget->Uint32Value(context).ToChecked(),
                writeTarget->Uint32Value(context).ToChecked(),
                static_cast<ssize_t>(readOffset->IntegerValue(context).ToChecked()),
                static_cast<ssize_t>(writeOffset->IntegerValue(context).ToChecked()),
                static_cast<ssize_t>(size->IntegerValue(context).ToChecked()),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::CopyTexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto level = args[1];
    auto xoffset = args[2];
    auto yoffset = args[3];
    auto zoffset = args[4];
    auto x = args[5];
    auto y = args[6];
    auto width = args[7];
    auto height = args[8];

    if (args.Length() > 8) {
        canvas_native_webgl2_copy_tex_sub_image3d(
                target->Uint32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                xoffset->Int32Value(context).ToChecked(),
                yoffset->Int32Value(context).ToChecked(),
                zoffset->Int32Value(context).ToChecked(),
                x->Int32Value(context).ToChecked(),
                y->Int32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::CreateQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl2_create_query(ptr->GetState());
    args.GetReturnValue().Set(
            WebGLQuery::NewInstance(isolate, ret)
    );
}

void WebGL2RenderingContext::CreateSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl2_create_sampler(ptr->GetState());
    args.GetReturnValue().Set(
            WebGLSampler::NewInstance(isolate, ret)
    );
}

void WebGL2RenderingContext::CreateTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl2_create_transform_feedback(ptr->GetState());
    args.GetReturnValue().Set(
            WebGLTransformFeedback::NewInstance(isolate, ret)
    );
}

void WebGL2RenderingContext::CreateVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl2_create_vertex_array(ptr->GetState());
    args.GetReturnValue().Set(
            WebGLVertexArrayObject::NewInstance(isolate, ret)
    );
}

void WebGL2RenderingContext::DeleteQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto query = args[0];

    if (Helpers::GetInstanceType(isolate, query) == ObjectType::WebGLQuery) {
        auto queryValue = Helpers::GetPrivate(isolate, query.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_delete_query_with_query(
                queryValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::DeleteSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto sampler = args[0];

    if (Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSampler) {
        auto samplerValue = Helpers::GetPrivate(isolate, sampler.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_delete_sampler_with_sampler(
                samplerValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::DeleteSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto sampler = args[0];

    if (Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSync) {
        auto sync = WebGLSyncImpl::GetPointer(sampler->ToObject(context).ToLocalChecked());
        canvas_native_webgl2_delete_sync_with_sync(
                sync->GetSync(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::DeleteTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto transformFeedback = args[0];

    if (Helpers::GetInstanceType(isolate, transformFeedback) == ObjectType::WebGLTransformFeedback) {
        auto transformFeedbackValue = Helpers::GetPrivate(isolate, transformFeedback.As<v8::Object>(),
                                                          "instance")->ToUint32(context);
        canvas_native_webgl2_delete_transform_feedback(
                transformFeedbackValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::DeleteVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto vertexArray = args[0];

    if (Helpers::GetInstanceType(isolate, vertexArray) == ObjectType::WebGLVertexArrayObject) {
        auto vertexArrayValue = Helpers::GetPrivate(isolate, vertexArray.As<v8::Object>(), "instance")->ToUint32(
                context);
        canvas_native_webgl2_delete_vertex_array_with_vertex_array(
                vertexArrayValue.ToLocalChecked()->Value(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::DrawArraysInstanced(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto mode = args[0];
    auto first = args[1];
    auto count = args[2];
    auto instanceCount = args[3];

    if (args.Length() > 3) {
        canvas_native_webgl2_draw_arrays_instanced(
                mode->Uint32Value(context).ToChecked(),
                first->Int32Value(context).ToChecked(),
                count->Int32Value(context).ToChecked(),
                instanceCount->Int32Value(context).ToChecked(),
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }
}

void WebGL2RenderingContext::DrawBuffers(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto buffers = args[0];
    if (buffers->IsArray()) {
        auto array = buffers.As<v8::Array>();
        std::vector <uint32_t> buf;
        auto len = array->Length();
        for (int j = 0; j < len; ++j) {
            auto item = Helpers::ObjectGet(context, array, j);
            buf.push_back(item->Uint32Value(context).ToChecked());
        }
        rust::Slice<const uint32_t> slice(buf.data(), buf.size());
        canvas_native_webgl2_draw_buffers(slice, ptr->GetState());
    }
}

void WebGL2RenderingContext::DrawElementsInstanced(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto mode = args[0];
    auto count = args[1];
    auto type = args[2];
    auto offset = args[3];
    auto instanceCount = args[4];
    if (args.Length() > 4) {
        canvas_native_webgl2_draw_elements_instanced(
                mode->Uint32Value(context).ToChecked(),
                count->Int32Value(context).ToChecked(),
                type->Uint32Value(context).ToChecked(),
                static_cast<ssize_t>(offset->NumberValue(context).ToChecked()),
                instanceCount->Int32Value(context).ToChecked(),
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }
}

void WebGL2RenderingContext::DrawRangeElements(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto mode = args[0];
    auto start = args[1];
    auto end = args[2];
    auto count = args[3];
    auto type = args[4];
    auto offset = args[5];
    if (args.Length() > 5) {
        canvas_native_webgl2_draw_range_elements(
                mode->Uint32Value(context).ToChecked(),
                start->Uint32Value(context).ToChecked(),
                end->Uint32Value(context).ToChecked(),
                count->Int32Value(context).ToChecked(),
                type->Uint32Value(context).ToChecked(),
                static_cast<ssize_t>(offset->IntegerValue(context).ToChecked()),
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }
}

void WebGL2RenderingContext::EndQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    if (target->IsNumber()) {
        canvas_native_webgl2_end_query(target->Uint32Value(context).ToChecked(), ptr->GetState());
    }
}

void WebGL2RenderingContext::EndTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    canvas_native_webgl2_end_transform_feedback(ptr->GetState());
}

void WebGL2RenderingContext::FenceSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto condition = args[0];
    auto flags = args[1];
    if (args.Length() > 1) {
        canvas_native_webgl2_fence_sync(
                condition->Uint32Value(context).ToChecked(),
                flags->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::FramebufferTextureLayer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto attachment = args[1];
    auto texture = args[2];
    auto level = args[3];
    auto layer = args[4];
    if (args.Length() > 4 && Helpers::GetInstanceType(isolate, texture) == ObjectType::WebGLTexture) {
        auto textureValue = Helpers::GetPrivate(isolate, texture.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_framebuffer_texture_layer(
                target->Uint32Value(context).ToChecked(),
                attachment->Uint32Value(context).ToChecked(),
                textureValue.ToLocalChecked()->Value(),
                level->Int32Value(context).ToChecked(),
                layer->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}


void WebGL2RenderingContext::GetActiveUniformBlockName(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto uniformBlockIndex = args[1];
    if (args.Length() > 1 && Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
        auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);
        auto name = canvas_native_webgl2_get_active_uniform_block_name(
                programValue.ToLocalChecked()->Value(),
                uniformBlockIndex->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
        args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, std::string(name.data(), name.size())));
    }

}

void WebGL2RenderingContext::GetActiveUniformBlockParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto uniformBlockIndex = args[1];
    auto pname = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
        auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto ret = canvas_native_webgl2_get_active_uniform_block_parameter(
                programValue.ToLocalChecked()->Value(),
                uniformBlockIndex->Uint32Value(context).ToChecked(),
                pnameValue,
                ptr->GetState()
        );

        switch (pnameValue) {
            case GL_UNIFORM_BLOCK_BINDING:
            case GL_UNIFORM_BLOCK_DATA_SIZE:
            case GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS:
                args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(*ret));
                break;
            case GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: {
                auto value = canvas_native_webgl_result_get_i32_array(*ret);
                auto size = value.size() * sizeof(int32_t);
                auto buf = v8::ArrayBuffer::New(isolate, size);
                auto array = v8::Uint32Array::New(buf, 0, size);
                auto len = value.size();
                for (int j = 0; j < len; ++j) {
                    auto item = value[j];
                    Helpers::ObjectSet(context, array, j, v8::Uint32::New(isolate, item));
                }
                args.GetReturnValue().Set(array);
            }
                break;
            case GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER:
            case GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:
                args.GetReturnValue().Set(
                        canvas_native_webgl_result_get_bool(*ret)
                );
                break;
            default:
                args.GetReturnValue().Set(v8::Null(isolate));
                break;
        }
        return;
    }
    args.GetReturnValue().SetNull();

}

void WebGL2RenderingContext::GetActiveUniforms(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto uniformIndices = args[1];
    auto pname = args[2];
    if (args.Length() > 2 && (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) &&
        uniformIndices->IsArray()) {
        auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto array = uniformIndices.As<v8::Array>();
        std::vector <uint32_t> buf;
        for (int j = 0; j < array->Length(); ++j) {
            auto item = array->Uint32Value(context).ToChecked();
            buf.push_back(item);
        }
        rust::Slice<const uint32_t> slice(buf.data(), buf.size());
        auto ret = canvas_native_webgl2_get_active_uniforms(
                programValue.ToLocalChecked()->Value(),
                slice,
                pnameValue,
                ptr->GetState()
        );

        switch (pnameValue) {
            case GL_UNIFORM_TYPE:
            case GL_UNIFORM_SIZE: {
                auto value = canvas_native_webgl_result_get_u32_array(*ret);
                auto array = v8::Array::New(isolate, value.size());
                auto len = value.size();
                for (int j = 0; j < len; ++j) {
                    auto item = value[j];
                    Helpers::ObjectSet(context, array, j, v8::Uint32::New(isolate, item));
                }
                args.GetReturnValue().Set(array);
            }
                break;
            case GL_UNIFORM_BLOCK_INDEX:
            case GL_UNIFORM_OFFSET:
            case GL_UNIFORM_ARRAY_STRIDE:
            case GL_UNIFORM_MATRIX_STRIDE: {
                auto value = canvas_native_webgl_result_get_i32_array(*ret);
                auto array = v8::Array::New(isolate, value.size());
                auto len = value.size();
                for (int j = 0; j < len; ++j) {
                    auto item = value[j];
                    Helpers::ObjectSet(context, array, j, v8::Int32::New(isolate, item));
                }
                args.GetReturnValue().Set(array);
            }
            case GL_UNIFORM_IS_ROW_MAJOR: {
                auto value = canvas_native_webgl_result_get_bool_array(*ret);
                auto len = value.size();
                auto array = v8::Array::New(isolate, len);
                for (int j = 0; j < len; ++j) {
                    bool item = value[j] == 1;
                    Helpers::ObjectSet(context, array, j, v8::Boolean::New(isolate, item));
                }
                args.GetReturnValue().Set(array);
            }
                break;
            default:
                args.GetReturnValue().SetNull();
                break;
        }

        return;
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetBufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto srcByteOffset = args[1];
    auto dstData = args[2];
    if (args.Length() >= 3 && dstData->IsArrayBufferView()) {
        auto dstOffset = args[3];
        ssize_t dstOffsetValue = 0;
        if (dstOffset->IsNumber()) {
            dstOffsetValue = static_cast<ssize_t>(dstOffset->IntegerValue(context).ToChecked());
        }
        auto length = args[4];
        ssize_t lengthValue = 0;
        if (length->IsNumber()) {
            lengthValue = static_cast<ssize_t>(length->IntegerValue(context).ToChecked());
        }

        auto buf = dstData.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<uint8_t> slice(data, store->ByteLength());

        canvas_native_webgl2_get_buffer_sub_data(
                target->Uint32Value(context).ToChecked(),
                static_cast<ssize_t>(srcByteOffset->IntegerValue(context).ToChecked()),
                slice,
                dstOffsetValue,
                lengthValue,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::GetFragDataLocation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto name = args[1];
    if (args.Length() > 1 && (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) &&
        name->IsString()) {
        auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);

        auto nameValue = Helpers::ConvertFromV8String(isolate, name);


        auto ret = canvas_native_webgl2_get_frag_data_location(
                programValue.ToLocalChecked()->Value(),
                rust::Str(nameValue.c_str(), nameValue.size()),
                ptr->GetState()
        );

        args.GetReturnValue().Set(ret);
        return;
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetIndexedParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto index = args[1];
    if (args.Length() > 1) {
        auto targetValue = target->Uint32Value(context).ToChecked();
        auto ret = canvas_native_webgl2_get_indexed_parameter(
                targetValue,
                index->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );

        switch (targetValue) {
            case GL_UNIFORM_BUFFER_BINDING:
            case GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: {
                auto buffer = canvas_native_webgl2_indexed_parameter_get_buffer_value(*ret);
                args.GetReturnValue().Set(WebGLBuffer::NewInstance(isolate, buffer));
            }
                break;
            case GL_TRANSFORM_FEEDBACK_BUFFER_SIZE:
            case GL_TRANSFORM_FEEDBACK_BUFFER_START:
            case GL_UNIFORM_BUFFER_SIZE:
            case GL_UNIFORM_BUFFER_START: {
                auto value = canvas_native_webgl2_indexed_parameter_get_value(*ret);
                args.GetReturnValue().Set(v8::Number::New(isolate, static_cast<double>(value)));
            }
                break;
            default:
                args.GetReturnValue().SetNull();
                break;
        }

        return;
    }

    args.GetReturnValue().SetNull();

}

void WebGL2RenderingContext::GetInternalformatParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto internalformat = args[1];
    auto pname = args[2];
    if (args.Length() > 2) {
        auto internalformatValue = internalformat->Uint32Value(context).ToChecked();
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto returnEarly = false;
        switch (internalformatValue) {
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
                auto ab = v8::ArrayBuffer::New(isolate, 0);
                auto array = v8::Int32Array::New(ab, 0, 0);
                args.GetReturnValue().Set(array);
                returnEarly = true;
            }
                break;
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
                args.GetReturnValue().Set(v8::Null(isolate));
                returnEarly = true;
                break;
        }

        if (returnEarly) {
            return;
        }


        auto ret = canvas_native_webgl2_get_internalformat_parameter(
                target->Uint32Value(context).ToChecked(),
                internalformatValue,
                pnameValue,
                ptr->GetState()
        );

        if (pnameValue == GL_SAMPLES) {
            auto value = canvas_native_webgl_result_get_i32_array(*ret);
            auto size = value.size() * sizeof(int32_t);
            auto ab = v8::ArrayBuffer::New(isolate, size);
            auto array = v8::Int32Array::New(ab, 0, size);
            auto len = value.size();
            for (int j = 0; j < len; ++j) {
                auto item = value[j];
                Helpers::ObjectSet(context, array, j, v8::Int32::New(isolate, item));
            }
            args.GetReturnValue().Set(array);

        } else {
            args.GetReturnValue().SetNull();
        }
        return;
    }

    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto pname = args[0];
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto result = canvas_native_webgl2_get_parameter(pname->Uint32Value(context).ToChecked(), ptr->GetState());
        switch (pnameValue) {
            case GL_COPY_READ_BUFFER_BINDING:
            case GL_COPY_WRITE_BUFFER_BINDING:
            case GL_DRAW_FRAMEBUFFER_BINDING:
                args.GetReturnValue().Set(
                        canvas_native_webgl_result_get_i32(*result)
                );
                break;
            default:
                WebGLRenderingContext::GetParameterInternal(args, pnameValue, std::move(result));
                break;
        }
        return;
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetQueryParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto query = args[0];
    auto pname = args[1];
    if (Helpers::GetInstanceType(isolate, query) == ObjectType::WebGLQuery) {
        auto queryValue = Helpers::GetPrivate(isolate, query.As<v8::Object>(), "instance")->ToUint32(context);
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto ret = canvas_native_webgl2_get_query_parameter(queryValue.ToLocalChecked()->Value(), pnameValue,
                                                            ptr->GetState());
        if (pnameValue == GL_QUERY_RESULT) {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(*ret));
            return;
        } else if (pnameValue == GL_QUERY_RESULT_AVAILABLE) {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_u32(*ret));
            return;
        }
    }

    args.GetReturnValue().SetNull();

}

void WebGL2RenderingContext::GetQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto pname = args[1];

    if (args.Length() > 1) {
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto ret = canvas_native_webgl2_get_query(target->Uint32Value(context).ToChecked(), pnameValue,
                                                  ptr->GetState());
        if (pnameValue == GL_CURRENT_QUERY) {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(*ret));
            return;
        }
    }

    args.GetReturnValue().SetNull();
}


void WebGL2RenderingContext::GetSamplerParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto sampler = args[0];
    auto pname = args[1];
    if (args.Length() > 1) {
        if (Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSampler) {
            auto samplerValue = Helpers::GetPrivate(isolate, sampler.As<v8::Object>(), "instance")->ToUint32(context);
            auto pnameValue = pname->Uint32Value(context).ToChecked();
            auto ret = canvas_native_webgl2_get_sampler_parameter(
                    samplerValue.ToLocalChecked()->Value(), pnameValue,
                    ptr->GetState());

            switch (pnameValue) {
                case GL_TEXTURE_MAX_LOD:
                case GL_TEXTURE_MIN_LOD:
                    args.GetReturnValue().Set(static_cast<double>(canvas_native_webgl_result_get_f32(*ret)));
                    break;
                case GL_TEXTURE_COMPARE_FUNC:
                case GL_TEXTURE_COMPARE_MODE:
                case GL_TEXTURE_MAG_FILTER:
                case GL_TEXTURE_MIN_FILTER:
                case GL_TEXTURE_WRAP_R:
                case GL_TEXTURE_WRAP_S:
                case GL_TEXTURE_WRAP_T:
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(*ret));
                    break;
                default:
                    args.GetReturnValue().SetNull();
                    break;
            }

            return;
        }
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetSyncParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto sync = args[0];
    auto pname = args[1];
    if (args.Length() > 1) {
        if (Helpers::GetInstanceType(isolate, sync) == ObjectType::WebGLSync) {
            auto syncVal = WebGLSyncImpl::GetPointer(sync->ToObject(context).ToLocalChecked());
            auto pnameValue = pname->Uint32Value(context).ToChecked();
            auto ret = canvas_native_webgl2_get_sync_parameter(
                    syncVal->GetSync(),
                    pnameValue,
                    ptr->GetState());

            switch (pnameValue) {
                case GL_OBJECT_TYPE:
                case GL_SYNC_STATUS:
                case GL_SYNC_CONDITION:
                case GL_SYNC_FLAGS:
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(*ret));
                    break;
                default:
                    args.GetReturnValue().SetNull();
                    break;
            }

            return;
        }
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetTransformFeedbackVarying(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto index = args[1];
    if (args.Length() > 1) {
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);

            auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                    programValue.ToLocalChecked()->Value(),
                    index->Uint32Value(context).ToChecked(),
                    ptr->GetState()
            );

            if (canvas_native_webgl_active_info_get_is_empty(*ret)) {
                args.GetReturnValue().Set(v8::Null(isolate));
                return;
            }

            args.GetReturnValue().Set(WebGLActiveInfoImpl::NewInstance(isolate, std::move(ret)));

            return;
        }
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::GetUniformBlockIndex(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto index = args[1];
    if (args.Length() > 1) {
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);

            auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                    programValue.ToLocalChecked()->Value(),
                    index->Uint32Value(context).ToChecked(),
                    ptr->GetState()
            );

            if (canvas_native_webgl_active_info_get_is_empty(*ret)) {
                args.GetReturnValue().Set(v8::Null(isolate));
                return;
            }

            args.GetReturnValue().Set(WebGLActiveInfoImpl::NewInstance(isolate, std::move(ret)));

            return;
        }
    }
    args.GetReturnValue().Set(v8::Null(isolate));
}

void WebGL2RenderingContext::GetUniformIndices(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto uniformNames = args[1];
    if (args.Length() > 1) {
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram && uniformNames->IsArray()) {
            auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);
            auto array = uniformNames.As<v8::Array>();
            rust::Vec<rust::Str> store;
            auto len = array->Length();
            for (int j = 0; j < len; ++j) {
                auto name = Helpers::ObjectGet(context, array, j);
                auto nameValue = Helpers::ConvertFromV8String(isolate, name);
                rust::Str val(nameValue.c_str(), nameValue.size());
                store.push_back(val);
            }
            rust::Slice<const rust::Str> buf(store.data(), store.size());
            auto ret = canvas_native_webgl2_get_uniform_indices(programValue.ToLocalChecked()->Value(), buf,
                                                                ptr->GetState());

            auto retSize = ret.size();
            auto result = v8::Array::New(isolate, retSize);
            for (int j = 0; j < retSize; ++j) {
                auto item = ret[j];
                Helpers::ObjectSet(context, result, j, v8::Uint32::New(isolate, item));
            }

            args.GetReturnValue().Set(result);

            return;
        }
    }
    args.GetReturnValue().SetNull();
}

void WebGL2RenderingContext::InvalidateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto attachments = args[1];
    if (args.Length() > 1 && attachments->IsArray()) {
        auto array = attachments.As<v8::Array>();
        std::vector <uint32_t> buf;

        auto len = array->Length();
        for (int j = 0; j < len; ++j) {
            auto item = Helpers::ObjectGet(context, array, j);
            buf.push_back(item->Uint32Value(context).ToChecked());
        }
        rust::Slice<const uint32_t> slice(buf.data(), buf.size());

        canvas_native_webgl2_invalidate_framebuffer(target->Uint32Value(context).ToChecked(), slice,
                                                    ptr->GetState());
        return;
    }
}

void WebGL2RenderingContext::InvalidateSubFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto target = args[0];
    auto attachments = args[1];
    auto x = args[2];
    auto y = args[3];
    auto width = args[4];
    auto height = args[5];
    if (args.Length() > 5 && attachments->IsArray()) {
        auto array = attachments.As<v8::Array>();
        std::vector <uint32_t> buf;

        auto len = array->Length();
        for (int j = 0; j < len; ++j) {
            auto item = Helpers::ObjectGet(context, array, j);
            buf.push_back(item->Uint32Value(context).ToChecked());
        }
        rust::Slice<const uint32_t> slice(buf.data(), buf.size());

        canvas_native_webgl2_invalidate_sub_framebuffer(target->Uint32Value(context).ToChecked(),
                                                        slice,
                                                        x->Int32Value(context).ToChecked(),
                                                        y->Int32Value(context).ToChecked(),
                                                        width->Int32Value(context).ToChecked(),
                                                        height->Int32Value(context).ToChecked(),
                                                        ptr->GetState());
        return;
    }
}

void WebGL2RenderingContext::IsQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto query = args[0];
        if (Helpers::GetInstanceType(isolate, query) == ObjectType::WebGLQuery) {
            auto instance = Helpers::GetPrivate(isolate, query.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl2_is_query(instance->Uint32Value(context).ToChecked(), ptr->GetState());
                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::IsSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto sampler = args[0];
        if (Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSampler) {
            auto instance = Helpers::GetPrivate(isolate, sampler.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl2_is_sampler(instance->Uint32Value(context).ToChecked(),
                                                           ptr->GetState());
                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::IsSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto sync = args[0];
        if (Helpers::GetInstanceType(isolate, sync) == ObjectType::WebGLSync) {
            auto instance = WebGLSyncImpl::GetPointer(sync->ToObject(context).ToLocalChecked());
            auto ret = canvas_native_webgl2_is_sync(instance->GetSync(),
                                                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);

}

void WebGL2RenderingContext::IsTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto transformFeedback = args[0];
        if (Helpers::GetInstanceType(isolate, transformFeedback) == ObjectType::WebGLTransformFeedback) {
            auto instance = Helpers::GetPrivate(isolate, transformFeedback.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl2_is_transform_feedback(instance->Uint32Value(context).ToChecked(),
                                                                      ptr->GetState());
                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::IsVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto vertexArray = args[0];
        if (Helpers::GetInstanceType(isolate, vertexArray) == ObjectType::WebGLVertexArrayObject) {
            auto instance = Helpers::GetPrivate(isolate, vertexArray.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl2_is_vertex_array(instance->Uint32Value(context).ToChecked(),
                                                                ptr->GetState());
                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGL2RenderingContext::PauseTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    canvas_native_webgl2_pause_transform_feedback(ptr->GetState());
}

void WebGL2RenderingContext::ReadBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto src = args[0];
        canvas_native_webgl2_read_buffer(
                src->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::RenderbufferStorageMultisample(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 4) {
        auto target = args[0];
        auto samples = args[1];
        auto internalFormat = args[2];
        auto width = args[3];
        auto height = args[4];
        canvas_native_webgl2_renderbuffer_storage_multisample(
                target->Uint32Value(context).ToChecked(),
                samples->Int32Value(context).ToChecked(),
                internalFormat->Uint32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::ResumeTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    canvas_native_webgl2_resume_transform_feedback(ptr->GetState());
}

void WebGL2RenderingContext::SamplerParameterf(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto sampler = args[0];
        auto pname = args[1];
        auto param = args[2];
        if (Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSampler) {
            auto instance = Helpers::GetPrivate(isolate, sampler.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl2_sampler_parameterf(
                        instance->Uint32Value(context).ToChecked(),
                        pname->Uint32Value(context).ToChecked(),
                        static_cast<float>(param->NumberValue(context).ToChecked()),
                        ptr->GetState());
            }
        }
    }
}

void WebGL2RenderingContext::SamplerParameteri(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto sampler = args[0];
        auto pname = args[1];
        auto param = args[2];
        if (Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSampler) {
            auto instance = Helpers::GetPrivate(isolate, sampler.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl2_sampler_parameteri(
                        instance->Uint32Value(context).ToChecked(),
                        pname->Uint32Value(context).ToChecked(),
                        param->Int32Value(context).ToChecked(),
                        ptr->GetState());
            }
        }
    }
}

void WebGL2RenderingContext::TexImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
// target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, offset: any
// target, level, internalformat, width, height, depth, border, format, type, srcData, srcOffset
// target, level, internalformat, width, height, depth, border, format, type, source

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    if (args.Length() == 10) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto width = args[3];
        auto height = args[4];
        auto depth = args[5];
        auto border = args[6];
        auto format = args[7];
        auto type = args[8];
        auto imageOrPixelsOrOffset = args[9];

        if (imageOrPixelsOrOffset->IsNumber()) {
            canvas_native_webgl2_tex_image3d_none(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Uint32Value(context).ToChecked(),
                    static_cast<ssize_t>(imageOrPixelsOrOffset->IntegerValue(context).ToChecked()),
                    ptr->GetState()
            );
            return;
        }

        if (imageOrPixelsOrOffset->IsArrayBufferView()) {
            auto buf = imageOrPixelsOrOffset.As<v8::ArrayBufferView>();
            auto array = buf->Buffer();
            auto store = array->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
            rust::Slice<const uint8_t> slice(data, store->ByteLength());
            canvas_native_webgl2_tex_image3d(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Uint32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );
            return;
        }

        if (Helpers::GetInstanceType(isolate, imageOrPixelsOrOffset) == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(imageOrPixelsOrOffset->ToObject(context).ToLocalChecked());
            canvas_native_webgl2_tex_image3d_asset(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Uint32Value(context).ToChecked(),
                    asset->GetImageAsset(),
                    ptr->GetState()
            );
        }
    } else if (args.Length() > 10) {

        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto width = args[3];
        auto height = args[4];
        auto depth = args[5];
        auto border = args[6];
        auto format = args[7];
        auto type = args[8];
        auto imageOrPixelsOrOffset = args[9];
        auto srcOffset = args[10];
        size_t srcOffsetValue = 0;
        if (srcOffset->IsNumber()) {
            srcOffsetValue = static_cast<size_t>(srcOffset->IntegerValue(context).ToChecked());
        }

        if (imageOrPixelsOrOffset->IsArrayBufferView()) {
            auto buf = imageOrPixelsOrOffset.As<v8::TypedArray>();
            auto array = buf->Buffer();
            auto store = array->GetBackingStore();

            srcOffsetValue = srcOffsetValue * (buf->ByteLength() / buf->Length());
            if (srcOffsetValue > buf->Length()) {
                return;
            }


            auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
            rust::Slice<const uint8_t> slice(data, store->ByteLength());
            canvas_native_webgl2_tex_image3d_offset(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Uint32Value(context).ToChecked(),
                    slice,
                    srcOffsetValue,
                    ptr->GetState()
            );
            return;
        }
    }

}

void WebGL2RenderingContext::TexStorage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 4) {
        auto target = args[0];
        auto levels = args[1];
        auto internalFormat = args[2];
        auto width = args[3];
        auto height = args[4];
        canvas_native_webgl2_tex_storage2d(
                target->Uint32Value(context).ToChecked(),
                levels->Int32Value(context).ToChecked(),
                internalFormat->Uint32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::TexStorage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 5) {
        auto target = args[0];
        auto levels = args[1];
        auto internalFormat = args[2];
        auto width = args[3];
        auto height = args[4];
        auto depth = args[5];
        canvas_native_webgl2_tex_storage3d(
                target->Uint32Value(context).ToChecked(),
                levels->Int32Value(context).ToChecked(),
                internalFormat->Uint32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                depth->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::TexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    if (args.Length() == 11) {
        auto target = args[0];
        auto level = args[1];
        auto xoffset = args[2];
        auto yoffset = args[3];
        auto zoffset = args[4];
        auto width = args[5];
        auto height = args[6];
        auto depth = args[7];
        auto format = args[8];
        auto type = args[9];
        auto imageOrPixelsOrOffset = args[10];

        if (imageOrPixelsOrOffset->IsNumber()) {
            canvas_native_webgl2_tex_sub_image3d_none(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    zoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    static_cast<ssize_t>(imageOrPixelsOrOffset->IntegerValue(context).ToChecked()),
                    ptr->GetState()
            );
            return;
        }

        if (imageOrPixelsOrOffset->IsArrayBufferView()) {

            auto buf = imageOrPixelsOrOffset.As<v8::ArrayBufferView>();
            auto array = buf->Buffer();
            auto store = array->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
            rust::Slice<const uint8_t> slice(data, store->ByteLength());

            canvas_native_webgl2_tex_sub_image3d(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    zoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );

            return;
        }

        if (Helpers::GetInstanceType(isolate, imageOrPixelsOrOffset) == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(imageOrPixelsOrOffset->ToObject(context).ToLocalChecked());

            canvas_native_webgl2_tex_sub_image3d_asset(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    zoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    asset->GetImageAsset(),
                    ptr->GetState()
            );
        }

    } else if (args.Length() > 11) {
        auto target = args[0];
        auto level = args[1];
        auto xoffset = args[2];
        auto yoffset = args[3];
        auto zoffset = args[4];
        auto width = args[5];
        auto height = args[6];
        auto depth = args[7];
        auto format = args[8];
        auto type = args[9];
        auto imageOrPixelsOrOffset = args[10];

        auto srcOffset = args[11];
        size_t srcOffsetValue = 0;
        if (srcOffset->IsNumber()) {
            srcOffsetValue = static_cast<size_t>(srcOffset->IntegerValue(context).ToChecked());
        }

        if (imageOrPixelsOrOffset->IsArrayBufferView()) {
            auto buf = imageOrPixelsOrOffset.As<v8::TypedArray>();
            auto array = buf->Buffer();
            auto store = array->GetBackingStore();

            srcOffsetValue = srcOffsetValue * (buf->ByteLength() / buf->Length());
            if (srcOffsetValue > buf->Length()) {
                return;
            }


            auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
            rust::Slice<const uint8_t> slice(data, store->ByteLength());
            canvas_native_webgl2_tex_sub_image3d_offset(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    zoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    depth->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    slice,
                    srcOffsetValue,
                    ptr->GetState()
            );
            return;
        }


    }

}

void WebGL2RenderingContext::TransformFeedbackVaryings(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto program = args[0];
        auto varyings = args[1];
        auto bufferMode = args[2];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram && varyings->IsArray()) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {

                auto array = varyings.As<v8::Array>();
                rust::Vec<rust::Str> store;

                auto len = array->Length();
                for (int j = 0; j < len; ++j) {
                    auto name = Helpers::ObjectGet(context, array, j);
                    auto nameValue = Helpers::ConvertFromV8String(isolate, name);
                    rust::Str val(nameValue.data(), nameValue.size());
                    store.push_back(val);
                }

                rust::Slice<const rust::Str> buf(store.data(), store.size());

                canvas_native_webgl2_transform_feedback_varyings(
                        instance->Uint32Value(context).ToChecked(),
                        buf,
                        bufferMode->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGL2RenderingContext::Uniform1ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto v0 = args[1];
    if (args.Length() > 1 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);
        canvas_native_webgl2_uniform1ui(
                locationValue.ToLocalChecked()->Value(),
                v0->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::Uniform1uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto data = args[1];
    if (args.Length() > 1 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsUint32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Uint32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const uint32_t> slice(reinterpret_cast<uint32_t *>(data), len);

        canvas_native_webgl2_uniform1uiv(
                locationValue.ToLocalChecked()->Value(),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::Uniform2ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto v0 = args[1];
    auto v1 = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);
        canvas_native_webgl2_uniform2ui(
                locationValue.ToLocalChecked()->Value(),
                v0->Uint32Value(context).ToChecked(),
                v1->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::Uniform2uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto data = args[1];
    if (args.Length() > 1 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsUint32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Uint32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const uint32_t> slice(reinterpret_cast<uint32_t *>(data), len);

        canvas_native_webgl2_uniform2uiv(
                locationValue.ToLocalChecked()->Value(),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::Uniform3ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto v0 = args[1];
    auto v1 = args[2];
    auto v2 = args[3];
    auto v3 = args[4];
    if (args.Length() > 3 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);
        canvas_native_webgl2_uniform3ui(
                locationValue.ToLocalChecked()->Value(),
                v0->Uint32Value(context).ToChecked(),
                v1->Uint32Value(context).ToChecked(),
                v2->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::Uniform3uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto data = args[1];
    if (args.Length() > 1 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsUint32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Uint32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const uint32_t> slice(reinterpret_cast<uint32_t *>(data), len);

        canvas_native_webgl2_uniform3uiv(
                locationValue.ToLocalChecked()->Value(),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::Uniform4ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto v0 = args[1];
    auto v1 = args[2];
    auto v2 = args[3];
    auto v3 = args[4];
    if (args.Length() > 4 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);
        canvas_native_webgl2_uniform4ui(
                locationValue.ToLocalChecked()->Value(),
                v0->Uint32Value(context).ToChecked(),
                v1->Uint32Value(context).ToChecked(),
                v2->Uint32Value(context).ToChecked(),
                v3->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::Uniform4uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto data = args[1];
    if (args.Length() > 1 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsUint32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Uint32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const uint32_t> slice(reinterpret_cast<uint32_t *>(data), len);

        canvas_native_webgl2_uniform4uiv(
                locationValue.ToLocalChecked()->Value(),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::UniformBlockBinding(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto program = args[0];
    auto uniformBlockIndex = args[1];
    auto uniformBlockBinding = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
        auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance")->ToUint32(context);
        canvas_native_webgl2_uniform_block_binding(
                programValue.ToLocalChecked()->Value(),
                uniformBlockIndex->Uint32Value(context).ToChecked(),
                uniformBlockBinding->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::UniformMatrix2x3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto transpose = args[1];
    auto data = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsFloat32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Float32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const float> slice(reinterpret_cast<float *>(data), len);

        canvas_native_webgl2_uniform_matrix2x3fv(
                locationValue.ToLocalChecked()->Value(),
                transpose->BooleanValue(isolate),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::UniformMatrix2x4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto transpose = args[1];
    auto data = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsFloat32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Float32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const float> slice(reinterpret_cast<float *>(data), len);

        canvas_native_webgl2_uniform_matrix2x4fv(
                locationValue.ToLocalChecked()->Value(),
                transpose->BooleanValue(isolate),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::UniformMatrix3x2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto transpose = args[1];
    auto data = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsFloat32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Float32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const float> slice(reinterpret_cast<float *>(data), len);

        canvas_native_webgl2_uniform_matrix3x2fv(
                locationValue.ToLocalChecked()->Value(),
                transpose->BooleanValue(isolate),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::UniformMatrix3x4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto transpose = args[1];
    auto data = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsFloat32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Float32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const float> slice(reinterpret_cast<float *>(data), len);

        canvas_native_webgl2_uniform_matrix3x4fv(
                locationValue.ToLocalChecked()->Value(),
                transpose->BooleanValue(isolate),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::UniformMatrix4x2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto transpose = args[1];
    auto data = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsFloat32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Float32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const float> slice(reinterpret_cast<float *>(data), len);

        canvas_native_webgl2_uniform_matrix4x2fv(
                locationValue.ToLocalChecked()->Value(),
                transpose->BooleanValue(isolate),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::UniformMatrix4x3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto location = args[0];
    auto transpose = args[1];
    auto data = args[2];
    if (args.Length() > 2 && Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation &&
        data->IsFloat32Array()) {
        auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance")->ToInt32(context);

        auto len = data.As<v8::Float32Array>()->Length();
        auto buf = data.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const float> slice(reinterpret_cast<float *>(data), len);

        canvas_native_webgl2_uniform_matrix4x3fv(
                locationValue.ToLocalChecked()->Value(),
                transpose->BooleanValue(isolate),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::VertexAttribDivisor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto index = args[0];
    auto divisor = args[1];
    if (args.Length() > 1) {
        canvas_native_webgl2_vertex_attrib_divisor(
                index->Uint32Value(context).ToChecked(),
                divisor->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::VertexAttribI4i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto index = args[0];
    auto v0 = args[1];
    auto v1 = args[2];
    auto v2 = args[3];
    auto v3 = args[4];

    if (args.Length() > 4) {
        canvas_native_webgl2_vertex_attrib_i4i(
                index->Uint32Value(context).ToChecked(),
                v0->Int32Value(context).ToChecked(),
                v1->Int32Value(context).ToChecked(),
                v2->Int32Value(context).ToChecked(),
                v3->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::VertexAttribI4iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto index = args[0];
    auto value = args[1];
    if (args.Length() > 1 && value->IsInt32Array()) {
        auto len = value.As<v8::Int32Array>()->Length();
        auto buf = value.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const int32_t> slice(reinterpret_cast<int32_t *>(data), len);
        canvas_native_webgl2_vertex_attrib_i4iv(
                index->Uint32Value(context).ToChecked(),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::VertexAttribI4ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto index = args[0];
    auto v0 = args[1];
    auto v1 = args[2];
    auto v2 = args[3];
    auto v3 = args[4];

    if (args.Length() > 4) {
        canvas_native_webgl2_vertex_attrib_i4ui(
                index->Uint32Value(context).ToChecked(),
                v0->Uint32Value(context).ToChecked(),
                v1->Uint32Value(context).ToChecked(),
                v2->Uint32Value(context).ToChecked(),
                v3->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::VertexAttribI4uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto index = args[0];
    auto value = args[1];
    if (args.Length() > 1 && value->IsUint32Array()) {
        auto len = value.As<v8::Uint32Array>()->Length();
        auto buf = value.As<v8::ArrayBufferView>();
        auto array = buf->Buffer();
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + buf->ByteOffset();
        rust::Slice<const uint32_t> slice(reinterpret_cast<uint32_t *>(data), len);
        canvas_native_webgl2_vertex_attrib_i4uiv(
                index->Uint32Value(context).ToChecked(),
                slice,
                ptr->GetState()
        );
    }
}

void WebGL2RenderingContext::SetConstants(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl) {
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
    tmpl->Set(isolate, "MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS", v8::Uint32::New(isolate, 0x8C80));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_VARYINGS", v8::Uint32::New(isolate, 0x8C83));

    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BUFFER_START", v8::Uint32::New(isolate, 0x8C84));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_BUFFER_SIZE", v8::Uint32::New(isolate, 0x8C85));
    tmpl->Set(isolate, "TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN", v8::Uint32::New(isolate, 0x8C88));
    tmpl->Set(isolate, "MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS", v8::Uint32::New(isolate, 0x8C8A));

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
    tmpl->Set(isolate, "MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS", v8::Uint32::New(isolate, 0x8A33));

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

    tmpl->Set(isolate, "UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER", v8::Uint32::New(isolate, 0x8A44));
    tmpl->Set(isolate, "UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER", v8::Uint32::New(isolate, 0x8A46));
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

    tmpl->Set(isolate, "INVALID_INDEX", v8::Uint32::New(isolate, 0xFFFFFFFF));
    tmpl->Set(isolate, "TIMEOUT_IGNORED", v8::Int32::New(isolate, -1));

    /* Vertex attributes */

    /* Transform feedback */

    tmpl->Set(isolate, "MAX_CLIENT_WAIT_TIMEOUT_WEBGL", v8::Uint32::New(isolate, 0x9247));

    /* Transform feedback */

}

void WebGL2RenderingContext::SetProps(v8::Isolate *isolate,
                                      const v8::Local<v8::ObjectTemplate> &webgl2RenderingContextTpl) {}

void WebGL2RenderingContext::SetMethods(v8::Isolate *isolate,
                                        const v8::Local<v8::ObjectTemplate> &webgl2RenderingContextTpl) {

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "beginQuery"),
            v8::FunctionTemplate::New(isolate, &BeginQuery)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "beginTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &BeginTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindBufferBase"),
            v8::FunctionTemplate::New(isolate, &BindBufferBase)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindBufferRange"),
            v8::FunctionTemplate::New(isolate, &BindBufferRange)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindSampler"),
            v8::FunctionTemplate::New(isolate, &BindSampler)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &BindTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindVertexArray"),
            v8::FunctionTemplate::New(isolate, &BindVertexArray)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "blitFramebuffer"),
            v8::FunctionTemplate::New(isolate, &BlitFramebuffer)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearBufferfi"),
            v8::FunctionTemplate::New(isolate, &ClearBufferfi)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearBufferfv"),
            v8::FunctionTemplate::New(isolate, &ClearBufferfv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearBufferiv"),
            v8::FunctionTemplate::New(isolate, &ClearBufferiv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearBufferuiv"),
            v8::FunctionTemplate::New(isolate, &ClearBufferuiv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clientWaitSync"),
            v8::FunctionTemplate::New(isolate, &ClientWaitSync)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "compressedTexSubImage3D"),
            v8::FunctionTemplate::New(isolate, &CompressedTexSubImage3D)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "copyBufferSubData"),
            v8::FunctionTemplate::New(isolate, &CopyBufferSubData)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "copyTexSubImage3D"),
            v8::FunctionTemplate::New(isolate, &CopyTexSubImage3D)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createQuery"),
            v8::FunctionTemplate::New(isolate, &CreateQuery)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createSampler"),
            v8::FunctionTemplate::New(isolate, &CreateSampler)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &CreateTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createVertexArray"),
            v8::FunctionTemplate::New(isolate, &CreateVertexArray)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteQuery"),
            v8::FunctionTemplate::New(isolate, &DeleteQuery)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteSampler"),
            v8::FunctionTemplate::New(isolate, &DeleteSampler)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteSync"),
            v8::FunctionTemplate::New(isolate, &DeleteSync)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &DeleteTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteVertexArray"),
            v8::FunctionTemplate::New(isolate, &DeleteVertexArray)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawArraysInstanced"),
            v8::FunctionTemplate::New(isolate, &DrawArraysInstanced)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawBuffers"),
            v8::FunctionTemplate::New(isolate, &DrawBuffers)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawElementsInstanced"),
            v8::FunctionTemplate::New(isolate, &DrawElementsInstanced)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawRangeElements"),
            v8::FunctionTemplate::New(isolate, &DrawRangeElements)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "endQuery"),
            v8::FunctionTemplate::New(isolate, &EndQuery)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "endTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &EndTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "fenceSync"),
            v8::FunctionTemplate::New(isolate, &FenceSync)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "framebufferTextureLayer"),
            v8::FunctionTemplate::New(isolate, &FramebufferTextureLayer)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform1ui"),
            v8::FunctionTemplate::New(isolate, &Uniform1ui)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform1uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform1uiv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform2ui"),
            v8::FunctionTemplate::New(isolate, &Uniform2ui)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform2uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform2uiv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform3ui"),
            v8::FunctionTemplate::New(isolate, &Uniform3ui)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform3uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform3uiv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform4ui"),
            v8::FunctionTemplate::New(isolate, &Uniform4ui)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform4uiv"),
            v8::FunctionTemplate::New(isolate, &Uniform4uiv)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformBlockBinding"),
            v8::FunctionTemplate::New(isolate, &UniformBlockBinding)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix2x3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix2x3fv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix2x4fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix2x4fv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix3x2fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix3x2fv)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix3x4fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix3x4fv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix4x2fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4x2fv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix4x3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4x3fv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix4x3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4x3fv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttribDivisor"),
            v8::FunctionTemplate::New(isolate, &VertexAttribDivisor)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttribI4i"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4i)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttribI4iv"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4iv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttribI4ui"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4ui)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttribI4uiv"),
            v8::FunctionTemplate::New(isolate, &VertexAttribI4uiv)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getActiveUniformBlockName"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniformBlockName)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getActiveUniformBlockParameter"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniformBlockParameter)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getActiveUniforms"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniforms)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getBufferSubData"),
            v8::FunctionTemplate::New(isolate, &GetBufferSubData)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getFragDataLocation"),
            v8::FunctionTemplate::New(isolate, &GetFragDataLocation)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getIndexedParameter"),
            v8::FunctionTemplate::New(isolate, &GetIndexedParameter)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getInternalformatParameter"),
            v8::FunctionTemplate::New(isolate, &GetInternalformatParameter)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getParameter"),
            v8::FunctionTemplate::New(isolate, &GetParameter)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getQueryParameter"),
            v8::FunctionTemplate::New(isolate, &GetQueryParameter)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getQuery"),
            v8::FunctionTemplate::New(isolate, &GetQuery)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getSamplerParameter"),
            v8::FunctionTemplate::New(isolate, &GetSamplerParameter)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getSyncParameter"),
            v8::FunctionTemplate::New(isolate, &GetSyncParameter)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getTransformFeedbackVarying"),
            v8::FunctionTemplate::New(isolate, &GetTransformFeedbackVarying)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getUniformBlockIndex"),
            v8::FunctionTemplate::New(isolate, &GetUniformBlockIndex)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getUniformIndices"),
            v8::FunctionTemplate::New(isolate, &GetUniformIndices)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "invalidateFramebuffer"),
            v8::FunctionTemplate::New(isolate, &InvalidateFramebuffer)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "invalidateSubFramebuffer"),
            v8::FunctionTemplate::New(isolate, &InvalidateSubFramebuffer)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isQuery"),
            v8::FunctionTemplate::New(isolate, &IsQuery)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isSampler"),
            v8::FunctionTemplate::New(isolate, &IsSampler)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isSync"),
            v8::FunctionTemplate::New(isolate, &IsSync)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &IsTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isVertexArray"),
            v8::FunctionTemplate::New(isolate, &IsVertexArray)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "pauseTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &PauseTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "readBuffer"),
            v8::FunctionTemplate::New(isolate, &ReadBuffer)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "renderbufferStorageMultisample"),
            v8::FunctionTemplate::New(isolate, &RenderbufferStorageMultisample)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "resumeTransformFeedback"),
            v8::FunctionTemplate::New(isolate, &ResumeTransformFeedback)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "samplerParameterf"),
            v8::FunctionTemplate::New(isolate, &SamplerParameterf)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "samplerParameteri"),
            v8::FunctionTemplate::New(isolate, &SamplerParameteri)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texImage3D"),
            v8::FunctionTemplate::New(isolate, &TexImage3D)
    );


    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texStorage2D"),
            v8::FunctionTemplate::New(isolate, &TexStorage2D)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texStorage3D"),
            v8::FunctionTemplate::New(isolate, &TexStorage3D)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texSubImage3D"),
            v8::FunctionTemplate::New(isolate, &TexSubImage3D)
    );

    webgl2RenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "transformFeedbackVaryings"),
            v8::FunctionTemplate::New(isolate, &TransformFeedbackVaryings)
    );
}

