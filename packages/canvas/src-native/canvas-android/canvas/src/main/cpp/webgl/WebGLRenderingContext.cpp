//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGLRenderingContext.h"
#include "canvas-android/src/lib.rs.h"
#include "../canvas2d/CanvasRenderingContext2DImpl.h"

WebGLRenderingContext::WebGLRenderingContext(rust::Box<WebGLState> state)
        : WebGLRenderingContextBase(
        std::move(state), WebGLRenderingVersion::V1) {}

std::vector<jsi::PropNameID> WebGLRenderingContext::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("activeTexture")),
            jsi::PropNameID::forUtf8(rt, std::string("attachShader")),
            jsi::PropNameID::forUtf8(rt, std::string("bindAttribLocation")),
            jsi::PropNameID::forUtf8(rt, std::string("bindBuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("bindFramebuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("bindRenderbuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("bindTexture")),
            jsi::PropNameID::forUtf8(rt, std::string("blendColor")),
            jsi::PropNameID::forUtf8(rt, std::string("blendEquationSeparate")),
            jsi::PropNameID::forUtf8(rt, std::string("blendEquation")),
            jsi::PropNameID::forUtf8(rt, std::string("blendFuncSeparate")),
            jsi::PropNameID::forUtf8(rt, std::string("blendFunc")),
            jsi::PropNameID::forUtf8(rt, std::string("bufferData")),
            jsi::PropNameID::forUtf8(rt, std::string("bufferSubData")),
            jsi::PropNameID::forUtf8(rt, std::string("checkFramebufferStatus")),
            jsi::PropNameID::forUtf8(rt, std::string("clearColor")),
            jsi::PropNameID::forUtf8(rt, std::string("clearDepth")),
            jsi::PropNameID::forUtf8(rt, std::string("clearStencil")),
            jsi::PropNameID::forUtf8(rt, std::string("clear")),
            jsi::PropNameID::forUtf8(rt, std::string("colorMask")),
            jsi::PropNameID::forUtf8(rt, std::string("commit")),
            jsi::PropNameID::forUtf8(rt, std::string("compileShader")),
            jsi::PropNameID::forUtf8(rt, std::string("compressedTexImage2D")),
            jsi::PropNameID::forUtf8(rt, std::string("compressedTexSubImage2D")),
            jsi::PropNameID::forUtf8(rt, std::string("copyTexImage2D")),
            jsi::PropNameID::forUtf8(rt, std::string("copyTexSubImage2D")),
            jsi::PropNameID::forUtf8(rt, std::string("createBuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("createFramebuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("createProgram")),
            jsi::PropNameID::forUtf8(rt, std::string("createRenderbuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("createShader")),
            jsi::PropNameID::forUtf8(rt, std::string("createTexture")),
            jsi::PropNameID::forUtf8(rt, std::string("cullFace")),
            jsi::PropNameID::forUtf8(rt, std::string("deleteBuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("deleteFramebuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("deleteProgram")),
            jsi::PropNameID::forUtf8(rt, std::string("deleteRenderbuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("deleteShader")),
            jsi::PropNameID::forUtf8(rt, std::string("deleteTexture")),
            jsi::PropNameID::forUtf8(rt, std::string("depthFunc")),
            jsi::PropNameID::forUtf8(rt, std::string("depthMask")),
            jsi::PropNameID::forUtf8(rt, std::string("depthRange")),
            jsi::PropNameID::forUtf8(rt, std::string("detachShader")),
            jsi::PropNameID::forUtf8(rt, std::string("disableVertexAttribArray")),
            jsi::PropNameID::forUtf8(rt, std::string("disable")),
            jsi::PropNameID::forUtf8(rt, std::string("drawArrays")),
            jsi::PropNameID::forUtf8(rt, std::string("drawElements")),
            jsi::PropNameID::forUtf8(rt, std::string("enableVertexAttribArray")),
            jsi::PropNameID::forUtf8(rt, std::string("enable")),
            jsi::PropNameID::forUtf8(rt, std::string("finish")),
            jsi::PropNameID::forUtf8(rt, std::string("flush")),
            jsi::PropNameID::forUtf8(rt, std::string("framebufferRenderbuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("framebufferTexture2D")),
            jsi::PropNameID::forUtf8(rt, std::string("frontFace")),
            jsi::PropNameID::forUtf8(rt, std::string("generateMipmap")),
            jsi::PropNameID::forUtf8(rt, std::string("getActiveAttrib")),
            jsi::PropNameID::forUtf8(rt, std::string("getActiveUniform")),
            jsi::PropNameID::forUtf8(rt, std::string("getAttachedShaders")),
            jsi::PropNameID::forUtf8(rt, std::string("getAttribLocation")),
            jsi::PropNameID::forUtf8(rt, std::string("getBufferParameter")),
            jsi::PropNameID::forUtf8(rt, std::string("getContextAttributes")),
            jsi::PropNameID::forUtf8(rt, std::string("getError")),
            jsi::PropNameID::forUtf8(rt, std::string("getExtension")),
            jsi::PropNameID::forUtf8(rt, std::string("getFramebufferAttachmentParameter")),
            jsi::PropNameID::forUtf8(rt, std::string("getParameter")),
            jsi::PropNameID::forUtf8(rt, std::string("getProgramInfoLog")),
            jsi::PropNameID::forUtf8(rt, std::string("getProgramParameter")),
            jsi::PropNameID::forUtf8(rt, std::string("getRenderbufferParameter")),
            jsi::PropNameID::forUtf8(rt, std::string("getShaderInfoLog")),
            jsi::PropNameID::forUtf8(rt, std::string("getShaderParameter")),
            jsi::PropNameID::forUtf8(rt, std::string("getShaderPrecisionFormat")),
            jsi::PropNameID::forUtf8(rt, std::string("getShaderSource")),
            jsi::PropNameID::forUtf8(rt, std::string("getSupportedExtensions")),
            jsi::PropNameID::forUtf8(rt, std::string("getTexParameter")),
            jsi::PropNameID::forUtf8(rt, std::string("getUniformLocation")),
            jsi::PropNameID::forUtf8(rt, std::string("getUniform")),
            jsi::PropNameID::forUtf8(rt, std::string("getVertexAttribOffset")),
            jsi::PropNameID::forUtf8(rt, std::string("getVertexAttrib")),
            jsi::PropNameID::forUtf8(rt, std::string("hint")),
            jsi::PropNameID::forUtf8(rt, std::string("isBuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("isContextLost")),
            jsi::PropNameID::forUtf8(rt, std::string("isEnabled")),
            jsi::PropNameID::forUtf8(rt, std::string("isFramebuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("isProgram")),
            jsi::PropNameID::forUtf8(rt, std::string("isRenderbuffer")),
            jsi::PropNameID::forUtf8(rt, std::string("isShader")),
            jsi::PropNameID::forUtf8(rt, std::string("isTexture")),
            jsi::PropNameID::forUtf8(rt, std::string("lineWidth")),
            jsi::PropNameID::forUtf8(rt, std::string("linkProgram")),
            jsi::PropNameID::forUtf8(rt, std::string("pixelStorei")),
            jsi::PropNameID::forUtf8(rt, std::string("polygonOffset")),
            jsi::PropNameID::forUtf8(rt, std::string("readPixels")),
            jsi::PropNameID::forUtf8(rt, std::string("renderbufferStorage")),
            jsi::PropNameID::forUtf8(rt, std::string("sampleCoverage")),
            jsi::PropNameID::forUtf8(rt, std::string("scissor")),
            jsi::PropNameID::forUtf8(rt, std::string("shaderSource")),
            jsi::PropNameID::forUtf8(rt, std::string("stencilFuncSeparate")),
            jsi::PropNameID::forUtf8(rt, std::string("stencilFunc")),
            jsi::PropNameID::forUtf8(rt, std::string("stencilMaskSeparate")),
            jsi::PropNameID::forUtf8(rt, std::string("stencilMask")),
            jsi::PropNameID::forUtf8(rt, std::string("stencilOpSeparate")),
            jsi::PropNameID::forUtf8(rt, std::string("stencilOp")),
            jsi::PropNameID::forUtf8(rt, std::string("texImage2D")),
            jsi::PropNameID::forUtf8(rt, std::string("texParameterf")),
            jsi::PropNameID::forUtf8(rt, std::string("texParameteri")),
            jsi::PropNameID::forUtf8(rt, std::string("texSubImage2D")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform1f")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform1iv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform1fv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform1i")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform2f")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform2iv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform2fv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform2i")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform3f")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform3iv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform3fv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform3i")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform4f")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform4iv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform4fv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniform4i")),
            jsi::PropNameID::forUtf8(rt, std::string("uniformMatrix2fv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniformMatrix3fv")),
            jsi::PropNameID::forUtf8(rt, std::string("uniformMatrix4fv")),
            jsi::PropNameID::forUtf8(rt, std::string("useProgram")),
            jsi::PropNameID::forUtf8(rt, std::string("validateProgram")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib1f")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib1fv")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib2f")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib2fv")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib3f")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib3fv")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib4f")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttrib4fv")),
            jsi::PropNameID::forUtf8(rt, std::string("vertexAttribPointer")),
            jsi::PropNameID::forUtf8(rt, std::string("viewport")),
    };
}

jsi::Value WebGLRenderingContext::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "activeTexture") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto texture = (uint32_t) arguments[0].asNumber();
                                                         canvas_native_webgl_active_texture(texture,
                                                                                            this->GetState());

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "attachShader") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto program = arguments[0].asObject(
                                                                 runtime).asHostObject<WebGLProgram>(
                                                                 runtime);
                                                         auto shader = arguments[1].asObject(
                                                                 runtime).asHostObject<WebGLShader>(
                                                                 runtime);

                                                         if (program == nullptr) {
                                                             return Value::undefined();
                                                         }

                                                         if (shader == nullptr) {
                                                             return Value::undefined();
                                                         }

                                                         canvas_native_webgl_attach_shader(
                                                                 program->GetProgram(),
                                                                 shader->GetShader(),
                                                                 this->GetState()
                                                         );


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindAttribLocation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto program = arguments[0].asObject(
                                                                     runtime).asHostObject<WebGLProgram>(
                                                                     runtime);

                                                             if (program != nullptr &&
                                                                 arguments[1].isNumber() &&
                                                                 arguments[2].isString()) {
                                                                 auto index = (uint32_t) arguments[1].asNumber();
                                                                 auto name = arguments[2].asString(
                                                                         runtime).utf8(runtime);
                                                                 canvas_native_webgl_bind_attrib_location(
                                                                         program->GetProgram(),
                                                                         index,
                                                                         rust::Str(name.data(),
                                                                                   name.size()),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindBuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {

                                                             if (arguments[0].isNumber()) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();
                                                                 if (!arguments[1].isNull() &&
                                                                     arguments[1].isObject()) {
                                                                     auto buffer = arguments[1].asObject(
                                                                             runtime).asHostObject<WebGLBuffer>(
                                                                             runtime);
                                                                     if (buffer ==
                                                                         nullptr) { return Value::undefined(); }
                                                                     canvas_native_webgl_bind_buffer(
                                                                             target,
                                                                             buffer->GetBuffer(),
                                                                             this->GetState()
                                                                     );
                                                                 } else {
                                                                     // unbind
                                                                     // check for null or undefined ?
                                                                     canvas_native_webgl_bind_buffer(
                                                                             target,
                                                                             0,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindFramebuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {

                                                             if (arguments[0].isNumber()) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();
                                                                 if (arguments[1].isObject()) {
                                                                     auto framebuffer = arguments[1].asObject(
                                                                             runtime).asHostObject<WebGLFramebuffer>(
                                                                             runtime);
                                                                     canvas_native_webgl_bind_frame_buffer(
                                                                             target,
                                                                             framebuffer->GetFrameBuffer(),
                                                                             this->GetState()
                                                                     );
                                                                 } else {
                                                                     // null value
                                                                     // unbind
                                                                     canvas_native_webgl_bind_frame_buffer(
                                                                             target,
                                                                             0,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindRenderbuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             if (arguments[0].isNumber()) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();
                                                                 if (arguments[1].isObject()) {
                                                                     auto renderbuffer = arguments[1].asObject(
                                                                             runtime).asHostObject<WebGLRenderbuffer>(
                                                                             runtime);

                                                                     if (renderbuffer ==
                                                                         nullptr) { return Value::undefined(); }
                                                                     canvas_native_webgl_bind_render_buffer(
                                                                             target,
                                                                             renderbuffer->GetRenderBuffer(),
                                                                             this->GetState()
                                                                     );
                                                                 } else {
                                                                     canvas_native_webgl_bind_render_buffer(
                                                                             target,
                                                                             0,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindTexture") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             if (arguments[0].isNumber()) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();
                                                                 if (arguments[1].isObject()) {
                                                                     auto texture = arguments[1].asObject(
                                                                             runtime).asHostObject<WebGLTexture>(
                                                                             runtime);
                                                                     canvas_native_webgl_bind_texture(
                                                                             target,
                                                                             texture->GetTexture(),
                                                                             this->GetState()
                                                                     );
                                                                 } else {
                                                                     canvas_native_webgl_bind_texture(
                                                                             target,
                                                                             0,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendColor") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto red = arguments[0].asNumber();
                                                             auto green = arguments[1].asNumber();
                                                             auto blue = arguments[2].asNumber();
                                                             auto alpha = arguments[3].asNumber();

                                                             canvas_native_webgl_blend_color(
                                                                     static_cast<float>(red),
                                                                     static_cast<float>(green),
                                                                     static_cast<float>(blue),
                                                                     static_cast<float>(alpha),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquationSeparate") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto modeRGB = (uint32_t) arguments[0].asNumber();
                                                             auto modeAlpha = (uint32_t) arguments[1].asNumber();

                                                             canvas_native_webgl_blend_equation_separate(
                                                                     modeRGB,
                                                                     modeAlpha,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto mode = (uint32_t) arguments[0].asNumber();
                                                             canvas_native_webgl_blend_equation(
                                                                     mode,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendFuncSeparate") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 3) {
                                                             auto srcRGB = (uint32_t) arguments[0].asNumber();
                                                             auto dstRGB = (uint32_t) arguments[1].asNumber();
                                                             auto srcAlpha = (uint32_t) arguments[2].asNumber();
                                                             auto dstAlpha = (uint32_t) arguments[3].asNumber();

                                                             canvas_native_webgl_blend_func_separate(
                                                                     srcRGB,
                                                                     dstRGB,
                                                                     srcAlpha,
                                                                     dstAlpha,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendFunc") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto sfactor = (uint32_t) arguments[0].asNumber();
                                                             auto dfactor = (uint32_t) arguments[1].asNumber();

                                                             canvas_native_webgl_blend_func(
                                                                     sfactor,
                                                                     dfactor,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bufferData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 2) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto usage = (uint32_t) arguments[1].asNumber();

                                                             canvas_native_webgl_buffer_data_none(
                                                                     target,
                                                                     0,
                                                                     usage,
                                                                     this->GetState()
                                                             );
                                                         } else if (count == 3) {
                                                             auto target = (uint32_t) arguments[0].asNumber();

                                                             auto usage = (uint32_t) arguments[2].asNumber();

                                                             if (arguments[1].isObject()) {
                                                                 auto sizeOrBuf = arguments[1].asObject(
                                                                         runtime);
                                                                 if (sizeOrBuf.isArrayBufferView(
                                                                         runtime)) {
                                                                     if (sizeOrBuf.isUint16Array(
                                                                             runtime)) {
                                                                         auto array = sizeOrBuf.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<uint16_t>(
                                                                                 runtime, array);

                                                                         canvas_native_webgl_buffer_data_u16(
                                                                                 target,
                                                                                 buf,
                                                                                 usage,
                                                                                 this->GetState()
                                                                         );
                                                                     } else if (sizeOrBuf.isFloat32Array(
                                                                             runtime)) {
                                                                         auto array = sizeOrBuf.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const float>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_buffer_data_f32(
                                                                                 target,
                                                                                 buf,
                                                                                 usage,
                                                                                 this->GetState()
                                                                         );
                                                                     } else {
                                                                         auto array = sizeOrBuf.getTypedArray(
                                                                                 runtime);
                                                                         auto buf = GetTypedArrayData<const uint8_t>(
                                                                                 runtime, array);
                                                                         canvas_native_webgl_buffer_data(
                                                                                 target,
                                                                                 buf,
                                                                                 usage,
                                                                                 this->GetState()
                                                                         );
                                                                     }
                                                                 } else if (sizeOrBuf.isArrayBuffer(
                                                                         runtime)) {
                                                                     auto buffer = sizeOrBuf.getArrayBuffer(
                                                                             runtime);
                                                                     auto data = buffer.data(
                                                                             runtime);
                                                                     auto size = buffer.size(
                                                                             runtime);
                                                                     rust::Slice<const uint8_t> buf(
                                                                             data, size);
                                                                     canvas_native_webgl_buffer_data(
                                                                             target,
                                                                             buf,
                                                                             usage,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             } else {
                                                                 auto sizeOrBuf = arguments[1].asNumber();
                                                                 canvas_native_webgl_buffer_data_none(
                                                                         target,
                                                                         static_cast<ssize_t>(sizeOrBuf),
                                                                         usage,
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bufferSubData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 2) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto offset = arguments[1].asNumber();

                                                             canvas_native_webgl_buffer_sub_data_none(
                                                                     target,
                                                                     static_cast<ssize_t>(offset),
                                                                     this->GetState()
                                                             );
                                                         } else if (count == 3) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto offset = arguments[1].asNumber();

                                                             if (arguments[2].isObject()) {
                                                                 auto buf = arguments[2].asObject(
                                                                         runtime);

                                                                 if (buf.isTypedArray(runtime)) {
                                                                     auto array = arguments[2].asObject(
                                                                             runtime).getTypedArray(
                                                                             runtime);
                                                                     auto buff = GetTypedArrayData<const uint8_t>(
                                                                             runtime, array);
                                                                     canvas_native_webgl_buffer_sub_data(
                                                                             target,
                                                                             static_cast<ssize_t>(offset),
                                                                             buff,
                                                                             this->GetState()
                                                                     );
                                                                 } else if (buf.isArrayBuffer(
                                                                         runtime)) {
                                                                     auto arrayBuffer = buf.getArrayBuffer(
                                                                             runtime);
                                                                     auto store = arrayBuffer.data(
                                                                             runtime);
                                                                     auto size = arrayBuffer.size(
                                                                             runtime);
                                                                     rust::Slice<const uint8_t> data(
                                                                             store, size);
                                                                     canvas_native_webgl_buffer_sub_data(
                                                                             target,
                                                                             static_cast<ssize_t>(offset),
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                 }

                                                             }

                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "checkFramebufferStatus") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {

                                                             if (arguments[0].isNumber()) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();

                                                                 auto ret = canvas_native_webgl_check_frame_buffer_status(
                                                                         target,
                                                                         this->GetState()
                                                                 );
                                                                 return {ret};
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clearColor") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 3) {
                                                             auto red = arguments[0].asNumber();
                                                             auto green = arguments[1].asNumber();
                                                             auto blue = arguments[2].asNumber();
                                                             auto alpha = arguments[3].asNumber();

                                                             canvas_native_webgl_clear_color(
                                                                     static_cast<float>(red),
                                                                     static_cast<float>(green),
                                                                     static_cast<float>(blue),
                                                                     static_cast<float>(alpha),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clearDepth") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto depth = arguments[0].asNumber();

                                                             canvas_native_webgl_clear_depth(
                                                                     static_cast<float>(depth),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clearStencil") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto stencil = (int32_t) arguments[0].asNumber();
                                                             canvas_native_webgl_clear_stencil(
                                                                     stencil,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clear") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 0) {
                                                             auto mask = (uint32_t) arguments[0].asNumber();

                                                             canvas_native_webgl_clear(
                                                                     mask,
                                                                     this->GetState()
                                                             );

                                                             this->UpdateInvalidateState();
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "colorMask") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto red = arguments[0].asBool();
                                                             auto green = arguments[1].asBool();
                                                             auto blue = arguments[2].asBool();
                                                             auto alpha = arguments[3].asBool();

                                                             canvas_native_webgl_color_mask(
                                                                     red,
                                                                     green,
                                                                     blue,
                                                                     alpha,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "commit") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "compileShader") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto shader = arguments[0].asObject(
                                                                     runtime).asHostObject<WebGLShader>(
                                                                     runtime);
                                                             if (shader != nullptr) {
                                                                 canvas_native_webgl_compile_shader(
                                                                         shader->GetShader(),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blendEquation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    }


    return Value::undefined();
}


WebGLRenderingContext::~WebGLRenderingContext() {}

void WebGLRenderingContext::ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    std::string type("image/png");
    int quality = 92;
    auto typeVal = args[0];
    auto qualityVal = args[1];

    if (typeVal->IsStringObject()) {
        type = Helpers::ConvertFromV8String(isolate, typeVal.As<v8::StringObject>()->ValueOf());
    }

    if (typeVal->IsString()) {
        type = Helpers::ConvertFromV8String(isolate, typeVal);
    }

    if (qualityVal->IsNumberObject()) {
        quality = static_cast<int32_t>(qualityVal.As<v8::NumberObject>()->ValueOf());
    }

    if (qualityVal->IsNumber()) {
        quality = qualityVal->Int32Value(context).FromMaybe(quality);
    }

    auto data = canvas_native_webgl_to_data_url(ptr->GetState(),
                                                rust::Str(type.c_str(), type.size()),
                                                quality);
    args.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(data.c_str(), data.size())));
}

void WebGLRenderingContext::Resized(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    // auto width = args[0];
    // auto height = args[1];
    // width->NumberValue(context).FromMaybe(1)
    // width->NumberValue(context).FromMaybe(1))
    canvas_native_webgl_resized(ptr->GetState());
}

void WebGLRenderingContext::Init(v8::Isolate *isolate) {

    auto ctorFunc = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLRenderingContext"),
                ctorFunc->GetFunction(context).ToLocalChecked());

    auto funcTpl = v8::FunctionTemplate::New(isolate, &InstanceFromPointer);
    global->Set(context, Helpers::ConvertToV8String(isolate, "__getWebGLRenderingContext"),
                funcTpl->GetFunction(context).ToLocalChecked());
}

void
WebGLRenderingContext::GetSuppressLogs(v8::Local<v8::String> name,
                                       const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(ptr->supressLogs_);
}

void WebGLRenderingContext::SetSuppressLogs(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                            const v8::PropertyCallbackInfo<void> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    ptr->supressLogs_ = value->BooleanValue(isolate);
}


void WebGLRenderingContext::InstanceFromPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
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
            version = Helpers::ConvertFromV8String(isolate, versionValue->ToString(
                    context).ToLocalChecked());
        }

        auto alphaMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "alpha"));
        if (!alphaMaybe.IsEmpty()) {
            auto alphaLocal = alphaMaybe.ToLocalChecked();
            if (Helpers::IsBoolean(alphaLocal)) {
                alpha = alphaLocal->BooleanValue(isolate);
            }
        }

        auto antialiasMaybe = config->Get(context,
                                          Helpers::ConvertToV8String(isolate, "antialias"));
        if (!antialiasMaybe.IsEmpty()) {
            auto antialiasLocal = antialiasMaybe.ToLocalChecked();
            if (Helpers::IsBoolean(antialiasLocal)) {
                antialias = antialiasLocal->BooleanValue(isolate);
            }
        }

        auto failIfMajorPerformanceCaveatMaybe = config->Get(context,
                                                             Helpers::ConvertToV8String(isolate,
                                                                                        "failIfMajorPerformanceCaveat"));
        if (!failIfMajorPerformanceCaveatMaybe.IsEmpty()) {
            auto failIfMajorPerformanceCaveatLocal = failIfMajorPerformanceCaveatMaybe.ToLocalChecked();
            if (failIfMajorPerformanceCaveatLocal->IsBoolean()) {
                fail_if_major_performance_caveat = failIfMajorPerformanceCaveatLocal->BooleanValue(
                        isolate);
            }
        }

        auto powerPreferenceMaybe = config->Get(context, Helpers::ConvertToV8String(isolate,
                                                                                    "powerPreference"));
        if (!powerPreferenceMaybe.IsEmpty()) {
            auto powerPreferenceLocal = powerPreferenceMaybe.ToLocalChecked();
            if (powerPreferenceLocal->IsString()) {
                power_preference = Helpers::ConvertFromV8String(isolate,
                                                                powerPreferenceLocal);
            }
        }

        auto premultipliedAlphaMaybe = config->Get(context, Helpers::ConvertToV8String(isolate,
                                                                                       "premultipliedAlpha"));
        if (!premultipliedAlphaMaybe.IsEmpty()) {
            auto premultipliedAlphaLocal = premultipliedAlphaMaybe.ToLocalChecked();
            if (premultipliedAlphaLocal->IsBoolean()) {
                premultiplied_alpha = premultipliedAlphaLocal->BooleanValue(isolate);
            }
        }

        auto preserveDrawingBufferMaybe = config->Get(context,
                                                      Helpers::ConvertToV8String(isolate,
                                                                                 "preserveDrawingBuffer"));
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

        auto desynchronizedMaybe = config->Get(context, Helpers::ConvertToV8String(isolate,
                                                                                   "desynchronized"));
        if (!desynchronizedMaybe.IsEmpty()) {
            auto desynchronizedLocal = desynchronizedMaybe.ToLocalChecked();
            if (desynchronizedLocal->IsBoolean()) {
                desynchronized = desynchronizedLocal->BooleanValue(isolate);
            }
        }

        auto xrCompatibleMaybe = config->Get(context,
                                             Helpers::ConvertToV8String(isolate, "xrCompatible"));
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

            WebGLRenderingContext *renderingContext = nullptr;

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
                renderingContext = new WebGLRenderingContext(std::move(ctx));

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

                renderingContext = new WebGLRenderingContext(std::move(ctx));
            }

            auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(renderingContext));
            auto raf_callback = new OnRafCallback(ctx_ptr, 1);
            auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
            auto raf = canvas_native_raf_create(raf_callback_ptr);
            renderingContext->SetRaf(
                    std::make_shared<RafImpl>(raf_callback, raf_callback_ptr, std::move(raf)));

            auto _raf = renderingContext->GetRaf();
            canvas_native_raf_start(_raf->GetRaf());

            auto tmpl = GetCtor(isolate)->InstanceTemplate();
            tmpl->SetInternalFieldCount(1);
            auto ret = tmpl->NewInstance(context).ToLocalChecked();
            Helpers::SetInstanceType(isolate, ret, ObjectType::WebGLRenderingContext);
            AddWeakListener(isolate, ret, renderingContext);

            args.GetReturnValue().Set(ret);
        }
        return;
    }
    args.GetReturnValue().SetUndefined();
}

void WebGLRenderingContext::CompressedTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 6) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto width = args[3];
        auto height = args[4];
        auto border = args[5];

        canvas_native_webgl_compressed_tex_image2d_none(
                target->Uint32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                internalformat->Uint32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                border->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    } else if (args.Length() > 6) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto width = args[3];
        auto height = args[4];
        auto border = args[5];
        auto pixels = args[6];
        if (pixels->IsArrayBufferView()) {
            auto buf = Helpers::GetTypedArrayData<const uint8_t>(pixels.As<v8::TypedArray>());
            canvas_native_webgl_compressed_tex_image2d(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Uint32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
        } else if (pixels->IsArrayBuffer()) {
            auto ab = pixels.As<v8::ArrayBuffer>();
            auto store = ab->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data());
            rust::Slice<const uint8_t> buf(data, store->ByteLength());

            canvas_native_webgl_compressed_tex_image2d(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Uint32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
        }
    }
}

void
WebGLRenderingContext::CompressedTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 7) {
        auto target = args[0];
        auto level = args[1];
        auto xoffset = args[2];
        auto yoffset = args[3];
        auto width = args[4];
        auto height = args[5];
        auto format = args[6];
        auto pixels = args[7];
        if (pixels->IsArrayBufferView()) {
            auto buf = Helpers::GetTypedArrayData<const uint8_t>(pixels.As<v8::TypedArray>());
            canvas_native_webgl_compressed_tex_sub_image2d(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
        } else if (pixels->IsArrayBuffer()) {
            auto buffer = pixels.As<v8::ArrayBuffer>();
            auto store = buffer->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data());
            rust::Slice<const uint8_t> buf(data, store->ByteLength());

            canvas_native_webgl_compressed_tex_sub_image2d(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::CopyTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 7) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto x = args[3];
        auto y = args[4];
        auto width = args[5];
        auto height = args[6];
        auto border = args[7];

        canvas_native_webgl_copy_tex_image2d(
                target->Uint32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                internalformat->Uint32Value(context).ToChecked(),
                x->Int32Value(context).ToChecked(),
                y->Int32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                border->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::CopyTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 7) {
        auto target = args[0];
        auto level = args[1];
        auto xoffset = args[2];
        auto yoffset = args[3];
        auto x = args[4];
        auto y = args[5];
        auto width = args[6];
        auto height = args[7];

        canvas_native_webgl_copy_tex_sub_image2d(
                target->Uint32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                xoffset->Int32Value(context).ToChecked(),
                yoffset->Int32Value(context).ToChecked(),
                x->Int32Value(context).ToChecked(),
                y->Int32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::CreateBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto buffer = canvas_native_webgl_create_buffer(ptr->GetState());
    if (buffer == 0) {
        args.GetReturnValue().SetUndefined();
    } else {
        args.GetReturnValue().Set(WebGLBuffer::NewInstance(isolate, buffer));
    }
}

void WebGLRenderingContext::CreateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl_create_framebuffer(ptr->GetState());
    if (ret == 0) {
        args.GetReturnValue().Set(v8::Undefined(isolate));
        return;
    }
    args.GetReturnValue().Set(WebGLFramebuffer::NewInstance(isolate, ret));
}

void WebGLRenderingContext::CreateProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl_create_program(ptr->GetState());
    if (ret == 0) {
        args.GetReturnValue().Set(v8::Undefined(isolate));
        return;
    }
    args.GetReturnValue().Set(WebGLProgram::NewInstance(isolate, ret));
}

void WebGLRenderingContext::CreateRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl_create_renderbuffer(ptr->GetState());
    if (ret == 0) {
        args.GetReturnValue().Set(v8::Undefined(isolate));
        return;
    }
    args.GetReturnValue().Set(WebGLRenderbuffer::NewInstance(isolate, ret));
}

void WebGLRenderingContext::CreateShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 0) {
        args.GetReturnValue().SetUndefined();
        return;
    }
    if (args[0]->IsNumber()) {
        // todo throw ?
    }
    auto type = args[0]->Uint32Value(context).ToChecked();
    auto ret = canvas_native_webgl_create_shader(type, ptr->GetState());
    if (ret == 0) {
        args.GetReturnValue().SetUndefined();
        return;
    }
    args.GetReturnValue().Set(WebGLShader::NewInstance(isolate, ret));
}

void WebGLRenderingContext::CreateTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl_create_texture(ptr->GetState());
    if (ret == 0) {
        args.GetReturnValue().SetUndefined();
        return;
    }
    args.GetReturnValue().Set(WebGLTexture::NewInstance(isolate, ret));
}

void WebGLRenderingContext::CullFace(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto mode = args[0];

        canvas_native_webgl_cull_face(
                mode->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DeleteBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto buffer = args[0];
        if (Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
            auto instance = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_delete_buffer(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::DeleteFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto buffer = args[0];
        if (Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLFramebuffer) {
            auto instance = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_delete_framebuffer(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::DeleteProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto program = args[0];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_delete_framebuffer(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::DeleteRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto buffer = args[0];
        if (Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLRenderbuffer) {
            auto instance = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_delete_renderbuffer(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::DeleteShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto shader = args[0];
        if (Helpers::GetInstanceType(isolate, shader) == ObjectType::WebGLRenderbuffer) {
            auto instance = Helpers::GetPrivate(isolate, shader.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_delete_shader(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::DeleteTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto texture = args[0];
        if (Helpers::GetInstanceType(isolate, texture) == ObjectType::WebGLTexture) {
            auto instance = Helpers::GetPrivate(isolate, texture.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_delete_texture(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::DepthFunc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto func = args[0];

        canvas_native_webgl_depth_func(
                func->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DepthMask(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto mask = args[0];

        canvas_native_webgl_depth_mask(
                mask->BooleanValue(isolate),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DepthRange(const v8::FunctionCallbackInfo<v8::Value> &args) {

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto zNear = args[0];
        auto zFar = args[1];

        canvas_native_webgl_depth_range(
                static_cast<float>(zNear->NumberValue(context).ToChecked()),
                static_cast<float>(zFar->NumberValue(context).ToChecked()),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DetachShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        auto shader = args[1];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram &&
            Helpers::GetInstanceType(isolate, shader) == ObjectType::WebGLShader) {
            auto p = program.As<v8::Object>();
            auto s = shader.As<v8::Object>();
            auto programValue = Helpers::GetPrivate(isolate, p, "instance")->ToUint32(context);
            auto shaderValue = Helpers::GetPrivate(isolate, s, "instance")->ToUint32(context);
            canvas_native_webgl_detach_shader(
                    programValue.ToLocalChecked()->Value(),
                    shaderValue.ToLocalChecked()->Value(),
                    ptr->GetState()
            );
        }
    }
}

void
WebGLRenderingContext::DisableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto index = args[0];

        canvas_native_webgl_disable_vertex_attrib_array(
                index->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::Disable(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto cap = args[0];

        canvas_native_webgl_disable(
                cap->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::DrawArrays(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto mode = args[0];
        auto first = args[1];
        auto count = args[2];

        canvas_native_webgl_draw_arrays(
                mode->Uint32Value(context).ToChecked(),
                first->Int32Value(context).ToChecked(),
                count->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
        ptr->UpdateInvalidateState();
    }
}

void WebGLRenderingContext::DrawElements(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 3) {
        auto mode = args[0];
        auto count = args[1];
        auto type = args[2];
        auto offset = args[3];

        canvas_native_webgl_draw_elements(
                mode->Uint32Value(context).ToChecked(),
                count->Int32Value(context).ToChecked(),
                type->Uint32Value(context).ToChecked(),
                static_cast<ssize_t>(offset->IntegerValue(context).ToChecked()),
                ptr->GetState()
        );
        ptr->UpdateInvalidateState();
    }
}

void
WebGLRenderingContext::EnableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto index = args[0];

        canvas_native_webgl_enable_vertex_attrib_array(
                index->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::Enable(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto cap = args[0];

        canvas_native_webgl_enable(
                cap->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::Finish(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    canvas_native_webgl_finish(
            ptr->GetState()
    );
}

void WebGLRenderingContext::Flush(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    canvas_native_webgl_flush(
            ptr->GetState()
    );
}

void
WebGLRenderingContext::FramebufferRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 4) {
        auto target = args[0];
        auto attachment = args[1];
        auto renderbuffertarget = args[2];
        auto renderbuffer = args[3];

        if (Helpers::GetInstanceType(isolate, renderbuffer) == ObjectType::WebGLRenderbuffer) {
            auto instance = Helpers::GetPrivate(isolate, renderbuffer.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_framebuffer_renderbuffer(
                        target->Uint32Value(context).ToChecked(),
                        attachment->Uint32Value(context).ToChecked(),
                        renderbuffertarget->Uint32Value(context).ToChecked(),
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::FramebufferTexture2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 5) {
        auto target = args[0];
        auto attachment = args[1];
        auto textarget = args[2];
        auto texture = args[3];
        auto level = args[4];
        if (Helpers::GetInstanceType(isolate, texture) == ObjectType::WebGLTexture) {
            auto instance = Helpers::GetPrivate(isolate, texture.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_framebuffer_texture2d(
                        target->Uint32Value(context).ToChecked(),
                        attachment->Uint32Value(context).ToChecked(),
                        textarget->Uint32Value(context).ToChecked(),
                        instance->Uint32Value(context).ToChecked(),
                        level->Int32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::FrontFace(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto mode = args[0];

        canvas_native_webgl_front_face(
                mode->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::GenerateMipmap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto target = args[0];

        canvas_native_webgl_generate_mipmap(
                target->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::GetActiveAttrib(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        auto index = args[1];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto info = canvas_native_webgl_get_active_attrib(
                        instance->Uint32Value(context).ToChecked(),
                        index->Int32Value(context).ToChecked(),
                        ptr->GetState()
                );
                auto ret = WebGLActiveInfoImpl::NewInstance(isolate, std::move(info));
                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }
    args.GetReturnValue().SetUndefined();
}

void WebGLRenderingContext::GetActiveUniform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        auto index = args[1];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto info = canvas_native_webgl_get_active_uniform(
                        instance->Uint32Value(context).ToChecked(),
                        index->Int32Value(context).ToChecked(),
                        ptr->GetState()
                );
                auto ret = WebGLActiveInfoImpl::NewInstance(isolate, std::move(info));
                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }
    args.GetReturnValue().SetUndefined();
}

void WebGLRenderingContext::GetAttachedShaders(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto info = canvas_native_webgl_get_attached_shaders(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
                auto len = info.size();
                auto array = v8::Array::New(isolate, len);
                for (int i = 0; i < len; ++i) {
                    auto shader = info[i];
                    array->Set(context, i,
                               WebGLShader::NewInstance(isolate, shader));
                }
                args.GetReturnValue().Set(array);
                return;
            }
        }
    }
    args.GetReturnValue().Set(v8::Array::New(isolate, 0));
}

void WebGLRenderingContext::GetAttribLocation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        auto name = args[1];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto nameValue = Helpers::ConvertFromV8String(isolate, name);

                auto location = canvas_native_webgl_get_attrib_location(
                        instance->Uint32Value(context).ToChecked(),
                        rust::Str(nameValue.c_str(), nameValue.size()),
                        ptr->GetState()
                );
                args.GetReturnValue().Set(location);
                return;
            }
        }
    }
    args.GetReturnValue().Set(-1);
}

void WebGLRenderingContext::GetBufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto target = args[0];
        auto pname = args[1];

        auto param = canvas_native_webgl_get_buffer_parameter(
                target->Uint32Value(context).ToChecked(),
                pname->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );

        args.GetReturnValue().Set(param);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void WebGLRenderingContext::GetContextAttributes(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto attr = canvas_native_webgl_get_context_attributes(ptr->GetState());
    auto ret = v8::Object::New(isolate);
    auto alpha = canvas_native_webgl_context_attribute_get_get_alpha(*attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "alpha"),
             v8::Boolean::New(isolate, alpha));

    auto antialias = canvas_native_webgl_context_attribute_get_get_antialias(*attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "antialias"),
             v8::Boolean::New(isolate, antialias));

    auto depth = canvas_native_webgl_context_attribute_get_get_depth(*attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "depth"),
             v8::Boolean::New(isolate, depth));

    auto fail_if_major_performance_caveat = canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
            *attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "failIfMajorPerformanceCaveat"),
             v8::Boolean::New(isolate, fail_if_major_performance_caveat));

    auto power_preference = canvas_native_webgl_context_attribute_get_get_power_preference(*attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "powerPreference"),
             Helpers::ConvertToV8String(isolate, power_preference.c_str()));

    auto premultiplied_alpha = canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
            *attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "premultipliedAlpha"),
             v8::Boolean::New(isolate, premultiplied_alpha));

    auto preserve_drawing_buffer = canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
            *attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "preserveDrawingBuffer"),
             v8::Boolean::New(isolate, preserve_drawing_buffer));

    auto stencil = canvas_native_webgl_context_attribute_get_get_stencil(*attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "stencil"),
             v8::Boolean::New(isolate, stencil));

    auto desynchronized = canvas_native_webgl_context_attribute_get_get_desynchronized(*attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "desynchronized"),
             v8::Boolean::New(isolate, desynchronized));

    auto xr_compatible = canvas_native_webgl_context_attribute_get_get_xr_compatible(*attr);
    ret->Set(context, Helpers::ConvertToV8String(isolate, "xrCompatible"),
             v8::Boolean::New(isolate, xr_compatible));

    args.GetReturnValue().Set(ret);
}

void WebGLRenderingContext::GetError(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl_get_error(ptr->GetState());
    args.GetReturnValue().Set(ret);
}


void WebGLRenderingContext::GetExtension(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 0) {
        args.GetReturnValue().SetNull();
        return;
    }
    auto name = args[0];
    if (!name->IsString()) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto val = Helpers::ConvertFromV8String(isolate, name);
    auto ext = canvas_native_webgl_get_extension(rust::Str(val.c_str(), val.size()),
                                                 ptr->GetState());

    if (canvas_native_webgl_context_extension_is_none(*ext)) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto type = canvas_native_webgl_context_extension_get_type(*ext);
    switch (type) {
        case WebGLExtensionType::EXT_blend_minmax:
            args.GetReturnValue().Set(EXT_blend_minmaxImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::EXT_color_buffer_half_float:
            args.GetReturnValue().Set(EXT_color_buffer_half_floatImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::EXT_disjoint_timer_query: {
            auto ret = canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
                    std::move(ext));
            args.GetReturnValue().Set(
                    EXT_disjoint_timer_queryImpl::NewInstance(isolate, std::move(ret)));
        }
            break;
        case WebGLExtensionType::EXT_sRGB:
            args.GetReturnValue().Set(EXT_sRGBImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::EXT_shader_texture_lod:
            args.GetReturnValue().Set(EXT_shader_texture_lodImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::EXT_texture_filter_anisotropic:
            args.GetReturnValue().Set(EXT_texture_filter_anisotropicImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::OES_element_index_uint:
            args.GetReturnValue().Set(OES_element_index_uintImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::OES_standard_derivatives:
            args.GetReturnValue().Set(OES_standard_derivativesImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::OES_texture_float:
            args.GetReturnValue().Set(OES_texture_floatImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::OES_texture_float_linear:
            args.GetReturnValue().Set(OES_texture_float_linearImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::OES_texture_half_float:
            args.GetReturnValue().Set(OES_texture_half_floatImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::OES_texture_half_float_linear:
            args.GetReturnValue().Set(OES_texture_half_float_linearImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::OES_vertex_array_object: {
            auto ret = canvas_native_webgl_context_extension_to_oes_vertex_array_object(
                    std::move(ext));
            args.GetReturnValue().Set(
                    OES_vertex_array_objectImpl::NewInstance(isolate, std::move(ret)));
        }
            break;
        case WebGLExtensionType::WEBGL_color_buffer_float:
            args.GetReturnValue().Set(WEBGL_color_buffer_floatImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_compressed_texture_atc:
            args.GetReturnValue().Set(WEBGL_compressed_texture_atcImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_compressed_texture_etc1:
            args.GetReturnValue().Set(WEBGL_compressed_texture_etc1Impl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_compressed_texture_s3tc:
            args.GetReturnValue().Set(WEBGL_compressed_texture_s3tcImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb:
            args.GetReturnValue().Set(WEBGL_compressed_texture_s3tc_srgbImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_compressed_texture_etc:
            args.GetReturnValue().Set(WEBGL_compressed_texture_etcImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_compressed_texture_pvrtc:
            args.GetReturnValue().Set(WEBGL_compressed_texture_pvrtcImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_lose_context: {
            auto ret = canvas_native_webgl_context_extension_to_lose_context(std::move(ext));
            args.GetReturnValue().Set(WEBGL_lose_contextImpl::NewInstance(isolate, std::move(ret)));
        }
            break;
        case WebGLExtensionType::ANGLE_instanced_arrays: {
            auto ret = canvas_native_webgl_context_extension_to_angle_instanced_arrays(
                    std::move(ext));
            args.GetReturnValue().Set(
                    ANGLE_instanced_arraysImpl::NewInstance(isolate, std::move(ret)));
        }
            break;
        case WebGLExtensionType::WEBGL_depth_texture:
            args.GetReturnValue().Set(WEBGL_depth_textureImpl::NewInstance(isolate));
            break;
        case WebGLExtensionType::WEBGL_draw_buffers: {
            auto ret = canvas_native_webgl_context_extension_to_draw_buffers(std::move(ext));
            args.GetReturnValue().Set(WEBGL_draw_buffersImpl::NewInstance(isolate, std::move(ret)));
        }
            break;
        case WebGLExtensionType::None:
            args.GetReturnValue().SetUndefined();
            break;
    }
}

void WebGLRenderingContext::GetFramebufferAttachmentParameter(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    if (args.Length() > 2) {
        auto target = args[0];
        auto attachment = args[1];
        auto pname = args[2];
        auto ret = canvas_native_webgl_get_framebuffer_attachment_parameter(
                target->Uint32Value(context).ToChecked(),
                attachment->Uint32Value(context).ToChecked(),
                pname->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
        if (canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(*ret)) {
            auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(*ret);
            args.GetReturnValue().Set(WebGLTexture::NewInstance(isolate, value));
            return;
        }
        if (canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(*ret)) {
            auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(*ret);
            args.GetReturnValue().Set(WebGLRenderbuffer::NewInstance(isolate, value));
            return;
        }

        args.GetReturnValue().Set(
                canvas_native_webgl_framebuffer_attachment_parameter_get_value(*ret)
        );
    }
}

void WebGLRenderingContext::GetParameterInternal(const v8::FunctionCallbackInfo<v8::Value> &args,
                                                 uint32_t pnameValue,
                                                 rust::Box<WebGLResult> result) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
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
                args.GetReturnValue().SetNull();
                break;
            }
            args.GetReturnValue().Set(value);
        }
            break;
        case UNPACK_COLORSPACE_CONVERSION_WEBGL:
            args.GetReturnValue().Set(
                    canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
                            ptr->GetState())
            );

            break;
        case GL_ALIASED_LINE_WIDTH_RANGE:
        case GL_ALIASED_POINT_SIZE_RANGE:
        case GL_DEPTH_RANGE: {
            auto ret = canvas_native_webgl_result_get_f32_array(*result);
            auto len = ret.size();
            auto byte_len = len * sizeof(float);
            auto buffer = v8::ArrayBuffer::New(isolate, byte_len);
            auto view = v8::Float32Array::New(buffer, 0, len);
            for (int j = 0; j < len; ++j) {
                view->Set(context, j,
                          v8::Number::New(isolate, static_cast<double>(ret[j])));
            }
            args.GetReturnValue().Set(view);
        }
            break;
        case GL_BLEND_COLOR:
        case GL_COLOR_CLEAR_VALUE: {
            auto ret = canvas_native_webgl_result_get_f32_array(*result);
            auto len = ret.size();
            auto byte_len = len * sizeof(float);
            auto buffer = v8::ArrayBuffer::New(isolate, byte_len);
            auto view = v8::Float32Array::New(buffer, 0, len);
            for (int j = 0; j < len; ++j) {
                view->Set(context, j,
                          v8::Number::New(isolate, static_cast<double>(ret[j])));
            }
            args.GetReturnValue().Set(view);
        }
            break;
        case UNPACK_FLIP_Y_WEBGL:
            args.GetReturnValue().Set(canvas_native_webgl_state_get_flip_y(ptr->GetState()));
            break;
        case UNPACK_PREMULTIPLY_ALPHA_WEBGL:
            args.GetReturnValue().Set(
                    canvas_native_webgl_state_get_premultiplied_alpha(ptr->GetState()));
            break;
        case GL_BLEND:
        case GL_CULL_FACE:
        case GL_DEPTH_TEST:
        case GL_DEPTH_WRITEMASK:
        case GL_DITHER:
        case GL_POLYGON_OFFSET_FILL:
        case GL_SAMPLE_COVERAGE_INVERT:
        case GL_SCISSOR_TEST:
        case GL_STENCIL_TEST: {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(*result));
        }
            break;
        case GL_COLOR_WRITEMASK: {
            auto ret = canvas_native_webgl_result_get_bool_array(*result);
            auto len = ret.size();
            auto array = v8::Array::New(isolate, len);
            for (int j = 0; j < len; ++j) {
                array->Set(context, j, v8::Boolean::New(isolate, ret[j] == 1));
            }
            args.GetReturnValue().Set(array);
        }
            break;
        case GL_COMPRESSED_TEXTURE_FORMATS:
        case GL_MAX_VIEWPORT_DIMS:
        case GL_SCISSOR_BOX:
        case GL_VIEWPORT: {
            auto ret = canvas_native_webgl_result_get_i32_array(*result);
            auto len = ret.size();
            auto byte_len = len * sizeof(int32_t);
            Helpers::LogToConsole("GL_VIEWPORT");
            Helpers::LogToConsole("len: " + std::to_string(len));
            Helpers::LogToConsole("byte_len: " + std::to_string(byte_len));

            auto buffer = v8::ArrayBuffer::New(isolate, byte_len);
            auto view = v8::Int32Array::New(buffer, 0, len);
            for (int j = 0; j < len; ++j) {
                Helpers::LogToConsole(
                        "Set: index " + std::to_string(j) + " value: " + std::to_string(ret[j]));
                view->Set(context, j, v8::Int32::New(isolate, ret[j]));
            }
            args.GetReturnValue().Set(view);
        }
            break;
        case GL_DEPTH_CLEAR_VALUE:
        case GL_LINE_WIDTH:
        case GL_POLYGON_OFFSET_FACTOR:
        case GL_POLYGON_OFFSET_UNITS:
        case GL_SAMPLE_COVERAGE_VALUE: {
            args.GetReturnValue().Set(
                    static_cast<double>(canvas_native_webgl_result_get_f32(*result))
            );
        }
            break;
        case GL_RENDERER:
        case GL_SHADING_LANGUAGE_VERSION:
        case GL_VENDOR:
        case GL_VERSION: {
            auto ret = canvas_native_webgl_result_get_string(*result);
            args.GetReturnValue().Set(
                    Helpers::ConvertToV8String(isolate, std::string(ret.data(), ret.size())));
        }
            break;
        default:
            args.GetReturnValue().SetNull();
            break;

    }
}

void WebGLRenderingContext::GetParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    // TODO remove extra allocations
    if (args.Length() > 0) {
        auto pname = args[0];
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto result = canvas_native_webgl_get_parameter(pnameValue,
                                                        ptr->GetState());
        Helpers::LogToConsole("GetParameter: " + std::to_string(pnameValue));

        GetParameterInternal(args, pnameValue, std::move(result));
        return;
    }
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetProgramInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto program = args[0];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto log = canvas_native_webgl_get_program_info_log(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );


                if (log.empty()) {
                    return args.GetReturnValue().SetEmptyString();
                }

                args.GetReturnValue().Set(
                        Helpers::ConvertToV8String(isolate, std::string(log.data(), log.size())));
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, ""));
}

void WebGLRenderingContext::GetProgramParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        auto pname = args[1];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_get_program_parameter(
                        instance->Uint32Value(context).ToChecked(),
                        pname->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );

                if (canvas_native_webgl_result_get_is_none(*ret)) {
                    args.GetReturnValue().SetNull();
                    return;
                }
                switch (pname->Uint32Value(context).ToChecked()) {
                    case GL_DELETE_STATUS:
                    case GL_LINK_STATUS:
                    case GL_VALIDATE_STATUS:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(*ret));
                        break;
                    default:
                        args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(*ret));
                        break;
                }

                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::GetRenderbufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto target = args[0];
        auto pname = args[1];
        auto ret = canvas_native_webgl_get_renderbuffer_parameter(
                target->Uint32Value(context).ToChecked(),
                pname->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
        args.GetReturnValue().Set(ret);
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetShaderInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto shader = args[0];
        if (Helpers::GetInstanceType(isolate, shader) == ObjectType::WebGLShader) {
            auto instance = Helpers::GetPrivate(isolate, shader.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto log = canvas_native_webgl_get_shader_info_log(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );

                args.GetReturnValue().Set(
                        Helpers::ConvertToV8String(isolate, std::string(log.data(), log.size())));
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetEmptyString();
}

void WebGLRenderingContext::GetShaderParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto shader = args[0];
        auto pname = args[1];
        if (Helpers::GetInstanceType(isolate, shader) == ObjectType::WebGLShader) {
            auto pnameValue = pname->Uint32Value(context).ToChecked();
            auto instance = Helpers::GetPrivate(isolate, shader.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_get_shader_parameter(
                        instance->Uint32Value(context).ToChecked(),
                        pnameValue,
                        ptr->GetState()
                );

                if (canvas_native_webgl_result_get_is_none(*ret)) {
                    args.GetReturnValue().Set(v8::Null(isolate));
                    return;
                }

                if (pnameValue == GL_DELETE_STATUS || pnameValue == GL_COMPILE_STATUS) {
                    args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(*ret));
                    return;
                }

                args.GetReturnValue().Set(
                        canvas_native_webgl_result_get_i32(*ret));
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void
WebGLRenderingContext::GetShaderPrecisionFormat(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto shaderType = args[0];
        auto precisionType = args[1];
        auto ret = canvas_native_webgl_get_shader_precision_format(
                shaderType->Uint32Value(context).ToChecked(),
                precisionType->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
        args.GetReturnValue().Set(
                WebGLShaderPrecisionFormatImpl::NewInstance(isolate, std::move(ret)));
        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();

}

void WebGLRenderingContext::GetShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto shader = args[0];
        if (Helpers::GetInstanceType(isolate, shader) == ObjectType::WebGLShader) {
            auto instance = Helpers::GetPrivate(isolate, shader.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto source = canvas_native_webgl_get_shader_source(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );

                args.GetReturnValue().Set(
                        Helpers::ConvertToV8String(isolate,
                                                   std::string(source.data(), source.size())));
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetEmptyString();
}

void
WebGLRenderingContext::GetSupportedExtensions(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto exts = canvas_native_webgl_get_supported_extensions(ptr->GetState());
    auto len = exts.size();
    auto array = v8::Array::New(isolate, len);
    for (int i = 0; i < len; ++i) {
        auto item = exts[i];
        array->Set(context, i, Helpers::ConvertToV8String(isolate, item.c_str()));
    }
    args.GetReturnValue().Set(array);
}


void
WebGLRenderingContext::GetSupportedExtensionsString(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointerBase(args.This());
    auto exts = canvas_native_webgl_get_supported_extensions_to_string(ptr->GetState());
    args.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(
                    exts.data(), exts.size()
            ))
    );
}

void WebGLRenderingContext::GetTexParameter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto target = args[0];
        auto pname = args[1];
        auto ret = canvas_native_webgl_get_tex_parameter(
                target->Uint32Value(context).ToChecked(),
                pname->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
        args.GetReturnValue().Set(ret);
        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetUniformLocation(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        auto name = args[1];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {

                auto nameValue = Helpers::ConvertFromV8String(isolate, name);

                auto ret = canvas_native_webgl_get_uniform_location(
                        instance->Uint32Value(context).ToChecked(),
                        rust::Str(nameValue.c_str(), nameValue.size()),
                        ptr->GetState()
                );

                args.GetReturnValue().Set(WebGLUniformLocation::NewInstance(isolate, ret));
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetUniform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto program = args[0];
        auto location = args[1];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram &&
            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto programInstance = Helpers::GetPrivate(isolate, program.As<v8::Object>(),
                                                       "instance");
            auto locationInstance = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                        "instance");
            if (!programInstance.IsEmpty() && !locationInstance.IsEmpty()) {

                auto val = canvas_native_webgl_get_uniform(
                        programInstance->Uint32Value(context).ToChecked(),
                        location->Uint32Value(context).ToChecked(),
                        ptr->GetState());
                switch (canvas_native_webgl_result_get_type(*val)) {
                    case WebGLResultType::Boolean:
                        args.GetReturnValue().Set(
                                canvas_native_webgl_result_get_bool(*val)
                        );
                        break;
                    case WebGLResultType::None:
                        args.GetReturnValue().Set(v8::Null(isolate));
                        break;
                    case WebGLResultType::String: {
                        auto str = canvas_native_webgl_result_get_string(*val);
                        args.GetReturnValue().Set(
                                Helpers::ConvertToV8String(isolate,
                                                           std::string(str.data(), str.size()))
                        );
                    }
                        break;
                    case WebGLResultType::BooleanArray: {
                        auto ret = canvas_native_webgl_result_get_bool_array(*val);
                        auto len = ret.size();
                        auto array = v8::Array::New(isolate, len);
                        for (int i = 0; i < len; ++i) {
                            auto item = ret[i];
                            array->Set(context, i,
                                       v8::Boolean::New(isolate, item == 1));
                        }
                        args.GetReturnValue().Set(array);
                    }
                        break;
                    case WebGLResultType::F32Array: {
                        auto ret = canvas_native_webgl_result_get_f32_array(*val);
                        auto len = ret.size();
                        auto byte_len = len * sizeof(float);
                        auto array = v8::ArrayBuffer::New(isolate, byte_len);
                        auto view = v8::Float32Array::New(array, 0, len);
                        for (int i = 0; i < len; ++i) {
                            auto item = ret[i];
                            view->Set(context, i,
                                      v8::Number::New(isolate, static_cast<double>(item)));
                        }
                        args.GetReturnValue().Set(view);
                    }
                        break;
                    case WebGLResultType::I32Array: {
                        auto ret = canvas_native_webgl_result_get_i32_array(*val);
                        auto len = ret.size();
                        auto byte_len = len * sizeof(int32_t);
                        auto array = v8::ArrayBuffer::New(isolate, byte_len);
                        auto view = v8::Int32Array::New(array, 0, len);
                        for (int i = 0; i < len; ++i) {
                            auto item = ret[i];
                            view->Set(context, i, v8::Int32::New(isolate, item));
                        }
                        args.GetReturnValue().Set(view);
                    }
                        break;
                    case WebGLResultType::U32Array: {
                        auto ret = canvas_native_webgl_result_get_u32_array(*val);
                        auto len = ret.size();
                        auto byte_len = len * sizeof(uint32_t);
                        auto array = v8::ArrayBuffer::New(isolate, byte_len);
                        auto view = v8::Uint32Array::New(array, 0, len);
                        for (int i = 0; i < len; ++i) {
                            auto item = ret[i];
                            view->Set(context, i, v8::Uint32::New(isolate, item));
                        }
                        args.GetReturnValue().Set(view);
                    }
                        break;
                    case WebGLResultType::F32:
                        args.GetReturnValue().Set(
                                canvas_native_webgl_result_get_f32(*val)
                        );
                        break;
                    case WebGLResultType::I32:
                        args.GetReturnValue().Set(
                                canvas_native_webgl_result_get_i32(*val)
                        );
                        break;
                    case WebGLResultType::U32:
                        args.GetReturnValue().Set(
                                canvas_native_webgl_result_get_u32(*val)
                        );
                        break;
                }

                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetVertexAttribOffset(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto index = args[0];
        auto pname = args[1];
        auto ret = canvas_native_webgl_get_vertex_attrib_offset(
                index->Uint32Value(context).ToChecked(),
                pname->Uint32Value(context).ToChecked(),
                ptr->GetState());
        args.GetReturnValue().Set(v8::Number::New(isolate, static_cast<double>(ret)));
        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::GetVertexAttrib(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto index = args[0];
        auto pname = args[1];
        auto pnameValue = pname->Uint32Value(context).ToChecked();
        auto ret = canvas_native_webgl_get_vertex_attrib(index->Uint32Value(context).ToChecked(),
                                                         pnameValue, ptr->GetState());

        if (pnameValue == GL_CURRENT_VERTEX_ATTRIB) {
            auto val = canvas_native_webgl_result_get_f32_array(*ret);
            auto len = val.size();
            auto byte_len = len * sizeof(float);
            auto array = v8::ArrayBuffer::New(isolate, byte_len);
            auto view = v8::Float32Array::New(array, 0, len);
            for (int i = 0; i < len; ++i) {
                auto item = val[i];
                view->Set(context, i,
                          v8::Number::New(isolate, static_cast<double>(item)));
            }
            args.GetReturnValue().Set(view);
        } else if (pnameValue == GL_VERTEX_ATTRIB_ARRAY_ENABLED ||
                   pnameValue == GL_VERTEX_ATTRIB_ARRAY_NORMALIZED) {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(*ret));
        } else {
            args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(*ret));
        }

        return;
    }

    // todo check return
    args.GetReturnValue().SetNull();
}

void WebGLRenderingContext::Hint(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto target = args[0];
        auto mode = args[1];
        canvas_native_webgl_hint(target->Uint32Value(context).ToChecked(),
                                 mode->Uint32Value(context).ToChecked(),
                                 ptr->GetState());
    }
}

void WebGLRenderingContext::IsBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto buffer = args[0];
        if (Helpers::GetInstanceType(isolate, buffer) == ObjectType::WebGLBuffer) {
            auto instance = Helpers::GetPrivate(isolate, buffer.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_is_buffer(instance->Uint32Value(context).ToChecked(),
                                                         ptr->GetState());

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGLRenderingContext::IsContextLost(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    auto ret = canvas_native_webgl_get_is_context_lost(ptr->GetState());
    args.GetReturnValue().Set(ret);
}

void WebGLRenderingContext::IsEnabled(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto cap = args[0];
        auto ret = canvas_native_webgl_is_enabled(cap->Uint32Value(context).ToChecked(),
                                                  ptr->GetState());
        args.GetReturnValue().Set(ret);
        return;
    }
    args.GetReturnValue().Set(false);
}

void WebGLRenderingContext::IsFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto framebuffer = args[0];
        if (Helpers::GetInstanceType(isolate, framebuffer) == ObjectType::WebGLFramebuffer) {
            auto instance = Helpers::GetPrivate(isolate, framebuffer.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_is_framebuffer(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState());

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGLRenderingContext::IsProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto program = args[0];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_is_program(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState());

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGLRenderingContext::IsRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto renderbuffer = args[0];
        if (Helpers::GetInstanceType(isolate, renderbuffer) == ObjectType::WebGLRenderbuffer) {
            auto instance = Helpers::GetPrivate(isolate, renderbuffer.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_is_renderbuffer(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState());

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGLRenderingContext::IsShader(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto shader = args[0];
        if (Helpers::GetInstanceType(isolate, shader) == ObjectType::WebGLShader) {
            auto instance = Helpers::GetPrivate(isolate, shader.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_is_shader(instance->Uint32Value(context).ToChecked(),
                                                         ptr->GetState());

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGLRenderingContext::IsTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto texture = args[0];
        if (Helpers::GetInstanceType(isolate, texture) == ObjectType::WebGLTexture) {
            auto instance = Helpers::GetPrivate(isolate, texture.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                auto ret = canvas_native_webgl_is_texture(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState());

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    // todo check return
    args.GetReturnValue().Set(false);
}

void WebGLRenderingContext::LineWidth(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto width = args[0];
        canvas_native_webgl_line_width(static_cast<float>(width->NumberValue(context).ToChecked()),
                                       ptr->GetState());
    }
}

void WebGLRenderingContext::LinkProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto program = args[0];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_link_program(instance->Uint32Value(context).ToChecked(),
                                                 ptr->GetState());
            }
        }
    }
}

void WebGLRenderingContext::PixelStorei(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto pname = args[0];
        auto param = args[1];
        canvas_native_webgl_pixel_storei(
                pname->Uint32Value(context).ToChecked(),
                param->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::PolygonOffset(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto factor = args[0];
        auto units = args[1];
        canvas_native_webgl_polygon_offset(
                static_cast<float>(factor->NumberValue(context).ToChecked()),
                static_cast<float>(units->NumberValue(context).ToChecked()),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::ReadPixels(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 6) {
        auto x = args[0];
        auto y = args[1];
        auto width = args[2];
        auto height = args[3];
        auto format = args[4];
        auto type = args[5];
        auto pixels = args[6];

        if (pixels->IsArrayBufferView()) {

            auto slice = Helpers::GetTypedArrayData<uint8_t>(pixels.As<v8::TypedArray>());
            canvas_native_webgl_read_pixels_u8(
                    x->Int32Value(context).ToChecked(),
                    y->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Uint32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );
            return;
        }


        if (pixels->IsArrayBuffer()) {
            auto slice = Helpers::GetArrayBufferData(pixels.As<v8::ArrayBuffer>());
            canvas_native_webgl_read_pixels_u8(
                    x->Int32Value(context).ToChecked(),
                    y->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Uint32Value(context).ToChecked(),
                    slice,
                    ptr->GetState()
            );
            return;
        }
    }
}

void WebGLRenderingContext::RenderbufferStorage(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 3) {
        auto target = args[0];
        auto internalFormat = args[1];
        auto width = args[2];
        auto height = args[3];
        canvas_native_webgl_renderbuffer_storage(
                target->Uint32Value(context).ToChecked(),
                internalFormat->Uint32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::SampleCoverage(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto value = args[0];
        auto invert = args[1];
        canvas_native_webgl_sample_coverage(
                static_cast<float>(value->NumberValue(context).ToChecked()),
                invert->BooleanValue(isolate),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::Scissor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 3) {
        auto x = args[0];
        auto y = args[1];
        auto width = args[2];
        auto height = args[3];
        canvas_native_webgl_scissor(
                x->Int32Value(context).ToChecked(),
                y->Int32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::ShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto shader = args[0];
        auto source = args[1];

        if (Helpers::GetInstanceType(isolate, shader) == ObjectType::WebGLShader) {
            auto instance = Helpers::GetPrivate(isolate, shader.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {

                auto sourceValue = Helpers::ConvertFromV8String(isolate, source);

                canvas_native_webgl_shader_source(instance->Uint32Value(context).ToChecked(),
                                                  rust::Str(sourceValue.c_str(),
                                                            sourceValue.size()),
                                                  ptr->GetState());
            }
        }
    }
}

void WebGLRenderingContext::StencilFuncSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 3) {
        auto face = args[0];
        auto func = args[1];
        auto ref = args[2];
        auto mask = args[3];
        canvas_native_webgl_stencil_func_separate(
                face->Uint32Value(context).ToChecked(),
                func->Uint32Value(context).ToChecked(),
                ref->Uint32Value(context).ToChecked(),
                mask->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::StencilFunc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto func = args[0];
        auto ref = args[1];
        auto mask = args[2];
        canvas_native_webgl_stencil_func(
                func->Uint32Value(context).ToChecked(),
                ref->Uint32Value(context).ToChecked(),
                mask->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::StencilMaskSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto face = args[0];
        auto mask = args[1];
        canvas_native_webgl_stencil_mask_separate(
                face->Uint32Value(context).ToChecked(),
                mask->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::StencilMask(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto mask = args[0];
        canvas_native_webgl_stencil_mask(
                mask->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::StencilOpSeparate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 3) {
        auto face = args[0];
        auto fail = args[1];
        auto zfail = args[2];
        auto zpass = args[3];
        canvas_native_webgl_stencil_op_separate(
                face->Uint32Value(context).ToChecked(),
                fail->Uint32Value(context).ToChecked(),
                zfail->Uint32Value(context).ToChecked(),
                zpass->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::StencilOp(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto fail = args[0];
        auto zfail = args[1];
        auto zpass = args[2];
        canvas_native_webgl_stencil_op(
                fail->Uint32Value(context).ToChecked(),
                zfail->Uint32Value(context).ToChecked(),
                zpass->Uint32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::TexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // TODO tidy
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    if (args.Length() == 5) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto format = args[3];
        auto type = args[4];

        canvas_native_webgl_tex_image2d_image_none(
                target->Int32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                internalformat->Int32Value(context).ToChecked(),
                format->Int32Value(context).ToChecked(),
                type->Int32Value(context).ToChecked(),
                ptr->GetState()
        );

    } else if (args.Length() == 6) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto format = args[3];
        auto type = args[4];
        auto pixels = args[5];

        auto draw_asset = [&](v8::Local<v8::Value> &pixels) {
            auto asset = ImageAssetImpl::GetPointer(pixels->ToObject(context).ToLocalChecked());
            canvas_native_webgl_tex_image2d_asset(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    asset->GetImageAsset(),
                    ptr->GetState()
            );
        };

        auto draw_canvas2d = [&](v8::Local<v8::Value> &pixels) {
            auto canvas2d = CanvasRenderingContext2DImpl::GetPointer(pixels.As<v8::Object>());
            canvas_native_webgl_tex_image2d_canvas2d(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    canvas2d->GetContext(),
                    ptr->GetState()
            );
        };

        auto draw_webgl = [&](v8::Local<v8::Value> &pixels) {
            auto state = WebGLRenderingContextBase::GetPointerBase(pixels.As<v8::Object>());
            canvas_native_webgl_tex_image2d_webgl(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    state->GetState(),
                    ptr->GetState()
            );
        };

        if (Helpers::GetInstanceType(isolate, pixels) == ObjectType::ImageAsset) {
            draw_asset(pixels);
        } else if (Helpers::GetInstanceType(isolate, pixels) == ObjectType::ImageBitmap) {
            auto bitmap = ImageBitmapImpl::GetPointer(pixels.As<v8::Object>());
            canvas_native_webgl_tex_image2d_asset(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    bitmap->GetImageAsset(),
                    ptr->GetState()
            );
        } else {
            if (Helpers::IsObject(pixels)) {
                auto canvas = pixels->ToObject(context).ToLocalChecked();

                v8::Local<v8::Value> ctx;
                canvas->Get(context,
                            Helpers::ConvertToV8String(isolate, "__native__context")).ToLocal(&ctx);
                if (!ctx.IsEmpty()) {
                    auto instanceType = Helpers::GetInstanceType(isolate, ctx);
                    if (instanceType == ObjectType::CanvasRenderingContext2D) {
                        draw_canvas2d(ctx);
                    } else if (instanceType == ObjectType::WebGLRenderingContext ||
                               instanceType == ObjectType::WebGL2RenderingContext) {
                        draw_webgl(ctx);
                    } else {
                        auto jsInstanceType = ObjectType::Unknown;

                        if (canvas->Get(context, Helpers::ConvertToV8String(isolate,
                                                                            "__instanceType")).ToLocal(
                                &ctx)) {
                            jsInstanceType = (ObjectType) ctx->Uint32Value(context).FromMaybe(0);

                            if (jsInstanceType == ObjectType::HTMLImage) {
                                auto asset = canvas->Get(context,
                                                         Helpers::ConvertToV8String(isolate,
                                                                                    "_asset"));
                                if (!asset.IsEmpty()) {
                                    auto val = asset.ToLocalChecked();
                                    draw_asset(val);
                                }
                            } else if (jsInstanceType == ObjectType::HTMLCanvas) {
                                auto nativeContext = canvas->Get(context,
                                                                 Helpers::ConvertToV8String(isolate,
                                                                                            "__native__context"));
                                if (!nativeContext.IsEmpty()) {
                                    auto val = nativeContext.ToLocalChecked();
                                    auto instanceType = Helpers::GetInstanceType(isolate, val);
                                    if (instanceType == ObjectType::CanvasRenderingContext2D) {
                                        draw_canvas2d(val);
                                    } else if (instanceType == ObjectType::WebGLRenderingContext ||
                                               instanceType == ObjectType::WebGL2RenderingContext) {
                                        draw_webgl(val);
                                    }

                                }
                            }
                        }

                    }
                }
            }
        }
    } else if (args.Length() == 8) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto width = args[3];
        auto height = args[4];
        auto border = args[5];
        auto format = args[6];
        auto type = args[7];
        canvas_native_webgl_tex_image2d_none(
                target->Int32Value(context).ToChecked(),
                level->Int32Value(context).ToChecked(),
                internalformat->Int32Value(context).ToChecked(),
                width->Int32Value(context).ToChecked(),
                height->Int32Value(context).ToChecked(),
                border->Int32Value(context).ToChecked(),
                format->Int32Value(context).ToChecked(),
                type->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    } else if (args.Length() == 9) {
        auto target = args[0];
        auto level = args[1];
        auto internalformat = args[2];
        auto width = args[3];
        auto height = args[4];
        auto border = args[5];
        auto format = args[6];
        auto type = args[7];
        auto pixels = args[8];

        if (pixels->IsArrayBufferView()) {
            auto buf = Helpers::GetTypedArrayData<uint8_t>(pixels.As<v8::TypedArray>());
            canvas_native_webgl_tex_image2d(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
            return;
        } else if (pixels->IsArrayBuffer()) {
            auto array = pixels.As<v8::ArrayBuffer>();
            auto store = array->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data());
            rust::Slice<uint8_t> buf(data, store->ByteLength());
            canvas_native_webgl_tex_image2d(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
            return;
        }

        if (pixels->IsNullOrUndefined()) {
            canvas_native_webgl_tex_image2d_none(
                    target->Int32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    internalformat->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    border->Int32Value(context).ToChecked(),
                    format->Int32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    ptr->GetState()
            );
        }
    }
}

void WebGLRenderingContext::TexParameterf(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto target = args[0];
        auto pname = args[1];
        auto param = args[2];
        canvas_native_webgl_tex_parameterf(
                target->Uint32Value(context).ToChecked(),
                pname->Uint32Value(context).ToChecked(),
                static_cast<float>(param->NumberValue(context).ToChecked()),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::TexParameteri(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto target = args[0];
        auto pname = args[1];
        auto param = args[2];
        canvas_native_webgl_tex_parameteri(
                target->Uint32Value(context).ToChecked(),
                pname->Uint32Value(context).ToChecked(),
                param->Int32Value(context).ToChecked(),
                ptr->GetState()
        );
    }
}

void WebGLRenderingContext::TexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());

    if (args.Length() == 7) {
        auto target = args[0];
        auto level = args[1];
        auto xoffset = args[2];
        auto yoffset = args[3];
        auto format = args[4];
        auto type = args[5];
        auto pixels = args[6];

        if (Helpers::GetInstanceType(isolate, pixels) == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(pixels->ToObject(context).ToLocalChecked());

            canvas_native_webgl_tex_sub_image2d_asset(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    asset->GetImageAsset(),
                    ptr->GetState()
            );
        } else if (Helpers::GetInstanceType(isolate, pixels) == ObjectType::ImageBitmap) {
            auto bitmap = ImageBitmapImpl::GetPointer(pixels->ToObject(context).ToLocalChecked());

            canvas_native_webgl_tex_sub_image2d_asset(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    bitmap->GetImageAsset(),
                    ptr->GetState()
            );
        }

    } else if (args.Length() == 9) {
        auto target = args[0];
        auto level = args[1];
        auto xoffset = args[2];
        auto yoffset = args[3];
        auto width = args[4];
        auto height = args[5];
        auto format = args[6];
        auto type = args[7];
        auto pixels = args[8];

        if (pixels->IsArrayBufferView()) {
            auto buf = Helpers::GetTypedArrayData<const uint8_t>(pixels.As<v8::TypedArray>());
            canvas_native_webgl_tex_sub_image2d(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
        } else if (pixels->IsArrayBuffer()) {
            auto array = pixels.As<v8::ArrayBuffer>();
            auto store = array->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data());
            rust::Slice<const uint8_t> buf(data, store->ByteLength());
            canvas_native_webgl_tex_sub_image2d(
                    target->Uint32Value(context).ToChecked(),
                    level->Int32Value(context).ToChecked(),
                    xoffset->Int32Value(context).ToChecked(),
                    yoffset->Int32Value(context).ToChecked(),
                    width->Int32Value(context).ToChecked(),
                    height->Int32Value(context).ToChecked(),
                    format->Uint32Value(context).ToChecked(),
                    type->Int32Value(context).ToChecked(),
                    buf,
                    ptr->GetState()
            );
        }
    }

}

void WebGLRenderingContext::VertexAttrib1f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_webgl_vertex_attrib1f(index, v0, ptr->GetState());
    }
}

void WebGLRenderingContext::VertexAttrib1fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = args[1];
        if (v0->IsFloat32Array()) {
            auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());
            canvas_native_webgl_vertex_attrib1fv(index, buf, ptr->GetState());
        }
    }
}

void WebGLRenderingContext::VertexAttrib2f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 3) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto v1 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        canvas_native_webgl_vertex_attrib2f(index, v0, v1, ptr->GetState());
    }
}

void WebGLRenderingContext::VertexAttrib2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = args[1];
        if (v0->IsFloat32Array()) {
            auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());
            canvas_native_webgl_vertex_attrib2fv(index, buf, ptr->GetState());
        }
    }
}

void WebGLRenderingContext::VertexAttrib3f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 4) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto v1 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto v2 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_webgl_vertex_attrib3f(index, v0, v1, v2, ptr->GetState());
    }
}

void WebGLRenderingContext::VertexAttrib3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = args[1];
        if (v0->IsFloat32Array()) {
            auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());;
            canvas_native_webgl_vertex_attrib3fv(index, buf, ptr->GetState());
        }
    }
}

void WebGLRenderingContext::VertexAttrib4f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 5) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto v1 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto v2 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto v3 = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        canvas_native_webgl_vertex_attrib4f(index, v0, v1, v2, v3, ptr->GetState());
    }
}

void WebGLRenderingContext::VertexAttrib4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto v0 = args[1];
        if (v0->IsFloat32Array()) {
            auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());
            canvas_native_webgl_vertex_attrib4fv(index, buf, ptr->GetState());
        }
    }
}

void WebGLRenderingContext::VertexAttribPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 6) {
        auto index = args[0]->Uint32Value(context).ToChecked();
        auto size = args[1]->Int32Value(context).ToChecked();
        auto type = args[2]->Uint32Value(context).ToChecked();
        auto normalized = args[3]->BooleanValue(isolate);
        auto stride = args[4]->Int32Value(context).ToChecked();
        auto offset = static_cast<ssize_t>(args[5]->IntegerValue(context).ToChecked());
        canvas_native_webgl_vertex_attrib_pointer(index, size, type, normalized, stride, offset,
                                                  ptr->GetState());
    }
}

void WebGLRenderingContext::Uniform1f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto location = args[0];
        auto v0 = args[1];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform1f(
                        instance->Int32Value(context).ToChecked(),
                        static_cast<float>(v0->NumberValue(context).ToChecked()),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Uniform1iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                if (v0->IsInt32Array()) {
                    auto buf = Helpers::GetTypedArrayData<const int32_t>(v0.As<v8::TypedArray>());
                    canvas_native_webgl_uniform1iv(instance->Int32Value(context).ToChecked(), buf,
                                                   ptr->GetState());
                } else if (v0->IsArray()) {
                    auto array = v0.As<v8::Array>();
                    auto len = array->Length();

                    std::vector<int32_t> buf;
                    for (int i = 0; i < len; ++i) {
                        auto item = array->Get(context, (uint32_t) i).ToLocalChecked();
                        buf.push_back(static_cast<float>(item->Int32Value(context).ToChecked()));
                    }
                    rust::Slice<const int32_t> slice(buf.data(), buf.size());
                    canvas_native_webgl_uniform1iv(instance->Int32Value(context).ToChecked(), slice,
                                                   ptr->GetState());
                }
            }
        }
    }
}

void WebGLRenderingContext::Uniform1fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (instance.IsEmpty()) {
                return;
            }
            if (v0->IsFloat32Array()) {
                auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());
                canvas_native_webgl_uniform1fv(instance->Int32Value(context).ToChecked(), buf,
                                               ptr->GetState());
            } else if (v0->IsArray()) {
                auto array = v0.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                for (int i = 0; i < len; ++i) {
                    auto item = array->Get(context, i).ToLocalChecked();
                    if (item.IsEmpty()) {
                        buf.push_back(nanf(""));
                    } else {
                        auto value = item->NumberValue(context);
                        buf.push_back(static_cast<float>(value.ToChecked()));
                    }
                }
                rust::Slice<const float> slice(buf.data(), buf.size());
                canvas_native_webgl_uniform1fv(instance->Int32Value(context).ToChecked(), slice,
                                               ptr->GetState());
            }
        }
    }
}

void WebGLRenderingContext::Uniform1i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 1) {
        auto location = args[0];
        auto v0 = args[1];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform1i(
                        instance->Int32Value(context).ToChecked(),
                        v0->Int32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Uniform2f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto location = args[0];
        auto v0 = args[1];
        auto v1 = args[2];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform2f(
                        instance->Int32Value(context).ToChecked(),
                        static_cast<float>(v0->NumberValue(context).ToChecked()),
                        static_cast<float>(v1->NumberValue(context).ToChecked()),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Uniform2iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                if (v0->IsInt32Array()) {
                    auto buf = Helpers::GetTypedArrayData<const int32_t>(v0.As<v8::TypedArray>());
                    canvas_native_webgl_uniform2iv(instance->Int32Value(context).ToChecked(), buf,
                                                   ptr->GetState());
                } else if (v0->IsArray()) {
                    auto array = v0.As<v8::Array>();
                    auto len = array->Length();

                    std::vector<int32_t> buf;
                    for (int i = 0; i < len; ++i) {
                        auto item = array->Get(context, (uint32_t) i).ToLocalChecked();
                        buf.push_back(static_cast<float>(item->Int32Value(context).ToChecked()));
                    }
                    rust::Slice<const int32_t> slice(buf.data(), buf.size());
                    canvas_native_webgl_uniform2iv(instance->Int32Value(context).ToChecked(), slice,
                                                   ptr->GetState());
                }
            }
        }
    }
}

void WebGLRenderingContext::Uniform2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (instance.IsEmpty()) {
                return;
            }
            if (v0->IsFloat32Array()) {
                auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());
                canvas_native_webgl_uniform2fv(instance->Int32Value(context).ToChecked(), buf,
                                               ptr->GetState());
            } else if (v0->IsArray()) {
                auto array = v0.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                for (int i = 0; i < len; ++i) {
                    auto item = array->Get(context, i).ToLocalChecked();
                    if (item.IsEmpty()) {
                        buf.push_back(nanf(""));
                    } else {
                        auto value = item->NumberValue(context);
                        buf.push_back(static_cast<float>(value.ToChecked()));
                    }
                }
                rust::Slice<const float> slice(buf.data(), buf.size());
                canvas_native_webgl_uniform2fv(instance->Int32Value(context).ToChecked(), slice,
                                               ptr->GetState());
            }
        }
    }
}

void WebGLRenderingContext::Uniform2i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 2) {
        auto location = args[0];
        auto v0 = args[1];
        auto v1 = args[2];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform2i(
                        instance->Int32Value(context).ToChecked(),
                        v0->Int32Value(context).ToChecked(),
                        v1->Int32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Uniform3f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 3) {
        auto location = args[0];
        auto v0 = args[1];
        auto v1 = args[2];
        auto v2 = args[3];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform3f(
                        instance->Int32Value(context).ToChecked(),
                        static_cast<float>(v0->NumberValue(context).ToChecked()),
                        static_cast<float>(v1->NumberValue(context).ToChecked()),
                        static_cast<float>(v2->NumberValue(context).ToChecked()),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Uniform3iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                if (v0->IsInt32Array()) {
                    auto buf = Helpers::GetTypedArrayData<const int32_t>(v0.As<v8::TypedArray>());
                    canvas_native_webgl_uniform3iv(instance->Int32Value(context).ToChecked(), buf,
                                                   ptr->GetState());
                } else if (v0->IsArray()) {
                    auto array = v0.As<v8::Array>();
                    auto len = array->Length();

                    std::vector<int32_t> buf;
                    for (int i = 0; i < len; ++i) {
                        auto item = array->Get(context, (uint32_t) i).ToLocalChecked();
                        buf.push_back(static_cast<float>(item->Int32Value(context).ToChecked()));
                    }
                    rust::Slice<const int32_t> slice(buf.data(), buf.size());
                    canvas_native_webgl_uniform3iv(instance->Int32Value(context).ToChecked(), slice,
                                                   ptr->GetState());
                }
            }
        }
    }
}

void WebGLRenderingContext::Uniform3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (instance.IsEmpty()) {
                return;
            }
            if (v0->IsFloat32Array()) {
                auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());
                canvas_native_webgl_uniform3fv(instance->Int32Value(context).ToChecked(), buf,
                                               ptr->GetState());
            } else if (v0->IsArray()) {
                auto array = v0.As<v8::Array>();
                auto len = array->Length();
                std::vector<float> buf;
                for (int i = 0; i < len; ++i) {
                    auto item = array->Get(context, i);
                    if (item.IsEmpty()) {
                        buf.push_back(nanf(""));
                    } else {
                        auto value = item.ToLocalChecked()->NumberValue(context);
                        buf.push_back(static_cast<float>(value.ToChecked()));
                    }
                }
                rust::Slice<const float> slice(buf.data(), buf.size());
                canvas_native_webgl_uniform3fv(instance->Int32Value(context).ToChecked(), slice,
                                               ptr->GetState());
            }
        }
    }
}

void WebGLRenderingContext::Uniform3i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 3) {
        auto location = args[0];
        auto v0 = args[1];
        auto v1 = args[2];
        auto v2 = args[3];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform3i(
                        instance->Int32Value(context).ToChecked(),
                        v0->Int32Value(context).ToChecked(),
                        v1->Int32Value(context).ToChecked(),
                        v2->Int32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Uniform4f(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 4) {
        auto location = args[0];
        auto v0 = args[1];
        auto v1 = args[2];
        auto v2 = args[3];
        auto v3 = args[4];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform4f(
                        instance->Int32Value(context).ToChecked(),
                        static_cast<float>(v0->NumberValue(context).ToChecked()),
                        static_cast<float>(v1->NumberValue(context).ToChecked()),
                        static_cast<float>(v2->NumberValue(context).ToChecked()),
                        static_cast<float>(v3->NumberValue(context).ToChecked()),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Uniform4iv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                if (v0->IsInt32Array()) {
                    auto buf = Helpers::GetTypedArrayData<const int32_t>(v0.As<v8::TypedArray>());
                    canvas_native_webgl_uniform4iv(instance->Int32Value(context).ToChecked(), buf,
                                                   ptr->GetState());
                } else if (v0->IsArray()) {
                    auto array = v0.As<v8::Array>();
                    auto len = array->Length();

                    std::vector<int32_t> buf;
                    for (int i = 0; i < len; ++i) {
                        auto item = array->Get(context, (uint32_t) i).ToLocalChecked();
                        buf.push_back(static_cast<float>(item->Int32Value(context).ToChecked()));
                    }
                    rust::Slice<const int32_t> slice(buf.data(), buf.size());
                    canvas_native_webgl_uniform4iv(instance->Int32Value(context).ToChecked(), slice,
                                                   ptr->GetState());
                }
            }
        }
    }
}

void WebGLRenderingContext::Uniform4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    v8::TryCatch tryCatch(isolate);
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 2) {
        auto location = args[0];
        auto v0 = args[1];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (instance.IsEmpty()) {
                return;
            }
            if (v0->IsFloat32Array()) {
                auto buf = Helpers::GetTypedArrayData<const float>(v0.As<v8::TypedArray>());
                canvas_native_webgl_uniform4fv(instance->Int32Value(context).ToChecked(), buf,
                                               ptr->GetState());
            } else if (v0->IsArray()) {
                auto array = v0.As<v8::Array>();
                auto len = array->Length();

                std::vector<float> buf;
                for (int i = 0; i < len; ++i) {
                    auto item = array->Get(context, (uint32_t) i).ToLocalChecked();
                    buf.push_back(static_cast<float>(item->NumberValue(context).ToChecked()));
                }
                rust::Slice<const float> slice(buf.data(), buf.size());
                canvas_native_webgl_uniform4fv(instance->Int32Value(context).ToChecked(), slice,
                                               ptr->GetState());
            }
        }
    }
}

void WebGLRenderingContext::Uniform4i(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 4) {
        auto location = args[0];
        auto v0 = args[1];
        auto v1 = args[2];
        auto v2 = args[3];
        auto v3 = args[4];
        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_uniform4i(
                        instance->Int32Value(context).ToChecked(),
                        v0->Int32Value(context).ToChecked(),
                        v1->Int32Value(context).ToChecked(),
                        v2->Int32Value(context).ToChecked(),
                        v3->Int32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::UniformMatrix2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 3) {
        auto location = args[0];
        auto transpose = args[1];
        auto value = args[2];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");

            if (!instance.IsEmpty()) {
                if (value->IsFloat32Array()) {
                    auto buf = Helpers::GetTypedArrayData<const float>(value.As<v8::TypedArray>());
                    canvas_native_webgl_uniform_matrix2fv(instance->Int32Value(context).ToChecked(),
                                                          transpose->BooleanValue(isolate), buf,
                                                          ptr->GetState());
                } else if (value->IsArray()) {
                    auto array = value.As<v8::Array>();
                    std::vector<float> buf;
                    auto len = array->Length();
                    for (int i = 0; i < len; i++) {
                        auto item = array->Get(context, i).ToLocalChecked();
                        if (Helpers::IsNumber(item)) {
                            buf.push_back(
                                    static_cast<float>(Helpers::GetNumberValue(isolate, item)));
                        } else {
                            buf.push_back(std::nanf(""));
                        }
                    }

                    rust::Slice<const float> slice(buf.data(), buf.size());
                    canvas_native_webgl_uniform_matrix2fv(instance->Int32Value(context).ToChecked(),
                                                          transpose->BooleanValue(isolate), slice,
                                                          ptr->GetState());
                }
            }
        }
    }
}

void WebGLRenderingContext::UniformMatrix3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 3) {
        auto location = args[0];
        auto transpose = args[1];
        auto value = args[2];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                if (value->IsFloat32Array()) {
                    auto buf = Helpers::GetTypedArrayData<const float>(value.As<v8::TypedArray>());
                    canvas_native_webgl_uniform_matrix3fv(instance->Int32Value(context).ToChecked(),
                                                          transpose->BooleanValue(isolate), buf,
                                                          ptr->GetState());
                } else if (value->IsArray()) {
                    auto array = value.As<v8::Array>();
                    std::vector<float> buf;
                    auto len = array->Length();
                    for (int i = 0; i < len; i++) {
                        auto item = array->Get(context, i).ToLocalChecked();
                        if (Helpers::IsNumber(item)) {
                            buf.push_back(
                                    static_cast<float>(Helpers::GetNumberValue(isolate, item)));
                        } else {
                            buf.push_back(std::nanf(""));
                        }
                    }

                    rust::Slice<const float> slice(buf.data(), buf.size());
                    canvas_native_webgl_uniform_matrix3fv(instance->Int32Value(context).ToChecked(),
                                                          transpose->BooleanValue(isolate), slice,
                                                          ptr->GetState());
                }
            }
        }
    }
}

void WebGLRenderingContext::UniformMatrix4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 3) {
        auto location = args[0];
        auto transpose = args[1];
        auto value = args[2];

        if (Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
            auto instance = Helpers::GetPrivate(isolate, location.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                if (value->IsFloat32Array()) {
                    auto buf = Helpers::GetTypedArrayData<const float>(value.As<v8::TypedArray>());
                    canvas_native_webgl_uniform_matrix4fv(instance->Int32Value(context).ToChecked(),
                                                          transpose->BooleanValue(isolate), buf,
                                                          ptr->GetState());
                } else if (value->IsArray()) {
                    auto array = value.As<v8::Array>();
                    std::vector<float> buf;
                    auto len = array->Length();
                    for (int i = 0; i < len; i++) {
                        auto item = array->Get(context, i).ToLocalChecked();
                        if (Helpers::IsNumber(item)) {
                            buf.push_back(
                                    static_cast<float>(Helpers::GetNumberValue(isolate, item)));
                        } else {
                            buf.push_back(std::nanf(""));
                        }
                    }

                    rust::Slice<const float> slice(buf.data(), buf.size());
                    canvas_native_webgl_uniform_matrix4fv(instance->Int32Value(context).ToChecked(),
                                                          transpose->BooleanValue(isolate), slice,
                                                          ptr->GetState());
                }
            }
        }
    }
}

void WebGLRenderingContext::UseProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto program = args[0];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_use_program(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::ValidateProgram(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() > 0) {
        auto program = args[0];
        if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
            auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
            if (!instance.IsEmpty()) {
                canvas_native_webgl_validate_program(
                        instance->Uint32Value(context).ToChecked(),
                        ptr->GetState()
                );
            }
        }
    }
}

void WebGLRenderingContext::Viewport(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(args.This());
    if (args.Length() == 4) {
        auto x = args[0]->Int32Value(context).ToChecked();
        auto y = args[1]->Int32Value(context).ToChecked();
        auto width = args[2]->Int32Value(context).ToChecked();
        auto height = args[3]->Int32Value(context).ToChecked();
        canvas_native_webgl_viewport(x, y, width, height, ptr->GetState());
    }
}


void WebGLRenderingContext::GetDrawingBufferWidth(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto ptr = GetPointerBase(info.This());

    int32_t ret = 0;
    if (ptr != nullptr) {
        ret = canvas_native_webgl_state_get_drawing_buffer_width(ptr->GetState());
    }
    info.GetReturnValue().Set(ret);
}

void WebGLRenderingContext::GetDrawingBufferHeight(v8::Local<v8::String> name,
                                                   const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(info.This());
    int32_t ret = 0;
    if (ptr != nullptr) {
        ret = canvas_native_webgl_state_get_drawing_buffer_height(ptr->GetState());
    }
    info.GetReturnValue().Set(ret);
}


void WebGLRenderingContext::GetFlipY(v8::Local<v8::String> name,
                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointerBase(info.This());
    info.GetReturnValue().Set(canvas_native_webgl_state_get_flip_y(ptr->GetState()));
}


v8::Local<v8::FunctionTemplate> WebGLRenderingContext::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->WebGLRenderingContextTmpl.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }

    auto context = isolate->GetCurrentContext();
    auto webglRenderingContextFunc = v8::FunctionTemplate::New(isolate, &Create);
    webglRenderingContextFunc->SetClassName(
            Helpers::ConvertToV8String(isolate, "WebGLRenderingContext"));
    webglRenderingContextFunc->InstanceTemplate()->SetInternalFieldCount(1);

    auto webglRenderingContextTpl = webglRenderingContextFunc->PrototypeTemplate();

    SetConstants(isolate, webglRenderingContextTpl);

    SetProps(isolate, webglRenderingContextTpl);

    SetMethods(isolate, webglRenderingContextTpl);

    cache->WebGLRenderingContextTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(
            isolate,
            webglRenderingContextFunc);
    return webglRenderingContextFunc;
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

void
WebGLRenderingContext::SetProps(v8::Isolate *isolate,
                                const v8::Local<v8::ObjectTemplate> &webglRenderingContextTpl) {
    webglRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "drawingBufferWidth"),
            &GetDrawingBufferWidth
    );

    webglRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "drawingBufferHeight"),
            &GetDrawingBufferHeight
    );

    webglRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "__flipY"),
            &GetFlipY
    );

    webglRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "__suppressLogs"),
            &GetSuppressLogs,
            &SetSuppressLogs
    );

}

void
WebGLRenderingContext::SetMethods(v8::Isolate *isolate,
                                  const v8::Local<v8::ObjectTemplate> &webglRenderingContextTpl) {

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__resized"),
            v8::FunctionTemplate::New(isolate, &Resized)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__toDataURL"),
            v8::FunctionTemplate::New(isolate, &ToDataURL)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "activeTexture"),
            v8::FunctionTemplate::New(isolate, &ActiveTexture)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "attachShader"),
            v8::FunctionTemplate::New(isolate, &AttachShader)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindAttribLocation"),
            v8::FunctionTemplate::New(isolate, &BindAttribLocation)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindBuffer"),
            v8::FunctionTemplate::New(isolate, &BindBuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindFramebuffer"),
            v8::FunctionTemplate::New(isolate, &BindFramebuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &BindRenderbuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bindTexture"),
            v8::FunctionTemplate::New(isolate, &BindTexture)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "blendColor"),
            v8::FunctionTemplate::New(isolate, &BlendColor)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "blendEquationSeparate"),
            v8::FunctionTemplate::New(isolate, &BlendEquationSeparate)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "blendEquation"),
            v8::FunctionTemplate::New(isolate, &BlendEquation)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "blendFuncSeparate"),
            v8::FunctionTemplate::New(isolate, &BlendFuncSeparate)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "blendFunc"),
            v8::FunctionTemplate::New(isolate, &BlendFunc)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bufferData"),
            v8::FunctionTemplate::New(isolate, &BufferData)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bufferSubData"),
            v8::FunctionTemplate::New(isolate, &BufferSubData)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "checkFramebufferStatus"),
            v8::FunctionTemplate::New(isolate, &CheckFramebufferStatus)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearColor"),
            v8::FunctionTemplate::New(isolate, &ClearColor)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearDepth"),
            v8::FunctionTemplate::New(isolate, &ClearDepth)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearStencil"),
            v8::FunctionTemplate::New(isolate, &ClearStencil)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clear"),
            v8::FunctionTemplate::New(isolate, &Clear)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "colorMask"),
            v8::FunctionTemplate::New(isolate, &ColorMask)
    );
    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "commit"),
            v8::FunctionTemplate::New(isolate, &Commit)
    );
    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "compileShader"),
            v8::FunctionTemplate::New(isolate, &CompileShader)
    );
    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "compressedTexImage2D"),
            v8::FunctionTemplate::New(isolate, &CompressedTexImage2D)
    );
    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "compressedTexSubImage2D"),
            v8::FunctionTemplate::New(isolate, &CompressedTexSubImage2D)
    );
    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "copyTexImage2D"),
            v8::FunctionTemplate::New(isolate, &CopyTexImage2D)
    );
    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "copyTexSubImage2D"),
            v8::FunctionTemplate::New(isolate, &CopyTexSubImage2D)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createBuffer"),
            v8::FunctionTemplate::New(isolate, &CreateBuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createFramebuffer"),
            v8::FunctionTemplate::New(isolate, &CreateFramebuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createProgram"),
            v8::FunctionTemplate::New(isolate, &CreateProgram)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &CreateRenderbuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createShader"),
            v8::FunctionTemplate::New(isolate, &CreateShader)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createTexture"),
            v8::FunctionTemplate::New(isolate, &CreateTexture)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "cullFace"),
            v8::FunctionTemplate::New(isolate, &CullFace)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteBuffer"),
            v8::FunctionTemplate::New(isolate, &DeleteBuffer)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteFramebuffer"),
            v8::FunctionTemplate::New(isolate, &DeleteFramebuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteProgram"),
            v8::FunctionTemplate::New(isolate, &DeleteProgram)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &DeleteRenderbuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteShader"),
            v8::FunctionTemplate::New(isolate, &DeleteShader)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "deleteTexture"),
            v8::FunctionTemplate::New(isolate, &DeleteTexture)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "depthFunc"),
            v8::FunctionTemplate::New(isolate, &DepthFunc)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "depthMask"),
            v8::FunctionTemplate::New(isolate, &DepthMask)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "depthRange"),
            v8::FunctionTemplate::New(isolate, &DepthRange)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "detachShader"),
            v8::FunctionTemplate::New(isolate, &DetachShader)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "disableVertexAttribArray"),
            v8::FunctionTemplate::New(isolate, &DisableVertexAttribArray)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "disable"),
            v8::FunctionTemplate::New(isolate, &Disable)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawArrays"),
            v8::FunctionTemplate::New(isolate, &DrawArrays)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawElements"),
            v8::FunctionTemplate::New(isolate, &DrawElements)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "enableVertexAttribArray"),
            v8::FunctionTemplate::New(isolate, &EnableVertexAttribArray)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "enable"),
            v8::FunctionTemplate::New(isolate, &Enable)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "finish"),
            v8::FunctionTemplate::New(isolate, &Finish)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "flush"),
            v8::FunctionTemplate::New(isolate, &Flush)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "framebufferRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &FramebufferRenderbuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "framebufferTexture2D"),
            v8::FunctionTemplate::New(isolate, &FramebufferTexture2D)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "frontFace"),
            v8::FunctionTemplate::New(isolate, &FrontFace)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "generateMipmap"),
            v8::FunctionTemplate::New(isolate, &GenerateMipmap)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getActiveAttrib"),
            v8::FunctionTemplate::New(isolate, &GetActiveAttrib)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getActiveUniform"),
            v8::FunctionTemplate::New(isolate, &GetActiveUniform)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getAttachedShaders"),
            v8::FunctionTemplate::New(isolate, &GetAttachedShaders)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getAttribLocation"),
            v8::FunctionTemplate::New(isolate, &GetAttribLocation)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getBufferParameter"),
            v8::FunctionTemplate::New(isolate, &GetBufferParameter)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getContextAttributes"),
            v8::FunctionTemplate::New(isolate, &GetContextAttributes)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getError"),
            v8::FunctionTemplate::New(isolate, &GetError)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getExtension"),
            v8::FunctionTemplate::New(isolate, &GetExtension)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getFramebufferAttachmentParameter"),
            v8::FunctionTemplate::New(isolate, &GetFramebufferAttachmentParameter)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getParameter"),
            v8::FunctionTemplate::New(isolate, &GetParameter)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getProgramInfoLog"),
            v8::FunctionTemplate::New(isolate, &GetProgramInfoLog)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getProgramParameter"),
            v8::FunctionTemplate::New(isolate, &GetProgramParameter)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getRenderbufferParameter"),
            v8::FunctionTemplate::New(isolate, &GetRenderbufferParameter)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getShaderInfoLog"),
            v8::FunctionTemplate::New(isolate, &GetShaderInfoLog)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getShaderParameter"),
            v8::FunctionTemplate::New(isolate, &GetShaderParameter)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getShaderPrecisionFormat"),
            v8::FunctionTemplate::New(isolate, &GetShaderPrecisionFormat)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getShaderSource"),
            v8::FunctionTemplate::New(isolate, &GetShaderSource)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getSupportedExtensions"),
            v8::FunctionTemplate::New(isolate, &GetSupportedExtensions)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__getSupportedExtensions"),
            v8::FunctionTemplate::New(isolate, &GetSupportedExtensionsString)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getTexParameter"),
            v8::FunctionTemplate::New(isolate, &GetTexParameter)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getUniformLocation"),
            v8::FunctionTemplate::New(isolate, &GetUniformLocation)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getUniform"),
            v8::FunctionTemplate::New(isolate, &GetUniform)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getVertexAttribOffset"),
            v8::FunctionTemplate::New(isolate, &GetVertexAttribOffset)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getVertexAttrib"),
            v8::FunctionTemplate::New(isolate, &GetVertexAttrib)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "hint"),
            v8::FunctionTemplate::New(isolate, &Hint)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isBuffer"),
            v8::FunctionTemplate::New(isolate, &IsBuffer)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isContextLost"),
            v8::FunctionTemplate::New(isolate, &IsContextLost)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isEnabled"),
            v8::FunctionTemplate::New(isolate, &IsEnabled)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isFramebuffer"),
            v8::FunctionTemplate::New(isolate, &IsFramebuffer)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isProgram"),
            v8::FunctionTemplate::New(isolate, &IsProgram)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isRenderbuffer"),
            v8::FunctionTemplate::New(isolate, &IsRenderbuffer)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isShader"),
            v8::FunctionTemplate::New(isolate, &IsShader)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isTexture"),
            v8::FunctionTemplate::New(isolate, &IsTexture)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "lineWidth"),
            v8::FunctionTemplate::New(isolate, &LineWidth)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "linkProgram"),
            v8::FunctionTemplate::New(isolate, &LinkProgram)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "pixelStorei"),
            v8::FunctionTemplate::New(isolate, &PixelStorei)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "polygonOffset"),
            v8::FunctionTemplate::New(isolate, &PolygonOffset)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "readPixels"),
            v8::FunctionTemplate::New(isolate, &ReadPixels)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "renderbufferStorage"),
            v8::FunctionTemplate::New(isolate, &RenderbufferStorage)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "sampleCoverage"),
            v8::FunctionTemplate::New(isolate, &SampleCoverage)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "scissor"),
            v8::FunctionTemplate::New(isolate, &Scissor)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "shaderSource"),
            v8::FunctionTemplate::New(isolate, &ShaderSource)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stencilFuncSeparate"),
            v8::FunctionTemplate::New(isolate, &StencilFuncSeparate)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stencilFunc"),
            v8::FunctionTemplate::New(isolate, &StencilFunc)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stencilMaskSeparate"),
            v8::FunctionTemplate::New(isolate, &StencilMaskSeparate)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stencilMask"),
            v8::FunctionTemplate::New(isolate, &StencilMask)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stencilOpSeparate"),
            v8::FunctionTemplate::New(isolate, &StencilOpSeparate)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stencilOp"),
            v8::FunctionTemplate::New(isolate, &StencilOp)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texImage2D"),
            v8::FunctionTemplate::New(isolate, &TexImage2D)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texParameterf"),
            v8::FunctionTemplate::New(isolate, &TexParameterf)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texParameteri"),
            v8::FunctionTemplate::New(isolate, &TexParameteri)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "texSubImage2D"),
            v8::FunctionTemplate::New(isolate, &TexSubImage2D)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib1f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib1f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib1fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib1fv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib2f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib2f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib2fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib2fv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib3f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib3f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib3fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib3fv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib4f"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib4f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttrib4fv"),
            v8::FunctionTemplate::New(isolate, &VertexAttrib4fv)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "vertexAttribPointer"),
            v8::FunctionTemplate::New(isolate, &VertexAttribPointer)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform1f"),
            v8::FunctionTemplate::New(isolate, &Uniform1f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform1iv"),
            v8::FunctionTemplate::New(isolate, &Uniform1iv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform1fv"),
            v8::FunctionTemplate::New(isolate, &Uniform1fv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform1i"),
            v8::FunctionTemplate::New(isolate, &Uniform1i)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform2f"),
            v8::FunctionTemplate::New(isolate, &Uniform2f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform2iv"),
            v8::FunctionTemplate::New(isolate, &Uniform2iv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform2fv"),
            v8::FunctionTemplate::New(isolate, &Uniform2fv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform2i"),
            v8::FunctionTemplate::New(isolate, &Uniform2i)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform3f"),
            v8::FunctionTemplate::New(isolate, &Uniform3f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform3iv"),
            v8::FunctionTemplate::New(isolate, &Uniform3iv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform3fv"),
            v8::FunctionTemplate::New(isolate, &Uniform3fv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform3i"),
            v8::FunctionTemplate::New(isolate, &Uniform3i)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform4f"),
            v8::FunctionTemplate::New(isolate, &Uniform4f)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform4iv"),
            v8::FunctionTemplate::New(isolate, &Uniform4iv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform4fv"),
            v8::FunctionTemplate::New(isolate, &Uniform4fv)
    );

    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniform4i"),
            v8::FunctionTemplate::New(isolate, &Uniform4i)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix2fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix2fv)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix3fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix3fv)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "uniformMatrix4fv"),
            v8::FunctionTemplate::New(isolate, &UniformMatrix4fv)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "useProgram"),
            v8::FunctionTemplate::New(isolate, &UseProgram)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "validateProgram"),
            v8::FunctionTemplate::New(isolate, &ValidateProgram)
    );


    webglRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "viewport"),
            v8::FunctionTemplate::New(isolate, &Viewport)
    );

}
