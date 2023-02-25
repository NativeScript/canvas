//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "Common.h"
#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "TextEncoderImpl.h"
#include "TextDecoderImpl.h"
#include "canvas2d/Canvas2D.h"
#include "./webgl/WebGL.h"
#include "./webgl2/WebGL2.h"

using V8FunctionCallbackInfo = v8::FunctionCallbackInfo<v8::Value>;

class Bridge {};

void Init(const v8::FunctionCallbackInfo<v8::Value> &args);
