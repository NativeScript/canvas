//
// Created by Osei Fortune on 04/03/2023.
//

#pragma once

#include "canvas-cxx/src/lib.rs.h"

#ifdef __ANDROID__

#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>
#include "gles3jni.h"

#endif

#ifdef __APPLE__
#include <OpenGLES/ES2/gl.h>
#include <OpenGLES/ES3/gl.h>
#endif

