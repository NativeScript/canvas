//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "canvas2d/CanvasRenderingContext2DImpl.h"
#include "RafImpl.h"
#include <vector>
#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"
#include "VecMutableBuffer.h"
#include "canvas-cxx/src/webgl2.rs.h"
#include <cmath>
#include "Helpers.h"
#include "webgl/WebGLRenderingContextBase.h"
#include "webgl/WebGLRenderingContext.h"
#include "WebGLQuery.h"
#include "WebGLSampler.h"
#include "WebGLSyncImpl.h"
#include "WebGLTransformFeedback.h"
#include "WebGLVertexArrayObject.h"
#include "gl.h"
#include "canvas-cxx/src/constants.rs.h"

using namespace facebook;

class JSI_EXPORT WebGL2RenderingContext : public WebGLRenderingContext {
public:

    WebGL2RenderingContext(rust::Box<WebGLState> state);

    WebGL2RenderingContext(rust::Box<WebGLState> state, WebGLRenderingVersion version);

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    static inline jsi::Value GetProperty(const std::string &methodName) {
        if (methodName == "READ_BUFFER") { return {0x0C02}; }

        if (methodName == "UNPACK_ROW_LENGTH") { return {0x0CF2}; }

        if (methodName == "UNPACK_SKIP_ROWS") { return {0x0CF3}; }

        if (methodName == "UNPACK_SKIP_PIXELS") { return {0x0CF4}; }

        if (methodName == "PACK_ROW_LENGTH") { return {0x0D02}; }

        if (methodName == "PACK_SKIP_ROWS") { return {0x0D03}; }

        if (methodName == "PACK_SKIP_PIXELS") { return {0x0D04}; }

        if (methodName == "TEXTURE_BINDING_3D") { return {0x806A}; }

        if (methodName == "UNPACK_SKIP_IMAGES") { return {0x806D}; }

        if (methodName == "UNPACK_IMAGE_HEIGHT") { return {0x806E}; }

        if (methodName == "MAX_3D_TEXTURE_SIZE") { return {0x8073}; }

        if (methodName == "MAX_ELEMENTS_VERTICES") { return {0x80E8}; }

        if (methodName == "MAX_ELEMENTS_INDICES") { return {0x80E9}; }

        if (methodName == "MAX_TEXTURE_LOD_BIAS") { return {0x84FD}; }

        if (methodName == "MAX_FRAGMENT_UNIFORM_COMPONENTS") { return {0x8B49}; }

        if (methodName == "MAX_VERTEX_UNIFORM_COMPONENTS") { return {0x8B4A}; }

        if (methodName == "MAX_ARRAY_TEXTURE_LAYERS") { return {0x88FF}; }

        if (methodName == "MIN_PROGRAM_TEXEL_OFFSET") { return {0x8904}; }

        if (methodName == "MAX_PROGRAM_TEXEL_OFFSET") { return {0x8905}; }

        if (methodName == "MAX_VARYING_COMPONENTS") { return {0x8B4B}; }

        if (methodName == "FRAGMENT_SHADER_DERIVATIVE_HINT") { return {0x8B8B}; }

        if (methodName == "RASTERIZER_DISCARD") { return {0x8C89}; }

        if (methodName == "VERTEX_ARRAY_BINDING") { return {0x85B5}; }
        if (methodName == "MAX_VERTEX_OUTPUT_COMPONENTS") { return {0x9122}; }
        if (methodName == "MAX_FRAGMENT_INPUT_COMPONENTS") { return {0x9125}; }
        if (methodName == "MAX_SERVER_WAIT_TIMEOUT") { return {0x9111}; }
        if (methodName == "MAX_ELEMENT_INDEX") { return {0x8D6B}; }

        if (methodName == "RED") { return {0x1903}; }
        if (methodName == "RGB8") { return {0x8051}; }
        if (methodName == "RGBA8") { return {0x8058}; }
        if (methodName == "RGB10_A2") { return {0x8059}; }
        if (methodName == "TEXTURE_3D") { return {0x806F}; }

        if (methodName == "TEXTURE_WRAP_R") { return {0x8072}; }
        if (methodName == "TEXTURE_MIN_LOD") { return {0x813A}; }
        if (methodName == "TEXTURE_MAX_LOD") { return {0x813B}; }
        if (methodName == "TEXTURE_BASE_LEVEL") { return {0x813C}; }
        if (methodName == "TEXTURE_MAX_LEVEL") { return {0x813D}; }


        if (methodName == "TEXTURE_COMPARE_MODE") { return {0x884C}; }
        if (methodName == "TEXTURE_COMPARE_FUNC") { return {0x884D}; }
        if (methodName == "SRGB") { return {0x8C40}; }
        if (methodName == "SRGB8") { return {0x8C41}; }
        if (methodName == "SRGB8_ALPHA8") { return {0x8C43}; }

        if (methodName == "COMPARE_REF_TO_TEXTURE") { return {0x884E}; }
        if (methodName == "RGBA32F") { return {0x8814}; }
        if (methodName == "RGB32F") { return {0x8815}; }
        if (methodName == "RGBA16F") { return {0x881A}; }
        if (methodName == "RGB16F") { return {0x881B}; }

        if (methodName == "TEXTURE_2D_ARRAY") { return {0x8C1A}; }
        if (methodName == "TEXTURE_BINDING_2D_ARRAY") { return {0x8C1D}; }
        if (methodName == "R11F_G11F_B10F") { return {0x8C3A}; }
        if (methodName == "RGB9_E5") { return {0x8C3D}; }
        if (methodName == "RGBA32UI") { return {0x8D70}; }


        if (methodName == "RGB32UI") { return {0x8D71}; }
        if (methodName == "RGBA16UI") { return {0x8D76}; }
        if (methodName == "RGB16UI") { return {0x8D77}; }
        if (methodName == "RGBA8UI") { return {0x8D7C}; }
        if (methodName == "RGB8UI") { return {0x8D7D}; }


        if (methodName == "RGBA32I") { return {0x8D82}; }
        if (methodName == "RGB32I") { return {0x8D83}; }
        if (methodName == "RGBA16I") { return {0x8D88}; }
        if (methodName == "RGB16I") { return {0x8D89}; }
        if (methodName == "RGBA8I") { return {0x8D8E}; }


        if (methodName == "RGB8I") { return {0x8D8F}; }
        if (methodName == "RED_INTEGER") { return {0x8D94}; }
        if (methodName == "RGB_INTEGER") { return {0x8D98}; }
        if (methodName == "RGBA_INTEGER") { return {0x8D99}; }
        if (methodName == "R8") { return {0x8229}; }


        if (methodName == "RG8") { return {0x822B}; }
        if (methodName == "R16F") { return {0x822D}; }
        if (methodName == "R32F") { return {0x822E}; }
        if (methodName == "RG16F") { return {0x822F}; }
        if (methodName == "RG32F") { return {0x8230}; }


        if (methodName == "R8I") { return {0x8231}; }
        if (methodName == "R8UI") { return {0x8232}; }
        if (methodName == "R16I") { return {0x8233}; }
        if (methodName == "R16UI") { return {0x8234}; }
        if (methodName == "R32I") { return {0x8235}; }


        if (methodName == "R32UI") { return {0x8236}; }
        if (methodName == "RG8I") { return {0x8237}; }
        if (methodName == "RG8UI") { return {0x8238}; }
        if (methodName == "RG16I") { return {0x8239}; }
        if (methodName == "RG16UI") { return {0x823A}; }

        if (methodName == "RG32I") { return {0x823B}; }
        if (methodName == "RG32UI") { return {0x823C}; }
        if (methodName == "R8_SNORM") { return {0x8F94}; }
        if (methodName == "RG8_SNORM") { return {0x8F95}; }
        if (methodName == "RGB8_SNORM") { return {0x8F96}; }


        if (methodName == "RGBA8_SNORM") { return {0x8F97}; }
        if (methodName == "RGB10_A2UI") { return {0x906F}; }
        if (methodName == "TEXTURE_IMMUTABLE_FORMAT") { return {0x912F}; }
        if (methodName == "TEXTURE_IMMUTABLE_LEVELS") { return {0x82DF}; }
        if (methodName == "UNSIGNED_INT_2_10_10_10_REV") { return {0x8368}; }


        if (methodName == "UNSIGNED_INT_10F_11F_11F_REV") { return {0x8C3B}; }
        if (methodName == "UNSIGNED_INT_5_9_9_9_REV") { return {0x8C3E}; }
        if (methodName == "FLOAT_32_UNSIGNED_INT_24_8_REV") { return {0x8DAD}; }
        if (methodName == "UNSIGNED_INT_24_8") { return {0x84FA}; }
        if (methodName == "HALF_FLOAT") { return {0x140B}; }


        if (methodName == "RG") { return {0x8227}; }
        if (methodName == "RG_INTEGER") { return {0x8228}; }
        if (methodName == "INT_2_10_10_10_REV") { return {0x8D9F}; }
        if (methodName == "QUERY_RESULT_AVAILABLE") { return {0x8865}; }
        if (methodName == "QUERY_RESULT") { return {0x8866}; }


        if (methodName == "CURRENT_QUERY") { return {0x8867}; }
        if (methodName == "ANY_SAMPLES_PASSED") { return {0x8C2F}; }
        if (methodName == "ANY_SAMPLES_PASSED_CONSERVATIVE") { return {0x8D6A}; }
        if (methodName == "MAX_DRAW_BUFFERS") { return {0x8824}; }

        if (methodName == "DRAW_BUFFER0") { return {0x8825}; }
        if (methodName == "DRAW_BUFFER1") { return {0x8826}; }
        if (methodName == "DRAW_BUFFER2") { return {0x8827}; }
        if (methodName == "DRAW_BUFFER3") { return {0x8828}; }
        if (methodName == "DRAW_BUFFER4") { return {0x8829}; }
        if (methodName == "DRAW_BUFFER5") { return {0x882A}; }
        if (methodName == "DRAW_BUFFER6") { return {0x882B}; }
        if (methodName == "DRAW_BUFFER7") { return {0x882C}; }
        if (methodName == "DRAW_BUFFER8") { return {0x882D}; }
        if (methodName == "DRAW_BUFFER9") { return {0x882E}; }
        if (methodName == "DRAW_BUFFER10") { return {0x882F}; }

        /* Getting GL parameter information */

        /* Textures */

        if (methodName == "DRAW_BUFFER11") { return {0x8830}; }
        if (methodName == "DRAW_BUFFER12") { return {0x8831}; }
        if (methodName == "DRAW_BUFFER13") { return {0x8832}; }
        if (methodName == "DRAW_BUFFER14") { return {0x8833}; }
        if (methodName == "DRAW_BUFFER15") { return {0x8834}; }

        if (methodName == "MAX_COLOR_ATTACHMENTS") { return {0x8CDF}; }
        if (methodName == "COLOR_ATTACHMENT1") { return {0x8CE1}; }
        if (methodName == "COLOR_ATTACHMENT2") { return {0x8CE2}; }
        if (methodName == "COLOR_ATTACHMENT3") { return {0x8CE3}; }
        if (methodName == "COLOR_ATTACHMENT4") { return {0x8CE4}; }
        if (methodName == "COLOR_ATTACHMENT5") { return {0x8CE5}; }
        if (methodName == "COLOR_ATTACHMENT6") { return {0x8CE6}; }
        if (methodName == "COLOR_ATTACHMENT7") { return {0x8CE7}; }
        if (methodName == "COLOR_ATTACHMENT8") { return {0x8CE8}; }
        if (methodName == "COLOR_ATTACHMENT9") { return {0x8CE9}; }
        if (methodName == "COLOR_ATTACHMENT10") { return {0x8CEA}; }
        if (methodName == "COLOR_ATTACHMENT11") { return {0x8CEB}; }
        if (methodName == "COLOR_ATTACHMENT12") { return {0x8CEC}; }
        if (methodName == "COLOR_ATTACHMENT13") { return {0x8CED}; }
        if (methodName == "COLOR_ATTACHMENT14") { return {0x8CEE}; }
        if (methodName == "COLOR_ATTACHMENT15") { return {0x8CEF}; }

        if (methodName == "SAMPLER_3D") { return {0x8B5F}; }
        if (methodName == "SAMPLER_2D_SHADOW") { return {0x8B62}; }
        if (methodName == "SAMPLER_2D_ARRAY") { return {0x8DC1}; }
        if (methodName == "SAMPLER_2D_ARRAY_SHADOW") { return {0x8DC4}; }
        if (methodName == "SAMPLER_CUBE_SHADOW") { return {0x8DC5}; }

        if (methodName == "INT_SAMPLER_2D") { return {0x8DCA}; }
        if (methodName == "INT_SAMPLER_3D") { return {0x8DCB}; }
        if (methodName == "INT_SAMPLER_CUBE") { return {0x8DCC}; }
        if (methodName == "INT_SAMPLER_2D_ARRAY") { return {0x8DCF}; }
        if (methodName == "UNSIGNED_INT_SAMPLER_2D") { return {0x8DD2}; }

        if (methodName == "UNSIGNED_INT_SAMPLER_3D") { return {0x8DD3}; }
        if (methodName == "UNSIGNED_INT_SAMPLER_CUBE") { return {0x8DD4}; }
        if (methodName == "UNSIGNED_INT_SAMPLER_2D_ARRAY") { return {0x8DD7}; }
        if (methodName == "MAX_SAMPLES") { return {0x8D57}; }
        if (methodName == "SAMPLER_BINDING") { return {0x8919}; }

        if (methodName == "PIXEL_PACK_BUFFER") { return {0x88EB}; }
        if (methodName == "PIXEL_UNPACK_BUFFER") { return {0x88EC}; }
        if (methodName == "PIXEL_PACK_BUFFER_BINDING") { return {0x88ED}; }
        if (methodName == "PIXEL_UNPACK_BUFFER_BINDING") { return {0x88EF}; }
        if (methodName == "COPY_READ_BUFFER") { return {0x8F36}; }

        if (methodName == "COPY_WRITE_BUFFER") { return {0x8F37}; }
        if (methodName == "COPY_READ_BUFFER_BINDING") { return {0x8F36}; }
        if (methodName == "COPY_WRITE_BUFFER_BINDING") { return {0x8F37}; }
        if (methodName == "FLOAT_MAT2x3") { return {0x8B65}; }
        if (methodName == "FLOAT_MAT2x4") { return {0x8B66}; }

        if (methodName == "FLOAT_MAT3x2") { return {0x8B67}; }
        if (methodName == "FLOAT_MAT3x4") { return {0x8B68}; }
        if (methodName == "FLOAT_MAT4x2") { return {0x8B69}; }
        if (methodName == "FLOAT_MAT4x3") { return {0x8B6A}; }
        if (methodName == "UNSIGNED_INT_VEC2") { return {0x8DC6}; }

        if (methodName == "UNSIGNED_INT_VEC3") { return {0x8DC7}; }
        if (methodName == "UNSIGNED_INT_VEC4") { return {0x8DC8}; }
        if (methodName == "UNSIGNED_NORMALIZED") { return {0x8C17}; }
        if (methodName == "SIGNED_NORMALIZED") { return {0x8F9C}; }

        /* Vertex attributes */

        if (methodName == "VERTEX_ATTRIB_ARRAY_INTEGER") { return {0x88FD}; }
        if (methodName == "VERTEX_ATTRIB_ARRAY_DIVISOR") { return {0x88FE}; }
        if (methodName == "TRANSFORM_FEEDBACK_BUFFER_MODE") { return {0x8C7F}; }
        if (methodName == "MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS") { return {0x8C80}; }
        if (methodName == "TRANSFORM_FEEDBACK_VARYINGS") { return {0x8C83}; }

        if (methodName == "TRANSFORM_FEEDBACK_BUFFER_START") { return {0x8C84}; }
        if (methodName == "TRANSFORM_FEEDBACK_BUFFER_SIZE") { return {0x8C85}; }
        if (methodName == "TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN") { return {0x8C88}; }
        if (methodName == "MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS") { return {0x8C8A}; }

        /* Textures */

        /* Pixel types */

        if (methodName == "MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS") { return {0x8C8B}; }
        if (methodName == "INTERLEAVED_ATTRIBS") { return {0x8C8C}; }
        if (methodName == "SEPARATE_ATTRIBS") { return {0x8C8D}; }
        if (methodName == "TRANSFORM_FEEDBACK_BUFFER") { return {0x8C8E}; }
        if (methodName == "TRANSFORM_FEEDBACK_BUFFER_BINDING") { return {0x8C8F}; }

        if (methodName == "TRANSFORM_FEEDBACK") { return {0x8E22}; }
        if (methodName == "TRANSFORM_FEEDBACK_PAUSED") { return {0x8E23}; }
        if (methodName == "TRANSFORM_FEEDBACK_ACTIVE") { return {0x8E24}; }
        if (methodName == "TRANSFORM_FEEDBACK_BINDING") { return {0x8E25}; }

        /* Pixel types */

        /* Queries */

        if (methodName == "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING") { return {0x8210}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE") { return {0x8211}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_RED_SIZE") { return {0x8212}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_GREEN_SIZE") { return {0x8213}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_BLUE_SIZE") { return {0x8214}; }

        /* Queries */

        /* Draw buffers */

        if (methodName == "FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE") { return {0x8215}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE") { return {0x8216}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE") { return {0x8217}; }
        if (methodName == "FRAMEBUFFER_DEFAULT") { return {0x8218}; }
        if (methodName == "DEPTH_STENCIL_ATTACHMENT") { return {0x821A}; }

        if (methodName == "DEPTH_STENCIL") { return {0x84F9}; }
        if (methodName == "DEPTH24_STENCIL8") { return {0x88F0}; }
        if (methodName == "DRAW_FRAMEBUFFER_BINDING") { return {0x8CA6}; }
        if (methodName == "READ_FRAMEBUFFER") { return {0x8CA8}; }
        if (methodName == "DRAW_FRAMEBUFFER") { return {0x8CA9}; }

        if (methodName == "READ_FRAMEBUFFER_BINDING") { return {0x8CAA}; }
        if (methodName == "RENDERBUFFER_SAMPLES") { return {0x8CAB}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER") { return {0x8CD4}; }
        if (methodName == "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE") { return {0x8D56}; }
        if (methodName == "UNIFORM_BUFFER") { return {0x8A11}; }

        if (methodName == "UNIFORM_BUFFER_BINDING") { return {0x8A28}; }
        if (methodName == "UNIFORM_BUFFER_START") { return {0x8A29}; }
        if (methodName == "UNIFORM_BUFFER_SIZE") { return {0x8A2A}; }
        if (methodName == "MAX_VERTEX_UNIFORM_BLOCKS") { return {0x8A2B}; }
        if (methodName == "MAX_FRAGMENT_UNIFORM_BLOCKS") { return {0x8A2D}; }

        if (methodName == "MAX_COMBINED_UNIFORM_BLOCKS") { return {0x8A2E}; }
        if (methodName == "MAX_UNIFORM_BUFFER_BINDINGS") { return {0x8A2F}; }
        if (methodName == "MAX_UNIFORM_BLOCK_SIZE") { return {0x8A30}; }
        if (methodName == "MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS") { return {0x8A31}; }
        if (methodName == "MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS") { return {0x8A33}; }

        if (methodName == "UNIFORM_BUFFER_OFFSET_ALIGNMENT") { return {0x8A34}; }
        if (methodName == "ACTIVE_UNIFORM_BLOCKS") { return {0x8A36}; }
        if (methodName == "UNIFORM_TYPE") { return {0x8A37}; }
        if (methodName == "UNIFORM_SIZE") { return {0x8A38}; }
        if (methodName == "UNIFORM_BLOCK_INDEX") { return {0x8A3A}; }

        if (methodName == "UNIFORM_OFFSET") { return {0x8A3B}; }
        if (methodName == "UNIFORM_ARRAY_STRIDE") { return {0x8A3C}; }
        if (methodName == "UNIFORM_MATRIX_STRIDE") { return {0x8A3D}; }

        /* Draw buffers */

        /* Samplers */

        if (methodName == "UNIFORM_IS_ROW_MAJOR") { return {0x8A3E}; }
        if (methodName == "UNIFORM_BLOCK_BINDING") { return {0x8A3F}; }
        if (methodName == "UNIFORM_BLOCK_DATA_SIZE") { return {0x8A40}; }
        if (methodName == "UNIFORM_BLOCK_ACTIVE_UNIFORMS") { return {0x8A42}; }
        if (methodName == "UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES") { return {0x8A43}; }

        if (methodName == "UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER") { return {0x8A44}; }
        if (methodName == "UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER") { return {0x8A46}; }
        if (methodName == "OBJECT_TYPE") { return {0x9112}; }
        if (methodName == "SYNC_CONDITION") { return {0x9113}; }
        if (methodName == "SYNC_STATUS") { return {0x9114}; }

        if (methodName == "SYNC_FLAGS") { return {0x9115}; }
        if (methodName == "SYNC_FENCE") { return {0x9116}; }
        if (methodName == "SYNC_GPU_COMMANDS_COMPLETE") { return {0x9117}; }
        if (methodName == "UNSIGNALED") { return {0x9118}; }
        if (methodName == "SIGNALED") { return {0x9119}; }

        /* Samplers */

        /* Buffers */

        if (methodName == "ALREADY_SIGNALED") { return {0x911A}; }
        if (methodName == "TIMEOUT_EXPIRED") { return {0x911B}; }
        if (methodName == "CONDITION_SATISFIED") { return {0x911C}; }
        if (methodName == "WAIT_FAILED") { return {0x911D}; }
        if (methodName == "SYNC_FLUSH_COMMANDS_BIT") { return {0x00000001}; }

        if (methodName == "COLOR") { return {0x1800}; }
        if (methodName == "DEPTH") { return {0x1801}; }
        if (methodName == "STENCIL") { return {0x1802}; }

        /* Buffers */

        /* Data types */

        if (methodName == "MIN") { return {0x8007}; }
        if (methodName == "MAX") { return {0x8008}; }
        if (methodName == "DEPTH_COMPONENT24") { return {0x81A6}; }
        if (methodName == "STREAM_READ") { return {0x88E1}; }
        if (methodName == "STREAM_COPY") { return {0x88E2}; }

        if (methodName == "STATIC_READ") { return {0x88E5}; }
        if (methodName == "STATIC_COPY") { return {0x88E6}; }
        if (methodName == "DYNAMIC_READ") { return {0x88E9}; }
        if (methodName == "DYNAMIC_COPY") { return {0x88EA}; }
        if (methodName == "DEPTH_COMPONENT32F") { return {0x8CAC}; }
        if (methodName == "DEPTH32F_STENCIL8") { return {0x8CAD}; }

        /* Data types */

        if (methodName == "INVALID_INDEX") { return {(double) 0xFFFFFFFF}; }
        if (methodName == "TIMEOUT_IGNORED") { return {-1}; }

        /* Vertex attributes */

        /* Transform feedback */

        if (methodName == "MAX_CLIENT_WAIT_TIMEOUT_WEBGL") { return {0x9247}; }

        /* Transform feedback */

        return jsi::Value::undefined();
    }

};
