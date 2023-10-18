//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGLRenderingContext.h"
#include "OneByteStringResource.h"

WebGLRenderingContext::WebGLRenderingContext(WebGLState *state)
        : WebGLRenderingContextBase(state, WebGLRenderingVersion::V1) {

}

WebGLRenderingContext::WebGLRenderingContext(WebGLState *state,
                                             WebGLRenderingVersion version)
        : WebGLRenderingContextBase(state, version) {

}


v8::Local<v8::FunctionTemplate> WebGLRenderingContext::GetCtor(v8::Isolate *isolate) {
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

    SetConstants(isolate, tmpl);
    SetProps(isolate, tmpl);
    SetMethods(isolate, tmpl);

    cache->WebGLRenderingContextTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

WebGLRenderingContext *WebGLRenderingContext::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WebGLRenderingContext *>(ptr);
}


v8::Local<v8::Value> WebGLRenderingContext::GetParameterInternal(v8::Isolate *isolate,
                                                                 uint32_t pnameValue,
                                                                 WebGLResult *result) {

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
            auto value = canvas_native_webgl_result_get_i32(result);
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
        case GL_BLEND_COLOR:
        case GL_COLOR_CLEAR_VALUE:
        case GL_DEPTH_RANGE: {
            auto ret = canvas_native_webgl_result_get_f32_array(result);
            auto buf = (uint8_t *) canvas_native_f32_buffer_get_bytes(ret);
            auto size = canvas_native_f32_buffer_get_length(ret);
            auto bytes_size = size * sizeof(float);


            auto store = v8::ArrayBuffer::NewBackingStore(buf, bytes_size,
                                                          [](void *data, size_t length,
                                                             void *deleter_data) {
                                                              if (deleter_data != nullptr) {
                                                                  canvas_native_f32_buffer_destroy(
                                                                          (F32Buffer *) deleter_data);
                                                              }
                                                          },
                                                          ret);

            auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));

            return scope.Escape(v8::Float32Array::New(arraybuffer, 0, size));
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
                    v8::Boolean::New(isolate, canvas_native_webgl_result_get_bool(result)));
        case GL_COLOR_WRITEMASK: {
            auto ret = canvas_native_webgl_result_get_bool_array(result);
            auto len = canvas_native_u8_buffer_get_length(ret);
            auto buf = canvas_native_u8_buffer_get_bytes(ret);
            auto array = v8::Array::New(isolate, (int) len);

            for (int j = 0; j < len; ++j) {
                array->Set(context, j, v8::Boolean::New(isolate, buf[j] == 1));
            }
            return scope.Escape(array);
        }
        case GL_COMPRESSED_TEXTURE_FORMATS:
        case GL_MAX_VIEWPORT_DIMS:
        case GL_SCISSOR_BOX:
        case GL_VIEWPORT: {
            auto ret = canvas_native_webgl_result_get_i32_array(result);

            auto buf = (uint8_t *) canvas_native_i32_buffer_get_bytes(ret);
            auto size = canvas_native_i32_buffer_get_length(ret);
            auto bytes_size = size * sizeof(int32_t);


            auto store = v8::ArrayBuffer::NewBackingStore(buf, bytes_size,
                                                          [](void *data, size_t length,
                                                             void *deleter_data) {
                                                              if (deleter_data != nullptr) {
                                                                  canvas_native_i32_buffer_destroy(
                                                                          (I32Buffer *) deleter_data);
                                                              }
                                                          },
                                                          ret);

            auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));


            return scope.Escape(v8::Int32Array::New(arraybuffer, 0, size));
        }
        case GL_DEPTH_CLEAR_VALUE:
        case GL_LINE_WIDTH:
        case GL_POLYGON_OFFSET_FACTOR:
        case GL_POLYGON_OFFSET_UNITS:
        case GL_SAMPLE_COVERAGE_VALUE: {
            return scope.Escape(v8::Number::New(isolate,
                                                static_cast<double>(canvas_native_webgl_result_get_f32(
                                                        result))));
        }
        case GL_RENDERER:
        case GL_SHADING_LANGUAGE_VERSION:
        case GL_VENDOR:
        case GL_VERSION: {
            auto ret = canvas_native_webgl_result_get_string(result);
            auto value = new OneByteStringResource((char *) ret);
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
                                       ptr->GetState());
}

void WebGLRenderingContext::AttachShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

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
        auto type = GetNativeType(isolate, programValue);
        if (type == NativeType::WebGLProgram && args[1]->IsNumber() &&
            args[2]->IsString()) {
            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());

            auto index = (uint32_t) args[1]->NumberValue(context).ToChecked();
            auto name = ConvertFromV8String(isolate, args[2]);
            canvas_native_webgl_bind_attrib_location(
                    program->GetProgram(),
                    index,
                    name.c_str(),
                    ptr->GetState()
            );

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
                    auto buf = sizeOrBuf.As<v8::Uint16Array>();

                    auto array = buf->Buffer();
                    auto offset = buf->ByteOffset();
                    auto size = buf->Length();
                    auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                    auto data = static_cast<uint16_t *>((void *) data_ptr);

                    canvas_native_webgl_buffer_data_u16(
                            target,
                            data, size,
                            usage,
                            ptr->GetState()
                    );
                } else if (sizeOrBuf->IsFloat32Array()) {
                    auto buf = sizeOrBuf.As<v8::Float32Array>();

                    auto array = buf->Buffer();
                    auto offset = buf->ByteOffset();
                    auto size = buf->Length();
                    auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                    auto data = static_cast<float *>((void *) data_ptr);

                    canvas_native_webgl_buffer_data_f32(
                            target,
                            data, size,
                            usage,
                            ptr->GetState()
                    );
                } else {
                    auto buf = sizeOrBuf.As<v8::ArrayBufferView>();

                    auto array = buf->Buffer();
                    auto offset = buf->ByteOffset();
                    auto size = buf->ByteLength();
                    auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                    auto data = static_cast<uint8_t *>((void *) data_ptr);


                    canvas_native_webgl_buffer_data(
                            target,
                            data, size,
                            usage,
                            ptr->GetState()
                    );
                }
            } else if (sizeOrBuf->IsArrayBuffer()) {
                auto array = sizeOrBuf.As<v8::ArrayBuffer>();

                auto size = array->ByteLength();
                auto data = (uint8_t *) array->GetBackingStore()->Data();


                canvas_native_webgl_buffer_data(
                        target,
                        data, size,
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

            if (buf->IsArrayBufferView()) {
                auto buff = buf.As<v8::ArrayBufferView>();

                auto array = buff->Buffer();
                auto os = buff->ByteOffset();
                auto size = buff->ByteLength();
                auto data_ptr = static_cast<uint8_t *>(array->GetBackingStore()->Data()) + os;
                auto data = static_cast<uint8_t *>((void *) data_ptr);


                canvas_native_webgl_buffer_sub_data(
                        target,
                        static_cast<ssize_t>(offset),
                        data, size,
                        ptr->GetState()
                );
            } else if (buf->IsArrayBuffer()) {
                auto arrayBuffer = buf.As<v8::ArrayBuffer>();

                auto size = arrayBuffer->ByteLength();
                auto data = (uint8_t *) arrayBuffer->GetBackingStore()->Data();

                canvas_native_webgl_buffer_sub_data(
                        target,
                        static_cast<intptr_t>(offset),
                        data, size,
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

    ptr->UpdateInvalidateState();
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
            if (pixels->IsArrayBufferView()) {
                auto buf = pixels.As<v8::ArrayBufferView>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->ByteLength();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<uint8_t *>((void *) data_ptr);


                canvas_native_webgl_compressed_tex_image2d(
                        target,
                        level,
                        internalformat,
                        width,
                        height,
                        border,
                        data, size,
                        ptr->GetState()
                );
            } else if (pixels->IsArrayBuffer()) {
                auto array = pixels.As<v8::ArrayBuffer>();

                auto size = array->ByteLength();
                auto data = (uint8_t *) array->GetBackingStore()->Data();


                canvas_native_webgl_compressed_tex_image2d(
                        target,
                        level,
                        internalformat,
                        width,
                        height,
                        border,
                        data, size,
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
            if (pixels->IsArrayBufferView()) {
                auto buf = pixels.As<v8::ArrayBufferView>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = array->ByteLength();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<uint8_t *>((void *) data_ptr);

                canvas_native_webgl_compressed_tex_sub_image2d(
                        target,
                        level,
                        xoffset,
                        yoffset,
                        width,
                        height,
                        format,
                        data, size,
                        ptr->GetState()
                );
            } else if (pixels->IsArrayBuffer()) {
                auto array = pixels.As<v8::ArrayBuffer>();
                auto size = array->ByteLength();
                auto data = (uint8_t *) array->GetBackingStore()->Data();

                canvas_native_webgl_compressed_tex_sub_image2d(
                        target,
                        level,
                        xoffset,
                        yoffset,
                        width,
                        height,
                        format,
                        data, size,
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


    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLFramebuffer) {
        auto buffer = WebGLFramebuffer::GetPointer(value.As<v8::Object>());
        if (buffer != nullptr) {
            canvas_native_webgl_delete_framebuffer(
                    buffer->GetFrameBuffer(),
                    ptr->GetState()
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
        auto mask = value->BooleanValue(isolate);

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
    ptr->UpdateInvalidateState();
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
            ptr->GetState()
    );
    ptr->UpdateInvalidateState();
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
            ptr->GetState()
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
                    info));
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
                    ptr->GetState()
            );
            auto ret = WebGLActiveInfoImpl::NewInstance(isolate, new WebGLActiveInfoImpl(
                    info));
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

            auto len = canvas_native_u32_buffer_get_length(info);
            auto buf = canvas_native_u32_buffer_get_bytes(info);
            auto array = v8::Array::New(
                    isolate, (int) len);
            for (int i = 0; i < len; ++i) {
                auto shader = WebGLShader::NewInstance(isolate, new WebGLShader(
                        buf[i]));
                array->Set(context, i, shader);
            }
            args.GetReturnValue().Set(array);

            canvas_native_u32_buffer_destroy(info);
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

    auto programValue = args[0];
    auto type = GetNativeType(isolate, programValue);

    if (type == NativeType::WebGLProgram) {
        auto name = ConvertFromV8String(isolate, args[1]);
        auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
        if (program != nullptr) {
            auto location = canvas_native_webgl_get_attrib_location(
                    program->GetProgram(),
                    name.c_str(),
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

void WebGLRenderingContext::GetContextAttributes(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto attr = canvas_native_webgl_get_context_attributes(
            ptr->GetState());

    auto ret = v8::Object::New(isolate);

    auto alpha = canvas_native_webgl_context_attribute_get_get_alpha(
            attr);


    ret->Set(context, ConvertToV8String(isolate, "alpha"), v8::Boolean::New(isolate, alpha));

    auto antialias = canvas_native_webgl_context_attribute_get_get_antialias(
            attr);

    ret->Set(context, ConvertToV8String(isolate, "antialias"),
             v8::Boolean::New(isolate, antialias));

    auto depth = canvas_native_webgl_context_attribute_get_get_depth(
            attr);

    ret->Set(context, ConvertToV8String(isolate, "depth"), v8::Boolean::New(isolate, depth));

    auto fail_if_major_performance_caveat = canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
            attr);

    ret->Set(context,
             ConvertToV8String(isolate, "failIfMajorPerformanceCaveat"),
             v8::Boolean::New(isolate, fail_if_major_performance_caveat));

    auto power_preference = canvas_native_webgl_context_attribute_get_get_power_preference(
            attr);

    ret->Set(context, ConvertToV8String(isolate, "powerPreference"),
             ConvertToV8OneByteString(isolate, (char *) power_preference));

    auto premultiplied_alpha = canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
            attr);

    ret->Set(context,
             ConvertToV8String(isolate, "premultipliedAlpha"),
             v8::Boolean::New(isolate, premultiplied_alpha));

    auto preserve_drawing_buffer = canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
            attr);

    ret->Set(context,
             ConvertToV8String(isolate, "preserveDrawingBuffer"),
             v8::Boolean::New(isolate, preserve_drawing_buffer));

    auto stencil = canvas_native_webgl_context_attribute_get_get_stencil(
            attr);

    ret->Set(context, ConvertToV8String(isolate, "stencil"),
             v8::Boolean::New(isolate, stencil));

    auto desynchronized = canvas_native_webgl_context_attribute_get_get_desynchronized(
            attr);

    ret->Set(context, ConvertToV8String(isolate, "desynchronized"),
             v8::Boolean::New(isolate, desynchronized));

    auto xr_compatible = canvas_native_webgl_context_attribute_get_get_xr_compatible(
            attr);

    ret->Set(context, ConvertToV8String(isolate, "xrCompatible"),
             v8::Boolean::New(isolate, xr_compatible));

    canvas_native_context_attributes_destroy(attr);

    args.GetReturnValue().Set(ret);

}

void WebGLRenderingContext::GetError(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }

    auto ret = canvas_native_webgl_get_error(
            ptr->GetState());

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
            name.c_str(),
            ptr->GetState());

    if (canvas_native_webgl_context_extension_is_none(
            ext)) {
        canvas_native_webgl_extension_destroy(ext);
        args.GetReturnValue().SetNull();
        return;
    }

    auto type = canvas_native_webgl_context_extension_get_type(
            ext);
    switch (type) {
        case WebGLExtensionType::WebGLExtensionTypeOES_fbo_render_mipmap: {
            auto ret = OES_fbo_render_mipmapImpl::NewInstance(isolate,
                                                              new OES_fbo_render_mipmapImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeEXT_blend_minmax: {
            auto ret = EXT_blend_minmaxImpl::NewInstance(isolate, new EXT_blend_minmaxImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeEXT_color_buffer_half_float: {
            auto ret = EXT_color_buffer_half_floatImpl::NewInstance(isolate,
                                                                    new EXT_color_buffer_half_floatImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeEXT_disjoint_timer_query: {
            auto ret = canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(ext);
            auto query = EXT_disjoint_timer_queryImpl::NewInstance(isolate,
                                                                   new EXT_disjoint_timer_queryImpl(
                                                                           ret));

            args.GetReturnValue().Set(query);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeEXT_sRGB: {
            auto ret = EXT_sRGBImpl::NewInstance(isolate, new EXT_sRGBImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeEXT_shader_texture_lod: {
            auto ret = EXT_shader_texture_lodImpl::NewInstance(isolate,
                                                               new EXT_shader_texture_lodImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeEXT_texture_filter_anisotropic: {
            auto ret = EXT_texture_filter_anisotropicImpl::NewInstance(isolate,
                                                                       new EXT_texture_filter_anisotropicImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeOES_element_index_uint: {
            auto ret = OES_element_index_uintImpl::NewInstance(isolate,
                                                               new OES_element_index_uintImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeOES_standard_derivatives: {
            auto ret = OES_standard_derivativesImpl::NewInstance(
                    isolate, new OES_standard_derivativesImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeOES_texture_float: {
            auto ret = OES_texture_floatImpl::NewInstance(isolate, new OES_texture_floatImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeOES_texture_float_linear: {
            auto ret = OES_texture_float_linearImpl::NewInstance(isolate,
                                                                 new OES_texture_float_linearImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeOES_texture_half_float: {
            auto ret = OES_texture_half_floatImpl::NewInstance(isolate,
                                                               new OES_texture_half_floatImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeOES_texture_half_float_linear: {
            auto ret = OES_texture_half_float_linearImpl::NewInstance(isolate,
                                                                      new OES_texture_half_float_linearImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeOES_vertex_array_object: {
            auto ret = canvas_native_webgl_context_extension_to_oes_vertex_array_object(ext);
            auto array = OES_vertex_array_objectImpl::NewInstance(isolate,
                                                                  new OES_vertex_array_objectImpl(
                                                                          ret));
            args.GetReturnValue().Set(array);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_color_buffer_float: {
            auto ret = WEBGL_color_buffer_floatImpl::NewInstance(isolate,
                                                                 new WEBGL_color_buffer_floatImpl());
            if (ptr->GetVersion() ==
                WebGLRenderingVersion::V2) {
                ret->Set(context, ConvertToV8String(isolate, "ext_name"),
                         ConvertToV8String(isolate, "EXT_color_buffer_float"));
            }
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_atc: {
            auto ret = WEBGL_compressed_texture_atcImpl::NewInstance(isolate,
                                                                     new WEBGL_compressed_texture_atcImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_etc1: {
            auto ret = WEBGL_compressed_texture_etc1Impl::NewInstance(isolate,
                                                                      new WEBGL_compressed_texture_etc1Impl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_s3tc: {
            auto ret = WEBGL_compressed_texture_s3tcImpl::NewInstance(isolate,
                                                                      new WEBGL_compressed_texture_s3tcImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_s3tc_srgb: {
            auto ret = WEBGL_compressed_texture_s3tc_srgbImpl::NewInstance(isolate,
                                                                           new WEBGL_compressed_texture_s3tc_srgbImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_etc: {
            auto ret = WEBGL_compressed_texture_etcImpl::NewInstance(isolate,
                                                                     new WEBGL_compressed_texture_etcImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_pvrtc: {
            auto ret = WEBGL_compressed_texture_pvrtcImpl::NewInstance(isolate,
                                                                       new WEBGL_compressed_texture_pvrtcImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_lose_context: {
            auto ret = canvas_native_webgl_context_extension_to_lose_context(ext);
            auto ctx = WEBGL_lose_contextImpl::NewInstance(isolate,
                                                           new WEBGL_lose_contextImpl(ret));

            args.GetReturnValue().Set(ctx);

            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeANGLE_instanced_arrays: {
            auto ret = canvas_native_webgl_context_extension_to_angle_instanced_arrays(ext);
            auto instance = ANGLE_instanced_arraysImpl::NewInstance(isolate,
                                                                    new ANGLE_instanced_arraysImpl(
                                                                            ret));

            args.GetReturnValue().Set(instance);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_depth_texture: {
            auto ret = WEBGL_depth_textureImpl::NewInstance(isolate, new WEBGL_depth_textureImpl());
            args.GetReturnValue().Set(ret);
            canvas_native_webgl_extension_destroy(ext);
            return;
        }
        case WebGLExtensionType::WebGLExtensionTypeWEBGL_draw_buffers: {
            auto ret = canvas_native_webgl_context_extension_to_draw_buffers(ext);

            auto buffers = WEBGL_draw_buffersImpl::NewInstance(isolate,
                                                               new WEBGL_draw_buffersImpl(ret));

            args.GetReturnValue().Set(buffers);

            canvas_native_webgl_extension_destroy(ext);
            return;


        }
        case WebGLExtensionType::WebGLExtensionTypeNone:
            args.GetReturnValue().SetUndefined();
            canvas_native_webgl_extension_destroy(ext);
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
            ret)) {
        auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(
                ret);

        auto texture = WebGLTexture::NewInstance(isolate, new WebGLTexture(
                value));
        args.GetReturnValue().Set(texture);

        canvas_native_webgl_framebuffer_attachment_parameter_destroy(ret);
        return;
    }
    if (canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
            ret)) {
        auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(
                ret);
        auto object = WebGLRenderbuffer::NewInstance(isolate, new WebGLRenderbuffer(
                value));


        object->Set(context,
                    ConvertToV8String(isolate, "isRenderbuffer"),
                    v8::Boolean::New(isolate,
                                     true));
        args.GetReturnValue().Set(object);

        canvas_native_webgl_framebuffer_attachment_parameter_destroy(ret);
        return;

    }

    args.GetReturnValue().Set(canvas_native_webgl_framebuffer_attachment_parameter_get_value(
            ret));

    canvas_native_webgl_framebuffer_attachment_parameter_destroy(ret);

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

        auto ret = ptr->GetParameterInternal(isolate, pname, result);

        canvas_native_webgl_WebGLResult_destroy(result);

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


                if (strlen(log) == 0) {
                    args.GetReturnValue().SetEmptyString();
                    canvas_native_string_destroy((char *) log);
                    return;
                }

                args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, (char *) log));
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
                        ret)) {
                    args.GetReturnValue().SetNull();

                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                }
                switch (pname) {
                    case GL_DELETE_STATUS:
                    case GL_LINK_STATUS:
                    case GL_VALIDATE_STATUS:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                                ret));
                        canvas_native_webgl_WebGLResult_destroy(ret);
                        return;
                    default:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                                ret));

                        canvas_native_webgl_WebGLResult_destroy(ret);
                        return;
                }
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::GetRenderbufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

                if (strlen(log) == 0) {
                    args.GetReturnValue().SetEmptyString();
                    canvas_native_string_destroy((char *) log);
                    return;
                }

                args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, (char *) log));

                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetEmptyString();
}

void WebGLRenderingContext::GetShaderParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto value = args[0];
        auto type = GetNativeType(isolate, value);
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        if (type == NativeType::WebGLShader) {
            auto shader = WebGLShader::GetPointer(value.As<v8::Object>());
            if (shader != nullptr) {
                auto ret = canvas_native_webgl_get_shader_parameter(
                        shader->GetShader(),
                        pname,
                        ptr->GetState()
                );

                if (canvas_native_webgl_result_get_is_none(
                        ret)) {
                    args.GetReturnValue().SetNull();

                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                }

                if (pname ==
                    GL_DELETE_STATUS ||
                    pname ==
                    GL_COMPILE_STATUS) {
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                            ret));

                    canvas_native_webgl_WebGLResult_destroy(ret);
                    return;
                }

                args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                        ret));

                canvas_native_webgl_WebGLResult_destroy(ret);

                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::GetShaderPrecisionFormat(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto shaderType = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto precisionType = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        auto ret = canvas_native_webgl_get_shader_precision_format(
                shaderType,
                precisionType,
                ptr->GetState()
        );
        auto shader = WebGLShaderPrecisionFormatImpl::NewInstance(isolate,
                                                                  new WebGLShaderPrecisionFormatImpl(
                                                                          ret));
        args.GetReturnValue().Set(shader);
        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();

    if (args.Length() > 0) {
        auto value = args[0];
        auto type = GetNativeType(isolate, value);
        if (type == NativeType::WebGLShader) {
            auto shader = WebGLShader::GetPointer(value.As<v8::Object>());

            if (shader != nullptr) {
                auto source = canvas_native_webgl_get_shader_source(
                        shader->GetShader(),
                        ptr->GetState()
                );

                if (strlen(source) == 0) {
                    args.GetReturnValue().SetEmptyString();
                    canvas_native_string_destroy((char *) source);
                    return;
                }

                args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, (char *) source));
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetEmptyString();
}

void
WebGLRenderingContext::GetSupportedExtensions(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    auto isolate = args.GetIsolate();
    if (ptr == nullptr) {
        args.GetReturnValue().Set(v8::Array::New(isolate));
        return;
    }

    auto context = isolate->GetCurrentContext();


    auto exts = canvas_native_webgl_get_supported_extensions(
            ptr->GetState());

    auto len = canvas_native_string_buffer_get_length(exts);

    auto array = v8::Array::New(isolate, (int) len);
    for (int i = 0; i < len; ++i) {
        auto item = canvas_native_string_buffer_get_value_at(exts, i);
        if (item != nullptr) {
            array->Set(context, i, ConvertToV8OneByteString(isolate, (char *) item));
        }

    }

    args.GetReturnValue().Set(array);

    canvas_native_string_buffer_destroy(exts);
}

void
WebGLRenderingContext::__GetSupportedExtensions(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetEmptyString();
        return;
    }

    auto isolate = args.GetIsolate();


    auto exts = canvas_native_webgl_get_supported_extensions_to_string(
            ptr->GetState());


    args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, (char *) exts));
}

void
WebGLRenderingContext::GetTexParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        auto ret = canvas_native_webgl_get_tex_parameter(
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

void
WebGLRenderingContext::GetUniformLocation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();

    if (args.Length() > 1) {
        auto programValue = args[0];
        auto type = GetNativeType(isolate, programValue);
        auto nameValue = args[1];
        if (type == NativeType::WebGLProgram && nameValue->IsString()) {
            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
            if (program != nullptr) {
                auto name = ConvertFromV8String(isolate, args[1]);

                auto ret = canvas_native_webgl_get_uniform_location(
                        program->GetProgram(),
                        name.c_str(),
                        ptr->GetState()
                );

                auto location = WebGLUniformLocation::NewInstance(isolate, new WebGLUniformLocation(
                        ret));

                args.GetReturnValue().Set(location);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::GetUniform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto programValue = args[0];
        auto programType = GetNativeType(isolate, programValue);
        auto locationValue = args[1];
        auto locationType = GetNativeType(isolate, locationValue);
        if (programType == NativeType::WebGLProgram &&
            locationType == NativeType::WebGLUniformLocation) {

            auto program = WebGLProgram::GetPointer(programValue.As<v8::Object>());
            auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());


            if (program != nullptr &&
                location != nullptr) {

                auto val = canvas_native_webgl_get_uniform(
                        program->GetProgram(),
                        location->GetUniformLocation(),
                        ptr->GetState());

                switch (canvas_native_webgl_result_get_type(
                        val)) {
                    case WebGLResultType::WebGLResultTypeBoolean:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                                val));
                        canvas_native_webgl_WebGLResult_destroy(val);
                        return;
                    case WebGLResultType::WebGLResultTypeNone:
                        args.GetReturnValue().SetNull();
                        canvas_native_webgl_WebGLResult_destroy(val);
                        return;
                    case WebGLResultType::WebGLResultTypeString: {
                        auto str = canvas_native_webgl_result_get_string(
                                val);

                        args.GetReturnValue().Set(
                                ConvertToV8OneByteString(isolate, (char *) str));
                        canvas_native_webgl_WebGLResult_destroy(val);
                        return;
                    }
                    case WebGLResultType::WebGLResultTypeBooleanArray: {
                        auto ret = canvas_native_webgl_result_get_bool_array(
                                val);
                        auto len = canvas_native_u8_buffer_get_length(ret);
                        auto buf = canvas_native_u8_buffer_get_bytes(ret);
                        auto array = v8::Array::New(
                                isolate, (int) len);
                        for (int i = 0;
                             i < len; ++i) {
                            auto item = buf[i];
                            array->Set(
                                    context, i,
                                    v8::Boolean::New(isolate,
                                                     item ==
                                                     1));
                        }
                        args.GetReturnValue().Set(array);
                        canvas_native_webgl_WebGLResult_destroy(val);
                        return;
                    }
                    case WebGLResultType::WebGLResultTypeF32Array: {
                        auto ret = canvas_native_webgl_result_get_f32_array(
                                val);


                        auto buf = (uint8_t *) canvas_native_f32_buffer_get_bytes_mut(ret);
                        auto size = canvas_native_f32_buffer_get_length(ret);
                        auto bytes_size = size * sizeof(float);


                        auto store = v8::ArrayBuffer::NewBackingStore(buf,
                                                                      bytes_size,
                                                                      [](void *data, size_t length,
                                                                         void *deleter_data) {
                                                                          if (deleter_data !=
                                                                              nullptr) {
                                                                              canvas_native_f32_buffer_destroy(
                                                                                      (F32Buffer *) deleter_data);
                                                                          }
                                                                      },
                                                                      ret);

                        auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));

                        args.GetReturnValue().Set(
                                v8::Float32Array::New(arraybuffer, 0, size)
                        );

                        canvas_native_webgl_WebGLResult_destroy(val);

                        return;
                    }
                    case WebGLResultType::WebGLResultTypeI32Array: {
                        auto ret = canvas_native_webgl_result_get_i32_array(
                                val);


                        auto buf = (uint8_t *) canvas_native_i32_buffer_get_bytes_mut(ret);
                        auto size = canvas_native_i32_buffer_get_length(ret);
                        auto bytes_size = size * sizeof(int32_t);


                        auto store = v8::ArrayBuffer::NewBackingStore(buf, bytes_size,
                                                                      [](void *data, size_t length,
                                                                         void *deleter_data) {
                                                                          if (deleter_data !=
                                                                              nullptr) {
                                                                              canvas_native_i32_buffer_destroy(
                                                                                      (I32Buffer *) deleter_data);
                                                                          }
                                                                      },
                                                                      ret);

                        auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));


                        args.GetReturnValue().Set(
                                v8::Int32Array::New(arraybuffer, 0, size)
                        );

                        canvas_native_webgl_WebGLResult_destroy(val);

                        return;
                    }
                    case WebGLResultType::WebGLResultTypeU32Array: {
                        auto ret = canvas_native_webgl_result_get_u32_array(
                                val);

                        auto buf = (uint8_t *) canvas_native_u32_buffer_get_bytes_mut(ret);
                        auto size = canvas_native_u32_buffer_get_length(ret);
                        auto bytes_size = size * sizeof(uint32_t);


                        auto store = v8::ArrayBuffer::NewBackingStore(buf, bytes_size,
                                                                      [](void *data, size_t length,
                                                                         void *deleter_data) {
                                                                          if (deleter_data !=
                                                                              nullptr) {
                                                                              canvas_native_u32_buffer_destroy(
                                                                                      (U32Buffer *) deleter_data);
                                                                          }
                                                                      },
                                                                      ret);

                        auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));


                        args.GetReturnValue().Set(
                                v8::Uint32Array::New(arraybuffer, 0, size)
                        );


                        canvas_native_webgl_WebGLResult_destroy(val);

                        return;


                    }
                    case WebGLResultType::WebGLResultTypeF32:
                        args.GetReturnValue().Set((double) canvas_native_webgl_result_get_f32(
                                val));

                        canvas_native_webgl_WebGLResult_destroy(val);
                        return;
                    case WebGLResultType::WebGLResultTypeI32:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                                val));

                        canvas_native_webgl_WebGLResult_destroy(val);
                        return;
                    case WebGLResultType::WebGLResultTypeU32:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_u32(
                                val));

                        canvas_native_webgl_WebGLResult_destroy(val);
                        return;
                }
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::GetVertexAttribOffset(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        auto ret = canvas_native_webgl_get_vertex_attrib_offset(
                index,
                pname,
                ptr->GetState());
        args.GetReturnValue().Set(static_cast<double>(ret));
        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::GetVertexAttrib(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 1) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto pname = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        auto ret = canvas_native_webgl_get_vertex_attrib(
                index,
                pname,
                ptr->GetState());

        if (pname ==
            GL_CURRENT_VERTEX_ATTRIB) {
            auto val = canvas_native_webgl_result_get_f32_array(
                    ret);


            auto buf = (uint8_t *) canvas_native_f32_buffer_get_bytes_mut(val);
            auto size = canvas_native_f32_buffer_get_length(val);
            auto bytes_size = size * sizeof(float);

            auto store = v8::ArrayBuffer::NewBackingStore(buf, bytes_size,
                                                          [](void *data, size_t length,
                                                             void *deleter_data) {
                                                              if (deleter_data != nullptr) {
                                                                  canvas_native_f32_buffer_destroy(
                                                                          (F32Buffer *) deleter_data);
                                                              }
                                                          },
                                                          val);

            auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));


            args.GetReturnValue().Set(
                    v8::Float32Array::New(arraybuffer, 0, size)
            );

            canvas_native_webgl_WebGLResult_destroy(ret);
            return;
        } else if (pname ==
                   GL_VERTEX_ATTRIB_ARRAY_ENABLED ||
                   pname ==
                   GL_VERTEX_ATTRIB_ARRAY_NORMALIZED) {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                    ret));
            canvas_native_webgl_WebGLResult_destroy(ret);
            return;
        } else {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
                    ret));
            canvas_native_webgl_WebGLResult_destroy(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::Hint(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        auto mode = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        canvas_native_webgl_hint(target,
                                 mode,
                                 ptr->GetState());
    }
}

void
WebGLRenderingContext::IsBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLBuffer) {
        auto buffer = WebGLBuffer::GetPointer(value.As<v8::Object>());
        if (buffer != nullptr) {
            auto ret = canvas_native_webgl_is_buffer(
                    buffer->GetBuffer(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void
WebGLRenderingContext::IsContextLost(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }


    auto ret = canvas_native_webgl_get_is_context_lost(
            ptr->GetState());
    args.GetReturnValue().Set(ret);
}

void
WebGLRenderingContext::IsEnabled(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto cap = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto ret = canvas_native_webgl_is_enabled(
                cap, ptr->GetState());
        args.GetReturnValue().Set(ret);
        return;
    }
    args.GetReturnValue().Set(false);
}

void
WebGLRenderingContext::IsFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLFramebuffer) {
        auto framebuffer = WebGLFramebuffer::GetPointer(value.As<v8::Object>());
        if (framebuffer != nullptr) {
            auto ret = canvas_native_webgl_is_framebuffer(
                    framebuffer->GetFrameBuffer(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void
WebGLRenderingContext::IsProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(value.As<v8::Object>());
        if (program != nullptr) {
            auto ret = canvas_native_webgl_is_program(
                    program->GetProgram(),
                    ptr->GetState());

            args.GetReturnValue().Set(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void
WebGLRenderingContext::IsRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLRenderbuffer) {
        auto renderbuffer = WebGLRenderbuffer::GetPointer(value.As<v8::Object>());
        if (renderbuffer != nullptr) {
            auto ret = canvas_native_webgl_is_renderbuffer(
                    renderbuffer->GetRenderBuffer(),
                    ptr->GetState());

            args.GetReturnValue().Set(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void
WebGLRenderingContext::IsShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLShader) {
        auto shader = WebGLShader::GetPointer(value.As<v8::Object>());
        if (shader != nullptr) {
            auto ret = canvas_native_webgl_is_shader(
                    shader->GetShader(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void
WebGLRenderingContext::IsTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLTexture) {
        auto texture = WebGLTexture::GetPointer(value.As<v8::Object>());
        if (texture != nullptr) {
            auto ret = canvas_native_webgl_is_texture(
                    texture->GetTexture(),
                    ptr->GetState());
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void
WebGLRenderingContext::LineWidth(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto width = args[0]->NumberValue(
                context).ToChecked();
        canvas_native_webgl_line_width(
                static_cast<float>(width),
                ptr->GetState());
    }

}

void
WebGLRenderingContext::LinkProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(value.As<v8::Object>());
        if (program != nullptr) {
            canvas_native_webgl_link_program(
                    program->GetProgram(),
                    ptr->GetState());
        }
    }
}

void
WebGLRenderingContext::PixelStorei(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto pname = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        if (args[1]->IsBoolean()) {
            auto param = args[1]->BooleanValue(isolate);
            canvas_native_webgl_pixel_storei(
                    pname,
                    param ? 1 : 0,
                    ptr->GetState()
            );
        } else {
            auto param = (int32_t) args[1]->NumberValue(
                    context).ToChecked();
            canvas_native_webgl_pixel_storei(
                    pname,
                    param,
                    ptr->GetState()
            );
        }

    }
}

void
WebGLRenderingContext::PolygonOffset(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto factor = args[0]->NumberValue(
                context).ToChecked();
        auto units = args[1]->NumberValue(
                context).ToChecked();
        canvas_native_webgl_polygon_offset(
                static_cast<float>(factor),
                static_cast<float>(units),
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::ReadPixels(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 6) {
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

        auto pixels = args[6];
        if (pixels->IsArrayBufferView()) {
            auto buf = pixels.As<v8::ArrayBufferView>();

            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = array->ByteLength();
            auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
            auto data = static_cast<uint8_t *>((void *) data_ptr);


            canvas_native_webgl_read_pixels_u8(
                    x,
                    y,
                    width,
                    height,
                    format,
                    type,
                    data, size,
                    ptr->GetState()
            );
            return;
        }


        if (pixels->IsArrayBuffer()) {
            auto array = pixels.As<v8::ArrayBuffer>();

            auto size = array->ByteLength();
            auto data = (uint8_t *) array->GetBackingStore()->Data();

            canvas_native_webgl_read_pixels_u8(
                    x,
                    y,
                    width,
                    height,
                    format,
                    type,
                    data, size,
                    ptr->GetState()
            );
        }
    }

}

void
WebGLRenderingContext::RenderbufferStorage(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::SampleCoverage(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto value = args[0]->NumberValue(
                context).ToChecked();
        auto invert = args[1]->BooleanValue(isolate);
        canvas_native_webgl_sample_coverage(
                static_cast<float>(value),
                invert,
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::Scissor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::ShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    auto sourceValue = args[1];
    if (type == NativeType::WebGLShader && sourceValue->IsString()) {
        auto shader = WebGLShader::GetPointer(value.As<v8::Object>());
        auto source = ConvertFromV8String(isolate, sourceValue);
        if (shader != nullptr) {
            canvas_native_webgl_shader_source(
                    shader->GetShader(),
                    source.c_str(),
                    ptr->GetState());
        }
    }

}

void
WebGLRenderingContext::StencilFuncSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::StencilFunc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::StencilMaskSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto face = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto mask = (uint32_t) args[1]->NumberValue(
                context).ToChecked();
        canvas_native_webgl_stencil_mask_separate(
                face,
                mask,
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::StencilMask(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 0) {
        auto mask = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        canvas_native_webgl_stencil_mask(
                mask,
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::StencilOpSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() > 3) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::StencilOp(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::TexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();


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
                ptr->GetState()
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

        auto pixels = args[5];
        auto pixelsType = GetNativeType(isolate, pixels);

        switch (pixelsType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(pixels.As<v8::Object>());

                if (image_asset !=
                    nullptr) {


                    canvas_native_webgl_tex_image2d_image_asset(
                            target,
                            level,
                            internalformat,
                            format,
                            type,
                            image_asset->GetImageAsset(),
                            ptr->GetState()
                    );
                }
                return;
            }
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(pixels.As<v8::Object>());

                if (image_bitmap !=
                    nullptr) {
                    canvas_native_webgl_tex_image2d_image_asset(
                            target,
                            level,
                            internalformat,
                            format,
                            type,
                            image_bitmap->GetImageAsset(),
                            ptr->GetState()
                    );
                }
                return;
            }
            case NativeType::CanvasRenderingContext2D: {
                auto canvas_2d = CanvasRenderingContext2DImpl::GetPointer(
                        pixels.As<v8::Object>());

                if (canvas_2d != nullptr) {
                    canvas_native_webgl_tex_image2d_canvas2d(
                            target,
                            level,
                            internalformat,
                            format,
                            type,
                            canvas_2d->GetContext(),
                            ptr->GetState()
                    );
                }

                return;
            }
            case NativeType::WebGLRenderingContextBase: {
                auto gl = WebGLRenderingContext::GetPointer(pixels.As<v8::Object>());

                if (gl != nullptr) {
                    canvas_native_webgl_tex_image2d_webgl(
                            target,
                            level,
                            internalformat,
                            format,
                            type,
                            gl->GetState(),
                            ptr->GetState()
                    );
                }
                return;
            }
            default:
                break;
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
                ptr->GetState()
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


        auto value = args[8];
        if (value->IsNullOrUndefined()) {
            canvas_native_webgl_tex_image2d_none(
                    target,
                    level,
                    internalformat,
                    width,
                    height,
                    border,
                    format,
                    type,
                    ptr->GetState()
            );
        } else if (value->IsArrayBufferView()) {
            auto buf = value.As<v8::ArrayBufferView>();

            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = array->ByteLength();
            auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
            auto data = static_cast<uint8_t *>((void *) data_ptr);


            canvas_native_webgl_tex_image2d(
                    target,
                    level,
                    internalformat,
                    width,
                    height,
                    border,
                    format,
                    type,
                    data, size,
                    ptr->GetState()
            );
            return;
        } else if (value->IsArrayBuffer()) {
            auto array = value.As<v8::ArrayBuffer>();

            auto size = array->ByteLength();
            auto data = (uint8_t *) array->GetBackingStore()->Data();


            canvas_native_webgl_tex_image2d(
                    target,
                    level,
                    internalformat,
                    width,
                    height,
                    border,
                    format,
                    type,
                    data, size,
                    ptr->GetState()
            );
            return;
        }

    }
}

void
WebGLRenderingContext::TexParameterf(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::TexParameteri(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
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
                ptr->GetState()
        );
    }

}

void
WebGLRenderingContext::TexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();
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


        auto pixels = args[6];
        auto pixelsType = GetNativeType(isolate, pixels);

        switch (pixelsType) {
            case NativeType::ImageAsset: {
                auto asset = ImageAssetImpl::GetPointer(pixels.As<v8::Object>());
                if (asset != nullptr) {
                    canvas_native_webgl_tex_sub_image2d_asset(
                            target,
                            level,
                            xoffset,
                            yoffset,
                            format,
                            type,
                            asset->GetImageAsset(),
                            ptr->GetState()
                    );
                }
                return;
            }
            case NativeType::ImageBitmap: {
                auto bitmap = ImageBitmapImpl::GetPointer(pixels.As<v8::Object>());

                if (bitmap != nullptr) {
                    canvas_native_webgl_tex_sub_image2d_asset(
                            target,
                            level,
                            xoffset,
                            yoffset,
                            format,
                            type,
                            bitmap->GetImageAsset(),
                            ptr->GetState()
                    );
                }
                return;
            }
            case NativeType::CanvasRenderingContext2D: {
                auto canvas2d = CanvasRenderingContext2DImpl::GetPointer(pixels.As<v8::Object>());

                if (canvas2d != nullptr) {
                    canvas_native_webgl_tex_sub_image2d_canvas2d(
                            target,
                            level,
                            xoffset,
                            yoffset,
                            format,
                            type,
                            canvas2d->GetContext(),
                            ptr->GetState()
                    );
                }
                return;
            }
            case NativeType::WebGLRenderingContextBase: {

                auto webgl = WebGLRenderingContext::GetPointer(pixels.As<v8::Object>());

                if (webgl != nullptr) {
                    canvas_native_webgl_tex_sub_image2d_webgl(
                            target,
                            level,
                            xoffset,
                            yoffset,
                            format,
                            type,
                            webgl->GetState(),
                            ptr->GetState()
                    );
                }
                return;
            }
            default:
                break;
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

        auto pixels = args[8];


        if (pixels->IsArrayBufferView()) {
            auto buf = pixels.As<v8::ArrayBufferView>();

            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = array->ByteLength();
            auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
            auto data = static_cast<uint8_t *>((void *) data_ptr);


            canvas_native_webgl_tex_sub_image2d(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    type,
                    data, size,
                    ptr->GetState()
            );
        } else if (pixels->IsArrayBuffer()) {
            auto array = pixels.As<v8::ArrayBuffer>();

            auto size = array->ByteLength();
            auto data = (uint8_t *) array->GetBackingStore()->Data();


            canvas_native_webgl_tex_sub_image2d(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    type,
                    data, size,
                    ptr->GetState()
            );
        }
    }

}


void
WebGLRenderingContext::VertexAttrib1f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto v0 = static_cast<float>(args[1]->NumberValue(
                context).ToChecked());
        canvas_native_webgl_vertex_attrib1f(
                index, v0, ptr->GetState());
    }

}

void
WebGLRenderingContext::VertexAttrib2f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 2) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto v0 = static_cast<float>(args[1]->NumberValue(
                context).ToChecked());
        auto v1 = static_cast<float>(args[2]->NumberValue(
                context).ToChecked());
        canvas_native_webgl_vertex_attrib2f(
                index, v0, v1,
                ptr->GetState());
    }

}

void
WebGLRenderingContext::VertexAttrib3f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
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
                ptr->GetState());
    }

}

void
WebGLRenderingContext::VertexAttrib4f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 4) {
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
                ptr->GetState());
    }

}

void
WebGLRenderingContext::VertexAttrib1fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();

        auto value = args[1];
        if (value->IsFloat32Array()) {
            auto buf = value.As<v8::Float32Array>();

            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
            auto data = static_cast<float *>((void *) data_ptr);


            canvas_native_webgl_vertex_attrib1fv(
                    index, data, size,
                    ptr->GetState());
        }
    }
}

void
WebGLRenderingContext::VertexAttrib2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();

        auto value = args[1];
        if (value->IsFloat32Array()) {
            auto buf = value.As<v8::Float32Array>();

            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
            auto data = static_cast<float *>((void *) data_ptr);


            canvas_native_webgl_vertex_attrib2fv(
                    index, data, size,
                    ptr->GetState());
        }
    }
}

void
WebGLRenderingContext::VertexAttrib3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();

        auto value = args[1];
        if (value->IsFloat32Array()) {
            auto buf = value.As<v8::Float32Array>();

            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
            auto data = static_cast<float *>((void *) data_ptr);


            canvas_native_webgl_vertex_attrib3fv(
                    index, data, size,
                    ptr->GetState());
        }
    }
}

void
WebGLRenderingContext::VertexAttrib4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();

        auto value = args[1];
        if (value->IsFloat32Array()) {
            auto buf = value.As<v8::Float32Array>();

            auto array = buf->Buffer();
            auto offset = buf->ByteOffset();
            auto size = buf->Length();
            auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
            auto data = static_cast<float *>((void *) data_ptr);


            canvas_native_webgl_vertex_attrib4fv(
                    index, data, size,
                    ptr->GetState());
        }
    }
}

void
WebGLRenderingContext::VertexAttribPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 5) {
        auto index = (uint32_t) args[0]->NumberValue(
                context).ToChecked();
        auto size = (int32_t) args[1]->NumberValue(
                context).ToChecked();
        auto type = (uint32_t) args[2]->NumberValue(
                context).ToChecked();
        auto normalized = args[3]->BooleanValue(isolate);
        auto stride = (int32_t) args[4]->NumberValue(
                context).ToChecked();
        auto offset = static_cast<ssize_t>(args[5]->NumberValue(
                context).ToChecked());
        canvas_native_webgl_vertex_attrib_pointer(
                index, size, type, normalized,
                stride, offset,
                ptr->GetState());
    }
}

void
WebGLRenderingContext::Uniform1f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
    auto v0 = args[1]->NumberValue(
            context).ToChecked();
    if (location != nullptr) {
        canvas_native_webgl_uniform1f(
                location->GetUniformLocation(),
                static_cast<float>(v0),
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform2f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
    auto v0 = args[1]->NumberValue(
            context).ToChecked();
    auto v1 = args[2]->NumberValue(
            context).ToChecked();
    if (location != nullptr) {
        canvas_native_webgl_uniform2f(
                location->GetUniformLocation(),
                static_cast<float>(v0),
                static_cast<float>(v1),
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform3f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
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
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform4f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
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
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform1fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsFloat32Array()) {
                auto buf = v0Value.As<v8::Float32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<float *>((void *) data_ptr);


                canvas_native_webgl_uniform1fv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (!item->IsNumber()) {
                        buf.push_back(
                                nanf(""));
                    } else {
                        auto value = item->NumberValue(
                                context).ToChecked();
                        buf.push_back(
                                static_cast<float>(value));
                    }
                }

                canvas_native_webgl_uniform1fv(
                        location->GetUniformLocation(),
                        buf.data(), buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::Uniform2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsFloat32Array()) {
                auto buf = v0Value.As<v8::Float32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<float *>((void *) data_ptr);


                canvas_native_webgl_uniform2fv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (!item->IsNumber()) {
                        buf.push_back(
                                nanf(""));
                    } else {
                        auto value = item->NumberValue(
                                context).ToChecked();
                        buf.push_back(
                                static_cast<float>(value));
                    }
                }

                canvas_native_webgl_uniform2fv(
                        location->GetUniformLocation(),
                        buf.data(), buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::Uniform3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsFloat32Array()) {
                auto buf = v0Value.As<v8::Float32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<float *>((void *) data_ptr);


                canvas_native_webgl_uniform3fv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (!item->IsNumber()) {
                        buf.push_back(
                                nanf(""));
                    } else {
                        auto value = item->NumberValue(
                                context).ToChecked();
                        buf.push_back(
                                static_cast<float>(value));
                    }
                }
                canvas_native_webgl_uniform3fv(
                        location->GetUniformLocation(),
                        buf.data(), buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::Uniform4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsFloat32Array()) {
                auto buf = v0Value.As<v8::Float32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<float *>((void *) data_ptr);


                canvas_native_webgl_uniform4fv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();
                    if (!item->IsNumber()) {
                        buf.push_back(
                                nanf(""));
                    } else {
                        auto value = item->NumberValue(
                                context).ToChecked();
                        buf.push_back(
                                static_cast<float>(value));
                    }
                }
                canvas_native_webgl_uniform4fv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::Uniform1i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
    auto v0 = args[1]->NumberValue(
            context).ToChecked();
    if (location != nullptr) {
        canvas_native_webgl_uniform1i(
                location->GetUniformLocation(),
                static_cast<int32_t>(v0),
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform2i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
    auto v0 = args[1]->NumberValue(
            context).ToChecked();
    auto v1 = args[2]->NumberValue(
            context).ToChecked();
    if (location != nullptr) {
        canvas_native_webgl_uniform2i(
                location->GetUniformLocation(),
                static_cast<int32_t>(v0),
                static_cast<int32_t>(v1),
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform3i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
    auto v0 = args[1]->NumberValue(
            context).ToChecked();
    auto v1 = args[2]->NumberValue(
            context).ToChecked();
    auto v2 = args[3]->NumberValue(
            context).ToChecked();
    if (location != nullptr) {
        canvas_native_webgl_uniform3i(
                location->GetUniformLocation(),
                static_cast<int32_t>(v0),
                static_cast<int32_t>(v1),
                static_cast<int32_t>(v2),
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform4i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    //if (count > 1) {
    //   if (args[0].isObject()) {
    auto location = WebGLUniformLocation::GetPointer(value.As<v8::Object>());
    auto v0 = args[1]->NumberValue(
            context).ToChecked();
    auto v1 = args[2]->NumberValue(
            context).ToChecked();
    auto v2 = args[3]->NumberValue(
            context).ToChecked();
    auto v3 = args[4]->NumberValue(
            context).ToChecked();
    if (location != nullptr) {
        canvas_native_webgl_uniform4i(
                location->GetUniformLocation(),
                static_cast<int32_t>(v0),
                static_cast<int32_t>(v1),
                static_cast<int32_t>(v2),
                static_cast<int32_t>(v3),
                ptr->GetState()
        );
    }
    //   }
    // }
}

void
WebGLRenderingContext::Uniform1iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsInt32Array()) {
                auto buf = v0Value.As<v8::Int32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<int32_t *>((void *) data_ptr);


                canvas_native_webgl_uniform1iv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<int32_t> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();

                    auto value = item->NumberValue(
                            context).ToChecked();
                    buf.push_back(
                            static_cast<int32_t>(value));
                }
                canvas_native_webgl_uniform1iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::Uniform2iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsInt32Array()) {
                auto buf = v0Value.As<v8::Int32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<int32_t *>((void *) data_ptr);

                canvas_native_webgl_uniform2iv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<int32_t> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();

                    auto value = item->NumberValue(
                            context).ToChecked();
                    buf.push_back(
                            static_cast<int32_t>(value));
                }
                canvas_native_webgl_uniform2iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::Uniform3iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsInt32Array()) {
                auto buf = v0Value.As<v8::Int32Array>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<int32_t *>((void *) data_ptr);


                canvas_native_webgl_uniform3iv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<int32_t> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();

                    auto value = item->NumberValue(
                            context).ToChecked();
                    buf.push_back(
                            static_cast<int32_t>(value));
                }

                canvas_native_webgl_uniform3iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::Uniform4iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto type = GetNativeType(isolate, locationValue);
    auto v0Value = args[1];


    if (type == NativeType::WebGLUniformLocation) {
        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());

        if (location != nullptr) {
            if (v0Value->IsInt32Array()) {
                auto buf = v0Value.As<v8::Int32Array>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<int32_t *>((void *) data_ptr);


                canvas_native_webgl_uniform4iv(
                        location->GetUniformLocation(),
                        data, size,
                        ptr->GetState());
            } else if (v0Value->IsArray()) {
                auto array = v0Value.As<v8::Array>();
                auto len = array->Length();
                std::vector<int32_t> buf;
                buf.reserve(len);
                for (int i = 0;
                     i < len; ++i) {
                    auto item = array->Get(
                            context, i).ToLocalChecked();

                    auto value = item->NumberValue(
                            context).ToChecked();
                    buf.push_back(
                            static_cast<int32_t>(value));
                }
                canvas_native_webgl_uniform4iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::UniformMatrix2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto locationType = GetNativeType(isolate, locationValue);
    auto value = args[2];
    if (locationType == NativeType::WebGLUniformLocation) {

        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto transpose = args[1]->BooleanValue(isolate);

        if (location != nullptr) {
            if (value->IsFloat32Array()) {
                auto buf = value.As<v8::Float32Array>();
                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<float *>((void *) data_ptr);

                canvas_native_webgl_uniform_matrix2fv(
                        location->GetUniformLocation(),
                        transpose, data, size,
                        ptr->GetState());
            } else if (value->IsArray()) {
                auto array = value.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                buf.reserve(len);

                for (int i = 0;
                     i < len; i++) {
                    auto item = array->Get(context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(
                                        ""));
                    }
                }

                canvas_native_webgl_uniform_matrix2fv(
                        location->GetUniformLocation(),
                        transpose, buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::UniformMatrix3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto locationType = GetNativeType(isolate, locationValue);
    auto value = args[2];
    if (locationType == NativeType::WebGLUniformLocation) {

        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto transpose = args[1]->BooleanValue(isolate);

        if (location != nullptr) {
            if (value->IsFloat32Array()) {
                auto buf = value.As<v8::Float32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<float *>((void *) data_ptr);

                canvas_native_webgl_uniform_matrix3fv(
                        location->GetUniformLocation(),
                        transpose, data, size,
                        ptr->GetState());
            } else if (value->IsArray()) {
                auto array = value.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                buf.reserve(len);

                for (int i = 0;
                     i < len; i++) {
                    auto item = array->Get(context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(
                                        ""));
                    }
                }


                canvas_native_webgl_uniform_matrix3fv(
                        location->GetUniformLocation(),
                        transpose, buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::UniformMatrix4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto locationValue = args[0];
    auto locationType = GetNativeType(isolate, locationValue);
    auto value = args[2];
    if (locationType == NativeType::WebGLUniformLocation) {

        auto location = WebGLUniformLocation::GetPointer(locationValue.As<v8::Object>());
        auto transpose = args[1]->BooleanValue(isolate);

        if (location != nullptr) {
            if (value->IsFloat32Array()) {
                auto buf = value.As<v8::Float32Array>();

                auto array = buf->Buffer();
                auto offset = buf->ByteOffset();
                auto size = buf->Length();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;
                auto data = static_cast<float *>((void *) data_ptr);


                canvas_native_webgl_uniform_matrix4fv(
                        location->GetUniformLocation(),
                        transpose, data, size,
                        ptr->GetState());
            } else if (value->IsArray()) {
                auto array = value.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                buf.reserve(len);

                for (int i = 0;
                     i < len; i++) {
                    auto item = array->Get(context, i).ToLocalChecked();
                    if (item->IsNumber()) {
                        buf.push_back(
                                static_cast<float>(item->NumberValue(
                                        context).ToChecked()));
                    } else {
                        buf.push_back(
                                std::nanf(
                                        ""));
                    }
                }


                canvas_native_webgl_uniform_matrix4fv(
                        location->GetUniformLocation(),
                        transpose, buf.data(),
                        buf.size(),
                        ptr->GetState());
            }
        }
    }
}

void
WebGLRenderingContext::UseProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    if (args.Length() > 0) {
        auto value = args[0];
        if (value->IsNull()) {
            canvas_native_webgl_use_program(0,
                                            ptr->GetState());

            return;
        }

        auto type = GetNativeType(isolate, value);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(value.As<v8::Object>());
            if (program != nullptr) {
                canvas_native_webgl_use_program(
                        program->GetProgram(),
                        ptr->GetState()
                );
            }
        }
    }

}

void
WebGLRenderingContext::ValidateProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);
    if (type == NativeType::WebGLProgram) {
        auto program = WebGLProgram::GetPointer(value.As<v8::Object>());
        if (program != nullptr) {
            canvas_native_webgl_validate_program(
                    program->GetProgram(),
                    ptr->GetState()
            );
        }
    }

}

void
WebGLRenderingContext::Viewport(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 3) {
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
                                     ptr->GetState());
    }

}

void
WebGLRenderingContext::__ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args) {
    WebGLRenderingContext *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetEmptyString();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    std::string type("image/png");
    int quality = 92;
    if (args[0]->IsString()) {
        type = ConvertFromV8String(isolate, args[0]);
    }


    if (args[1]->IsNumber()) {
        quality = (int) args[1]->NumberValue(
                context).ToChecked();
    }


    auto data = canvas_native_webgl_to_data_url(
            ptr->GetState(),
            type.c_str(),
            quality);
    args.GetReturnValue().Set(ConvertToV8OneByteString(isolate, (char *) data));

}

void WebGLRenderingContext::SetConstants(v8::Isolate *isolate,
                                         const v8::Local<v8::ObjectTemplate> &tmpl) {
    tmpl->Set(isolate, "DEPTH_BUFFER_BIT", v8::Uint32::New(isolate, 0x00000100));

    tmpl->Set(isolate, "STENCIL_BUFFER_BIT", v8::Uint32::New(isolate, 0x00000400));

    tmpl->Set(isolate, "COLOR_BUFFER_BIT", v8::Uint32::New(isolate, 0x00004000));

    tmpl->Set(isolate, "POINTS", v8::Uint32::New(isolate, 0x0000));

    tmpl->Set(isolate, "LINES", v8::Uint32::New(isolate, 0x0001));

    tmpl->Set(isolate, "LINE_LOOP", v8::Uint32::New(isolate, 0x0002));

    tmpl->Set(isolate, "LINE_STRIP", v8::Uint32::New(isolate, 0x0003));

    tmpl->Set(isolate, "TRIANGLES", v8::Uint32::New(isolate, 0x0004));

    tmpl->Set(isolate, "TRIANGLE_STRIP", v8::Uint32::New(isolate, 0x0005));

    tmpl->Set(isolate, "TRIANGLE_FAN", v8::Uint32::New(isolate, 0x0006));

    tmpl->Set(isolate, "ZERO", v8::Uint32::New(isolate, 0));

    tmpl->Set(isolate, "ONE", v8::Uint32::New(isolate, 1));

    tmpl->Set(isolate, "SRC_COLOR", v8::Uint32::New(isolate, 0x0300));

    tmpl->Set(isolate, "ONE_MINUS_SRC_COLOR", v8::Uint32::New(isolate, 0x0301));

    tmpl->Set(isolate, "SRC_ALPHA", v8::Uint32::New(isolate, 0x0302));

    tmpl->Set(isolate, "ONE_MINUS_SRC_ALPHA", v8::Uint32::New(isolate, 0x0303));

    tmpl->Set(isolate, "DST_ALPHA", v8::Uint32::New(isolate, 0x0304));

    tmpl->Set(isolate, "ONE_MINUS_DST_ALPHA", v8::Uint32::New(isolate, 0x0305));

    tmpl->Set(isolate, "DST_COLOR", v8::Uint32::New(isolate, 0x0306));

    tmpl->Set(isolate, "ONE_MINUS_DST_COLOR", v8::Uint32::New(isolate, 0x0307));

    tmpl->Set(isolate, "SRC_ALPHA_SATURATE", v8::Uint32::New(isolate, 0x0308));


    tmpl->Set(isolate, "CONSTANT_COLOR", v8::Uint32::New(isolate, 0x8001));
    tmpl->Set(isolate, "ONE_MINUS_CONSTANT_COLOR", v8::Uint32::New(isolate, 0x8002));
    tmpl->Set(isolate, "CONSTANT_ALPHA", v8::Uint32::New(isolate, 0x8003));
    tmpl->Set(isolate, "ONE_MINUS_CONSTANT_ALPHA", v8::Uint32::New(isolate, 0x8004));


    /* Blending equations */

    tmpl->Set(isolate, "FUNC_ADD", v8::Uint32::New(isolate, 0x8006));
    tmpl->Set(isolate, "FUNC_SUBTRACT", v8::Uint32::New(isolate, 0x800A));
    tmpl->Set(isolate, "FUNC_REVERSE_SUBTRACT", v8::Uint32::New(isolate, 0x800B));
    tmpl->Set(isolate, "BLEND_EQUATION", v8::Uint32::New(isolate, 0x8009));
    tmpl->Set(isolate, "BLEND_EQUATION_RGB", v8::Uint32::New(isolate, 0x8009));
    tmpl->Set(isolate, "BLEND_EQUATION_ALPHA", v8::Uint32::New(isolate, 0x883D));


    tmpl->Set(isolate, "BLEND_DST_RGB", v8::Uint32::New(isolate, 0x80C8));
    tmpl->Set(isolate, "BLEND_SRC_RGB", v8::Uint32::New(isolate, 0x80C9));
    tmpl->Set(isolate, "BLEND_DST_ALPHA", v8::Uint32::New(isolate, 0x80CA));


    tmpl->Set(isolate, "BLEND_SRC_ALPHA", v8::Uint32::New(isolate, 0x80CB));
    tmpl->Set(isolate, "BLEND_COLOR", v8::Uint32::New(isolate, 0x8005));
    tmpl->Set(isolate, "ARRAY_BUFFER_BINDING", v8::Uint32::New(isolate, 0x8894));


    tmpl->Set(isolate, "ELEMENT_ARRAY_BUFFER_BINDING", v8::Uint32::New(isolate, 0x8895));
    tmpl->Set(isolate, "LINE_WIDTH", v8::Uint32::New(isolate, 0x0B21));
    tmpl->Set(isolate, "ALIASED_POINT_SIZE_RANGE", v8::Uint32::New(isolate, 0x846D));

    tmpl->Set(isolate, "ALIASED_LINE_WIDTH_RANGE", v8::Uint32::New(isolate, 0x846E));
    tmpl->Set(isolate, "CULL_FACE_MODE", v8::Uint32::New(isolate, 0x0B45));
    tmpl->Set(isolate, "FRONT_FACE", v8::Uint32::New(isolate, 0x0B46));

    tmpl->Set(isolate, "DEPTH_RANGE", v8::Uint32::New(isolate, 0x0B70));
    tmpl->Set(isolate, "DEPTH_WRITEMASK", v8::Uint32::New(isolate, 0x0B72));
    tmpl->Set(isolate, "DEPTH_CLEAR_VALUE", v8::Uint32::New(isolate, 0x0B73));

    tmpl->Set(isolate, "DEPTH_FUNC", v8::Uint32::New(isolate, 0x0B74));
    tmpl->Set(isolate, "STENCIL_CLEAR_VALUE", v8::Uint32::New(isolate, 0x0B91));
    tmpl->Set(isolate, "STENCIL_FUNC", v8::Uint32::New(isolate, 0x0B92));

    tmpl->Set(isolate, "STENCIL_FAIL", v8::Uint32::New(isolate, 0x0B94));
    tmpl->Set(isolate, "STENCIL_PASS_DEPTH_FAIL", v8::Uint32::New(isolate, 0x0B95));
    tmpl->Set(isolate, "STENCIL_PASS_DEPTH_PASS", v8::Uint32::New(isolate, 0x0B96));

    tmpl->Set(isolate, "STENCIL_REF", v8::Uint32::New(isolate, 0x0B97));
    tmpl->Set(isolate, "STENCIL_VALUE_MASK", v8::Uint32::New(isolate, 0x0B93));
    tmpl->Set(isolate, "STENCIL_WRITEMASK", v8::Uint32::New(isolate, 0x0B98));

    tmpl->Set(isolate, "STENCIL_BACK_FUNC", v8::Uint32::New(isolate, 0x8800));
    tmpl->Set(isolate, "STENCIL_BACK_FAIL", v8::Uint32::New(isolate, 0x8801));
    tmpl->Set(isolate, "STENCIL_BACK_PASS_DEPTH_FAIL", v8::Uint32::New(isolate, 0x8802));

    tmpl->Set(isolate, "STENCIL_BACK_PASS_DEPTH_PASS", v8::Uint32::New(isolate, 0x8803));
    tmpl->Set(isolate, "STENCIL_BACK_REF", v8::Uint32::New(isolate, 0x8CA3));
    tmpl->Set(isolate, "STENCIL_BACK_VALUE_MASK", v8::Uint32::New(isolate, 0x8CA4));
    tmpl->Set(isolate, "STENCIL_BACK_WRITEMASK", v8::Uint32::New(isolate, 0x8CA5));

    tmpl->Set(isolate, "VIEWPORT", v8::Uint32::New(isolate, 0x0BA2));
    tmpl->Set(isolate, "SCISSOR_BOX", v8::Uint32::New(isolate, 0x0C10));
    tmpl->Set(isolate, "COLOR_CLEAR_VALUE", v8::Uint32::New(isolate, 0x0C22));
    tmpl->Set(isolate, "COLOR_WRITEMASK", v8::Uint32::New(isolate, 0x0C23));

    tmpl->Set(isolate, "UNPACK_ALIGNMENT", v8::Uint32::New(isolate, 0x0CF5));
    tmpl->Set(isolate, "PACK_ALIGNMENT", v8::Uint32::New(isolate, 0x0D05));
    tmpl->Set(isolate, "MAX_TEXTURE_SIZE", v8::Uint32::New(isolate, 0x0D33));


    tmpl->Set(isolate, "MAX_VIEWPORT_DIMS", v8::Uint32::New(isolate, 0x0D3A));
    tmpl->Set(isolate, "SUBPIXEL_BITS", v8::Uint32::New(isolate, 0x0D50));

    tmpl->Set(isolate, "RED_BITS", v8::Uint32::New(isolate, 0x0D52));
    tmpl->Set(isolate, "GREEN_BITS", v8::Uint32::New(isolate, 0x0D53));
    tmpl->Set(isolate, "BLUE_BITS", v8::Uint32::New(isolate, 0x0D54));
    tmpl->Set(isolate, "ALPHA_BITS", v8::Uint32::New(isolate, 0x0D55));

    tmpl->Set(isolate, "STENCIL_BITS", v8::Uint32::New(isolate, 0x0D57));
    tmpl->Set(isolate, "POLYGON_OFFSET_UNITS", v8::Uint32::New(isolate, 0x2A00));
    tmpl->Set(isolate, "POLYGON_OFFSET_FACTOR", v8::Uint32::New(isolate, 0x8038));

    tmpl->Set(isolate, "TEXTURE_BINDING_2D", v8::Uint32::New(isolate, 0x8069));
    tmpl->Set(isolate, "SAMPLE_BUFFERS", v8::Uint32::New(isolate, 0x80A8));
    tmpl->Set(isolate, "SAMPLES", v8::Uint32::New(isolate, 0x80A9));
    tmpl->Set(isolate, "SAMPLE_COVERAGE_VALUE", v8::Uint32::New(isolate, 0x80AA));

    tmpl->Set(isolate, "SAMPLE_COVERAGE_INVERT", v8::Uint32::New(isolate, 0x80AB));
    tmpl->Set(isolate, "COMPRESSED_TEXTURE_FORMATS", v8::Uint32::New(isolate, 0x86A3));
    tmpl->Set(isolate, "VENDOR", v8::Uint32::New(isolate, 0x1F00));
    tmpl->Set(isolate, "RENDERER", v8::Uint32::New(isolate, 0x1F01));

    tmpl->Set(isolate, "VERSION", v8::Uint32::New(isolate, 0x1F02));
    tmpl->Set(isolate, "IMPLEMENTATION_COLOR_READ_TYPE", v8::Uint32::New(isolate, 0x8B9A));
    tmpl->Set(isolate, "IMPLEMENTATION_COLOR_READ_FORMAT", v8::Uint32::New(isolate, 0x8B9B));
    tmpl->Set(isolate, "BROWSER_DEFAULT_WEBGL", v8::Uint32::New(isolate, 0x9244));

    tmpl->Set(isolate, "STATIC_DRAW", v8::Uint32::New(isolate, 0x88E4));
    tmpl->Set(isolate, "STREAM_DRAW", v8::Uint32::New(isolate, 0x88E0));
    tmpl->Set(isolate, "DYNAMIC_DRAW", v8::Uint32::New(isolate, 0x88E8));
    tmpl->Set(isolate, "ARRAY_BUFFER", v8::Uint32::New(isolate, 0x8892));

    tmpl->Set(isolate, "ELEMENT_ARRAY_BUFFER", v8::Uint32::New(isolate, 0x8893));
    tmpl->Set(isolate, "BUFFER_SIZE", v8::Uint32::New(isolate, 0x8764));
    tmpl->Set(isolate, "BUFFER_USAGE", v8::Uint32::New(isolate, 0x8765));
    tmpl->Set(isolate, "CURRENT_VERTEX_ATTRIB", v8::Uint32::New(isolate, 0x8626));


    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_ENABLED", v8::Uint32::New(isolate, 0x8622));
    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_SIZE", v8::Uint32::New(isolate, 0x8623));
    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_STRIDE", v8::Uint32::New(isolate, 0x8624));
    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_TYPE", v8::Uint32::New(isolate, 0x8625));
    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_NORMALIZED", v8::Uint32::New(isolate, 0x886A));
    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_POINTER", v8::Uint32::New(isolate, 0x8645));
    tmpl->Set(isolate, "VERTEX_ATTRIB_ARRAY_BUFFER_BINDING", v8::Uint32::New(isolate, 0x889F));

    tmpl->Set(isolate, "CULL_FACE", v8::Uint32::New(isolate, 0x0B44));
    tmpl->Set(isolate, "FRONT", v8::Uint32::New(isolate, 0x0404));
    tmpl->Set(isolate, "BACK", v8::Uint32::New(isolate, 0x0405));
    tmpl->Set(isolate, "FRONT_AND_BACK", v8::Uint32::New(isolate, 0x0408));

    tmpl->Set(isolate, "BLEND", v8::Uint32::New(isolate, 0x0BE2));
    tmpl->Set(isolate, "DEPTH_TEST", v8::Uint32::New(isolate, 0x0B71));
    tmpl->Set(isolate, "DITHER", v8::Uint32::New(isolate, 0x0BD0));
    tmpl->Set(isolate, "POLYGON_OFFSET_FILL", v8::Uint32::New(isolate, 0x8037));

    tmpl->Set(isolate, "SAMPLE_ALPHA_TO_COVERAGE", v8::Uint32::New(isolate, 0x809E));
    tmpl->Set(isolate, "SAMPLE_COVERAGE", v8::Uint32::New(isolate, 0x80A0));
    tmpl->Set(isolate, "SCISSOR_TEST", v8::Uint32::New(isolate, 0x0C11));
    tmpl->Set(isolate, "STENCIL_TEST", v8::Uint32::New(isolate, 0x0B90));


    /* Errors */

    tmpl->Set(isolate, "NO_ERROR", v8::Uint32::New(isolate, 0));
    tmpl->Set(isolate, "INVALID_ENUM", v8::Uint32::New(isolate, 0x0500));
    tmpl->Set(isolate, "INVALID_VALUE", v8::Uint32::New(isolate, 0x0501));
    tmpl->Set(isolate, "INVALID_OPERATION", v8::Uint32::New(isolate, 0x0502));

    tmpl->Set(isolate, "OUT_OF_MEMORY", v8::Uint32::New(isolate, 0x0505));
    tmpl->Set(isolate, "CONTEXT_LOST_WEBGL", v8::Uint32::New(isolate, 0x9242));
    tmpl->Set(isolate, "CW", v8::Uint32::New(isolate, 0x0900));
    tmpl->Set(isolate, "CCW", v8::Uint32::New(isolate, 0x0901));

    tmpl->Set(isolate, "DONT_CARE", v8::Uint32::New(isolate, 0x1100));
    tmpl->Set(isolate, "FASTEST", v8::Uint32::New(isolate, 0x1101));
    tmpl->Set(isolate, "NICEST", v8::Uint32::New(isolate, 0x1102));
    tmpl->Set(isolate, "GENERATE_MIPMAP_HINT", v8::Uint32::New(isolate, 0x8192));

    tmpl->Set(isolate, "BYTE", v8::Uint32::New(isolate, 0x1400));
    tmpl->Set(isolate, "UNSIGNED_BYTE", v8::Uint32::New(isolate, 0x1401));
    tmpl->Set(isolate, "SHORT", v8::Uint32::New(isolate, 0x1402));
    tmpl->Set(isolate, "UNSIGNED_SHORT", v8::Uint32::New(isolate, 0x1403));

    tmpl->Set(isolate, "BYTE", v8::Uint32::New(isolate, 0x1400));
    tmpl->Set(isolate, "UNSIGNED_BYTE", v8::Uint32::New(isolate, 0x1401));
    tmpl->Set(isolate, "SHORT", v8::Uint32::New(isolate, 0x1402));
    tmpl->Set(isolate, "UNSIGNED_SHORT", v8::Uint32::New(isolate, 0x1403));

    tmpl->Set(isolate, "INT", v8::Uint32::New(isolate, 0x1404));
    tmpl->Set(isolate, "UNSIGNED_INT", v8::Uint32::New(isolate, 0x1405));
    tmpl->Set(isolate, "FLOAT", v8::Uint32::New(isolate, 0x1406));
    tmpl->Set(isolate, "DEPTH_COMPONENT", v8::Uint32::New(isolate, 0x1902));

    tmpl->Set(isolate, "ALPHA", v8::Uint32::New(isolate, 0x1906));
    tmpl->Set(isolate, "RGB", v8::Uint32::New(isolate, 0x1907));

    /* Clearing buffers */

    tmpl->Set(isolate, "RGBA", v8::Uint32::New(isolate, 0x1908));
    tmpl->Set(isolate, "LUMINANCE", v8::Uint32::New(isolate, 0x1909));
    tmpl->Set(isolate, "LUMINANCE_ALPHA", v8::Uint32::New(isolate, 0x190A));

    /* Clearing buffers */

    /* Rendering primitives */

    tmpl->Set(isolate, "UNSIGNED_SHORT_4_4_4_4", v8::Uint32::New(isolate, 0x8033));
    tmpl->Set(isolate, "UNSIGNED_SHORT_5_5_5_1", v8::Uint32::New(isolate, 0x8034));
    tmpl->Set(isolate, "UNSIGNED_SHORT_5_6_5", v8::Uint32::New(isolate, 0x8363));
    tmpl->Set(isolate, "FRAGMENT_SHADER", v8::Uint32::New(isolate, 0x8B30));
    tmpl->Set(isolate, "VERTEX_SHADER", v8::Uint32::New(isolate, 0x8B31));
    tmpl->Set(isolate, "COMPILE_STATUS", v8::Uint32::New(isolate, 0x8B81));
    tmpl->Set(isolate, "DELETE_STATUS", v8::Uint32::New(isolate, 0x8B80));

    /* Rendering primitives */

    /* Blending modes */

    tmpl->Set(isolate, "LINK_STATUS", v8::Uint32::New(isolate, 0x8B82));
    tmpl->Set(isolate, "VALIDATE_STATUS", v8::Uint32::New(isolate, 0x8B83));
    tmpl->Set(isolate, "ATTACHED_SHADERS", v8::Uint32::New(isolate, 0x8B85));
    tmpl->Set(isolate, "ACTIVE_ATTRIBUTES", v8::Uint32::New(isolate, 0x8B89));
    tmpl->Set(isolate, "ACTIVE_UNIFORMS", v8::Uint32::New(isolate, 0x8B86));
    tmpl->Set(isolate, "MAX_VERTEX_ATTRIBS", v8::Uint32::New(isolate, 0x8869));
    tmpl->Set(isolate, "MAX_VERTEX_UNIFORM_VECTORS", v8::Uint32::New(isolate, 0x8DFB));
    tmpl->Set(isolate, "MAX_VARYING_VECTORS", v8::Uint32::New(isolate, 0x8DFC));
    tmpl->Set(isolate, "MAX_COMBINED_TEXTURE_IMAGE_UNITS", v8::Uint32::New(isolate, 0x8B4D));
    tmpl->Set(isolate, "MAX_VERTEX_TEXTURE_IMAGE_UNITS", v8::Uint32::New(isolate, 0x8B4C));
    tmpl->Set(isolate, "MAX_TEXTURE_IMAGE_UNITS", v8::Uint32::New(isolate, 0x8872));
    tmpl->Set(isolate, "MAX_FRAGMENT_UNIFORM_VECTORS", v8::Uint32::New(isolate, 0x8DFD));
    tmpl->Set(isolate, "SHADER_TYPE", v8::Uint32::New(isolate, 0x8B4F));
    tmpl->Set(isolate, "SHADING_LANGUAGE_VERSION", v8::Uint32::New(isolate, 0x8B8C));
    tmpl->Set(isolate, "CURRENT_PROGRAM", v8::Uint32::New(isolate, 0x8B8D));

    /* Blending modes */

    tmpl->Set(isolate, "NEVER", v8::Uint32::New(isolate, 0x0200));
    tmpl->Set(isolate, "LESS", v8::Uint32::New(isolate, 0x0201));
    tmpl->Set(isolate, "EQUAL", v8::Uint32::New(isolate, 0x0202));

    /* Blending equations */

    /* Getting GL parameter information */

    tmpl->Set(isolate, "LEQUAL", v8::Uint32::New(isolate, 0x0203));
    tmpl->Set(isolate, "GREATER", v8::Uint32::New(isolate, 0x0204));
    tmpl->Set(isolate, "NOTEQUAL", v8::Uint32::New(isolate, 0x0205));
    tmpl->Set(isolate, "GEQUAL", v8::Uint32::New(isolate, 0x0206));
    tmpl->Set(isolate, "ALWAYS", v8::Uint32::New(isolate, 0x0207));
    tmpl->Set(isolate, "KEEP", v8::Uint32::New(isolate, 0x1E00));
    tmpl->Set(isolate, "REPLACE", v8::Uint32::New(isolate, 0x1E01));
    tmpl->Set(isolate, "INCR", v8::Uint32::New(isolate, 0x1E02));
    tmpl->Set(isolate, "DECR", v8::Uint32::New(isolate, 0x1E03));
    tmpl->Set(isolate, "INVERT", v8::Uint32::New(isolate, 0x150A));
    tmpl->Set(isolate, "INCR_WRAP", v8::Uint32::New(isolate, 0x8507));
    tmpl->Set(isolate, "DECR_WRAP", v8::Uint32::New(isolate, 0x8508));
    tmpl->Set(isolate, "NEAREST", v8::Uint32::New(isolate, 0x2600));
    tmpl->Set(isolate, "LINEAR", v8::Uint32::New(isolate, 0x2601));
    tmpl->Set(isolate, "NEAREST_MIPMAP_NEAREST", v8::Uint32::New(isolate, 0x2700));
    tmpl->Set(isolate, "LINEAR_MIPMAP_NEAREST", v8::Uint32::New(isolate, 0x2701));
    tmpl->Set(isolate, "NEAREST_MIPMAP_LINEAR", v8::Uint32::New(isolate, 0x2702));
    tmpl->Set(isolate, "LINEAR_MIPMAP_LINEAR", v8::Uint32::New(isolate, 0x2703));
    tmpl->Set(isolate, "TEXTURE_MAG_FILTER", v8::Uint32::New(isolate, 0x2800));
    tmpl->Set(isolate, "TEXTURE_MIN_FILTER", v8::Uint32::New(isolate, 0x2801));
    tmpl->Set(isolate, "TEXTURE_WRAP_S", v8::Uint32::New(isolate, 0x2802));
    tmpl->Set(isolate, "TEXTURE_WRAP_T", v8::Uint32::New(isolate, 0x2803));
    tmpl->Set(isolate, "TEXTURE_2D", v8::Uint32::New(isolate, 0x0DE1));
    tmpl->Set(isolate, "TEXTURE", v8::Uint32::New(isolate, 0x1702));
    tmpl->Set(isolate, "TEXTURE_CUBE_MAP", v8::Uint32::New(isolate, 0x8513));
    tmpl->Set(isolate, "TEXTURE_BINDING_CUBE_MAP", v8::Uint32::New(isolate, 0x8514));
    tmpl->Set(isolate, "TEXTURE_CUBE_MAP_POSITIVE_X", v8::Uint32::New(isolate, 0x8515));
    tmpl->Set(isolate, "TEXTURE_CUBE_MAP_NEGATIVE_X", v8::Uint32::New(isolate, 0x8516));
    tmpl->Set(isolate, "TEXTURE_CUBE_MAP_POSITIVE_Y", v8::Uint32::New(isolate, 0x8517));
    tmpl->Set(isolate, "TEXTURE_CUBE_MAP_NEGATIVE_Y", v8::Uint32::New(isolate, 0x8518));
    tmpl->Set(isolate, "TEXTURE_CUBE_MAP_POSITIVE_Z", v8::Uint32::New(isolate, 0x8519));
    tmpl->Set(isolate, "TEXTURE_CUBE_MAP_NEGATIVE_Z", v8::Uint32::New(isolate, 0x851A));
    tmpl->Set(isolate, "MAX_CUBE_MAP_TEXTURE_SIZE", v8::Uint32::New(isolate, 0x851C));
    tmpl->Set(isolate, "TEXTURE0", v8::Uint32::New(isolate, 0x84C0));
    tmpl->Set(isolate, "TEXTURE1", v8::Uint32::New(isolate, 0x84C1));
    tmpl->Set(isolate, "TEXTURE2", v8::Uint32::New(isolate, 0x84C2));
    tmpl->Set(isolate, "TEXTURE3", v8::Uint32::New(isolate, 0x84C3));
    tmpl->Set(isolate, "TEXTURE4", v8::Uint32::New(isolate, 0x84C4));
    tmpl->Set(isolate, "TEXTURE5", v8::Uint32::New(isolate, 0x84C5));
    tmpl->Set(isolate, "TEXTURE6", v8::Uint32::New(isolate, 0x84C6));
    tmpl->Set(isolate, "TEXTURE7", v8::Uint32::New(isolate, 0x84C7));
    tmpl->Set(isolate, "TEXTURE8", v8::Uint32::New(isolate, 0x84C8));
    tmpl->Set(isolate, "TEXTURE9", v8::Uint32::New(isolate, 0x84C9));
    tmpl->Set(isolate, "TEXTURE10", v8::Uint32::New(isolate, 0x84CA));
    tmpl->Set(isolate, "TEXTURE11", v8::Uint32::New(isolate, 0x84CB));
    tmpl->Set(isolate, "TEXTURE12", v8::Uint32::New(isolate, 0x84CC));
    tmpl->Set(isolate, "TEXTURE13", v8::Uint32::New(isolate, 0x84CD));
    tmpl->Set(isolate, "TEXTURE14", v8::Uint32::New(isolate, 0x84CE));
    tmpl->Set(isolate, "TEXTURE15", v8::Uint32::New(isolate, 0x84CF));
    tmpl->Set(isolate, "TEXTURE16", v8::Uint32::New(isolate, 0x84D0));
    tmpl->Set(isolate, "TEXTURE17", v8::Uint32::New(isolate, 0x84D1));
    tmpl->Set(isolate, "TEXTURE18", v8::Uint32::New(isolate, 0x84D2));
    tmpl->Set(isolate, "TEXTURE19", v8::Uint32::New(isolate, 0x84D3));
    tmpl->Set(isolate, "TEXTURE20", v8::Uint32::New(isolate, 0x84D4));
    tmpl->Set(isolate, "TEXTURE21", v8::Uint32::New(isolate, 0x84D5));
    tmpl->Set(isolate, "TEXTURE22", v8::Uint32::New(isolate, 0x84D6));
    tmpl->Set(isolate, "TEXTURE23", v8::Uint32::New(isolate, 0x84D7));
    tmpl->Set(isolate, "TEXTURE24", v8::Uint32::New(isolate, 0x84D8));
    tmpl->Set(isolate, "TEXTURE25", v8::Uint32::New(isolate, 0x84D9));
    tmpl->Set(isolate, "TEXTURE26", v8::Uint32::New(isolate, 0x84DA));
    tmpl->Set(isolate, "TEXTURE27", v8::Uint32::New(isolate, 0x84DB));
    tmpl->Set(isolate, "TEXTURE28", v8::Uint32::New(isolate, 0x84DC));
    tmpl->Set(isolate, "TEXTURE29", v8::Uint32::New(isolate, 0x84DD));

    /* Getting GL parameter information */

    /* Buffers */

    tmpl->Set(isolate, "TEXTURE30", v8::Uint32::New(isolate, 0x84DE));
    tmpl->Set(isolate, "TEXTURE31", v8::Uint32::New(isolate, 0x84DF));
    tmpl->Set(isolate, "ACTIVE_TEXTURE", v8::Uint32::New(isolate, 0x84E0));
    tmpl->Set(isolate, "REPEAT", v8::Uint32::New(isolate, 0x2901));
    tmpl->Set(isolate, "CLAMP_TO_EDGE", v8::Uint32::New(isolate, 0x812F));
    tmpl->Set(isolate, "MIRRORED_REPEAT", v8::Uint32::New(isolate, 0x8370));
    tmpl->Set(isolate, "FLOAT_VEC2", v8::Uint32::New(isolate, 0x8B50));

    /* Buffers */

    /* Vertex attributes */

    tmpl->Set(isolate, "FLOAT_VEC3", v8::Uint32::New(isolate, 0x8B51));
    tmpl->Set(isolate, "FLOAT_VEC4", v8::Uint32::New(isolate, 0x8B52));
    tmpl->Set(isolate, "INT_VEC2", v8::Uint32::New(isolate, 0x8B53));
    tmpl->Set(isolate, "INT_VEC3", v8::Uint32::New(isolate, 0x8B54));
    tmpl->Set(isolate, "INT_VEC4", v8::Uint32::New(isolate, 0x8B55));
    tmpl->Set(isolate, "BOOL", v8::Uint32::New(isolate, 0x8B56));
    tmpl->Set(isolate, "BOOL_VEC2", v8::Uint32::New(isolate, 0x8B57));
    tmpl->Set(isolate, "BOOL_VEC3", v8::Uint32::New(isolate, 0x8B58));

    /* Vertex attributes */

    /* Culling */

    tmpl->Set(isolate, "BOOL_VEC4", v8::Uint32::New(isolate, 0x8B59));
    tmpl->Set(isolate, "FLOAT_MAT2", v8::Uint32::New(isolate, 0x8B5A));
    tmpl->Set(isolate, "FLOAT_MAT3", v8::Uint32::New(isolate, 0x8B5B));
    tmpl->Set(isolate, "FLOAT_MAT4", v8::Uint32::New(isolate, 0x8B5C));

    /* Culling */

    /* Enabling and disabling */

    tmpl->Set(isolate, "SAMPLER_2D", v8::Uint32::New(isolate, 0x8B5E));
    tmpl->Set(isolate, "SAMPLER_CUBE", v8::Uint32::New(isolate, 0x8B60));
    tmpl->Set(isolate, "LOW_FLOAT", v8::Uint32::New(isolate, 0x8DF0));
    tmpl->Set(isolate, "MEDIUM_FLOAT", v8::Uint32::New(isolate, 0x8DF1));
    tmpl->Set(isolate, "HIGH_FLOAT", v8::Uint32::New(isolate, 0x8DF2));
    tmpl->Set(isolate, "LOW_INT", v8::Uint32::New(isolate, 0x8DF3));
    tmpl->Set(isolate, "MEDIUM_INT", v8::Uint32::New(isolate, 0x8DF4));
    tmpl->Set(isolate, "HIGH_INT", v8::Uint32::New(isolate, 0x8DF5));

    /* Enabling and disabling */

    tmpl->Set(isolate, "FRAMEBUFFER", v8::Uint32::New(isolate, 0x8D40));
    tmpl->Set(isolate, "RENDERBUFFER", v8::Uint32::New(isolate, 0x8D41));
    tmpl->Set(isolate, "RGBA4", v8::Uint32::New(isolate, 0x8056));
    tmpl->Set(isolate, "RGB5_A1", v8::Uint32::New(isolate, 0x8057));
    tmpl->Set(isolate, "RGB565", v8::Uint32::New(isolate, 0x8D62));
    tmpl->Set(isolate, "DEPTH_COMPONENT16", v8::Uint32::New(isolate, 0x81A5));
    tmpl->Set(isolate, "STENCIL_INDEX8", v8::Uint32::New(isolate, 0x8D48));

    /* Errors */

    /* Front face directions */

    tmpl->Set(isolate, "DEPTH_STENCIL", v8::Uint32::New(isolate, 0x84F9));
    tmpl->Set(isolate, "RENDERBUFFER_WIDTH", v8::Uint32::New(isolate, 0x8D42));

    /* Front face directions */

    /* Hints */

    tmpl->Set(isolate, "RENDERBUFFER_HEIGHT", v8::Uint32::New(isolate, 0x8D43));
    tmpl->Set(isolate, "RENDERBUFFER_INTERNAL_FORMAT", v8::Uint32::New(isolate, 0x8D44));
    tmpl->Set(isolate, "RENDERBUFFER_RED_SIZE", v8::Uint32::New(isolate, 0x8D50));
    tmpl->Set(isolate, "RENDERBUFFER_GREEN_SIZE", v8::Uint32::New(isolate, 0x8D51));

    /* Hints */

    /* Data types */

    tmpl->Set(isolate, "RENDERBUFFER_BLUE_SIZE", v8::Uint32::New(isolate, 0x8D52));
    tmpl->Set(isolate, "RENDERBUFFER_ALPHA_SIZE", v8::Uint32::New(isolate, 0x8D53));
    tmpl->Set(isolate, "RENDERBUFFER_DEPTH_SIZE", v8::Uint32::New(isolate, 0x8D54));
    tmpl->Set(isolate, "RENDERBUFFER_STENCIL_SIZE", v8::Uint32::New(isolate, 0x8D55));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE", v8::Uint32::New(isolate, 0x8CD0));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_OBJECT_NAME", v8::Uint32::New(isolate, 0x8CD1));
    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL", v8::Uint32::New(isolate, 0x8CD2));

    /* Data types */

    /* Pixel formats */

    tmpl->Set(isolate, "FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE",
              v8::Uint32::New(isolate, 0x8CD3));
    tmpl->Set(isolate, "COLOR_ATTACHMENT0", v8::Uint32::New(isolate, 0x8CE0));
    tmpl->Set(isolate, "DEPTH_ATTACHMENT", v8::Uint32::New(isolate, 0x8D00));
    tmpl->Set(isolate, "STENCIL_ATTACHMENT", v8::Uint32::New(isolate, 0x8D20));
    tmpl->Set(isolate, "DEPTH_STENCIL_ATTACHMENT", v8::Uint32::New(isolate, 0x821A));
    tmpl->Set(isolate, "NONE", v8::Uint32::New(isolate, 0));

    /* Pixel formats */

    /* Pixel types */

    tmpl->Set(isolate, "FRAMEBUFFER_COMPLETE", v8::Uint32::New(isolate, 0x8CD5));
    tmpl->Set(isolate, "FRAMEBUFFER_INCOMPLETE_ATTACHMENT", v8::Uint32::New(isolate, 0x8CD6));
    tmpl->Set(isolate, "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT",
              v8::Uint32::New(isolate, 0x8CD7));

    /* Pixel types */

    /* Shaders */

    tmpl->Set(isolate, "FRAMEBUFFER_INCOMPLETE_DIMENSIONS", v8::Uint32::New(isolate, 0x8CD9));
    tmpl->Set(isolate, "FRAMEBUFFER_UNSUPPORTED", v8::Uint32::New(isolate, 0x8CDD));
    tmpl->Set(isolate, "FRAMEBUFFER_BINDING", v8::Uint32::New(isolate, 0x8CA6));
    tmpl->Set(isolate, "RENDERBUFFER_BINDING", v8::Uint32::New(isolate, 0x8CA7));
    tmpl->Set(isolate, "MAX_RENDERBUFFER_SIZE", v8::Uint32::New(isolate, 0x84E8));
    tmpl->Set(isolate, "INVALID_FRAMEBUFFER_OPERATION", v8::Uint32::New(isolate, 0x0506));
    tmpl->Set(isolate, "UNPACK_FLIP_Y_WEBGL", v8::Uint32::New(isolate, 0x9240));
    tmpl->Set(isolate, "UNPACK_PREMULTIPLY_ALPHA_WEBGL", v8::Uint32::New(isolate, 0x9241));
    tmpl->Set(isolate, "UNPACK_COLORSPACE_CONVERSION_WEBGL", v8::Uint32::New(isolate, 0x9243));
}

void WebGLRenderingContext::SetProps(v8::Isolate *isolate,
                                     const v8::Local<v8::ObjectTemplate> &tmpl) {

    tmpl->SetAccessor(ConvertToV8String(isolate, "drawingBufferWidth"), &GetDrawingBufferWidth);
    tmpl->SetAccessor(ConvertToV8String(isolate, "drawingBufferHeight"), &GetDrawingBufferHeight);
    tmpl->SetAccessor(ConvertToV8String(isolate, "__flipY"), &GetFlipY);
}


void
WebGLRenderingContext::SetMethods(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl) {

    tmpl->Set(ConvertToV8String(isolate, "__resized"),
              v8::FunctionTemplate::New(isolate, &__Resized));
    tmpl->Set(ConvertToV8String(isolate, "__startRaf"),
              v8::FunctionTemplate::New(isolate, &__StartRaf));
    tmpl->Set(ConvertToV8String(isolate, "__stopRaf"),
              v8::FunctionTemplate::New(isolate, &__StopRaf));

    tmpl->Set(ConvertToV8String(isolate, "__toDataURL"),
              v8::FunctionTemplate::New(isolate, &__ToDataURL));

    tmpl->Set(ConvertToV8String(isolate, "__getSupportedExtensions"),
              v8::FunctionTemplate::New(isolate, &__GetSupportedExtensions));


    tmpl->Set(
            ConvertToV8String(isolate, "activeTexture"),
            v8::FunctionTemplate::New(isolate, &ActiveTexture)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "attachShader"),
            v8::FunctionTemplate::New(isolate, &AttachShader)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindAttribLocation"),
            v8::FunctionTemplate::New(isolate, &BindAttribLocation)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindBuffer"),
            v8::FunctionTemplate::New(isolate, &BindBuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindFramebuffer"),
            v8::FunctionTemplate::New(isolate, &BindFramebuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &BindRenderbuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bindTexture"),
            v8::FunctionTemplate::New(isolate, &BindTexture)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "blendColor"),
            v8::FunctionTemplate::New(isolate, &BlendColor)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "blendEquationSeparate"),
            v8::FunctionTemplate::New(isolate, &BlendEquationSeparate)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "blendEquation"),
            v8::FunctionTemplate::New(isolate, &BlendEquation)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "blendFuncSeparate"),
            v8::FunctionTemplate::New(isolate, &BlendFuncSeparate)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "blendFunc"),
            v8::FunctionTemplate::New(isolate, &BlendFunc)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bufferData"),
            v8::FunctionTemplate::New(isolate, &BufferData)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "bufferSubData"),
            v8::FunctionTemplate::New(isolate, &BufferSubData)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "checkFramebufferStatus"),
            v8::FunctionTemplate::New(isolate, &CheckFramebufferStatus)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clearColor"),
            v8::FunctionTemplate::New(isolate, &ClearColor)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clearDepth"),
            v8::FunctionTemplate::New(isolate, &ClearDepth)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clearStencil"),
            v8::FunctionTemplate::New(isolate, &ClearStencil)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "clear"),
            v8::FunctionTemplate::New(isolate, &Clear)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "colorMask"),
            v8::FunctionTemplate::New(isolate, &ColorMask)
    );
    tmpl->Set(
            ConvertToV8String(isolate, "commit"),
            v8::FunctionTemplate::New(isolate, &Commit)
    );
    tmpl->Set(
            ConvertToV8String(isolate, "compileShader"),
            v8::FunctionTemplate::New(isolate, &CompileShader)
    );
    tmpl->Set(
            ConvertToV8String(isolate, "compressedTexImage2D"),
            v8::FunctionTemplate::New(isolate, &CompressedTexImage2D)
    );
    tmpl->Set(
            ConvertToV8String(isolate, "compressedTexSubImage2D"),
            v8::FunctionTemplate::New(isolate, &CompressedTexSubImage2D)
    );
    tmpl->Set(
            ConvertToV8String(isolate, "copyTexImage2D"),
            v8::FunctionTemplate::New(isolate, &CopyTexImage2D)
    );
    tmpl->Set(
            ConvertToV8String(isolate, "copyTexSubImage2D"),
            v8::FunctionTemplate::New(isolate, &CopyTexSubImage2D)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createBuffer"),
            v8::FunctionTemplate::New(isolate, &CreateBuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createFramebuffer"),
            v8::FunctionTemplate::New(isolate, &CreateFramebuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createProgram"),
            v8::FunctionTemplate::New(isolate, &CreateProgram)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &CreateRenderbuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createShader"),
            v8::FunctionTemplate::New(isolate, &CreateShader)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createTexture"),
            v8::FunctionTemplate::New(isolate, &CreateTexture)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "cullFace"),
            v8::FunctionTemplate::New(isolate, &CullFace)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteBuffer"),
            v8::FunctionTemplate::New(isolate, &DeleteBuffer)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "deleteFramebuffer"),
            v8::FunctionTemplate::New(isolate, &DeleteFramebuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteProgram"),
            v8::FunctionTemplate::New(isolate, &DeleteProgram)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &DeleteRenderbuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteShader"),
            v8::FunctionTemplate::New(isolate, &DeleteShader)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "deleteTexture"),
            v8::FunctionTemplate::New(isolate, &DeleteTexture)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "depthFunc"),
            v8::FunctionTemplate::New(isolate, &DepthFunc)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "depthMask"),
            v8::FunctionTemplate::New(isolate, &DepthMask)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "depthRange"),
            v8::FunctionTemplate::New(isolate, &DepthRange)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "detachShader"),
            v8::FunctionTemplate::New(isolate, &DetachShader)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "disableVertexAttribArray"),
            v8::FunctionTemplate::New(isolate, &DisableVertexAttribArray)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "disable"),
            v8::FunctionTemplate::New(isolate, &Disable)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "drawArrays"),
            v8::FunctionTemplate::New(isolate, &DrawArrays)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "drawElements"),
            v8::FunctionTemplate::New(isolate, &DrawElements)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "enableVertexAttribArray"),
            v8::FunctionTemplate::New(isolate, &EnableVertexAttribArray)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "enable"),
            v8::FunctionTemplate::New(isolate, &Enable)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "finish"),
            v8::FunctionTemplate::New(isolate, &Finish)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "flush"),
            v8::FunctionTemplate::New(isolate, &Flush)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "framebufferRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &FramebufferRenderbuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "framebufferTexture2D"),
            v8::FunctionTemplate::New(isolate, &FramebufferTexture2D)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "frontFace"),
            v8::FunctionTemplate::New(isolate, &FrontFace)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "generateMipmap"),
            v8::FunctionTemplate::New(isolate, &GenerateMipmap)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getActiveAttrib"),
            v8::FunctionTemplate::New(isolate, &GetActiveAttrib)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getActiveUniform"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniform)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getAttachedShaders"),
            v8::FunctionTemplate::New(isolate, &GetAttachedShaders)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getAttribLocation"),
            v8::FunctionTemplate::New(isolate, &GetAttribLocation)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getBufferParameter"),
            v8::FunctionTemplate::New(isolate, &GetBufferParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getContextAttributes"),
            v8::FunctionTemplate::New(isolate, &GetContextAttributes)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getError"),
            v8::FunctionTemplate::New(isolate, &GetError)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getExtension"),
            v8::FunctionTemplate::New(isolate, &GetExtension)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getFramebufferAttachmentParameter"),
            v8::FunctionTemplate::New(isolate, &GetFramebufferAttachmentParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getParameter"),
            v8::FunctionTemplate::New(isolate, &GetParameter)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getProgramInfoLog"),
            v8::FunctionTemplate::New(isolate, &GetProgramInfoLog)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getProgramParameter"),
            v8::FunctionTemplate::New(isolate, &GetProgramParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getRenderbufferParameter"),
            v8::FunctionTemplate::New(isolate, &GetRenderbufferParameter)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getShaderInfoLog"),
            v8::FunctionTemplate::New(isolate, &GetShaderInfoLog)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getShaderParameter"),
            v8::FunctionTemplate::New(isolate, &GetShaderParameter)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getShaderPrecisionFormat"),
            v8::FunctionTemplate::New(isolate, &GetShaderPrecisionFormat)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getShaderSource"),
            v8::FunctionTemplate::New(isolate, &GetShaderSource)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getSupportedExtensions"),
            v8::FunctionTemplate::New(isolate, &GetSupportedExtensions)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getTexParameter"),
            v8::FunctionTemplate::New(isolate, &GetTexParameter)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getUniformLocation"),
            v8::FunctionTemplate::New(isolate, &GetUniformLocation)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getUniform"),
            v8::FunctionTemplate::New(isolate, &GetUniform)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getVertexAttribOffset"),
            v8::FunctionTemplate::New(isolate, &GetVertexAttribOffset)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "getVertexAttrib"),
            v8::FunctionTemplate::New(isolate, &GetVertexAttrib)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "hint"),
            v8::FunctionTemplate::New(isolate, &Hint)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "isBuffer"),
            v8::FunctionTemplate::New(isolate, &IsBuffer)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "isContextLost"),
            v8::FunctionTemplate::New(isolate, &IsContextLost)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "isEnabled"),
            v8::FunctionTemplate::New(isolate, &IsEnabled)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "isFramebuffer"),
            v8::FunctionTemplate::New(isolate, &IsFramebuffer)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "isProgram"),
            v8::FunctionTemplate::New(isolate, &IsProgram)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "isRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &IsRenderbuffer)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "isShader"),
            v8::FunctionTemplate::New(isolate, &IsShader)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "isTexture"),
            v8::FunctionTemplate::New(isolate, &IsTexture)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "lineWidth"),
            v8::FunctionTemplate::New(isolate, &LineWidth)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "linkProgram"),
            v8::FunctionTemplate::New(isolate, &LinkProgram)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "pixelStorei"),
            v8::FunctionTemplate::New(isolate, &PixelStorei)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "polygonOffset"),
            v8::FunctionTemplate::New(isolate, &PolygonOffset)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "readPixels"),
            v8::FunctionTemplate::New(isolate, &ReadPixels)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "renderbufferStorage"),
            v8::FunctionTemplate::New(isolate, &RenderbufferStorage)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "sampleCoverage"),
            v8::FunctionTemplate::New(isolate, &SampleCoverage)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "scissor"),
            v8::FunctionTemplate::New(isolate, &Scissor)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "shaderSource"),
            v8::FunctionTemplate::New(isolate, &ShaderSource)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "stencilFuncSeparate"),
            v8::FunctionTemplate::New(isolate, &StencilFuncSeparate)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "stencilFunc"),
            v8::FunctionTemplate::New(isolate, &StencilFunc)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "stencilMaskSeparate"),
            v8::FunctionTemplate::New(isolate, &StencilMaskSeparate)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "stencilMask"),
            v8::FunctionTemplate::New(isolate, &StencilMask)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "stencilOpSeparate"),
            v8::FunctionTemplate::New(isolate, &StencilOpSeparate)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "stencilOp"),
            v8::FunctionTemplate::New(isolate, &StencilOp)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "texImage2D"),
            v8::FunctionTemplate::New(isolate, &TexImage2D)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "texParameterf"),
            v8::FunctionTemplate::New(isolate, &TexParameterf)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "texParameteri"),
            v8::FunctionTemplate::New(isolate, &TexParameteri)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "texSubImage2D"),
            v8::FunctionTemplate::New(isolate, &TexSubImage2D)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib1f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib1f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib1fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib1fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib2f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib2f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib2fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib2fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib3f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib3f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib3fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib3fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib4f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib4f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttrib4fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib4fv)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttribPointer"),
            v8::FunctionTemplate::New(isolate, &VertexAttribPointer)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniform1f"),
            v8::FunctionTemplate::New(isolate, &Uniform1f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform1iv"),
            v8::FunctionTemplate::New(isolate, &Uniform1iv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform1fv"),
            v8::FunctionTemplate::New(isolate, &Uniform1fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform1i"),
            v8::FunctionTemplate::New(isolate, &Uniform1i)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniform2f"),
            v8::FunctionTemplate::New(isolate, &Uniform2f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform2iv"),
            v8::FunctionTemplate::New(isolate, &Uniform2iv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform2fv"),
            v8::FunctionTemplate::New(isolate, &Uniform2fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform2i"),
            v8::FunctionTemplate::New(isolate, &Uniform2i)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform3f"),
            v8::FunctionTemplate::New(isolate, &Uniform3f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform3iv"),
            v8::FunctionTemplate::New(isolate, &Uniform3iv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform3fv"),
            v8::FunctionTemplate::New(isolate, &Uniform3fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform3i"),
            v8::FunctionTemplate::New(isolate, &Uniform3i)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform4f"),
            v8::FunctionTemplate::New(isolate, &Uniform4f)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform4iv"),
            v8::FunctionTemplate::New(isolate, &Uniform4iv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform4fv"),
            v8::FunctionTemplate::New(isolate, &Uniform4fv)
    );

    tmpl->Set(
            ConvertToV8String(isolate, "uniform4i"),
            v8::FunctionTemplate::New(isolate, &Uniform4i)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix2fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix2fv)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix3fv)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "uniformMatrix4fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4fv)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "useProgram"),
            v8::FunctionTemplate::New(isolate, &UseProgram)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "validateProgram"),
            v8::FunctionTemplate::New(isolate, &ValidateProgram)
    );


    tmpl->Set(
            ConvertToV8String(isolate, "viewport"),
            v8::FunctionTemplate::New(isolate, &Viewport)
    );

}
