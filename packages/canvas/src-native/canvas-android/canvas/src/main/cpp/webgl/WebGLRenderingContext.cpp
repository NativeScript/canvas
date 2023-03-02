//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGLRenderingContext.h"

WebGLRenderingContext::WebGLRenderingContext(rust::Box<WebGLState> state)
        : WebGLRenderingContextBase(
        std::move(state), WebGLRenderingVersion::V1) {}

jsi::Value WebGLRenderingContext::GetParameterInternal(jsi::Runtime &runtime,
                                                 uint32_t pnameValue,
                                                 rust::Box<WebGLResult> result) {

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
                return Value::null();
            }
            return {value};
        }
        case UNPACK_COLORSPACE_CONVERSION_WEBGL:
            return {canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
                    this->GetState()};
        case GL_ALIASED_LINE_WIDTH_RANGE:
        case GL_ALIASED_POINT_SIZE_RANGE:
        case GL_DEPTH_RANGE: {
            auto ret = canvas_native_webgl_result_get_f32_array(*result);
            auto buf = std::make_shared<VecMutableBuffer<float>>(std::move(ret));
            auto array = jsi::ArrayBuffer(runtime, buf);

            auto Float32Array = runtime.global()
                    .getProperty(runtime,
                                 "Float32Array")
                    .asObject(runtime)
                    .asFunction(runtime);


            return Float32Array.callAsConstructor(
                    runtime, array);
        }
            break;
        case GL_BLEND_COLOR:
        case GL_COLOR_CLEAR_VALUE: {
            auto ret = canvas_native_webgl_result_get_f32_array(*result);

            auto buf = std::make_shared<VecMutableBuffer<float>>(std::move(ret));
            auto array = jsi::ArrayBuffer(runtime, buf);

            auto Float32Array = runtime.global()
                    .getProperty(runtime,
                                 "Float32Array")
                    .asObject(runtime)
                    .asFunction(runtime);


            return Float32Array.callAsConstructor(
                    runtime, array);

        }
            break;
        case UNPACK_FLIP_Y_WEBGL:
           return {canvas_native_webgl_state_get_flip_y(this->GetState())};
        case UNPACK_PREMULTIPLY_ALPHA_WEBGL:
            return { canvas_native_webgl_state_get_premultiplied_alpha(this->GetState())};
        case GL_BLEND:
        case GL_CULL_FACE:
        case GL_DEPTH_TEST:
        case GL_DEPTH_WRITEMASK:
        case GL_DITHER:
        case GL_POLYGON_OFFSET_FILL:
        case GL_SAMPLE_COVERAGE_INVERT:
        case GL_SCISSOR_TEST:
        case GL_STENCIL_TEST:
            return {canvas_native_webgl_result_get_bool(*result)};
        case GL_COLOR_WRITEMASK: {
            auto ret = canvas_native_webgl_result_get_bool_array(*result);
            auto len = ret.size();
            auto array = jsi::Array(runtime, len);
            for (int j = 0; j < len; ++j) {
                array.setValueAtIndex(runtime, j, Value(ret[j] == 1));
            }
            return array;
        }
        case GL_COMPRESSED_TEXTURE_FORMATS:
        case GL_MAX_VIEWPORT_DIMS:
        case GL_SCISSOR_BOX:
        case GL_VIEWPORT: {
            auto ret = canvas_native_webgl_result_get_i32_array(*result);

            auto buf = std::make_shared<VecMutableBuffer<int32_t>>(std::move(ret));
            auto array = jsi::ArrayBuffer(runtime, buf);

            auto Int32Array = runtime.global()
                    .getProperty(runtime,
                                 "Int32Array")
                    .asObject(runtime)
                    .asFunction(runtime);


            return Int32Array.callAsConstructor(
                    runtime, array);

        }
            break;
        case GL_DEPTH_CLEAR_VALUE:
        case GL_LINE_WIDTH:
        case GL_POLYGON_OFFSET_FACTOR:
        case GL_POLYGON_OFFSET_UNITS:
        case GL_SAMPLE_COVERAGE_VALUE: {
            return {static_cast<double>(canvas_native_webgl_result_get_f32(*result))};
        }
        case GL_RENDERER:
        case GL_SHADING_LANGUAGE_VERSION:
        case GL_VENDOR:
        case GL_VERSION: {
            auto ret = canvas_native_webgl_result_get_string(*result);
            return jsi::String::createFromAscii(runtime, ret.data(), ret.size());
        }
        default:
            return Value::null();

    }
}

std::vector<jsi::PropNameID> WebGLRenderingContext::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, "__resized"),
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
            jsi::PropNameID::forUtf8(rt, std::string("__toDataURL")),

            jsi::PropNameID::forUtf8(rt, "DEPTH_BUFFER_BIT"),

            jsi::PropNameID::forUtf8(rt, "STENCIL_BUFFER_BIT"),

            jsi::PropNameID::forUtf8(rt, "COLOR_BUFFER_BIT"),

            jsi::PropNameID::forUtf8(rt, "POINTS"),

            jsi::PropNameID::forUtf8(rt, "LINES"),

            jsi::PropNameID::forUtf8(rt, "LINE_LOOP"),

            jsi::PropNameID::forUtf8(rt, "LINE_STRIP"),

            jsi::PropNameID::forUtf8(rt, "TRIANGLES"),

            jsi::PropNameID::forUtf8(rt, "TRIANGLE_STRIP"),

            jsi::PropNameID::forUtf8(rt, "TRIANGLE_FAN"),

            jsi::PropNameID::forUtf8(rt, "ZERO"),

            jsi::PropNameID::forUtf8(rt, "ONE"),

            jsi::PropNameID::forUtf8(rt, "SRC_COLOR"),

            jsi::PropNameID::forUtf8(rt, "ONE_MINUS_SRC_COLOR"),

            jsi::PropNameID::forUtf8(rt, "SRC_ALPHA"),

            jsi::PropNameID::forUtf8(rt, "ONE_MINUS_SRC_ALPHA"),

            jsi::PropNameID::forUtf8(rt, "DST_ALPHA"),

            jsi::PropNameID::forUtf8(rt, "ONE_MINUS_DST_ALPHA"),

            jsi::PropNameID::forUtf8(rt, "DST_COLOR"),

            jsi::PropNameID::forUtf8(rt, "ONE_MINUS_DST_COLOR"),

            jsi::PropNameID::forUtf8(rt, "SRC_ALPHA_SATURATE"),


            jsi::PropNameID::forUtf8(rt, "CONSTANT_COLOR"),
            jsi::PropNameID::forUtf8(rt, "ONE_MINUS_CONSTANT_COLOR"),
            jsi::PropNameID::forUtf8(rt, "CONSTANT_ALPHA"),
            jsi::PropNameID::forUtf8(rt, "ONE_MINUS_CONSTANT_ALPHA"),


            /* Blending equations */

            jsi::PropNameID::forUtf8(rt, "FUNC_ADD"),
            jsi::PropNameID::forUtf8(rt, "FUNC_SUBTRACT"),
            jsi::PropNameID::forUtf8(rt, "FUNC_REVERSE_SUBTRACT"),
            jsi::PropNameID::forUtf8(rt, "BLEND_EQUATION"),
            jsi::PropNameID::forUtf8(rt, "BLEND_EQUATION_RGB"),
            jsi::PropNameID::forUtf8(rt, "BLEND_EQUATION_ALPHA"),


            jsi::PropNameID::forUtf8(rt, "BLEND_DST_RGB"),
            jsi::PropNameID::forUtf8(rt, "BLEND_SRC_RGB"),
            jsi::PropNameID::forUtf8(rt, "BLEND_DST_ALPHA"),


            jsi::PropNameID::forUtf8(rt, "BLEND_SRC_ALPHA"),
            jsi::PropNameID::forUtf8(rt, "BLEND_COLOR"),
            jsi::PropNameID::forUtf8(rt, "ARRAY_BUFFER_BINDING"),


            jsi::PropNameID::forUtf8(rt, "ELEMENT_ARRAY_BUFFER_BINDING"),
            jsi::PropNameID::forUtf8(rt, "LINE_WIDTH"),
            jsi::PropNameID::forUtf8(rt, "ALIASED_POINT_SIZE_RANGE"),

            jsi::PropNameID::forUtf8(rt, "ALIASED_LINE_WIDTH_RANGE"),
            jsi::PropNameID::forUtf8(rt, "CULL_FACE_MODE"),
            jsi::PropNameID::forUtf8(rt, "FRONT_FACE"),

            jsi::PropNameID::forUtf8(rt, "DEPTH_RANGE"),
            jsi::PropNameID::forUtf8(rt, "DEPTH_WRITEMASK"),
            jsi::PropNameID::forUtf8(rt, "DEPTH_CLEAR_VALUE"),

            jsi::PropNameID::forUtf8(rt, "DEPTH_FUNC"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_CLEAR_VALUE"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_FUNC"),

            jsi::PropNameID::forUtf8(rt, "STENCIL_FAIL"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_PASS_DEPTH_FAIL"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_PASS_DEPTH_PASS"),

            jsi::PropNameID::forUtf8(rt, "STENCIL_REF"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_VALUE_MASK"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_WRITEMASK"),

            jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_FUNC"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_FAIL"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_PASS_DEPTH_FAIL"),

            jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_PASS_DEPTH_PASS"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_REF"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_VALUE_MASK"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_BACK_WRITEMASK"),

            jsi::PropNameID::forUtf8(rt, "VIEWPORT"),
            jsi::PropNameID::forUtf8(rt, "SCISSOR_BOX"),
            jsi::PropNameID::forUtf8(rt, "COLOR_CLEAR_VALUE"),
            jsi::PropNameID::forUtf8(rt, "COLOR_WRITEMASK"),

            jsi::PropNameID::forUtf8(rt, "UNPACK_ALIGNMENT"),
            jsi::PropNameID::forUtf8(rt, "PACK_ALIGNMENT"),
            jsi::PropNameID::forUtf8(rt, "MAX_TEXTURE_SIZE"),


            jsi::PropNameID::forUtf8(rt, "MAX_VIEWPORT_DIMS"),
            jsi::PropNameID::forUtf8(rt, "SUBPIXEL_BITS"),

            jsi::PropNameID::forUtf8(rt, "RED_BITS"),
            jsi::PropNameID::forUtf8(rt, "GREEN_BITS"),
            jsi::PropNameID::forUtf8(rt, "BLUE_BITS"),
            jsi::PropNameID::forUtf8(rt, "ALPHA_BITS"),

            jsi::PropNameID::forUtf8(rt, "STENCIL_BITS"),
            jsi::PropNameID::forUtf8(rt, "POLYGON_OFFSET_UNITS"),
            jsi::PropNameID::forUtf8(rt, "POLYGON_OFFSET_FACTOR"),

            jsi::PropNameID::forUtf8(rt, "TEXTURE_BINDING_2D"),
            jsi::PropNameID::forUtf8(rt, "SAMPLE_BUFFERS"),
            jsi::PropNameID::forUtf8(rt, "SAMPLES"),
            jsi::PropNameID::forUtf8(rt, "SAMPLE_COVERAGE_VALUE"),

            jsi::PropNameID::forUtf8(rt, "SAMPLE_COVERAGE_INVERT"),
            jsi::PropNameID::forUtf8(rt, "COMPRESSED_TEXTURE_FORMATS"),
            jsi::PropNameID::forUtf8(rt, "VENDOR"),
            jsi::PropNameID::forUtf8(rt, "RENDERER"),

            jsi::PropNameID::forUtf8(rt, "VERSION"),
            jsi::PropNameID::forUtf8(rt, "IMPLEMENTATION_COLOR_READ_TYPE"),
            jsi::PropNameID::forUtf8(rt, "IMPLEMENTATION_COLOR_READ_FORMAT"),
            jsi::PropNameID::forUtf8(rt, "BROWSER_DEFAULT_WEBGL"),

            jsi::PropNameID::forUtf8(rt, "STATIC_DRAW"),
            jsi::PropNameID::forUtf8(rt, "STREAM_DRAW"),
            jsi::PropNameID::forUtf8(rt, "DYNAMIC_DRAW"),
            jsi::PropNameID::forUtf8(rt, "ARRAY_BUFFER"),

            jsi::PropNameID::forUtf8(rt, "ELEMENT_ARRAY_BUFFER"),
            jsi::PropNameID::forUtf8(rt, "BUFFER_SIZE"),
            jsi::PropNameID::forUtf8(rt, "BUFFER_USAGE"),
            jsi::PropNameID::forUtf8(rt, "CURRENT_VERTEX_ATTRIB"),


            jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_ENABLED"),
            jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_SIZE"),
            jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_STRIDE"),
            jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_TYPE"),
            jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_NORMALIZED"),
            jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_POINTER"),
            jsi::PropNameID::forUtf8(rt, "VERTEX_ATTRIB_ARRAY_BUFFER_BINDING"),

            jsi::PropNameID::forUtf8(rt, "CULL_FACE"),
            jsi::PropNameID::forUtf8(rt, "FRONT"),
            jsi::PropNameID::forUtf8(rt, "BACK"),
            jsi::PropNameID::forUtf8(rt, "FRONT_AND_BACK"),

            jsi::PropNameID::forUtf8(rt, "BLEND"),
            jsi::PropNameID::forUtf8(rt, "DEPTH_TEST"),
            jsi::PropNameID::forUtf8(rt, "DITHER"),
            jsi::PropNameID::forUtf8(rt, "POLYGON_OFFSET_FILL"),

            jsi::PropNameID::forUtf8(rt, "SAMPLE_ALPHA_TO_COVERAGE"),
            jsi::PropNameID::forUtf8(rt, "SAMPLE_COVERAGE"),
            jsi::PropNameID::forUtf8(rt, "SCISSOR_TEST"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_TEST"),


            /* Errors */

            jsi::PropNameID::forUtf8(rt, "NO_ERROR"),
            jsi::PropNameID::forUtf8(rt, "INVALID_ENUM"),
            jsi::PropNameID::forUtf8(rt, "INVALID_VALUE"),
            jsi::PropNameID::forUtf8(rt, "INVALID_OPERATION"),

            jsi::PropNameID::forUtf8(rt, "OUT_OF_MEMORY"),
            jsi::PropNameID::forUtf8(rt, "CONTEXT_LOST_WEBGL"),
            jsi::PropNameID::forUtf8(rt, "CW"),
            jsi::PropNameID::forUtf8(rt, "CCW"),

            jsi::PropNameID::forUtf8(rt, "DONT_CARE"),
            jsi::PropNameID::forUtf8(rt, "FASTEST"),
            jsi::PropNameID::forUtf8(rt, "NICEST"),
            jsi::PropNameID::forUtf8(rt, "GENERATE_MIPMAP_HINT"),

            jsi::PropNameID::forUtf8(rt, "BYTE"),
            jsi::PropNameID::forUtf8(rt, "UNSIGNED_BYTE"),
            jsi::PropNameID::forUtf8(rt, "SHORT"),
            jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT"),

            jsi::PropNameID::forUtf8(rt, "INT"),
            jsi::PropNameID::forUtf8(rt, "UNSIGNED_INT"),
            jsi::PropNameID::forUtf8(rt, "FLOAT"),
            jsi::PropNameID::forUtf8(rt, "DEPTH_COMPONENT"),

            jsi::PropNameID::forUtf8(rt, "ALPHA"),
            jsi::PropNameID::forUtf8(rt, "RGB"),

            /* Clearing buffers */

            jsi::PropNameID::forUtf8(rt, "RGBA"),
            jsi::PropNameID::forUtf8(rt, "LUMINANCE"),
            jsi::PropNameID::forUtf8(rt, "LUMINANCE_ALPHA"),

            /* Clearing buffers */

            /* Rendering primitives */

            jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT_4_4_4_4"),
            jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT_5_5_5_1"),
            jsi::PropNameID::forUtf8(rt, "UNSIGNED_SHORT_5_6_5"),
            jsi::PropNameID::forUtf8(rt, "FRAGMENT_SHADER"),
            jsi::PropNameID::forUtf8(rt, "VERTEX_SHADER"),
            jsi::PropNameID::forUtf8(rt, "COMPILE_STATUS"),
            jsi::PropNameID::forUtf8(rt, "DELETE_STATUS"),

            /* Rendering primitives */

            /* Blending modes */

            jsi::PropNameID::forUtf8(rt, "LINK_STATUS"),
            jsi::PropNameID::forUtf8(rt, "VALIDATE_STATUS"),
            jsi::PropNameID::forUtf8(rt, "ATTACHED_SHADERS"),
            jsi::PropNameID::forUtf8(rt, "ACTIVE_ATTRIBUTES"),
            jsi::PropNameID::forUtf8(rt, "ACTIVE_UNIFORMS"),
            jsi::PropNameID::forUtf8(rt, "MAX_VERTEX_ATTRIBS"),
            jsi::PropNameID::forUtf8(rt, "MAX_VERTEX_UNIFORM_VECTORS"),
            jsi::PropNameID::forUtf8(rt, "MAX_VARYING_VECTORS"),
            jsi::PropNameID::forUtf8(rt, "MAX_COMBINED_TEXTURE_IMAGE_UNITS"),
            jsi::PropNameID::forUtf8(rt, "MAX_VERTEX_TEXTURE_IMAGE_UNITS"),
            jsi::PropNameID::forUtf8(rt, "MAX_TEXTURE_IMAGE_UNITS"),
            jsi::PropNameID::forUtf8(rt, "MAX_FRAGMENT_UNIFORM_VECTORS"),
            jsi::PropNameID::forUtf8(rt, "SHADER_TYPE"),
            jsi::PropNameID::forUtf8(rt, "SHADING_LANGUAGE_VERSION"),
            jsi::PropNameID::forUtf8(rt, "CURRENT_PROGRAM"),

            /* Blending modes */

            jsi::PropNameID::forUtf8(rt, "NEVER"),
            jsi::PropNameID::forUtf8(rt, "LESS"),
            jsi::PropNameID::forUtf8(rt, "EQUAL"),

            /* Blending equations */

            /* Getting GL parameter information */

            jsi::PropNameID::forUtf8(rt, "LEQUAL"),
            jsi::PropNameID::forUtf8(rt, "GREATER"),
            jsi::PropNameID::forUtf8(rt, "NOTEQUAL"),
            jsi::PropNameID::forUtf8(rt, "GEQUAL"),
            jsi::PropNameID::forUtf8(rt, "ALWAYS"),
            jsi::PropNameID::forUtf8(rt, "KEEP"),
            jsi::PropNameID::forUtf8(rt, "REPLACE"),
            jsi::PropNameID::forUtf8(rt, "INCR"),
            jsi::PropNameID::forUtf8(rt, "DECR"),
            jsi::PropNameID::forUtf8(rt, "INVERT"),
            jsi::PropNameID::forUtf8(rt, "INCR_WRAP"),
            jsi::PropNameID::forUtf8(rt, "DECR_WRAP"),
            jsi::PropNameID::forUtf8(rt, "NEAREST"),
            jsi::PropNameID::forUtf8(rt, "LINEAR"),
            jsi::PropNameID::forUtf8(rt, "NEAREST_MIPMAP_NEAREST"),
            jsi::PropNameID::forUtf8(rt, "LINEAR_MIPMAP_NEAREST"),
            jsi::PropNameID::forUtf8(rt, "NEAREST_MIPMAP_LINEAR"),
            jsi::PropNameID::forUtf8(rt, "LINEAR_MIPMAP_LINEAR"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_MAG_FILTER"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_MIN_FILTER"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_WRAP_S"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_WRAP_T"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_2D"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_BINDING_CUBE_MAP"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_POSITIVE_X"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_NEGATIVE_X"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_POSITIVE_Y"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_NEGATIVE_Y"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_POSITIVE_Z"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE_CUBE_MAP_NEGATIVE_Z"),
            jsi::PropNameID::forUtf8(rt, "MAX_CUBE_MAP_TEXTURE_SIZE"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE0"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE1"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE2"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE3"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE4"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE5"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE6"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE7"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE8"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE9"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE10"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE11"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE12"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE13"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE14"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE15"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE16"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE17"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE18"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE19"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE20"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE21"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE22"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE23"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE24"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE25"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE26"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE27"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE28"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE29"),

            /* Getting GL parameter information */

            /* Buffers */

            jsi::PropNameID::forUtf8(rt, "TEXTURE30"),
            jsi::PropNameID::forUtf8(rt, "TEXTURE31"),
            jsi::PropNameID::forUtf8(rt, "ACTIVE_TEXTURE"),
            jsi::PropNameID::forUtf8(rt, "REPEAT"),
            jsi::PropNameID::forUtf8(rt, "CLAMP_TO_EDGE"),
            jsi::PropNameID::forUtf8(rt, "MIRRORED_REPEAT"),
            jsi::PropNameID::forUtf8(rt, "FLOAT_VEC2"),

            /* Buffers */

            /* Vertex attributes */

            jsi::PropNameID::forUtf8(rt, "FLOAT_VEC3"),
            jsi::PropNameID::forUtf8(rt, "FLOAT_VEC4"),
            jsi::PropNameID::forUtf8(rt, "INT_VEC2"),
            jsi::PropNameID::forUtf8(rt, "INT_VEC3"),
            jsi::PropNameID::forUtf8(rt, "INT_VEC4"),
            jsi::PropNameID::forUtf8(rt, "BOOL"),
            jsi::PropNameID::forUtf8(rt, "BOOL_VEC2"),
            jsi::PropNameID::forUtf8(rt, "BOOL_VEC3"),

            /* Vertex attributes */

            /* Culling */

            jsi::PropNameID::forUtf8(rt, "BOOL_VEC4"),
            jsi::PropNameID::forUtf8(rt, "FLOAT_MAT2"),
            jsi::PropNameID::forUtf8(rt, "FLOAT_MAT3"),
            jsi::PropNameID::forUtf8(rt, "FLOAT_MAT4"),

            /* Culling */

            /* Enabling and disabling */

            jsi::PropNameID::forUtf8(rt, "SAMPLER_2D"),
            jsi::PropNameID::forUtf8(rt, "SAMPLER_CUBE"),
            jsi::PropNameID::forUtf8(rt, "LOW_FLOAT"),
            jsi::PropNameID::forUtf8(rt, "MEDIUM_FLOAT"),
            jsi::PropNameID::forUtf8(rt, "HIGH_FLOAT"),
            jsi::PropNameID::forUtf8(rt, "LOW_INT"),
            jsi::PropNameID::forUtf8(rt, "MEDIUM_INT"),
            jsi::PropNameID::forUtf8(rt, "HIGH_INT"),

            /* Enabling and disabling */

            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER"),
            jsi::PropNameID::forUtf8(rt, "RGBA4"),
            jsi::PropNameID::forUtf8(rt, "RGB5_A1"),
            jsi::PropNameID::forUtf8(rt, "RGB565"),
            jsi::PropNameID::forUtf8(rt, "DEPTH_COMPONENT16"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_INDEX8"),

            /* Errors */

            /* Front face directions */

            jsi::PropNameID::forUtf8(rt, "DEPTH_STENCIL"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_WIDTH"),

            /* Front face directions */

            /* Hints */

            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_HEIGHT"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_INTERNAL_FORMAT"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_RED_SIZE"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_GREEN_SIZE"),

            /* Hints */

            /* Data types */

            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_BLUE_SIZE"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_ALPHA_SIZE"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_DEPTH_SIZE"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_STENCIL_SIZE"),
            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE"),
            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_OBJECT_NAME"),
            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL"),

            /* Data types */

            /* Pixel formats */

            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE"),
            jsi::PropNameID::forUtf8(rt, "COLOR_ATTACHMENT0"),
            jsi::PropNameID::forUtf8(rt, "DEPTH_ATTACHMENT"),
            jsi::PropNameID::forUtf8(rt, "STENCIL_ATTACHMENT"),
            jsi::PropNameID::forUtf8(rt, "DEPTH_STENCIL_ATTACHMENT"),
            jsi::PropNameID::forUtf8(rt, "NONE"),

            /* Pixel formats */

            /* Pixel types */

            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_COMPLETE"),
            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_INCOMPLETE_ATTACHMENT"),
            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT"),

            /* Pixel types */

            /* Shaders */

            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_INCOMPLETE_DIMENSIONS"),
            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_UNSUPPORTED"),
            jsi::PropNameID::forUtf8(rt, "FRAMEBUFFER_BINDING"),
            jsi::PropNameID::forUtf8(rt, "RENDERBUFFER_BINDING"),
            jsi::PropNameID::forUtf8(rt, "MAX_RENDERBUFFER_SIZE"),
            jsi::PropNameID::forUtf8(rt, "INVALID_FRAMEBUFFER_OPERATION"),
            jsi::PropNameID::forUtf8(rt, "UNPACK_FLIP_Y_WEBGL"),
            jsi::PropNameID::forUtf8(rt, "UNPACK_PREMULTIPLY_ALPHA_WEBGL"),
            jsi::PropNameID::forUtf8(rt, "UNPACK_COLORSPACE_CONVERSION_WEBGL"),
    };
}

jsi::Value WebGLRenderingContext::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    auto prop_return = GetProperty(methodName);

    if(!prop_return.isUndefined()){
        return prop_return;
    }

    if(methodName == "drawingBufferWidth"){
        return {canvas_native_webgl_state_get_drawing_buffer_width(this->GetState())};
    }if(methodName == "drawingBufferHeight"){
        return {canvas_native_webgl_state_get_drawing_buffer_height(this->GetState())};
    }if(methodName == "__flipY"){
        return {canvas_native_webgl_state_get_flip_y(this->GetState())};
    }

    if (methodName == "__resized") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    // auto width = args[0];
                                                    // auto height = args[1];
                                                    // width->NumberValue(context).FromMaybe(1)
                                                    // width->NumberValue(context).FromMaybe(1))
                                                    canvas_native_webgl_resized(this->GetState());
                                                }
        );
    }

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
    } else if (methodName == "compressedTexImage2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     7,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 6) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto internalformat = (uint32_t)arguments[2].asNumber();
                                                             auto width = (int32_t)arguments[3].asNumber();
                                                             auto height = (int32_t)arguments[4].asNumber();
                                                             auto border = (int32_t)arguments[5].asNumber();

                                                             canvas_native_webgl_compressed_tex_image2d_none(
                                                                     target,
                                                                     level,
                                                                     internalformat,
                                                                     width,
                                                                     height,
                                                                     border,
                                                                     this->GetState()
                                                             );
                                                         } else if (count > 6) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto internalformat = (uint32_t)arguments[2].asNumber();
                                                             auto width = (int32_t)arguments[3].asNumber();
                                                             auto height = (int32_t)arguments[4].asNumber();
                                                             auto border = (int32_t)arguments[5].asNumber();
                                                             if(arguments[6].isObject()){
                                                                 auto pixels = arguments[6].asObject(runtime);
                                                                 if (pixels.isTypedArray(runtime)) {
                                                                     auto array = pixels.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const uint8_t>(runtime, array);
                                                                     canvas_native_webgl_compressed_tex_image2d(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             width,
                                                                             height,
                                                                             border,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                 } else if (pixels.isArrayBuffer(runtime)) {
                                                                     auto ab = pixels.getArrayBuffer(runtime);
                                                                     auto size = ab.size(runtime);
                                                                     auto data = ab.data(runtime);
                                                                     rust::Slice<const uint8_t> buf(data, size);

                                                                     canvas_native_webgl_compressed_tex_image2d(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             width,
                                                                             height,
                                                                             border,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "compressedTexSubImage2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     8,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 7) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto xoffset = (int32_t)arguments[2].asNumber();
                                                             auto yoffset = (int32_t)arguments[3].asNumber();
                                                             auto width = (int32_t)arguments[4].asNumber();
                                                             auto height = (int32_t)arguments[5].asNumber();
                                                             auto format = (uint32_t)arguments[6].asNumber();

                                                             if(arguments[7].isObject()){
                                                                 auto pixels = arguments[7].asObject(runtime);
                                                                 if (pixels.isTypedArray(runtime)) {
                                                                     auto px = pixels.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const uint8_t>(runtime, px);
                                                                     canvas_native_webgl_compressed_tex_sub_image2d(
                                                                             target,
                                                                             level,
                                                                             xoffset,
                                                                             yoffset,
                                                                             width,
                                                                             height,
                                                                             format,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                 } else if (pixels.isArrayBuffer(runtime)) {
                                                                     auto buffer = pixels.getArrayBuffer(runtime);
                                                                     auto size = buffer.size(runtime);
                                                                     auto data = buffer.data(runtime);
                                                                     rust::Slice<const uint8_t> buf(data, size);

                                                                     canvas_native_webgl_compressed_tex_sub_image2d(
                                                                             target,
                                                                             level,
                                                                             xoffset,
                                                                             yoffset,
                                                                             width,
                                                                             height,
                                                                             format,
                                                                             buf,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "copyTexImage2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     8,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 7) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto internalformat = (uint32_t)arguments[2].asNumber();
                                                             auto x = (int32_t)arguments[3].asNumber();
                                                             auto y = (int32_t)arguments[4].asNumber();
                                                             auto width = (int32_t)arguments[5].asNumber();
                                                             auto height = (int32_t)arguments[6].asNumber();
                                                             auto border = (int32_t)arguments[7].asNumber();

                                                             canvas_native_webgl_copy_tex_image2d(
                                                                     target,
                                                                     level,
                                                                     internalformat,
                                                                     x,
                                                                     y,
                                                                     width,
                                                                     height,
                                                                     border,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "copyTexSubImage2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     8,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 7) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto xoffset = (int32_t)arguments[2].asNumber();
                                                             auto yoffset = (int32_t)arguments[3].asNumber();
                                                             auto x = (int32_t)arguments[4].asNumber();
                                                             auto y = (int32_t)arguments[5].asNumber();
                                                             auto width = (int32_t)arguments[6].asNumber();
                                                             auto height = (int32_t)arguments[7].asNumber();

                                                             canvas_native_webgl_copy_tex_sub_image2d(
                                                                     target,
                                                                     level,
                                                                     xoffset,
                                                                     yoffset,
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
    } else if (methodName == "createBuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto buffer = canvas_native_webgl_create_buffer(this->GetState());
                                                         if (buffer != 0) {
                                                             auto ret = std::make_shared<WebGLBuffer>(buffer);
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createFramebuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto buffer = canvas_native_webgl_create_framebuffer(this->GetState());
                                                         if (buffer != 0) {;
                                                             auto ret = std::make_shared<WebGLFramebuffer>(buffer);
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createProgram") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto program = canvas_native_webgl_create_program(this->GetState());
                                                         if (program != 0) {
                                                             auto ret = std::make_shared<WebGLProgram>(program);
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createRenderbuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto buffer = canvas_native_webgl_create_renderbuffer(this->GetState());
                                                         if (buffer != 0) {
                                                             auto ret = std::make_shared<WebGLRenderbuffer>(buffer);
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createShader") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 0) {
                                                             return Value::undefined();
                                                         }

                                                         auto type = (uint32_t)arguments[0].asNumber();
                                                         auto shader = canvas_native_webgl_create_shader(type, this->GetState());
                                                         if (shader != 0) {
                                                             auto ret = std::make_shared<WebGLShader>(shader);
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createTexture") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto texture = canvas_native_webgl_create_texture(this->GetState());
                                                         if (texture != 0) {
                                                             auto ret = std::make_shared<WebGLTexture>(texture);
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "cullFace") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto mode = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_cull_face(
                                                                     mode,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteBuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (arguments[0].isObject()) {
                                                                 auto buffer = arguments[0].asObject(runtime).asHostObject<WebGLBuffer>(runtime);
                                                                if(buffer != nullptr){
                                                                    canvas_native_webgl_delete_buffer(
                                                                            buffer->GetBuffer(),
                                                                            this->GetState()
                                                                    );
                                                                }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteFramebuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (arguments[0].isObject()) {
                                                                 auto buffer = arguments[0].asObject(runtime).asHostObject<WebGLFramebuffer>(runtime);

                                                                 if (buffer != nullptr) {
                                                                     canvas_native_webgl_delete_framebuffer(
                                                                             buffer->GetFrameBuffer(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteProgram") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {

                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     canvas_native_webgl_delete_framebuffer(
                                                                             program->GetProgram(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteRenderbuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {

                                                             if (arguments[0].isObject()) {
                                                                 auto buffer = arguments[0].asObject(runtime).asHostObject<WebGLRenderbuffer>(runtime);
                                                                 if (buffer != nullptr) {
                                                                     canvas_native_webgl_delete_renderbuffer(
                                                                             buffer->GetRenderBuffer(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteShader") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (arguments[0].isObject()) {
                                                                 auto shader = arguments[0].asObject(runtime).asHostObject<WebGLRenderbuffer>(runtime);
                                                                 if (shader != nullptr) {
                                                                     canvas_native_webgl_delete_shader(
                                                                             shader->GetRenderBuffer(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteTexture") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (arguments[0].isObject()) {
                                                                 auto texture = arguments[0].asObject(runtime).asHostObject<WebGLTexture>(runtime);
                                                                 if (texture != nullptr) {
                                                                     canvas_native_webgl_delete_texture(
                                                                             texture->GetTexture(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "depthFunc") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto func = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_depth_func(
                                                                     func,
                                                                     this->GetState()
                                                             );
                                                         }



                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "DepthMask") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto mask = arguments[0].asBool();

                                                             canvas_native_webgl_depth_mask(
                                                                     mask,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "depthRange") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto zNear = arguments[0].asNumber();
                                                             auto zFar = arguments[1].asNumber();

                                                             canvas_native_webgl_depth_range(
                                                                     static_cast<float>(zNear),
                                                                     static_cast<float>(zFar),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "detachShader") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && arguments[0].isObject() && arguments[1].isObject())  {
                                                             auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                             auto shader = arguments[1].asObject(runtime).asHostObject<WebGLShader>(runtime);
                                                             if (program != nullptr && shader != nullptr) {
                                                                 canvas_native_webgl_detach_shader(
                                                                         program->GetProgram(),
                                                                         shader->GetShader(),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "disableVertexAttribArray") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto index = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_disable_vertex_attrib_array(
                                                                     index,
                                                                     this->GetState()
                                                             );
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "disable") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto cap = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_disable(
                                                                     cap,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawArrays") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2) {
                                                             auto mode = (uint32_t)arguments[0].asNumber();
                                                             auto first = (int32_t)arguments[1].asNumber();
                                                             auto count_ = (int32_t)arguments[2].asNumber();

                                                             canvas_native_webgl_draw_arrays(
                                                                     mode,
                                                                     first,
                                                                     count_,
                                                                     this->GetState()
                                                             );
                                                             this->UpdateInvalidateState();
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawElements") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto mode = (uint32_t)arguments[0].asNumber();
                                                             auto count_ = (int32_t)arguments[1].asNumber();
                                                             auto type = (uint32_t)arguments[2].asNumber();
                                                             auto offset = arguments[3].asNumber();

                                                             canvas_native_webgl_draw_elements(
                                                                     mode,
                                                                     count,
                                                                     type,
                                                                     static_cast<ssize_t>(offset),
                                                                     this->GetState()
                                                             );
                                                             this->UpdateInvalidateState();
                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "enableVertexAttribArray") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto index = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_enable_vertex_attrib_array(
                                                                     index,
                                                                     this->GetState()
                                                             );
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "enable") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto cap = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_enable(
                                                                     cap,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "finish") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         canvas_native_webgl_finish(
                                                                 this->GetState()
                                                         );

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "flush") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         canvas_native_webgl_flush(
                                                                 this->GetState()
                                                         );
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "framebufferRenderbuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 4) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto attachment = (uint32_t)arguments[1].asNumber();
                                                             auto renderbuffertarget = (uint32_t)arguments[2].asNumber();

                                                             if (arguments[3].isObject()) {
                                                                 auto renderbuffer = arguments[3].asObject(runtime).asHostObject<WebGLRenderbuffer>(runtime);
                                                                 if (renderbuffer != nullptr) {
                                                                     canvas_native_webgl_framebuffer_renderbuffer(
                                                                             target,
                                                                             attachment,
                                                                             renderbuffertarget,
                                                                             renderbuffer->GetRenderBuffer(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "framebufferTexture2D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 5) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto attachment = (uint32_t)arguments[1].asNumber();
                                                             auto textarget = (uint32_t)arguments[2].asNumber();
                                                             auto level = (int32_t)arguments[4].asNumber();
                                                             if (arguments[3].isObject()) {
                                                                 auto texture = arguments[3].asObject(runtime).asHostObject<WebGLTexture>(runtime);
                                                                 if (texture != nullptr) {
                                                                     canvas_native_webgl_framebuffer_texture2d(
                                                                             target,
                                                                             attachment,
                                                                             textarget,
                                                                             texture->GetTexture(),
                                                                             level,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "frontFace") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto mode = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_front_face(
                                                                     mode,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "generateMipmap") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             auto target = (uint32_t)arguments[0].asNumber();

                                                             canvas_native_webgl_generate_mipmap(
                                                                     target,
                                                                     this->GetState()
                                                             );
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getActiveAttrib") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto index = (int32_t)arguments[1].asNumber();
                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     auto info = canvas_native_webgl_get_active_attrib(
                                                                             program->GetProgram(),
                                                                             index,
                                                                             this->GetState()
                                                                     );
                                                                     auto ret = std::make_shared<WebGLActiveInfoImpl>(std::move(info));
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getActiveUniform") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto index = (int32_t)arguments[1].asNumber();
                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     auto info = canvas_native_webgl_get_active_uniform(
                                                                             program->GetProgram(),
                                                                             index,
                                                                             this->GetState()
                                                                     );
                                                                     auto ret = std::make_shared<WebGLActiveInfoImpl>(std::move(info));
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getAttachedShaders") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 0) {
                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     auto info = canvas_native_webgl_get_attached_shaders(
                                                                             program->GetProgram(),
                                                                             this->GetState()
                                                                     );
                                                                     auto len = info.size();
                                                                     auto array = jsi::Array(runtime, len);
                                                                     for (int i = 0; i < len; ++i) {
                                                                         auto shader = std::make_shared<WebGLShader>(info[i]);
                                                                         array.setValueAtIndex(runtime, i, jsi::Object::createFromHostObject(runtime, shader));
                                                                     }
                                                                     return array;
                                                                 }
                                                             }
                                                         }


                                                         return jsi::Array(runtime, 0);
                                                     }
        );
    } else if (methodName == "getAttribLocation") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto name = arguments[1].asString(runtime).utf8(runtime);
                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     auto location = canvas_native_webgl_get_attrib_location(
                                                                             program->GetProgram(),
                                                                             rust::Str(name.data(), name.size()),
                                                                             this->GetState()
                                                                     );
                                                                    return {location}
                                                                 }
                                                             }
                                                         }


                                                         return {-1};
                                                     }
        );
    } else if (methodName == "getBufferParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto pname = (uint32_t)arguments[1].asNumber();

                                                             auto param = canvas_native_webgl_get_buffer_parameter(
                                                                     target,
                                                                     pname,
                                                                     this->GetState()
                                                             );

                                                             return {param};
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getContextAttributes") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto attr = canvas_native_webgl_get_context_attributes(this->GetState());
                                                         auto ret = jsi::Object(runtime);
                                                         auto alpha = canvas_native_webgl_context_attribute_get_get_alpha(*attr);

                                                         ret.setProperty(runtime, "alpha", alpha);

                                                         auto antialias = canvas_native_webgl_context_attribute_get_get_antialias(*attr);

                                                         ret.setProperty(runtime, "antialias", antialias);

                                                         auto depth = canvas_native_webgl_context_attribute_get_get_depth(*attr);

                                                         ret.setProperty(runtime, "depth", depth);

                                                         auto fail_if_major_performance_caveat = canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
                                                                 *attr);

                                                         ret.setProperty(runtime, "failIfMajorPerformanceCaveat", fail_if_major_performance_caveat);

                                                         auto power_preference = canvas_native_webgl_context_attribute_get_get_power_preference(*attr);

                                                         ret.setProperty(runtime, "powerPreference", jsi::String::createFromAscii(runtime, power_preference.data(), power_preference.size()));

                                                         auto premultiplied_alpha = canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
                                                                 *attr);

                                                         ret.setProperty(runtime, "premultipliedAlpha", premultiplied_alpha);

                                                         auto preserve_drawing_buffer = canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
                                                                 *attr);

                                                         ret.setProperty(runtime, "preserveDrawingBuffer", preserve_drawing_buffer);

                                                         auto stencil = canvas_native_webgl_context_attribute_get_get_stencil(*attr);

                                                         ret.setProperty(runtime, "stencil", stencil);

                                                         auto desynchronized = canvas_native_webgl_context_attribute_get_get_desynchronized(*attr);

                                                         ret.setProperty(runtime, "desynchronized", desynchronized);

                                                         auto xr_compatible = canvas_native_webgl_context_attribute_get_get_xr_compatible(*attr);

                                                         ret.setProperty(runtime, "xrCompatible", xr_compatible);

                                                        return ret;
                                                     }
        );
    } else if (methodName == "getError") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto ret = canvas_native_webgl_get_error(this->GetState());

                                                         return {ret};
                                                     }
        );
    } else if (methodName == "getExtension") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count == 0) {
                                                             return Value::undefined();
                                                         }

                                                         if (!arguments[0].isString()) {
                                                             return Value::null();
                                                         }
                                                         auto name = arguments[0].asString(runtime).utf8(runtime);

                                                         auto ext = canvas_native_webgl_get_extension(rust::Str(name.c_str(), name.size()),
                                                                                                      this->GetState());

                                                         if (canvas_native_webgl_context_extension_is_none(*ext)) {
                                                             return Value::null();
                                                         }

                                                         auto type = anvas_native_webgl_context_extension_get_type(*ext);
                                                         switch (type) {
                                                             case WebGLExtensionType::EXT_blend_minmax:
                                                             {
                                                                 auto ret = std::make_shared<EXT_blend_minmaxImpl>();
                                                                 return jsi::Object::createFromHostObject(runtime, ret);
                                                             }
                                                             case WebGLExtensionType::EXT_color_buffer_half_float:
                                                             {
                                                                 auto ret = std::make_shared<EXT_color_buffer_half_floatImpl>();
                                                                 return jsi::Object::createFromHostObject(runtime, ret);
                                                             }
                                                             case WebGLExtensionType::EXT_disjoint_timer_query: {
                                                                 auto ret = canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
                                                                         std::move(ext));
                                                                 auto query = std::make_shared<EXT_disjoint_timer_queryImpl>(std::move(ret));
                                                                 return jsi::Object::createFromHostObject(runtime, query);

                                                             }
                                                                 break;
                                                             case WebGLExtensionType::EXT_sRGB:{
                                                             auto ret = std::make_shared<EXT_sRGBImpl>();
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::EXT_shader_texture_lod:{
                                                             auto ret = std::make_shared<EXT_shader_texture_lodImpl>();
                                                             return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::EXT_texture_filter_anisotropic:
                                                                 {
                                                                     auto ret = std::make_shared<EXT_texture_filter_anisotropicImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::OES_element_index_uint:
                                                                 {
                                                                     auto ret = std::make_shared<OES_element_index_uintImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::OES_standard_derivatives:
                                                                 {
                                                                     auto ret = std::make_shared<OES_standard_derivativesImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::OES_texture_float:
                                                                 {
                                                                     auto ret = std::make_shared<OES_texture_floatImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::OES_texture_float_linear:
                                                                 {
                                                                     auto ret = std::make_shared<OES_texture_float_linearImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::OES_texture_half_float:
                                                                 {
                                                                     auto ret = std::make_shared<OES_texture_half_floatImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::OES_texture_half_float_linear:
                                                                 {
                                                                     auto ret = std::make_shared<OES_texture_half_float_linearImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::OES_vertex_array_object: {
                                                                 auto ret = canvas_native_webgl_context_extension_to_oes_vertex_array_object(
                                                                         std::move(ext));
                                                                 auto array = std::make_shared<OES_vertex_array_objectImpl>(std::move(ret));
                                                                 return jsi::Object::createFromHostObject(runtime, array);
                                                             }
                                                                 break;
                                                             case WebGLExtensionType::WEBGL_color_buffer_float:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_color_buffer_floatImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_compressed_texture_atc:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_compressed_texture_atcImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_compressed_texture_etc1:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_compressed_texture_etc1Impl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_compressed_texture_s3tc:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_compressed_texture_s3tcImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_compressed_texture_s3tc_srgbImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_compressed_texture_etc:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_compressed_texture_etcImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_compressed_texture_pvrtc:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_compressed_texture_pvrtcImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_lose_context: {
                                                                 auto ret = canvas_native_webgl_context_extension_to_lose_context(std::move(ext));
                                                                 auto context = std::make_shared<WEBGL_lose_contextImpl>(std::move(ret));
                                                                 return jsi::Object::createFromHostObject(runtime, context);
                                                             }
                                                                 break;
                                                             case WebGLExtensionType::ANGLE_instanced_arrays: {
                                                                 auto ret = canvas_native_webgl_context_extension_to_angle_instanced_arrays(
                                                                         std::move(ext));
                                                                 auto instance = std::make_shared<ANGLE_instanced_arraysImpl>(std::move(ret));
                                                                 return jsi::Object::createFromHostObject(runtime, instance);
                                                             }
                                                                 break;
                                                             case WebGLExtensionType::WEBGL_depth_texture:
                                                                 {
                                                                     auto ret = std::make_shared<WEBGL_depth_textureImpl>();
                                                                     return jsi::Object::createFromHostObject(runtime, ret);
                                                                 }
                                                             case WebGLExtensionType::WEBGL_draw_buffers: {
                                                                 auto ret = canvas_native_webgl_context_extension_to_draw_buffers(std::move(ext));

                                                                 auto buffers = std::make_shared<WEBGL_draw_buffersImpl>(std::move(ret));
                                                                 return jsi::Object::createFromHostObject(runtime, buffers);
                                                             }
                                                                 break;
                                                             case WebGLExtensionType::None:
                                                                 return jsi::Value::undefined();
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getFramebufferAttachmentParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 2) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto attachment = (uint32_t)arguments[1].asNumber();
                                                             auto pname = (uint32_t)arguments[2].asNumber();
                                                             auto ret = canvas_native_webgl_get_framebuffer_attachment_parameter(
                                                                     target,
                                                                     attachment,
                                                                     pname,
                                                                     this->GetState()
                                                             );
                                                             if (canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(*ret)) {
                                                                 auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(*ret);
                                                                 auto texture = std::make_shared<WebGLTexture>(value);
                                                                 return jsi::Object::createFromHostObject(runtime, texture);
                                                             }
                                                             if (canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(*ret)) {
                                                                 auto value = canvas_native_webgl_framebuffer_attachment_parameter_get_value(*ret);
                                                                 auto buffer = std::make_shared<WebGLRenderbuffer>(value);
                                                                 return jsi::Object::createFromHostObject(runtime, buffer);
                                                             }

                                                             return {canvas_native_webgl_framebuffer_attachment_parameter_get_value(*ret)};
                                                         }


                                                         return jsi::Value::undefined();
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


                                                         // TODO remove extra allocations
                                                         if (count > 0) {
                                                             auto pname = (uint32_t)arguments[0].asNumber();
                                                             auto result = canvas_native_webgl_get_parameter(pname,
                                                                                                             this->GetState());

                                                             return GetParameterInternal(runtime, pname, std::move(result));
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getProgramInfoLog") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if ( arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     auto log = canvas_native_webgl_get_program_info_log(
                                                                             program->GetProgram(),
                                                                             this->GetState()
                                                                     );


                                                                     if (log.empty()) {
                                                                         return jsi::String::createFromAscii(runtime, "");
                                                                     }

                                                                     return jsi::String::createFromAscii(runtime, log.data(), log.size());
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::String::createFromAscii(runtime, "");
                                                     }
        );
    } else if (methodName == "getProgramParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto pname = (uint32_t)arguments[1].asNumber();
                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     auto ret = canvas_native_webgl_get_program_parameter(
                                                                             program->GetProgram(),
                                                                             pname,
                                                                             this->GetState()
                                                                     );

                                                                     if (canvas_native_webgl_result_get_is_none(*ret)) {
                                                                        return Value::null();
                                                                     }
                                                                     switch (pname) {
                                                                         case GL_DELETE_STATUS:
                                                                         case GL_LINK_STATUS:
                                                                         case GL_VALIDATE_STATUS:
                                                                             return {canvas_native_webgl_result_get_bool(*ret)};
                                                                         default:
                                                                             return {canvas_native_webgl_result_get_i32(*ret)};
                                                                     }
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getRenderbufferParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


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


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getShaderInfoLog") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 0) {
                                                             if (arguments[0].isObject()) {
                                                                 auto shader = arguments[0].asObject(runtime).asHostObject<WebGLShader>(runtime);
                                                                 if (shader != nullptr) {
                                                                     auto log = canvas_native_webgl_get_shader_info_log(
                                                                             shader->GetShader(),
                                                                             this->GetState()
                                                                     );

                                                                     return jsi::String::createFromAscii(runtime, log.data(), log.size());
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::String::createFromAscii(runtime, "");
                                                     }
        );
    } else if (methodName == "getShaderParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto pname = (uint32_t)arguments[1].asNumber();
                                                             if (arguments[0].isObject()) {
                                                                 auto shader = arguments[0].asObject(runtime).asHostObject<WebGLShader>(runtime);
                                                                 if (shader != nullptr) {
                                                                     auto ret = canvas_native_webgl_get_shader_parameter(
                                                                             shader->GetShader(),
                                                                             pname,
                                                                             this->GetState()
                                                                     );

                                                                     if (canvas_native_webgl_result_get_is_none(*ret)) {
                                                                         return Value::null();
                                                                     }

                                                                     if (pname == GL_DELETE_STATUS || pname == GL_COMPILE_STATUS) {
                                                                         return {canvas_native_webgl_result_get_bool(*ret)};
                                                                     }

                                                                     return {canvas_native_webgl_result_get_i32(*ret)};
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
                                                             auto shaderType = (uint32_t)arguments[0].asNumber();
                                                             auto precisionType = (uint32_t)arguments[1].asNumber();
                                                             auto ret = canvas_native_webgl_get_shader_precision_format(
                                                                     shaderType,
                                                                     precisionType,
                                                                     this->GetState()
                                                             );
                                                             auto shader = std::make_shared<WebGLShaderPrecisionFormatImpl>(std::move(ret));
                                                             return jsi::Object::createFromHostObject(runtime, shader);
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
                                                             if (arguments[0].isObject()) {
                                                                 auto shader = arguments[0].asObject(runtime).asHostObject<WebGLShader>(runtime);

                                                                 if (shader != nullptr) {
                                                                     auto source = canvas_native_webgl_get_shader_source(
                                                                             shader->GetShader(),
                                                                             this->GetState()
                                                                     );

                                                                    return jsi::String::createFromAscii(runtime, source.data(), source.size());
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return jsi::String::createFromAscii(runtime, "");
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

                                                         auto exts = canvas_native_webgl_get_supported_extensions(this->GetState());
                                                         auto len = exts.size();
                                                         auto array = jsi::Array(runtime, len);
                                                         for (int i = 0; i < len; ++i) {
                                                             auto item = exts[i];
                                                             array.setValueAtIndex(runtime, i, jsi::String::createFromAscii(runtime, item.data(), item.size()));
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

                                                         auto exts = canvas_native_webgl_get_supported_extensions_to_string(this->GetState());

                                                         return jsi::String::createFromAscii(runtime, exts.data(), exts.size());
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
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto pname = (uint32_t)arguments[1].asNumber();
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
                                                             if (arguments[0].isObject() && arguments[1].isString()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     auto name = arguments[1].asString(runtime).utf8(runtime);

                                                                     auto ret = canvas_native_webgl_get_uniform_location(
                                                                             program->GetProgram(),
                                                                             rust::Str(name.c_str(), name.size()),
                                                                             this->GetState()
                                                                     );

                                                                     auto location = std::make_shared<WebGLUniformLocation>(ret);

                                                                     return jsi::Object::createFromHostObject(runtime, location);
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
                                                             if (arguments[0].isObject() && arguments[1].isObject()) {

                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 auto location = arguments[1].asObject(runtime).asHostObject<WebGLUniformLocation>(runtime);


                                                                 if (program != nullptr && location != nullptr) {

                                                                     auto val = canvas_native_webgl_get_uniform(
                                                                             program,
                                                                             location,
                                                                             this->GetState());
                                                                     switch (canvas_native_webgl_result_get_type(*val)) {
                                                                         case WebGLResultType::Boolean:
                                                                            return {canvas_native_webgl_result_get_bool(*val)};
                                                                         case WebGLResultType::None:
                                                                             return Value::null();
                                                                         case WebGLResultType::String: {
                                                                             auto str = canvas_native_webgl_result_get_string(*val);
                                                                             return jsi::String::createFromAscii(runtime, str.data(), str.size());
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::BooleanArray: {
                                                                             auto ret = canvas_native_webgl_result_get_bool_array(*val);
                                                                             auto len = ret.size();
                                                                             auto array = jsi::Array(runtime, len);
                                                                             for (int i = 0; i < len; ++i) {
                                                                                 auto item = ret[i];
                                                                                 array.setValueAtIndex(runtime, i, Value(item == 1));
                                                                             }
                                                                             return array;
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::F32Array: {
                                                                             auto ret = canvas_native_webgl_result_get_f32_array(*val);

                                                                             auto buf = std::make_shared<VecMutableBuffer<float>>(std::move(ret));
                                                                             auto array = jsi::ArrayBuffer(runtime, buf);

                                                                             auto Float32Array = runtime.global()
                                                                                     .getProperty(runtime,
                                                                                                  "Float32Array")
                                                                                     .asObject(runtime)
                                                                                     .asFunction(runtime);


                                                                             return Float32Array.callAsConstructor(
                                                                                     runtime, array);
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::I32Array: {
                                                                             auto ret = canvas_native_webgl_result_get_i32_array(*val);

                                                                             auto buf = std::make_shared<VecMutableBuffer<int32_t>>(std::move(ret));
                                                                             auto array = jsi::ArrayBuffer(runtime, buf);

                                                                             auto Int32Array = runtime.global()
                                                                                     .getProperty(runtime,
                                                                                                  "Int32Array")
                                                                                     .asObject(runtime)
                                                                                     .asFunction(runtime);


                                                                             return Int32Array.callAsConstructor(
                                                                                     runtime, array);
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::U32Array: {
                                                                             auto ret = canvas_native_webgl_result_get_u32_array(*val);

                                                                             auto buf = std::make_shared<VecMutableBuffer<uint32_t>>(std::move(ret));
                                                                             auto array = jsi::ArrayBuffer(runtime, buf);

                                                                             auto Uint32Array = runtime.global()
                                                                                     .getProperty(runtime,
                                                                                                  "Uint32Array")
                                                                                     .asObject(runtime)
                                                                                     .asFunction(runtime);


                                                                             return Uint32Array.callAsConstructor(
                                                                                     runtime, array);
                                                                         }
                                                                             break;
                                                                         case WebGLResultType::F32:
                                                                             return {(double)canvas_native_webgl_result_get_f32(*val)};
                                                                         case WebGLResultType::I32:
                                                                             return { canvas_native_webgl_result_get_i32(*val)};
                                                                         case WebGLResultType::U32:
                                                                            return { canvas_native_webgl_result_get_u32(*val)};
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             auto pname = (uint32_t)arguments[1].asNumber();
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
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

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

                                                                     auto buf = std::make_shared<VecMutableBuffer<float>>(std::move(value));
                                                                     auto array = jsi::ArrayBuffer(runtime, buf);

                                                                     auto Float32Array = runtime.global()
                                                                             .getProperty(runtime,
                                                                                          "Float32Array")
                                                                             .asObject(runtime)
                                                                             .asFunction(runtime);


                                                                     return Float32Array.callAsConstructor(
                                                                             runtime, array);
                                                                 } else if (pnameValue == GL_VERTEX_ATTRIB_ARRAY_ENABLED ||
                                                                            pnameValue == GL_VERTEX_ATTRIB_ARRAY_NORMALIZED) {
                                                                     return {canvas_native_webgl_result_get_bool(*ret)};
                                                                 } else {
                                                                     return {canvas_native_webgl_result_get_i32(*ret)};
                                                                 }
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
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto mode = (uint32_t)arguments[1].asNumber();
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
                                                             if (arguments[0].isObject()) {
                                                                 auto buffer = arguments[0].asObject(runtime).asHostObject<WebGLBuffer>(runtime);
                                                                 if (buffer != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_buffer(buffer->GetBuffer(),
                                                                                                              this->GetState());

                                                                     return {ret};
                                                                 }
                                                             }
                                                         }

                                                         // todo check return
                                                         return {false};
                                                     }
        );
    } else if (methodName == "IsContextLost") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto ret = canvas_native_webgl_get_is_context_lost(this->GetState());
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
                                                             auto cap = (uint32_t)arguments[0].asNumber();
                                                             auto ret = canvas_native_webgl_is_enabled(cap,this->GetState());
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
                                                             if (arguments[0].isObject()) {
                                                                 auto framebuffer = arguments[0].asObject(runtime).asHostObject<WebGLFramebuffer>(runtime);
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
                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
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
                                                             if (arguments[0].isObject()) {
                                                                 auto renderbuffer = arguments[0].asObject(runtime).asHostObject<WebGLRenderbuffer>(runtime);
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
                                                         if (count> 0) {
                                                             if (arguments[0].isObject()) {
                                                                 auto shader = arguments[0].asObject(runtime).asHostObject<WebGLShader>(runtime);
                                                                 if (shader != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_shader(shader->GetShader(), this->GetState());
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
                                                             if (arguments[0].isObject()) {
                                                                 auto texture = arguments[0].asObject(runtime).asHostObject<WebGLTexture>(runtime);
                                                                 if (texture != nullptr) {
                                                                     auto ret = canvas_native_webgl_is_texture(
                                                                             texture->GetTexture() ,
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
                                                             auto width = arguments[0].asNumber();
                                                             canvas_native_webgl_line_width(static_cast<float>(width),
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
                                                             if (arguments[0].isObject()) {
                                                                 auto program = arguments[0].asObject(runtime).asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     canvas_native_webgl_link_program(program->GetProgram(),this->GetState());
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
                                                             auto pname = (uint32_t)arguments[0].asNumber();
                                                             auto param = (uint32_t)arguments[1].asNumber();
                                                             canvas_native_webgl_pixel_storei(
                                                                     pname,
                                                                     param,
                                                                     this->GetState()
                                                             );
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
                                                             auto factor = arguments[0].asNumber();
                                                             auto units = arguments[1].asNumber();
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
                                                             auto x = (int32_t)arguments[0].asNumber();
                                                             auto y = (int32_t)arguments[1].asNumber();
                                                             auto width = (int32_t)arguments[2].asNumber();
                                                             auto height = (int32_t)arguments[3].asNumber();
                                                             auto format = (uint32_t)arguments[4].asNumber();
                                                             auto type = (uint32_t)arguments[5].asNumber();

                                                             if(arguments[6].isObject()){
                                                                 auto pixels = arguments[6].asObject(runtime);
                                                                 if (pixels.isTypedArray(runtime)) {
                                                                        auto buf = pixels.getTypedArray(runtime);
                                                                     auto slice = GetTypedArrayData<uint8_t>(runtime, buf);
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
                                                                     return Value::undefined();
                                                                 }


                                                                 if (pixels.isArrayBuffer(runtime)) {
                                                                     auto buf = pixels.getArrayBuffer(runtime);
                                                                     auto size = buf.size(runtime);
                                                                     auto data = buf.data(runtime);
                                                                     auto slice = rust::Slice<uint8_t>(data, size);
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
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto internalFormat = (uint32_t)arguments[1].asNumber();
                                                             auto width = (int32_t)arguments[2].asNumber();
                                                             auto height = (int32_t)arguments[3].asNumber();
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
                                                             auto value = arguments[0].asNumber();
                                                             auto invert = arguments[1].asBool();
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
                                                             auto x = (int32_t)arguments[0].asNumber();
                                                             auto y = (int32_t)arguments[1].asNumber();
                                                             auto width = (int32_t)arguments[2].asNumber();
                                                             auto height = (int32_t)arguments[3].asNumber();
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
                                                             if (arguments[0].isObject() && arguments[1].isString()) {
                                                                 auto shader = arguments[0].asObject(runtime).asHostObject<WebGLShader>(runtime);
                                                                 auto source = arguments[1].asString(runtime).utf8(runtime);
                                                                 if (shader != nullptr) {
                                                                     canvas_native_webgl_shader_source(shader->GetShader(),
                                                                                                       rust::Str(source.data(),
                                                                                                                 source.size()),
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
                                                             auto face = (uint32_t)arguments[0].asNumber();
                                                             auto func = (uint32_t)arguments[1].asNumber();
                                                             auto ref = (uint32_t)arguments[2].asNumber();
                                                             auto mask = (uint32_t)arguments[3].asNumber();
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
                                                             auto func = (uint32_t)arguments[0].asNumber();
                                                             auto ref = (uint32_t)arguments[1].asNumber();
                                                             auto mask = (uint32_t)arguments[2].asNumber();
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
                                                             auto face = (uint32_t)arguments[0].asNumber();
                                                             auto mask = (uint32_t)arguments[1].asNumber();
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
                                                             auto mask = (uint32_t)arguments[0].asNumber();
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
                                                             auto face = (uint32_t)arguments[0].asNumber();
                                                             auto fail = (uint32_t)arguments[1].asNumber();
                                                             auto zfail = (uint32_t)arguments[2].asNumber();
                                                             auto zpass = (uint32_t)arguments[3].asNumber();
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
                                                             auto fail = (uint32_t)arguments[0].asNumber();
                                                             auto zfail = (uint32_t)arguments[1].asNumber();
                                                             auto zpass = (uint32_t)arguments[2].asNumber();
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
                                                             auto target = (int32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto internalformat = (int32_t)arguments[2].asNumber();
                                                             auto format = (int32_t)arguments[3].asNumber();
                                                             auto type = (int32_t)arguments[4].asNumber();

                                                             canvas_native_webgl_tex_image2d_image_none(
                                                                     target,
                                                                     level,
                                                                     internalformat,
                                                                     format,
                                                                     type,
                                                                     this->GetState()
                                                             );

                                                         } else if (count == 6) {
                                                             auto target = (int32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto internalformat = (int32_t)arguments[2].asNumber();
                                                             auto format = (int32_t)arguments[3].asNumber();
                                                             auto type = (int32_t)arguments[4].asNumber();

                                                             if(arguments[5].isObject()){
                                                                 auto pixels = arguments[5].asObject(runtime);

                                                                 auto draw_asset = [&](ImageAssetImpl &asset) {
                                                                     canvas_native_webgl_tex_image2d_asset(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             format,
                                                                             type,
                                                                             asset.GetImageAsset(),
                                                                             this->GetState()
                                                                     );
                                                                 };

                                                                 auto draw_canvas2d = [&](CanvasRenderingContext2DImpl &canvas2d) {
                                                                     canvas_native_webgl_tex_image2d_canvas2d(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             format,
                                                                             type,
                                                                             canvas2d.GetContext(),
                                                                             this->GetState()
                                                                     );
                                                                 };

                                                                 auto draw_webgl = [&](WebGLRenderingContextBase &state) {
                                                                     canvas_native_webgl_tex_image2d_webgl(
                                                                             target,
                                                                             level,
                                                                             internalformat,
                                                                             format,
                                                                             type,
                                                                             state.GetState(),
                                                                             this->GetState()
                                                                     );
                                                                 };

                                                                 if(pixels.isHostObject(runtime)){
                                                                     auto image_asset = pixels.asHostObject<ImageAssetImpl>(runtime);

                                                                     if (image_asset != nullptr) {
                                                                         draw_asset(*image_asset);
                                                                         return Value::undefined();
                                                                     }

                                                                     auto image_bitmap = pixels.asHostObject<ImageBitmapImpl>(runtime);

                                                                     if (image_bitmap != nullptr) {
                                                                         canvas_native_webgl_tex_image2d_asset(
                                                                                 target,
                                                                                 level,
                                                                                 internalformat,
                                                                                 format,
                                                                                 type,
                                                                                 image_bitmap->GetImageAsset(),
                                                                                 this->GetState()
                                                                         );
                                                                     }

                                                                 }else {
                                                                     auto is_canvas = pixels.hasProperty(runtime, "__native__context");
                                                                     auto is_asset = pixels.hasProperty(runtime, "_asset");
                                                                     auto is_video = pixels.hasProperty(runtime, "_video");

                                                                     if(is_canvas){
                                                                         auto ctx = pixels.getProperty(runtime, "__native__context");
                                                                         if(ctx.isObject()){
                                                                             auto gl_object = ctx.asObject(runtime);
                                                                             if(gl_object.isHostObject(runtime)){
                                                                                 auto canvas_2d = gl_object.asHostObject<CanvasRenderingContext2DImpl>(runtime);
                                                                                 if (canvas_2d !=
                                                                                         nullptr) {
                                                                                     draw_canvas2d(*canvas_2d);
                                                                                     return Value::undefined();
                                                                                 }

                                                                                 auto gl_context = gl_object.asHostObject<WebGLRenderingContextBase>(runtime);

                                                                                 if (gl_context != nullptr) {
                                                                                     draw_webgl(*gl_context);
                                                                                     return Value::undefined();
                                                                                 }
                                                                             }
                                                                         }
                                                                     }else if(is_asset){
                                                                         auto asset = pixels.getProperty(runtime, "_asset");
                                                                         if (asset.isObject()) {
                                                                             auto val = asset.asObject(runtime);
                                                                             if(val.isHostObject(runtime)){
                                                                                 auto image_asset = val.asHostObject<ImageAssetImpl>(runtime);
                                                                                 draw_asset(*image_asset);
                                                                             }
                                                                         }
                                                                     }
                                                                 }

                                                             }
                                                         } else if (count == 8) {
                                                             auto target = (int32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto internalformat = (int32_t)arguments[2].asNumber();
                                                             auto width = (int32_t)arguments[3].asNumber();
                                                             auto height = (int32_t)arguments[4].asNumber();
                                                             auto border = (int32_t)arguments[5].asNumber();
                                                             auto format = (int32_t)arguments[6].asNumber();
                                                             auto type = (int32_t)arguments[7].asNumber();
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
                                                             auto target = (int32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto internalformat = (int32_t)arguments[2].asNumber();
                                                             auto width = (int32_t)arguments[3].asNumber();
                                                             auto height = (int32_t)arguments[4].asNumber();
                                                             auto border = (int32_t)arguments[5].asNumber();
                                                             auto format = (int32_t)arguments[6].asNumber();
                                                             auto type = (int32_t)arguments[7].asNumber();


                                                             if(arguments[8].isObject()){
                                                                 auto pixels = arguments[8].asObject(runtime);
                                                                 if (pixels.isTypedArray(runtime)) {
                                                                     auto typedArray = pixels.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<uint8_t>(runtime,typedArray);
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
                                                                     return Value::undefined();
                                                                 } else if (pixels.isArrayBuffer(runtime)) {
                                                                     auto array = pixels.getArrayBuffer(runtime);
                                                                     auto data = array.data(runtime);
                                                                     auto size = array.size(runtime);
                                                                     rust::Slice<uint8_t> buf(data, size);
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
                                                                     return Value::undefined();
                                                                 }
                                                             }else if(arguments[8].isNull() || arguments[8].isUndefined()){
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
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto pname = (uint32_t)arguments[1].asNumber();
                                                             auto param = arguments[2].asNumber();
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
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto pname = (uint32_t)arguments[1].asNumber();
                                                             auto param = (int32_t)arguments[2].asNumber();
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
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto xoffset = (int32_t)arguments[2].asNumber();
                                                             auto yoffset = (int32_t)arguments[3].asNumber();
                                                             auto format = (uint32_t)arguments[4].asNumber();
                                                             auto type = (int32_t)arguments[5].asNumber();


                                                             if(arguments[6].isObject()){
                                                                 auto pixels = arguments[6].asObject(runtime);
                                                                 if(pixels.isHostObject(runtime)){
                                                                     auto asset = pixels.asHostObject<ImageAssetImpl>(runtime);
                                                                     if(asset != nullptr){
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
                                                                         return Value::undefined();
                                                                     }

                                                                     auto bitmap = pixels.asHostObject<ImageBitmapImpl>(runtime);

                                                                    if(bitmap != nullptr){
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

                                                                        return Value::undefined();
                                                                    }

                                                                 }
                                                             }

                                                         } else if (count == 9) {
                                                             auto target = (uint32_t)arguments[0].asNumber();
                                                             auto level = (int32_t)arguments[1].asNumber();
                                                             auto xoffset = (int32_t)arguments[2].asNumber();
                                                             auto yoffset = (int32_t)arguments[3].asNumber();
                                                             auto width = (int32_t)arguments[4].asNumber();
                                                             auto height = (int32_t)arguments[5].asNumber();
                                                             auto format = (uint32_t)arguments[6].asNumber();
                                                             auto type = (int32_t)arguments[7].asNumber();

                                                             if(arguments[8].isObject()){
                                                                 auto pixels = arguments[8].asObject(runtime);

                                                                 if (pixels.isTypedArray(runtime)) {
                                                                     auto array = pixels.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const uint8_t>(runtime,array);
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
                                                                 } else if (pixels.isArrayBuffer(runtime)) {
                                                                     auto array = pixels.getArrayBuffer(runtime);
                                                                     auto data = array.data(runtime);
                                                                     auto size = array.size(runtime);
                                                                     rust::Slice<const uint8_t> buf(data, size);
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             auto v0 = static_cast<float>(arguments[1].asNumber());
                                                             canvas_native_webgl_vertex_attrib1f(index, v0, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             if (arguments[1].isObject()) {
                                                                 auto v0 = arguments[1].asObject(runtime);
                                                                 if(v0.isFloat32Array(runtime)){
                                                                     auto array = v0.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const float>(runtime,array);
                                                                     canvas_native_webgl_vertex_attrib1fv(index, buf, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             auto v0 = static_cast<float>(arguments[1].asNumber());
                                                             auto v1 = static_cast<float>(arguments[2].asNumber());
                                                             canvas_native_webgl_vertex_attrib2f(index, v0, v1, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             if (arguments[1].isObject()) {
                                                                 auto v0 = arguments[1].asObject(runtime);
                                                                 if(v0.isFloat32Array(runtime)){
                                                                     auto array = v0.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const float>(runtime,array);
                                                                     canvas_native_webgl_vertex_attrib2fv(index, buf, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             auto v0 = static_cast<float>(arguments[1].asNumber());
                                                             auto v1 = static_cast<float>(arguments[2].asNumber());
                                                             auto v2 = static_cast<float>(arguments[3].asNumber());
                                                             canvas_native_webgl_vertex_attrib3f(index, v0, v1, v2, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             if (arguments[1].isObject()) {
                                                                 auto v0 = arguments[1].asObject(runtime);
                                                                 if(v0.isFloat32Array(runtime)){
                                                                     auto array = v0.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const float>(runtime,array);
                                                                     canvas_native_webgl_vertex_attrib3fv(index, buf, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             auto v0 = static_cast<float>(arguments[1].asNumber());
                                                             auto v1 = static_cast<float>(arguments[2].asNumber());
                                                             auto v2 = static_cast<float>(arguments[3].asNumber());
                                                             auto v3 = static_cast<float>(arguments[4].asNumber());
                                                             canvas_native_webgl_vertex_attrib4f(index, v0, v1, v2, v3, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             if (arguments[1].isObject()) {
                                                                 auto v0 = arguments[1].asObject(runtime);
                                                                 if(v0.isFloat32Array(runtime)){
                                                                     auto array = v0.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const float>(runtime,array);
                                                                     canvas_native_webgl_vertex_attrib4fv(index, buf, this->GetState());
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
                                                             auto index = (uint32_t)arguments[0].asNumber();
                                                             auto size = (int32_t)arguments[1].asNumber();
                                                             auto type = (uint32_t)arguments[2].asNumber();
                                                             auto normalized = arguments[3].asBool();
                                                             auto stride = (int32_t)arguments[4].asNumber();
                                                             auto offset = static_cast<ssize_t>(arguments[5].asNumber());
                                                             canvas_native_webgl_vertex_attrib_pointer(index, size, type, normalized, stride, offset,
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

                                                         if (count > 1) {
                                                             if (arguments[0].isObject()) {
                                                                 auto location = arguments[0].asObject(runtime).asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = arguments[1].asNumber();
                                                                 if (location != nullptr) {
                                                                     canvas_native_webgl_uniform1f(
                                                                             location->GetUniformLocation(),
                                                                             static_cast<float>(v0),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

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
                                                             if (arguments[0].isObject() && arguments[1].isObject()) {
                                                                 auto locationObject = arguments[0].asObject(runtime);
                                                                 if(locationObject.isHostObject(runtime)){
                                                                     auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                     auto v0 = arguments[1].asObject(runtime);

                                                                     if (location != nullptr) {
                                                                         if (v0.isInt32Array(runtime)) {
                                                                             auto array = v0.getTypedArray(runtime);
                                                                             auto buf = GetTypedArrayData<const int32_t>(runtime, array);
                                                                             canvas_native_webgl_uniform1iv(location->GetUniformLocation(), buf,
                                                                                                            this->GetState());
                                                                         } else if (v0.isArray(runtime)) {
                                                                             auto array = v0.getArray(runtime);
                                                                             auto len = array.size(runtime);

                                                                             std::vector<int32_t> buf;
                                                                             buf.reserve(len);
                                                                             for (int i = 0; i < len; ++i) {
                                                                                 auto item = array.getValueAtIndex(runtime, (uint32_t) i).asNumber();
                                                                                 buf.push_back(static_cast<int32_t>(item));
                                                                             }
                                                                             rust::Slice<const int32_t> slice(buf.data(), buf.size());
                                                                             canvas_native_webgl_uniform1iv(location->GetUniformLocation(), slice,
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

                                                         if (count > 1 && arguments[0].isObject() && arguments[1].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             auto v0 = arguments[1].asObject(runtime);

                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 if(location != nullptr){
                                                                     if (v0.isFloat32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const float>(runtime, array);
                                                                         canvas_native_webgl_uniform1fv(location, buf,
                                                                                                        this->GetState());
                                                                     } else if (v0.isArray(runtime)) {
                                                                         auto array = v0.getArray(runtime);
                                                                         auto len = array.size(runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0; i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(runtime, i);
                                                                             if (!item.isNumber()) {
                                                                                 buf.push_back(nanf(""));
                                                                             } else {
                                                                                 auto value = item.asNumber();
                                                                                 buf.push_back(static_cast<float>(value));
                                                                             }
                                                                         }
                                                                         rust::Slice<const float> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform1fv(location->GetUniformLocation(), slice,
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

                                                         if (count > 1 && arguments[0].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = (int32_t)arguments[1].asNumber();
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
                                                             if (arguments[0].isObject()) {
                                                                 auto locationObject = arguments[0].asObject(runtime);
                                                                 if(locationObject.isHostObject(runtime)){
                                                                     auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                     auto v0 = arguments[1].asNumber();
                                                                     auto v1 = arguments[2].asNumber();
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

                                                         if (count > 1 && arguments[0].isObject() && arguments[1].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = arguments[1].asObject(runtime);
                                                                 if(location != nullptr){
                                                                     if (v0.isInt32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const int32_t>(runtime,array);
                                                                         canvas_native_webgl_uniform2iv(location->GetUniformLocation(), buf,
                                                                                                        this->GetState());
                                                                     } else if (v0.isArray(runtime)) {
                                                                         auto array = v0.asArray(runtime);
                                                                         auto len = array.size(runtime);

                                                                         std::vector<int32_t> buf;
                                                                         buf.reserve(len);
                                                                         for (size_t i = 0; i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(runtime, i).asNumber();
                                                                             buf.push_back(static_cast<int32_t>(item));
                                                                         }
                                                                         rust::Slice<const int32_t> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform2iv(location->GetUniformLocation(), slice,
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

                                                         if (count > 1 && arguments[0].isObject() && arguments[1].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = arguments[1].asObject(runtime);
                                                                 if(location != nullptr){
                                                                     if (v0.isFloat32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const float>(runtime,array);
                                                                         canvas_native_webgl_uniform2fv(location->GetUniformLocation(), buf,
                                                                                                        this->GetState());
                                                                     } else if (v0.isArray(runtime)) {
                                                                         auto array = v0.asArray(runtime);
                                                                         auto len = array.size(runtime);

                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (size_t i = 0; i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(runtime, i).asNumber();
                                                                             buf.push_back(static_cast<float>(item));
                                                                         }
                                                                         rust::Slice<const float> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform2fv(location->GetUniformLocation(), slice,
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

                                                         if (count > 2 && arguments[0].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = (int32_t)arguments[1].asNumber();
                                                                 auto v1 = (int32_t)arguments[2].asNumber();
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

                                                         if (count > 3 && arguments[0].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                            if(locationObject.isHostObject(runtime)){
                                                                auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                auto v0 = arguments[1].asNumber();
                                                                auto v1 = arguments[2].asNumber();
                                                                auto v2 = arguments[3].asNumber();
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


                                                         if (count > 1 && arguments[0].isObject() && arguments[1].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = arguments[1].asObject(runtime);

                                                                 if (location != nullptr) {
                                                                     if (v0.isInt32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const int32_t>(runtime, array);
                                                                         canvas_native_webgl_uniform3iv(location->GetUniformLocation(), buf,
                                                                                                        this->GetState());
                                                                     } else if (v0.isArray(runtime)) {
                                                                         auto array = v0.asArray(runtime);
                                                                         auto len = array.size(runtime);

                                                                         std::vector<int32_t> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0; i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(runtime, (uint32_t) i).asNumber();
                                                                             buf.push_back(static_cast<int32_t>(item));
                                                                         }
                                                                         rust::Slice<const int32_t> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform3iv(location->GetUniformLocation(), slice,
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

                                                         if (count > 1  && arguments[0].isObject() && arguments[1].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             auto v0 = arguments[1].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);

                                                                 if(location != nullptr){


                                                                     if (v0.isFloat32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const float>(runtime, array);
                                                                         canvas_native_webgl_uniform3fv(location->GetUniformLocation(), buf,
                                                                                                        this->GetState());
                                                                     } else if (v0.isArray(runtime)) {
                                                                         auto array = v0.getArray(runtime);
                                                                         auto len = array.size(runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0; i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(runtime, i);
                                                                             if (!item.isNumber()) {
                                                                                 buf.push_back(nanf(""));
                                                                             } else {
                                                                                 auto value = item.asNumber();
                                                                                 buf.push_back(static_cast<float>(value));
                                                                             }
                                                                         }
                                                                         rust::Slice<const float> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform3fv(location->GetUniformLocation(), slice,
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

                                                         if (count > 3 && arguments[0].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = (int32_t)arguments[1].asNumber();
                                                                 auto v1 = (int32_t)arguments[2].asNumber();
                                                                 auto v2 = (int32_t)arguments[3].asNumber();

                                                                 if(location != nullptr){
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

                                                         if (count > 4 && arguments[0].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = arguments[1].asNumber();
                                                                 auto v1 = arguments[2].asNumber();
                                                                 auto v2 = arguments[3].asNumber();
                                                                 auto v3 = arguments[4].asNumber();

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

                                                         if (count > 1 && arguments[0].isObject() && arguments[1].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             auto v0 = arguments[1].asObject(runtime);

                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 if(location != nullptr){}
                                                                 if (v0.isInt32Array(runtime)) {
                                                                     auto array = v0.getTypedArray(runtime);
                                                                     auto buf = GetTypedArrayData<const int32_t>(runtime, array);
                                                                     canvas_native_webgl_uniform4iv(location->GetUniformLocation() buf,
                                                                                                    this->GetState());
                                                                 } else if (v0.isArray(runtime)) {
                                                                     auto array = v0.getArray(runtime);
                                                                     auto len = array.size(runtime);

                                                                     std::vector<int32_t> buf;
                                                                     buf.reserve(len);
                                                                     for (int i = 0; i < len; ++i) {
                                                                         auto item = array.getValueAtIndex(runtime, (uint32_t) i).asNumber();
                                                                         buf.push_back(static_cast<int32_t>(item));
                                                                     }
                                                                     rust::Slice<const int32_t> slice(buf.data(), buf.size());
                                                                     canvas_native_webgl_uniform4iv(location->GetUniformLocation(), slice,
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

                                                         if (count > 1 && arguments[0].isObject() && arguments[1].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             auto v0 = arguments[1].asObject(runtime);

                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 if(location != nullptr){
                                                                     if (v0.isFloat32Array(runtime)) {
                                                                         auto array = v0.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const float>(runtime, array);
                                                                         canvas_native_webgl_uniform4fv(location->GetUniformLocation(), buf,
                                                                                                        this->GetState());
                                                                     } else if (v0.isArray(runtime)) {
                                                                         auto array = v0.getArray(runtime);
                                                                         auto len = array.size(runtime);

                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);
                                                                         for (int i = 0; i < len; ++i) {
                                                                             auto item = array.getValueAtIndex(runtime, (uint32_t) i).asNumber();
                                                                             buf.push_back(static_cast<float>(item));
                                                                         }
                                                                         rust::Slice<const float> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform4fv(location->GetUniformLocation(), slice,
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

                                                         if (count > 4 && arguments[0].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto v0 = (int32_t)arguments[1].asNumber();
                                                                 auto v1 = (int32_t)arguments[2].asNumber();
                                                                 auto v2 = (int32_t)arguments[3].asNumber();
                                                                 auto v3 = (int32_t)arguments[4].asNumber();

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

                                                         if (count > 2 && arguments[0].isObject() && arguments[2].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto transpose = arguments[1].asBool();
                                                                 auto value = arguments[2].asObject(runtime);

                                                                 if (location != nullptr) {
                                                                     if (value.isFloat32Array(runtime)) {
                                                                         auto array = value.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const float>(runtime, array);
                                                                         canvas_native_webgl_uniform_matrix2fv(location->GetUniformLocation(),
                                                                                                               transpose, buf,
                                                                                                               this->GetState());
                                                                     } else if (value.isArray(runtime)) {
                                                                         auto array = value.getArray(runtime);
                                                                         auto len = array.size(runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);

                                                                         for (int i = 0; i < len; i++) {
                                                                             auto item = array.getValueAtIndex(runtime, i);
                                                                             if (item.isNumber()) {
                                                                                 buf.push_back(
                                                                                         static_cast<float>(item.asNumber()));
                                                                             } else {
                                                                                 buf.push_back(std::nanf(""));
                                                                             }
                                                                         }

                                                                         rust::Slice<const float> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform_matrix2fv(location->GetUniformLocation(),
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

                                                         if (count > 2 && arguments[0].isObject() && arguments[2].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto transpose = arguments[1].asBool();
                                                                 auto value = arguments[2].asObject(runtime);

                                                                 if (location != nullptr) {
                                                                     if (value.isFloat32Array(runtime)) {
                                                                         auto array = value.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const float>(runtime, array);
                                                                         canvas_native_webgl_uniform_matrix3fv(location->GetUniformLocation(),
                                                                                                               transpose, buf,
                                                                                                               this->GetState());
                                                                     } else if (value.isArray(runtime)) {
                                                                         auto array = value.getArray(runtime);
                                                                         auto len = array.size(runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);

                                                                         for (int i = 0; i < len; i++) {
                                                                             auto item = array.getValueAtIndex(runtime, i);
                                                                             if (item.isNumber()) {
                                                                                 buf.push_back(
                                                                                         static_cast<float>(item.asNumber()));
                                                                             } else {
                                                                                 buf.push_back(std::nanf(""));
                                                                             }
                                                                         }

                                                                         rust::Slice<const float> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform_matrix3fv(location->GetUniformLocation(),
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

                                                         if (count > 2 && arguments[0].isObject() && arguments[2].isObject()) {
                                                             auto locationObject = arguments[0].asObject(runtime);
                                                             if(locationObject.isHostObject(runtime)){
                                                                 auto location = locationObject.asHostObject<WebGLUniformLocation>(runtime);
                                                                 auto transpose = arguments[1].asBool();
                                                                 auto value = arguments[2].asObject(runtime);

                                                                 if (location != nullptr) {
                                                                     if (value.isFloat32Array(runtime)) {
                                                                         auto array = value.getTypedArray(runtime);
                                                                         auto buf = GetTypedArrayData<const float>(runtime, array);
                                                                         canvas_native_webgl_uniform_matrix4fv(location->GetUniformLocation(),
                                                                                                               transpose, buf,
                                                                                                               this->GetState());
                                                                     } else if (value.isArray(runtime)) {
                                                                         auto array = value.getArray(runtime);
                                                                         auto len = array.size(runtime);
                                                                         std::vector<float> buf;
                                                                         buf.reserve(len);

                                                                         for (int i = 0; i < len; i++) {
                                                                             auto item = array.getValueAtIndex(runtime, i);
                                                                             if (item.isNumber()) {
                                                                                 buf.push_back(
                                                                                         static_cast<float>(item.asNumber()));
                                                                             } else {
                                                                                 buf.push_back(std::nanf(""));
                                                                             }
                                                                         }

                                                                         rust::Slice<const float> slice(buf.data(), buf.size());
                                                                         canvas_native_webgl_uniform_matrix4fv(location->GetUniformLocation(),
                                                                                                               transpose, slice,
                                                                                                               this->GetState());
                                                                     }
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

                                                         if (count > 0 && arguments[0].isObject()) {
                                                             auto programObject = arguments[0].asObject(runtime);
                                                             if(programObject.isHostObject(runtime)){
                                                                 auto program = programObject.asHostObject<WebGLProgram>(runtime);
                                                                 if (program != nullptr) {
                                                                     canvas_native_webgl_use_program(
                                                                             program->GetProgram(),
                                                                             this->GetState()
                                                                     );
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

                                                         if (count > 0 && arguments[0].isObject()) {
                                                             auto programObject = arguments[0].asObject(runtime);
                                                             if(programObject.isHostObject(runtime)){
                                                                 auto program = programObject.asHostObject<WebGLProgram>(runtime);
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
                                                             auto x = (int32_t)arguments[0].asNumber();
                                                             auto y = (int32_t)arguments[1].asNumber();
                                                             auto width = (int32_t)arguments[2].asNumber();
                                                             auto height = (int32_t)arguments[3].asNumber();
                                                             canvas_native_webgl_viewport(x, y, width, height, this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "__toDataURL") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         std::string type("image/png");
                                                         int quality = 92;
                                                         if (arguments[0].isString()) {
                                                             type = arguments[0].asString(runtime).utf8(
                                                                     runtime);
                                                         }


                                                         if (arguments[1].isNumber()) {
                                                             quality = (int) arguments[1].asNumber();
                                                         }


                                                         auto data = canvas_native_webgl_to_data_url(this->GetState(),
                                                                                                     rust::Str(type.data(), type.size()),
                                                                                                     quality);
                                                         return jsi::String::createFromAscii(runtime,
                                                                                             data.data(),
                                                                                             data.size());
                                                     }
        );
    }


    return Value::undefined();
}

WebGLRenderingContext::~WebGLRenderingContext() {}

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
