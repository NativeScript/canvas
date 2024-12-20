//
// Created by Osei Fortune on 04/03/2023.
//

#pragma once


#ifdef __ANDROID__
#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>
#include "gles3jni.h"
#endif

#ifdef __APPLE__
#include <OpenGLES/ES1/gl.h>
#include <OpenGLES/ES1/glext.h>
#include <OpenGLES/ES2/gl.h>
#include <OpenGLES/ES2/glext.h>
#include <OpenGLES/ES3/gl.h>
#include <OpenGLES/ES3/glext.h>

#define GL_COMPRESSED_RGB_S3TC_DXT1_EXT 0x83F0
#define GL_COMPRESSED_RGBA_S3TC_DXT1_EXT 0x83F1
#define GL_COMPRESSED_RGBA_S3TC_DXT3_EXT 0x83F2
#define GL_COMPRESSED_RGBA_S3TC_DXT5_EXT 0x83F3

#define GL_COMPRESSED_SRGB_S3TC_DXT1_EXT  0x8C4C
#define GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT 0x8C4D
#define GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT 0x8C4E
#define GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT 0x8C4F

#define GL_ATC_RGB_AMD                    0x8C92
#define GL_ATC_RGBA_EXPLICIT_ALPHA_AMD    0x8C93
#define GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD 0x87EE



#define GL_COLOR_ATTACHMENT0_EXT GL_COLOR_ATTACHMENT0
#define GL_COLOR_ATTACHMENT1_EXT GL_COLOR_ATTACHMENT1
#define GL_COLOR_ATTACHMENT2_EXT GL_COLOR_ATTACHMENT2
#define GL_COLOR_ATTACHMENT3_EXT GL_COLOR_ATTACHMENT3
#define GL_COLOR_ATTACHMENT4_EXT GL_COLOR_ATTACHMENT4
#define GL_COLOR_ATTACHMENT5_EXT GL_COLOR_ATTACHMENT5
#define GL_COLOR_ATTACHMENT6_EXT GL_COLOR_ATTACHMENT6
#define GL_COLOR_ATTACHMENT7_EXT GL_COLOR_ATTACHMENT7
#define GL_COLOR_ATTACHMENT8_EXT GL_COLOR_ATTACHMENT8
#define GL_COLOR_ATTACHMENT9_EXT GL_COLOR_ATTACHMENT9
#define GL_COLOR_ATTACHMENT10_EXT GL_COLOR_ATTACHMENT10
#define GL_COLOR_ATTACHMENT11_EXT GL_COLOR_ATTACHMENT11
#define GL_COLOR_ATTACHMENT12_EXT GL_COLOR_ATTACHMENT12
#define GL_COLOR_ATTACHMENT13_EXT GL_COLOR_ATTACHMENT13
#define GL_COLOR_ATTACHMENT14_EXT GL_COLOR_ATTACHMENT14
#define GL_COLOR_ATTACHMENT15_EXT GL_COLOR_ATTACHMENT15


#define GL_DRAW_BUFFER0_EXT GL_DRAW_BUFFER0
#define GL_DRAW_BUFFER1_EXT GL_DRAW_BUFFER1
#define GL_DRAW_BUFFER2_EXT GL_DRAW_BUFFER2
#define GL_DRAW_BUFFER3_EXT GL_DRAW_BUFFER3
#define GL_DRAW_BUFFER4_EXT GL_DRAW_BUFFER4
#define GL_DRAW_BUFFER5_EXT GL_DRAW_BUFFER5
#define GL_DRAW_BUFFER6_EXT GL_DRAW_BUFFER6
#define GL_DRAW_BUFFER7_EXT GL_DRAW_BUFFER7
#define GL_DRAW_BUFFER8_EXT GL_DRAW_BUFFER8
#define GL_DRAW_BUFFER9_EXT GL_DRAW_BUFFER9
#define GL_DRAW_BUFFER10_EXT GL_DRAW_BUFFER10
#define GL_DRAW_BUFFER11_EXT GL_DRAW_BUFFER11
#define GL_DRAW_BUFFER12_EXT GL_DRAW_BUFFER12
#define GL_DRAW_BUFFER13_EXT GL_DRAW_BUFFER13
#define GL_DRAW_BUFFER14_EXT GL_DRAW_BUFFER14
#define GL_DRAW_BUFFER15_EXT GL_DRAW_BUFFER15

#define GL_MAX_COLOR_ATTACHMENTS_EXT GL_MAX_COLOR_ATTACHMENTS
#define GL_MAX_DRAW_BUFFERS_EXT GL_MAX_DRAW_BUFFERS



#endif

