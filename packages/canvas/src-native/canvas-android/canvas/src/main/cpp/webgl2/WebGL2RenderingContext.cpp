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


std::vector<jsi::PropNameID> WebGL2RenderingContext::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;

    // gl2 + gl1 props
    ret.reserve(351 + 434);

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


    if (methodName == "beginQuery") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && arguments[0].isNumber() &&
                                                             arguments[1].isObject()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto queryObject = arguments[1].asObject(
                                                                     runtime);

                                                             if (queryObject.isHostObject(
                                                                     runtime)) {
                                                                 auto query = queryObject.asHostObject<WebGLQuery>(
                                                                         runtime);
                                                                 canvas_native_webgl2_begin_query(
                                                                         target,
                                                                         query->GetQuery(),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "beginTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0 && arguments[0].isNumber()) {
                                                             canvas_native_webgl2_begin_transform_feedback(
                                                                     (uint32_t) arguments[0].asNumber(),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindBufferBase") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && arguments[2].isObject()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto index = (uint32_t) arguments[1].asNumber();
                                                             auto bufferObject = arguments[2].asObject(
                                                                     runtime);
                                                             if (bufferObject.isHostObject(
                                                                     runtime)) {
                                                                 auto buffer = bufferObject.asHostObject<WebGLBuffer>(
                                                                         runtime);

                                                                 canvas_native_webgl2_bind_buffer_base(
                                                                         target,
                                                                         index,
                                                                         buffer->GetBuffer(),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindBufferRange") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 4 && arguments[2].isObject()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto index = (uint32_t) arguments[1].asNumber();
                                                             auto offset = arguments[3].asNumber();
                                                             auto size = arguments[4].asNumber();
                                                             auto bufferObject = arguments[2].asObject(
                                                                     runtime);
                                                             if (bufferObject.isHostObject(
                                                                     runtime)) {
                                                                 auto buffer = bufferObject.asHostObject<WebGLBuffer>(
                                                                         runtime);
                                                                 canvas_native_webgl2_bind_buffer_range(
                                                                         target,
                                                                         index,
                                                                         buffer->GetBuffer(),
                                                                         static_cast<ssize_t>(offset),
                                                                         static_cast<ssize_t>(size),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "bindSampler") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto samplerObject = arguments[1].asObject(
                                                                     runtime);
                                                             if (samplerObject.isHostObject(
                                                                     runtime)) {
                                                                 auto unit = (uint32_t) arguments[0].asNumber();
                                                                 auto sampler = samplerObject.asHostObject<WebGLSampler>(
                                                                         runtime);

                                                                 canvas_native_webgl2_bind_sampler(
                                                                         unit,
                                                                         sampler->GetSampler(),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "bindTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto transformFeedbackObject = arguments[1].asObject(
                                                                     runtime);
                                                             if (transformFeedbackObject.isHostObject(
                                                                     runtime)) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();
                                                                 auto transformFeedback = transformFeedbackObject.asHostObject<WebGLTransformFeedback>(
                                                                         runtime);

                                                                 canvas_native_webgl2_bind_transform_feedback(
                                                                         target,
                                                                         transformFeedback->GetFeedback(),
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bindVertexArray") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 0) {
                                                             if (arguments[0].isNull()) {
                                                                 canvas_native_webgl2_bind_vertex_array(
                                                                         0,
                                                                         this->GetState()
                                                                 );
                                                                 return jsi::Value::undefined();
                                                             }

                                                             if (arguments[0].isObject()) {
                                                                 auto vertexArrayObject = arguments[0].asObject(
                                                                         runtime);
                                                                 if (vertexArrayObject.isHostObject(
                                                                         runtime)) {
                                                                     auto vertexArray = vertexArrayObject.asHostObject<WebGLVertexArrayObject>(
                                                                             runtime);

                                                                     if (vertexArray != nullptr) {

                                                                         canvas_native_webgl2_bind_vertex_array(
                                                                                 vertexArray->GetVertexArrayObject(),
                                                                                 this->GetState()
                                                                         );
                                                                     }
                                                                 }
                                                             }

                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "blitFramebuffer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     10,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 9) {
                                                             auto srcX0 = (int32_t) arguments[0].asNumber();
                                                             auto srcY0 = (int32_t) arguments[1].asNumber();

                                                             auto srcX1 = (int32_t) arguments[2].asNumber();
                                                             auto srcY1 = (int32_t) arguments[3].asNumber();

                                                             auto dstX0 = (int32_t) arguments[4].asNumber();
                                                             auto dstY0 = (int32_t) arguments[5].asNumber();

                                                             auto dstX1 = (int32_t) arguments[6].asNumber();
                                                             auto dstY1 = (int32_t) arguments[7].asNumber();

                                                             auto mask = (uint32_t) arguments[8].asNumber();
                                                             auto filter = (uint32_t) arguments[9].asNumber();
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
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "clearBufferfv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && arguments[0].isObject() &&
                                                             arguments[1].isObject()) {
                                                             auto bufferObject = arguments[0].asObject(
                                                                     runtime);
                                                             auto drawbuffer = (int32_t) arguments[1].asNumber();
                                                             auto values = arguments[2].asObject(
                                                                     runtime);
                                                             if (bufferObject.isHostObject(
                                                                     runtime)) {
                                                                 auto buffer = bufferObject.asHostObject<WebGLBuffer>(
                                                                         runtime);
                                                                 if (values.isArray(runtime)) {
                                                                     auto array = values.getArray(
                                                                             runtime);
                                                                     auto len = array.size(runtime);
                                                                     rust::Vec<float> buf;
                                                                     buf.reserve(len);
                                                                     for (int j = 0; j < len; ++j) {
                                                                         auto item = array.getValueAtIndex(
                                                                                 runtime, j);
                                                                         if (!item.isNumber()) {
                                                                             buf.push_back(
                                                                                     std::nanf(""));
                                                                         } else {
                                                                             buf.push_back(
                                                                                     static_cast<float>(item.asNumber())
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
                                                                             this->GetState()
                                                                     );

                                                                 } else if (values.isFloat32Array(
                                                                         runtime)) {
                                                                     auto buff = values.getTypedArray(
                                                                             runtime);
                                                                     auto slice = GetTypedArrayData<const float>(
                                                                             runtime, buff);
                                                                     canvas_native_webgl2_clear_bufferfv(
                                                                             buffer->GetBuffer(),
                                                                             drawbuffer,
                                                                             slice,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clearBufferiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 2 && arguments[0].isObject() &&
                                                             arguments[1].isObject()) {
                                                             auto bufferObject = arguments[0].asObject(
                                                                     runtime);
                                                             auto drawbuffer = (int32_t) arguments[1].asNumber();
                                                             auto values = arguments[2].asObject(
                                                                     runtime);
                                                             if (bufferObject.isHostObject(
                                                                     runtime)) {
                                                                 auto buffer = bufferObject.asHostObject<WebGLBuffer>(
                                                                         runtime);
                                                                 if (values.isArray(runtime)) {
                                                                     auto array = values.getArray(
                                                                             runtime);
                                                                     auto len = array.size(runtime);
                                                                     rust::Vec<int32_t> buf;
                                                                     buf.reserve(len);
                                                                     for (int j = 0; j < len; ++j) {
                                                                         auto item = array.getValueAtIndex(
                                                                                 runtime, j);
                                                                         buf.push_back(
                                                                                 static_cast<int32_t>(item.asNumber())
                                                                         );
                                                                     }
                                                                     rust::Slice<const int32_t> slice(
                                                                             buf.data(),
                                                                             buf.size());

                                                                     canvas_native_webgl2_clear_bufferiv(
                                                                             buffer->GetBuffer(),
                                                                             drawbuffer,
                                                                             slice,
                                                                             this->GetState()
                                                                     );

                                                                 } else if (values.isInt32Array(
                                                                         runtime)) {
                                                                     auto buff = values.getTypedArray(
                                                                             runtime);
                                                                     auto slice = GetTypedArrayData<const int32_t>(
                                                                             runtime, buff);
                                                                     canvas_native_webgl2_clear_bufferiv(
                                                                             buffer->GetBuffer(),
                                                                             drawbuffer,
                                                                             slice,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clearBufferuiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 2 && arguments[0].isObject() &&
                                                             arguments[1].isObject()) {
                                                             auto bufferObject = arguments[0].asObject(
                                                                     runtime);
                                                             auto drawbuffer = (int32_t) arguments[1].asNumber();
                                                             auto values = arguments[2].asObject(
                                                                     runtime);
                                                             if (bufferObject.isHostObject(
                                                                     runtime)) {
                                                                 auto buffer = bufferObject.asHostObject<WebGLBuffer>(
                                                                         runtime);
                                                                 if (values.isArray(runtime)) {
                                                                     auto array = values.getArray(
                                                                             runtime);
                                                                     auto len = array.size(runtime);
                                                                     rust::Vec<uint32_t> buf;
                                                                     buf.reserve(len);
                                                                     for (int j = 0; j < len; ++j) {
                                                                         auto item = array.getValueAtIndex(
                                                                                 runtime, j);
                                                                         buf.push_back(
                                                                                 static_cast<uint32_t>(item.asNumber())
                                                                         );
                                                                     }

                                                                     rust::Slice<const uint32_t> slice(
                                                                             buf.data(),
                                                                             buf.size());

                                                                     canvas_native_webgl2_clear_bufferuiv(
                                                                             buffer->GetBuffer(),
                                                                             drawbuffer,
                                                                             slice,
                                                                             this->GetState()
                                                                     );

                                                                 } else if (values.isUint32Array(
                                                                         runtime)) {
                                                                     auto buff = values.getTypedArray(
                                                                             runtime);
                                                                     auto slice = GetTypedArrayData<const uint32_t>(
                                                                             runtime, buff);
                                                                     canvas_native_webgl2_clear_bufferuiv(
                                                                             buffer->GetBuffer(),
                                                                             drawbuffer,
                                                                             slice,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "clientWaitSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 2 && arguments[0].isObject()) {
                                                             auto syncObject = arguments[0].asObject(
                                                                     runtime);

                                                             if (syncObject.isHostObject(runtime)) {
                                                                 auto sync = syncObject.asHostObject<WebGLSyncImpl>(
                                                                         runtime);
                                                                 if (sync != nullptr) {
                                                                     auto flags = (uint32_t) arguments[1].asNumber();
                                                                     auto timeout = arguments[2].asNumber();
                                                                     auto ret = canvas_native_webgl2_client_wait_sync(
                                                                             sync->GetSync(),
                                                                             flags,
                                                                             static_cast<ssize_t>(timeout),
                                                                             this->GetState()
                                                                     );
                                                                     return {(int32_t) ret};
                                                                 }
                                                             }
                                                         }
                                                         // todo decide if WAIT_FAILED should be returned here
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "compressedTexSubImage3D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     12,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 8) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto level = (int32_t) arguments[1].asNumber();
                                                             auto xoffset = (int32_t) arguments[2].asNumber();
                                                             auto yoffset = (int32_t) arguments[3].asNumber();
                                                             auto zoffset = (int32_t) arguments[4].asNumber();
                                                             auto width = (int32_t) arguments[5].asNumber();
                                                             auto height = (int32_t) arguments[6].asNumber();
                                                             auto depth = (int32_t) arguments[7].asNumber();
                                                             auto format = (uint32_t) arguments[8].asNumber();

                                                             if (arguments[9].isObject()) {
                                                                 auto imageSizeOrBuf = arguments[9].asObject(
                                                                         runtime);
                                                                 if (imageSizeOrBuf.isTypedArray(
                                                                         runtime)) {
                                                                     auto array = imageSizeOrBuf.getTypedArray(
                                                                             runtime);
                                                                     auto slice = GetTypedArrayData<const uint8_t>(
                                                                             runtime, array);

                                                                     size_t srcOffset = 0;
                                                                     if (arguments[10].isNumber()) {
                                                                         srcOffset = static_cast<size_t>(arguments[10].asNumber());
                                                                     }
                                                                     size_t srcLengthOverride = 0;
                                                                     if (arguments[11].isNumber()) {
                                                                         srcLengthOverride = static_cast<size_t>(arguments[11].asNumber());
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
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             } else {
                                                                 auto imageSizeOrBuf = (int32_t) arguments[0].asNumber();
                                                                 auto offset = 0;
                                                                 if (arguments[10].isNumber()) {
                                                                     offset = (int32_t) arguments[10].asNumber();
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
                                                                         this->GetState()
                                                                 );
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "copyBufferSubData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 4) {
                                                             auto readTarget = (uint32_t) arguments[0].asNumber();
                                                             auto writeTarget = (uint32_t) arguments[1].asNumber();
                                                             auto readOffset = arguments[2].asNumber();
                                                             auto writeOffset = arguments[3].asNumber();
                                                             auto size = arguments[4].asNumber();
                                                             canvas_native_webgl2_copy_buffer_sub_data(
                                                                     readTarget,
                                                                     writeTarget,
                                                                     static_cast<ssize_t>(readOffset),
                                                                     static_cast<ssize_t>(writeOffset),
                                                                     static_cast<ssize_t>(size),
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "copyTexSubImage3D") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     9,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 8) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto level = (int32_t) arguments[1].asNumber();
                                                             auto xoffset = (int32_t) arguments[2].asNumber();
                                                             auto yoffset = (int32_t) arguments[3].asNumber();
                                                             auto zoffset = (int32_t) arguments[4].asNumber();
                                                             auto x = (int32_t) arguments[5].asNumber();
                                                             auto y = (int32_t) arguments[6].asNumber();
                                                             auto width = (int32_t) arguments[7].asNumber();
                                                             auto height = (int32_t) arguments[8].asNumber();
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
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createQuery") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto ret = canvas_native_webgl2_create_query(
                                                                 this->GetState());
                                                         auto query = std::make_shared<WebGLQuery>(
                                                                 ret);
                                                         return jsi::Object::createFromHostObject(
                                                                 runtime, query);
                                                     }
        );
    } else if (methodName == "createSampler") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto ret = canvas_native_webgl2_create_sampler(
                                                                 this->GetState());
                                                         auto sampler = std::make_shared<WebGLSampler>(
                                                                 ret);
                                                         return jsi::Object::createFromHostObject(
                                                                 runtime, sampler);
                                                     }
        );
    } else if (methodName == "createTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto ret = canvas_native_webgl2_create_transform_feedback(
                                                                 this->GetState());

                                                         auto feedback = std::make_shared<WebGLTransformFeedback>(
                                                                 ret);
                                                         return jsi::Object::createFromHostObject(
                                                                 runtime, feedback);
                                                     }
        );
    } else if (methodName == "createVertexArray") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto ret = canvas_native_webgl2_create_vertex_array(
                                                                 this->GetState());

                                                         auto vertexArrayObject = std::make_shared<WebGLVertexArrayObject>(
                                                                 ret);
                                                         return jsi::Object::createFromHostObject(
                                                                 runtime, vertexArrayObject);
                                                     }
        );
    } else if (methodName == "clearBufferfi") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 3 && arguments[0].isObject()) {
                                                             auto bufferObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (bufferObject.isHostObject(
                                                                     runtime)) {
                                                                 auto buffer = bufferObject.asHostObject<WebGLBuffer>(
                                                                         runtime);
                                                                 if (buffer != nullptr) {
                                                                     auto drawbuffer = (int32_t) arguments[1].asNumber();
                                                                     auto depth = arguments[2].asNumber();
                                                                     auto stencil = (int32_t) arguments[3].asNumber();
                                                                     canvas_native_webgl2_clear_bufferfi(
                                                                             buffer->GetBuffer(),
                                                                             drawbuffer,
                                                                             static_cast<float>(depth),
                                                                             stencil,
                                                                             this->GetState()
                                                                     );
                                                                 }

                                                             }
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "deleteQuery") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[0].isObject()) {
                                                             auto queryObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (queryObject.isHostObject(
                                                                     runtime)) {
                                                                 auto query = queryObject.asHostObject<WebGLQuery>(
                                                                         runtime);

                                                                 if (query != nullptr) {
                                                                     canvas_native_webgl2_delete_query_with_query(
                                                                             query->GetQuery(),
                                                                             this->GetState()
                                                                     );
                                                                 }

                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteSampler") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[0].isObject()) {
                                                             auto samplerObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (samplerObject.isHostObject(
                                                                     runtime)) {
                                                                 auto sampler = samplerObject.asHostObject<WebGLSampler>(
                                                                         runtime);

                                                                 if (sampler != nullptr) {
                                                                     canvas_native_webgl2_delete_sampler_with_sampler(
                                                                             sampler->GetSampler(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[0].isObject()) {
                                                             auto syncObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (syncObject.isHostObject(runtime)) {
                                                                 auto sync = syncObject.asHostObject<WebGLSyncImpl>(
                                                                         runtime);

                                                                 if (sync != nullptr) {
                                                                     canvas_native_webgl2_delete_sync_with_sync(
                                                                             sync->GetSync(),
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[0].isObject()) {
                                                             auto transformFeedbackObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (transformFeedbackObject.isHostObject(
                                                                     runtime)) {
                                                                 auto transformFeedback = transformFeedbackObject.asHostObject<WebGLTransformFeedback>(
                                                                         runtime);

                                                                 if (transformFeedback != nullptr) {
                                                                     canvas_native_webgl2_delete_transform_feedback(
                                                                             transformFeedback->GetFeedback(),
                                                                             this->GetState()
                                                                     );
                                                                 }

                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "deleteVertexArray") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[0].isObject()) {
                                                             auto vertexArrayObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (vertexArrayObject.isHostObject(
                                                                     runtime)) {
                                                                 auto vertexArray = vertexArrayObject.asHostObject<WebGLVertexArrayObject>(
                                                                         runtime);
                                                                 if (vertexArray != nullptr) {
                                                                     canvas_native_webgl2_delete_vertex_array_with_vertex_array(
                                                                             vertexArray->GetVertexArrayObject(),
                                                                             this->GetState()
                                                                     );
                                                                 }

                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawArraysInstanced") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 3) {
                                                             auto mode = (uint32_t) arguments[0].asNumber();
                                                             auto first = (int32_t) arguments[1].asNumber();
                                                             auto count_ = (int32_t) arguments[2].asNumber();
                                                             auto instanceCount = (int32_t) arguments[3].asNumber();
                                                             canvas_native_webgl2_draw_arrays_instanced(
                                                                     mode,
                                                                     first,
                                                                     count_,
                                                                     instanceCount,
                                                                     this->GetState()
                                                             );

                                                             this->UpdateInvalidateState();
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawBuffers") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (arguments[0].isObject()) {
                                                             auto buffersObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (buffersObject.isArray(runtime)) {
                                                                 auto array = buffersObject.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, j);
                                                                     buf.emplace_back(
                                                                             (uint32_t) item.asNumber());
                                                                 }
                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl2_draw_buffers(
                                                                         slice,
                                                                         this->GetState());
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawElementsInstanced") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 4) {
                                                             auto mode = (uint32_t) arguments[0].asNumber();
                                                             auto count_ = (int32_t) arguments[1].asNumber();
                                                             auto type = (uint32_t) arguments[2].asNumber();
                                                             auto offset = arguments[3].asNumber();
                                                             auto instanceCount = (int32_t) arguments[4].asNumber();
                                                             canvas_native_webgl2_draw_elements_instanced(
                                                                     mode,
                                                                     count_,
                                                                     type,
                                                                     static_cast<ssize_t>(offset),
                                                                     instanceCount,
                                                                     this->GetState()
                                                             );

                                                             this->UpdateInvalidateState();
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawRangeElements") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 5) {
                                                             auto mode = (uint32_t) arguments[0].asNumber();
                                                             auto start = (uint32_t) arguments[1].asNumber();
                                                             auto end = (uint32_t) arguments[2].asNumber();
                                                             auto count_ = (int32_t) arguments[3].asNumber();
                                                             auto type = (uint32_t) arguments[4].asNumber();
                                                             auto offset = arguments[5].asNumber();
                                                             canvas_native_webgl2_draw_range_elements(
                                                                     mode,
                                                                     start,
                                                                     end,
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
    } else if (methodName == "endQuery") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (arguments[0].isNumber()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             canvas_native_webgl2_end_query(target,
                                                                                            this->GetState());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "endTransformFeedback") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     03,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         canvas_native_webgl2_end_transform_feedback(
                                                                 this->GetState());
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "fenceSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto condition = (uint32_t) arguments[0].asNumber();
                                                             auto flags = (uint32_t) arguments[1].asNumber();
                                                             canvas_native_webgl2_fence_sync(
                                                                     condition,
                                                                     flags,
                                                                     this->GetState()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "framebufferTextureLayer") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 4 && arguments[2].isObject()) {
                                                             auto textureObject = arguments[2].asObject(
                                                                     runtime);
                                                             if (textureObject.isHostObject(
                                                                     runtime)) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();
                                                                 auto attachment = (uint32_t) arguments[1].asNumber();
                                                                 auto texture = textureObject.asHostObject<WebGLTexture>(
                                                                         runtime);
                                                                 auto level = (int32_t) arguments[3].asNumber();
                                                                 auto layer = (int32_t) arguments[4].asNumber();
                                                                 if (texture != nullptr) {
                                                                     canvas_native_webgl2_framebuffer_texture_layer(
                                                                             target,
                                                                             attachment,
                                                                             texture->GetTexture(),
                                                                             level,
                                                                             layer,
                                                                             this->GetState()
                                                                     );
                                                                 }
                                                             }

                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getActiveUniformBlockName") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 1) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, arguments[0]);
                                                             if (program != nullptr) {
                                                                 auto uniformBlockIndex = (uint32_t) arguments[1].asNumber();
                                                                 auto name = canvas_native_webgl2_get_active_uniform_block_name(
                                                                         program->GetProgram(),
                                                                         uniformBlockIndex,
                                                                         this->GetState()
                                                                 );
                                                                 return jsi::String::createFromAscii(
                                                                         runtime, name.data(),
                                                                         name.size());
                                                             }

                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getActiveUniformBlockParameter") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 2) {

                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, arguments[0]);

                                                             if (program != nullptr) {
                                                                 auto uniformBlockIndex = (uint32_t) arguments[1].asNumber();
                                                                 auto pname = (uint32_t) arguments[2].asNumber();
                                                                 auto ret = canvas_native_webgl2_get_active_uniform_block_parameter(
                                                                         program->GetProgram(),
                                                                         uniformBlockIndex,
                                                                         pname,
                                                                         this->GetState()
                                                                 );

                                                                 switch (pname) {
                                                                     case GL_UNIFORM_BLOCK_BINDING:
                                                                     case GL_UNIFORM_BLOCK_DATA_SIZE:
                                                                     case GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS:
                                                                         return {canvas_native_webgl_result_get_i32(
                                                                                 *ret)};
                                                                     case GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: {
                                                                         auto value = canvas_native_webgl_result_get_u32_array(
                                                                                 *ret);
                                                                         auto buffer = std::make_shared<VecMutableBuffer<uint32_t>>(
                                                                                 std::move(value));
                                                                         auto array = jsi::ArrayBuffer(
                                                                                 runtime, buffer);

                                                                         auto Uint32Array = runtime.global()
                                                                                 .getProperty(
                                                                                         runtime,
                                                                                         "Uint32Array")
                                                                                 .asObject(runtime)
                                                                                 .asFunction(
                                                                                         runtime);


                                                                         return Uint32Array.callAsConstructor(
                                                                                 runtime, array);
                                                                     }
                                                                     case GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER:
                                                                     case GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:
                                                                         return {canvas_native_webgl_result_get_bool(
                                                                                 *ret)};
                                                                     default:
                                                                         return jsi::Value::null();
                                                                 }
                                                             }
                                                         }

                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getActiveUniforms") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count > 2 && arguments[1].isObject()) {

                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, arguments[0]);
                                                             auto uniformIndicesObject = arguments[1].asObject(
                                                                     runtime);
                                                             auto pname = (uint32_t) arguments[2].asNumber();

                                                             if (uniformIndicesObject.isArray(
                                                                     runtime)) {
                                                                 auto uniformIndices = uniformIndicesObject.asArray(
                                                                         runtime);
                                                                 auto size = uniformIndices.size(
                                                                         runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(size);
                                                                 for (int j = 0; j < size; j++) {
                                                                     auto item = (uint32_t) uniformIndices.getValueAtIndex(
                                                                             runtime, j).asNumber();
                                                                     buf.emplace_back(item);
                                                                 }
                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 auto ret = canvas_native_webgl2_get_active_uniforms(
                                                                         program->GetProgram(),
                                                                         slice,
                                                                         pname,
                                                                         this->GetState()
                                                                 );

                                                                 switch (pname) {
                                                                     case GL_UNIFORM_TYPE:
                                                                     case GL_UNIFORM_SIZE: {
                                                                         auto value = canvas_native_webgl_result_get_u32_array(
                                                                                 *ret);
                                                                         auto array = jsi::Array(
                                                                                 runtime,
                                                                                 value.size());
                                                                         auto len = value.size();
                                                                         for (int j = 0;
                                                                              j < len; ++j) {
                                                                             auto item = value[j];
                                                                             array.setValueAtIndex(
                                                                                     runtime, j,
                                                                                     jsi::Value(
                                                                                             (int32_t) item));
                                                                         }
                                                                         return array;
                                                                     }
                                                                         break;
                                                                     case GL_UNIFORM_BLOCK_INDEX:
                                                                     case GL_UNIFORM_OFFSET:
                                                                     case GL_UNIFORM_ARRAY_STRIDE:
                                                                     case GL_UNIFORM_MATRIX_STRIDE: {
                                                                         auto value = canvas_native_webgl_result_get_i32_array(
                                                                                 *ret);
                                                                         auto array = jsi::Array(
                                                                                 runtime,
                                                                                 value.size());
                                                                         auto len = value.size();
                                                                         for (int j = 0;
                                                                              j < len; ++j) {
                                                                             auto item = value[j];
                                                                             array.setValueAtIndex(
                                                                                     runtime, j,
                                                                                     jsi::Value(
                                                                                             item));
                                                                         }
                                                                         return array;
                                                                     }
                                                                     case GL_UNIFORM_IS_ROW_MAJOR: {
                                                                         auto value = canvas_native_webgl_result_get_bool_array(
                                                                                 *ret);
                                                                         auto len = value.size();
                                                                         auto array = jsi::Array(
                                                                                 runtime,
                                                                                 len);
                                                                         for (int j = 0;
                                                                              j < len; ++j) {
                                                                             bool item =
                                                                                     value[j] == 1;
                                                                             array.setValueAtIndex(
                                                                                     runtime, j,
                                                                                     jsi::Value(
                                                                                             item));
                                                                         }
                                                                         return array;
                                                                     }
                                                                     default:
                                                                         return jsi::Value::null();
                                                                 }
                                                             }


                                                         }
                                                         return jsi::Value::null();
                                                     }
        );
    } else if (methodName == "getBufferSubData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count >= 3 &&
                                                             arguments[2].isObject()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto srcByteOffset = arguments[1].asNumber();
                                                             auto dstDataObject = arguments[2].asObject(
                                                                     runtime);

                                                             if (dstDataObject.isTypedArray(
                                                                     runtime)) {
                                                                 ssize_t dstOffsetValue = 0;
                                                                 if (arguments[3].isNumber()) {
                                                                     dstOffsetValue = static_cast<ssize_t>(arguments[3].asNumber());
                                                                 }

                                                                 ssize_t lengthValue = 0;
                                                                 if (arguments[4].isNumber()) {
                                                                     lengthValue = static_cast<ssize_t>(arguments[4].asNumber());
                                                                 }

                                                                 auto array = dstDataObject.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<uint8_t>(
                                                                         runtime, array);

                                                                 canvas_native_webgl2_get_buffer_sub_data(
                                                                         target,
                                                                         static_cast<ssize_t>(srcByteOffset),
                                                                         slice,
                                                                         dstOffsetValue,
                                                                         lengthValue,
                                                                         this->GetState()
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


                                                         if (count > 1 && arguments[1].isString()) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, arguments[0]);

                                                             if (program != nullptr) {
                                                                 auto name = arguments[1].asString(
                                                                         runtime).utf8(runtime);

                                                                 auto ret = canvas_native_webgl2_get_frag_data_location(
                                                                         program->GetProgram(),
                                                                         rust::Str(name.c_str()),
                                                                         this->GetState()
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
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto index = (uint32_t) arguments[1].asNumber();
                                                             auto ret = canvas_native_webgl2_get_indexed_parameter(
                                                                     target,
                                                                     index,
                                                                     this->GetState()
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
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto internalformat = (uint32_t) arguments[1].asNumber();
                                                             auto pname = (uint32_t) arguments[2].asNumber();
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
                                                                     this->GetState()
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
                                                             auto pname = (uint32_t) arguments[0].asNumber();
                                                             auto result = canvas_native_webgl2_get_parameter(
                                                                     pname,
                                                                     this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             if (query != nullptr) {
                                                                 auto pname = (uint32_t) arguments[1].asNumber();


                                                                 auto ret = canvas_native_webgl2_get_query_parameter(
                                                                         query->GetQuery(),
                                                                         pname,
                                                                         this->GetState());
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
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto pname = (uint32_t) arguments[1].asNumber();
                                                             auto ret = canvas_native_webgl2_get_query(
                                                                     target,
                                                                     pname,
                                                                     this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             if (sampler != nullptr) {
                                                                 auto pname = (uint32_t) arguments[1].asNumber();
                                                                 auto ret = canvas_native_webgl2_get_sampler_parameter(
                                                                         sampler->GetSampler(),
                                                                         pname,
                                                                         this->GetState());

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
                                                                     runtime, arguments[0]);
                                                             if (sync != nullptr) {
                                                                 auto pname = (uint32_t) arguments[1].asNumber();
                                                                 auto ret = canvas_native_webgl2_get_sync_parameter(
                                                                         sync->GetSync(),
                                                                         pname,
                                                                         this->GetState());

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
                                                                     runtime, arguments[0]);
                                                             auto index = (uint32_t) arguments[1].asNumber();
                                                             if (program != nullptr) {
                                                                 auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                                                                         program->GetProgram(),
                                                                         index,
                                                                         this->GetState()
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
                                                                     runtime, arguments[0]);
                                                             auto index = (uint32_t) arguments[1].asNumber();
                                                             if (program != nullptr) {
                                                                 auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                                                                         program->GetProgram(),
                                                                         index,
                                                                         this->GetState()
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
    } else if (methodName == "getUniformIndices") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, arguments[0]);
                                                             auto uniformNamesObject = arguments[1].asObject(
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
                                                                         this->GetState());

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


                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto attachments = arguments[1].asObject(
                                                                     runtime);

                                                             if (attachments.isArray(runtime)) {
                                                                 auto array = attachments.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, j).asNumber();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl2_invalidate_framebuffer(
                                                                         target, slice,
                                                                         this->GetState());
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


                                                         if (count > 5 && arguments[1].isObject()) {
                                                             auto attachments = arguments[1].asObject(
                                                                     runtime);
                                                             if (attachments.isArray(runtime)) {
                                                                 auto target = (uint32_t) arguments[0].asNumber();
                                                                 auto x = (int32_t) arguments[2].asNumber();
                                                                 auto y = (int32_t) arguments[3].asNumber();
                                                                 auto width = (int32_t) arguments[4].asNumber();
                                                                 auto height = (int32_t) arguments[5].asNumber();

                                                                 auto array = attachments.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto item = (uint) array.getValueAtIndex(
                                                                             runtime, j).asNumber();
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
                                                                         this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             if (query != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_query(
                                                                         query->GetQuery(),
                                                                         this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             if (sampler != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_sampler(
                                                                         sampler->GetSampler(),
                                                                         this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             if (sync != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_sync(
                                                                         sync->GetSync(),
                                                                         this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             if (transformFeedback != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_transform_feedback(
                                                                         transformFeedback->GetFeedback(),
                                                                         this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             if (vertexArray != nullptr) {
                                                                 auto ret = canvas_native_webgl2_is_vertex_array(
                                                                         vertexArray->GetVertexArrayObject(),
                                                                         this->GetState());
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
                                                                 this->GetState());

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
                                                             auto src = (uint32_t) arguments[0].asNumber();
                                                             canvas_native_webgl2_read_buffer(
                                                                     src,
                                                                     this->GetState()
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
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto samples = (int32_t) arguments[1].asNumber();
                                                             auto internalFormat = (uint32_t) arguments[2].asNumber();
                                                             auto width = (int32_t) arguments[3].asNumber();
                                                             auto height = (int32_t) arguments[4].asNumber();
                                                             canvas_native_webgl2_renderbuffer_storage_multisample(
                                                                     target,
                                                                     samples,
                                                                     internalFormat,
                                                                     width,
                                                                     height,
                                                                     this->GetState()
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
                                                                 this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             auto pname = (uint32_t) arguments[1].asNumber();
                                                             auto param = arguments[2].asNumber();
                                                             if (sampler != nullptr) {
                                                                 canvas_native_webgl2_sampler_parameterf(
                                                                         sampler->GetSampler(),
                                                                         pname,
                                                                         static_cast<float>(param),
                                                                         this->GetState());
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
                                                                     runtime, arguments[0]);
                                                             auto pname = (uint32_t) arguments[1].asNumber();
                                                             auto param = (int32_t) arguments[2].asNumber();
                                                             if (sampler != nullptr) {
                                                                 canvas_native_webgl2_sampler_parameteri(
                                                                         sampler->GetSampler(),
                                                                         pname,
                                                                         param,
                                                                         this->GetState());
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
                                                         // target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, offset: any
// target, level, internalformat, width, height, depth, border, format, type, srcData, srcOffset
// target, level, internalformat, width, height, depth, border, format, type, source



                                                         if (count == 10) {
                                                             auto target = (int32_t) arguments[0].asNumber();
                                                             auto level = (int32_t) arguments[1].asNumber();
                                                             auto internalformat = (int32_t) arguments[2].asNumber();
                                                             auto width = (int32_t) arguments[3].asNumber();
                                                             auto height = (int32_t) arguments[4].asNumber();
                                                             auto depth = (int32_t) arguments[5].asNumber();
                                                             auto border = (int32_t) arguments[6].asNumber();
                                                             auto format = (int32_t) arguments[7].asNumber();
                                                             auto type = (uint32_t) arguments[8].asNumber();


                                                             if (arguments[9].isNumber()) {
                                                                 auto imageOrPixelsOrOffset = arguments[9].asNumber();
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
                                                                         this->GetState()
                                                                 );
                                                                 return jsi::Value::undefined();
                                                             }

                                                             if (arguments[9].isObject()) {
                                                                 auto imageOrPixelsOrOffsetObject = arguments[9].asObject(
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
                                                                             this->GetState()
                                                                     );
                                                                     return jsi::Value::undefined();
                                                                 }


                                                                 auto image_asset = getHostObject<ImageAssetImpl>(
                                                                         runtime, arguments[9]);
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
                                                                             this->GetState()
                                                                     );
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             }
                                                         } else if (count > 10) {

                                                             auto target = (int32_t) arguments[0].asNumber();
                                                             auto level = (int32_t) arguments[1].asNumber();
                                                             auto internalformat = (int32_t) arguments[2].asNumber();
                                                             auto width = (int32_t) arguments[3].asNumber();
                                                             auto height = (int32_t) arguments[4].asNumber();
                                                             auto depth = (int32_t) arguments[5].asNumber();
                                                             auto border = (int32_t) arguments[6].asNumber();
                                                             auto format = (int32_t) arguments[7].asNumber();
                                                             auto type = (uint32_t) arguments[8].asNumber();
                                                             if (arguments[9].isObject()) {
                                                                 auto imageOrPixelsOrOffset = arguments[9].asObject(
                                                                         runtime);
                                                                 size_t srcOffsetValue = 0;
                                                                 if (arguments[9].isNumber()) {
                                                                     srcOffsetValue = static_cast<size_t>(arguments[9].asNumber());
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
                                                                             this->GetState()
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
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto levels = (int32_t) arguments[1].asNumber();
                                                             auto internalFormat = (uint32_t) arguments[2].asNumber();
                                                             auto width = (int32_t) arguments[3].asNumber();
                                                             auto height = (int32_t) arguments[4].asNumber();
                                                             canvas_native_webgl2_tex_storage2d(
                                                                     target,
                                                                     levels,
                                                                     internalFormat,
                                                                     width,
                                                                     height,
                                                                     this->GetState()
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
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto levels = (int32_t) arguments[1].asNumber();
                                                             auto internalFormat = (uint32_t) arguments[2].asNumber();
                                                             auto width = (int32_t) arguments[3].asNumber();
                                                             auto height = (int32_t) arguments[4].asNumber();
                                                             auto depth = (int32_t) arguments[5].asNumber();
                                                             canvas_native_webgl2_tex_storage3d(
                                                                     target,
                                                                     levels,
                                                                     internalFormat,
                                                                     width,
                                                                     height,
                                                                     depth,
                                                                     this->GetState()
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
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto level = (int32_t) arguments[1].asNumber();
                                                             auto xoffset = (int32_t) arguments[2].asNumber();
                                                             auto yoffset = (int32_t) arguments[3].asNumber();
                                                             auto zoffset = (int32_t) arguments[4].asNumber();
                                                             auto width = (int32_t) arguments[5].asNumber();
                                                             auto height = (int32_t) arguments[6].asNumber();
                                                             auto depth = (int32_t) arguments[7].asNumber();
                                                             auto format = (uint32_t) arguments[8].asNumber();
                                                             auto type = (int32_t) arguments[9].asNumber();

                                                             if (arguments[10].isNumber()) {
                                                                 auto imageOrPixelsOrOffset = arguments[10].asNumber();
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
                                                                         this->GetState()
                                                                 );
                                                                 return jsi::Value::undefined();
                                                             }

                                                             if (arguments[10].isObject()) {
                                                                 auto imageOrPixelsOrOffsetObject = arguments[10].asObject(
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
                                                                             this->GetState()
                                                                     );

                                                                     return jsi::Value::undefined();
                                                                 }

                                                                 auto asset = getHostObject<ImageAssetImpl>(
                                                                         runtime, arguments[10]);
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
                                                                             this->GetState()
                                                                     );
                                                                 }

                                                             }

                                                         } else if (count > 11) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto level = (int32_t) arguments[1].asNumber();
                                                             auto xoffset = (int32_t) arguments[2].asNumber();
                                                             auto yoffset = (int32_t) arguments[3].asNumber();
                                                             auto zoffset = (int32_t) arguments[4].asNumber();
                                                             auto width = (int32_t) arguments[5].asNumber();
                                                             auto height = (int32_t) arguments[6].asNumber();
                                                             auto depth = (int32_t) arguments[7].asNumber();
                                                             auto format = (uint32_t) arguments[8].asNumber();
                                                             auto type = (uint32_t) arguments[9].asNumber();

                                                             size_t srcOffsetValue = 0;
                                                             if (arguments[11].isNumber()) {
                                                                 srcOffsetValue = static_cast<size_t>(arguments[11].asNumber());
                                                             }

                                                             if (arguments[10].isObject()) {
                                                                 auto imageOrPixelsOrOffsetObject = arguments[10].asObject(
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
                                                                             this->GetState()
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
                                                         if (count > 2 && arguments[1].isObject()) {
                                                             auto program = getHostObject<WebGLProgram>(
                                                                     runtime, arguments[0]);
                                                             auto varyingsObject = arguments[1].asObject(
                                                                     runtime);
                                                             auto bufferMode = (uint32_t) arguments[2].asNumber();

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
                                                                         this->GetState()
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

                                                         if (count > 1 && arguments[1].isNumber()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto v0 = (uint32_t) arguments[1].asNumber();

                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform1ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         this->GetState()
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

                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto data = arguments[1].asObject(
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
                                                                         this->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, i).asNumber();
                                                                     buf.push_back(item);
                                                                 }
                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl2_uniform1uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         this->GetState()
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
                                                                     runtime, arguments[0]);
                                                             auto v0 = (uint32_t) arguments[1].asNumber();
                                                             auto v1 = (uint32_t) arguments[2].asNumber();

                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform2ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         v1,
                                                                         this->GetState()
                                                                 );
                                                             }


                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "Uniform2uiv") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1 && arguments[1].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto data = arguments[1].asObject(
                                                                     runtime);

                                                             if (data.isUint32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform2uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         this->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, i).asNumber();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl2_uniform2uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         this->GetState()
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
                                                                     runtime, arguments[0]);
                                                             auto v0 = (uint32_t) arguments[1].asNumber();
                                                             auto v1 = (uint32_t) arguments[2].asNumber();
                                                             auto v2 = (uint32_t) arguments[3].asNumber();
                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform3ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         v1,
                                                                         v2,
                                                                         this->GetState()
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

                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto data = arguments[1].asObject(
                                                                     runtime);

                                                             if (data.isUint32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform3uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         this->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i).asNumber();
                                                                     buf.push_back((uint32_t) item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform3uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         this->GetState()
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
                                                                     runtime, arguments[0]);
                                                             auto v0 = (uint32_t) arguments[1].asNumber();
                                                             auto v1 = (uint32_t) arguments[2].asNumber();
                                                             auto v2 = (uint32_t) arguments[3].asNumber();
                                                             auto v3 = (uint32_t) arguments[4].asNumber();
                                                             if (location != nullptr) {
                                                                 canvas_native_webgl2_uniform4ui(
                                                                         location->GetUniformLocation(),
                                                                         v0,
                                                                         v1,
                                                                         v2,
                                                                         v3,
                                                                         this->GetState()
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


                                                         if (count > 1 && arguments[1].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto data = arguments[1].asObject(
                                                                     runtime);

                                                             if (data.isUint32Array(runtime)) {
                                                                 auto array = data.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_uniform4uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         this->GetState()
                                                                 );
                                                             } else {
                                                                 rust::Vec<uint32_t> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i).asNumber();
                                                                     buf.push_back((uint32_t) item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_uniform4uiv(
                                                                         location->GetUniformLocation(),
                                                                         slice,
                                                                         this->GetState()
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
                                                                     runtime, arguments[0]);
                                                             auto uniformBlockIndex = arguments[1].asNumber();
                                                             auto uniformBlockBinding = arguments[2].asNumber();

                                                             if (program != nullptr) {
                                                                 canvas_native_webgl2_uniform_block_binding(
                                                                         program->GetProgram(),
                                                                         (uint32_t) uniformBlockIndex,
                                                                         (uint32_t) uniformBlockBinding,
                                                                         this->GetState()
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

                                                         if (count > 2 && arguments[2].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto transpose = arguments[1].asBool();
                                                             auto data = arguments[2].asObject(
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
                                                                         this->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item.isNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item.asNumber()));
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
                                                                         this->GetState()
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

                                                         if (count > 2 && arguments[2].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto transpose = arguments[1].asBool();
                                                             auto data = arguments[2].asObject(
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
                                                                         this->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item.isNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item.asNumber()));
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
                                                                         this->GetState()
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

                                                         if (count > 2 && arguments[2].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto transpose = arguments[1].asBool();
                                                             auto data = arguments[2].asObject(
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
                                                                         this->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item.isNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item.asNumber()));
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
                                                                         this->GetState()
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
                                                         if (count > 2 && arguments[2].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto transpose = arguments[1].asBool();
                                                             auto data = arguments[2].asObject(
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
                                                                         this->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item.isNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item.asNumber()));
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
                                                                         this->GetState()
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
                                                         if (count > 2 && arguments[2].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto transpose = arguments[1].asBool();
                                                             auto data = arguments[2].asObject(
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
                                                                         this->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item.isNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item.asNumber()));
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
                                                                         this->GetState()
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
                                                         if (count > 2 && arguments[2].isObject()) {

                                                             auto location = getHostObject<WebGLUniformLocation>(
                                                                     runtime, arguments[0]);
                                                             auto transpose = arguments[1].asBool();
                                                             auto data = arguments[2].asObject(
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
                                                                         this->GetState()
                                                                 );
                                                             } else if (data.isArray(runtime)) {
                                                                 rust::Vec<float> buf;
                                                                 auto array = data.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = array.getValueAtIndex(
                                                                             runtime, i);
                                                                     if (item.isNumber()) {
                                                                         buf.push_back(
                                                                                 static_cast<float>(item.asNumber()));
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
                                                                         this->GetState()
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
                                                             auto index = arguments[0].asNumber();
                                                             auto divisor = arguments[1].asNumber();
                                                             canvas_native_webgl2_vertex_attrib_divisor(
                                                                     (uint32_t) index,
                                                                     (uint32_t) divisor,
                                                                     this->GetState()
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
                                                             auto index = arguments[0].asNumber();
                                                             auto v0 = arguments[1].asNumber();
                                                             auto v1 = arguments[2].asNumber();
                                                             auto v2 = arguments[3].asNumber();
                                                             auto v3 = arguments[4].asNumber();
                                                             canvas_native_webgl2_vertex_attrib_i4i(
                                                                     (uint32_t) index,
                                                                     (int32_t) v0,
                                                                     (int32_t) v1,
                                                                     (int32_t) v2,
                                                                     (int32_t) v3,
                                                                     this->GetState()
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

                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto index = (uint32_t) arguments[0].asNumber();
                                                             auto value = arguments[1].asObject(
                                                                     runtime);
                                                             if (value.isInt32Array(runtime)) {
                                                                 auto array = value.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const int32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_vertex_attrib_i4iv(
                                                                         index,
                                                                         slice,
                                                                         this->GetState()
                                                                 );
                                                             } else if (value.isArray(runtime)) {
                                                                 auto array = value.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<int32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (int32_t) array.getValueAtIndex(
                                                                             runtime, i).asNumber();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const int32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_vertex_attrib_i4iv(
                                                                         index,
                                                                         slice,
                                                                         this->GetState()
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
                                                             auto index = (uint32_t) arguments[0].asNumber();
                                                             auto v0 = (uint32_t) arguments[1].asNumber();
                                                             auto v1 = (uint32_t) arguments[2].asNumber();
                                                             auto v2 = (uint32_t) arguments[3].asNumber();
                                                             auto v3 = (uint32_t) arguments[4].asNumber();

                                                             canvas_native_webgl2_vertex_attrib_i4ui(
                                                                     index,
                                                                     v0,
                                                                     v1,
                                                                     v2,
                                                                     v3,
                                                                     this->GetState()
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


                                                         if (count > 1 && arguments[1].isObject()) {
                                                             auto index = (uint32_t) arguments[0].asNumber();
                                                             auto value = arguments[1].asObject(
                                                                     runtime);
                                                             if (value.isUint32Array(runtime)) {
                                                                 auto array = value.getTypedArray(
                                                                         runtime);
                                                                 auto slice = GetTypedArrayData<const uint32_t>(
                                                                         runtime, array);
                                                                 canvas_native_webgl2_vertex_attrib_i4uiv(
                                                                         index,
                                                                         slice,
                                                                         this->GetState()
                                                                 );
                                                             } else if (value.isArray(runtime)) {
                                                                 auto array = value.getArray(
                                                                         runtime);
                                                                 auto len = array.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int i = 0; i < len; i++) {
                                                                     auto item = (uint32_t) array.getValueAtIndex(
                                                                             runtime, i).asNumber();
                                                                     buf.push_back(item);
                                                                 }

                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());

                                                                 canvas_native_webgl2_vertex_attrib_i4uiv(
                                                                         index,
                                                                         slice,
                                                                         this->GetState()
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
