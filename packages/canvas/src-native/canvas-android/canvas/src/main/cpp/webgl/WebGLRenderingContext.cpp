//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGLRenderingContext.h"
#include "OneByteStringResource.h"

WebGLRenderingContext::WebGLRenderingContext(rust::Box<WebGLState> state)
        : WebGLRenderingContextBase(
        std::move(state), WebGLRenderingVersion::V1) {

}

WebGLRenderingContext::WebGLRenderingContext(rust::Box<WebGLState> state,
                                             WebGLRenderingVersion version)
        : WebGLRenderingContextBase(std::move(state), version) {

}


v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLRenderingContextTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLRenderingContext"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    cache->WebGLRenderingContextTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

WebGLRenderingContext *GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WebGLRenderingContext *>(ptr);
}


v8::Local<v8::Value> WebGLRenderingContext::GetParameterInternal(v8::Isolate *isolate,
                                                                 uint32_t pnameValue,
                                                                 rust::Box<WebGLResult> result) {

    auto context = isolate->GetCurrentContext();
    v8::EscapableHandleScope scope(isolate);
    switch (pnameValue) {
        case GL_ACTIVE_TEXTURE:
        case GL_ALPHA_BITS:
        case GL_ARRAY_BUFFER_BINDING:
        case GL_BLEND_DST_ALPHA:
        case GL_BLEND_DST_RGB:
        case GL_BLEND_EQUATION:
        case GL_BLEND_EQUATION_ALPHA:
        case GL_BLEND_SRC_ALPHA:
        case GL_BLEND_SRC_RGB:
        case GL_BLUE_BITS:
        case GL_CULL_FACE_MODE:
        case GL_CURRENT_PROGRAM:
        case GL_DEPTH_BITS:
        case GL_DEPTH_FUNC:
        case GL_ELEMENT_ARRAY_BUFFER_BINDING:
        case GL_FRAMEBUFFER_BINDING:
        case GL_FRONT_FACE:
        case GL_GENERATE_MIPMAP_HINT:
        case GL_GREEN_BITS:
        case GL_IMPLEMENTATION_COLOR_READ_FORMAT:
        case GL_IMPLEMENTATION_COLOR_READ_TYPE:
        case GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS:
        case GL_MAX_CUBE_MAP_TEXTURE_SIZE:
        case GL_MAX_FRAGMENT_UNIFORM_VECTORS:
        case GL_MAX_RENDERBUFFER_SIZE:
        case GL_MAX_TEXTURE_IMAGE_UNITS:
        case GL_MAX_TEXTURE_SIZE:
        case GL_MAX_VARYING_VECTORS:
        case GL_MAX_VERTEX_ATTRIBS:
        case GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS:
        case GL_MAX_VERTEX_UNIFORM_VECTORS:
        case GL_PACK_ALIGNMENT:
        case GL_RED_BITS:
        case GL_RENDERBUFFER_BINDING:
        case GL_SAMPLE_BUFFERS:
        case GL_SAMPLES:
        case GL_STENCIL_BACK_FAIL:
        case GL_STENCIL_BACK_FUNC:
        case GL_STENCIL_BACK_PASS_DEPTH_FAIL:
        case GL_STENCIL_BACK_PASS_DEPTH_PASS:
        case GL_STENCIL_BACK_REF:
        case GL_STENCIL_BACK_VALUE_MASK:
        case GL_STENCIL_BACK_WRITEMASK:
        case GL_STENCIL_BITS:
        case GL_STENCIL_CLEAR_VALUE:
        case GL_STENCIL_FAIL:
        case GL_STENCIL_FUNC:
        case GL_STENCIL_PASS_DEPTH_FAIL:
        case GL_STENCIL_PASS_DEPTH_PASS:
        case GL_STENCIL_REF:
        case GL_STENCIL_VALUE_MASK:
        case GL_STENCIL_WRITEMASK:
        case GL_SUBPIXEL_BITS:
        case GL_TEXTURE_BINDING_2D:
        case GL_TEXTURE_BINDING_CUBE_MAP:
        case GL_UNPACK_ALIGNMENT: {
            auto value = canvas_native_webgl_result_get_i32(*result);
            if ((pnameValue == GL_CURRENT_PROGRAM || pnameValue == GL_ARRAY_BUFFER_BINDING ||
                 pnameValue == GL_ELEMENT_ARRAY_BUFFER_BINDING ||
                 pnameValue == GL_TEXTURE_BINDING_2D ||
                 pnameValue == GL_TEXTURE_BINDING_CUBE_MAP ||
                 pnameValue == GL_RENDERBUFFER_BINDING ||
                 pnameValue == GL_FRAMEBUFFER_BINDING) &&
                value == 0) {
                return scope.Escape(v8::Null(isolate));
            }
            return scope.Escape(v8::Number::New(isolate, (double) value));
        }
        case (uint32_t) GLConstants::UNPACK_COLORSPACE_CONVERSION_WEBGL:
            return scope.Escape(v8::Number::New(isolate,
                                                (double) canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
                                                        this->GetState())));
        case GL_ALIASED_LINE_WIDTH_RANGE:
        case GL_ALIASED_POINT_SIZE_RANGE:
        case GL_DEPTH_RANGE: {
            auto ret = canvas_native_webgl_result_get_f32_array(*result);
            auto buf = new VecMutableBuffer<float>(std::move(ret));


            auto store = v8::ArrayBuffer::NewBackingStore(buf->data(), buf->buffer_size(),
                                                          [](void *data, size_t length,
                                                             void *deleter_data) {
                                                              if (deleter_data != nullptr) {
                                                                  delete (VecMutableBuffer<float> *) deleter_data;
                                                              }
                                                          },
                                                          buf);

            auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));


            return scope.Escape(v8::Float32Array::New(arraybuffer, 0, buf->size()));
        }
        case GL_BLEND_COLOR:
        case GL_COLOR_CLEAR_VALUE: {
            auto ret = canvas_native_webgl_result_get_f32_array(*result);

            auto buf = new VecMutableBuffer<float>(std::move(ret));

            auto store = v8::ArrayBuffer::NewBackingStore(buf->data(), buf->buffer_size(),
                                                          [](void *data, size_t length,
                                                             void *deleter_data) {
                                                              if (deleter_data != nullptr) {
                                                                  delete (VecMutableBuffer<float> *) deleter_data;
                                                              }
                                                          },
                                                          buf);

            auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));


            return scope.Escape(v8::Float32Array::New(arraybuffer, 0, buf->size()));

        }
        case (uint32_t) GLConstants::UNPACK_FLIP_Y_WEBGL:
            return scope.Escape(v8::Boolean::New(isolate, canvas_native_webgl_state_get_flip_y(
                    this->GetState())));
        case (uint32_t) GLConstants::UNPACK_PREMULTIPLY_ALPHA_WEBGL:
            return scope.Escape(v8::Boolean::New(isolate,
                                                 canvas_native_webgl_state_get_premultiplied_alpha(
                                                         this->GetState())));
        case GL_BLEND:
        case GL_CULL_FACE:
        case GL_DEPTH_TEST:
        case GL_DEPTH_WRITEMASK:
        case GL_DITHER:
        case GL_POLYGON_OFFSET_FILL:
        case GL_SAMPLE_COVERAGE_INVERT:
        case GL_SCISSOR_TEST:
        case GL_STENCIL_TEST:
            return scope.Escape(
                    v8::Boolean::New(isolate, canvas_native_webgl_result_get_bool(*result)));
        case GL_COLOR_WRITEMASK: {
            auto ret = canvas_native_webgl_result_get_bool_array(*result);
            auto len = ret.size();
            auto array = v8::Array::New(isolate, (int) len);

            for (int j = 0; j < len; ++j) {
                array->Set(context, j, v8::Boolean::New(isolate, ret[j] == 1));
            }
            return array;
        }
        case GL_COMPRESSED_TEXTURE_FORMATS:
        case GL_MAX_VIEWPORT_DIMS:
        case GL_SCISSOR_BOX:
        case GL_VIEWPORT: {
            auto ret = canvas_native_webgl_result_get_i32_array(*result);
            auto buf = new VecMutableBuffer<int32_t>(std::move(ret));

            auto store = v8::ArrayBuffer::NewBackingStore(buf->data(), buf->buffer_size(),
                                                          [](void *data, size_t length,
                                                             void *deleter_data) {
                                                              if (deleter_data != nullptr) {
                                                                  delete (VecMutableBuffer<int32_t> *) deleter_data;
                                                              }
                                                          },
                                                          buf);

            auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));


            return scope.Escape(v8::Int32Array::New(arraybuffer, 0, buf->size()));
        }
        case GL_DEPTH_CLEAR_VALUE:
        case GL_LINE_WIDTH:
        case GL_POLYGON_OFFSET_FACTOR:
        case GL_POLYGON_OFFSET_UNITS:
        case GL_SAMPLE_COVERAGE_VALUE: {
            return scope.Escape(v8::Number::New(isolate,
                                                static_cast<double>(canvas_native_webgl_result_get_f32(
                                                        *result))));
        }
        case GL_RENDERER:
        case GL_SHADING_LANGUAGE_VERSION:
        case GL_VENDOR:
        case GL_VERSION: {
            auto ret = canvas_native_webgl_result_get_string(*result);


            auto value = new OneByteStringResource(std::move(ret));
            return scope.Escape(v8::String::NewExternalOneByte(isolate, value).ToLocalChecked());
        }
        default:
            return scope.Escape(v8::Null(isolate));

    }
}


void
WebGLRenderingContext::GetDrawingBufferWidth(v8::Local<v8::String> name,
                                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_webgl_state_get_drawing_buffer_width(ptr->GetState());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}

void
WebGLRenderingContext::GetFlipY(v8::Local<v8::String> name,
                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_webgl_state_get_flip_y(ptr->GetState());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(false);
}

void WebGLRenderingContext::GetDrawingBufferHeight(v8::Local<v8::String> name,
                                                   const v8::PropertyCallbackInfo<v8::Value> &info
) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_webgl_state_get_drawing_buffer_height(ptr->GetState());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}

void WebGLRenderingContext::__Resized(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgl_resized(
            ptr->GetState());

}

void WebGLRenderingContext::__StartRaf(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    ptr->StartRaf();

}

void WebGLRenderingContext::__StopRaf(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    ptr->StopRaf();

}

void WebGLRenderingContext::ActiveTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto texture = (uint32_t) args[0]->NumberValue(context).ToChecked();
    canvas_native_webgl_active_texture(texture,
                                       this->GetState());
}

void WebGLRenderingContext::AttachShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto shaderValue = args[1];
    if (programValue->IsObject() && shaderValue->IsObject()) {
        WebGLProgram *program = nullptr;
        WebGLShader *shader = nullptr;
        if (GetNativeType(isolate, programValue.As<v8::Object>()) == NativeType::WebGLProgram) {
            program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        }

        if (GetNativeType(isolate, shaderValue.As<v8::Object>()) == NativeType::WebGLShader) {
            shader = WebGLShader::GetPointer(shaderValue.As<v8::Object>());
        }

        if (program == nullptr) {
            return;
        }

        if (shader == nullptr) {
            return;
        }

        canvas_native_webgl_attach_shader(
                program->GetProgram(),
                shader->GetShader(),
                ptr->GetState()
        );

    }
}

void WebGLRenderingContext::BindAttribLocation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
        auto programValue = args[0];

        if (programValue->IsObject()) {
            auto type = GetNativeType(isolate, programValue.As<v8::Object>());
            if (type == NativeType::WebGLProgram && args[1]->IsNumber() &&
                args[2]->IsString()) {
                auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());

                auto index = (uint32_t) args[1]->NumberValue(context).ToChecked();
                auto name = ConvertFromV8String(isolate, args[2]);
                canvas_native_webgl_bind_attrib_location(
                        program->GetProgram(),
                        index,
                        rust::Str(name.c_str()),
                        this->GetState()
                );

            }
        }
    }
}

void WebGLRenderingContext::BindBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        if (args[0]->IsNumber()) {
            auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
            if (!args[1]->IsNull() &&
                args[1]->IsObject()) {
                auto type = GetNativeType(isolate, args[1].As<v8::Object>());
                if (type == NativeType::WebGLBuffer) {
                    auto buffer = WebGLBuffer::GetPointer(args[1].As<v8::Object>());
                    if (buffer ==
                        nullptr) { return; }
                    canvas_native_webgl_bind_buffer(
                            target,
                            buffer->GetBuffer(),
                            ptr->GetState()
                    );
                }

            } else {
                // unbind
                // check for null or undefined ?
                canvas_native_webgl_bind_buffer(
                        target,
                        0,
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::BindFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        if (args[0]->IsNumber()) {
            auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
            if (args[1]->IsObject()) {
                auto type = GetNativeType(isolate, args[1].As<v8::Object>());
                if (type == NativeType::WebGLFramebuffer) {
                    auto framebuffer = WebGLFramebuffer::GetPointer(args[1].As<v8::Object>());
                    canvas_native_webgl_bind_frame_buffer(
                            target,
                            framebuffer->GetFrameBuffer(),
                            ptr->GetState()
                    );
                }
            } else {
                // null value
                // unbind
                canvas_native_webgl_bind_frame_buffer(
                        target,
                        0,
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::BindRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        if (args[0]->IsNumber()) {
            auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
            if (args[1]->IsObject()) {
                auto type = GetNativeType(isolate, args[1].As<v8::Object>());
                if (type == NativeType::WebGLRenderbuffer) {
                    auto renderbuffer = WebGLRenderbuffer::GetPointer(args[1].As<v8::Object>());

                    if (renderbuffer ==
                        nullptr) { return; }
                    canvas_native_webgl_bind_render_buffer(
                            target,
                            renderbuffer->GetRenderBuffer(),
                            ptr->GetState()
                    );
                }
            } else {
                canvas_native_webgl_bind_render_buffer(
                        target,
                        0,
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::BindTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        if (args[0]->IsNumber()) {
            auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
            if (args[1]->IsObject()) {
                auto type = GetNativeType(isolate, args[1].As<v8::Object>());
                if (type == NativeType::WebGLTexture) {
                    auto texture = WebGLTexture::GetPointer(args[1].As<v8::Object>());
                    canvas_native_webgl_bind_texture(
                            target,
                            texture->GetTexture(),
                            ptr->GetState()
                    );
                }
            } else {
                canvas_native_webgl_bind_texture(
                        target,
                        0,
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::BlendColor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
        auto red = args[0]->NumberValue(context).ToChecked();
        auto green = args[1]->NumberValue(context).ToChecked();
        auto blue = args[2]->NumberValue(context).ToChecked();
        auto alpha = args[3]->NumberValue(context).ToChecked();

        canvas_native_webgl_blend_color(
                static_cast<float>(red),
                static_cast<float>(green),
                static_cast<float>(blue),
                static_cast<float>(alpha),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::BlendEquationSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto modeRGB = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto modeAlpha = (uint32_t) args[1]->NumberValue(context).ToChecked();

        canvas_native_webgl_blend_equation_separate(
                modeRGB,
                modeAlpha,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::BlendEquation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
        canvas_native_webgl_blend_equation(
                mode,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::BlendFuncSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
        auto srcRGB = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto dstRGB = (uint32_t) args[1]->NumberValue(context).ToChecked();
        auto srcAlpha = (uint32_t) args[2]->NumberValue(context).ToChecked();
        auto dstAlpha = (uint32_t) args[3]->NumberValue(context).ToChecked();

        canvas_native_webgl_blend_func_separate(
                srcRGB,
                dstRGB,
                srcAlpha,
                dstAlpha,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::BlendFunc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto sfactor = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto dfactor = (uint32_t) args[1]->NumberValue(context).ToChecked();

        canvas_native_webgl_blend_func(
                sfactor,
                dfactor,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::BufferData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();

    if (count == 2) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto usage = (uint32_t) args[1]->NumberValue(context).ToChecked();

        canvas_native_webgl_buffer_data_none(
                target,
                0,
                usage,
                ptr->GetState()
        );
    } else if (count == 3) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();

        auto usage = (uint32_t) args[2]->NumberValue(context).ToChecked();

        if (args[1]->IsObject()) {

            auto sizeOrBuf = args[1];
            if (sizeOrBuf->IsArrayBufferView()) {
                if (sizeOrBuf->IsUint16Array()) {
                    auto array = sizeOrBuf.As<v8::TypedArray>();
                    auto buf = GetTypedArrayData<const uint16_t>(array);

                    canvas_native_webgl_buffer_data_u16(
                            target,
                            buf,
                            usage,
                            ptr->GetState()
                    );
                } else if (sizeOrBuf->IsFloat32Array()) {
                    auto array = sizeOrBuf.As<v8::TypedArray>();
                    auto buf = GetTypedArrayData<const float>(array);
                    canvas_native_webgl_buffer_data_f32(
                            target,
                            buf,
                            usage,
                            ptr->GetState()
                    );
                } else {
                    auto array = sizeOrBuf.As<v8::TypedArray>();
                    auto buf = GetTypedArrayData<const uint8_t>(array);
                    canvas_native_webgl_buffer_data(
                            target,
                            buf,
                            usage,
                            ptr->GetState()
                    );
                }
            } else if (sizeOrBuf->IsArrayBuffer()) {
                auto buffer = sizeOrBuf.As<v8::ArrayBuffer>();
                auto data = GetArrayBufferData<const uint8_t>(buffer);
                canvas_native_webgl_buffer_data(
                        target,
                        data,
                        usage,
                        ptr->GetState()
                );
            }
        } else {
            auto sizeOrBuf = args[1]->NumberValue(context).ToChecked();
            canvas_native_webgl_buffer_data_none(
                    target,
                    static_cast<ssize_t>(sizeOrBuf),
                    usage,
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::BufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();

    if (count == 2) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto offset = args[1]->NumberValue(context).ToChecked();

        canvas_native_webgl_buffer_sub_data_none(
                target,
                static_cast<ssize_t>(offset),
                ptr->GetState()
        );
    } else if (count == 3) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto offset = args[1]->NumberValue(context).ToChecked();

        if (args[2]->IsObject()) {
            auto buf = args[2];

            if (buf->IsTypedArray()) {
                auto array = buf.As<v8::TypedArray>();
                auto buff = GetTypedArrayData<const uint8_t>(array);
                canvas_native_webgl_buffer_sub_data(
                        target,
                        static_cast<ssize_t>(offset),
                        buff,
                        ptr->GetState()
                );
            } else if (buf->IsArrayBuffer()) {
                auto arrayBuffer = buf.As<v8::ArrayBuffer>();

                auto data = GetArrayBufferData<const uint8_t>(arrayBuffer);

                canvas_native_webgl_buffer_sub_data(
                        target,
                        static_cast<rust::isize>(offset),
                        data,
                        ptr->GetState()
                );
            }

        }

    }
}

void
WebGLRenderingContext::CheckFramebufferStatus(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();

    if (count > 0) {

        if (args[0]->IsNumber()) {
            auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();

            auto ret = canvas_native_webgl_check_frame_buffer_status(
                    target,
                    ptr->GetState()
            );

            args.GetReturnValue().Set((int32_t) ret);
            return;
        }
    }

    args.GetReturnValue().Set(0);
}

void WebGLRenderingContext::ClearColor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
        auto red = args[0]->NumberValue(context).ToChecked();
        auto green = args[1]->NumberValue(context).ToChecked();
        auto blue = args[2]->NumberValue(context).ToChecked();
        auto alpha = args[3]->NumberValue(context).ToChecked();

        canvas_native_webgl_clear_color(
                static_cast<float>(red),
                static_cast<float>(green),
                static_cast<float>(blue),
                static_cast<float>(alpha),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::ClearDepth(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto depth = args[0]->NumberValue(context).ToChecked();

        canvas_native_webgl_clear_depth(
                static_cast<float>(depth),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::ClearStencil(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto stencil = (int32_t) args[0]->NumberValue(context).ToChecked();
        canvas_native_webgl_clear_stencil(
                stencil,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::Clear(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    //if (count > 0) {
    auto mask = (uint32_t) args[0]->NumberValue(context).ToChecked();

    canvas_native_webgl_clear(
            mask,
            ptr->GetState()
    );

    this->UpdateInvalidateState();
    // }
}

void WebGLRenderingContext::ColorMask(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    if (args.Length() > 3) {
        auto red = args[0]->BooleanValue(isolate);
        auto green = args[1]->BooleanValue(isolate);
        auto blue = args[2]->BooleanValue(isolate);
        auto alpha = args[3]->BooleanValue(isolate);

        canvas_native_webgl_color_mask(
                red,
                green,
                blue,
                alpha,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::Commit(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
}

void WebGLRenderingContext::CompileShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto shader = WebGLShader::GetPointer(args[0].As<v8::Object>());
        if (shader != nullptr) {
            canvas_native_webgl_compile_shader(
                    shader->GetShader(),
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::CompressedTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();

    if (count == 6) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto internalformat = (uint32_t) args[2]->NumberValue(context).ToChecked();
        auto width = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto border = (int32_t) args[5]->NumberValue(context).ToChecked();

        canvas_native_webgl_compressed_tex_image2d_none(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                ptr->GetState()
        );
    } else if (count > 6) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto internalformat = (uint32_t) args[2]->NumberValue(context).ToChecked();
        auto width = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto border = (int32_t) args[5]->NumberValue(context).ToChecked();
        auto pixels = args[6];
        if (pixels->IsObject()) {
            if (pixels->IsTypedArray()) {
                auto array = pixels.As<v8::TypedArray>();
                auto buf = GetTypedArrayData<const uint8_t>(array);
                canvas_native_webgl_compressed_tex_image2d(
                        target,
                        level,
                        internalformat,
                        width,
                        height,
                        border,
                        buf,
                        ptr->GetState()
                );
            } else if (pixels->IsArrayBuffer()) {
                auto ab = pixels.As<v8::ArrayBuffer>();
                auto buf = GetArrayBufferData<const uint8_t>(ab);

                canvas_native_webgl_compressed_tex_image2d(
                        target,
                        level,
                        internalformat,
                        width,
                        height,
                        border,
                        buf,
                        ptr->GetState()
                );
            }
        }
    }
}

void
WebGLRenderingContext::CompressedTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();


    if (count > 7) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto xoffset = (int32_t) args[2]->NumberValue(context).ToChecked();
        auto yoffset = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto width = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[5]->NumberValue(context).ToChecked();
        auto format = (uint32_t) args[6]->NumberValue(context).ToChecked();
        if (args[7]->IsObject()) {
            auto pixels = args[7];
            if (pixels->IsTypedArray()) {
                auto px = pixels.As<v8::TypedArray>();
                auto buf = GetTypedArrayData<const uint8_t>(px);
                canvas_native_webgl_compressed_tex_sub_image2d(
                        target,
                        level,
                        xoffset,
                        yoffset,
                        width,
                        height,
                        format,
                        buf,
                        ptr->GetState()
                );
            } else if (pixels->IsArrayBuffer()) {
                auto buffer = pixels.As<v8::ArrayBuffer>();
                auto buf = GetArrayBufferData<const uint8_t>(buffer);

                canvas_native_webgl_compressed_tex_sub_image2d(
                        target,
                        level,
                        xoffset,
                        yoffset,
                        width,
                        height,
                        format,
                        buf,
                        ptr->GetState()
                );
            }
        }
    }

}

void WebGLRenderingContext::CopyTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();


    if (count > 7) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto internalformat = (uint32_t) args[2]->NumberValue(context).ToChecked();
        auto x = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto y = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto width = (int32_t) args[5]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[6]->NumberValue(context).ToChecked();
        auto border = (int32_t) args[7]->NumberValue(context).ToChecked();

        canvas_native_webgl_copy_tex_image2d(
                target,
                level,
                internalformat,
                x,
                y,
                width,
                height,
                border,
                ptr->GetState()
        );
    }

}

void WebGLRenderingContext::CopyTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();


    if (count > 7) {
        auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto level = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto xoffset = (int32_t) args[2]->NumberValue(context).ToChecked();
        auto yoffset = (int32_t) args[3]->NumberValue(context).ToChecked();
        auto x = (int32_t) args[4]->NumberValue(context).ToChecked();
        auto y = (int32_t) args[5]->NumberValue(context).ToChecked();
        auto width = (int32_t) args[6]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[7]->NumberValue(context).ToChecked();

        canvas_native_webgl_copy_tex_sub_image2d(
                target,
                level,
                xoffset,
                yoffset,
                x,
                y,
                width,
                height,
                ptr->GetState()
        );
    }

}

void WebGLRenderingContext::CreateBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto buffer = canvas_native_webgl_create_buffer(
            ptr->GetState());
    if (buffer != 0) {
        auto ret = WebGLBuffer::NewInstance(isolate, new WebGLBuffer(
                buffer));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetNull();

}

void WebGLRenderingContext::CreateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto buffer = canvas_native_webgl_create_framebuffer(
            ptr->GetState());
    if (buffer != 0) {
        auto ret = WebGLFramebuffer::NewInstance(isolate, new WebGLFramebuffer(
                buffer));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetNull();

}

void WebGLRenderingContext::CreateProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto program = canvas_native_webgl_create_program(
            ptr->GetState());
    if (program != 0) {
        auto ret = WebGLProgram::NewInstance(isolate, new WebGLProgram(
                program));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetNull();

}

void WebGLRenderingContext::CreateRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto buffer = canvas_native_webgl_create_renderbuffer(
            ptr->GetState());
    if (buffer != 0) {
        auto ret = WebGLRenderbuffer::NewInstance(isolate, new WebGLRenderbuffer(
                buffer));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetNull();

}

void WebGLRenderingContext::CreateShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() == 0) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto type = (uint32_t) args[0]->NumberValue(context).ToChecked();
    auto shader = canvas_native_webgl_create_shader(
            type, ptr->GetState());
    if (shader != 0) {
        auto ret = WebGLShader::NewInstance(isolate, new WebGLShader(
                shader));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();

}

void WebGLRenderingContext::CreateTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto texture = canvas_native_webgl_create_texture(
            ptr->GetState());
    if (texture != 0) {
        auto ret = WebGLTexture::NewInstance(isolate, new WebGLTexture(
                texture));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();

}

void WebGLRenderingContext::CullFace(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 0) {
        auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();

        canvas_native_webgl_cull_face(
                mode,
                ptr->GetState()
        );
    }

}

void WebGLRenderingContext::DeleteBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 0) {
        auto value = args[0];
        if (value->IsObject()) {
            auto type = GetNativeType(isolate, value.As<v8::Object>());
            if (type == NativeType::WebGLBuffer) {
                auto buffer = WebGLBuffer::GetPointer(value.As<v8::Object>());
                if (buffer != nullptr) {
                    canvas_native_webgl_delete_buffer(
                            buffer->GetBuffer(),
                            ptr->GetState()
                    );
                }
            }
        }
    }

}

void WebGLRenderingContext::DeleteFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLFramebuffer) {
        auto buffer = WebGLFramebuffer::GetPointer(value.As<v8::Object>());
        if (buffer != nullptr) {
            canvas_native_webgl_delete_framebuffer(
                    buffer->GetFrameBuffer(),
                    this->GetState()
            );
        }
    }

}

void WebGLRenderingContext::DeleteProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(value.As<v8::Object>());
        if (program != nullptr) {
            canvas_native_webgl_delete_framebuffer(
                    program->GetProgram(),
                    ptr->GetState()
            );
        }
    }


}

void WebGLRenderingContext::DeleteRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::WebGLProgram) {
        auto buffer = WebGLRenderbuffer::GetPointer(value.As<v8::Object>());
        if (buffer != nullptr) {
            canvas_native_webgl_delete_renderbuffer(
                    buffer->GetRenderBuffer(),
                    ptr->GetState()
            );
        }
    }


}

void WebGLRenderingContext::DeleteShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::WebGLProgram) {
        auto shader = WebGLShader::GetPointer(value.As<v8::Object>());
        if (shader != nullptr) {
            canvas_native_webgl_delete_shader(
                    shader->GetShader(),
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::DeleteTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::WebGLProgram) {
        auto texture = WebGLTexture::GetPointer(value.As<v8::Object>());
        if (texture != nullptr) {
            canvas_native_webgl_delete_texture(
                    texture->GetTexture(),
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::DepthFunc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto value = args[0];

    if (args.Length() > 0) {
        auto func = (uint32_t) value->NumberValue(context).ToChecked();

        canvas_native_webgl_depth_func(
                func,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DepthMask(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();


    auto value = args[0];

    if (args.Length() > 0) {
        auto mask = args[0]->BooleanValue(isolate);

        canvas_native_webgl_depth_mask(
                mask,
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DepthRange(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto zNear = args[0];
    auto zFar = args[1];

    if (args.Length() > 1) {

        canvas_native_webgl_depth_range(
                static_cast<float>(zNear->NumberValue(context).ToChecked()),
                static_cast<float>(zFar->NumberValue(context).ToChecked()),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DetachShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto programValue = args[0];
    auto shaderValue = args[1];

    if (programValue->IsObject() &&
        shaderValue->IsObject()) {
        auto programType = GetNativeType(isolate, programValue);
        auto shaderType = GetNativeType(isolate, shaderValue);
        WebGLProgram *program = nullptr;
        WebGLShader *shader = nullptr;
        if (programType == NativeType::WebGLProgram) {
            program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        }
        if (shaderType == NativeType::WebGLShader) {
            shader = WebGLShader::GetPointer(shaderValue.As<v8::Object>());
        }

        if (program != nullptr &&
            shader != nullptr) {
            canvas_native_webgl_detach_shader(
                    program->GetProgram(),
                    shader->GetShader(),
                    ptr->GetState()
            );
        }
    }
}

void
WebGLRenderingContext::DisableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto index = (uint32_t) args[0]->NumberValue(context).ToChecked();

    canvas_native_webgl_disable_vertex_attrib_array(
            index,
            ptr->GetState()
    );
}

void WebGLRenderingContext::Disable(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto cap = (uint32_t) args[0]->NumberValue(context).ToChecked();

    canvas_native_webgl_disable(
            cap,
            ptr->GetState()
    );
}

void WebGLRenderingContext::DrawArrays(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
    auto first = (int32_t) args[1]->NumberValue(context).ToChecked();
    auto count = (int32_t) args[2]->NumberValue(context).ToChecked();

    canvas_native_webgl_draw_arrays(
            mode,
            first,
            count,
            ptr->GetState()
    );
    this->UpdateInvalidateState();
}

void WebGLRenderingContext::DrawElements(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
    auto count = (int32_t) args[1]->NumberValue(context).ToChecked();
    auto type = (uint32_t) args[2]->NumberValue(context).ToChecked();
    auto offset = args[3]->NumberValue(context).ToChecked();

    canvas_native_webgl_draw_elements(
            mode,
            count,
            type,
            static_cast<ssize_t>(offset),
            this->GetState()
    );
    this->UpdateInvalidateState();
}

void
WebGLRenderingContext::EnableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto index = (uint32_t) args[0]->NumberValue(context).ToChecked();

    canvas_native_webgl_enable_vertex_attrib_array(
            index,
            ptr->GetState()
    );
}

void WebGLRenderingContext::Enable(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto cap = (uint32_t) args[0]->NumberValue(context).ToChecked();

    canvas_native_webgl_enable(
            cap,
            ptr->GetState()
    );
}

void WebGLRenderingContext::Finish(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgl_finish(
            ptr->GetState()
    );
}

void WebGLRenderingContext::Flush(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgl_flush(
            ptr->GetState()
    );
}

void
WebGLRenderingContext::FramebufferRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
    auto attachment = (uint32_t) args[1]->NumberValue(context).ToChecked();
    auto renderbuffertarget = (uint32_t) args[2]->NumberValue(context).ToChecked();
    auto renderbufferValue = args[3];
    auto type = GetNativeType(isolate, renderbufferValue);
    if (type == NativeType::WebGLRenderbuffer) {
        auto renderbuffer = WebGLRenderbuffer::GetPointer(renderbufferValue.As<v8::Object>());
        if (renderbuffer != nullptr) {
            canvas_native_webgl_framebuffer_renderbuffer(
                    target,
                    attachment,
                    renderbuffertarget,
                    renderbuffer->GetRenderBuffer(),
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::FramebufferTexture2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();
    auto attachment = (uint32_t) args[1]->NumberValue(context).ToChecked();
    auto textarget = (uint32_t) args[2]->NumberValue(context).ToChecked();
    auto level = (int32_t) args[4]->NumberValue(context).ToChecked();
    auto textureValue = args[3];
    auto type = GetNativeType(isolate, textureValue);
    if (type == NativeType::WebGLTexture) {
        auto texture = WebGLTexture::GetPointer(textureValue.As<v8::Object>());
        if (texture != nullptr) {
            canvas_native_webgl_framebuffer_texture2d(
                    target,
                    attachment,
                    textarget,
                    texture->GetTexture(),
                    level,
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::FrontFace(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();

    canvas_native_webgl_front_face(
            mode,
            ptr->GetState()
    );
}

void WebGLRenderingContext::GenerateMipmap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto target = (uint32_t) args[0]->NumberValue(context).ToChecked();

    canvas_native_webgl_generate_mipmap(
            target,
            this->GetState()
    );
}

void WebGLRenderingContext::GetActiveAttrib(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);
    if (type == NativeType::WebGLProgram) {
        auto index = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        if (program != nullptr) {
            auto info = canvas_native_webgl_get_active_attrib(
                    program->GetProgram(),
                    index,
                    ptr->GetState()
            );
            auto ret = WebGLActiveInfoImpl::NewInstance(isolate, new WebGLActiveInfoImpl(
                    std::move(info)));
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    args.GetReturnValue().SetUndefined();

}

void WebGLRenderingContext::GetActiveUniform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);
    if (type == NativeType::WebGLProgram) {
        auto index = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        if (program != nullptr) {
            auto info = canvas_native_webgl_get_active_uniform(
                    program->GetProgram(),
                    index,
                    this->GetState()
            );
            auto ret = WebGLActiveInfoImpl::NewInstance(isolate, new WebGLActiveInfoImpl(
                    std::move(info)));
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    args.GetReturnValue().SetUndefined();

}

void WebGLRenderingContext::GetAttachedShaders(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);
    if (type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        if (program != nullptr) {
            auto info = canvas_native_webgl_get_attached_shaders(
                    program->GetProgram(),
                    ptr->GetState()
            );
            auto len = info.size();
            auto array = v8::Array::New(
                    isolate, (int) len);
            for (int i = 0; i < len; ++i) {
                auto shader = WebGLShader::NewInstance(isolate, new WebGLShader(
                        info[i]));
                array->Set(context, i, shader);
            }
            args.GetReturnValue().Set(array);
            return;
        }
    }


    args.GetReturnValue().Set(v8::Array::New(isolate));

}

void WebGLRenderingContext::GetAttribLocation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);

    if (type == NativeType::WebGLProgram) {
        auto name = ConvertFromV8String(isolate, args[1]);
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        if (program != nullptr) {
            auto location = canvas_native_webgl_get_attrib_location(
                    program->GetProgram(),
                    rust::Str(
                            name.c_str()),
                    ptr->GetState()
            );
            args.GetReturnValue().Set(location);
            return;
        }
    }

    args.GetReturnValue().Set(-1);

}

void WebGLRenderingContext::GetBufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();

        auto param = canvas_native_webgl_get_buffer_parameter(
                target,
                pname,
                ptr->GetState()
        );

        args.GetReturnValue().Set(param);
        return;
    }

    args.GetReturnValue().SetUndefined();

}

void WebGLRenderingContext::GetBufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();

        auto param = canvas_native_webgl_get_buffer_parameter(
                target,
                pname,
                ptr->GetState()
        );

        args.GetReturnValue().Set(param);
        return;
    }

    args.GetReturnValue().SetUndefined();

}

void WebGLRenderingContext::GetContextAttributes(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto attr = canvas_native_webgl_get_context_attributes(
            this->GetState());

    auto ret = v8::Object::New(isolate);

    auto alpha = canvas_native_webgl_context_attribute_get_get_alpha(
            *attr);

    ret->Set(context, ConvertToV8String(isolate, "alpha"), v8::Boolean::New(isolate, alpha));

    auto antialias = canvas_native_webgl_context_attribute_get_get_antialias(
            *attr);

    ret->Set(context, ConvertToV8String(isolate, "antialias"),
             v8::Boolean::New(isolate, antialias));

    auto depth = canvas_native_webgl_context_attribute_get_get_depth(
            *attr);

    ret->Set(context, ConvertToV8String(isolate, "depth"), v8::Boolean::New(isolate, depth));

    auto fail_if_major_performance_caveat = canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
            *attr);

    ret->Set(context,
             ConvertToV8String(isolate, "failIfMajorPerformanceCaveat"),
             v8::Boolean::New(isolate, fail_if_major_performance_caveat));

    auto power_preference = canvas_native_webgl_context_attribute_get_get_power_preference(
            *attr);

    ret->Set(context, ConvertToV8String(isolate, "powerPreference"),
             ConvertToV8OneByteString(isolate, std::move(power_preference)));

    auto premultiplied_alpha = canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
            *attr);

    ret->Set(context,
             ConvertToV8String(isolate, "premultipliedAlpha"),
             v8::Boolean::New(isolate, premultiplied_alpha));

    auto preserve_drawing_buffer = canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
            *attr);

    ret->Set(context,
             ConvertToV8String(isolate, "preserveDrawingBuffer"),
             v8::Boolean::New(isolate, preserve_drawing_buffer));

    auto stencil = canvas_native_webgl_context_attribute_get_get_stencil(
            *attr);

    ret->Set(context, ConvertToV8String(isolate, "stencil"),
             v8::Boolean::New(isolate, stencil));

    auto desynchronized = canvas_native_webgl_context_attribute_get_get_desynchronized(
            *attr);

    ret->Set(context, ConvertToV8String(isolate, "desynchronized"),
             v8::Boolean::New(isolate, desynchronized));

    auto xr_compatible = canvas_native_webgl_context_attribute_get_get_xr_compatible(
            *attr);

    ret->Set(context, ConvertToV8String(isolate, "xrCompatible"),
             v8::Boolean::New(isolate, xr_compatible));

    args.GetReturnValue().Set(ret);

}

void WebGLRenderingContext::GetError(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto ret = canvas_native_webgl_get_error(
            this->GetState());

    args.GetReturnValue().Set((int32_t) ret);

}

void WebGLRenderingContext::GetExtension(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto nameValue = args[0];

    if (!nameValue->IsString()) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto name = ConvertFromV8String(isolate, nameValue);

    auto ext = canvas_native_webgl_get_extension(
            rust::Str(name.c_str()),
            ptr->GetState());

    if (canvas_native_webgl_context_extension_is_none(
            *ext)) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto type = canvas_native_webgl_context_extension_get_type(
            *ext);
    switch (type) {
        case WebGLExtensionType::OES_fbo_render_mipmap: {
            auto ret = OES_fbo_render_mipmapImpl::NewInstance(isolate,
                                                              new OES_fbo_render_mipmapImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::EXT_blend_minmax: {
            auto ret = EXT_blend_minmaxImpl::NewInstance(isolate, new EXT_blend_minmaxImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::EXT_color_buffer_half_float: {
            auto ret = EXT_color_buffer_half_floatImpl::NewInstance(isolate,
                                                                    new EXT_color_buffer_half_floatImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::EXT_disjoint_timer_query: {
            auto ret = canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
                    std::move(ext));
            auto query = EXT_disjoint_timer_queryImpl::NewInstance(isolate,
                                                                   new EXT_disjoint_timer_queryImpl(
                                                                           std::move(ret)));

            args.GetReturnValue().Set(query);
            return;
        }
        case WebGLExtensionType::EXT_sRGB: {
            auto ret = EXT_sRGBImpl::NewInstance(isolate, new EXT_sRGBImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::EXT_shader_texture_lod: {
            auto ret = EXT_shader_texture_lodImpl::NewInstance(isolate,
                                                               new EXT_shader_texture_lodImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::EXT_texture_filter_anisotropic: {
            auto ret = EXT_texture_filter_anisotropicImpl::NewInstance(isolate,
                                                                       new EXT_texture_filter_anisotropicImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::OES_element_index_uint: {
            auto ret = OES_element_index_uintImpl::NewInstance(isolate,
                                                               new OES_element_index_uintImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::OES_standard_derivatives: {
            auto ret = OES_standard_derivativesImpl::NewInstance(
                    new OES_standard_derivativesImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::OES_texture_float: {
            auto ret = OES_texture_floatImpl::NewInstance(isolate, new OES_texture_floatImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::OES_texture_float_linear: {
            auto ret = OES_texture_float_linearImpl::NewInstance(isolate,
                                                                 new OES_texture_float_linearImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::OES_texture_half_float: {
            auto ret = OES_texture_half_floatImpl::NewInstance(isolate,
                                                               new OES_texture_half_floatImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::OES_texture_half_float_linear: {
            auto ret = OES_texture_half_float_linearImpl::NewInstance(isolate,
                                                                      new OES_texture_half_float_linearImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::OES_vertex_array_object: {
            auto ret = canvas_native_webgl_context_extension_to_oes_vertex_array_object(
                    std::move(ext));
            auto array = OES_vertex_array_objectImpl::NewInstance(isolate,
                                                                  new OES_vertex_array_objectImpl(
                                                                          std::move(ret)));
            args.GetReturnValue().Set(array);
            return;
        }
        case WebGLExtensionType::WEBGL_color_buffer_float: {
            auto ret = WEBGL_color_buffer_floatImpl::NewInstance(isolate,
                                                                 new WEBGL_color_buffer_floatImpl());
            if (this->GetVersion() ==
                WebGLRenderingVersion::V2) {
                ret->Set(context, ConvertToV8String(isolate, "ext_name"),
                         ConvertToV8String(isolate, "EXT_color_buffer_float"));
            }
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_compressed_texture_atc: {
            auto ret = WEBGL_compressed_texture_atcImpl::NewInstance(isolate,
                                                                     new WEBGL_compressed_texture_atcImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_compressed_texture_etc1: {
            auto ret = WEBGL_compressed_texture_etc1Impl::NewInstance(isolate,
                                                                      new WEBGL_compressed_texture_etc1Impl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_compressed_texture_s3tc: {
            auto ret = WEBGL_compressed_texture_s3tcImpl::NewInstance(isolate,
                                                                      new WEBGL_compressed_texture_s3tcImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb: {
            auto ret = WEBGL_compressed_texture_s3tc_srgbImpl::NewInstance(isolate,
                                                                           new WEBGL_compressed_texture_s3tc_srgbImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_compressed_texture_etc: {
            auto ret = WEBGL_compressed_texture_etcImpl::NewInstance(isolate,
                                                                     new WEBGL_compressed_texture_etcImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_compressed_texture_pvrtc: {
            auto ret = WEBGL_compressed_texture_pvrtcImpl::NewInstance(isolate,
                                                                       new WEBGL_compressed_texture_pvrtcImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_lose_context: {
            auto ret = canvas_native_webgl_context_extension_to_lose_context(
                    std::move(ext));
            auto ctx = WEBGL_lose_contextImpl::NewInstance(isolate, new WEBGL_lose_contextImpl(
                    std::move(ret)));

            args.GetReturnValue().Set(ctx);
            return;
        }
        case WebGLExtensionType::ANGLE_instanced_arrays: {
            auto ret = canvas_native_webgl_context_extension_to_angle_instanced_arrays(
                    std::move(ext));
            auto instance = ANGLE_instanced_arraysImpl::NewInstance(isolate,
                                                                    new ANGLE_instanced_arraysImpl(
                                                                            std::move(ret)));

            args.GetReturnValue().Set(instance);
            return;
        }
        case WebGLExtensionType::WEBGL_depth_texture: {
            auto ret = WEBGL_depth_textureImpl::NewInstance(isolate, new WEBGL_depth_textureImpl());
            args.GetReturnValue().Set(ret);
            return;
        }
        case WebGLExtensionType::WEBGL_draw_buffers: {
            auto ret = canvas_native_webgl_context_extension_to_draw_buffers(
                    std::move(ext));

            auto buffers = WEBGL_draw_buffersImpl::NewInstance(isolate, new WEBGL_draw_buffersImpl(
                    std::move(ret)));

            args.GetReturnValue().Set(buffers);
            return;


        }
        case WebGLExtensionType::None:
            args.GetReturnValue().SetUndefined();
            return;
    }

    args.GetReturnValue().SetUndefined();

}

void WebGLRenderingContext::GetFramebufferAttachmentParameter(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto target = (uint32_t) args[0]->NumberValue(
            context).ToChecked();
    auto attachment = (uint32_t) args[1]->NumberValue(
            context).ToChecked();
    auto pname = (uint32_t) args[2]->NumberValue(
            context).ToChecked();
    auto ret = canvas_native_webgl_get_framebuffer_attachment_parameter(
            target,
            attachment,
            pname,
            ptr->GetState()
    );
    if (canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(
            *ret)) {
        auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(
                *ret);
        auto texture = WebGLTexture::NewInstance(isolate, new WebGLTexture(
                value));
        args.GetReturnValue().Set(texture);
        return;
    }
    if (canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
            *ret)) {
        auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(
                *ret);
        auto object = WebGLRenderbuffer::NewInstance(isolate, new WebGLRenderbuffer(
                value));


        object->Set(context,
                    ConvertToV8String(isolate, "isRenderbuffer"),
                    v8::Boolean::New(isolate,
                                     true));
        args.GetReturnValue().Set(object);
        return;

    }

    args.GetReturnValue().Set(canvas_native_webgl_framebuffer_attachment_parameter_get_value(
            *ret));

}

void WebGLRenderingContext::GetParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    // TODO remove extra allocations
    if (args.Length() > 0) {
        auto pname = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto result = canvas_native_webgl_get_parameter(
                pname,
                ptr->GetState());

        auto ret = GetParameterInternal(isolate,
                                        pname,
                                        std::move(
                                                result));

        args.GetReturnValue().Set(ret);
        return;
    }
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetProgramInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto programValue = args[0];
        auto type = GetNativeType(isolate, programValue);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
            if (program != nullptr) {
                auto log = canvas_native_webgl_get_program_info_log(
                        program->GetProgram(),
                        ptr->GetState()
                );


                if (log.empty()) {
                    args.GetReturnValue().SetEmptyString();
                    return;
                }

                args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, std::move(log)));
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetEmptyString();
}

void WebGLRenderingContext::GetProgramParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto programValue = args[0];
        auto type = GetNativeType(isolate, programValue);
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
            if (program != nullptr) {
                auto ret = canvas_native_webgl_get_program_parameter(
                        program->GetProgram(),
                        pname,
                        ptr->GetState()
                );

                if (canvas_native_webgl_result_get_is_none(
                        *ret)) {
                    args.GetReturnValue().SetNull();
                    return;
                }
                switch (pname) {
                    case GL_DELETE_STATUS:
                    case GL_LINK_STATUS:
                    case GL_VALIDATE_STATUS:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                                *ret));
                        return;
                    default:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                                *ret));
                        return;
                }
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetRenderbufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        auto ret = canvas_native_webgl_get_renderbuffer_parameter(
                target,
                pname,
                ptr->GetState()
        );
        args.GetReturnValue().Set(ret);
        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetRenderbufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto target = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        auto ret = canvas_native_webgl_get_renderbuffer_parameter(
                target,
                pname,
                ptr->GetState()
        );
        args.GetReturnValue().Set(ret);
        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetShaderInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto value = args[0];
        auto type = GetNativeType(isolate, value);
        if (type == NativeType::WebGLShader) {
            auto shader = WebGLShader::GetPointer(value.As<v8::Object>());
            if (shader != nullptr) {
                auto log = canvas_native_webgl_get_shader_info_log(
                        shader->GetShader(),
                        ptr->GetState()
                );

                 args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, std::move(log)));

                 return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetEmptyString();
}


std::vector<jsi::PropNameID> WebGLRenderingContext::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(436);

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

    return ret;
}

jsi::Value WebGLRenderingContext::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    auto prop_return = GetProperty(methodName);

    if (!prop_return.isUndefined()) {
        return prop_return;
    }


    if (methodName == "getShaderParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto pname = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             if (args[0].isObject()) {
                                                                 auto shader = args[0].asObject(
                                                                         runtime).asHostObject<WebGLShader>(
                                                                         runtime);
                                                                 if (shader != nullptr) {
                                                                     auto ret = canvas_native_webgl_get_shader_parameter(
                                                                             shader->GetShader(),
                                                                             pname,
                                                                             this->GetState()
                                                                     );

                                                                     if (canvas_native_webgl_result_get_is_none(
                                                                             *ret)) {
                                                                         return jsi::Value::null();
                                                                     }

                                                                     if (pname ==
                                                                         GL_DELETE_STATUS ||
                                                                         pname ==
                                                                         GL_COMPILE_STATUS) {
                                                                         return {canvas_native_webgl_result_get_bool(
                                                                                 *ret)};
                                                                     }

                                                                     return {canvas_native_webgl_result_get_i32(
                                                                             *ret)};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getShaderPrecisionFormat") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto shaderType = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto precisionType = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto ret = canvas_native_webgl_get_shader_precision_format(
                                                                     shaderType,
                                                                     precisionType,
                                                                     this->GetState()
                                                             );
                                                             auto shader = std::make_shared<WebGLShaderPrecisionFormatImpl>(
                                                                     std::move(ret));
                                                             return jsi::Object::createFromHostObject(
                                                                     runtime, shader);
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getShaderSource") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto shader = args[0].asObject(
                                                                         runtime).asHostObject<WebGLShader>(
                                                                         runtime);

                                                                 if (shader != nullptr) {
                                                                     auto source = canvas_native_webgl_get_shader_source(
                                                                             shader->GetShader(),
                                                                             this->GetState()
                                                                     );

                                                                     return jsi::String::createFromAscii(
                                                                             runtime, source.data(),
                                                                             source.size());
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::String::createFromAscii(
                                                                 runtime, "");
                                                     }
        );
    } else if (methodName == "getSupportedExtensions") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto exts = canvas_native_webgl_get_supported_extensions(
                                                                 this->GetState());
                                                         auto len = exts.size();
                                                         auto array = jsi::Array(runtime, len);
                                                         for (int i = 0; i < len; ++i) {
                                                             auto item = exts[i];
                                                             array.setValueAtIndex(runtime, i,
                                                                                   jsi::String::createFromAscii(
                                                                                           runtime,
                                                                                           item.data(),
                                                                                           item.size()));
                                                         }
                                                         return array;
                                                     }
        );
    } else if (methodName == "__getSupportedExtensions") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto exts = canvas_native_webgl_get_supported_extensions_to_string(
                                                                 this->GetState());

                                                         return jsi::String::createFromAscii(
                                                                 runtime, exts.data(), exts.size());
                                                     }
        );
    } else if (methodName == "getTexParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto target = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto pname = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto ret = canvas_native_webgl_get_tex_parameter(
                                                                     target,
                                                                     pname,
                                                                     this->GetState()
                                                             );
                                                             return {ret};
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getUniformLocation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             if (args[0].isObject() &&
                                                                 args[1].isString()) {
                                                                 auto program = args[0].asObject(
                                                                         runtime).asHostObject<WebGLProgram>(
                                                                         runtime);
                                                                 if (program != nullptr) {
                                                                     auto name = args[1].asString(
                                                                             runtime).utf8(runtime);

                                                                     auto ret = canvas_native_webgl_get_uniform_location(
                                                                             program->GetProgram(),
                                                                             rust::Str(
                                                                                     name.c_str()),
                                                                             this->GetState()
                                                                     );

                                                                     auto location = std::make_shared<WebGLUniformLocation>(
                                                                             ret);

                                                                     return jsi::Object::createFromHostObject(
                                                                             runtime, location);
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getUniform") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             if (args[0].isObject() &&
                                                                 args[1].isObject()) {

                                                                 auto program = args[0].asObject(
                                                                         runtime).asHostObject<WebGLProgram>(
                                                                         runtime);
                                                                 auto location = args[1].asObject(
                                                                         runtime).asHostObject<WebGLUniformLocation>(
                                                                         runtime);


                                                                 if (program != nullptr &&
                                                                     location != nullptr) {

                                                                     auto val = canvas_native_webgl_get_uniform(
                                                                             program->GetProgram(),
                                                                             location->GetUniformLocation(),
                                                                             this->GetState());

                                                                     switch (canvas_native_webgl_result_get_type(
                                                                             *val)) {
                                                                         case WebGLResultType::Boolean:
                                                                             return {canvas_native_webgl_result_get_bool(
                                                                                     *val)};
                                                                         case WebGLResultType::None:
                                                                             return jsi::Value::null();
                                                                         case WebGLResultType::String: {
                                                                             auto str = canvas_native_webgl_result_get_string(
                                                                                     *val);
                                                                             return jsi::String::createFromAscii(
                                                                                     runtime,
                                                                                     str.data(),
                                                                                     str.size());
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::BooleanArray: {
                                                                             auto ret = canvas_native_webgl_result_get_bool_array(
                                                                                     *val);
                                                                             auto len = ret.size();
                                                                             auto array = jsi::Array(
                                                                                     runtime, len);
                                                                             for (int i = 0;
                                                                                  i < len; ++i) {
                                                                                 auto item = ret[i];
                                                                                 array.setValueAtIndex(
                                                                                         runtime, i,
                                                                                         jsi::Value(
                                                                                                 item ==
                                                                                                 1));
                                                                             }
                                                                             return array;
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::F32Array: {
                                                                             auto ret = canvas_native_webgl_result_get_f32_array(
                                                                                     *val);

                                                                             auto buf = std::make_shared<VecMutableBuffer<float>>(
                                                                                     std::move(
                                                                                             ret));
                                                                             auto array = jsi::ArrayBuffer(
                                                                                     runtime, buf);

                                                                             auto Float32Array = runtime.global()
                                                                                     .getProperty(
                                                                                             runtime,
                                                                                             "Float32Array")
                                                                                     .asObject(
                                                                                             runtime)
                                                                                     .asFunction(
                                                                                             runtime);


                                                                             return Float32Array.callAsConstructor(
                                                                                     runtime,
                                                                                     array);
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::I32Array: {
                                                                             auto ret = canvas_native_webgl_result_get_i32_array(
                                                                                     *val);

                                                                             auto buf = std::make_shared<VecMutableBuffer<int32_t>>(
                                                                                     std::move(
                                                                                             ret));
                                                                             auto array = jsi::ArrayBuffer(
                                                                                     runtime, buf);

                                                                             auto Int32Array = runtime.global()
                                                                                     .getProperty(
                                                                                             runtime,
                                                                                             "Int32Array")
                                                                                     .asObject(
                                                                                             runtime)
                                                                                     .asFunction(
                                                                                             runtime);


                                                                             return Int32Array.callAsConstructor(
                                                                                     runtime,
                                                                                     array);
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::U32Array: {
                                                                             auto ret = canvas_native_webgl_result_get_u32_array(
                                                                                     *val);

                                                                             auto buf = std::make_shared<VecMutableBuffer<uint32_t>>(
                                                                                     std::move(
                                                                                             ret));
                                                                             auto array = jsi::ArrayBuffer(
                                                                                     runtime, buf);

                                                                             auto Uint32Array = runtime.global()
                                                                                     .getProperty(
                                                                                             runtime,
                                                                                             "Uint32Array")
                                                                                     .asObject(
                                                                                             runtime)
                                                                                     .asFunction(
                                                                                             runtime);


                                                                             return Uint32Array.callAsConstructor(
                                                                                     runtime,
                                                                                     array);
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::F32:
                                                                             return {(double) canvas_native_webgl_result_get_f32(
                                                                                     *val)};
                                                                         case WebGLResultType::I32:
                                                                             return {canvas_native_webgl_result_get_i32(
                                                                                     *val)};
                                                                         case WebGLResultType::U32:
                                                                             return {(int32_t) canvas_native_webgl_result_get_u32(
                                                                                     *val)};
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getVertexAttribOffset") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto pname = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto ret = canvas_native_webgl_get_vertex_attrib_offset(
                                                                     index,
                                                                     pname,
                                                                     this->GetState());
                                                             return {static_cast<double>(ret)};
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getVertexAttrib") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto pname = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto ret = canvas_native_webgl_get_vertex_attrib(
                                                                     index,
                                                                     pname,
                                                                     this->GetState());

                                                             if (pname ==
                                                                 GL_CURRENT_VERTEX_ATTRIB) {
                                                                 auto val = canvas_native_webgl_result_get_f32_array(
                                                                         *ret);

                                                                 auto buf = std::make_shared<VecMutableBuffer<float>>(
                                                                         std::move(val));
                                                                 auto array = jsi::ArrayBuffer(
                                                                         runtime, buf);

                                                                 auto Float32Array = runtime.global()
                                                                         .getProperty(runtime,
                                                                                      "Float32Array")
                                                                         .asObject(runtime)
                                                                         .asFunction(runtime);


                                                                 return Float32Array.callAsConstructor(
                                                                         runtime, array);
                                                             } else if (pname ==
                                                                        GL_VERTEX_ATTRIB_ARRAY_ENABLED ||
                                                                        pname ==
                                                                        GL_VERTEX_ATTRIB_ARRAY_NORMALIZED) {
                                                                 return {canvas_native_webgl_result_get_bool(
                                                                         *ret)};
                                                             } else {
                                                                 return {canvas_native_webgl_result_get_i32(
                                                                         *ret)};
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "hint") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto target = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto mode = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_hint(target,
                                                                                      mode,
                                                                                      this->GetState());
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "isBuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto buffer = args[0].asObject(
                                                                         runtime).asHostObject<WebGLBuffer>(
                                                                         runtime);
                                                                 if (buffer != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_buffer(
                                                                             buffer->GetBuffer(),
                                                                             this->GetState());

                                                                     return {ret};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isContextLost") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto ret = canvas_native_webgl_get_is_context_lost(
                                                                 this->GetState());
                                                         return {ret};
                                                     }
        );
    } else if (methodName == "isEnabled") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto cap = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto ret = canvas_native_webgl_is_enabled(
                                                                     cap, this->GetState());
                                                             return {ret};
                                                         }
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isFramebuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto framebuffer = args[0].asObject(
                                                                         runtime).asHostObject<WebGLFramebuffer>(
                                                                         runtime);
                                                                 if (framebuffer != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_framebuffer(
                                                                             framebuffer->GetFrameBuffer(),
                                                                             this->GetState());

                                                                     return {ret};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isProgram") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto program = args[0].asObject(
                                                                         runtime).asHostObject<WebGLProgram>(
                                                                         runtime);
                                                                 if (program != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_program(
                                                                             program->GetProgram(),
                                                                             this->GetState());

                                                                     return {ret};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isRenderbuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto renderbuffer = args[0].asObject(
                                                                         runtime).asHostObject<WebGLRenderbuffer>(
                                                                         runtime);
                                                                 if (renderbuffer != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_renderbuffer(
                                                                             renderbuffer->GetRenderBuffer(),
                                                                             this->GetState());

                                                                     return {ret};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "isShader") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto shader = args[0].asObject(
                                                                         runtime).asHostObject<WebGLShader>(
                                                                         runtime);
                                                                 if (shader != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_shader(
                                                                             shader->GetShader(),
                                                                             this->GetState());
                                                                     return {ret};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "isTexture") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto texture = args[0].asObject(
                                                                         runtime).asHostObject<WebGLTexture>(
                                                                         runtime);
                                                                 if (texture != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_texture(
                                                                             texture->GetTexture(),
                                                                             this->GetState());
                                                                     return {ret};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "lineWidth") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto width = args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_line_width(
                                                                     static_cast<float>(width),
                                                                     this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "linkProgram") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (args[0].isObject()) {
                                                                 auto program = args[0].asObject(
                                                                         runtime).asHostObject<WebGLProgram>(
                                                                         runtime);
                                                                 if (program != nullptr) {
                                                                     canvas_native_webgl_link_program(
                                                                             program->GetProgram(),
                                                                             this->GetState());
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "pixelStorei") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto pname = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             if (args[1].isBool()) {
                                                                 auto param = args[1].asBool();
                                                                 canvas_native_webgl_pixel_storei(
                                                                         pname,
                                                                         param ? 1 : 0,
                                                                         this->GetState()
                                                                 );
                                                             } else {
                                                                 auto param = (int32_t) args[1]->NumberValue(
                                                                         context).ToChecked();
                                                                 canvas_native_webgl_pixel_storei(
                                                                         pname,
                                                                         param,
                                                                         this->GetState()
                                                                 );
                                                             }

                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "polygonOffset") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto factor = args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto units = args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_polygon_offset(
                                                                     static_cast<float>(factor),
                                                                     static_cast<float>(units),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "readPixels") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     7,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 6) {
                                                             auto x = (int32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto y = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto width = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto height = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             auto format = (uint32_t) args[4]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (uint32_t) args[5]->NumberValue(
                                                                     context).ToChecked();

                                                             if (args[6].isObject()) {
                                                                 auto pixels = args[6].asObject(
                                                                         runtime);
                                                                 if (pixels.isTypedArray(runtime)) {
                                                                     auto buf = pixels.getTypedArray(
                                                                             runtime);
                                                                     auto slice = GetTypedArrayData<uint8_t>(
                                                                             runtime, buf);
                                                                     canvas_native_webgl_read_pixels_u8(
                                                                             x,
                                                                             y,
                                                                             width,
                                                                             height,
                                                                             format,
                                                                             type,
                                                                             slice,
                                                                             this->GetState()
                                                                     );
                                                                     return jsi::Value::undefined();
                                                                 }


                                                                 if (pixels.isArrayBuffer(
                                                                         runtime)) {
                                                                     auto buf = pixels.getArrayBuffer(
                                                                             runtime);
                                                                     auto size = buf.size(runtime);
                                                                     auto data = buf.data(runtime);
                                                                     auto slice = rust::Slice<uint8_t>(
                                                                             data, size);
                                                                     canvas_native_webgl_read_pixels_u8(
                                                                             x,
                                                                             y,
                                                                             width,
                                                                             height,
                                                                             format,
                                                                             type,
                                                                             slice,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "renderbufferStorage") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 3) {
                                                             auto target = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto internalFormat = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto width = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto height = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_renderbuffer_storage(
                                                                     target,
                                                                     internalFormat,
                                                                     width,
                                                                     height,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "sampleCoverage") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto value = args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto invert = args[1].asBool();
                                                             canvas_native_webgl_sample_coverage(
                                                                     static_cast<float>(value),
                                                                     invert,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "scissor") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto x = (int32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto y = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto width = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto height = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_scissor(
                                                                     x,
                                                                     y,
                                                                     width,
                                                                     height,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "shaderSource") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             if (args[0].isObject() &&
                                                                 args[1].isString()) {
                                                                 auto shader = args[0].asObject(
                                                                         runtime).asHostObject<WebGLShader>(
                                                                         runtime);
                                                                 auto source = args[1].asString(
                                                                         runtime).utf8(runtime);
                                                                 if (shader != nullptr) {
                                                                     canvas_native_webgl_shader_source(
                                                                             shader->GetShader(),
                                                                             rust::Str(
                                                                                     source.c_str()),
                                                                             this->GetState());
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "stencilFuncSeparate") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto face = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto func = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto ref = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto mask = (uint32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_stencil_func_separate(
                                                                     face,
                                                                     func,
                                                                     ref,
                                                                     mask,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "stencilFunc") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto func = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto ref = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto mask = (uint32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_stencil_func(
                                                                     func,
                                                                     ref,
                                                                     mask,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "stencilMaskSeparate") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto face = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto mask = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_stencil_mask_separate(
                                                                     face,
                                                                     mask,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "stencilMask") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto mask = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_stencil_mask(
                                                                     mask,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "stencilOpSeparate") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto face = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto fail = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto zfail = (uint32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto zpass = (uint32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_stencil_op_separate(
                                                                     face,
                                                                     fail,
                                                                     zfail,
                                                                     zpass,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "stencilOp") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto fail = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto zfail = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto zpass = (uint32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_stencil_op(
                                                                     fail,
                                                                     zfail,
                                                                     zpass,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "texImage2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     9,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         // TODO tidy

                                                         if (count == 5) {
                                                             auto target = (int32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto internalformat = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto format = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (int32_t) args[4]->NumberValue(
                                                                     context).ToChecked();

                                                             canvas_native_webgl_tex_image2d_image_none(
                                                                     target,
                                                                     level,
                                                                     internalformat,
                                                                     format,
                                                                     type,
                                                                     this->GetState()
                                                             );

                                                         } else if (count == 6) {
                                                             auto target = (int32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto internalformat = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto format = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (int32_t) args[4]->NumberValue(
                                                                     context).ToChecked();

                                                             if (args[5].isObject()) {
                                                                 auto pixels = args[5].asObject(
                                                                         runtime);


                                                                 if (pixels.isHostObject(runtime)) {
                                                                     /*
                                                                     try {
                                                                         auto image_asset = pixels.asHostObject<ImageAssetImpl>(
                                                                                 runtime);

                                                                         if (image_asset !=
                                                                             nullptr) {

                                                                             canvas_native_webgl_tex_image2d_image_asset(
                                                                                     target,
                                                                                     level,
                                                                                     internalformat,
                                                                                     format,
                                                                                     type,
                                                                                     image_asset->GetImageAsset(),
                                                                                     this->GetState()
                                                                             );


                                                                             return jsi::Value::undefined();
                                                                         }
                                                                     } catch (...) {}


                                                                     try {
                                                                         // try ImageBitmapImpl
                                                                         auto image_bitmap = pixels.asHostObject<ImageBitmapImpl>(
                                                                                 runtime);

                                                                         if (image_bitmap !=
                                                                             nullptr) {
                                                                             canvas_native_webgl_tex_image2d_image_asset(
                                                                                     target,
                                                                                     level,
                                                                                     internalformat,
                                                                                     format,
                                                                                     type,
                                                                                     image_bitmap->GetImageAsset(),
                                                                                     this->GetState()
                                                                             );
                                                                         }

                                                                     } catch (...) {}


                                                                     try {
                                                                         auto canvas_2d = pixels.asHostObject<CanvasRenderingContext2DImpl>(
                                                                                 runtime);

                                                                         if (canvas_2d != nullptr) {
                                                                             canvas_native_webgl_tex_image2d_canvas2d(
                                                                                     target,
                                                                                     level,
                                                                                     internalformat,
                                                                                     format,
                                                                                     type,
                                                                                     canvas_2d->GetContext(),
                                                                                     this->GetState()
                                                                             );
                                                                             return jsi::Value::undefined();
                                                                         }
                                                                     } catch (...) {}
                                                                      */


                                                                     try {
                                                                         auto gl = pixels.asHostObject<WebGLRenderingContext>(
                                                                                 runtime);

                                                                         if (gl != nullptr) {
                                                                             canvas_native_webgl_tex_image2d_webgl(
                                                                                     target,
                                                                                     level,
                                                                                     internalformat,
                                                                                     format,
                                                                                     type,
                                                                                     gl->GetState(),
                                                                                     this->GetState()
                                                                             );

                                                                             return jsi::Value::undefined();
                                                                         }
                                                                     } catch (...) {}


                                                                 }

                                                             }
                                                         } else if (count == 8) {
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
                                                             auto border = (int32_t) args[5]->NumberValue(
                                                                     context).ToChecked();
                                                             auto format = (int32_t) args[6]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (int32_t) args[7]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_tex_image2d_none(
                                                                     target,
                                                                     level,
                                                                     internalformat,
                                                                     width,
                                                                     height,
                                                                     border,
                                                                     format,
                                                                     type,
                                                                     this->GetState()
                                                             );
                                                         } else if (count == 9) {
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
                                                             auto border = (int32_t) args[5]->NumberValue(
                                                                     context).ToChecked();
                                                             auto format = (int32_t) args[6]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (int32_t) args[7]->NumberValue(
                                                                     context).ToChecked();


                                                             if (args[8].isObject()) {
                                                                 auto pixels = args[8].asObject(
                                                                         runtime);
                                                                 if (pixels.isTypedArray(runtime)) {
                                                                     auto typedArray = pixels.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<uint8_t>(
                                                                             runtime, typedArray);
                                                                     canvas_native_webgl_tex_image2d(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             width,
                                                                             height,
                                                                             border,
                                                                             format,
                                                                             type,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                     return jsi::Value::undefined();
                                                                 } else if (pixels.isArrayBuffer(
                                                                         runtime)) {
                                                                     auto array = pixels.getArrayBuffer(
                                                                             runtime);
                                                                     auto data = array.data(
                                                                             runtime);
                                                                     auto size = array.size(
                                                                             runtime);
                                                                     rust::Slice<uint8_t> buf(data,
                                                                                              size);
                                                                     canvas_native_webgl_tex_image2d(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             width,
                                                                             height,
                                                                             border,
                                                                             format,
                                                                             type,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } else if (args[8].isNull() ||
                                                                        args[8].isUndefined()) {
                                                                 canvas_native_webgl_tex_image2d_none(
                                                                         target,
                                                                         level,
                                                                         internalformat,
                                                                         width,
                                                                         height,
                                                                         border,
                                                                         format,
                                                                         type,
                                                                         this->GetState()
                                                                 );
                                                             }

                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "texParameterf") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto target = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto pname = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto param = args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_tex_parameterf(
                                                                     target,
                                                                     pname,
                                                                     static_cast<float>(param),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "texParameteri") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto target = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto pname = (uint32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto param = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_tex_parameteri(
                                                                     target,
                                                                     pname,
                                                                     param,
                                                                     this->GetState()
                                                             );
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "texSubImage2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     9,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count == 7) {
                                                             auto target = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto xoffset = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto yoffset = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             auto format = (uint32_t) args[4]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (int32_t) args[5]->NumberValue(
                                                                     context).ToChecked();


                                                             if (args[6].isObject()) {
                                                                 auto pixels = args[6].asObject(
                                                                         runtime);
                                                                 if (pixels.isHostObject(runtime)) {
                                                                     /*
                                                                     try {
                                                                         auto asset = pixels.asHostObject<ImageAssetImpl>(
                                                                                 runtime);
                                                                         if (asset != nullptr) {
                                                                             canvas_native_webgl_tex_sub_image2d_asset(
                                                                                     target,
                                                                                     level,
                                                                                     xoffset,
                                                                                     yoffset,
                                                                                     format,
                                                                                     type,
                                                                                     asset->GetImageAsset(),
                                                                                     this->GetState()
                                                                             );
                                                                             return jsi::Value::undefined();
                                                                         }
                                                                     } catch (...) {}

                                                                     try {
                                                                         auto bitmap = pixels.asHostObject<ImageBitmapImpl>(
                                                                                 runtime);

                                                                         if (bitmap != nullptr) {
                                                                             canvas_native_webgl_tex_sub_image2d_asset(
                                                                                     target,
                                                                                     level,
                                                                                     xoffset,
                                                                                     yoffset,
                                                                                     format,
                                                                                     type,
                                                                                     bitmap->GetImageAsset(),
                                                                                     this->GetState()
                                                                             );

                                                                             return jsi::Value::undefined();
                                                                         }
                                                                     } catch (...) {}


                                                                     try {
                                                                         auto canvas2d = pixels.asHostObject<CanvasRenderingContext2DImpl>(
                                                                                 runtime);

                                                                         if (canvas2d != nullptr) {
                                                                             canvas_native_webgl_tex_sub_image2d_canvas2d(
                                                                                     target,
                                                                                     level,
                                                                                     xoffset,
                                                                                     yoffset,
                                                                                     format,
                                                                                     type,
                                                                                     canvas2d->GetContext(),
                                                                                     this->GetState()
                                                                             );

                                                                             return jsi::Value::undefined();
                                                                         }
                                                                     } catch (...) {}

                                                                      */

                                                                     try {
                                                                         auto webgl = pixels.asHostObject<WebGLRenderingContext>(
                                                                                 runtime);

                                                                         if (webgl != nullptr) {
                                                                             canvas_native_webgl_tex_sub_image2d_webgl(
                                                                                     target,
                                                                                     level,
                                                                                     xoffset,
                                                                                     yoffset,
                                                                                     format,
                                                                                     type,
                                                                                     webgl->GetState(),
                                                                                     this->GetState()
                                                                             );

                                                                             return jsi::Value::undefined();
                                                                         }
                                                                     } catch (...) {}

                                                                 }
                                                             }

                                                         } else if (count == 9) {
                                                             auto target = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto level = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto xoffset = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto yoffset = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             auto width = (int32_t) args[4]->NumberValue(
                                                                     context).ToChecked();
                                                             auto height = (int32_t) args[5]->NumberValue(
                                                                     context).ToChecked();
                                                             auto format = (uint32_t) args[6]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (int32_t) args[7]->NumberValue(
                                                                     context).ToChecked();

                                                             if (args[8].isObject()) {
                                                                 auto pixels = args[8].asObject(
                                                                         runtime);

                                                                 if (pixels.isTypedArray(runtime)) {
                                                                     auto array = pixels.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<const uint8_t>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_tex_sub_image2d(
                                                                             target,
                                                                             level,
                                                                             xoffset,
                                                                             yoffset,
                                                                             width,
                                                                             height,
                                                                             format,
                                                                             type,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                 } else if (pixels.isArrayBuffer(
                                                                         runtime)) {
                                                                     auto array = pixels.getArrayBuffer(
                                                                             runtime);
                                                                     auto data = array.data(
                                                                             runtime);
                                                                     auto size = array.size(
                                                                             runtime);
                                                                     rust::Slice<const uint8_t> buf(
                                                                             data, size);
                                                                     canvas_native_webgl_tex_sub_image2d(
                                                                             target,
                                                                             level,
                                                                             xoffset,
                                                                             yoffset,
                                                                             width,
                                                                             height,
                                                                             format,
                                                                             type,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib1f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto v0 = static_cast<float>(args[1]->NumberValue(
                                                                     context).ToChecked());
                                                             canvas_native_webgl_vertex_attrib1f(
                                                                     index, v0, this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib1fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             if (args[1].isObject()) {
                                                                 auto v0 = args[1].asObject(
                                                                         runtime);
                                                                 if (v0.isFloat32Array(runtime)) {
                                                                     auto array = v0.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<const float>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_vertex_attrib1fv(
                                                                             index, buf,
                                                                             this->GetState());
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib2f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto v0 = static_cast<float>(args[1]->NumberValue(
                                                                     context).ToChecked());
                                                             auto v1 = static_cast<float>(args[2]->NumberValue(
                                                                     context).ToChecked());
                                                             canvas_native_webgl_vertex_attrib2f(
                                                                     index, v0, v1,
                                                                     this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib2fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             if (args[1].isObject()) {
                                                                 auto v0 = args[1].asObject(
                                                                         runtime);
                                                                 if (v0.isFloat32Array(runtime)) {
                                                                     auto array = v0.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<const float>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_vertex_attrib2fv(
                                                                             index, buf,
                                                                             this->GetState());
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib3f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto v0 = static_cast<float>(args[1]->NumberValue(
                                                                     context).ToChecked());
                                                             auto v1 = static_cast<float>(args[2]->NumberValue(
                                                                     context).ToChecked());
                                                             auto v2 = static_cast<float>(args[3]->NumberValue(
                                                                     context).ToChecked());
                                                             canvas_native_webgl_vertex_attrib3f(
                                                                     index, v0, v1, v2,
                                                                     this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib3fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             if (args[1].isObject()) {
                                                                 auto v0 = args[1].asObject(
                                                                         runtime);
                                                                 if (v0.isFloat32Array(runtime)) {
                                                                     auto array = v0.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<const float>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_vertex_attrib3fv(
                                                                             index, buf,
                                                                             this->GetState());
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib4f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto v0 = static_cast<float>(args[1]->NumberValue(
                                                                     context).ToChecked());
                                                             auto v1 = static_cast<float>(args[2]->NumberValue(
                                                                     context).ToChecked());
                                                             auto v2 = static_cast<float>(args[3]->NumberValue(
                                                                     context).ToChecked());
                                                             auto v3 = static_cast<float>(args[4]->NumberValue(
                                                                     context).ToChecked());
                                                             canvas_native_webgl_vertex_attrib4f(
                                                                     index, v0, v1, v2, v3,
                                                                     this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttrib4fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             if (args[1].isObject()) {
                                                                 auto v0 = args[1].asObject(
                                                                         runtime);
                                                                 if (v0.isFloat32Array(runtime)) {
                                                                     auto array = v0.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<const float>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_vertex_attrib4fv(
                                                                             index, buf,
                                                                             this->GetState());
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttribPointer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 5) {
                                                             auto index = (uint32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto size = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto type = (uint32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto normalized = args[3].asBool();
                                                             auto stride = (int32_t) args[4]->NumberValue(
                                                                     context).ToChecked();
                                                             auto offset = static_cast<ssize_t>(args[5]->NumberValue(
                                                                     context).ToChecked());
                                                             canvas_native_webgl_vertex_attrib_pointer(
                                                                     index, size, type, normalized,
                                                                     stride, offset,
                                                                     this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform1f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         //if (count > 1) {
                                                         //   if (args[0].isObject()) {
                                                         auto location = args[0].asObject(
                                                                 runtime).asHostObject<WebGLUniformLocation>(
                                                                 runtime);
                                                         auto v0 = args[1]->NumberValue(
                                                                 context).ToChecked();
                                                         if (location != nullptr) {
                                                             canvas_native_webgl_uniform1f(
                                                                     location->GetUniformLocation(),
                                                                     static_cast<float>(v0),
                                                                     this->GetState()
                                                             );
                                                         }
                                                         //   }
                                                         // }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform1iv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             if (args[0].isObject() &&
                                                                 args[1].isObject()) {
                                                                 auto locationObject = args[0].asObject(
                                                                         runtime);
                                                                 if (locationObject.isHostObject(
                                                                         runtime)) {
                                                                     auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                             runtime);
                                                                     auto v0 = args[1].asObject(
                                                                             runtime);

                                                                     if (location != nullptr) {
                                                                         if (v0.isInt32Array(
                                                                                 runtime)) {
                                                                             auto array = v0.getTypedArray(
                                                                                     runtime);
                                                                             auto buf = GetTypedArrayData<const int32_t>(
                                                                                     runtime,
                                                                                     array);
                                                                             canvas_native_webgl_uniform1iv(
                                                                                     location->GetUniformLocation(),
                                                                                     buf,
                                                                                     this->GetState());
                                                                         } else if (v0.isArray(
                                                                                 runtime)) {
                                                                             auto array = v0.getArray(
                                                                                     runtime);
                                                                             auto len = array.size(
                                                                                     runtime);

                                                                             std::vector<int32_t> buf;
                                                                             buf.reserve(len);
                                                                             for (int i = 0;
                                                                                  i < len; ++i) {
                                                                                 auto item = array.getValueAtIndex(
                                                                                         runtime,
                                                                                         (uint32_t) i)->NumberValue(
                                                                                         context).ToChecked();
                                                                                 buf.push_back(
                                                                                         static_cast<int32_t>(item));
                                                                             }
                                                                             rust::Slice<const int32_t> slice(
                                                                                     buf.data(),
                                                                                     buf.size());
                                                                             canvas_native_webgl_uniform1iv(
                                                                                     location->GetUniformLocation(),
                                                                                     slice,
                                                                                     this->GetState());
                                                                         }
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform1fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[0].isObject() &&
                                                             args[1].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             auto v0 = args[1].asObject(
                                                                     runtime);

                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 if (location != nullptr) {
                                                                     if (v0.isFloat32Array(
                                                                             runtime)) {
                                                                         auto array = v0.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const float>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform1fv(
                                                                                 location->GetUniformLocation(),
                                                                                 buf,
                                                                                 this->GetState());
                                                                     } else if (v0.isArray(
                                                                             runtime)) {
                                                                         auto array = v0.getArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0;
                                                                              i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime, i);
                                                                             if (!item.isNumber()) {
                                                                                 buf.push_back(
                                                                                         nanf(""));
                                                                             } else {
                                                                                 auto value = item->NumberValue(
                                                                                         context).ToChecked();
                                                                                 buf.push_back(
                                                                                         static_cast<float>(value));
                                                                             }
                                                                         }
                                                                         rust::Slice<const float> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform1fv(
                                                                                 location->GetUniformLocation(),
                                                                                 slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform1i") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[0].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = (int32_t) args[1]->NumberValue(
                                                                         context).ToChecked();
                                                                 if (location != nullptr) {
                                                                     canvas_native_webgl_uniform1i(
                                                                             location->GetUniformLocation(),
                                                                             v0,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform2f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             if (args[0].isObject()) {
                                                                 auto locationObject = args[0].asObject(
                                                                         runtime);
                                                                 if (locationObject.isHostObject(
                                                                         runtime)) {
                                                                     auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                             runtime);
                                                                     auto v0 = args[1]->NumberValue(
                                                                             context).ToChecked();
                                                                     auto v1 = args[2]->NumberValue(
                                                                             context).ToChecked();
                                                                     if (location != nullptr) {
                                                                         canvas_native_webgl_uniform2f(
                                                                                 location->GetUniformLocation(),
                                                                                 static_cast<float>(v0),
                                                                                 static_cast<float>(v1),
                                                                                 this->GetState()
                                                                         );
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform2iv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[0].isObject() &&
                                                             args[1].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = args[1].asObject(
                                                                         runtime);
                                                                 if (location != nullptr) {
                                                                     if (v0.isInt32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const int32_t>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform2iv(
                                                                                 location->GetUniformLocation(),
                                                                                 buf,
                                                                                 this->GetState());
                                                                     } else if (v0.isArray(
                                                                             runtime)) {
                                                                         auto array = v0.asArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);

                                                                         std::vector<int32_t> buf;
                                                                         buf.reserve(len);
                                                                         for (size_t i = 0;
                                                                              i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime,
                                                                                     i)->NumberValue(
                                                                                     context).ToChecked();
                                                                             buf.push_back(
                                                                                     static_cast<int32_t>(item));
                                                                         }
                                                                         rust::Slice<const int32_t> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform2iv(
                                                                                 location->GetUniformLocation(),
                                                                                 slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform2fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[0].isObject() &&
                                                             args[1].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = args[1].asObject(
                                                                         runtime);
                                                                 if (location != nullptr) {
                                                                     if (v0.isFloat32Array(
                                                                             runtime)) {
                                                                         auto array = v0.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const float>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform2fv(
                                                                                 location->GetUniformLocation(),
                                                                                 buf,
                                                                                 this->GetState());
                                                                     } else if (v0.isArray(
                                                                             runtime)) {
                                                                         auto array = v0.asArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);

                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (size_t i = 0;
                                                                              i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime,
                                                                                     i)->NumberValue(
                                                                                     context).ToChecked();
                                                                             buf.push_back(
                                                                                     static_cast<float>(item));
                                                                         }
                                                                         rust::Slice<const float> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform2fv(
                                                                                 location->GetUniformLocation(),
                                                                                 slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform2i") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && args[0].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = (int32_t) args[1]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v1 = (int32_t) args[2]->NumberValue(
                                                                         context).ToChecked();
                                                                 if (location != nullptr) {
                                                                     canvas_native_webgl_uniform2i(
                                                                             location->GetUniformLocation(),
                                                                             v0,
                                                                             v1,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform3f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3 && args[0].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = args[1]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v1 = args[2]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v2 = args[3]->NumberValue(
                                                                         context).ToChecked();
                                                                 if (location != nullptr) {
                                                                     canvas_native_webgl_uniform3f(
                                                                             location->GetUniformLocation(),
                                                                             static_cast<float>(v0),
                                                                             static_cast<float>(v1),
                                                                             static_cast<float>(v2),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform3iv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1 && args[0].isObject() &&
                                                             args[1].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = args[1].asObject(
                                                                         runtime);

                                                                 if (location != nullptr) {
                                                                     if (v0.isInt32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const int32_t>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform3iv(
                                                                                 location->GetUniformLocation(),
                                                                                 buf,
                                                                                 this->GetState());
                                                                     } else if (v0.isArray(
                                                                             runtime)) {
                                                                         auto array = v0.asArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);

                                                                         std::vector<int32_t> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0;
                                                                              i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime,
                                                                                     (uint32_t) i)->NumberValue(
                                                                                     context).ToChecked();
                                                                             buf.push_back(
                                                                                     static_cast<int32_t>(item));
                                                                         }
                                                                         rust::Slice<const int32_t> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform3iv(
                                                                                 location->GetUniformLocation(),
                                                                                 slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform3fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[0].isObject() &&
                                                             args[1].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             auto v0 = args[1].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);

                                                                 if (location != nullptr) {


                                                                     if (v0.isFloat32Array(
                                                                             runtime)) {
                                                                         auto array = v0.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const float>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform3fv(
                                                                                 location->GetUniformLocation(),
                                                                                 buf,
                                                                                 this->GetState());
                                                                     } else if (v0.isArray(
                                                                             runtime)) {
                                                                         auto array = v0.getArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0;
                                                                              i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime, i);
                                                                             if (!item.isNumber()) {
                                                                                 buf.push_back(
                                                                                         nanf(""));
                                                                             } else {
                                                                                 auto value = item->NumberValue(
                                                                                         context).ToChecked();
                                                                                 buf.push_back(
                                                                                         static_cast<float>(value));
                                                                             }
                                                                         }
                                                                         rust::Slice<const float> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform3fv(
                                                                                 location->GetUniformLocation(),
                                                                                 slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }

                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform3i") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3 && args[0].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = (int32_t) args[1]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v1 = (int32_t) args[2]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v2 = (int32_t) args[3]->NumberValue(
                                                                         context).ToChecked();

                                                                 if (location != nullptr) {
                                                                     canvas_native_webgl_uniform3i(
                                                                             location->GetUniformLocation(),
                                                                             v0,
                                                                             v1,
                                                                             v2,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform4f") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4 && args[0].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = args[1]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v1 = args[2]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v2 = args[3]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v3 = args[4]->NumberValue(
                                                                         context).ToChecked();

                                                                 if (location != nullptr) {
                                                                     canvas_native_webgl_uniform4f(
                                                                             location->GetUniformLocation(),
                                                                             static_cast<float>(v0),
                                                                             static_cast<float>(v1),
                                                                             static_cast<float>(v2),
                                                                             static_cast<float>(v3),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform4iv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[0].isObject() &&
                                                             args[1].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             auto v0 = args[1].asObject(
                                                                     runtime);

                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 if (location != nullptr) {}
                                                                 if (v0.isInt32Array(runtime)) {
                                                                     auto array = v0.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<const int32_t>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_uniform4iv(
                                                                             location->GetUniformLocation(),
                                                                             buf,
                                                                             this->GetState());
                                                                 } else if (v0.isArray(runtime)) {
                                                                     auto array = v0.getArray(
                                                                             runtime);
                                                                     auto len = array.size(runtime);

                                                                     std::vector<int32_t> buf;
                                                                     buf.reserve(len);
                                                                     for (int i = 0; i < len; ++i) {
                                                                         auto item = array.getValueAtIndex(
                                                                                 runtime,
                                                                                 (uint32_t) i)->NumberValue(
                                                                                 context).ToChecked();
                                                                         buf.push_back(
                                                                                 static_cast<int32_t>(item));
                                                                     }
                                                                     rust::Slice<const int32_t> slice(
                                                                             buf.data(),
                                                                             buf.size());
                                                                     canvas_native_webgl_uniform4iv(
                                                                             location->GetUniformLocation(),
                                                                             slice,
                                                                             this->GetState());
                                                                 }
                                                             }

                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform4fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && args[0].isObject() &&
                                                             args[1].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             auto v0 = args[1].asObject(
                                                                     runtime);

                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 if (location != nullptr) {
                                                                     if (v0.isFloat32Array(
                                                                             runtime)) {
                                                                         auto array = v0.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const float>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform4fv(
                                                                                 location->GetUniformLocation(),
                                                                                 buf,
                                                                                 this->GetState());
                                                                     } else if (v0.isArray(
                                                                             runtime)) {
                                                                         auto array = v0.getArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);

                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0;
                                                                              i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime,
                                                                                     (uint32_t) i)->NumberValue(
                                                                                     context).ToChecked();
                                                                             buf.push_back(
                                                                                     static_cast<float>(item));
                                                                         }
                                                                         rust::Slice<const float> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform4fv(
                                                                                 location->GetUniformLocation(),
                                                                                 slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }
                                                             }

                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniform4i") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4 && args[0].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto v0 = (int32_t) args[1]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v1 = (int32_t) args[2]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v2 = (int32_t) args[3]->NumberValue(
                                                                         context).ToChecked();
                                                                 auto v3 = (int32_t) args[4]->NumberValue(
                                                                         context).ToChecked();

                                                                 if (location != nullptr) {
                                                                     canvas_native_webgl_uniform4i(
                                                                             location->GetUniformLocation(),
                                                                             v0,
                                                                             v1,
                                                                             v2,
                                                                             v3,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniformMatrix2fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && args[0].isObject() &&
                                                             args[2].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto transpose = args[1].asBool();
                                                                 auto value = args[2].asObject(
                                                                         runtime);

                                                                 if (location != nullptr) {
                                                                     if (value.isFloat32Array(
                                                                             runtime)) {
                                                                         auto array = value.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const float>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform_matrix2fv(
                                                                                 location->GetUniformLocation(),
                                                                                 transpose, buf,
                                                                                 this->GetState());
                                                                     } else if (value.isArray(
                                                                             runtime)) {
                                                                         auto array = value.getArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);

                                                                         for (int i = 0;
                                                                              i < len; i++) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime, i);
                                                                             if (item.isNumber()) {
                                                                                 buf.push_back(
                                                                                         static_cast<float>(item->NumberValue(
                                                                                                 context).ToChecked()));
                                                                             } else {
                                                                                 buf.push_back(
                                                                                         std::nanf(
                                                                                                 ""));
                                                                             }
                                                                         }

                                                                         rust::Slice<const float> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform_matrix2fv(
                                                                                 location->GetUniformLocation(),
                                                                                 transpose, slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniformMatrix3fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && args[0].isObject() &&
                                                             args[2].isObject()) {
                                                             auto locationObject = args[0].asObject(
                                                                     runtime);
                                                             if (locationObject.isHostObject(
                                                                     runtime)) {
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(
                                                                         runtime);
                                                                 auto transpose = args[1].asBool();
                                                                 auto value = args[2].asObject(
                                                                         runtime);

                                                                 if (location != nullptr) {
                                                                     if (value.isFloat32Array(
                                                                             runtime)) {
                                                                         auto array = value.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const float>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_uniform_matrix3fv(
                                                                                 location->GetUniformLocation(),
                                                                                 transpose, buf,
                                                                                 this->GetState());
                                                                     } else if (value.isArray(
                                                                             runtime)) {
                                                                         auto array = value.getArray(
                                                                                 runtime);
                                                                         auto len = array.size(
                                                                                 runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);

                                                                         for (int i = 0;
                                                                              i < len; i++) {
                                                                             auto item = array.getValueAtIndex(
                                                                                     runtime, i);
                                                                             if (item.isNumber()) {
                                                                                 buf.push_back(
                                                                                         static_cast<float>(item->NumberValue(
                                                                                                 context).ToChecked()));
                                                                             } else {
                                                                                 buf.push_back(
                                                                                         std::nanf(
                                                                                                 ""));
                                                                             }
                                                                         }

                                                                         rust::Slice<const float> slice(
                                                                                 buf.data(),
                                                                                 buf.size());
                                                                         canvas_native_webgl_uniform_matrix3fv(
                                                                                 location->GetUniformLocation(),
                                                                                 transpose, slice,
                                                                                 this->GetState());
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "uniformMatrix4fv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && args[0].isObject() &&
                                                             args[2].isObject()) {
                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, args[0]);
                                                             if (location != nullptr) {
                                                                 auto transpose = args[1].asBool();
                                                                 auto value = args[2].asObject(
                                                                         runtime);

                                                                 if (value.isFloat32Array(
                                                                         runtime)) {
                                                                     auto array = value.getTypedArray(
                                                                             runtime);
                                                                     auto buf = GetTypedArrayData<const float>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_uniform_matrix4fv(
                                                                             location->GetUniformLocation(),
                                                                             transpose, buf,
                                                                             this->GetState());
                                                                 } else if (value.isArray(
                                                                         runtime)) {
                                                                     auto array = value.getArray(
                                                                             runtime);
                                                                     auto len = array.size(
                                                                             runtime);
                                                                     std::vector<float> buf;
                                                                     buf.reserve(len);

                                                                     for (int i = 0;
                                                                          i < len; i++) {
                                                                         auto item = array.getValueAtIndex(
                                                                                 runtime, i);
                                                                         if (item.isNumber()) {
                                                                             buf.push_back(
                                                                                     static_cast<float>(item->NumberValue(
                                                                                             context).ToChecked()));
                                                                         } else {
                                                                             buf.push_back(
                                                                                     std::nanf(
                                                                                             ""));
                                                                         }
                                                                     }

                                                                     rust::Slice<const float> slice(
                                                                             buf.data(),
                                                                             buf.size());
                                                                     canvas_native_webgl_uniform_matrix4fv(
                                                                             location->GetUniformLocation(),
                                                                             transpose, slice,
                                                                             this->GetState());
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "useProgram") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (args[0].isNull()) {
                                                                 canvas_native_webgl_use_program(0,
                                                                                                 this->GetState());

                                                                 return jsi::Value::undefined();
                                                             }

                                                             if (args[0].isObject()) {
                                                                 auto programObject = args[0].asObject(
                                                                         runtime);
                                                                 if (programObject.isHostObject(
                                                                         runtime)) {
                                                                     auto program = programObject.asHostObject<WebGLProgram>(
                                                                             runtime);
                                                                     if (program != nullptr) {
                                                                         canvas_native_webgl_use_program(
                                                                                 program->GetProgram(),
                                                                                 this->GetState()
                                                                         );
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "validateProgram") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0 && args[0].isObject()) {
                                                             auto programObject = args[0].asObject(
                                                                     runtime);
                                                             if (programObject.isHostObject(
                                                                     runtime)) {
                                                                 auto program = programObject.asHostObject<WebGLProgram>(
                                                                         runtime);
                                                                 if (program != nullptr) {
                                                                     canvas_native_webgl_validate_program(
                                                                             program->GetProgram(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "viewport") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto x = (int32_t) args[0]->NumberValue(
                                                                     context).ToChecked();
                                                             auto y = (int32_t) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                             auto width = (int32_t) args[2]->NumberValue(
                                                                     context).ToChecked();
                                                             auto height = (int32_t) args[3]->NumberValue(
                                                                     context).ToChecked();
                                                             canvas_native_webgl_viewport(x, y,
                                                                                          width,
                                                                                          height,
                                                                                          this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "__toDataURL") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         std::string type("image/png");
                                                         int quality = 92;
                                                         if (args[0].isString()) {
                                                             type = args[0].asString(
                                                                     runtime).utf8(
                                                                     runtime);
                                                         }


                                                         if (args[1].isNumber()) {
                                                             quality = (int) args[1]->NumberValue(
                                                                     context).ToChecked();
                                                         }


                                                         auto data = canvas_native_webgl_to_data_url(
                                                                 this->GetState(),
                                                                 rust::Str(type.c_str()),
                                                                 quality);
                                                         return jsi::String::createFromAscii(
                                                                 runtime,
                                                                 data.data(),
                                                                 data.size());
                                                     }
        );
    }


    return jsi::Value::undefined();
}


