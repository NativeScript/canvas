#[macro_export]
macro_rules! impl_webgl_context_constants {
  ($struct_name:ident) => {
    #[napi]
    impl $struct_name {
      #[napi(getter, js_name = "DEPTH_BUFFER_BIT")]
      pub fn DEPTH_BUFFER_BIT(&self) -> u32 {
        0x00000100
      }

      #[napi(getter, js_name = "STENCIL_BUFFER_BIT")]
      pub fn STENCIL_BUFFER_BIT(&self) -> u32 {
        0x00000400
      }

      #[napi(getter, js_name = "COLOR_BUFFER_BIT")]
      pub fn COLOR_BUFFER_BIT(&self) -> u32 {
        0x00004000
      }

      #[napi(getter, js_name = "POINTS")]
      pub fn POINTS(&self) -> u32 {
        0x0000
      }

      #[napi(getter, js_name = "LINES")]
      pub fn LINES(&self) -> u32 {
        0x0001
      }

      #[napi(getter, js_name = "LINE_LOOP")]
      pub fn LINE_LOOP(&self) -> u32 {
        0x0002
      }

      #[napi(getter, js_name = "LINE_STRIP")]
      pub fn LINE_STRIP(&self) -> u32 {
        0x0003
      }

      #[napi(getter, js_name = "TRIANGLES")]
      pub fn TRIANGLES(&self) -> u32 {
        0x0004
      }

      #[napi(getter, js_name = "TRIANGLE_STRIP")]
      pub fn TRIANGLE_STRIP(&self) -> u32 {
        0x0005
      }

      #[napi(getter, js_name = "TRIANGLE_FAN")]
      pub fn TRIANGLE_FAN(&self) -> u32 {
        0x0006
      }

      #[napi(getter, js_name = "ZERO")]
      pub fn ZERO(&self) -> u32 {
        0
      }

      #[napi(getter, js_name = "ONE")]
      pub fn ONE(&self) -> u32 {
        1
      }

      #[napi(getter, js_name = "SRC_COLOR")]
      pub fn SRC_COLOR(&self) -> u32 {
        0x0300
      }

      #[napi(getter, js_name = "ONE_MINUS_SRC_COLOR")]
      pub fn ONE_MINUS_SRC_COLOR(&self) -> u32 {
        0x0301
      }

      #[napi(getter, js_name = "SRC_ALPHA")]
      pub fn SRC_ALPHA(&self) -> u32 {
        0x0302
      }

      #[napi(getter, js_name = "ONE_MINUS_SRC_ALPHA")]
      pub fn ONE_MINUS_SRC_ALPHA(&self) -> u32 {
        0x0303
      }

      #[napi(getter, js_name = "DST_ALPHA")]
      pub fn DST_ALPHA(&self) -> u32 {
        0x0304
      }

      #[napi(getter, js_name = "ONE_MINUS_DST_ALPHA")]
      pub fn ONE_MINUS_DST_ALPHA(&self) -> u32 {
        0x0305
      }

      #[napi(getter, js_name = "DST_COLOR")]
      pub fn DST_COLOR(&self) -> u32 {
        0x0306
      }

      #[napi(getter, js_name = "ONE_MINUS_DST_COLOR")]
      pub fn ONE_MINUS_DST_COLOR(&self) -> u32 {
        0x0307
      }

      #[napi(getter, js_name = "SRC_ALPHA_SATURATE")]
      pub fn SRC_ALPHA_SATURATE(&self) -> u32 {
        0x0308
      }

      #[napi(getter, js_name = "CONSTANT_COLOR")]
      pub fn CONSTANT_COLOR(&self) -> u32 {
        0x8001
      }

      #[napi(getter, js_name = "ONE_MINUS_CONSTANT_COLOR")]
      pub fn ONE_MINUS_CONSTANT_COLOR(&self) -> u32 {
        0x8002
      }

      #[napi(getter, js_name = "CONSTANT_ALPHA")]
      pub fn CONSTANT_ALPHA(&self) -> u32 {
        0x8003
      }

      #[napi(getter, js_name = "ONE_MINUS_CONSTANT_ALPHA")]
      pub fn ONE_MINUS_CONSTANT_ALPHA(&self) -> u32 {
        0x8004
      }

      /* Blending equations */
      #[napi(getter, js_name = "FUNC_ADD")]
      pub fn FUNC_ADD(&self) -> u32 {
        0x8006
      }

      #[napi(getter, js_name = "FUNC_SUBTRACT")]
      pub fn FUNC_SUBTRACT(&self) -> u32 {
        0x800A
      }

      #[napi(getter, js_name = "FUNC_REVERSE_SUBTRACT")]
      pub fn FUNC_REVERSE_SUBTRACT(&self) -> u32 {
        0x800B
      }

      #[napi(getter, js_name = "BLEND_EQUATION")]
      pub fn BLEND_EQUATION(&self) -> u32 {
        0x8009
      }

      #[napi(getter, js_name = "BLEND_EQUATION_RGB")]
      pub fn BLEND_EQUATION_RGB(&self) -> u32 {
        0x8009
      }

      #[napi(getter, js_name = "BLEND_EQUATION_ALPHA")]
      pub fn BLEND_EQUATION_ALPHA(&self) -> u32 {
        0x883D
      }

      #[napi(getter, js_name = "BLEND_DST_RGB")]
      pub fn BLEND_DST_RGB(&self) -> u32 {
        0x80C8
      }

      #[napi(getter, js_name = "BLEND_SRC_RGB")]
      pub fn BLEND_SRC_RGB(&self) -> u32 {
        0x80C9
      }

      #[napi(getter, js_name = "BLEND_DST_ALPHA")]
      pub fn BLEND_DST_ALPHA(&self) -> u32 {
        0x80CA
      }

      #[napi(getter, js_name = "BLEND_SRC_ALPHA")]
      pub fn BLEND_SRC_ALPHA(&self) -> u32 {
        0x80CB
      }

      #[napi(getter, js_name = "BLEND_COLOR")]
      pub fn BLEND_COLOR(&self) -> u32 {
        0x8005
      }

      #[napi(getter, js_name = "ARRAY_BUFFER_BINDING")]
      pub fn ARRAY_BUFFER_BINDING(&self) -> u32 {
        0x8894
      }

      #[napi(getter, js_name = "ELEMENT_ARRAY_BUFFER_BINDING")]
      pub fn ELEMENT_ARRAY_BUFFER_BINDING(&self) -> u32 {
        0x8895
      }

      #[napi(getter, js_name = "LINE_WIDTH")]
      pub fn LINE_WIDTH(&self) -> u32 {
        0x0B21
      }

      #[napi(getter, js_name = "ALIASED_POINT_SIZE_RANGE")]
      pub fn ALIASED_POINT_SIZE_RANGE(&self) -> u32 {
        0x846D
      }

      #[napi(getter, js_name = "ALIASED_LINE_WIDTH_RANGE")]
      pub fn ALIASED_LINE_WIDTH_RANGE(&self) -> u32 {
        0x846E
      }

      #[napi(getter, js_name = "CULL_FACE_MODE")]
      pub fn CULL_FACE_MODE(&self) -> u32 {
        0x0B45
      }

      #[napi(getter, js_name = "FRONT_FACE")]
      pub fn FRONT_FACE(&self) -> u32 {
        0x0B46
      }

      #[napi(getter, js_name = "DEPTH_RANGE")]
      pub fn DEPTH_RANGE(&self) -> u32 {
        0x0B70
      }

      #[napi(getter, js_name = "DEPTH_WRITEMASK")]
      pub fn DEPTH_WRITEMASK(&self) -> u32 {
        0x0B72
      }

      #[napi(getter, js_name = "DEPTH_CLEAR_VALUE")]
      pub fn DEPTH_CLEAR_VALUE(&self) -> u32 {
        0x0B73
      }

      #[napi(getter, js_name = "DEPTH_FUNC")]
      pub fn DEPTH_FUNC(&self) -> u32 {
        0x0B74
      }

      #[napi(getter, js_name = "STENCIL_CLEAR_VALUE")]
      pub fn STENCIL_CLEAR_VALUE(&self) -> u32 {
        0x0B91
      }

      #[napi(getter, js_name = "STENCIL_FUNC")]
      pub fn STENCIL_FUNC(&self) -> u32 {
        0x0B92
      }

      #[napi(getter, js_name = "STENCIL_FAIL")]
      pub fn STENCIL_FAIL(&self) -> u32 {
        0x0B94
      }

      #[napi(getter, js_name = "STENCIL_PASS_DEPTH_FAIL")]
      pub fn STENCIL_PASS_DEPTH_FAIL(&self) -> u32 {
        0x0B95
      }

      #[napi(getter, js_name = "STENCIL_PASS_DEPTH_PASS")]
      pub fn STENCIL_PASS_DEPTH_PASS(&self) -> u32 {
        0x0B96
      }

      #[napi(getter, js_name = "STENCIL_REF")]
      pub fn STENCIL_REF(&self) -> u32 {
        0x0B97
      }

      #[napi(getter, js_name = "STENCIL_VALUE_MASK")]
      pub fn STENCIL_VALUE_MASK(&self) -> u32 {
        0x0B93
      }

      #[napi(getter, js_name = "STENCIL_WRITEMASK")]
      pub fn STENCIL_WRITEMASK(&self) -> u32 {
        0x0B98
      }

      #[napi(getter, js_name = "STENCIL_BACK_FUNC")]
      pub fn STENCIL_BACK_FUNC(&self) -> u32 {
        0x8800
      }

      #[napi(getter, js_name = "STENCIL_BACK_FAIL")]
      pub fn STENCIL_BACK_FAIL(&self) -> u32 {
        0x8801
      }

      #[napi(getter, js_name = "STENCIL_BACK_PASS_DEPTH_FAIL")]
      pub fn STENCIL_BACK_PASS_DEPTH_FAIL(&self) -> u32 {
        0x8802
      }

      #[napi(getter, js_name = "STENCIL_BACK_PASS_DEPTH_PASS")]
      pub fn STENCIL_BACK_PASS_DEPTH_PASS(&self) -> u32 {
        0x8803
      }

      #[napi(getter, js_name = "STENCIL_BACK_REF")]
      pub fn STENCIL_BACK_REF(&self) -> u32 {
        0x8CA3
      }

      #[napi(getter, js_name = "STENCIL_BACK_VALUE_MASK")]
      pub fn STENCIL_BACK_VALUE_MASK(&self) -> u32 {
        0x8CA4
      }

      #[napi(getter, js_name = "STENCIL_BACK_WRITEMASK")]
      pub fn STENCIL_BACK_WRITEMASK(&self) -> u32 {
        0x8CA5
      }

      #[napi(getter, js_name = "VIEWPORT")]
      pub fn VIEWPORT(&self) -> u32 {
        0x0BA2
      }

      #[napi(getter, js_name = "SCISSOR_BOX")]
      pub fn SCISSOR_BOX(&self) -> u32 {
        0x0C10
      }

      #[napi(getter, js_name = "COLOR_CLEAR_VALUE")]
      pub fn COLOR_CLEAR_VALUE(&self) -> u32 {
        0x0C22
      }

      #[napi(getter, js_name = "COLOR_WRITEMASK")]
      pub fn COLOR_WRITEMASK(&self) -> u32 {
        0x0C23
      }

      #[napi(getter, js_name = "UNPACK_ALIGNMENT")]
      pub fn UNPACK_ALIGNMENT(&self) -> u32 {
        0x0CF5
      }

      #[napi(getter, js_name = "PACK_ALIGNMENT")]
      pub fn PACK_ALIGNMENT(&self) -> u32 {
        0x0D05
      }

      #[napi(getter, js_name = "MAX_TEXTURE_SIZE")]
      pub fn MAX_TEXTURE_SIZE(&self) -> u32 {
        0x0D33
      }

      #[napi(getter, js_name = "MAX_VIEWPORT_DIMS")]
      pub fn MAX_VIEWPORT_DIMS(&self) -> u32 {
        0x0D3A
      }

      #[napi(getter, js_name = "SUBPIXEL_BITS")]
      pub fn SUBPIXEL_BITS(&self) -> u32 {
        0x0D50
      }

      #[napi(getter, js_name = "RED_BITS")]
      pub fn RED_BITS(&self) -> u32 {
        0x0D52
      }

      #[napi(getter, js_name = "GREEN_BITS")]
      pub fn GREEN_BITS(&self) -> u32 {
        0x0D53
      }

      #[napi(getter, js_name = "BLUE_BITS")]
      pub fn BLUE_BITS(&self) -> u32 {
        0x0D54
      }

      #[napi(getter, js_name = "ALPHA_BITS")]
      pub fn ALPHA_BITS(&self) -> u32 {
        0x0D55
      }

      #[napi(getter, js_name = "DEPTH_BITS")]
      pub fn DEPTH_BITS(&self) -> u32 {
        0x0D56
      }

      #[napi(getter, js_name = "STENCIL_BITS")]
      pub fn STENCIL_BITS(&self) -> u32 {
        0x0D57
      }

      #[napi(getter, js_name = "POLYGON_OFFSET_UNITS")]
      pub fn POLYGON_OFFSET_UNITS(&self) -> u32 {
        0x2A00
      }

      #[napi(getter, js_name = "POLYGON_OFFSET_FACTOR")]
      pub fn POLYGON_OFFSET_FACTOR(&self) -> u32 {
        0x8038
      }

      #[napi(getter, js_name = "TEXTURE_BINDING_2D")]
      pub fn TEXTURE_BINDING_2D(&self) -> u32 {
        0x8069
      }

      #[napi(getter, js_name = "SAMPLE_BUFFERS")]
      pub fn SAMPLE_BUFFERS(&self) -> u32 {
        0x80A8
      }

      #[napi(getter, js_name = "SAMPLES")]
      pub fn SAMPLES(&self) -> u32 {
        0x80A9
      }

      #[napi(getter, js_name = "SAMPLE_COVERAGE_VALUE")]
      pub fn SAMPLE_COVERAGE_VALUE(&self) -> u32 {
        0x80AA
      }

      #[napi(getter, js_name = "SAMPLE_COVERAGE_INVERT")]
      pub fn SAMPLE_COVERAGE_INVERT(&self) -> u32 {
        0x80AB
      }

      #[napi(getter, js_name = "COMPRESSED_TEXTURE_FORMATS")]
      pub fn COMPRESSED_TEXTURE_FORMATS(&self) -> u32 {
        0x86A3
      }

      #[napi(getter, js_name = "VENDOR")]
      pub fn VENDOR(&self) -> u32 {
        0x1F00
      }

      #[napi(getter, js_name = "RENDERER")]
      pub fn RENDERER(&self) -> u32 {
        0x1F01
      }

      #[napi(getter, js_name = "VERSION")]
      pub fn VERSION(&self) -> u32 {
        0x1F02
      }

      #[napi(getter, js_name = "IMPLEMENTATION_COLOR_READ_TYPE")]
      pub fn IMPLEMENTATION_COLOR_READ_TYPE(&self) -> u32 {
        0x8B9A
      }

      #[napi(getter, js_name = "IMPLEMENTATION_COLOR_READ_FORMAT")]
      pub fn IMPLEMENTATION_COLOR_READ_FORMAT(&self) -> u32 {
        0x8B9B
      }

      #[napi(getter, js_name = "BROWSER_DEFAULT_WEBGL")]
      pub fn BROWSER_DEFAULT_WEBGL(&self) -> u32 {
        0x9244
      }

      #[napi(getter, js_name = "STATIC_DRAW")]
      pub fn STATIC_DRAW(&self) -> u32 {
        0x88E4
      }

      #[napi(getter, js_name = "STREAM_DRAW")]
      pub fn STREAM_DRAW(&self) -> u32 {
        0x88E0
      }

      #[napi(getter, js_name = "DYNAMIC_DRAW")]
      pub fn DYNAMIC_DRAW(&self) -> u32 {
        0x88E8
      }

      #[napi(getter, js_name = "ARRAY_BUFFER")]
      pub fn ARRAY_BUFFER(&self) -> u32 {
        0x8892
      }

      #[napi(getter, js_name = "ELEMENT_ARRAY_BUFFER")]
      pub fn ELEMENT_ARRAY_BUFFER(&self) -> u32 {
        0x8893
      }

      #[napi(getter, js_name = "BUFFER_SIZE")]
      pub fn BUFFER_SIZE(&self) -> u32 {
        0x8764
      }

      #[napi(getter, js_name = "BUFFER_USAGE")]
      pub fn BUFFER_USAGE(&self) -> u32 {
        0x8765
      }

      #[napi(getter, js_name = "CURRENT_VERTEX_ATTRIB")]
      pub fn CURRENT_VERTEX_ATTRIB(&self) -> u32 {
        0x8626
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_ENABLED")]
      pub fn VERTEX_ATTRIB_ARRAY_ENABLED(&self) -> u32 {
        0x8622
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_SIZE")]
      pub fn VERTEX_ATTRIB_ARRAY_SIZE(&self) -> u32 {
        0x8623
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_STRIDE")]
      pub fn VERTEX_ATTRIB_ARRAY_STRIDE(&self) -> u32 {
        0x8624
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_TYPE")]
      pub fn VERTEX_ATTRIB_ARRAY_TYPE(&self) -> u32 {
        0x8625
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_NORMALIZED")]
      pub fn VERTEX_ATTRIB_ARRAY_NORMALIZED(&self) -> u32 {
        0x886A
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_POINTER")]
      pub fn VERTEX_ATTRIB_ARRAY_POINTER(&self) -> u32 {
        0x8645
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_BUFFER_BINDING")]
      pub fn VERTEX_ATTRIB_ARRAY_BUFFER_BINDING(&self) -> u32 {
        0x889F
      }

      #[napi(getter, js_name = "CULL_FACE")]
      pub fn CULL_FACE(&self) -> u32 {
        0x0B44
      }

      #[napi(getter, js_name = "FRONT")]
      pub fn FRONT(&self) -> u32 {
        0x0404
      }

      #[napi(getter, js_name = "BACK")]
      pub fn BACK(&self) -> u32 {
        0x0405
      }

      #[napi(getter, js_name = "FRONT_AND_BACK")]
      pub fn FRONT_AND_BACK(&self) -> u32 {
        0x0408
      }

      #[napi(getter, js_name = "BLEND")]
      pub fn BLEND(&self) -> u32 {
        0x0BE2
      }

      #[napi(getter, js_name = "DEPTH_TEST")]
      pub fn DEPTH_TEST(&self) -> u32 {
        0x0B71
      }

      #[napi(getter, js_name = "DITHER")]
      pub fn DITHER(&self) -> u32 {
        0x0BD0
      }

      #[napi(getter, js_name = "POLYGON_OFFSET_FILL")]
      pub fn POLYGON_OFFSET_FILL(&self) -> u32 {
        0x8037
      }

      #[napi(getter, js_name = "SAMPLE_ALPHA_TO_COVERAGE")]
      pub fn SAMPLE_ALPHA_TO_COVERAGE(&self) -> u32 {
        0x809E
      }

      #[napi(getter, js_name = "SAMPLE_COVERAGE")]
      pub fn SAMPLE_COVERAGE(&self) -> u32 {
        0x80A0
      }

      #[napi(getter, js_name = "SCISSOR_TEST")]
      pub fn SCISSOR_TEST(&self) -> u32 {
        0x0C11
      }

      #[napi(getter, js_name = "STENCIL_TEST")]
      pub fn STENCIL_TEST(&self) -> u32 {
        0x0B90
      }

      /* Errors */
      #[napi(getter, js_name = "NO_ERROR")]
      pub fn NO_ERROR(&self) -> u32 {
        0
      }

      #[napi(getter, js_name = "INVALID_ENUM")]
      pub fn INVALID_ENUM(&self) -> u32 {
        0x0500
      }

      #[napi(getter, js_name = "INVALID_VALUE")]
      pub fn INVALID_VALUE(&self) -> u32 {
        0x0501
      }

      #[napi(getter, js_name = "INVALID_OPERATION")]
      pub fn INVALID_OPERATION(&self) -> u32 {
        0x0502
      }

      #[napi(getter, js_name = "OUT_OF_MEMORY")]
      pub fn OUT_OF_MEMORY(&self) -> u32 {
        0x0505
      }

      #[napi(getter, js_name = "CONTEXT_LOST_WEBGL")]
      pub fn CONTEXT_LOST_WEBGL(&self) -> u32 {
        0x9242
      }

      #[napi(getter, js_name = "CW")]
      pub fn CW(&self) -> u32 {
        0x0900
      }

      #[napi(getter, js_name = "CCW")]
      pub fn CCW(&self) -> u32 {
        0x0901
      }

      #[napi(getter, js_name = "DONT_CARE")]
      pub fn DONT_CARE(&self) -> u32 {
        0x1100
      }

      #[napi(getter, js_name = "FASTEST")]
      pub fn FASTEST(&self) -> u32 {
        0x1101
      }

      #[napi(getter, js_name = "NICEST")]
      pub fn NICEST(&self) -> u32 {
        0x1102
      }

      #[napi(getter, js_name = "GENERATE_MIPMAP_HINT")]
      pub fn GENERATE_MIPMAP_HINT(&self) -> u32 {
        0x8192
      }

      #[napi(getter, js_name = "BYTE")]
      pub fn BYTE(&self) -> u32 {
        0x1400
      }

      #[napi(getter, js_name = "UNSIGNED_BYTE")]
      pub fn UNSIGNED_BYTE(&self) -> u32 {
        0x1401
      }

      #[napi(getter, js_name = "SHORT")]
      pub fn SHORT(&self) -> u32 {
        0x1402
      }

      #[napi(getter, js_name = "UNSIGNED_SHORT")]
      pub fn UNSIGNED_SHORT(&self) -> u32 {
        0x1403
      }

      #[napi(getter, js_name = "INT")]
      pub fn INT(&self) -> u32 {
        0x1404
      }

      #[napi(getter, js_name = "UNSIGNED_INT")]
      pub fn UNSIGNED_INT(&self) -> u32 {
        0x1405
      }

      #[napi(getter, js_name = "FLOAT")]
      pub fn FLOAT(&self) -> u32 {
        0x1406
      }

      #[napi(getter, js_name = "DEPTH_COMPONENT")]
      pub fn DEPTH_COMPONENT(&self) -> u32 {
        0x1902
      }

      #[napi(getter, js_name = "ALPHA")]
      pub fn ALPHA(&self) -> u32 {
        0x1906
      }

      #[napi(getter, js_name = "RGB")]
      pub fn RGB(&self) -> u32 {
        0x1907
      }

      /* Clearing buffers */

      #[napi(getter, js_name = "RGBA")]
      pub fn RGBA(&self) -> u32 {
        0x1908
      }

      #[napi(getter, js_name = "LUMINANCE")]
      pub fn LUMINANCE(&self) -> u32 {
        0x1909
      }

      #[napi(getter, js_name = "LUMINANCE_ALPHA")]
      pub fn LUMINANCE_ALPHA(&self) -> u32 {
        0x190A
      }

      /* Clearing buffers */

      /* Rendering primitives */

      #[napi(getter, js_name = "UNSIGNED_SHORT_4_4_4_4")]
      pub fn UNSIGNED_SHORT_4_4_4_4(&self) -> u32 {
        0x8033
      }

      #[napi(getter, js_name = "UNSIGNED_SHORT_5_5_5_1")]
      pub fn UNSIGNED_SHORT_5_5_5_1(&self) -> u32 {
        0x8034
      }

      #[napi(getter, js_name = "UNSIGNED_SHORT_5_6_5")]
      pub fn UNSIGNED_SHORT_5_6_5(&self) -> u32 {
        0x8363
      }

      #[napi(getter, js_name = "FRAGMENT_SHADER")]
      pub fn FRAGMENT_SHADER(&self) -> u32 {
        0x8B30
      }

      #[napi(getter, js_name = "VERTEX_SHADER")]
      pub fn VERTEX_SHADER(&self) -> u32 {
        0x8B31
      }

      #[napi(getter, js_name = "COMPILE_STATUS")]
      pub fn COMPILE_STATUS(&self) -> u32 {
        0x8B81
      }

      #[napi(getter, js_name = "DELETE_STATUS")]
      pub fn DELETE_STATUS(&self) -> u32 {
        0x8B80
      }

      /* Rendering primitives */

      /* Blending modes */

      #[napi(getter, js_name = "LINK_STATUS")]
      pub fn LINK_STATUS(&self) -> u32 {
        0x8B82
      }

      #[napi(getter, js_name = "VALIDATE_STATUS")]
      pub fn VALIDATE_STATUS(&self) -> u32 {
        0x8B83
      }

      #[napi(getter, js_name = "ATTACHED_SHADERS")]
      pub fn ATTACHED_SHADERS(&self) -> u32 {
        0x8B85
      }

      #[napi(getter, js_name = "ACTIVE_ATTRIBUTES")]
      pub fn ACTIVE_ATTRIBUTES(&self) -> u32 {
        0x8B89
      }

      #[napi(getter, js_name = "ACTIVE_UNIFORMS")]
      pub fn ACTIVE_UNIFORMS(&self) -> u32 {
        0x8B86
      }

      #[napi(getter, js_name = "MAX_VERTEX_ATTRIBS")]
      pub fn MAX_VERTEX_ATTRIBS(&self) -> u32 {
        0x8869
      }

      #[napi(getter, js_name = "MAX_VERTEX_UNIFORM_VECTORS")]
      pub fn MAX_VERTEX_UNIFORM_VECTORS(&self) -> u32 {
        0x8DFB
      }

      #[napi(getter, js_name = "MAX_VARYING_VECTORS")]
      pub fn MAX_VARYING_VECTORS(&self) -> u32 {
        0x8DFC
      }

      #[napi(getter, js_name = "MAX_COMBINED_TEXTURE_IMAGE_UNITS")]
      pub fn MAX_COMBINED_TEXTURE_IMAGE_UNITS(&self) -> u32 {
        0x8B4D
      }

      #[napi(getter, js_name = "MAX_VERTEX_TEXTURE_IMAGE_UNITS")]
      pub fn MAX_VERTEX_TEXTURE_IMAGE_UNITS(&self) -> u32 {
        0x8B4C
      }

      #[napi(getter, js_name = "MAX_TEXTURE_IMAGE_UNITS")]
      pub fn MAX_TEXTURE_IMAGE_UNITS(&self) -> u32 {
        0x8872
      }

      #[napi(getter, js_name = "MAX_FRAGMENT_UNIFORM_VECTORS")]
      pub fn MAX_FRAGMENT_UNIFORM_VECTORS(&self) -> u32 {
        0x8DFD
      }

      #[napi(getter, js_name = "SHADER_TYPE")]
      pub fn SHADER_TYPE(&self) -> u32 {
        0x8B4F
      }

      #[napi(getter, js_name = "SHADING_LANGUAGE_VERSION")]
      pub fn SHADING_LANGUAGE_VERSION(&self) -> u32 {
        0x8B8C
      }

      #[napi(getter, js_name = "CURRENT_PROGRAM")]
      pub fn CURRENT_PROGRAM(&self) -> u32 {
        0x8B8D
      }

      /* Blending modes */

      #[napi(getter, js_name = "NEVER")]
      pub fn NEVER(&self) -> u32 {
        0x0200
      }

      #[napi(getter, js_name = "LESS")]
      pub fn LESS(&self) -> u32 {
        0x0201
      }

      #[napi(getter, js_name = "EQUAL")]
      pub fn EQUAL(&self) -> u32 {
        0x0202
      }

      /* Blending equations */

      /* Getting GL parameter information */

      #[napi(getter, js_name = "LEQUAL")]
      pub fn LEQUAL(&self) -> u32 {
        0x0203
      }

      #[napi(getter, js_name = "GREATER")]
      pub fn GREATER(&self) -> u32 {
        0x0204
      }

      #[napi(getter, js_name = "NOTEQUAL")]
      pub fn NOTEQUAL(&self) -> u32 {
        0x0205
      }

      #[napi(getter, js_name = "GEQUAL")]
      pub fn GEQUAL(&self) -> u32 {
        0x0206
      }

      #[napi(getter, js_name = "ALWAYS")]
      pub fn ALWAYS(&self) -> u32 {
        0x0207
      }

      #[napi(getter, js_name = "KEEP")]
      pub fn KEEP(&self) -> u32 {
        0x1E00
      }

      #[napi(getter, js_name = "REPLACE")]
      pub fn REPLACE(&self) -> u32 {
        0x1E01
      }

      #[napi(getter, js_name = "INCR")]
      pub fn INCR(&self) -> u32 {
        0x1E02
      }

      #[napi(getter, js_name = "DECR")]
      pub fn DECR(&self) -> u32 {
        0x1E03
      }

      #[napi(getter, js_name = "INVERT")]
      pub fn INVERT(&self) -> u32 {
        0x150A
      }

      #[napi(getter, js_name = "INCR_WRAP")]
      pub fn INCR_WRAP(&self) -> u32 {
        0x8507
      }

      #[napi(getter, js_name = "DECR_WRAP")]
      pub fn DECR_WRAP(&self) -> u32 {
        0x8508
      }

      #[napi(getter, js_name = "NEAREST")]
      pub fn NEAREST(&self) -> u32 {
        0x2600
      }

      #[napi(getter, js_name = "LINEAR")]
      pub fn LINEAR(&self) -> u32 {
        0x2601
      }

      #[napi(getter, js_name = "NEAREST_MIPMAP_NEAREST")]
      pub fn NEAREST_MIPMAP_NEAREST(&self) -> u32 {
        0x2700
      }

      #[napi(getter, js_name = "LINEAR_MIPMAP_NEAREST")]
      pub fn LINEAR_MIPMAP_NEAREST(&self) -> u32 {
        0x2701
      }

      #[napi(getter, js_name = "NEAREST_MIPMAP_LINEAR")]
      pub fn NEAREST_MIPMAP_LINEAR(&self) -> u32 {
        0x2702
      }

      #[napi(getter, js_name = "LINEAR_MIPMAP_LINEAR")]
      pub fn LINEAR_MIPMAP_LINEAR(&self) -> u32 {
        0x2703
      }

      #[napi(getter, js_name = "TEXTURE_MAG_FILTER")]
      pub fn TEXTURE_MAG_FILTER(&self) -> u32 {
        0x2800
      }

      #[napi(getter, js_name = "TEXTURE_MIN_FILTER")]
      pub fn TEXTURE_MIN_FILTER(&self) -> u32 {
        0x2801
      }

      #[napi(getter, js_name = "TEXTURE_WRAP_S")]
      pub fn TEXTURE_WRAP_S(&self) -> u32 {
        0x2802
      }

      #[napi(getter, js_name = "TEXTURE_WRAP_T")]
      pub fn TEXTURE_WRAP_T(&self) -> u32 {
        0x2803
      }

      #[napi(getter, js_name = "TEXTURE_2D")]
      pub fn TEXTURE_2D(&self) -> u32 {
        0x0DE1
      }

      #[napi(getter, js_name = "TEXTURE")]
      pub fn TEXTURE(&self) -> u32 {
        0x1702
      }

      #[napi(getter, js_name = "TEXTURE_CUBE_MAP")]
      pub fn TEXTURE_CUBE_MAP(&self) -> u32 {
        0x8513
      }

      #[napi(getter, js_name = "TEXTURE_BINDING_CUBE_MAP")]
      pub fn TEXTURE_BINDING_CUBE_MAP(&self) -> u32 {
        0x8514
      }

      #[napi(getter, js_name = "TEXTURE_CUBE_MAP_POSITIVE_X")]
      pub fn TEXTURE_CUBE_MAP_POSITIVE_X(&self) -> u32 {
        0x8515
      }

      #[napi(getter, js_name = "TEXTURE_CUBE_MAP_NEGATIVE_X")]
      pub fn TEXTURE_CUBE_MAP_NEGATIVE_X(&self) -> u32 {
        0x8516
      }

      #[napi(getter, js_name = "TEXTURE_CUBE_MAP_POSITIVE_Y")]
      pub fn TEXTURE_CUBE_MAP_POSITIVE_Y(&self) -> u32 {
        0x8517
      }

      #[napi(getter, js_name = "TEXTURE_CUBE_MAP_NEGATIVE_Y")]
      pub fn TEXTURE_CUBE_MAP_NEGATIVE_Y(&self) -> u32 {
        0x8518
      }

      #[napi(getter, js_name = "TEXTURE_CUBE_MAP_POSITIVE_Z")]
      pub fn TEXTURE_CUBE_MAP_POSITIVE_Z(&self) -> u32 {
        0x8519
      }

      #[napi(getter, js_name = "TEXTURE_CUBE_MAP_NEGATIVE_Z")]
      pub fn TEXTURE_CUBE_MAP_NEGATIVE_Z(&self) -> u32 {
        0x851A
      }

      #[napi(getter, js_name = "MAX_CUBE_MAP_TEXTURE_SIZE")]
      pub fn MAX_CUBE_MAP_TEXTURE_SIZE(&self) -> u32 {
        0x851C
      }

      #[napi(getter, js_name = "TEXTURE0")]
      pub fn TEXTURE0(&self) -> u32 {
        0x84C0
      }

      #[napi(getter, js_name = "TEXTURE1")]
      pub fn TEXTURE1(&self) -> u32 {
        0x84C1
      }

      #[napi(getter, js_name = "TEXTURE2")]
      pub fn TEXTURE2(&self) -> u32 {
        0x84C2
      }

      #[napi(getter, js_name = "TEXTURE3")]
      pub fn TEXTURE3(&self) -> u32 {
        0x84C3
      }

      #[napi(getter, js_name = "TEXTURE4")]
      pub fn TEXTURE4(&self) -> u32 {
        0x84C4
      }

      #[napi(getter, js_name = "TEXTURE5")]
      pub fn TEXTURE5(&self) -> u32 {
        0x84C5
      }

      #[napi(getter, js_name = "TEXTURE6")]
      pub fn TEXTURE6(&self) -> u32 {
        0x84C6
      }

      #[napi(getter, js_name = "TEXTURE7")]
      pub fn TEXTURE7(&self) -> u32 {
        0x84C7
      }

      #[napi(getter, js_name = "TEXTURE8")]
      pub fn TEXTURE8(&self) -> u32 {
        0x84C8
      }

      #[napi(getter, js_name = "TEXTURE9")]
      pub fn TEXTURE9(&self) -> u32 {
        0x84C9
      }

      #[napi(getter, js_name = "TEXTURE10")]
      pub fn TEXTURE10(&self) -> u32 {
        0x84CA
      }

      #[napi(getter, js_name = "TEXTURE11")]
      pub fn TEXTURE11(&self) -> u32 {
        0x84CB
      }

      #[napi(getter, js_name = "TEXTURE12")]
      pub fn TEXTURE12(&self) -> u32 {
        0x84CC
      }

      #[napi(getter, js_name = "TEXTURE13")]
      pub fn TEXTURE13(&self) -> u32 {
        0x84CD
      }

      #[napi(getter, js_name = "TEXTURE14")]
      pub fn TEXTURE14(&self) -> u32 {
        0x84CE
      }

      #[napi(getter, js_name = "TEXTURE15")]
      pub fn TEXTURE15(&self) -> u32 {
        0x84CF
      }

      #[napi(getter, js_name = "TEXTURE16")]
      pub fn TEXTURE16(&self) -> u32 {
        0x84D0
      }

      #[napi(getter, js_name = "TEXTURE17")]
      pub fn TEXTURE17(&self) -> u32 {
        0x84D1
      }

      #[napi(getter, js_name = "TEXTURE18")]
      pub fn TEXTURE18(&self) -> u32 {
        0x84D2
      }

      #[napi(getter, js_name = "TEXTURE19")]
      pub fn TEXTURE19(&self) -> u32 {
        0x84D3
      }

      #[napi(getter, js_name = "TEXTURE20")]
      pub fn TEXTURE20(&self) -> u32 {
        0x84D4
      }

      #[napi(getter, js_name = "TEXTURE21")]
      pub fn TEXTURE21(&self) -> u32 {
        0x84D5
      }

      #[napi(getter, js_name = "TEXTURE22")]
      pub fn TEXTURE22(&self) -> u32 {
        0x84D6
      }

      #[napi(getter, js_name = "TEXTURE23")]
      pub fn TEXTURE23(&self) -> u32 {
        0x84D7
      }

      #[napi(getter, js_name = "TEXTURE24")]
      pub fn TEXTURE24(&self) -> u32 {
        0x84D8
      }

      #[napi(getter, js_name = "TEXTURE25")]
      pub fn TEXTURE25(&self) -> u32 {
        0x84D9
      }

      #[napi(getter, js_name = "TEXTURE26")]
      pub fn TEXTURE26(&self) -> u32 {
        0x84DA
      }

      #[napi(getter, js_name = "TEXTURE27")]
      pub fn TEXTURE27(&self) -> u32 {
        0x84DB
      }

      #[napi(getter, js_name = "TEXTURE28")]
      pub fn TEXTURE28(&self) -> u32 {
        0x84DC
      }

      #[napi(getter, js_name = "TEXTURE29")]
      pub fn TEXTURE29(&self) -> u32 {
        0x84DD
      }

      /* Getting GL parameter information */

      /* Buffers */

      #[napi(getter, js_name = "TEXTURE30")]
      pub fn TEXTURE30(&self) -> u32 {
        0x84DE
      }

      #[napi(getter, js_name = "TEXTURE31")]
      pub fn TEXTURE31(&self) -> u32 {
        0x84DF
      }

      #[napi(getter, js_name = "ACTIVE_TEXTURE")]
      pub fn ACTIVE_TEXTURE(&self) -> u32 {
        0x84E0
      }

      #[napi(getter, js_name = "REPEAT")]
      pub fn REPEAT(&self) -> u32 {
        0x2901
      }

      #[napi(getter, js_name = "CLAMP_TO_EDGE")]
      pub fn CLAMP_TO_EDGE(&self) -> u32 {
        0x812F
      }

      #[napi(getter, js_name = "MIRRORED_REPEAT")]
      pub fn MIRRORED_REPEAT(&self) -> u32 {
        0x8370
      }

      #[napi(getter, js_name = "FLOAT_VEC2")]
      pub fn FLOAT_VEC2(&self) -> u32 {
        0x8B50
      }

      /* Buffers */

      /* Vertex attributes */

      #[napi(getter, js_name = "FLOAT_VEC3")]
      pub fn FLOAT_VEC3(&self) -> u32 {
        0x8B51
      }

      #[napi(getter, js_name = "FLOAT_VEC4")]
      pub fn FLOAT_VEC4(&self) -> u32 {
        0x8B52
      }

      #[napi(getter, js_name = "INT_VEC2")]
      pub fn INT_VEC2(&self) -> u32 {
        0x8B53
      }

      #[napi(getter, js_name = "INT_VEC3")]
      pub fn INT_VEC3(&self) -> u32 {
        0x8B54
      }

      #[napi(getter, js_name = "INT_VEC4")]
      pub fn INT_VEC4(&self) -> u32 {
        0x8B55
      }

      #[napi(getter, js_name = "BOOL")]
      pub fn BOOL(&self) -> u32 {
        0x8B56
      }

      #[napi(getter, js_name = "BOOL_VEC2")]
      pub fn BOOL_VEC2(&self) -> u32 {
        0x8B57
      }

      #[napi(getter, js_name = "BOOL_VEC3")]
      pub fn BOOL_VEC3(&self) -> u32 {
        0x8B58
      }

      /* Vertex attributes */

      /* Culling */

      #[napi(getter, js_name = "BOOL_VEC4")]
      pub fn BOOL_VEC4(&self) -> u32 {
        0x8B59
      }

      #[napi(getter, js_name = "FLOAT_MAT2")]
      pub fn FLOAT_MAT2(&self) -> u32 {
        0x8B5A
      }

      #[napi(getter, js_name = "FLOAT_MAT3")]
      pub fn FLOAT_MAT3(&self) -> u32 {
        0x8B5B
      }

      #[napi(getter, js_name = "FLOAT_MAT4")]
      pub fn FLOAT_MAT4(&self) -> u32 {
        0x8B5C
      }

      /* Culling */

      /* Enabling and disabling */

      #[napi(getter, js_name = "SAMPLER_2D")]
      pub fn SAMPLER_2D(&self) -> u32 {
        0x8B5E
      }

      #[napi(getter, js_name = "SAMPLER_CUBE")]
      pub fn SAMPLER_CUBE(&self) -> u32 {
        0x8B60
      }

      #[napi(getter, js_name = "LOW_FLOAT")]
      pub fn LOW_FLOAT(&self) -> u32 {
        0x8DF0
      }

      #[napi(getter, js_name = "MEDIUM_FLOAT")]
      pub fn MEDIUM_FLOAT(&self) -> u32 {
        0x8DF1
      }

      #[napi(getter, js_name = "HIGH_FLOAT")]
      pub fn HIGH_FLOAT(&self) -> u32 {
        0x8DF2
      }

      #[napi(getter, js_name = "LOW_INT")]
      pub fn LOW_INT(&self) -> u32 {
        0x8DF3
      }

      #[napi(getter, js_name = "MEDIUM_INT")]
      pub fn MEDIUM_INT(&self) -> u32 {
        0x8DF4
      }

      #[napi(getter, js_name = "HIGH_INT")]
      pub fn HIGH_INT(&self) -> u32 {
        0x8DF5
      }

      /* Enabling and disabling */

      #[napi(getter, js_name = "FRAMEBUFFER")]
      pub fn FRAMEBUFFER(&self) -> u32 {
        0x8D40
      }

      #[napi(getter, js_name = "RENDERBUFFER")]
      pub fn RENDERBUFFER(&self) -> u32 {
        0x8D41
      }

      #[napi(getter, js_name = "RGBA4")]
      pub fn RGBA4(&self) -> u32 {
        0x8056
      }

      #[napi(getter, js_name = "RGB5_A1")]
      pub fn RGB5_A1(&self) -> u32 {
        0x8057
      }

      #[napi(getter, js_name = "RGB565")]
      pub fn RGB565(&self) -> u32 {
        0x8D62
      }

      #[napi(getter, js_name = "DEPTH_COMPONENT16")]
      pub fn DEPTH_COMPONENT16(&self) -> u32 {
        0x81A5
      }

      #[napi(getter, js_name = "STENCIL_INDEX8")]
      pub fn STENCIL_INDEX8(&self) -> u32 {
        0x8D48
      }

      /* Errors */

      /* Front face directions */

      #[napi(getter, js_name = "DEPTH_STENCIL")]
      pub fn DEPTH_STENCIL(&self) -> u32 {
        0x84F9
      }

      #[napi(getter, js_name = "RENDERBUFFER_WIDTH")]
      pub fn RENDERBUFFER_WIDTH(&self) -> u32 {
        0x8D42
      }

      /* Front face directions */

      /* Hints */

      #[napi(getter, js_name = "RENDERBUFFER_HEIGHT")]
      pub fn RENDERBUFFER_HEIGHT(&self) -> u32 {
        0x8D43
      }

      #[napi(getter, js_name = "RENDERBUFFER_INTERNAL_FORMAT")]
      pub fn RENDERBUFFER_INTERNAL_FORMAT(&self) -> u32 {
        0x8D44
      }

      #[napi(getter, js_name = "RENDERBUFFER_RED_SIZE")]
      pub fn RENDERBUFFER_RED_SIZE(&self) -> u32 {
        0x8D50
      }

      #[napi(getter, js_name = "RENDERBUFFER_GREEN_SIZE")]
      pub fn RENDERBUFFER_GREEN_SIZE(&self) -> u32 {
        0x8D51
      }

      /* Hints */

      /* Data types */

      #[napi(getter, js_name = "RENDERBUFFER_BLUE_SIZE")]
      pub fn RENDERBUFFER_BLUE_SIZE(&self) -> u32 {
        0x8D52
      }

      #[napi(getter, js_name = "RENDERBUFFER_ALPHA_SIZE")]
      pub fn RENDERBUFFER_ALPHA_SIZE(&self) -> u32 {
        0x8D53
      }

      #[napi(getter, js_name = "RENDERBUFFER_DEPTH_SIZE")]
      pub fn RENDERBUFFER_DEPTH_SIZE(&self) -> u32 {
        0x8D54
      }

      #[napi(getter, js_name = "RENDERBUFFER_STENCIL_SIZE")]
      pub fn RENDERBUFFER_STENCIL_SIZE(&self) -> u32 {
        0x8D55
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE")]
      pub fn FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE(&self) -> u32 {
        0x8CD0
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_OBJECT_NAME")]
      pub fn FRAMEBUFFER_ATTACHMENT_OBJECT_NAME(&self) -> u32 {
        0x8CD1
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL")]
      pub fn FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL(&self) -> u32 {
        0x8CD2
      }

      /* Data types */

      /* Pixel formats */

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE")]
      pub fn FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE(&self) -> u32 {
        0x8CD3
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT0")]
      pub fn COLOR_ATTACHMENT0(&self) -> u32 {
        0x8CE0
      }

      #[napi(getter, js_name = "DEPTH_ATTACHMENT")]
      pub fn DEPTH_ATTACHMENT(&self) -> u32 {
        0x8D00
      }

      #[napi(getter, js_name = "STENCIL_ATTACHMENT")]
      pub fn STENCIL_ATTACHMENT(&self) -> u32 {
        0x8D20
      }

      #[napi(getter, js_name = "DEPTH_STENCIL_ATTACHMENT")]
      pub fn DEPTH_STENCIL_ATTACHMENT(&self) -> u32 {
        0x821A
      }

      #[napi(getter, js_name = "NONE")]
      pub fn NONE(&self) -> u32 {
        0
      }

      /* Pixel formats */

      /* Pixel types */

      // #[napi(getter, js_name = "UNSIGNED_BYTE")]
      // pub fn UNSIGNED_BYTE(&self) -> u32 { return UNSIGNED_BYTE}

      #[napi(getter, js_name = "FRAMEBUFFER_COMPLETE")]
      pub fn FRAMEBUFFER_COMPLETE(&self) -> u32 {
        0x8CD5
      }

      #[napi(getter, js_name = "FRAMEBUFFER_INCOMPLETE_ATTACHMENT")]
      pub fn FRAMEBUFFER_INCOMPLETE_ATTACHMENT(&self) -> u32 {
        0x8CD6
      }

      #[napi(getter, js_name = "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT")]
      pub fn FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT(&self) -> u32 {
        0x8CD7
      }

      /* Pixel types */

      /* Shaders */

      #[napi(getter, js_name = "FRAMEBUFFER_INCOMPLETE_DIMENSIONS")]
      pub fn FRAMEBUFFER_INCOMPLETE_DIMENSIONS(&self) -> u32 {
        0x8CD9
      }

      #[napi(getter, js_name = "FRAMEBUFFER_UNSUPPORTED")]
      pub fn FRAMEBUFFER_UNSUPPORTED(&self) -> u32 {
        0x8CDD
      }

      #[napi(getter, js_name = "FRAMEBUFFER_BINDING")]
      pub fn FRAMEBUFFER_BINDING(&self) -> u32 {
        0x8CA6
      }

      #[napi(getter, js_name = "RENDERBUFFER_BINDING")]
      pub fn RENDERBUFFER_BINDING(&self) -> u32 {
        0x8CA7
      }

      #[napi(getter, js_name = "MAX_RENDERBUFFER_SIZE")]
      pub fn MAX_RENDERBUFFER_SIZE(&self) -> u32 {
        0x84E8
      }

      #[napi(getter, js_name = "INVALID_FRAMEBUFFER_OPERATION")]
      pub fn INVALID_FRAMEBUFFER_OPERATION(&self) -> u32 {
        0x0506
      }

      #[napi(getter, js_name = "UNPACK_FLIP_Y_WEBGL")]
      pub fn UNPACK_FLIP_Y_WEBGL(&self) -> u32 {
        0x9240
      }

      #[napi(getter, js_name = "UNPACK_PREMULTIPLY_ALPHA_WEBGL")]
      pub fn UNPACK_PREMULTIPLY_ALPHA_WEBGL(&self) -> u32 {
        0x9241
      }

      #[napi(getter, js_name = "UNPACK_COLORSPACE_CONVERSION_WEBGL")]
      pub fn UNPACK_COLORSPACE_CONVERSION_WEBGL(&self) -> u32 {
        0x9243
      }
    }
  };
}

#[macro_export]
macro_rules! impl_webgl2_context_constants {
  ($struct_name:ident) => {
    #[napi]
    impl $struct_name {
      /* Getting GL parameter information */
      #[napi(getter, js_name = "READ_BUFFER")]
      pub fn READ_BUFFER(&self) -> u32 {
        return 0x0c02;
      }

      #[napi(getter, js_name = "UNPACK_ROW_LENGTH")]
      pub fn UNPACK_ROW_LENGTH(&self) -> u32 {
        return 0x0cf2;
      }

      #[napi(getter, js_name = "UNPACK_SKIP_ROWS")]
      pub fn UNPACK_SKIP_ROWS(&self) -> u32 {
        return 0x0cf3;
      }

      #[napi(getter, js_name = "UNPACK_SKIP_PIXELS")]
      pub fn UNPACK_SKIP_PIXELS(&self) -> u32 {
        return 0x0cf4;
      }

      #[napi(getter, js_name = "PACK_ROW_LENGTH")]
      pub fn PACK_ROW_LENGTH(&self) -> u32 {
        return 0x0d02;
      }

      #[napi(getter, js_name = "PACK_SKIP_ROWS")]
      pub fn PACK_SKIP_ROWS(&self) -> u32 {
        return 0x0d03;
      }

      #[napi(getter, js_name = "PACK_SKIP_PIXELS")]
      pub fn PACK_SKIP_PIXELS(&self) -> u32 {
        return 0x0d04;
      }

      #[napi(getter, js_name = "TEXTURE_BINDING_3D")]
      pub fn TEXTURE_BINDING_3D(&self) -> u32 {
        return 0x806a;
      }

      #[napi(getter, js_name = "UNPACK_SKIP_IMAGES")]
      pub fn UNPACK_SKIP_IMAGES(&self) -> u32 {
        return 0x806d;
      }

      #[napi(getter, js_name = "UNPACK_IMAGE_HEIGHT")]
      pub fn UNPACK_IMAGE_HEIGHT(&self) -> u32 {
        return 0x806e;
      }

      #[napi(getter, js_name = "MAX_3D_TEXTURE_SIZE")]
      pub fn MAX_3D_TEXTURE_SIZE(&self) -> u32 {
        return 0x8073;
      }

      #[napi(getter, js_name = "MAX_ELEMENTS_VERTICES")]
      pub fn MAX_ELEMENTS_VERTICES(&self) -> u32 {
        return 0x80e8;
      }

      #[napi(getter, js_name = "MAX_ELEMENTS_INDICES")]
      pub fn MAX_ELEMENTS_INDICES(&self) -> u32 {
        return 0x80e9;
      }

      #[napi(getter, js_name = "MAX_TEXTURE_LOD_BIAS")]
      pub fn MAX_TEXTURE_LOD_BIAS(&self) -> u32 {
        return 0x84fd;
      }

      #[napi(getter, js_name = "MAX_FRAGMENT_UNIFORM_COMPONENTS")]
      pub fn MAX_FRAGMENT_UNIFORM_COMPONENTS(&self) -> u32 {
        return 0x8b49;
      }

      #[napi(getter, js_name = "MAX_VERTEX_UNIFORM_COMPONENTS")]
      pub fn MAX_VERTEX_UNIFORM_COMPONENTS(&self) -> u32 {
        return 0x8b4a;
      }

      #[napi(getter, js_name = "MAX_ARRAY_TEXTURE_LAYERS")]
      pub fn MAX_ARRAY_TEXTURE_LAYERS(&self) -> u32 {
        return 0x88ff;
      }

      #[napi(getter, js_name = "MIN_PROGRAM_TEXEL_OFFSET")]
      pub fn MIN_PROGRAM_TEXEL_OFFSET(&self) -> u32 {
        return 0x8904;
      }

      #[napi(getter, js_name = "MAX_PROGRAM_TEXEL_OFFSET")]
      pub fn MAX_PROGRAM_TEXEL_OFFSET(&self) -> u32 {
        return 0x8905;
      }

      #[napi(getter, js_name = "MAX_VARYING_COMPONENTS")]
      pub fn MAX_VARYING_COMPONENTS(&self) -> u32 {
        return 0x8b4b;
      }

      #[napi(getter, js_name = "FRAGMENT_SHADER_DERIVATIVE_HINT")]
      pub fn FRAGMENT_SHADER_DERIVATIVE_HINT(&self) -> u32 {
        return 0x8b8b;
      }

      #[napi(getter, js_name = "RASTERIZER_DISCARD")]
      pub fn RASTERIZER_DISCARD(&self) -> u32 {
        return 0x8c89;
      }

      #[napi(getter, js_name = "VERTEX_ARRAY_BINDING")]
      pub fn VERTEX_ARRAY_BINDING(&self) -> u32 {
        return 0x85b5;
      }

      #[napi(getter, js_name = "MAX_VERTEX_OUTPUT_COMPONENTS")]
      pub fn MAX_VERTEX_OUTPUT_COMPONENTS(&self) -> u32 {
        return 0x9122;
      }

      #[napi(getter, js_name = "MAX_FRAGMENT_INPUT_COMPONENTS")]
      pub fn MAX_FRAGMENT_INPUT_COMPONENTS(&self) -> u32 {
        return 0x9125;
      }

      #[napi(getter, js_name = "MAX_SERVER_WAIT_TIMEOUT")]
      pub fn MAX_SERVER_WAIT_TIMEOUT(&self) -> u32 {
        return 0x9111;
      }

      #[napi(getter, js_name = "MAX_ELEMENT_INDEX")]
      pub fn MAX_ELEMENT_INDEX(&self) -> u32 {
        return 0x8d6b;
      }

      #[napi(getter, js_name = "RED")]
      pub fn RED(&self) -> u32 {
        return 0x1903;
      }

      #[napi(getter, js_name = "RGB8")]
      pub fn RGB8(&self) -> u32 {
        return 0x8051;
      }

      #[napi(getter, js_name = "RGBA8")]
      pub fn RGBA8(&self) -> u32 {
        return 0x8058;
      }

      #[napi(getter, js_name = "RGB10_A2")]
      pub fn RGB10_A2(&self) -> u32 {
        return 0x8059;
      }

      #[napi(getter, js_name = "TEXTURE_3D")]
      pub fn TEXTURE_3D(&self) -> u32 {
        return 0x806f;
      }

      #[napi(getter, js_name = "TEXTURE_WRAP_R")]
      pub fn TEXTURE_WRAP_R(&self) -> u32 {
        return 0x8072;
      }

      #[napi(getter, js_name = "TEXTURE_MIN_LOD")]
      pub fn TEXTURE_MIN_LOD(&self) -> u32 {
        return 0x813a;
      }

      #[napi(getter, js_name = "TEXTURE_MAX_LOD")]
      pub fn TEXTURE_MAX_LOD(&self) -> u32 {
        return 0x813b;
      }

      #[napi(getter, js_name = "TEXTURE_BASE_LEVEL")]
      pub fn TEXTURE_BASE_LEVEL(&self) -> u32 {
        return 0x813c;
      }

      #[napi(getter, js_name = "TEXTURE_MAX_LEVEL")]
      pub fn TEXTURE_MAX_LEVEL(&self) -> u32 {
        return 0x813d;
      }

      #[napi(getter, js_name = "TEXTURE_COMPARE_MODE")]
      pub fn TEXTURE_COMPARE_MODE(&self) -> u32 {
        return 0x884c;
      }

      #[napi(getter, js_name = "TEXTURE_COMPARE_FUNC")]
      pub fn TEXTURE_COMPARE_FUNC(&self) -> u32 {
        return 0x884d;
      }

      #[napi(getter, js_name = "SRGB")]
      pub fn SRGB(&self) -> u32 {
        return 0x8c40;
      }

      #[napi(getter, js_name = "SRGB8")]
      pub fn SRGB8(&self) -> u32 {
        return 0x8c41;
      }

      #[napi(getter, js_name = "SRGB8_ALPHA8")]
      pub fn SRGB8_ALPHA8(&self) -> u32 {
        return 0x8c43;
      }

      #[napi(getter, js_name = "COMPARE_REF_TO_TEXTURE")]
      pub fn COMPARE_REF_TO_TEXTURE(&self) -> u32 {
        return 0x884e;
      }

      #[napi(getter, js_name = "RGBA32F")]
      pub fn RGBA32F(&self) -> u32 {
        return 0x8814;
      }

      #[napi(getter, js_name = "RGB32F")]
      pub fn RGB32F(&self) -> u32 {
        return 0x8815;
      }

      #[napi(getter, js_name = "RGBA16F")]
      pub fn RGBA16F(&self) -> u32 {
        return 0x881a;
      }

      #[napi(getter, js_name = "RGB16F")]
      pub fn RGB16F(&self) -> u32 {
        return 0x881b;
      }

      #[napi(getter, js_name = "TEXTURE_2D_ARRAY")]
      pub fn TEXTURE_2D_ARRAY(&self) -> u32 {
        return 0x8c1a;
      }

      #[napi(getter, js_name = "TEXTURE_BINDING_2D_ARRAY")]
      pub fn TEXTURE_BINDING_2D_ARRAY(&self) -> u32 {
        return 0x8c1d;
      }

      #[napi(getter, js_name = "R11F_G11F_B10F")]
      pub fn R11F_G11F_B10F(&self) -> u32 {
        return 0x8c3a;
      }

      #[napi(getter, js_name = "RGB9_E5")]
      pub fn RGB9_E5(&self) -> u32 {
        return 0x8c3d;
      }

      #[napi(getter, js_name = "RGBA32UI")]
      pub fn RGBA32UI(&self) -> u32 {
        return 0x8d70;
      }

      #[napi(getter, js_name = "RGB32UI")]
      pub fn RGB32UI(&self) -> u32 {
        return 0x8d71;
      }

      #[napi(getter, js_name = "RGBA16UI")]
      pub fn RGBA16UI(&self) -> u32 {
        return 0x8d76;
      }


      /* Getting GL parameter information */

      #[napi(getter, js_name = "RGB16UI")]
      pub fn RGB16UI(&self) -> u32 {
        return 0x8d77;
      }

      #[napi(getter, js_name = "RGBA8UI")]
      pub fn RGBA8UI(&self) -> u32 {
        return 0x8d7c;
      }

      #[napi(getter, js_name = "RGB8UI")]
      pub fn RGB8UI(&self) -> u32 {
        return 0x8d7d;
      }

      #[napi(getter, js_name = "RGBA32I")]
      pub fn RGBA32I(&self) -> u32 {
        return 0x8d82;
      }

      #[napi(getter, js_name = "RGB32I")]
      pub fn RGB32I(&self) -> u32 {
        return 0x8d83;
      }

      #[napi(getter, js_name = "RGBA16I")]
      pub fn RGBA16I(&self) -> u32 {
        return 0x8d88;
      }

      #[napi(getter, js_name = "RGB16I")]
      pub fn RGB16I(&self) -> u32 {
        return 0x8d89;
      }

      #[napi(getter, js_name = "RGBA8I")]
      pub fn RGBA8I(&self) -> u32 {
        return 0x8d8e;
      }

      #[napi(getter, js_name = "RGB8I")]
      pub fn RGB8I(&self) -> u32 {
        return 0x8d8f;
      }

      #[napi(getter, js_name = "RED_INTEGER")]
      pub fn RED_INTEGER(&self) -> u32 {
        return 0x8d94;
      }

      #[napi(getter, js_name = "RGB_INTEGER")]
      pub fn RGB_INTEGER(&self) -> u32 {
        return 0x8d98;
      }

      #[napi(getter, js_name = "RGBA_INTEGER")]
      pub fn RGBA_INTEGER(&self) -> u32 {
        return 0x8d99;
      }

      #[napi(getter, js_name = "R8")]
      pub fn R8(&self) -> u32 {
        return 0x8229;
      }

      #[napi(getter, js_name = "RG8")]
      pub fn RG8(&self) -> u32 {
        return 0x822b;
      }

      #[napi(getter, js_name = "R16F")]
      pub fn R16F(&self) -> u32 {
        return 0x822d;
      }

      #[napi(getter, js_name = "R32F")]
      pub fn R32F(&self) -> u32 {
        return 0x822e;
      }

      #[napi(getter, js_name = "RG16F")]
      pub fn RG16F(&self) -> u32 {
        return 0x822f;
      }

      #[napi(getter, js_name = "RG32F")]
      pub fn RG32F(&self) -> u32 {
        return 0x8230;
      }

      #[napi(getter, js_name = "R8I")]
      pub fn R8I(&self) -> u32 {
        return 0x8231;
      }

      #[napi(getter, js_name = "R8UI")]
      pub fn R8UI(&self) -> u32 {
        return 0x8232;
      }

      #[napi(getter, js_name = "R16I")]
      pub fn R16I(&self) -> u32 {
        return 0x8233;
      }

      #[napi(getter, js_name = "R16UI")]
      pub fn R16UI(&self) -> u32 {
        return 0x8234;
      }

      #[napi(getter, js_name = "R32I")]
      pub fn R32I(&self) -> u32 {
        return 0x8235;
      }

      #[napi(getter, js_name = "R32UI")]
      pub fn R32UI(&self) -> u32 {
        return 0x8236;
      }

      #[napi(getter, js_name = "RG8I")]
      pub fn RG8I(&self) -> u32 {
        return 0x8237;
      }

      #[napi(getter, js_name = "RG8UI")]
      pub fn RG8UI(&self) -> u32 {
        return 0x8238;
      }

      #[napi(getter, js_name = "RG16I")]
      pub fn RG16I(&self) -> u32 {
        return 0x8239;
      }

      #[napi(getter, js_name = "RG16UI")]
      pub fn RG16UI(&self) -> u32 {
        return 0x823a;
      }

      #[napi(getter, js_name = "RG32I")]
      pub fn RG32I(&self) -> u32 {
        return 0x823b;
      }

      #[napi(getter, js_name = "RG32UI")]
      pub fn RG32UI(&self) -> u32 {
        return 0x823c;
      }

      #[napi(getter, js_name = "R8_SNORM")]
      pub fn R8_SNORM(&self) -> u32 {
        return 0x8f94;
      }

      #[napi(getter, js_name = "RG8_SNORM")]
      pub fn RG8_SNORM(&self) -> u32 {
        return 0x8f95;
      }

      #[napi(getter, js_name = "RGB8_SNORM")]
      pub fn RGB8_SNORM(&self) -> u32 {
        return 0x8f96;
      }

      #[napi(getter, js_name = "RGBA8_SNORM")]
      pub fn RGBA8_SNORM(&self) -> u32 {
        return 0x8f97;
      }

      #[napi(getter, js_name = "RGB10_A2UI")]
      pub fn RGB10_A2UI(&self) -> u32 {
        return 0x906f;
      }

      #[napi(getter, js_name = "TEXTURE_IMMUTABLE_FORMAT")]
      pub fn TEXTURE_IMMUTABLE_FORMAT(&self) -> u32 {
        return 0x912f;
      }

      #[napi(getter, js_name = "TEXTURE_IMMUTABLE_LEVELS")]
      pub fn TEXTURE_IMMUTABLE_LEVELS(&self) -> u32 {
        return 0x82df;
      }

      #[napi(getter, js_name = "UNSIGNED_INT_2_10_10_10_REV")]
      pub fn UNSIGNED_INT_2_10_10_10_REV(&self) -> u32 {
        return 0x8368;
      }

      #[napi(getter, js_name = "UNSIGNED_INT_10F_11F_11F_REV")]
      pub fn UNSIGNED_INT_10F_11F_11F_REV(&self) -> u32 {
        return 0x8c3b;
      }

      #[napi(getter, js_name = "UNSIGNED_INT_5_9_9_9_REV")]
      pub fn UNSIGNED_INT_5_9_9_9_REV(&self) -> u32 {
        return 0x8c3e;
      }

      #[napi(getter, js_name = "FLOAT_32_UNSIGNED_INT_24_8_REV")]
      pub fn FLOAT_32_UNSIGNED_INT_24_8_REV(&self) -> u32 {
        return 0x8dad;
      }

      #[napi(getter, js_name = "UNSIGNED_INT_24_8")]
      pub fn UNSIGNED_INT_24_8(&self) -> u32 {
        return 0x84fa;
      }

      #[napi(getter, js_name = "HALF_FLOAT")]
      pub fn HALF_FLOAT(&self) -> u32 {
        return 0x140b;
      }

      #[napi(getter, js_name = "RG")]
      pub fn RG(&self) -> u32 {
        return 0x8227;
      }

      #[napi(getter, js_name = "RG_INTEGER")]
      pub fn RG_INTEGER(&self) -> u32 {
        return 0x8228;
      }

      #[napi(getter, js_name = "INT_2_10_10_10_REV")]
      pub fn INT_2_10_10_10_REV(&self) -> u32 {
        return 0x8d9f;
      }

      #[napi(getter, js_name = "QUERY_RESULT_AVAILABLE")]
      pub fn QUERY_RESULT_AVAILABLE(&self) -> u32 {
        return 0x8865;
      }

      #[napi(getter, js_name = "QUERY_RESULT")]
      pub fn QUERY_RESULT(&self) -> u32 {
        return 0x8866;
      }

      #[napi(getter, js_name = "CURRENT_QUERY")]
      pub fn CURRENT_QUERY(&self) -> u32 {
        return 0x8867;
      }

      #[napi(getter, js_name = "ANY_SAMPLES_PASSED")]
      pub fn ANY_SAMPLES_PASSED(&self) -> u32 {
        return 0x8c2f;
      }

      #[napi(getter, js_name = "ANY_SAMPLES_PASSED_CONSERVATIVE")]
      pub fn ANY_SAMPLES_PASSED_CONSERVATIVE(&self) -> u32 {
        return 0x8d6a;
      }

      #[napi(getter, js_name = "MAX_DRAW_BUFFERS")]
      pub fn MAX_DRAW_BUFFERS(&self) -> u32 {
        return 0x8824;
      }

      #[napi(getter, js_name = "DRAW_BUFFER0")]
      pub fn DRAW_BUFFER0(&self) -> u32 {
        return 0x8825;
      }

      #[napi(getter, js_name = "DRAW_BUFFER1")]
      pub fn DRAW_BUFFER1(&self) -> u32 {
        return 0x8826;
      }

      #[napi(getter, js_name = "DRAW_BUFFER2")]
      pub fn DRAW_BUFFER2(&self) -> u32 {
        return 0x8827;
      }

      #[napi(getter, js_name = "DRAW_BUFFER3")]
      pub fn DRAW_BUFFER3(&self) -> u32 {
        return 0x8828;
      }

      #[napi(getter, js_name = "DRAW_BUFFER4")]
      pub fn DRAW_BUFFER4(&self) -> u32 {
        return 0x8829;
      }

      #[napi(getter, js_name = "DRAW_BUFFER5")]
      pub fn DRAW_BUFFER5(&self) -> u32 {
        return 0x882a;
      }

      #[napi(getter, js_name = "DRAW_BUFFER6")]
      pub fn DRAW_BUFFER6(&self) -> u32 {
        return 0x882b;
      }

      #[napi(getter, js_name = "DRAW_BUFFER7")]
      pub fn DRAW_BUFFER7(&self) -> u32 {
        return 0x882c;
      }

      #[napi(getter, js_name = "DRAW_BUFFER8")]
      pub fn DRAW_BUFFER8(&self) -> u32 {
        return 0x882d;
      }

      #[napi(getter, js_name = "DRAW_BUFFER9")]
      pub fn DRAW_BUFFER9(&self) -> u32 {
        return 0x882e;
      }

      #[napi(getter, js_name = "DRAW_BUFFER10")]
      pub fn DRAW_BUFFER10(&self) -> u32 {
        return 0x882f;
      }

      /* Textures */

      #[napi(getter, js_name = "DRAW_BUFFER11")]
      pub fn DRAW_BUFFER11(&self) -> u32 {
        0x8830
      }

      #[napi(getter, js_name = "DRAW_BUFFER12")]
      pub fn DRAW_BUFFER12(&self) -> u32 {
        0x8831
      }

      #[napi(getter, js_name = "DRAW_BUFFER13")]
      pub fn DRAW_BUFFER13(&self) -> u32 {
        0x8832
      }

      #[napi(getter, js_name = "DRAW_BUFFER14")]
      pub fn DRAW_BUFFER14(&self) -> u32 {
        0x8833
      }

      #[napi(getter, js_name = "DRAW_BUFFER15")]
      pub fn DRAW_BUFFER15(&self) -> u32 {
        0x8834
      }

      #[napi(getter, js_name = "MAX_COLOR_ATTACHMENTS")]
      pub fn MAX_COLOR_ATTACHMENTS(&self) -> u32 {
        0x8cdf
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT1")]
      pub fn COLOR_ATTACHMENT1(&self) -> u32 {
        0x8ce1
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT2")]
      pub fn COLOR_ATTACHMENT2(&self) -> u32 {
        0x8ce2
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT3")]
      pub fn COLOR_ATTACHMENT3(&self) -> u32 {
        0x8ce3
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT4")]
      pub fn COLOR_ATTACHMENT4(&self) -> u32 {
        0x8ce4
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT5")]
      pub fn COLOR_ATTACHMENT5(&self) -> u32 {
        0x8ce5
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT6")]
      pub fn COLOR_ATTACHMENT6(&self) -> u32 {
        0x8ce6
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT7")]
      pub fn COLOR_ATTACHMENT7(&self) -> u32 {
        0x8ce7
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT8")]
      pub fn COLOR_ATTACHMENT8(&self) -> u32 {
        0x8ce8
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT9")]
      pub fn COLOR_ATTACHMENT9(&self) -> u32 {
        0x8ce9
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT10")]
      pub fn COLOR_ATTACHMENT10(&self) -> u32 {
        0x8cea
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT11")]
      pub fn COLOR_ATTACHMENT11(&self) -> u32 {
        0x8ceb
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT12")]
      pub fn COLOR_ATTACHMENT12(&self) -> u32 {
        0x8cec
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT13")]
      pub fn COLOR_ATTACHMENT13(&self) -> u32 {
        0x8ced
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT14")]
      pub fn COLOR_ATTACHMENT14(&self) -> u32 {
        0x8cee
      }

      #[napi(getter, js_name = "COLOR_ATTACHMENT15")]
      pub fn COLOR_ATTACHMENT15(&self) -> u32 {
        0x8cef
      }

      #[napi(getter, js_name = "SAMPLER_3D")]
      pub fn SAMPLER_3D(&self) -> u32 {
        0x8b5f
      }

      #[napi(getter, js_name = "SAMPLER_2D_SHADOW")]
      pub fn SAMPLER_2D_SHADOW(&self) -> u32 {
        0x8b62
      }

      #[napi(getter, js_name = "SAMPLER_2D_ARRAY")]
      pub fn SAMPLER_2D_ARRAY(&self) -> u32 {
        0x8dc1
      }

      #[napi(getter, js_name = "SAMPLER_2D_ARRAY_SHADOW")]
      pub fn SAMPLER_2D_ARRAY_SHADOW(&self) -> u32 {
        0x8dc4
      }

      #[napi(getter, js_name = "SAMPLER_CUBE_SHADOW")]
      pub fn SAMPLER_CUBE_SHADOW(&self) -> u32 {
        0x8dc5
      }

      #[napi(getter, js_name = "INT_SAMPLER_2D")]
      pub fn INT_SAMPLER_2D(&self) -> u32 {
        0x8dca
      }

      #[napi(getter, js_name = "INT_SAMPLER_3D")]
      pub fn INT_SAMPLER_3D(&self) -> u32 {
        0x8dcb
      }

      #[napi(getter, js_name = "INT_SAMPLER_CUBE")]
      pub fn INT_SAMPLER_CUBE(&self) -> u32 {
        0x8dcc
      }

      #[napi(getter, js_name = "INT_SAMPLER_2D_ARRAY")]
      pub fn INT_SAMPLER_2D_ARRAY(&self) -> u32 {
        0x8dcf
      }

      #[napi(getter, js_name = "UNSIGNED_INT_SAMPLER_2D")]
      pub fn UNSIGNED_INT_SAMPLER_2D(&self) -> u32 {
        0x8dd2
      }

      #[napi(getter, js_name = "UNSIGNED_INT_SAMPLER_3D")]
      pub fn UNSIGNED_INT_SAMPLER_3D(&self) -> u32 {
        0x8dd3
      }

      #[napi(getter, js_name = "UNSIGNED_INT_SAMPLER_CUBE")]
      pub fn UNSIGNED_INT_SAMPLER_CUBE(&self) -> u32 {
        0x8dd4
      }

      #[napi(getter, js_name = "UNSIGNED_INT_SAMPLER_2D_ARRAY")]
      pub fn UNSIGNED_INT_SAMPLER_2D_ARRAY(&self) -> u32 {
        0x8dd7
      }

      #[napi(getter, js_name = "MAX_SAMPLES")]
      pub fn MAX_SAMPLES(&self) -> u32 {
        0x8d57
      }

      #[napi(getter, js_name = "SAMPLER_BINDING")]
      pub fn SAMPLER_BINDING(&self) -> u32 {
        0x8919
      }

      #[napi(getter, js_name = "PIXEL_PACK_BUFFER")]
      pub fn PIXEL_PACK_BUFFER(&self) -> u32 {
        0x88eb
      }

      #[napi(getter, js_name = "PIXEL_UNPACK_BUFFER")]
      pub fn PIXEL_UNPACK_BUFFER(&self) -> u32 {
        0x88ec
      }

      #[napi(getter, js_name = "PIXEL_PACK_BUFFER_BINDING")]
      pub fn PIXEL_PACK_BUFFER_BINDING(&self) -> u32 {
        0x88ed
      }

      #[napi(getter, js_name = "PIXEL_UNPACK_BUFFER_BINDING")]
      pub fn PIXEL_UNPACK_BUFFER_BINDING(&self) -> u32 {
        0x88ef
      }

      #[napi(getter, js_name = "COPY_READ_BUFFER")]
      pub fn COPY_READ_BUFFER(&self) -> u32 {
        0x8f36
      }

      #[napi(getter, js_name = "COPY_WRITE_BUFFER")]
      pub fn COPY_WRITE_BUFFER(&self) -> u32 {
        0x8f37
      }

      #[napi(getter, js_name = "COPY_READ_BUFFER_BINDING")]
      pub fn COPY_READ_BUFFER_BINDING(&self) -> u32 {
        0x8f36
      }

      #[napi(getter, js_name = "COPY_WRITE_BUFFER_BINDING")]
      pub fn COPY_WRITE_BUFFER_BINDING(&self) -> u32 {
        0x8f37
      }

      #[napi(getter, js_name = "FLOAT_MAT2x3")]
      pub fn FLOAT_MAT2x3(&self) -> u32 {
        0x8b65
      }

      #[napi(getter, js_name = "FLOAT_MAT2x4")]
      pub fn FLOAT_MAT2x4(&self) -> u32 {
        0x8b66
      }

      #[napi(getter, js_name = "FLOAT_MAT3x2")]
      pub fn FLOAT_MAT3x2(&self) -> u32 {
        0x8b67
      }

      #[napi(getter, js_name = "FLOAT_MAT3x4")]
      pub fn FLOAT_MAT3x4(&self) -> u32 {
        0x8b68
      }

      #[napi(getter, js_name = "FLOAT_MAT4x2")]
      pub fn FLOAT_MAT4x2(&self) -> u32 {
        0x8b69
      }

      #[napi(getter, js_name = "FLOAT_MAT4x3")]
      pub fn FLOAT_MAT4x3(&self) -> u32 {
        0x8b6a
      }

      #[napi(getter, js_name = "UNSIGNED_INT_VEC2")]
      pub fn UNSIGNED_INT_VEC2(&self) -> u32 {
        0x8dc6
      }

      #[napi(getter, js_name = "UNSIGNED_INT_VEC3")]
      pub fn UNSIGNED_INT_VEC3(&self) -> u32 {
        0x8dc7
      }

      #[napi(getter, js_name = "UNSIGNED_INT_VEC4")]
      pub fn UNSIGNED_INT_VEC4(&self) -> u32 {
        0x8dc8
      }

      #[napi(getter, js_name = "UNSIGNED_NORMALIZED")]
      pub fn UNSIGNED_NORMALIZED(&self) -> u32 {
        0x8c17
      }

      #[napi(getter, js_name = "SIGNED_NORMALIZED")]
      pub fn SIGNED_NORMALIZED(&self) -> u32 {
        0x8f9c
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_INTEGER")]
      pub fn VERTEX_ATTRIB_ARRAY_INTEGER(&self) -> u32 {
        0x88fd
      }

      #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_DIVISOR")]
      pub fn VERTEX_ATTRIB_ARRAY_DIVISOR(&self) -> u32 {
        0x88fe
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_BUFFER_MODE")]
      pub fn TRANSFORM_FEEDBACK_BUFFER_MODE(&self) -> u32 {
        0x8c7f
      }

      #[napi(getter, js_name = "MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS")]
      pub fn MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS(&self) -> u32 {
        0x8c80
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_VARYINGS")]
      pub fn TRANSFORM_FEEDBACK_VARYINGS(&self) -> u32 {
        0x8c83
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_BUFFER_START")]
      pub fn TRANSFORM_FEEDBACK_BUFFER_START(&self) -> u32 {
        0x8c84
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_BUFFER_SIZE")]
      pub fn TRANSFORM_FEEDBACK_BUFFER_SIZE(&self) -> u32 {
        0x8c85
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN")]
      pub fn TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN(&self) -> u32 {
        0x8c88
      }

      #[napi(getter, js_name = "MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS")]
      pub fn MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS(&self) -> u32 {
        0x8c8a
      }

      #[napi(getter, js_name = "MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS")]
      pub fn MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS(&self) -> u32 {
        0x8c8b
      }

      #[napi(getter, js_name = "INTERLEAVED_ATTRIBS")]
      pub fn INTERLEAVED_ATTRIBS(&self) -> u32 {
        0x8c8c
      }

      #[napi(getter, js_name = "SEPARATE_ATTRIBS")]
      pub fn SEPARATE_ATTRIBS(&self) -> u32 {
        0x8c8d
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_BUFFER")]
      pub fn TRANSFORM_FEEDBACK_BUFFER(&self) -> u32 {
        0x8c8e
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_BUFFER_BINDING")]
      pub fn TRANSFORM_FEEDBACK_BUFFER_BINDING(&self) -> u32 {
        0x8c8f
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK")]
      pub fn TRANSFORM_FEEDBACK(&self) -> u32 {
        0x8e22
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_PAUSED")]
      pub fn TRANSFORM_FEEDBACK_PAUSED(&self) -> u32 {
        0x8e23
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_ACTIVE")]
      pub fn TRANSFORM_FEEDBACK_ACTIVE(&self) -> u32 {
        0x8e24
      }

      #[napi(getter, js_name = "TRANSFORM_FEEDBACK_BINDING")]
      pub fn TRANSFORM_FEEDBACK_BINDING(&self) -> u32 {
        0x8e25
      }

      /* Pixel types */

      /* Queries */

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING")]
      pub fn FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING(&self) -> u32 {
        0x8210
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE")]
      pub fn FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE(&self) -> u32 {
        0x8211
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_RED_SIZE")]
      pub fn FRAMEBUFFER_ATTACHMENT_RED_SIZE(&self) -> u32 {
        0x8212
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_GREEN_SIZE")]
      pub fn FRAMEBUFFER_ATTACHMENT_GREEN_SIZE(&self) -> u32 {
        0x8213
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_BLUE_SIZE")]
      pub fn FRAMEBUFFER_ATTACHMENT_BLUE_SIZE(&self) -> u32 {
        0x8214
      }
      /* Queries */

      /* Draw buffers */

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE")]
      pub fn FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE(&self) -> u32 {
        0x8215
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE")]
      pub fn FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE(&self) -> u32 {
        0x8216
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE")]
      pub fn FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE(&self) -> u32 {
        0x8217
      }

      #[napi(getter, js_name = "FRAMEBUFFER_DEFAULT")]
      pub fn FRAMEBUFFER_DEFAULT(&self) -> u32 {
        0x8218
      }

      #[napi(getter, js_name = "DEPTH24_STENCIL8")]
      pub fn DEPTH24_STENCIL8(&self) -> u32 {
        0x88f0
      }

      #[napi(getter, js_name = "DRAW_FRAMEBUFFER_BINDING")]
      pub fn DRAW_FRAMEBUFFER_BINDING(&self) -> u32 {
        0x8ca6
      }

      #[napi(getter, js_name = "READ_FRAMEBUFFER")]
      pub fn READ_FRAMEBUFFER(&self) -> u32 {
        0x8ca8
      }

      #[napi(getter, js_name = "DRAW_FRAMEBUFFER")]
      pub fn DRAW_FRAMEBUFFER(&self) -> u32 {
        0x8ca9
      }

      #[napi(getter, js_name = "READ_FRAMEBUFFER_BINDING")]
      pub fn READ_FRAMEBUFFER_BINDING(&self) -> u32 {
        0x8caa
      }

      #[napi(getter, js_name = "RENDERBUFFER_SAMPLES")]
      pub fn RENDERBUFFER_SAMPLES(&self) -> u32 {
        0x8cab
      }

      #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER")]
      pub fn FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER(&self) -> u32 {
        0x8cd4
      }

      #[napi(getter, js_name = "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE")]
      pub fn FRAMEBUFFER_INCOMPLETE_MULTISAMPLE(&self) -> u32 {
        0x8d56
      }

      #[napi(getter, js_name = "UNIFORM_BUFFER")]
      pub fn UNIFORM_BUFFER(&self) -> u32 {
        0x8a11
      }

      #[napi(getter, js_name = "UNIFORM_BUFFER_BINDING")]
      pub fn UNIFORM_BUFFER_BINDING(&self) -> u32 {
        0x8a28
      }

      #[napi(getter, js_name = "UNIFORM_BUFFER_START")]
      pub fn UNIFORM_BUFFER_START(&self) -> u32 {
        0x8a29
      }

      #[napi(getter, js_name = "UNIFORM_BUFFER_SIZE")]
      pub fn UNIFORM_BUFFER_SIZE(&self) -> u32 {
        0x8a2a
      }

      #[napi(getter, js_name = "MAX_VERTEX_UNIFORM_BLOCKS")]
      pub fn MAX_VERTEX_UNIFORM_BLOCKS(&self) -> u32 {
        0x8a2b
      }

      #[napi(getter, js_name = "MAX_FRAGMENT_UNIFORM_BLOCKS")]
      pub fn MAX_FRAGMENT_UNIFORM_BLOCKS(&self) -> u32 {
        0x8a2d
      }

      #[napi(getter, js_name = "MAX_COMBINED_UNIFORM_BLOCKS")]
      pub fn MAX_COMBINED_UNIFORM_BLOCKS(&self) -> u32 {
        0x8a2e
      }

      #[napi(getter, js_name = "MAX_UNIFORM_BUFFER_BINDINGS")]
      pub fn MAX_UNIFORM_BUFFER_BINDINGS(&self) -> u32 {
        0x8a2f
      }

      #[napi(getter, js_name = "MAX_UNIFORM_BLOCK_SIZE")]
      pub fn MAX_UNIFORM_BLOCK_SIZE(&self) -> u32 {
        0x8a30
      }

      #[napi(getter, js_name = "MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS")]
      pub fn MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS(&self) -> u32 {
        0x8a31
      }

      #[napi(getter, js_name = "MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS")]
      pub fn MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS(&self) -> u32 {
        0x8a33
      }

      #[napi(getter, js_name = "UNIFORM_BUFFER_OFFSET_ALIGNMENT")]
      pub fn UNIFORM_BUFFER_OFFSET_ALIGNMENT(&self) -> u32 {
        0x8a34
      }

      #[napi(getter, js_name = "ACTIVE_UNIFORM_BLOCKS")]
      pub fn ACTIVE_UNIFORM_BLOCKS(&self) -> u32 {
        0x8a36
      }

      #[napi(getter, js_name = "UNIFORM_TYPE")]
      pub fn UNIFORM_TYPE(&self) -> u32 {
        0x8a37
      }

      #[napi(getter, js_name = "UNIFORM_SIZE")]
      pub fn UNIFORM_SIZE(&self) -> u32 {
        0x8a38
      }

      #[napi(getter, js_name = "UNIFORM_BLOCK_INDEX")]
      pub fn UNIFORM_BLOCK_INDEX(&self) -> u32 {
        0x8a3a
      }

      #[napi(getter, js_name = "UNIFORM_OFFSET")]
      pub fn UNIFORM_OFFSET(&self) -> u32 {
        0x8a3b
      }

      #[napi(getter, js_name = "UNIFORM_ARRAY_STRIDE")]
      pub fn UNIFORM_ARRAY_STRIDE(&self) -> u32 {
        0x8a3c
      }

      #[napi(getter, js_name = "UNIFORM_MATRIX_STRIDE")]
      pub fn UNIFORM_MATRIX_STRIDE(&self) -> u32 {
        0x8a3d
      }

      /* Draw buffers */

      /* Samplers */

      #[napi(getter, js_name = "UNIFORM_IS_ROW_MAJOR")]
      pub fn UNIFORM_IS_ROW_MAJOR(&self) -> u32 {
        0x8a3e
      }

      #[napi(getter, js_name = "UNIFORM_BLOCK_BINDING")]
      pub fn UNIFORM_BLOCK_BINDING(&self) -> u32 {
        0x8a3f
      }

      #[napi(getter, js_name = "UNIFORM_BLOCK_DATA_SIZE")]
      pub fn UNIFORM_BLOCK_DATA_SIZE(&self) -> u32 {
        0x8a40
      }

      #[napi(getter, js_name = "UNIFORM_BLOCK_ACTIVE_UNIFORMS")]
      pub fn UNIFORM_BLOCK_ACTIVE_UNIFORMS(&self) -> u32 {
        0x8a42
      }

      #[napi(getter, js_name = "UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES")]
      pub fn UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES(&self) -> u32 {
        0x8a43
      }

      #[napi(getter, js_name = "UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER")]
      pub fn UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER(&self) -> u32 {
        0x8a44
      }

      #[napi(getter, js_name = "UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER")]
      pub fn UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER(&self) -> u32 {
        0x8a46
      }

      #[napi(getter, js_name = "OBJECT_TYPE")]
      pub fn OBJECT_TYPE(&self) -> u32 {
        0x9112
      }

      #[napi(getter, js_name = "SYNC_CONDITION")]
      pub fn SYNC_CONDITION(&self) -> u32 {
        0x9113
      }

      #[napi(getter, js_name = "SYNC_STATUS")]
      pub fn SYNC_STATUS(&self) -> u32 {
        0x9114
      }

      #[napi(getter, js_name = "SYNC_FLAGS")]
      pub fn SYNC_FLAGS(&self) -> u32 {
        0x9115
      }

      #[napi(getter, js_name = "SYNC_FENCE")]
      pub fn SYNC_FENCE(&self) -> u32 {
        0x9116
      }

      #[napi(getter, js_name = "SYNC_GPU_COMMANDS_COMPLETE")]
      pub fn SYNC_GPU_COMMANDS_COMPLETE(&self) -> u32 {
        0x9117
      }

      #[napi(getter, js_name = "UNSIGNALED")]
      pub fn UNSIGNALED(&self) -> u32 {
        0x9118
      }

      #[napi(getter, js_name = "SIGNALED")]
      pub fn SIGNALED(&self) -> u32 {
        0x9119
      }

      /* Samplers */

      /* Buffers */

      #[napi(getter, js_name = "ALREADY_SIGNALED")]
      pub fn ALREADY_SIGNALED(&self) -> u32 {
        0x911a
      }

      #[napi(getter, js_name = "TIMEOUT_EXPIRED")]
      pub fn TIMEOUT_EXPIRED(&self) -> u32 {
        0x911b
      }

      #[napi(getter, js_name = "CONDITION_SATISFIED")]
      pub fn CONDITION_SATISFIED(&self) -> u32 {
        0x911c
      }

      #[napi(getter, js_name = "WAIT_FAILED")]
      pub fn WAIT_FAILED(&self) -> u32 {
        0x911d
      }

      #[napi(getter, js_name = "SYNC_FLUSH_COMMANDS_BIT")]
      pub fn SYNC_FLUSH_COMMANDS_BIT(&self) -> u32 {
        0x00000001
      }

      #[napi(getter, js_name = "COLOR")]
      pub fn COLOR(&self) -> u32 {
        0x1800
      }

      #[napi(getter, js_name = "DEPTH")]
      pub fn DEPTH(&self) -> u32 {
        0x1801
      }

      #[napi(getter, js_name = "STENCIL")]
      pub fn STENCIL(&self) -> u32 {
        0x1802
      }

      /* Buffers */

      /* Data types */

      #[napi(getter, js_name = "MIN")]
      pub fn MIN(&self) -> u32 {
        return 0x8007;
      }

      #[napi(getter, js_name = "MAX")]
      pub fn MAX(&self) -> u32 {
        return 0x8008;
      }

      #[napi(getter, js_name = "DEPTH_COMPONENT24")]
      pub fn DEPTH_COMPONENT24(&self) -> u32 {
        return 0x81a6;
      }

      #[napi(getter, js_name = "STREAM_READ")]
      pub fn STREAM_READ(&self) -> u32 {
        return 0x88e1;
      }

      #[napi(getter, js_name = "STREAM_COPY")]
      pub fn STREAM_COPY(&self) -> u32 {
        return 0x88e2;
      }

      #[napi(getter, js_name = "STATIC_READ")]
      pub fn STATIC_READ(&self) -> u32 {
        return 0x88e5;
      }

      #[napi(getter, js_name = "STATIC_COPY")]
      pub fn STATIC_COPY(&self) -> u32 {
        return 0x88e6;
      }


      #[napi(getter, js_name = "DYNAMIC_READ")]
      pub fn DYNAMIC_READ(&self) -> u32 {
        return 0x88e9;
      }

      #[napi(getter, js_name = "DYNAMIC_COPY")]
      pub fn DYNAMIC_COPY(&self) -> u32 {
        return 0x88ea;
      }

      #[napi(getter, js_name = "DEPTH_COMPONENT32F")]
      pub fn DEPTH_COMPONENT32F(&self) -> u32 {
        return 0x8cac;
      }

      #[napi(getter, js_name = "DEPTH32F_STENCIL8")]
      pub fn DEPTH32F_STENCIL8(&self) -> u32 {
        return 0x8cad;
      }

      /* Data types */

      #[napi(getter, js_name = "INVALID_INDEX")]
      pub fn INVALID_INDEX(&self) -> u32 {
        return 0xffffffff;
      }

      #[napi(getter, js_name = "TIMEOUT_IGNORED")]
      pub fn TIMEOUT_IGNORED(&self) -> i32 {
        return -1;
      }

      /* Vertex attributes */

      /* Transform feedback */

      #[napi(getter, js_name = "MAX_CLIENT_WAIT_TIMEOUT_WEBGL")]
      pub fn MAX_CLIENT_WAIT_TIMEOUT_WEBGL(&self) -> u32 {
        return 0x9247;
      }
    }
  };
}
