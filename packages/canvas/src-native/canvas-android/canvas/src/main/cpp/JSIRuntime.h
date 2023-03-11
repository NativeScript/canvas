//
// Created by Osei Fortune on 14/12/2022.
//

#include "CanvasJSIModule.h"

std::shared_ptr<rnv8::V8Runtime> jsi_runtime;

extern "C" void NSMain(const v8::FunctionCallbackInfo <v8::Value> &args);
