//
// Created by Osei Fortune on 25/02/2023.
//

#include "CanvasJSIModule.h"
#include "v8runtime/JSIV8ValueConverter.h"

void CanvasJSIModule::install(facebook::jsi::Runtime &jsiRuntime) {
    CREATE_FUNC("__Canvas_", 3,
                [](Runtime &runtime, const Value &thisValue,
                        const Value *arguments, size_t count) -> Value {});
}
