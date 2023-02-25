//
// Created by Osei Fortune on 14/12/2022.
//

#ifndef MASON_NATIVE_MASONJSIMODULE_H
#define MASON_NATIVE_MASONJSIMODULE_H

#include "mason-android/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"
#include <sstream>

using namespace facebook::jsi;
using namespace std;


template<typename NativeFunc>
static void createFunc(Runtime &jsiRuntime, const char *prop, int paramCount, NativeFunc &&func) {
    auto f = Function::createFromHostFunction(jsiRuntime,
                                              PropNameID::forAscii(jsiRuntime, prop),
                                              paramCount,
                                              std::forward<NativeFunc>(func));
    jsiRuntime.global().setProperty(jsiRuntime, prop, std::move(f));
}

#define CREATE_FUNC(prop, paramCount, func) \
    createFunc(jsiRuntime, prop, paramCount, func)

inline static int64_t getPointerValue(const facebook::jsi::Value &value, Runtime &runtime) {
    return value.asBigInt(runtime).Int64Value(runtime);
}

inline static Value gridPlacementToJS(Runtime &runtime, CMasonGridPlacement placement) {
    switch (placement.value_type) {
        case CMasonGridPlacementType::Auto: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", (int) placement.value);
            ret.setProperty(runtime, "type", 0);
            return ret;
        }
        case CMasonGridPlacementType::Line: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", (int) placement.value);
            ret.setProperty(runtime, "type", 1);
            return ret;
        }
        case CMasonGridPlacementType::Span: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", (int) placement.value);
            ret.setProperty(runtime, "type", 2);
            return ret;
        }
        default:
            return Value::undefined();
    }
}

inline static CMasonGridPlacement jsToGridPlacement(short value, int value_type) {
    switch ((CMasonGridPlacementType) value_type) {
        case CMasonGridPlacementType::Auto: {
            return CMasonGridPlacement{0, CMasonGridPlacementType::Auto};
        }
        case CMasonGridPlacementType::Line: {
            return CMasonGridPlacement{value, CMasonGridPlacementType::Line};
        }
        case CMasonGridPlacementType::Span: {
            return CMasonGridPlacement{value, CMasonGridPlacementType::Span};
        }
        default:
            // assert ??
            return CMasonGridPlacement{0, CMasonGridPlacementType::Auto};
    }
}

inline static Value lengthPercentageToJS(Runtime &runtime, CMasonLengthPercentage length) {
    switch (length.value_type) {
        case CMasonLengthPercentageType::Percent: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", length.value / 100);
            ret.setProperty(runtime, "unit", "%");
            return ret;
        }
        case CMasonLengthPercentageType::Points: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", length.value);
            ret.setProperty(runtime, "unit", "px");
            return ret;
        }
        default:
            return Value::undefined();
    }
}

inline static CMasonLengthPercentage jsToLengthPercentage(float value, int value_type) {
    switch ((CMasonLengthPercentageType) value_type) {
        case CMasonLengthPercentageType::Percent: {
            return CMasonLengthPercentage{value, CMasonLengthPercentageType::Percent};
        }
        case CMasonLengthPercentageType::Points: {
            return CMasonLengthPercentage{value, CMasonLengthPercentageType::Points};
        }
        default:
             // assert ??
            return CMasonLengthPercentage{0, CMasonLengthPercentageType::Points};
    }
}

inline static CMasonLengthPercentageType jsToLengthPercentageType(int value_type) {
    switch ((CMasonLengthPercentageType) value_type) {
        case CMasonLengthPercentageType::Percent: {
            return CMasonLengthPercentageType::Percent;
        }
        case CMasonLengthPercentageType::Points: {
            return CMasonLengthPercentageType::Points;
        }
        default:
            // assert invalid type ???
            return CMasonLengthPercentageType::Points;
    }
}

inline static Value lengthPercentageAutoToJS(Runtime &runtime, CMasonLengthPercentageAuto length) {
    switch (length.value_type) {
        case CMasonLengthPercentageAutoType::Auto:
            return facebook::jsi::String::createFromUtf8(runtime, "auto");
        case CMasonLengthPercentageAutoType::Percent: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", length.value / 100);
            ret.setProperty(runtime, "unit", "%");
            return ret;
        }
        case CMasonLengthPercentageAutoType::Points: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", length.value);
            ret.setProperty(runtime, "unit", "px");
            return ret;
        }
        default:
            return Value::undefined();
    }
}

inline static CMasonLengthPercentageAuto jsToLengthPercentageAuto(float value, int value_type) {
    switch ((CMasonLengthPercentageAutoType) value_type) {
        case CMasonLengthPercentageAutoType::Auto:
            return CMasonLengthPercentageAuto{value, CMasonLengthPercentageAutoType::Auto};
        case CMasonLengthPercentageAutoType::Percent: {
            return CMasonLengthPercentageAuto{value, CMasonLengthPercentageAutoType::Percent};
        }
        case CMasonLengthPercentageAutoType::Points: {
            return CMasonLengthPercentageAuto{value, CMasonLengthPercentageAutoType::Points};
        }
        default:
            // assert invalid type ???
            return CMasonLengthPercentageAuto{0, CMasonLengthPercentageAutoType::Points};
    }
}

inline static CMasonLengthPercentageAutoType jsToLengthPercentageAutoType(int value_type) {
    switch ((CMasonLengthPercentageAutoType) value_type) {
        case CMasonLengthPercentageAutoType::Auto:
            return CMasonLengthPercentageAutoType::Auto;
        case CMasonLengthPercentageAutoType::Percent: {
            return CMasonLengthPercentageAutoType::Percent;
        }
        case CMasonLengthPercentageAutoType::Points: {
            return CMasonLengthPercentageAutoType::Points;
        }
        default:
            // assert invalid type ???
            return CMasonLengthPercentageAutoType::Auto;
    }
}

inline static Value dimensionToJS(Runtime &runtime, CMasonDimension dimension) {
    switch ((CMasonDimensionType) dimension.value_type) {
        case CMasonDimensionType::Auto:
            return facebook::jsi::String::createFromUtf8(runtime, "auto");
        case CMasonDimensionType::Percent: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", dimension.value / 100);
            ret.setProperty(runtime, "unit", "%");
            return ret;
        }
        case CMasonDimensionType::Points: {
            auto ret = facebook::jsi::Object(runtime);
            ret.setProperty(runtime, "value", dimension.value);
            ret.setProperty(runtime, "unit", "px");
            return ret;
        }
        default:
            return Value::undefined();
    }
}

inline static CMasonDimension jsToDimension(float value, int value_type) {
    switch ((CMasonDimensionType) value_type) {
        case CMasonDimensionType::Auto:
            return CMasonDimension{value, CMasonDimensionType::Auto};
        case CMasonDimensionType::Percent: {
            return CMasonDimension{value, CMasonDimensionType::Percent};
        }
        case CMasonDimensionType::Points: {
            return CMasonDimension{value, CMasonDimensionType::Points};
        }
        default:
            // assert invalid type ???
            return CMasonDimension{0, CMasonDimensionType::Points};
    }
}

inline static CMasonDimensionType jsToDimensionType(int value_type) {
    switch ((CMasonDimensionType) value_type) {
        case CMasonDimensionType::Auto:
            return CMasonDimensionType::Auto;
        case CMasonDimensionType::Percent: {
            return CMasonDimensionType::Percent;
        }
        case CMasonDimensionType::Points: {
            return CMasonDimensionType::Points;
        }
        default:
            // assert invalid type ???
            return CMasonDimensionType::Auto;
    }
}

inline static Value sizeToJS(Runtime &runtime, CMasonDimensionSize size) {
    auto ret = facebook::jsi::Object(runtime);
    ret.setProperty(runtime, "width", dimensionToJS(runtime, size.width));
    ret.setProperty(runtime, "height", dimensionToJS(runtime, size.height));
    return ret;
}

inline static Value sizeToJS(Runtime &runtime, CMasonLengthPercentageSize size) {
    auto ret = facebook::jsi::Object(runtime);
    ret.setProperty(runtime, "width", lengthPercentageToJS(runtime, size.width));
    ret.setProperty(runtime, "height", lengthPercentageToJS(runtime, size.height));
    return ret;
}

inline static Value sizeToJS(Runtime &runtime, CMasonLengthPercentageAutoSize size) {
    auto ret = facebook::jsi::Object(runtime);
    ret.setProperty(runtime, "width", lengthPercentageAutoToJS(runtime, size.width));
    ret.setProperty(runtime, "height", lengthPercentageAutoToJS(runtime, size.height));
    return ret;
}

inline static Value rectToJS(Runtime &runtime, CMasonDimensionRect rect) {
    auto ret = facebook::jsi::Object(runtime);
    ret.setProperty(runtime, "left", dimensionToJS(runtime, rect.left));
    ret.setProperty(runtime, "right", dimensionToJS(runtime, rect.right));
    ret.setProperty(runtime, "top", dimensionToJS(runtime, rect.top));
    ret.setProperty(runtime, "bottom", dimensionToJS(runtime, rect.bottom));
    return ret;
}

inline static Value rectToJS(Runtime &runtime, CMasonLengthPercentageRect rect) {
    auto ret = facebook::jsi::Object(runtime);
    ret.setProperty(runtime, "left", lengthPercentageToJS(runtime, rect.left));
    ret.setProperty(runtime, "right", lengthPercentageToJS(runtime, rect.right));
    ret.setProperty(runtime, "top", lengthPercentageToJS(runtime, rect.top));
    ret.setProperty(runtime, "bottom", lengthPercentageToJS(runtime, rect.bottom));
    return ret;
}

inline static Value rectToJS(Runtime &runtime, CMasonLengthPercentageAutoRect rect) {
    auto ret = facebook::jsi::Object(runtime);
    ret.setProperty(runtime, "left", lengthPercentageAutoToJS(runtime, rect.left));
    ret.setProperty(runtime, "right", lengthPercentageAutoToJS(runtime, rect.right));
    ret.setProperty(runtime, "top", lengthPercentageAutoToJS(runtime, rect.top));
    ret.setProperty(runtime, "bottom", lengthPercentageAutoToJS(runtime, rect.bottom));
    return ret;
}





class MasonJSIModule {
public:
    static void install(facebook::jsi::Runtime &rt);
};


#endif //MASON_NATIVE_MASONJSIMODULE_H
