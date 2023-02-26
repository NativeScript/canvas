//
// Created by Osei Fortune on 25/02/2023.
//

#include "CanvasJSIModule.h"
#include "v8runtime/JSIV8ValueConverter.h"

#include "canvas2d/CanvasRenderingContext2DImpl.h"
void CanvasJSIModule::install(facebook::jsi::Runtime &jsiRuntime) {
    auto canvas_module = facebook::jsi::Object(jsiRuntime);

    CREATE_FUNC("create2DContext", canvas_module, 3,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {
                    auto width = (float) arguments[0].asNumber();
                    auto height = (float) arguments[1].asNumber();
                    auto density = (float) arguments[2].asNumber();
                    auto context = getPointerValue(arguments[3], runtime);
                    auto samples = (int) arguments[4].asNumber();
                    auto alpha = (bool) arguments[5].asBool();
                    auto font_color = (int) arguments[6].asNumber();
                    auto ppi = (float) arguments[7].asNumber();
                    auto direction = (int) arguments[7].asNumber();
                    auto context_2d = canvas_native_context_create_gl(width, height, density, context, samples, alpha,
                                                    font_color, ppi, direction);

                    auto obj = CanvasRenderingContext2DImpl(std::move(context_2d));

                }

    );
}
