//
// Created by Osei Fortune on 22/03/2022.
//

#include "WebGL2RenderingContext.h"

WebGL2RenderingContext::WebGL2RenderingContext(rust::Box <WebGLState> state): WebGLRenderingContext(
        std::move(state), WebGLRenderingVersion::V2) {
}

std::vector<jsi::PropNameID> WebGL2RenderingContext::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> names = WebGLRenderingContext::getPropertyNames(rt);

    std::vector<jsi::PropNameID> props {
            jsi::PropNameID::forAscii(rt,"beginQuery"),
            jsi::PropNameID::forAscii(rt,"beginTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"bindBufferBase"),
            jsi::PropNameID::forAscii(rt,"bindBufferRange"),
            jsi::PropNameID::forAscii(rt,"bindSampler"),
            jsi::PropNameID::forAscii(rt,"bindTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"bindVertexArray"),
            jsi::PropNameID::forAscii(rt,"blitFramebuffer"),
            jsi::PropNameID::forAscii(rt,"clearBufferfi"),
            jsi::PropNameID::forAscii(rt,"clearBufferfv"),
            jsi::PropNameID::forAscii(rt,"clearBufferiv"),
            jsi::PropNameID::forAscii(rt,"clearBufferuiv"),
            jsi::PropNameID::forAscii(rt,"clientWaitSync"),
            jsi::PropNameID::forAscii(rt,"compressedTexSubImage3D"),
            jsi::PropNameID::forAscii(rt,"copyBufferSubData"),
            jsi::PropNameID::forAscii(rt,"copyTexSubImage3D"),
            jsi::PropNameID::forAscii(rt,"createQuery"),
            jsi::PropNameID::forAscii(rt,"createSampler"),
            jsi::PropNameID::forAscii(rt,"createTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"createVertexArray"),
            jsi::PropNameID::forAscii(rt,"deleteQuery"),
            jsi::PropNameID::forAscii(rt,"deleteSampler"),
            jsi::PropNameID::forAscii(rt,"deleteSync"),
            jsi::PropNameID::forAscii(rt,"deleteTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"deleteVertexArray"),
            jsi::PropNameID::forAscii(rt,"drawArraysInstanced"),
            jsi::PropNameID::forAscii(rt,"drawBuffers"),
            jsi::PropNameID::forAscii(rt,"drawElementsInstanced"),
            jsi::PropNameID::forAscii(rt,"drawRangeElements"),
            jsi::PropNameID::forAscii(rt,"endQuery"),
            jsi::PropNameID::forAscii(rt,"endTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"fenceSync"),
            jsi::PropNameID::forAscii(rt,"framebufferTextureLayer"),
            jsi::PropNameID::forAscii(rt,"uniform1ui"),
            jsi::PropNameID::forAscii(rt,"uniform1uiv"),
            jsi::PropNameID::forAscii(rt,"uniform2ui"),
            jsi::PropNameID::forAscii(rt,"uniform2uiv"),
            jsi::PropNameID::forAscii(rt,"uniform3ui"),
            jsi::PropNameID::forAscii(rt,"uniform3uiv"),
            jsi::PropNameID::forAscii(rt,"uniform4ui"),
            jsi::PropNameID::forAscii(rt,"uniform4uiv"),
            jsi::PropNameID::forAscii(rt,"uniformBlockBinding"),
            jsi::PropNameID::forAscii(rt,"uniformMatrix2x3fv"),
            jsi::PropNameID::forAscii(rt,"uniformMatrix2x4fv"),
            jsi::PropNameID::forAscii(rt,"uniformMatrix3x2fv"),
            jsi::PropNameID::forAscii(rt,"uniformMatrix3x4fv"),
            jsi::PropNameID::forAscii(rt,"uniformMatrix4x2fv"),
            jsi::PropNameID::forAscii(rt,"uniformMatrix4x3fv"),
            jsi::PropNameID::forAscii(rt,"vertexAttribDivisor"),
            jsi::PropNameID::forAscii(rt,"vertexAttribI4i"),
            jsi::PropNameID::forAscii(rt,"vertexAttribI4iv"),
            jsi::PropNameID::forAscii(rt,"vertexAttribI4ui"),
            jsi::PropNameID::forAscii(rt,"vertexAttribI4uiv"),
            jsi::PropNameID::forAscii(rt,"getActiveUniformBlockName"),
            jsi::PropNameID::forAscii(rt,"getActiveUniformBlockParameter"),
            jsi::PropNameID::forAscii(rt,"getActiveUniforms"),
            jsi::PropNameID::forAscii(rt,"getBufferSubData"),
            jsi::PropNameID::forAscii(rt,"getFragDataLocation"),
            jsi::PropNameID::forAscii(rt,"getIndexedParameter"),
            jsi::PropNameID::forAscii(rt,"getInternalformatParameter"),
            jsi::PropNameID::forAscii(rt,"getParameter"),
            jsi::PropNameID::forAscii(rt,"getQueryParameter"),
            jsi::PropNameID::forAscii(rt,"getQuery"),
            jsi::PropNameID::forAscii(rt,"getSamplerParameter"),
            jsi::PropNameID::forAscii(rt,"getSyncParameter"),
            jsi::PropNameID::forAscii(rt,"getTransformFeedbackVarying"),
            jsi::PropNameID::forAscii(rt,"getUniformBlockIndex"),
            jsi::PropNameID::forAscii(rt,"getUniformIndices"),
            jsi::PropNameID::forAscii(rt,"invalidateFramebuffer"),
            jsi::PropNameID::forAscii(rt,"invalidateSubFramebuffer"),
            jsi::PropNameID::forAscii(rt,"isQuery"),
            jsi::PropNameID::forAscii(rt,"isSampler"),
            jsi::PropNameID::forAscii(rt,"isSync"),
            jsi::PropNameID::forAscii(rt,"isTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"isVertexArray"),
            jsi::PropNameID::forAscii(rt,"pauseTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"readBuffer"),
            jsi::PropNameID::forAscii(rt,"renderbufferStorageMultisample"),
            jsi::PropNameID::forAscii(rt,"resumeTransformFeedback"),
            jsi::PropNameID::forAscii(rt,"samplerParameterf"),
            jsi::PropNameID::forAscii(rt,"samplerParameteri"),
            jsi::PropNameID::forAscii(rt,"texImage3D"),
            jsi::PropNameID::forAscii(rt,"texStorage2D"),
            jsi::PropNameID::forAscii(rt,"texStorage3D"),
            jsi::PropNameID::forAscii(rt,"texSubImage3D"),
            jsi::PropNameID::forAscii(rt,"transformFeedbackVaryings"),


            jsi::PropNameID::forAscii(rt,"READ_BUFFER"),

            jsi::PropNameID::forAscii(rt,"UNPACK_ROW_LENGTH"),

            jsi::PropNameID::forAscii(rt,"UNPACK_SKIP_ROWS"),

            jsi::PropNameID::forAscii(rt,"UNPACK_SKIP_PIXELS"),

            jsi::PropNameID::forAscii(rt,"PACK_ROW_LENGTH"),

            jsi::PropNameID::forAscii(rt,"PACK_SKIP_ROWS"),

            jsi::PropNameID::forAscii(rt,"PACK_SKIP_PIXELS"),

            jsi::PropNameID::forAscii(rt,"TEXTURE_BINDING_3D"),

            jsi::PropNameID::forAscii(rt,"UNPACK_SKIP_IMAGES"),

            jsi::PropNameID::forAscii(rt,"UNPACK_IMAGE_HEIGHT"),

            jsi::PropNameID::forAscii(rt,"MAX_3D_TEXTURE_SIZE"),

            jsi::PropNameID::forAscii(rt,"MAX_ELEMENTS_VERTICES"),

            jsi::PropNameID::forAscii(rt,"MAX_ELEMENTS_INDICES"),

            jsi::PropNameID::forAscii(rt,"MAX_TEXTURE_LOD_BIAS"),

            jsi::PropNameID::forAscii(rt,"MAX_FRAGMENT_UNIFORM_COMPONENTS"),

            jsi::PropNameID::forAscii(rt,"MAX_VERTEX_UNIFORM_COMPONENTS"),

            jsi::PropNameID::forAscii(rt,"MAX_ARRAY_TEXTURE_LAYERS"),

            jsi::PropNameID::forAscii(rt,"MIN_PROGRAM_TEXEL_OFFSET"),

            jsi::PropNameID::forAscii(rt,"MAX_PROGRAM_TEXEL_OFFSET"),

            jsi::PropNameID::forAscii(rt,"MAX_VARYING_COMPONENTS"),

            jsi::PropNameID::forAscii(rt,"FRAGMENT_SHADER_DERIVATIVE_HINT"),

            jsi::PropNameID::forAscii(rt,"RASTERIZER_DISCARD"),

            jsi::PropNameID::forAscii(rt,"VERTEX_ARRAY_BINDING"),
            jsi::PropNameID::forAscii(rt,"MAX_VERTEX_OUTPUT_COMPONENTS"),
            jsi::PropNameID::forAscii(rt,"MAX_FRAGMENT_INPUT_COMPONENTS"),
            jsi::PropNameID::forAscii(rt,"MAX_SERVER_WAIT_TIMEOUT"),
            jsi::PropNameID::forAscii(rt,"MAX_ELEMENT_INDEX"),

            jsi::PropNameID::forAscii(rt,"RED"),
            jsi::PropNameID::forAscii(rt,"RGB8"),
            jsi::PropNameID::forAscii(rt,"RGBA8"),
            jsi::PropNameID::forAscii(rt,"RGB10_A2"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_3D"),

            jsi::PropNameID::forAscii(rt,"TEXTURE_WRAP_R"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_MIN_LOD"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_MAX_LOD"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_BASE_LEVEL"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_MAX_LEVEL"),


            jsi::PropNameID::forAscii(rt,"TEXTURE_COMPARE_MODE"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_COMPARE_FUNC"),
            jsi::PropNameID::forAscii(rt,"SRGB"),
            jsi::PropNameID::forAscii(rt,"SRGB8"),
            jsi::PropNameID::forAscii(rt,"SRGB8_ALPHA8"),

            jsi::PropNameID::forAscii(rt,"COMPARE_REF_TO_TEXTURE"),
            jsi::PropNameID::forAscii(rt,"RGBA32F"),
            jsi::PropNameID::forAscii(rt,"RGB32F"),
            jsi::PropNameID::forAscii(rt,"RGBA16F"),
            jsi::PropNameID::forAscii(rt,"RGB16F"),

            jsi::PropNameID::forAscii(rt,"TEXTURE_2D_ARRAY"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_BINDING_2D_ARRAY"),
            jsi::PropNameID::forAscii(rt,"R11F_G11F_B10F"),
            jsi::PropNameID::forAscii(rt,"RGB9_E5"),
            jsi::PropNameID::forAscii(rt,"RGBA32UI"),


            jsi::PropNameID::forAscii(rt,"RGB32UI"),
            jsi::PropNameID::forAscii(rt,"RGBA16UI"),
            jsi::PropNameID::forAscii(rt,"RGB16UI"),
            jsi::PropNameID::forAscii(rt,"RGBA8UI"),
            jsi::PropNameID::forAscii(rt,"RGB8UI"),


            jsi::PropNameID::forAscii(rt,"RGBA32I"),
            jsi::PropNameID::forAscii(rt,"RGB32I"),
            jsi::PropNameID::forAscii(rt,"RGBA16I"),
            jsi::PropNameID::forAscii(rt,"RGB16I"),
            jsi::PropNameID::forAscii(rt,"RGBA8I"),


            jsi::PropNameID::forAscii(rt,"RGB8I"),
            jsi::PropNameID::forAscii(rt,"RED_INTEGER"),
            jsi::PropNameID::forAscii(rt,"RGB_INTEGER"),
            jsi::PropNameID::forAscii(rt,"RGBA_INTEGER"),
            jsi::PropNameID::forAscii(rt,"R8"),


            jsi::PropNameID::forAscii(rt,"RG8"),
            jsi::PropNameID::forAscii(rt,"R16F"),
            jsi::PropNameID::forAscii(rt,"R32F"),
            jsi::PropNameID::forAscii(rt,"RG16F"),
            jsi::PropNameID::forAscii(rt,"RG32F"),


            jsi::PropNameID::forAscii(rt,"R8I"),
            jsi::PropNameID::forAscii(rt,"R8UI"),
            jsi::PropNameID::forAscii(rt,"R16I"),
            jsi::PropNameID::forAscii(rt,"R16UI"),
            jsi::PropNameID::forAscii(rt,"R32I"),


            jsi::PropNameID::forAscii(rt,"R32UI"),
            jsi::PropNameID::forAscii(rt,"RG8I"),
            jsi::PropNameID::forAscii(rt,"RG8UI"),
            jsi::PropNameID::forAscii(rt,"RG16I"),
            jsi::PropNameID::forAscii(rt,"RG16UI"),

            jsi::PropNameID::forAscii(rt,"RG32I"),
            jsi::PropNameID::forAscii(rt,"RG32UI"),
            jsi::PropNameID::forAscii(rt,"R8_SNORM"),
            jsi::PropNameID::forAscii(rt,"RG8_SNORM"),
            jsi::PropNameID::forAscii(rt,"RGB8_SNORM"),


            jsi::PropNameID::forAscii(rt,"RGBA8_SNORM"),
            jsi::PropNameID::forAscii(rt,"RGB10_A2UI"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_IMMUTABLE_FORMAT"),
            jsi::PropNameID::forAscii(rt,"TEXTURE_IMMUTABLE_LEVELS"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_2_10_10_10_REV"),


            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_10F_11F_11F_REV"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_5_9_9_9_REV"),
            jsi::PropNameID::forAscii(rt,"FLOAT_32_UNSIGNED_INT_24_8_REV"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_24_8"),
            jsi::PropNameID::forAscii(rt,"HALF_FLOAT"),


            jsi::PropNameID::forAscii(rt,"RG"),
            jsi::PropNameID::forAscii(rt,"RG_INTEGER"),
            jsi::PropNameID::forAscii(rt,"INT_2_10_10_10_REV"),
            jsi::PropNameID::forAscii(rt,"QUERY_RESULT_AVAILABLE"),
            jsi::PropNameID::forAscii(rt,"QUERY_RESULT"),


            jsi::PropNameID::forAscii(rt,"CURRENT_QUERY"),
            jsi::PropNameID::forAscii(rt,"ANY_SAMPLES_PASSED"),
            jsi::PropNameID::forAscii(rt,"ANY_SAMPLES_PASSED_CONSERVATIVE"),
            jsi::PropNameID::forAscii(rt,"MAX_DRAW_BUFFERS"),

            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER0"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER1"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER2"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER3)"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER4"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER5"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER6"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER7"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER8"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER9"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER10"),

            /* Getting GL parameter information */

            /* Textures */

            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER11"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER12"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER13"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER14"),
            jsi::PropNameID::forAscii(rt,"DRAW_BUFFER15"),

            jsi::PropNameID::forAscii(rt,"MAX_COLOR_ATTACHMENTS"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT1"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT2"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT3"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT4"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT5"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT6"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT7"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT8"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT9"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT10"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT11"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT12"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT13"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT14"),
            jsi::PropNameID::forAscii(rt,"COLOR_ATTACHMENT15"),

            jsi::PropNameID::forAscii(rt,"SAMPLER_3D"),
            jsi::PropNameID::forAscii(rt,"SAMPLER_2D_SHADOW"),
            jsi::PropNameID::forAscii(rt,"SAMPLER_2D_ARRAY"),
            jsi::PropNameID::forAscii(rt,"SAMPLER_2D_ARRAY_SHADOW"),
            jsi::PropNameID::forAscii(rt,"SAMPLER_CUBE_SHADOW"),

            jsi::PropNameID::forAscii(rt,"INT_SAMPLER_2D"),
            jsi::PropNameID::forAscii(rt,"INT_SAMPLER_3D"),
            jsi::PropNameID::forAscii(rt,"INT_SAMPLER_CUBE"),
            jsi::PropNameID::forAscii(rt,"INT_SAMPLER_2D_ARRAY"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_SAMPLER_2D"),

            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_SAMPLER_3D"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_SAMPLER_CUBE"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_SAMPLER_2D_ARRAY"),
            jsi::PropNameID::forAscii(rt,"MAX_SAMPLES"),
            jsi::PropNameID::forAscii(rt,"SAMPLER_BINDING"),

            jsi::PropNameID::forAscii(rt,"PIXEL_PACK_BUFFER"),
            jsi::PropNameID::forAscii(rt,"PIXEL_UNPACK_BUFFER"),
            jsi::PropNameID::forAscii(rt,"PIXEL_PACK_BUFFER_BINDING"),
            jsi::PropNameID::forAscii(rt,"PIXEL_UNPACK_BUFFER_BINDING"),
            jsi::PropNameID::forAscii(rt,"COPY_READ_BUFFER"),

            jsi::PropNameID::forAscii(rt,"COPY_WRITE_BUFFER"),
            jsi::PropNameID::forAscii(rt,"COPY_READ_BUFFER_BINDING"),
            jsi::PropNameID::forAscii(rt,"COPY_WRITE_BUFFER_BINDING"),
            jsi::PropNameID::forAscii(rt,"FLOAT_MAT2x3"),
            jsi::PropNameID::forAscii(rt,"FLOAT_MAT2x4"),

            jsi::PropNameID::forAscii(rt,"FLOAT_MAT3x2"),
            jsi::PropNameID::forAscii(rt,"FLOAT_MAT3x4"),
            jsi::PropNameID::forAscii(rt,"FLOAT_MAT4x2"),
            jsi::PropNameID::forAscii(rt,"FLOAT_MAT4x3"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_VEC2"),

            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_VEC3"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_INT_VEC4"),
            jsi::PropNameID::forAscii(rt,"UNSIGNED_NORMALIZED"),
            jsi::PropNameID::forAscii(rt,"SIGNED_NORMALIZED"),

            /* Vertex attributes */

            jsi::PropNameID::forAscii(rt,"VERTEX_ATTRIB_ARRAY_INTEGER"),
            jsi::PropNameID::forAscii(rt,"VERTEX_ATTRIB_ARRAY_DIVISOR"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_BUFFER_MODE"),
            jsi::PropNameID::forAscii(rt,"MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_VARYINGS"),

            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_BUFFER_START"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_BUFFER_SIZE"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN"),
            jsi::PropNameID::forAscii(rt,"MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS"),

            /* Textures */

            /* Pixel types */

            jsi::PropNameID::forAscii(rt,"MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS"),
            jsi::PropNameID::forAscii(rt,"INTERLEAVED_ATTRIBS"),
            jsi::PropNameID::forAscii(rt,"SEPARATE_ATTRIBS"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_BUFFER"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_BUFFER_BINDING"),

            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_PAUSED"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_ACTIVE"),
            jsi::PropNameID::forAscii(rt,"TRANSFORM_FEEDBACK_BINDING"),

            /* Pixel types */

            /* Queries */

            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_RED_SIZE"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_GREEN_SIZE"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_BLUE_SIZE"),

            /* Queries */

            /* Draw buffers */

            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_DEFAULT"),
            jsi::PropNameID::forAscii(rt,"DEPTH_STENCIL_ATTACHMENT"),

            jsi::PropNameID::forAscii(rt,"DEPTH_STENCIL"),
            jsi::PropNameID::forAscii(rt,"DEPTH24_STENCIL8"),
            jsi::PropNameID::forAscii(rt,"DRAW_FRAMEBUFFER_BINDING"),
            jsi::PropNameID::forAscii(rt,"READ_FRAMEBUFFER"),
            jsi::PropNameID::forAscii(rt,"DRAW_FRAMEBUFFER"),

            jsi::PropNameID::forAscii(rt,"READ_FRAMEBUFFER_BINDING"),
            jsi::PropNameID::forAscii(rt,"RENDERBUFFER_SAMPLES"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER"),
            jsi::PropNameID::forAscii(rt,"FRAMEBUFFER_INCOMPLETE_MULTISAMPLE"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BUFFER"),

            jsi::PropNameID::forAscii(rt,"UNIFORM_BUFFER_BINDING"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BUFFER_START"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BUFFER_SIZE"),
            jsi::PropNameID::forAscii(rt,"MAX_VERTEX_UNIFORM_BLOCKS"),
            jsi::PropNameID::forAscii(rt,"MAX_FRAGMENT_UNIFORM_BLOCKS"),

            jsi::PropNameID::forAscii(rt,"MAX_COMBINED_UNIFORM_BLOCKS"),
            jsi::PropNameID::forAscii(rt,"MAX_UNIFORM_BUFFER_BINDINGS"),
            jsi::PropNameID::forAscii(rt,"MAX_UNIFORM_BLOCK_SIZE"),
            jsi::PropNameID::forAscii(rt,"MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS"),
            jsi::PropNameID::forAscii(rt,"MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS"),

            jsi::PropNameID::forAscii(rt,"UNIFORM_BUFFER_OFFSET_ALIGNMENT"),
            jsi::PropNameID::forAscii(rt,"ACTIVE_UNIFORM_BLOCKS"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_TYPE"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_SIZE"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BLOCK_INDEX"),

            jsi::PropNameID::forAscii(rt,"UNIFORM_OFFSET"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_ARRAY_STRIDE"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_MATRIX_STRIDE"),

            /* Draw buffers */

            /* Samplers */

            jsi::PropNameID::forAscii(rt,"UNIFORM_IS_ROW_MAJOR"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BLOCK_BINDING"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BLOCK_DATA_SIZE"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BLOCK_ACTIVE_UNIFORMS"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES"),

            jsi::PropNameID::forAscii(rt,"UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER"),
            jsi::PropNameID::forAscii(rt,"UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER"),
            jsi::PropNameID::forAscii(rt,"OBJECT_TYPE"),
            jsi::PropNameID::forAscii(rt,"SYNC_CONDITION"),
            jsi::PropNameID::forAscii(rt,"SYNC_STATUS"),

            jsi::PropNameID::forAscii(rt,"SYNC_FLAGS"),
            jsi::PropNameID::forAscii(rt,"SYNC_FENCE"),
            jsi::PropNameID::forAscii(rt,"SYNC_GPU_COMMANDS_COMPLETE"),
            jsi::PropNameID::forAscii(rt,"UNSIGNALED"),
            jsi::PropNameID::forAscii(rt,"SIGNALED"),

            /* Samplers */

            /* Buffers */

            jsi::PropNameID::forAscii(rt,"ALREADY_SIGNALED"),
            jsi::PropNameID::forAscii(rt,"TIMEOUT_EXPIRED"),
            jsi::PropNameID::forAscii(rt,"CONDITION_SATISFIED"),
            jsi::PropNameID::forAscii(rt,"WAIT_FAILED"),
            jsi::PropNameID::forAscii(rt,"SYNC_FLUSH_COMMANDS_BIT"),

            jsi::PropNameID::forAscii(rt,"COLOR"),
            jsi::PropNameID::forAscii(rt,"DEPTH"),
            jsi::PropNameID::forAscii(rt,"STENCIL"),

            /* Buffers */

            /* Data types */

            jsi::PropNameID::forAscii(rt,"MIN"),
            jsi::PropNameID::forAscii(rt,"MAX"),
            jsi::PropNameID::forAscii(rt,"DEPTH_COMPONENT24"),
            jsi::PropNameID::forAscii(rt,"STREAM_READ"),
            jsi::PropNameID::forAscii(rt,"STREAM_COPY"),

            jsi::PropNameID::forAscii(rt,"STATIC_READ"),
            jsi::PropNameID::forAscii(rt,"STATIC_COPY"),
            jsi::PropNameID::forAscii(rt,"DYNAMIC_READ"),
            jsi::PropNameID::forAscii(rt,"DYNAMIC_COPY"),
            jsi::PropNameID::forAscii(rt,"DEPTH_COMPONENT32F"),
            jsi::PropNameID::forAscii(rt,"DEPTH32F_STENCIL8"),

            /* Data types */

            jsi::PropNameID::forAscii(rt,"INVALID_INDEX"),
            jsi::PropNameID::forAscii(rt,"TIMEOUT_IGNORED"),

            /* Vertex attributes */

            /* Transform feedback */

            jsi::PropNameID::forAscii(rt, "MAX_CLIENT_WAIT_TIMEOUT_WEBGL")

            /* Transform feedback */
    };

    names.reserve(names.size() + props.size());
    names.insert(names.end(), props.begin(), props.end());
    return names;
}

jsi::Value WebGL2RenderingContext::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto gl_return = WebGLRenderingContext::get(runtime, name);
    if(!gl_return.isNull()){
        return gl_return;
    }

    auto methodName = name.utf8(runtime);

    if(methodName == "beginQuery"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(count >1 && arguments[0].isNumber() && arguments[1].isObject()){
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto queryObject = arguments[1].asObject(runtime);

                                                        if(queryObject.isHostObject(runtime)){
                                                            auto query = queryObject.asHostObject<WebGLQuery>(runtime);
                                                            canvas_native_webgl2_begin_query(
                                                                    target,
                                                                    query->GetQuery(),
                                                                    this->GetState()
                                                            );
                                                        }
                                                    }
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "beginTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(count > 0 && arguments[0].isNumber()){
                                                        canvas_native_webgl2_begin_transform_feedback(
                                                                (uint32_t) arguments[0].asNumber(),
                                                                this->GetState()
                                                        );
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "bindBufferBase"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(count > 2 && arguments[2].isObject()){
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto index = (uint32_t)arguments[1].asNumber();
                                                        auto bufferObject = arguments[2].asObject(runtime);
                                                        if(bufferObject.isHostObject(runtime)){
                                                            auto buffer = bufferObject.asHostObject<WebGLBuffer>(runtime);

                                                            canvas_native_webgl2_bind_buffer_base(
                                                                    target,
                                                                    index,
                                                                    buffer->GetBuffer(),
                                                                    this->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "bindBufferRange"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 5,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 4 && arguments[2].isObject()) {
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto index = (uint32_t)arguments[1].asNumber();
                                                        auto offset = arguments[3].asNumber();
                                                        auto size = arguments[4].asNumber();
                                                        auto bufferObject = arguments[2].asObject(runtime);
                                                        if(bufferObject.isHostObject(runtime)){
                                                            auto buffer = bufferObject.asHostObject<WebGLBuffer>(runtime);
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

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "bindSampler"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(count > 1 && arguments[1].isObject()){
                                                        auto samplerObject = arguments[1].asObject(runtime);
                                                        if(samplerObject.isHostObject(runtime)){
                                                            auto unit = (uint32_t )arguments[0].asNumber();
                                                            auto sampler = samplerObject.asHostObject<WebGLSampler>(runtime);

                                                            canvas_native_webgl2_bind_sampler(
                                                                    unit,
                                                                    sampler->GetSampler(),
                                                                    this->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "bindTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(count > 1 && arguments[1].isObject()){
                                                        auto transformFeedbackObject = arguments[1].asObject(runtime);
                                                        if(transformFeedbackObject.isHostObject(runtime)){
                                                            auto target = (uint32_t)arguments[0].asNumber();
                                                            auto transformFeedback = transformFeedbackObject.asHostObject<WebGLTransformFeedback>(runtime);

                                                            canvas_native_webgl2_bind_transform_feedback(
                                                                    target,
                                                                    transformFeedback->GetFeedback(),
                                                                    this->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "bindVertexArray"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                   if(count > 0) {
                                                       if (arguments[0].isNull()) {
                                                           canvas_native_webgl2_bind_vertex_array(
                                                                   0,
                                                                   this->GetState()
                                                           );
                                                           return Value::undefined();
                                                       }

                                                       if (arguments[0].isObject()) {
                                                           auto vertexArrayObject = arguments[0].asObject(
                                                                   runtime);
                                                           if (vertexArrayObject.isHostObject(
                                                                   runtime)) {
                                                               auto vertexArray = vertexArrayObject.asHostObject<WebGLVertexArrayObject>(
                                                                       runtime);

                                                               if(vertexArray != nullptr){

                                                                   canvas_native_webgl2_bind_vertex_array(
                                                                           vertexArray->GetVertexArrayObject(),
                                                                           this->GetState()
                                                                   );
                                                               }
                                                           }
                                                       }

                                                   }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "blitFramebuffer"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 10,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 9) {
                                                        auto srcX0 = (int32_t)arguments[0].asNumber();
                                                        auto srcY0 = (int32_t)arguments[1].asNumber();

                                                        auto srcX1 = (int32_t)arguments[2].asNumber();
                                                        auto srcY1 = (int32_t)arguments[3].asNumber();

                                                        auto dstX0 = (int32_t)arguments[4].asNumber();
                                                        auto dstY0 = (int32_t)arguments[5].asNumber();

                                                        auto dstX1 = (int32_t)arguments[6].asNumber();
                                                        auto dstY1 = (int32_t)arguments[7].asNumber();

                                                        auto mask = (uint32_t)arguments[8].asNumber();
                                                        auto filter = (uint32_t)arguments[9].asNumber();
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

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "clearBufferfv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 2 && arguments[0].isObject() && arguments[1].isObject()) {
                                                        auto bufferObject = arguments[0].asObject(runtime);
                                                        auto drawbuffer = (int32_t)arguments[1].asNumber();
                                                        auto values = arguments[2].asObject(runtime);
                                                        if(bufferObject.isHostObject(runtime)){
                                                            auto buffer = bufferObject.asHostObject<WebGLBuffer>(runtime);
                                                            if (values.isArray(runtime)) {
                                                                auto array = values.getArray(runtime);
                                                                auto len = array.size(runtime);
                                                                std::vector<float> buf;
                                                                buf.reserve(len);
                                                                for (int j = 0; j < len; ++j) {
                                                                    auto item = array.getValueAtIndex(runtime, j);
                                                                    if (!item.isNumber()) {
                                                                        buf.push_back(std::nanf(""));
                                                                    } else {
                                                                        buf.push_back(
                                                                                static_cast<float>(item.asNumber())
                                                                        );
                                                                    }
                                                                }

                                                                rust::Slice<const float> slice(buf.data(), buf.size());
                                                                canvas_native_webgl2_clear_bufferfv(
                                                                        buffer->GetBuffer(),
                                                                        drawbuffer,
                                                                        slice,
                                                                        this->GetState()
                                                                );

                                                            } else if (values.isFloat32Array(runtime)) {
                                                                auto buff = values.getTypedArray(runtime);
                                                                auto slice = GetTypedArrayData<const float>(runtime,buff);
                                                                canvas_native_webgl2_clear_bufferfv(
                                                                        buffer->GetBuffer(),
                                                                        drawbuffer,
                                                                        slice,
                                                                        this->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "clearBufferiv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 2 && arguments[0].isObject() && arguments[1].isObject()) {
                                                        auto bufferObject = arguments[0].asObject(runtime);
                                                        auto drawbuffer = (int32_t)arguments[1].asNumber();
                                                        auto values = arguments[2].asObject(runtime);
                                                        if(bufferObject.isHostObject(runtime)){
                                                            auto buffer = bufferObject.asHostObject<WebGLBuffer>(runtime);
                                                            if (values.isArray(runtime)) {
                                                                auto array = values.getArray(runtime);
                                                                auto len = array.size(runtime);
                                                                std::vector<int32_t> buf;
                                                                buf.reserve(len);
                                                                for (int j = 0; j < len; ++j) {
                                                                    auto item = array.getValueAtIndex(runtime, j);
                                                                    buf.push_back(
                                                                            static_cast<int32_t>(item.asNumber())
                                                                    );
                                                                }

                                                                rust::Slice<const int32_t> slice(buf.data(), buf.size());
                                                                canvas_native_webgl2_clear_bufferiv(
                                                                        buffer->GetBuffer(),
                                                                        drawbuffer,
                                                                        slice,
                                                                        this->GetState()
                                                                );

                                                            } else if (values.isInt32Array(runtime)) {
                                                                auto buff = values.getTypedArray(runtime);
                                                                auto slice = GetTypedArrayData<const int32_t>(runtime,buff);
                                                                canvas_native_webgl2_clear_bufferiv(
                                                                        buffer->GetBuffer(),
                                                                        drawbuffer,
                                                                        slice,
                                                                        this->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "clearBufferuiv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count > 2 && arguments[0].isObject() && arguments[1].isObject()) {
                                                        auto bufferObject = arguments[0].asObject(runtime);
                                                        auto drawbuffer = (int32_t)arguments[1].asNumber();
                                                        auto values = arguments[2].asObject(runtime);
                                                        if(bufferObject.isHostObject(runtime)){
                                                            auto buffer = bufferObject.asHostObject<WebGLBuffer>(runtime);
                                                            if (values.isArray(runtime)) {
                                                                auto array = values.getArray(runtime);
                                                                auto len = array.size(runtime);
                                                                std::vector<uint32_t> buf;
                                                                buf.reserve(len);
                                                                for (int j = 0; j < len; ++j) {
                                                                    auto item = array.getValueAtIndex(runtime, j);
                                                                    buf.push_back(
                                                                            static_cast<uint32_t>(item.asNumber())
                                                                    );
                                                                }

                                                                rust::Slice<const uint32_t> slice(buf.data(), buf.size());
                                                                canvas_native_webgl2_clear_bufferuiv(
                                                                        buffer->GetBuffer(),
                                                                        drawbuffer,
                                                                        slice,
                                                                        this->GetState()
                                                                );

                                                            } else if (values.isUint32Array(runtime)) {
                                                                auto buff = values.getTypedArray(runtime);
                                                                auto slice = GetTypedArrayData<const uint32_t>(runtime,buff);
                                                                canvas_native_webgl2_clear_bufferuiv(
                                                                        buffer->GetBuffer(),
                                                                        drawbuffer,
                                                                        slice,
                                                                        this->GetState()
                                                                );
                                                            }
                                                        }
                                                    }
                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "clientWaitSync"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 2 && arguments[0].isObject()) {
                                                        auto syncObject = arguments[0].asObject(runtime);

                                                        if(syncObject.isHostObject(runtime)){
                                                            auto sync = syncObject.asHostObject<WebGLSyncImpl>(runtime);
                                                            if(sync != nullptr){
                                                                auto flags = (uint32_t)arguments[1].asNumber();
                                                                auto timeout = arguments[2].asNumber();
                                                                auto ret = canvas_native_webgl2_client_wait_sync(
                                                                        sync->GetSync(),
                                                                        flags,
                                                                        static_cast<ssize_t>(timeout),
                                                                        this->GetState()
                                                                );
                                                                return {ret};
                                                            }
                                                        }
                                                    }
                                                    // todo decide if WAIT_FAILED should be returned here
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "compressedTexSubImage3D"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 12,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if(count > 8){
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto level = (int32_t)arguments[1].asNumber();
                                                        auto xoffset = (int32_t)arguments[2].asNumber();
                                                        auto yoffset = (int32_t)arguments[3].asNumber();
                                                        auto zoffset = (int32_t)arguments[4].asNumber();
                                                        auto width = (int32_t)arguments[5].asNumber();
                                                        auto height = (int32_t)arguments[6].asNumber();
                                                        auto depth = (int32_t)arguments[7].asNumber();
                                                        auto format = (uint32_t)arguments[8].asNumber();

                                                        if (arguments[9].isObject()) {
                                                            auto imageSizeOrBuf = arguments[9].asObject(runtime);
                                                            if(imageSizeOrBuf.isTypedArray(runtime)){
                                                                auto array = imageSizeOrBuf.getTypedArray(runtime);
                                                                auto slice =  GetTypedArrayData<const uint8_t>(runtime, array);

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
                                                            auto imageSizeOrBuf = (int32_t)arguments[0].asNumber();
                                                            auto offset = 0;
                                                            if(arguments[10].isNumber()){
                                                                offset = (int32_t)arguments[10].asNumber();
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
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "copyBufferSubData"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 5,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count > 4) {
                                                        auto readTarget = (uint32_t)arguments[0].asNumber();
                                                        auto writeTarget = (uint32_t)arguments[1].asNumber();
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "copyTexSubImage3D"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 9,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 8) {
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto level = (int32_t)arguments[1].asNumber();
                                                        auto xoffset = (int32_t)arguments[2].asNumber();
                                                        auto yoffset = (int32_t)arguments[3].asNumber();
                                                        auto zoffset = (int32_t)arguments[4].asNumber();
                                                        auto x = (int32_t)arguments[5].asNumber();
                                                        auto y = (int32_t)arguments[6].asNumber();
                                                        auto width = (int32_t)arguments[7].asNumber();
                                                        auto height = (int32_t)arguments[8].asNumber();
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "createQuery"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    auto ret = canvas_native_webgl2_create_query(this->GetState());
                                                    auto query = std::make_shared<WebGLQuery>(ret);
                                                    return jsi::Object::createFromHostObject(runtime, query);
                                                }
        );
    }else if(methodName == "createSampler"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    auto ret = canvas_native_webgl2_create_sampler(this->GetState());
                                                    auto sampler = std::make_shared<WebGLSampler>(ret);
                                                    return jsi::Object::createFromHostObject(runtime, sampler);
                                                }
        );
    }else if(methodName == "createTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    auto ret = canvas_native_webgl2_create_transform_feedback(this->GetState());

                                                    auto feedback = std::make_shared<WebGLTransformFeedback>(ret);
                                                    return jsi::Object::createFromHostObject(runtime, feedback);
                                                }
        );
    }else if(methodName == "createVertexArray"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    auto ret = canvas_native_webgl2_create_vertex_array(this->GetState());

                                                    auto vertexArrayObject = std::make_shared<WebGLVertexArrayObject>(ret);
                                                    return jsi::Object::createFromHostObject(runtime, vertexArrayObject);
                                                }
        );
    }else if(methodName == "clearBufferfi"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 3 && arguments[0].isObject()) {
                                                      auto bufferObject = arguments[0].asObject(runtime);
                                                      if(bufferObject.isHostObject(runtime)){
                                                          auto buffer = bufferObject.asHostObject<WebGLBuffer>(runtime);
                                                          if(buffer != nullptr){
                                                              auto drawbuffer = (int32_t)arguments[1].asNumber();
                                                              auto depth = arguments[2].asNumber();
                                                              auto stencil = (int32_t)arguments[3].asNumber();
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

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "deleteQuery"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(arguments[0].isObject()){
                                                        auto queryObject = arguments[0].asObject(runtime);
                                                        if(queryObject.isHostObject(runtime)){
                                                            auto query = queryObject.asHostObject<WebGLQuery>(runtime);

                                                            if(query != nullptr){
                                                                canvas_native_webgl2_delete_query_with_query(
                                                                        query->GetQuery(),
                                                                        this->GetState()
                                                                );
                                                            }

                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "deleteSampler"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(arguments[0].isObject()){
                                                        auto samplerObject = arguments[0].asObject(runtime);
                                                        if(samplerObject.isHostObject(runtime)){
                                                            auto sampler = samplerObject.asHostObject<WebGLSampler>(runtime);

                                                            if(sampler != nullptr){
                                                                canvas_native_webgl2_delete_sampler_with_sampler(
                                                                        sampler->GetSampler(),
                                                                        this->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "deleteSync"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(arguments[0].isObject()){
                                                        auto syncObject = arguments[0].asObject(runtime);
                                                        if(syncObject.isHostObject(runtime)){
                                                            auto sync = syncObject.asHostObject<WebGLSyncImpl>(runtime);

                                                            if(sync != nullptr){
                                                                canvas_native_webgl2_delete_sync_with_sync(
                                                                        sync->GetSync(),
                                                                        this->GetState()
                                                                );
                                                            }
                                                        }
                                                    }
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "deleteTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(arguments[0].isObject()){
                                                        auto transformFeedbackObject = arguments[0].asObject(runtime);
                                                        if(transformFeedbackObject.isHostObject(runtime)){
                                                            auto transformFeedback = transformFeedbackObject.asHostObject<WebGLTransformFeedback>(runtime);

                                                            if(transformFeedback != nullptr){
                                                                canvas_native_webgl2_delete_transform_feedback(
                                                                        transformFeedback->GetFeedback(),
                                                                        this->GetState()
                                                                );
                                                            }

                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "deleteVertexArray"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(arguments[0].isObject()){
                                                        auto vertexArrayObject = arguments[0].asObject(runtime);
                                                        if(vertexArrayObject.isHostObject(runtime)){
                                                            auto vertexArray = vertexArrayObject.asHostObject<WebGLVertexArrayObject>(runtime);
                                                            if(vertexArray != nullptr){
                                                                canvas_native_webgl2_delete_vertex_array_with_vertex_array(
                                                                        vertexArray->GetVertexArrayObject(),
                                                                        this->GetState()
                                                                );
                                                            }

                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "drawArraysInstanced"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 3) {
                                                        auto mode = (uint32_t)arguments[0].asNumber();
                                                        auto first = (int32_t)arguments[1].asNumber();
                                                        auto count_ = (int32_t)arguments[2].asNumber();
                                                        auto instanceCount = (int32_t)arguments[3].asNumber();
                                                        canvas_native_webgl2_draw_arrays_instanced(
                                                                mode,
                                                                first,
                                                                count_,
                                                                instanceCount,
                                                                this->GetState()
                                                        );

                                                        this->UpdateInvalidateState();
                                                    }
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "drawBuffers"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (arguments[0].isObject()) {
                                                        auto buffersObject = arguments[0].asObject(runtime);
                                                        if(buffersObject.isArray(runtime)){
                                                            auto array = buffersObject.getArray(runtime);
                                                            auto len = array.size(runtime);
                                                            std::vector <uint32_t> buf;
                                                            buf.reserve(len);
                                                            for (int j = 0; j < len; ++j) {
                                                                auto item = array.getValueAtIndex(runtime, j);
                                                                buf.push_back((uint32_t)item.asNumber());
                                                            }
                                                            rust::Slice<const uint32_t> slice(buf.data(), buf.size());
                                                            canvas_native_webgl2_draw_buffers(slice, this->GetState());
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "drawElementsInstanced"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 5,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count > 4) {
                                                        auto mode = (uint32_t)arguments[0].asNumber();
                                                        auto count_ = (int32_t)arguments[1].asNumber();
                                                        auto type = (uint32_t)arguments[2].asNumber();
                                                        auto offset = arguments[3].asNumber();
                                                        auto instanceCount = (int32_t)arguments[4].asNumber();
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
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "drawRangeElements"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 6,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count > 5) {
                                                        auto mode = (uint32_t)arguments[0].asNumber();
                                                        auto start = (uint32_t)arguments[1].asNumber();
                                                        auto end = (uint32_t)arguments[2].asNumber();
                                                        auto count_ = (int32_t)arguments[3].asNumber();
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
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "endQuery"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (arguments[0].isNumber()) {
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        canvas_native_webgl2_end_query(target, this->GetState());
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "endTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 03,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    canvas_native_webgl2_end_transform_feedback(this->GetState());
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "fenceSync"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

            if(count > 1){
                auto condition = (uint32_t)arguments[0].asNumber();
                auto flags = (uint32_t)arguments[1].asNumber();
                canvas_native_webgl2_fence_sync(
                        condition,
                        flags,
                        this->GetState()
                );
            }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "framebufferTextureLayer"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 5,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 4 && arguments[2].isObject()) {
                                                        auto textureObject = arguments[2].asObject(runtime);
                                                        if(textureObject.isHostObject(runtime)){
                                                            auto target = (uint32_t)arguments[0].asNumber();
                                                            auto attachment = (uint32_t)arguments[1].asNumber();
                                                            auto texture = textureObject.asHostObject<WebGLTexture>(runtime);
                                                            auto level = (int32_t)arguments[3].asNumber();
                                                            auto layer = (int32_t)arguments[4].asNumber();
                                                            if(texture != nullptr){
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "getActiveUniformBlockName"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 1) {
                                                        auto program = getHostObject<WebGLProgram>(runtime, arguments[0]);
                                                        if(program != nullptr){
                                                            auto uniformBlockIndex = (uint32_t)arguments[1].asNumber();
                                                            auto name = canvas_native_webgl2_get_active_uniform_block_name(
                                                                    program->GetProgram(),
                                                                    uniformBlockIndex,
                                                                    this->GetState()
                                                            );
                                                            return jsi::String::createFromAscii(runtime, name.data(), name.size());
                                                        }

                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "getActiveUniformBlockParameter"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 2) {

                                                        auto program = getHostObject<WebGLProgram>(runtime, arguments[0]);

                                                        if(program != nullptr){
                                                            auto uniformBlockIndex = (uint32_t)arguments[1].asNumber();
                                                            auto pname = (uint32_t)arguments[2].asNumber();
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
                                                                    return {canvas_native_webgl_result_get_i32(*ret)};
                                                                case GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: {
                                                                    auto value = canvas_native_webgl_result_get_u32_array(*ret);
                                                                    auto buffer = std::make_shared<VecMutableBuffer<uint32_t>>(std::move(value));
                                                                    auto array = jsi::ArrayBuffer(runtime, buffer);

                                                                    auto Uint32Array = runtime.global()
                                                                            .getProperty(runtime,
                                                                                         "Uint32Array")
                                                                            .asObject(runtime)
                                                                            .asFunction(runtime);


                                                                    return Uint32Array.callAsConstructor(
                                                                            runtime, array);
                                                                }
                                                                case GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER:
                                                                case GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:
                                                                    return {canvas_native_webgl_result_get_bool(*ret)};
                                                                default:
                                                                    return Value::null();
                                                            }
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getActiveUniforms"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 2 && arguments[1].isObject()) {

                                                        auto program = getHostObject<WebGLProgram>(runtime, arguments[0]);
                                                        auto uniformIndicesObject = arguments[1].asObject(runtime);
                                                        auto pname = (uint32_t)arguments[2].asNumber();

                                                        if(uniformIndicesObject.isArray(runtime)){
                                                            auto uniformIndices = uniformIndicesObject.asArray(runtime);
                                                            auto size = uniformIndices.size(runtime);
                                                            std::vector <uint32_t> buf;
                                                            buf.reserve(size);
                                                            for (int j = 0; j < size;j++) {
                                                                auto item = (uint32_t)uniformIndices.getValueAtIndex(runtime, j).asNumber();
                                                                buf.push_back(item);
                                                            }
                                                            rust::Slice<const uint32_t> slice(buf.data(), buf.size());
                                                            auto ret = canvas_native_webgl2_get_active_uniforms(
                                                                    program->GetProgram(),
                                                                    slice,
                                                                    pname,
                                                                    this->GetState()
                                                            );

                                                            switch (pname) {
                                                                case GL_UNIFORM_TYPE:
                                                                case GL_UNIFORM_SIZE: {
                                                                    auto value = canvas_native_webgl_result_get_u32_array(*ret);
                                                                    auto array = jsi::Array(runtime, value.size());
                                                                    auto len = value.size();
                                                                    for (int j = 0; j < len; ++j) {
                                                                        auto item = value[j];
                                                                        array.setValueAtIndex(runtime, j, jsi::Value((int32_t)item));
                                                                    }
                                                                    return array;
                                                                }
                                                                    break;
                                                                case GL_UNIFORM_BLOCK_INDEX:
                                                                case GL_UNIFORM_OFFSET:
                                                                case GL_UNIFORM_ARRAY_STRIDE:
                                                                case GL_UNIFORM_MATRIX_STRIDE: {
                                                                    auto value = canvas_native_webgl_result_get_i32_array(*ret);
                                                                    auto array = jsi::Array(runtime, value.size());
                                                                    auto len = value.size();
                                                                    for (int j = 0; j < len; ++j) {
                                                                        auto item = value[j];
                                                                        array.setValueAtIndex(runtime, j, jsi::Value(item));
                                                                    }
                                                                    return array;
                                                                }
                                                                case GL_UNIFORM_IS_ROW_MAJOR: {
                                                                    auto value = canvas_native_webgl_result_get_bool_array(*ret);
                                                                    auto len = value.size();
                                                                    auto array = jsi::Array(runtime, len);
                                                                    for (int j = 0; j < len; ++j) {
                                                                        bool item = value[j] == 1;
                                                                        array.setValueAtIndex(runtime, j, jsi::Value(item));
                                                                    }
                                                                    return array;
                                                                }
                                                                default:
                                                                   return Value::null();
                                                            }
                                                        }


                                                    }
                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getBufferSubData"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count >= 3 && arguments[2].isObject()) {
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto srcByteOffset = arguments[1].asNumber();
                                                        auto dstDataObject = arguments[2].asObject(runtime);

                                                       if(dstDataObject.isTypedArray(runtime)){
                                                           ssize_t dstOffsetValue = 0;
                                                           if (arguments[3].isNumber()) {
                                                               dstOffsetValue = static_cast<ssize_t>(arguments[3].asNumber());
                                                           }

                                                           ssize_t lengthValue = 0;
                                                           if (arguments[4].isNumber()) {
                                                               lengthValue = static_cast<ssize_t>(arguments[4].asNumber());
                                                           }

                                                           auto array = dstDataObject.getTypedArray(runtime);
                                                           auto slice = GetTypedArrayData<uint8_t>(runtime, dstDataObject.getTypedArray(runtime));

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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "getFragDataLocation"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 1 && arguments[1].isString()) {
                                                        auto program = getHostObject<WebGLProgram>(runtime, arguments[0]);

                                                        if(program != nullptr){
                                                            auto name = arguments[1].asString(runtime).utf8(runtime);

                                                            auto ret = canvas_native_webgl2_get_frag_data_location(
                                                                    program->GetProgram(),
                                                                    rust::Str(name.data(), name.size()),
                                                                    this->GetState()
                                                            );

                                                           return {ret};
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getIndexedParameter"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1) {
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto index = (uint32_t)arguments[1].asNumber();
                                                        auto ret = canvas_native_webgl2_get_indexed_parameter(
                                                                target,
                                                                index,
                                                                this->GetState()
                                                        );

                                                        switch (target) {
                                                            case GL_UNIFORM_BUFFER_BINDING:
                                                            case GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: {
                                                                auto buffer = canvas_native_webgl2_indexed_parameter_get_buffer_value(*ret);
                                                                return jsi::Object::createFromHostObject(runtime, std::make_shared<WebGLBuffer>(buffer));
                                                            }
                                                                break;
                                                            case GL_TRANSFORM_FEEDBACK_BUFFER_SIZE:
                                                            case GL_TRANSFORM_FEEDBACK_BUFFER_START:
                                                            case GL_UNIFORM_BUFFER_SIZE:
                                                            case GL_UNIFORM_BUFFER_START: {
                                                                auto value = canvas_native_webgl2_indexed_parameter_get_value(*ret);
                                                                return {static_cast<double>(value)};
                                                            }
                                                                break;
                                                            default:
                                                                return Value::null();
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getInternalformatParameter"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count > 2) {
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto internalformat = (uint32_t)arguments[1].asNumber();
                                                        auto pname = (uint32_t)arguments[2].asNumber();
                                                        auto returnEarly = false;
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
                                                                auto buffer = std::make_shared<VecMutableBuffer<int32_t>>(std::move(value));
                                                                auto array = jsi::ArrayBuffer(runtime, buffer);

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
                                                                return Value::null();
                                                        }


                                                        auto ret = canvas_native_webgl2_get_internalformat_parameter(
                                                                target,
                                                                internalformat,
                                                                pname,
                                                                this->GetState()
                                                        );

                                                        if (pname == GL_SAMPLES) {
                                                            auto value = canvas_native_webgl_result_get_i32_array(*ret);

                                                            auto buffer = std::make_shared<VecMutableBuffer<int32_t>>(std::move(value));
                                                            auto array = jsi::ArrayBuffer(runtime, buffer);

                                                            auto Int32Array = runtime.global()
                                                                    .getProperty(runtime,
                                                                                 "Int32Array")
                                                                    .asObject(runtime)
                                                                    .asFunction(runtime);


                                                            return Int32Array.callAsConstructor(
                                                                    runtime, array);
                                                        } else {
                                                            return Value::null();
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getParameter"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 0) {
                                                        auto pname = (uint32_t)arguments[0].asNumber();
                                                        auto result = canvas_native_webgl2_get_parameter(pname,
                                                                                                         this->GetState());
                                                        switch (pname) {
                                                            case GL_COPY_READ_BUFFER_BINDING:
                                                            case GL_COPY_WRITE_BUFFER_BINDING:
                                                            case GL_DRAW_FRAMEBUFFER_BINDING:
                                                                return {canvas_native_webgl_result_get_i32(*result)};
                                                            default:
                                                               return this->GetParameterInternal(runtime, pname, std::move(result));
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getQueryParameter"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 1) {
                                                    auto query = getHostObject<WebGLQuery>(runtime, arguments[0]);
                                                    if(query != nullptr){
                                                        auto pname = (uint32_t)arguments[1].asNumber();


                                                        auto ret = canvas_native_webgl2_get_query_parameter(query,
                                                                                                            pname,
                                                                                                            this->GetState());
                                                        if (pname == GL_QUERY_RESULT) {
                                                            return {canvas_native_webgl_result_get_bool(*ret)};
                                                        } else if (pname == GL_QUERY_RESULT_AVAILABLE) {
                                                            return {canvas_native_webgl_result_get_u32(*ret)};
                                                        }
                                                    }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getQuery"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1) {
                                                        auto target = (uint32_t)arguments[0].asNumber();
                                                        auto pname = (uint32_t)arguments[1].asNumber();
                                                        auto ret = canvas_native_webgl2_get_query(target,
                                                                                                  pname,
                                                                                                  this->GetState());
                                                        if (pname == GL_CURRENT_QUERY) {
                                                            return {canvas_native_webgl_result_get_i32(*ret)};
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getSamplerParameter"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1) {
                                                        auto sampler = getHostObject<WebGLSampler>(runtime, arguments[0]);
                                                        if (sampler != nullptr) {
                                                            auto pname = (uint32_t)arguments[1].asNumber();
                                                            auto ret = canvas_native_webgl2_get_sampler_parameter(
                                                                    sampler->GetSampler(), pname,
                                                                    this->GetState());

                                                            switch (pname) {
                                                                case GL_TEXTURE_MAX_LOD:
                                                                case GL_TEXTURE_MIN_LOD:
                                                                   return {static_cast<double>(canvas_native_webgl_result_get_f32(*ret))};
                                                                case GL_TEXTURE_COMPARE_FUNC:
                                                                case GL_TEXTURE_COMPARE_MODE:
                                                                case GL_TEXTURE_MAG_FILTER:
                                                                case GL_TEXTURE_MIN_FILTER:
                                                                case GL_TEXTURE_WRAP_R:
                                                                case GL_TEXTURE_WRAP_S:
                                                                case GL_TEXTURE_WRAP_T:
                                                                    return {canvas_native_webgl_result_get_i32(*ret)};
                                                                default:
                                                                    return Value::null();
                                                            }
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getSyncParameter"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1) {
                                                        auto sync = getHostObject<WebGLSyncImpl>(runtime, arguments[0]);
                                                        if (sync != nullptr) {
                                                            auto pname = (uint32_t)arguments[1].asNumber();
                                                            auto ret = canvas_native_webgl2_get_sync_parameter(
                                                                    sync->GetSync(),
                                                                    pname,
                                                                    this->GetState());

                                                            switch (pname) {
                                                                case GL_OBJECT_TYPE:
                                                                case GL_SYNC_STATUS:
                                                                case GL_SYNC_CONDITION:
                                                                case GL_SYNC_FLAGS:
                                                                    return {canvas_native_webgl_result_get_i32(*ret)};
                                                                default:
                                                                    return Value::null();
                                                            }
                                                        }
                                                    }
                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getTransformFeedbackVarying"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count > 1) {
                                                        auto program = getHostObject<WebGLProgram>(runtime, arguments[0]);
                                                        auto index = (uint32_t)arguments[1].asNumber();
                                                        if (program != nullptr) {

                                                            auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                                                                    program,
                                                                    index,
                                                                    this->GetState()
                                                            );

                                                            if (canvas_native_webgl_active_info_get_is_empty(*ret)) {
                                                                return Value::null();
                                                            }

                                                            auto info = std::make_shared<WebGLActiveInfoImpl>(std::move(ret));

                                                            return jsi::Object::createFromHostObject(runtime, info);
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getUniformBlockIndex"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count > 1) {
                                                        auto program = getHostObject<WebGLProgram>(runtime,arguments[0]);
                                                        auto index = (uint32_t)arguments[1].asNumber();
                                                        if (program != nullptr) {
                                                            auto ret = canvas_native_webgl2_get_transform_feedback_varying(
                                                                    program->GetProgram(),
                                                                    index,
                                                                    this->GetState()
                                                            );

                                                            if (canvas_native_webgl_active_info_get_is_empty(*ret)) {
                                                               return Value::null();
                                                            }

                                                            auto info = std::make_shared<WebGLActiveInfoImpl>(std::move(ret));

                                                            return jsi::Object::createFromHostObject(runtime, info);
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "getUniformIndices"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1 && arguments[1].isObject()) {
                                                        auto program = getHostObject<WebGLProgram>(runtime, arguments[0]);
                                                        auto uniformNamesObject = arguments[1].asObject(runtime);
                                                        if (program != nullptr && uniformNamesObject.isArray(runtime)) {
                                                            auto uniformNames = uniformNamesObject.getArray(runtime);
                                                            auto len = uniformNames.size(runtime);
                                                            rust::Vec <rust::Str> store;
                                                            store.reserve(len);
                                                            for (int j = 0; j < len; ++j) {
                                                                auto name = uniformNames.getValueAtIndex(runtime, j).asString(runtime).utf8(runtime);
                                                                rust::Str val(name.data(), name.size());
                                                                store.push_back(val);
                                                            }
                                                            rust::Slice<const rust::Str> buf(store.data(), store.size());
                                                            auto ret = canvas_native_webgl2_get_uniform_indices(
                                                                    program->GetProgram(), buf,
                                                                    this->GetState());

                                                            auto retSize = ret.size();
                                                            auto result = jsi::Array(runtime, retSize);
                                                            for (int j = 0; j < retSize; ++j) {
                                                                auto item = ret[j];
                                                                result.setValueAtIndex(runtime, j, Value(item));
                                                            }

                                                            return result;
                                                        }
                                                    }

                                                    return Value::null();
                                                }
        );
    }else if(methodName == "invalidateFramebuffer"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void
                                                    WebGL2RenderingContext::InvalidateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
                                                                auto item = array->Get(context, j).ToLocalChecked();
                                                                buf.push_back(item->Uint32Value(context).ToChecked());
                                                            }
                                                            rust::Slice<const uint32_t> slice(buf.data(), buf.size());

                                                            canvas_native_webgl2_invalidate_framebuffer(target->Uint32Value(context).ToChecked(), slice,
                                                                                                        ptr->GetState());
                                                            return;
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "invalidateSubFramebuffer"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void
                                                    WebGL2RenderingContext::InvalidateSubFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
                                                                auto item = array->Get(context, j).ToLocalChecked();
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "isQuery"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::IsQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        if (args.Length() > 0) {
                                                            auto query = args[0];
                                                            if (Helpers::GetInstanceType(isolate, query) == ObjectType::WebGLQuery) {
                                                                auto instance = Helpers::GetPrivate(isolate, query.As<v8::Object>(), "instance");
                                                                if (!instance.IsEmpty()) {
                                                                    auto ret = canvas_native_webgl2_is_query(instance->Uint32Value(context).ToChecked(),
                                                                                                             ptr->GetState());
                                                                    args.GetReturnValue().Set(ret);
                                                                    return;
                                                                }
                                                            }
                                                        }

                                                        // todo check return
                                                        args.GetReturnValue().Set(false);
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "isSampler"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::IsSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        if (args.Length() > 0) {
                                                            auto sampler = args[0];
                                                            if (Helpers::GetInstanceType(isolate, sampler) == ObjectType::WebGLSampler) {
                                                                auto instance = Helpers::GetPrivate(isolate, sampler.As<v8::Object>(), "instance");
                                                                if (!instance.IsEmpty()) {
                                                                    auto ret = canvas_native_webgl2_is_sampler(
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "isSync"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "isTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::IsTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        if (args.Length() > 0) {
                                                            auto transformFeedback = args[0];
                                                            if (Helpers::GetInstanceType(isolate, transformFeedback) ==
                                                                ObjectType::WebGLTransformFeedback) {
                                                                auto instance = Helpers::GetPrivate(isolate, transformFeedback.As<v8::Object>(),
                                                                                                    "instance");
                                                                if (!instance.IsEmpty()) {
                                                                    auto ret = canvas_native_webgl2_is_transform_feedback(
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "isVertexArray"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::IsVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        if (args.Length() > 0) {
                                                            auto vertexArray = args[0];
                                                            if (Helpers::GetInstanceType(isolate, vertexArray) == ObjectType::WebGLVertexArrayObject) {
                                                                auto instance = Helpers::GetPrivate(isolate, vertexArray.As<v8::Object>(), "instance");
                                                                if (!instance.IsEmpty()) {
                                                                    auto ret = canvas_native_webgl2_is_vertex_array(
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "pauseTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    void
                                                    WebGL2RenderingContext::PauseTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        canvas_native_webgl2_pause_transform_feedback(ptr->GetState());
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "readBuffer"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "renderbufferStorageMultisample"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::RenderbufferStorageMultisample(
                                                            const v8::FunctionCallbackInfo<v8::Value> &args) {
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "resumeTransformFeedback"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void
                                                    WebGL2RenderingContext::ResumeTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        canvas_native_webgl2_resume_transform_feedback(ptr->GetState());
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "samplerParameterf"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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
                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "samplerParameteri"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "texImage3D"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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
                                                                auto asset = ImageAssetImpl::GetPointer(
                                                                        imageOrPixelsOrOffset->ToObject(context).ToLocalChecked());
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "texStorage2D"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "texStorage3D"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "bindBufferBase"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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
                                                                auto asset = ImageAssetImpl::GetPointer(
                                                                        imageOrPixelsOrOffset->ToObject(context).ToLocalChecked());

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

                                                    void
                                                    WebGL2RenderingContext::TransformFeedbackVaryings(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        if (args.Length() > 2) {
                                                            auto program = args[0];
                                                            auto varyings = args[1];
                                                            auto bufferMode = args[2];
                                                            if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram &&
                                                                varyings->IsArray()) {
                                                                auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
                                                                if (!instance.IsEmpty()) {

                                                                    auto array = varyings.As<v8::Array>();
                                                                    rust::Vec <rust::Str> store;

                                                                    auto len = array->Length();
                                                                    for (int j = 0; j < len; ++j) {
                                                                        auto name = array->Get(context, j).ToLocalChecked();
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "texSubImage3D"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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
                                                                auto asset = ImageAssetImpl::GetPointer(
                                                                        imageOrPixelsOrOffset->ToObject(context).ToLocalChecked());

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

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "transformFeedbackVaryings"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void
                                                    WebGL2RenderingContext::TransformFeedbackVaryings(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        if (args.Length() > 2) {
                                                            auto program = args[0];
                                                            auto varyings = args[1];
                                                            auto bufferMode = args[2];
                                                            if (Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram &&
                                                                varyings->IsArray()) {
                                                                auto instance = Helpers::GetPrivate(isolate, program.As<v8::Object>(), "instance");
                                                                if (!instance.IsEmpty()) {

                                                                    auto array = varyings.As<v8::Array>();
                                                                    rust::Vec <rust::Str> store;

                                                                    auto len = array->Length();
                                                                    for (int j = 0; j < len; ++j) {
                                                                        auto name = array->Get(context, j).ToLocalChecked();
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "uniform1ui"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform1ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto v0 = args[1];
                                                        if (args.Length() > 1 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);
                                                            canvas_native_webgl2_uniform1ui(
                                                                    locationValue.ToLocalChecked()->Value(),
                                                                    v0->Uint32Value(context).ToChecked(),
                                                                    ptr->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "Uniform1uiv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform1uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto data = args[1];
                                                        if (args.Length() > 1 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);

                                                            if (data->IsUint32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const uint32_t>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform1uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else {
                                                                std::vector <uint32_t> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i).ToLocalChecked();
                                                                    buf.push_back(item->Uint32Value(context).ToChecked());
                                                                }

                                                                rust::Slice<const uint32_t> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform1uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }


                                                        }
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "Uniform2ui"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform2ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto v0 = args[1];
                                                        auto v1 = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);
                                                            canvas_native_webgl2_uniform2ui(
                                                                    locationValue.ToLocalChecked()->Value(),
                                                                    v0->Uint32Value(context).ToChecked(),
                                                                    v1->Uint32Value(context).ToChecked(),
                                                                    ptr->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "Uniform2uiv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform2uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto data = args[1];
                                                        if (args.Length() > 1 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);

                                                            if (data->IsUint32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const uint32_t>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform2uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else {
                                                                std::vector <uint32_t> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i).ToLocalChecked();
                                                                    buf.push_back(item->Uint32Value(context).ToChecked());
                                                                }

                                                                rust::Slice<const uint32_t> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform2uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "Uniform3ui"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform3ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto v0 = args[1];
                                                        auto v1 = args[2];
                                                        auto v2 = args[3];
                                                        auto v3 = args[4];
                                                        if (args.Length() > 3 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);
                                                            canvas_native_webgl2_uniform3ui(
                                                                    locationValue.ToLocalChecked()->Value(),
                                                                    v0->Uint32Value(context).ToChecked(),
                                                                    v1->Uint32Value(context).ToChecked(),
                                                                    v2->Uint32Value(context).ToChecked(),
                                                                    ptr->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "Uniform3uiv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform3uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto data = args[1];
                                                        if (args.Length() > 1 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);

                                                            if (data->IsUint32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const uint32_t>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform3uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else {
                                                                std::vector <uint32_t> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i).ToLocalChecked();
                                                                    buf.push_back(item->Uint32Value(context).ToChecked());
                                                                }

                                                                rust::Slice<const uint32_t> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform3uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "Uniform4ui"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform4ui(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto v0 = args[1];
                                                        auto v1 = args[2];
                                                        auto v2 = args[3];
                                                        auto v3 = args[4];
                                                        if (args.Length() > 4 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "Uniform4uiv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::Uniform4uiv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto data = args[1];
                                                        if (args.Length() > 1 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);

                                                            if (data->IsUint32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const uint32_t>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform4uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else {
                                                                std::vector <uint32_t> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i).ToLocalChecked();
                                                                    buf.push_back(item->Uint32Value(context).ToChecked());
                                                                }

                                                                rust::Slice<const uint32_t> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform4uiv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "UniformBlockBinding"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::UniformBlockBinding(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto program = args[0];
                                                        auto uniformBlockIndex = args[1];
                                                        auto uniformBlockBinding = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, program) == ObjectType::WebGLProgram) {
                                                            auto programValue = Helpers::GetPrivate(isolate, program.As<v8::Object>(),
                                                                                                    "instance")->ToUint32(context);
                                                            canvas_native_webgl2_uniform_block_binding(
                                                                    programValue.ToLocalChecked()->Value(),
                                                                    uniformBlockIndex->Uint32Value(context).ToChecked(),
                                                                    uniformBlockBinding->Uint32Value(context).ToChecked(),
                                                                    ptr->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "UniformMatrix2x3fv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::UniformMatrix2x3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto transpose = args[1];
                                                        auto data = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);

                                                            if (data->IsFloat32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const float>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform_matrix2x3fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else if (data->IsArray()) {
                                                                std::vector<float> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i);
                                                                    if (!item.IsEmpty()) {
                                                                        buf.push_back(static_cast<float>(Helpers::GetNumberValue(isolate, item.ToLocalChecked())));
                                                                    } else {
                                                                        buf.push_back(std::nanf(""));
                                                                    }
                                                                }

                                                                rust::Slice<const float> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform_matrix2x3fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "UniformMatrix2x4fv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::UniformMatrix2x4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto transpose = args[1];
                                                        auto data = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);

                                                            if (data->IsFloat32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const float>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform_matrix2x4fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else if (data->IsArray()) {
                                                                std::vector<float> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i);
                                                                    if (!item.IsEmpty()) {
                                                                        buf.push_back(static_cast<float>(Helpers::GetNumberValue(isolate, item.ToLocalChecked())));
                                                                    } else {
                                                                        buf.push_back(std::nanf(""));
                                                                    }
                                                                }

                                                                rust::Slice<const float> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform_matrix2x4fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "UniformMatrix3x2fv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::UniformMatrix3x2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto transpose = args[1];
                                                        auto data = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);

                                                            if (data->IsFloat32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const float>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform_matrix3x2fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else if (data->IsArray()) {
                                                                std::vector<float> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i);
                                                                    if (!item.IsEmpty()) {
                                                                        buf.push_back(static_cast<float>(Helpers::GetNumberValue(isolate, item.ToLocalChecked())));
                                                                    } else {
                                                                        buf.push_back(std::nanf(""));
                                                                    }
                                                                }

                                                                rust::Slice<const float> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform_matrix3x2fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "UniformMatrix3x4fv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::UniformMatrix3x4fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto transpose = args[1];
                                                        auto data = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);
                                                            if (data->IsFloat32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const float>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform_matrix3x4fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else if (data->IsArray()) {
                                                                std::vector<float> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i);
                                                                    if (!item.IsEmpty()) {
                                                                        buf.push_back(static_cast<float>(Helpers::GetNumberValue(isolate, item.ToLocalChecked())));
                                                                    } else {
                                                                        buf.push_back(std::nanf(""));
                                                                    }
                                                                }

                                                                rust::Slice<const float> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform_matrix3x4fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "UniformMatrix4x2fv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::UniformMatrix4x2fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto transpose = args[1];
                                                        auto data = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);
                                                            if (data->IsFloat32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const float>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform_matrix4x2fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else if (data->IsArray()) {
                                                                std::vector<float> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i);
                                                                    if (!item.IsEmpty()) {
                                                                        buf.push_back(static_cast<float>(Helpers::GetNumberValue(isolate, item.ToLocalChecked())));
                                                                    } else {
                                                                        buf.push_back(std::nanf(""));
                                                                    }
                                                                }

                                                                rust::Slice<const float> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform_matrix4x2fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if(methodName == "UniformMatrix4x3fv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    void WebGL2RenderingContext::UniformMatrix4x3fv(const v8::FunctionCallbackInfo<v8::Value> &args) {
                                                        auto isolate = args.GetIsolate();
                                                        auto context = isolate->GetCurrentContext();
                                                        auto ptr = GetPointerBase(args.This());
                                                        auto location = args[0];
                                                        auto transpose = args[1];
                                                        auto data = args[2];
                                                        if (args.Length() > 2 &&
                                                            Helpers::GetInstanceType(isolate, location) == ObjectType::WebGLUniformLocation) {
                                                            auto locationValue = Helpers::GetPrivate(isolate, location.As<v8::Object>(),
                                                                                                     "instance")->ToInt32(context);
                                                            if (data->IsFloat32Array()) {
                                                                auto slice = Helpers::GetTypedArrayData<const float>(data.As<v8::TypedArray>());
                                                                canvas_native_webgl2_uniform_matrix4x3fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            } else if (data->IsArray()) {
                                                                std::vector<float> buf;
                                                                auto array = data.As<v8::Array>();
                                                                auto len = array->Length();
                                                                for (int i = 0; i < len; i++) {
                                                                    auto item = array->Get(context, i);
                                                                    if (!item.IsEmpty()) {
                                                                        buf.push_back(static_cast<float>(Helpers::GetNumberValue(isolate, item.ToLocalChecked())));
                                                                    } else {
                                                                        buf.push_back(std::nanf(""));
                                                                    }
                                                                }

                                                                rust::Slice<const float> slice(buf.data(), len);

                                                                canvas_native_webgl2_uniform_matrix4x3fv(
                                                                        locationValue.ToLocalChecked()->Value(),
                                                                        transpose->BooleanValue(isolate),
                                                                        slice,
                                                                        ptr->GetState()
                                                                );
                                                            }
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "VertexAttribDivisor"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "VertexAttribI4i"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
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

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "VertexAttribI4iv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1 && arguments[1].isObject()) {
                                                        auto index = (uint32_t)arguments[0].asNumber();
                                                        auto value = arguments[1].asObject(runtime);
                                                        if (value.isInt32Array(runtime)) {
                                                            auto array = value.getTypedArray(runtime);
                                                            auto slice = GetTypedArrayData<const int32_t>(runtime, array);
                                                            canvas_native_webgl2_vertex_attrib_i4iv(
                                                                    index,
                                                                    slice,
                                                                    this->GetState()
                                                            );
                                                        } else if (value.isArray(runtime)) {
                                                            auto array = value.getArray(runtime);
                                                            auto len = array.size(runtime);
                                                            std::vector <int32_t> buf;
                                                            buf.reserve(len);
                                                            for (int i = 0; i < len; i++) {
                                                                auto item = (int32_t)array.getValueAtIndex(runtime, i).asNumber();
                                                                buf.push_back(item);
                                                            }

                                                            rust::Slice<const int32_t> slice(buf.data(), len);

                                                            canvas_native_webgl2_vertex_attrib_i4iv(
                                                                    index,
                                                                    slice,
                                                                    this->GetState()
                                                            );
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    }else if(methodName == "VertexAttribI4ui"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 5,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                   if(count > 4){
                                                       auto index = (uint32_t)arguments[0].asNumber();
                                                       auto v0 = (uint32_t)arguments[1].asNumber();
                                                       auto v1 = (uint32_t)arguments[2].asNumber();
                                                       auto v2 = (uint32_t)arguments[3].asNumber();
                                                       auto v3 = (uint32_t)arguments[4].asNumber();

                                                       canvas_native_webgl2_vertex_attrib_i4ui(
                                                               index,
                                                               v0,
                                                               v1,
                                                               v2,
                                                               v3,
                                                               this->GetState()
                                                       );
                                                   }
                                                    return Value::undefined();
        }
        );
    }else if(methodName == "vertexAttribI4uiv"){
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count > 1 && arguments[1].isObject()) {
                                                        auto index = (uint32_t)arguments[0].asNumber();
                                                        auto value = arguments[1].asObject(runtime);
                                                        if (value.isUint32Array(runtime)) {
                                                            auto array = value.getTypedArray(runtime);
                                                            auto slice = GetTypedArrayData<const uint32_t>(runtime, array);
                                                            canvas_native_webgl2_vertex_attrib_i4uiv(
                                                                    index,
                                                                    slice,
                                                                    this->GetState()
                                                            );
                                                        } else if (value.isArray(runtime)) {
                                                            auto array = value.getArray(runtime);
                                                            auto len = array.size(runtime);
                                                            std::vector <uint32_t> buf;
                                                            buf.reserve(len);
                                                            for (int i = 0; i < len; i++) {
                                                                auto item = (uint32_t)array.getValueAtIndex(runtime, i).asNumber();
                                                                buf.push_back(item);
                                                            }

                                                            rust::Slice<const uint32_t> slice(buf.data(), len);

                                                            canvas_native_webgl2_vertex_attrib_i4uiv(
                                                                    index,
                                                                    slice,
                                                                    this->GetState()
                                                            );
                                                        }
                                                    }
                                                    return Value::undefined();
                                                }
        );
    }
    return Value::undefined();
}

void WebGL2RenderingContext::InstanceFromPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
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
            version = Helpers::ConvertFromV8String(isolate, versionValue->ToString(
                    context).ToLocalChecked());
        }


        auto alphaMaybe = config->Get(context, Helpers::ConvertToV8String(isolate, "alpha"));
        if (!alphaMaybe.IsEmpty()) {
            auto alphaLocal = alphaMaybe.ToLocalChecked();
            if (alphaLocal->IsBoolean()) {
                alpha = alphaLocal->BooleanValue(isolate);
            }
        }

        auto antialiasMaybe = config->Get(context,
                                          Helpers::ConvertToV8String(isolate, "antialias"));
        if (!antialiasMaybe.IsEmpty()) {
            auto antialiasLocal = antialiasMaybe.ToLocalChecked();
            if (antialiasLocal->IsBoolean()) {
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
                power_preference = Helpers::ConvertFromV8String(isolate, powerPreferenceLocal);
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
            renderingContext->SetRaf(
                    std::make_shared<RafImpl>(raf_callback, raf_callback_ptr, std::move(raf)));

            auto _raf = renderingContext->GetRaf();
            canvas_native_raf_start(_raf->GetRaf());

            auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
            Helpers::SetInstanceType(isolate, ret, ObjectType::WebGL2RenderingContext);
            AddWeakListener(isolate, ret, renderingContext);
            args.GetReturnValue().Set(ret);
        }
        return;
    }
    args.GetReturnValue().SetUndefined();
}


