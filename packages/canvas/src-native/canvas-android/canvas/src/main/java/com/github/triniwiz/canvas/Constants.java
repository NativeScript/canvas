package com.github.triniwiz.canvas;

import android.opengl.GLES11Ext;

/**
 * Created by triniwiz on 5/1/20
 */
public class Constants {
    public static int GL_RGBA16F_EXT = 0x881A;
    public static int GL_RGB16F_EXT = 0x881B;
    public static int GL_RG16F_EXT = 0x822F;
    public static int GL_R16F_EXT = 0x822D;
    public static int GL_R32F_EXT = 0x822E;
    public static int GL_RG32F_EXT = 0x8230;
    public static int GL_RGBA32F_EXT = 0x8814;
    public static int GL_RGB32F_EXT = 0x8815;
    public static int GL_MIN_EXT = 0x8007;
    public static int GL_MAX_EXT = 0x8008;
    public static int GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT = 0x8211;
    public static int GL_UNSIGNED_NORMALIZED_EXT = 0x8C17;

    public static int GL_SRGB_EXT = 0x8C40;
    public static int GL_SRGB_ALPHA_EXT = 0x8C42;
    public static int GL_SRGB8_ALPHA8_EXT = 0x8C43;
    public static int GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT = 0x8210;

    public static int GL_TEXTURE_MAX_ANISOTROPY_EXT = 0x84FE;
    public static int GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT = 0x84FF;


    public static int GL_QUERY_COUNTER_BITS_EXT = 0x8864;
    public static int GL_CURRENT_QUERY_EXT = 0x8865;
    public static int GL_QUERY_RESULT_EXT = 0x8866;
    public static int GL_QUERY_RESULT_AVAILABLE_EXT = 0x8867;
    public static int GL_TIME_ELAPSED_EXT = 0x88BF;
    public static int GL_TIMESTAMP_EXT = 0x8E28;
    public static int GL_GPU_DISJOINT_EXT = 0x8FBB;

    public static int GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES = 0x8B8B;

    public static int GL_HALF_FLOAT_OES = 0x8D61;
    public static int GL_VERTEX_ARRAY_BINDING_OES = 0x85B5;

    public static int COMPRESSED_RGB_ATC_WEBGL = GLES11Ext.GL_ATC_RGB_AMD;
    public static int COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL = GLES11Ext.GL_ATC_RGBA_EXPLICIT_ALPHA_AMD;
    public static int COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL = GLES11Ext.GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD;


    public static int GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG = 0x8C00;
    public static int GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG = 0x8C01;
    public static int GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG = 0x8C02;
    public static int GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG = 0x8C03;


    public static int GL_COMPRESSED_RGB_S3TC_DXT1_EXT = 0x83F0;
    public static int GL_COMPRESSED_RGBA_S3TC_DXT1_EXT = 0x83F1;

    public static int GL_COMPRESSED_RGBA_S3TC_DXT3_EXT = 0x83F2;
    public static int GL_COMPRESSED_RGBA_S3TC_DXT5_EXT = 0x83F3;

    public static int GL_COMPRESSED_SRGB_S3TC_DXT1_EXT = 0x8C4C;
    public static int GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT = 0x8C4D;
    public static int GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT = 0x8C4E;
    public static int GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT = 0x8C4F;


    public static int GL_MAX_COLOR_ATTACHMENTS_EXT = 0x8CDF;
    public static int GL_MAX_DRAW_BUFFERS_EXT = 0x8824;
    public static int GL_DRAW_BUFFER0_EXT = 0x8825;
    public static int GL_DRAW_BUFFER1_EXT = 0x8826;
    public static int GL_DRAW_BUFFER2_EXT = 0x8827;
    public static int GL_DRAW_BUFFER3_EXT = 0x8828;
    public static int GL_DRAW_BUFFER4_EXT = 0x8829;
    public static int GL_DRAW_BUFFER5_EXT = 0x882A;
    public static int GL_DRAW_BUFFER6_EXT = 0x882B;
    public static int GL_DRAW_BUFFER7_EXT = 0x882C;
    public static int GL_DRAW_BUFFER8_EXT = 0x882D;
    public static int GL_DRAW_BUFFER9_EXT = 0x882E;
    public static int GL_DRAW_BUFFER10_EXT = 0x882F;
    public static int GL_DRAW_BUFFER11_EXT = 0x8830;
    public static int GL_DRAW_BUFFER12_EXT = 0x8831;
    public static int GL_DRAW_BUFFER13_EXT = 0x8832;
    public static int GL_DRAW_BUFFER14_EXT = 0x8833;
    public static int GL_DRAW_BUFFER15_EXT = 0x8834;
    public static int GL_COLOR_ATTACHMENT0_EXT = 0x8CE0;
    public static int GL_COLOR_ATTACHMENT1_EXT = 0x8CE1;
    public static int GL_COLOR_ATTACHMENT2_EXT = 0x8CE2;
    public static int GL_COLOR_ATTACHMENT3_EXT = 0x8CE3;
    public static int GL_COLOR_ATTACHMENT4_EXT = 0x8CE4;
    public static int GL_COLOR_ATTACHMENT5_EXT = 0x8CE5;
    public static int GL_COLOR_ATTACHMENT6_EXT = 0x8CE6;
    public static int GL_COLOR_ATTACHMENT7_EXT = 0x8CE7;
    public static int GL_COLOR_ATTACHMENT8_EXT = 0x8CE8;
    public static int GL_COLOR_ATTACHMENT9_EXT = 0x8CE9;
    public static int GL_COLOR_ATTACHMENT10_EXT = 0x8CEA;
    public static int GL_COLOR_ATTACHMENT11_EXT = 0x8CEB;
    public static int GL_COLOR_ATTACHMENT12_EXT = 0x8CEC;
    public static int GL_COLOR_ATTACHMENT13_EXT = 0x8CED;
    public static int GL_COLOR_ATTACHMENT14_EXT = 0x8CEE;
    public static int GL_COLOR_ATTACHMENT15_EXT = 0x8CEF;
    public static int MAX_CLIENT_WAIT_TIMEOUT_WEBGL = 0x9247;
}
