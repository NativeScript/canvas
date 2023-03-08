//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#pragma process_pending_includes

#include <vector>
#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"
#include "VecMutableBuffer.h"

#include "WebGLRenderingContextBase.h"

#include <cmath>
#include "Helpers.h"

#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "RafImpl.h"

#include "canvas2d/CanvasRenderingContext2DImpl.h"

#include "WebGLBuffer.h"
#include "WebGLFramebuffer.h"
#include "WebGLProgram.h"
#include "WebGLRenderbuffer.h"
#include "WebGLShader.h"
#include "WebGLTexture.h"
#include "WebGLUniformLocation.h"
#include "WebGLShaderPrecisionFormatImpl.h"
#include "WebGLActiveInfoImpl.h"

#include "extensions/EXT_blend_minmaxImpl.h"
#include "extensions/EXT_color_buffer_half_floatImpl.h"
#include "extensions/EXT_disjoint_timer_queryImpl.h"
#include "extensions/EXT_sRGBImpl.h"
#include "extensions/EXT_shader_texture_lodImpl.h"
#include "extensions/EXT_texture_filter_anisotropicImpl.h"
#include "extensions/OES_element_index_uintImpl.h"
#include "extensions/OES_standard_derivativesImpl.h"
#include "extensions/OES_texture_floatImpl.h"
#include "extensions/OES_texture_float_linearImpl.h"
#include "extensions/OES_texture_half_floatImpl.h"
#include "extensions/OES_texture_half_float_linearImpl.h"
#include "extensions/OES_vertex_array_objectImpl.h"
#include "extensions/WEBGL_color_buffer_floatImpl.h"
#include "extensions/WEBGL_compressed_texture_atcImpl.h"
#include "extensions/WEBGL_compressed_texture_etcImpl.h"
#include "extensions/WEBGL_compressed_texture_etc1Impl.h"
#include "extensions/WEBGL_compressed_texture_pvrtcImpl.h"
#include "extensions/WEBGL_compressed_texture_s3tcImpl.h"
#include "extensions/WEBGL_compressed_texture_s3tc_srgbImpl.h"
#include "extensions/WEBGL_depth_textureImpl.h"
#include "extensions/WEBGL_lose_contextImpl.h"
#include "extensions/ANGLE_instanced_arraysImpl.h"
#include "extensions/WEBGL_draw_buffersImpl.h"
#include "gl.h"

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT WebGLRenderingContext : public WebGLRenderingContextBase {
public:

    WebGLRenderingContext(rust::Box<WebGLState> state);

    WebGLRenderingContext(rust::Box<WebGLState> state, WebGLRenderingVersion version);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    jsi::Value GetParameterInternal(jsi::Runtime &runtime, uint32_t pnameValue,
                                    rust::Box<WebGLResult> result);

    static inline jsi::Value GetProperty(const std::string &methodName) {
        if (methodName == "DEPTH_BUFFER_BIT") { return {0x00000100}; }

        if (methodName == "STENCIL_BUFFER_BIT") { return {0x00000400}; }

        if (methodName == "COLOR_BUFFER_BIT") { return {0x00004000}; }

        if (methodName == "POINTS") { return {0x0000}; }

        if (methodName == "LINES") { return {0x0001}; }

        if (methodName == "LINE_LOOP") { return {0x0002}; }

        if (methodName == "LINE_STRIP") { return {0x0003}; }

        if (methodName == "TRIANGLES") { return {0x0004}; }

        if (methodName == "TRIANGLE_STRIP") { return {0x0005}; }

        if (methodName == "TRIANGLE_FAN") { return {0x0006}; }

        if (methodName == "ZERO") { return {0}; }

        if (methodName == "ONE") { return {1}; }

        if (methodName == "SRC_COLOR") { return {0x0300}; }

        if (methodName == "ONE_MINUS_SRC_COLOR") { return {0x0301}; }

        if (methodName == "SRC_ALPHA") { return {0x0302}; }

        if (methodName == "ONE_MINUS_SRC_ALPHA") { return {0x0303}; }

        if (methodName == "DST_ALPHA") { return {0x0304}; }

        if (methodName == "ONE_MINUS_DST_ALPHA") { return {0x0305}; }

        if (methodName == "DST_COLOR") { return {0x0306}; }

        if (methodName == "ONE_MINUS_DST_COLOR") { return {0x0307}; }

        if (methodName == "SRC_ALPHA_SATURATE") { return {0x0308}; }


        if (methodName == "CONSTANT_COLOR") { return {0x8001}; }
        if (methodName == "ONE_MINUS_CONSTANT_COLOR") { return {0x8002}; }
        if (methodName == "CONSTANT_ALPHA") { return {0x8003}; }
        if (methodName == "ONE_MINUS_CONSTANT_ALPHA") { return {0x8004}; }


        /* Blending equations */

        if (methodName == "FUNC_ADD") { return {0x8006}; }
        if (methodName == "FUNC_SUBTRACT") { return {0x800A}; }
        if (methodName == "FUNC_REVERSE_SUBTRACT") { return {0x800B}; }
        if (methodName == "BLEND_EQUATION") { return {0x8009}; }
        if (methodName == "BLEND_EQUATION_RGB") { return {0x8009}; }
        if (methodName == "BLEND_EQUATION_ALPHA") { return {0x883D}; }


        if (methodName == "BLEND_DST_RGB") { return {0x80C8}; }
        if (methodName == "BLEND_SRC_RGB") { return {0x80C9}; }
        if (methodName == "BLEND_DST_ALPHA") { return {0x80CA}; }


        if (methodName == "BLEND_SRC_ALPHA") { return {0x80CB}; }
        if (methodName == "BLEND_COLOR") { return {0x8005}; }
        if (methodName == "ARRAY_BUFFER_BINDING") { return {0x8894}; }


        if (methodName == "ELEMENT_ARRAY_BUFFER_BINDING") { return {0x8895}; }
        if (methodName == "LINE_WIDTH") { return {0x0B21}; }
        if (methodName == "ALIASED_POINT_SIZE_RANGE") { return {0x846D}; }

        if (methodName == "ALIASED_LINE_WIDTH_RANGE") { return {0x846E}; }
        if (methodName == "CULL_FACE_MODE") { return {0x0B45}; }
        if (methodName == "FRONT_FACE") { return {0x0B46}; }

        if (methodName == "DEPTH_RANGE") { return {0x0B70}; }
        if (methodName == "DEPTH_WRITEMASK") { return {0x0B72}; }
        if (methodName == "DEPTH_CLEAR_VALUE") { return {0x0B73}; }

        if (methodName == "DEPTH_FUNC") { return {0x0B74}; }
        if (methodName == "STENCIL_CLEAR_VALUE") { return {0x0B91}; }
        if (methodName == "STENCIL_FUNC") { return {0x0B92}; }

        if (methodName == "STENCIL_FAIL") { return {0x0B94}; }
        if (methodName == "STENCIL_PASS_DEPTH_FAIL") { return {0x0B95}; }
        if (methodName == "STENCIL_PASS_DEPTH_PASS") { return {0x0B96}; }

        if (methodName == "STENCIL_REF") { return {0x0B97}; }
        if (methodName == "STENCIL_VALUE_MASK") { return {0x0B93}; }
        if (methodName == "STENCIL_WRITEMASK") { return {0x0B98}; }

        if (methodName == "STENCIL_BACK_FUNC") { return {0x8800}; }
        if (methodName == "STENCIL_BACK_FAIL") { return {0x8801}; }
        if (methodName == "STENCIL_BACK_PASS_DEPTH_FAIL") { return {0x8802}; }

        if (methodName == "STENCIL_BACK_PASS_DEPTH_PASS") { return {0x8803}; }
        if (methodName == "STENCIL_BACK_REF") { return {0x8CA3}; }
        if (methodName == "STENCIL_BACK_VALUE_MASK") { return {0x8CA4}; }
        if (methodName == "STENCIL_BACK_WRITEMASK") { return {0x8CA5}; }

        if (methodName == "VIEWPORT") { return {0x0BA2}; }
        if (methodName == "SCISSOR_BOX") { return {0x0C10}; }
        if (methodName == "COLOR_CLEAR_VALUE") { return {0x0C22}; }
        if (methodName == "COLOR_WRITEMASK") { return {0x0C23}; }

        if (methodName == "UNPACK_ALIGNMENT") { return {0x0CF5}; }
        if (methodName == "PACK_ALIGNMENT") { return {0x0D05}; }
        if (methodName == "MAX_TEXTURE_SIZE") { return {0x0D33}; }


        if (methodName == "MAX_VIEWPORT_DIMS") { return {0x0D3A}; }
        if (methodName == "SUBPIXEL_BITS") { return {0x0D50}; }

        if (methodName == "RED_BITS") { return {0x0D52}; }
        if (methodName == "GREEN_BITS") { return {0x0D53}; }
        if (methodName == "BLUE_BITS") { return {0x0D54}; }
        if (methodName == "ALPHA_BITS") { return {0x0D55}; }

        if (methodName == "STENCIL_BITS") { return {0x0D57}; }
        if (methodName == "POLYGON_OFFSET_UNITS") { return {0x2A00}; }
        if (methodName == "POLYGON_OFFSET_FACTOR") { return {0x8038}; }

        if (methodName == "TEXTURE_BINDING_2D") { return {0x8069}; }
        if (methodName == "SAMPLE_BUFFERS") { return {0x80A8}; }
        if (methodName == "SAMPLES") { return {0x80A9}; }
        if (methodName == "SAMPLE_COVERAGE_VALUE") { return {0x80AA}; }

        if (methodName == "SAMPLE_COVERAGE_INVERT") { return {0x80AB}; }
        if (methodName == "COMPRESSED_TEXTURE_FORMATS") { return {0x86A3}; }
        if (methodName == "VENDOR") { return {0x1F00}; }
        if (methodName == "RENDERER") { return {0x1F01}; }

        if (methodName == "VERSION") { return {0x1F02}; }
        if (methodName == "IMPLEMENTATION_COLOR_READ_TYPE") { return {0x8B9A}; }
        if (methodName == "IMPLEMENTATION_COLOR_READ_FORMAT") { return {0x8B9B}; }
        if (methodName == "BROWSER_DEFAULT_WEBGL") { return {0x9244}; }

        if (methodName == "STATIC_DRAW") { return {0x88E4}; }
        if (methodName == "STREAM_DRAW") { return {0x88E0}; }
        if (methodName == "DYNAMIC_DRAW") { return {0x88E8}; }
        if (methodName == "ARRAY_BUFFER") { return {0x8892}; }

        if (methodName == "ELEMENT_ARRAY_BUFFER") { return {0x8893}; }
        if (methodName == "BUFFER_SIZE") { return {0x8764}; }
        if (methodName == "BUFFER_USAGE") { return {0x8765}; }
        if (methodName == "CURRENT_VERTEX_ATTRIB") { return {0x8626}; }


        if (methodName == "VERTEX_ATTRIB_ARRAY_ENABLED") { return {0x8622}; }
        if (methodName == "VERTEX_ATTRIB_ARRAY_SIZE") { return {0x8623}; }
        if (methodName == "VERTEX_ATTRIB_ARRAY_STRIDE") { return {0x8624}; }
        if (methodName == "VERTEX_ATTRIB_ARRAY_TYPE") { return {0x8625}; }
        if (methodName == "VERTEX_ATTRIB_ARRAY_NORMALIZED") { return {0x886A}; }
        if (methodName == "VERTEX_ATTRIB_ARRAY_POINTER") { return {0x8645}; }
        if (methodName == "VERTEX_ATTRIB_ARRAY_BUFFER_BINDING") { return {0x889F}; }

        if (methodName == "CULL_FACE") { return {0x0B44}; }
        if (methodName == "FRONT") { return {0x0404}; }
        if (methodName == "BACK") { return {0x0405}; }
        if (methodName == "FRONT_AND_BACK") { return {0x0408}; }

        if (methodName == "BLEND") { return {0x0BE2}; }
        if (methodName == "DEPTH_TEST") { return {0x0B71}; }
        if (methodName == "DITHER") { return {0x0BD0}; }
        if (methodName == "POLYGON_OFFSET_FILL") { return {0x8037}; }

        if (methodName == "SAMPLE_ALPHA_TO_COVERAGE") { return {0x809E}; }
        if (methodName == "SAMPLE_COVERAGE") { return {0x80A0}; }
        if (methodName == "SCISSOR_TEST") { return {0x0C11}; }
        if (methodName == "STENCIL_TEST") { return {0x0B90}; }


        /* Errors */

        if (methodName == "NO_ERROR") { return {0}; }
        if (methodName == "INVALID_ENUM") { return {0x0500}; }
        if (methodName == "INVALID_VALUE") { return {0x0501}; }
        if (methodName == "INVALID_OPERATION") { return {0x0502}; }

        if (methodName == "OUT_OF_MEMORY") { return {0x0505}; }
        if (methodName == "CONTEXT_LOST_WEBGL") { return {0x9242}; }
        if (methodName == "CW") { return {0x0900}; }
        if (methodName == "CCW") { return {0x0901}; }

        if (methodName == "DONT_CARE") { return {0x1100}; }
        if (methodName == "FASTEST") { return {0x1101}; }
        if (methodName == "NICEST") { return {0x1102}; }
        if (methodName == "GENERATE_MIPMAP_HINT") { return {0x8192}; }

        if (methodName == "BYTE") { return {0x1400}; }
        if (methodName == "UNSIGNED_BYTE") { return {0x1401}; }
        if (methodName == "SHORT") { return {0x1402}; }
        if (methodName == "UNSIGNED_SHORT") { return {0x1403}; }

        if (methodName == "INT") { return {0x1404}; }
        if (methodName == "UNSIGNED_INT") { return {0x1405}; }
        if (methodName == "FLOAT") { return {0x1406}; }
        if (methodName == "DEPTH_COMPONENT") { return {0x1902}; }

        if (methodName == "ALPHA") { return {0x1906}; }
        if (methodName == "RGB") { return {0x1907}; }

        /* Clearing buffers */

        if (methodName == "RGBA") { return {0x1908}; }
        if (methodName == "LUMINANCE") { return {0x1909}; }
        if (methodName == "LUMINANCE_ALPHA") { return {0x190A}; }

        /* Clearing buffers */

        /* Rendering primitives */

        if (methodName == "UNSIGNED_SHORT_4_4_4_4") { return {0x8033}; }
        if (methodName == "UNSIGNED_SHORT_5_5_5_1") { return {0x8034}; }
        if (methodName == "UNSIGNED_SHORT_5_6_5") { return {0x8363}; }
        if (methodName == "FRAGMENT_SHADER") { return {0x8B30}; }
        if (methodName == "VERTEX_SHADER") { return {0x8B31}; }
        if (methodName == "COMPILE_STATUS") { return {0x8B81}; }
        if (methodName == "DELETE_STATUS") { return {0x8B80}; }

        /* Rendering primitives */

        /* Blending modes */

        if (methodName == "LINK_STATUS") { return {0x8B82}; }
        if (methodName == "VALIDATE_STATUS") { return {0x8B83}; }
        if (methodName == "ATTACHED_SHADERS") { return {0x8B85}; }
        if (methodName == "ACTIVE_ATTRIBUTES") { return {0x8B89}; }
        if (methodName == "ACTIVE_UNIFORMS") { return {0x8B86}; }
        if (methodName == "MAX_VERTEX_ATTRIBS") { return {0x8869}; }
        if (methodName == "MAX_VERTEX_UNIFORM_VECTORS") { return {0x8DFB}; }
        if (methodName == "MAX_VARYING_VECTORS") { return {0x8DFC}; }
        if (methodName == "MAX_COMBINED_TEXTURE_IMAGE_UNITS") { return {0x8B4D}; }
        if (methodName == "MAX_VERTEX_TEXTURE_IMAGE_UNITS") { return {0x8B4C}; }
        if (methodName == "MAX_TEXTURE_IMAGE_UNITS") { return {0x8872}; }
        if (methodName == "MAX_FRAGMENT_UNIFORM_VECTORS") { return {0x8DFD}; }
        if (methodName == "SHADER_TYPE") { return {0x8B4F}; }
        if (methodName == "SHADING_LANGUAGE_VERSION") { return {0x8B8C}; }
        if (methodName == "CURRENT_PROGRAM") { return {0x8B8D}; }

        /* Blending modes */

        if (methodName == "NEVER") { return {0x0200}; }
        if (methodName == "LESS") { return {0x0201}; }
        if (methodName == "EQUAL") { return {0x0202}; }

        /* Blending equations */

        /* Getting GL parameter information */

        if (methodName == "LEQUAL") { return {0x0203}; }
        if (methodName == "GREATER") { return {0x0204}; }
        if (methodName == "NOTEQUAL") { return {0x0205}; }
        if (methodName == "GEQUAL") { return {0x0206}; }
        if (methodName == "ALWAYS") { return {0x0207}; }
        if (methodName == "KEEP") { return {0x1E00}; }
        if (methodName == "REPLACE") { return {0x1E01}; }
        if (methodName == "INCR") { return {0x1E02}; }
        if (methodName == "DECR") { return {0x1E03}; }
        if (methodName == "INVERT") { return {0x150A}; }
        if (methodName == "INCR_WRAP") { return {0x8507}; }
        if (methodName == "DECR_WRAP") { return {0x8508}; }
        if (methodName == "NEAREST") { return {0x2600}; }
        if (methodName == "LINEAR") { return {0x2601}; }
        if (methodName == "NEAREST_MIPMAP_NEAREST") { return {0x2700}; }
        if (methodName == "LINEAR_MIPMAP_NEAREST") { return {0x2701}; }
        if (methodName == "NEAREST_MIPMAP_LINEAR") { return {0x2702}; }
        if (methodName == "LINEAR_MIPMAP_LINEAR") { return {0x2703}; }
        if (methodName == "TEXTURE_MAG_FILTER") { return {0x2800}; }
        if (methodName == "TEXTURE_MIN_FILTER") { return {0x2801}; }
        if (methodName == "TEXTURE_WRAP_S") { return {0x2802}; }
        if (methodName == "TEXTURE_WRAP_T") { return {0x2803}; }
        if (methodName == "TEXTURE_2D") { return {0x0DE1}; }
        if (methodName == "TEXTURE") { return {0x1702}; }
        if (methodName == "TEXTURE_CUBE_MAP") { return {0x8513}; }
        if (methodName == "TEXTURE_BINDING_CUBE_MAP") { return {0x8514}; }
        if (methodName == "TEXTURE_CUBE_MAP_POSITIVE_X") { return {0x8515}; }
        if (methodName == "TEXTURE_CUBE_MAP_NEGATIVE_X") { return {0x8516}; }
        if (methodName == "TEXTURE_CUBE_MAP_POSITIVE_Y") { return {0x8517}; }
        if (methodName == "TEXTURE_CUBE_MAP_NEGATIVE_Y") { return {0x8518}; }
        if (methodName == "TEXTURE_CUBE_MAP_POSITIVE_Z") { return {0x8519}; }
        if (methodName == "TEXTURE_CUBE_MAP_NEGATIVE_Z") { return {0x851A}; }
        if (methodName == "MAX_CUBE_MAP_TEXTURE_SIZE") { return {0x851C}; }
        if (methodName == "TEXTURE0") { return {0x84C0}; }
        if (methodName == "TEXTURE1") { return {0x84C1}; }
        if (methodName == "TEXTURE2") { return {0x84C2}; }
        if (methodName == "TEXTURE3") { return {0x84C3}; }
        if (methodName == "TEXTURE4") { return {0x84C4}; }
        if (methodName == "TEXTURE5") { return {0x84C5}; }
        if (methodName == "TEXTURE6") { return {0x84C6}; }
        if (methodName == "TEXTURE7") { return {0x84C7}; }
        if (methodName == "TEXTURE8") { return {0x84C8}; }
        if (methodName == "TEXTURE9") { return {0x84C9}; }
        if (methodName == "TEXTURE10") { return {0x84CA}; }
        if (methodName == "TEXTURE11") { return {0x84CB}; }
        if (methodName == "TEXTURE12") { return {0x84CC}; }
        if (methodName == "TEXTURE13") { return {0x84CD}; }
        if (methodName == "TEXTURE14") { return {0x84CE}; }
        if (methodName == "TEXTURE15") { return {0x84CF}; }
        if (methodName == "TEXTURE16") { return {0x84D0}; }
        if (methodName == "TEXTURE17") { return {0x84D1}; }
        if (methodName == "TEXTURE18") { return {0x84D2}; }
        if (methodName == "TEXTURE19") { return {0x84D3}; }
        if (methodName == "TEXTURE20") { return {0x84D4}; }
        if (methodName == "TEXTURE21") { return {0x84D5}; }
        if (methodName == "TEXTURE22") { return {0x84D6}; }
        if (methodName == "TEXTURE23") { return {0x84D7}; }
        if (methodName == "TEXTURE24") { return {0x84D8}; }
        if (methodName == "TEXTURE25") { return {0x84D9}; }
        if (methodName == "TEXTURE26") { return {0x84DA}; }
        if (methodName == "TEXTURE27") { return {0x84DB}; }
        if (methodName == "TEXTURE28") { return {0x84DC}; }
        if (methodName == "TEXTURE29") { return {0x84DD}; }

        /* Getting GL parameter information */

        /* Buffers */

        if (methodName == "TEXTURE30") { return {0x84DE}; }
        if (methodName == "TEXTURE31") { return {0x84DF}; }
        if (methodName == "ACTIVE_TEXTURE") { return {0x84E0}; }
        if (methodName == "REPEAT") { return {0x2901}; }
        if (methodName == "CLAMP_TO_EDGE") { return {0x812F}; }
        if (methodName == "MIRRORED_REPEAT") { return {0x8370}; }
        if (methodName == "FLOAT_VEC2") { return {0x8B50}; }

        /* Buffers */

        /* Vertex attributes */

        if (methodName == "FLOAT_VEC3") { return {0x8B51}; }
        if (methodName == "FLOAT_VEC4") { return {0x8B52}; }
        if (methodName == "INT_VEC2") { return {0x8B53}; }
        if (methodName == "INT_VEC3") { return {0x8B54}; }
        if (methodName == "INT_VEC4") { return {0x8B55}; }
        if (methodName == "BOOL") { return {0x8B56}; }
        if (methodName == "BOOL_VEC2") { return {0x8B57}; }
        if (methodName == "BOOL_VEC3") { return {0x8B58}; }

        /* Vertex attributes */

        /* Culling */

        if (methodName == "BOOL_VEC4") { return {0x8B59}; }
        if (methodName == "FLOAT_MAT2") { return {0x8B5A}; }
        if (methodName == "FLOAT_MAT3") { return {0x8B5B}; }
        if (methodName == "FLOAT_MAT4") { return {0x8B5C}; }

        /* Culling */

        /* Enabling and disabling */

        if (methodName == "SAMPLER_2D") { return {0x8B5E}; }
        if (methodName == "SAMPLER_CUBE") { return {0x8B60}; }
        if (methodName == "LOW_FLOAT") { return {0x8DF0}; }
        if (methodName == "MEDIUM_FLOAT") { return {0x8DF1}; }
        if (methodName == "HIGH_FLOAT") { return {0x8DF2}; }
        if (methodName == "LOW_INT") { return {0x8DF3}; }
        if (methodName == "MEDIUM_INT") { return {0x8DF4}; }
        if (methodName == "HIGH_INT") { return {0x8DF5}; }

        /* Enabling and disabling */

        if (methodName == "FRAMEBUFFER") { return {0x8D40}; }
        if (methodName == "RENDERBUFFER") { return {0x8D41}; }
        if (methodName == "RGBA4") { return {0x8056}; }
        if (methodName == "RGB5_A1") { return {0x8057}; }
        if (methodName == "RGB565") { return {0x8D62}; }
        if (methodName == "DEPTH_COMPONENT16") { return {0x81A5}; }
        if (methodName == "STENCIL_INDEX8") { return {0x8D48}; }

        /* Errors */

        /* Front face directions */

        if (methodName == "DEPTH_STENCIL") { return {0x84F9}; }
        if (methodName == "RENDERBUFFER_WIDTH") { return {0x8D42}; }

        /* Front face directions */

        /* Hints */

        if (methodName == "RENDERBUFFER_HEIGHT") { return {0x8D43}; }
        if (methodName == "RENDERBUFFER_INTERNAL_FORMAT") { return {0x8D44}; }
        if (methodName == "RENDERBUFFER_RED_SIZE") { return {0x8D50}; }
        if (methodName == "RENDERBUFFER_GREEN_SIZE") { return {0x8D51}; }

        /* Hints */

        /* Data types */

        if (methodName == "RENDERBUFFER_BLUE_SIZE") { return {0x8D52}; }
        if (methodName == "RENDERBUFFER_ALPHA_SIZE") { return {0x8D53}; }
        if (methodName == "RENDERBUFFER_DEPTH_SIZE") { return {0x8D54}; }
        if (methodName == "RENDERBUFFER_STENCIL_SIZE") { return {0x8D55}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE") { return {0x8CD0}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_OBJECT_NAME") { return {0x8CD1}; }
        if (methodName == "FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL") { return {0x8CD2}; }

        /* Data types */

        /* Pixel formats */

        if (methodName == "FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE") { return {0x8CD3}; }
        if (methodName == "COLOR_ATTACHMENT0") { return {0x8CE0}; }
        if (methodName == "DEPTH_ATTACHMENT") { return {0x8D00}; }
        if (methodName == "STENCIL_ATTACHMENT") { return {0x8D20}; }
        if (methodName == "DEPTH_STENCIL_ATTACHMENT") { return {0x821A}; }
        if (methodName == "NONE") { return {}; }

        /* Pixel formats */

        /* Pixel types */

        if (methodName == "FRAMEBUFFER_COMPLETE") { return {0x8CD5}; }
        if (methodName == "FRAMEBUFFER_INCOMPLETE_ATTACHMENT") { return {0x8CD6}; }
        if (methodName == "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT") { return {0x8CD7}; }

        /* Pixel types */

        /* Shaders */

        if (methodName == "FRAMEBUFFER_INCOMPLETE_DIMENSIONS") { return {0x8CD9}; }
        if (methodName == "FRAMEBUFFER_UNSUPPORTED") { return {0x8CDD}; }
        if (methodName == "FRAMEBUFFER_BINDING") { return {0x8CA6}; }
        if (methodName == "RENDERBUFFER_BINDING") { return {0x8CA7}; }
        if (methodName == "MAX_RENDERBUFFER_SIZE") { return {0x84E8}; }
        if (methodName == "INVALID_FRAMEBUFFER_OPERATION") { return {0x0506}; }
        if (methodName == "UNPACK_FLIP_Y_WEBGL") { return {0x9240}; }
        if (methodName == "UNPACK_PREMULTIPLY_ALPHA_WEBGL") { return {0x9241}; }
        if (methodName == "UNPACK_COLORSPACE_CONVERSION_WEBGL") { return {0x9243}; }

        return jsi::Value::undefined();
    }

};
