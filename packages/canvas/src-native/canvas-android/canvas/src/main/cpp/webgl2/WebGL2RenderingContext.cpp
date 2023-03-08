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
    std::vector<jsi::PropNameID> names = WebGLRenderingContext::getPropertyNames(rt);

    std::vector<jsi::PropNameID> props;
    // 351


    props.push_back(jsi::PropNameID::forAscii(rt, "beginQuery"));
    props.push_back(jsi::PropNameID::forAscii(rt, "beginTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "bindBufferBase"));
    props.push_back(jsi::PropNameID::forAscii(rt, "bindBufferRange"));
    props.push_back(jsi::PropNameID::forAscii(rt, "bindSampler"));
    props.push_back(jsi::PropNameID::forAscii(rt, "bindTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "bindVertexArray"));
    props.push_back(jsi::PropNameID::forAscii(rt, "blitFramebuffer"));
    props.push_back(jsi::PropNameID::forAscii(rt, "clearBufferfi"));
    props.push_back(jsi::PropNameID::forAscii(rt, "clearBufferfv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "clearBufferiv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "clearBufferuiv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "clientWaitSync"));
    props.push_back(jsi::PropNameID::forAscii(rt, "compressedTexSubImage3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "copyBufferSubData"));
    props.push_back(jsi::PropNameID::forAscii(rt, "copyTexSubImage3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "createQuery"));
    props.push_back(jsi::PropNameID::forAscii(rt, "createSampler"));
    props.push_back(jsi::PropNameID::forAscii(rt, "createTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "createVertexArray"));
    props.push_back(jsi::PropNameID::forAscii(rt, "deleteQuery"));
    props.push_back(jsi::PropNameID::forAscii(rt, "deleteSampler"));
    props.push_back(jsi::PropNameID::forAscii(rt, "deleteSync"));
    props.push_back(jsi::PropNameID::forAscii(rt, "deleteTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "deleteVertexArray"));
    props.push_back(jsi::PropNameID::forAscii(rt, "drawArraysInstanced"));
    props.push_back(jsi::PropNameID::forAscii(rt, "drawBuffers"));
    props.push_back(jsi::PropNameID::forAscii(rt, "drawElementsInstanced"));
    props.push_back(jsi::PropNameID::forAscii(rt, "drawRangeElements"));
    props.push_back(jsi::PropNameID::forAscii(rt, "endQuery"));
    props.push_back(jsi::PropNameID::forAscii(rt, "endTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "fenceSync"));
    props.push_back(jsi::PropNameID::forAscii(rt, "framebufferTextureLayer"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform1ui"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform1uiv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform2ui"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform2uiv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform3ui"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform3uiv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform4ui"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniform4uiv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniformBlockBinding"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix2x3fv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix2x4fv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix3x2fv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix3x4fv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix4x2fv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "uniformMatrix4x3fv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribDivisor"));
    props.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4i"));
    props.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4iv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4ui"));
    props.push_back(jsi::PropNameID::forAscii(rt, "vertexAttribI4uiv"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getActiveUniformBlockName"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getActiveUniformBlockParameter"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getActiveUniforms"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getBufferSubData"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getFragDataLocation"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getIndexedParameter"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getInternalformatParameter"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getParameter"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getQueryParameter"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getQuery"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getSamplerParameter"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getSyncParameter"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getTransformFeedbackVarying"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getUniformBlockIndex"));
    props.push_back(jsi::PropNameID::forAscii(rt, "getUniformIndices"));
    props.push_back(jsi::PropNameID::forAscii(rt, "invalidateFramebuffer"));
    props.push_back(jsi::PropNameID::forAscii(rt, "invalidateSubFramebuffer"));
    props.push_back(jsi::PropNameID::forAscii(rt, "isQuery"));
    props.push_back(jsi::PropNameID::forAscii(rt, "isSampler"));
    props.push_back(jsi::PropNameID::forAscii(rt, "isSync"));
    props.push_back(jsi::PropNameID::forAscii(rt, "isTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "isVertexArray"));
    props.push_back(jsi::PropNameID::forAscii(rt, "pauseTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "readBuffer"));
    props.push_back(jsi::PropNameID::forAscii(rt, "renderbufferStorageMultisample"));
    props.push_back(jsi::PropNameID::forAscii(rt, "resumeTransformFeedback"));
    props.push_back(jsi::PropNameID::forAscii(rt, "samplerParameterf"));
    props.push_back(jsi::PropNameID::forAscii(rt, "samplerParameteri"));
    props.push_back(jsi::PropNameID::forAscii(rt, "texImage3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "texStorage2D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "texStorage3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "texSubImage3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "transformFeedbackVaryings"));


    props.push_back(jsi::PropNameID::forAscii(rt, "READ_BUFFER"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_ROW_LENGTH"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_SKIP_ROWS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_SKIP_PIXELS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "PACK_ROW_LENGTH"));

    props.push_back(jsi::PropNameID::forAscii(rt, "PACK_SKIP_ROWS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "PACK_SKIP_PIXELS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_BINDING_3D"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_SKIP_IMAGES"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNPACK_IMAGE_HEIGHT"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_3D_TEXTURE_SIZE"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_ELEMENTS_VERTICES"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_ELEMENTS_INDICES"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_TEXTURE_LOD_BIAS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_FRAGMENT_UNIFORM_COMPONENTS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_VERTEX_UNIFORM_COMPONENTS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_ARRAY_TEXTURE_LAYERS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MIN_PROGRAM_TEXEL_OFFSET"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_PROGRAM_TEXEL_OFFSET"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_VARYING_COMPONENTS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "FRAGMENT_SHADER_DERIVATIVE_HINT"));

    props.push_back(jsi::PropNameID::forAscii(rt, "RASTERIZER_DISCARD"));

    props.push_back(jsi::PropNameID::forAscii(rt, "VERTEX_ARRAY_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_VERTEX_OUTPUT_COMPONENTS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_FRAGMENT_INPUT_COMPONENTS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_SERVER_WAIT_TIMEOUT"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_ELEMENT_INDEX"));

    props.push_back(jsi::PropNameID::forAscii(rt, "RED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB10_A2"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_3D"));

    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_WRAP_R"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_MIN_LOD"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_MAX_LOD"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_BASE_LEVEL"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_MAX_LEVEL"));


    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_COMPARE_MODE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_COMPARE_FUNC"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SRGB"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SRGB8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SRGB8_ALPHA8"));

    props.push_back(jsi::PropNameID::forAscii(rt, "COMPARE_REF_TO_TEXTURE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA32F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB32F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA16F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB16F"));

    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_2D_ARRAY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_BINDING_2D_ARRAY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R11F_G11F_B10F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB9_E5"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA32UI"));


    props.push_back(jsi::PropNameID::forAscii(rt, "RGB32UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA16UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB16UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA8UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB8UI"));


    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA32I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB32I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA16I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB16I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA8I"));


    props.push_back(jsi::PropNameID::forAscii(rt, "RGB8I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RED_INTEGER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB_INTEGER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA_INTEGER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R8"));


    props.push_back(jsi::PropNameID::forAscii(rt, "RG8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R16F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R32F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG16F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG32F"));


    props.push_back(jsi::PropNameID::forAscii(rt, "R8I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R8UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R16I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R16UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R32I"));


    props.push_back(jsi::PropNameID::forAscii(rt, "R32UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG8I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG8UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG16I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG16UI"));

    props.push_back(jsi::PropNameID::forAscii(rt, "RG32I"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG32UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "R8_SNORM"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG8_SNORM"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB8_SNORM"));


    props.push_back(jsi::PropNameID::forAscii(rt, "RGBA8_SNORM"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RGB10_A2UI"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_IMMUTABLE_FORMAT"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TEXTURE_IMMUTABLE_LEVELS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_2_10_10_10_REV"));


    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_10F_11F_11F_REV"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_5_9_9_9_REV"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_32_UNSIGNED_INT_24_8_REV"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_24_8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "HALF_FLOAT"));


    props.push_back(jsi::PropNameID::forAscii(rt, "RG"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RG_INTEGER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "INT_2_10_10_10_REV"));
    props.push_back(jsi::PropNameID::forAscii(rt, "QUERY_RESULT_AVAILABLE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "QUERY_RESULT"));


    props.push_back(jsi::PropNameID::forAscii(rt, "CURRENT_QUERY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "ANY_SAMPLES_PASSED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "ANY_SAMPLES_PASSED_CONSERVATIVE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_DRAW_BUFFERS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER0"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER1"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER2"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER3)"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER4"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER5"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER6"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER7"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER9"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER10"));

    /* Getting GL parameter information */

    /* Textures */

    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER11"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER12"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER13"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER14"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_BUFFER15"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_COLOR_ATTACHMENTS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT1"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT2"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT3"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT4"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT5"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT6"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT7"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT9"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT10"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT11"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT12"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT13"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT14"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR_ATTACHMENT15"));

    props.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_2D_SHADOW"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_2D_ARRAY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_2D_ARRAY_SHADOW"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_CUBE_SHADOW"));

    props.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_2D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_CUBE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "INT_SAMPLER_2D_ARRAY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_2D"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_3D"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_CUBE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_SAMPLER_2D_ARRAY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_SAMPLES"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SAMPLER_BINDING"));

    props.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_PACK_BUFFER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_UNPACK_BUFFER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_PACK_BUFFER_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "PIXEL_UNPACK_BUFFER_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COPY_READ_BUFFER"));

    props.push_back(jsi::PropNameID::forAscii(rt, "COPY_WRITE_BUFFER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COPY_READ_BUFFER_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "COPY_WRITE_BUFFER_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT2x3"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT2x4"));

    props.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT3x2"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT3x4"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT4x2"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FLOAT_MAT4x3"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_VEC2"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_VEC3"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_INT_VEC4"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNED_NORMALIZED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SIGNED_NORMALIZED"));

    /* Vertex attributes */

    props.push_back(jsi::PropNameID::forAscii(rt, "VERTEX_ATTRIB_ARRAY_INTEGER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "VERTEX_ATTRIB_ARRAY_DIVISOR"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_MODE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_VARYINGS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_START"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS"));

    /* Textures */

    /* Pixel types */

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "INTERLEAVED_ATTRIBS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SEPARATE_ATTRIBS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BUFFER_BINDING"));

    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_PAUSED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_ACTIVE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TRANSFORM_FEEDBACK_BINDING"));

    /* Pixel types */

    /* Queries */

    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_RED_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_GREEN_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_BLUE_SIZE"));

    /* Queries */

    /* Draw buffers */

    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_DEFAULT"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_STENCIL_ATTACHMENT"));

    props.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_STENCIL"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DEPTH24_STENCIL8"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_FRAMEBUFFER_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "READ_FRAMEBUFFER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DRAW_FRAMEBUFFER"));

    props.push_back(jsi::PropNameID::forAscii(rt, "READ_FRAMEBUFFER_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "RENDERBUFFER_SAMPLES"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_START"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_VERTEX_UNIFORM_BLOCKS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_FRAGMENT_UNIFORM_BLOCKS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_COMBINED_UNIFORM_BLOCKS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_UNIFORM_BUFFER_BINDINGS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_UNIFORM_BLOCK_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BUFFER_OFFSET_ALIGNMENT"));
    props.push_back(jsi::PropNameID::forAscii(rt, "ACTIVE_UNIFORM_BLOCKS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_TYPE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_INDEX"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_OFFSET"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_ARRAY_STRIDE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_MATRIX_STRIDE"));

    /* Draw buffers */

    /* Samplers */

    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_IS_ROW_MAJOR"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_BINDING"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_DATA_SIZE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_ACTIVE_UNIFORMS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES"));

    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER"));
    props.push_back(jsi::PropNameID::forAscii(rt, "OBJECT_TYPE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SYNC_CONDITION"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SYNC_STATUS"));

    props.push_back(jsi::PropNameID::forAscii(rt, "SYNC_FLAGS"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SYNC_FENCE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SYNC_GPU_COMMANDS_COMPLETE"));
    props.push_back(jsi::PropNameID::forAscii(rt, "UNSIGNALED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SIGNALED"));

    /* Samplers */

    /* Buffers */

    props.push_back(jsi::PropNameID::forAscii(rt, "ALREADY_SIGNALED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TIMEOUT_EXPIRED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "CONDITION_SATISFIED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "WAIT_FAILED"));
    props.push_back(jsi::PropNameID::forAscii(rt, "SYNC_FLUSH_COMMANDS_BIT"));

    props.push_back(jsi::PropNameID::forAscii(rt, "COLOR"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DEPTH"));
    props.push_back(jsi::PropNameID::forAscii(rt, "STENCIL"));

    /* Buffers */

    /* Data types */

    props.push_back(jsi::PropNameID::forAscii(rt, "MIN"));
    props.push_back(jsi::PropNameID::forAscii(rt, "MAX"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_COMPONENT24"));
    props.push_back(jsi::PropNameID::forAscii(rt, "STREAM_READ"));
    props.push_back(jsi::PropNameID::forAscii(rt, "STREAM_COPY"));

    props.push_back(jsi::PropNameID::forAscii(rt, "STATIC_READ"));
    props.push_back(jsi::PropNameID::forAscii(rt, "STATIC_COPY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DYNAMIC_READ"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DYNAMIC_COPY"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DEPTH_COMPONENT32F"));
    props.push_back(jsi::PropNameID::forAscii(rt, "DEPTH32F_STENCIL8"));

    /* Data types */

    props.push_back(jsi::PropNameID::forAscii(rt, "INVALID_INDEX"));
    props.push_back(jsi::PropNameID::forAscii(rt, "TIMEOUT_IGNORED"));

    /* Vertex attributes */

    /* Transform feedback */

    props.push_back(jsi::PropNameID::forAscii(rt, "MAX_CLIENT_WAIT_TIMEOUT_WEBGL"));

    /* Transform feedback */







    names.reserve(names.size() + props.size());

    names.insert(
            names.end(),
            std::make_move_iterator(props.begin()),
            std::make_move_iterator(props.end())
    );

    return names;
}

jsi::Value WebGL2RenderingContext::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto gl_return = WebGLRenderingContext::get(runtime, name);
    if (!gl_return.isNull()) {
        return gl_return;
    }

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

    return prop;
}
