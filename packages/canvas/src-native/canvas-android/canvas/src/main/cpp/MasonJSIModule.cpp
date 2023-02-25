//
// Created by Osei Fortune on 14/12/2022.
//

#include "MasonJSIModule.h"
#include <android/log.h>
#include "v8runtime/JSIV8ValueConverter.h"

using namespace facebook::jsi;
using namespace std;


rust::Vec<CMasonNonRepeatedTrackSizingFunction>
toNonRepeatedTrackSizingFunction(facebook::jsi::Runtime &runtime, facebook::jsi::Array &array) {
    auto len = array.length(runtime);

    auto auto_vec = rust::Vec<CMasonNonRepeatedTrackSizingFunction>();

    if (len == 0) {
        return std::move(auto_vec);
    }

    auto_vec.reserve(len);

    for (int i = 0; i < len; i++) {
        auto value = array.getValueAtIndex(runtime, i).asObject(
                runtime);

        // object {type: number, min_type: number, min_value: number, max_type: number, max_value: number};
        auto min_type = (int) value.getProperty(runtime, "min_type").getNumber();
        auto min_value = (float) value.getProperty(runtime, "min_value").getNumber();

        auto max_type = (int) value.getProperty(runtime, "max_type").getNumber();
        auto max_value = (float) value.getProperty(runtime, "max_value").getNumber();

        CMasonMinMax minMax = {};
        minMax.min_type = min_type;
        minMax.min_value = min_value;
        minMax.max_type = max_type;
        minMax.max_value = max_value;

        mason_util_create_non_repeated_track_sizing_function(minMax, -1, auto_vec);
    }

    return std::move(auto_vec);
}


rust::Vec<CMasonTrackSizingFunction>
toTrackSizingFunction(facebook::jsi::Runtime &runtime, facebook::jsi::Array &array) {
    auto len = array.length(runtime);

    auto auto_vec = rust::Vec<CMasonTrackSizingFunction>();

    if (len == 0) {
        return std::move(auto_vec);
    }

    auto_vec.reserve(len);

    for (int i = 0; i < len; i++) {
        auto object = array.getValueAtIndex(runtime, i).asObject(runtime);
        bool is_repeating = object.getProperty(runtime, "is_repeating").asBool();
        auto repeating_type = (int) object.getProperty(runtime, "repeating_type").asNumber();
        auto repeating_count = (short) object.getProperty(runtime, "repeating_count").asNumber();
        auto value = object.getProperty(runtime, "value");
        if (is_repeating) {
            auto value_array = value.asObject(runtime).asArray(runtime);
            auto repeating_length = value_array.size(runtime);

            rust::Vec<CMasonMinMax> vec;
            vec.reserve(repeating_length);

            for (int j = 0; j < repeating_length; j++) {
                auto repeat_object = value_array.getValueAtIndex(runtime, j).asObject(runtime);

                auto min_type = (int) repeat_object.getProperty(runtime, "min_type").asNumber();
                auto min_value = (float) repeat_object.getProperty(runtime, "min_value").asNumber();
                auto max_type = (int) repeat_object.getProperty(runtime, "max_type").asNumber();
                auto max_value = (float) repeat_object.getProperty(runtime, "max_value").asNumber();


                CMasonMinMax minMax{};
                minMax.min_type = min_type;
                minMax.min_value = min_value;

                minMax.max_type = max_type;
                minMax.max_value = max_value;

                vec.emplace_back(minMax);
            }

            mason_util_create_auto_repeating_track_sizing_function(
                    repeating_type, repeating_count, std::move(vec), -1, auto_vec
            );

        } else {

            auto single_object = value.asObject(runtime);

            auto min_type = (int) single_object.getProperty(runtime, "min_type").asNumber();
            auto min_value = (float) single_object.getProperty(runtime, "min_value").asNumber();
            auto max_type = (int) single_object.getProperty(runtime, "max_type").asNumber();
            auto max_value = (float) single_object.getProperty(runtime, "max_value").asNumber();


            CMasonMinMax minMax{};
            minMax.min_type = min_type;
            minMax.min_value = min_value;

            minMax.max_type = max_type;
            minMax.max_value = max_value;

            mason_util_create_single_track_sizing_function(minMax, -1, auto_vec);
        }
    }

    return std::move(auto_vec);
}

void MasonJSIModule::install(facebook::jsi::Runtime &jsiRuntime) {
    CREATE_FUNC("__Mason_updateNodeAndStyle", 3,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {

                    auto mason = getPointerValue(arguments[0], runtime);

                    auto node = getPointerValue(arguments[1], runtime);

                    auto style = getPointerValue(arguments[2], runtime);

                    mason_node_update_and_set_style(mason, node, style);

                    return Value::undefined();
                }

    );


    CREATE_FUNC("__Mason_updateStyleWithValues", 77,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {

                    auto style = getPointerValue(arguments[0], runtime);
                    auto display = (int) arguments[1].asNumber();
                    auto position = (int) arguments[2].asNumber();
                    auto direction = (int) arguments[3].asNumber();

                    auto flexDirection = (int) arguments[4].asNumber();
                    auto flexWrap = (int) arguments[5].asNumber();
                    auto overflow = (int) arguments[6].asNumber();

                    auto alignItems = (int) arguments[7].asNumber();
                    auto alignSelf = (int) arguments[8].asNumber();
                    auto alignContent = (int) arguments[9].asNumber();


                    auto justifyItems = (int) arguments[10].asNumber();
                    auto justifySelf = (int) arguments[11].asNumber();
                    auto justifyContent = (int) arguments[12].asNumber();

                    auto insetLeftType = (int) arguments[13].asNumber();
                    auto insetLeftValue = (float) arguments[14].asNumber();

                    auto insetRightType = (int) arguments[15].asNumber();
                    auto insetRightValue = (float) arguments[16].asNumber();

                    auto insetTopType = (int) arguments[17].asNumber();
                    auto insetTopValue = (float) arguments[18].asNumber();

                    auto insetBottomType = (int) arguments[19].asNumber();
                    auto insetBottomValue = (float) arguments[20].asNumber();


                    auto marginLeftType = (int) arguments[21].asNumber();
                    auto marginLeftValue = (float) arguments[22].asNumber();

                    auto marginRightType = (int) arguments[23].asNumber();
                    auto marginRightValue = (float) arguments[24].asNumber();

                    auto marginTopType = (int) arguments[25].asNumber();
                    auto marginTopValue = (float) arguments[26].asNumber();

                    auto marginBottomType = (int) arguments[27].asNumber();
                    auto marginBottomValue = (float) arguments[28].asNumber();


                    auto paddingLeftType = (int) arguments[29].asNumber();
                    auto paddingLeftValue = (float) arguments[30].asNumber();

                    auto paddingRightType = (int) arguments[31].asNumber();
                    auto paddingRightValue = (float) arguments[32].asNumber();

                    auto paddingTopType = (int) arguments[33].asNumber();
                    auto paddingTopValue = (float) arguments[34].asNumber();

                    auto paddingBottomType = (int) arguments[35].asNumber();
                    auto paddingBottomValue = (float) arguments[36].asNumber();


                    auto borderLeftType = (int) arguments[37].asNumber();
                    auto borderLeftValue = (float) arguments[38].asNumber();

                    auto borderRightType = (int) arguments[39].asNumber();
                    auto borderRightValue = (float) arguments[40].asNumber();

                    auto borderTopType = (int) arguments[41].asNumber();
                    auto borderTopValue = (float) arguments[42].asNumber();

                    auto borderBottomType = (int) arguments[43].asNumber();
                    auto borderBottomValue = (float) arguments[44].asNumber();

                    auto flexGrow = (float) arguments[45].asNumber();
                    auto flexShrink = (float) arguments[46].asNumber();

                    auto flexBasisType = (int) arguments[47].asNumber();
                    auto flexBasisValue = (float) arguments[48].asNumber();

                    auto widthType = (int) arguments[49].asNumber();
                    auto widthValue = (float) arguments[50].asNumber();

                    auto heightType = (int) arguments[51].asNumber();
                    auto heightValue = (float) arguments[52].asNumber();

                    auto minWidthType = (int) arguments[53].asNumber();
                    auto minWidthValue = (float) arguments[54].asNumber();

                    auto minHeightType = (int) arguments[55].asNumber();
                    auto minHeightValue = (float) arguments[56].asNumber();

                    auto maxWidthType = (int) arguments[57].asNumber();
                    auto maxWidthValue = (float) arguments[58].asNumber();

                    auto maxHeightType = (int) arguments[59].asNumber();
                    auto maxHeightValue = (float) arguments[60].asNumber();

                    auto gapRowType = (int) arguments[61].asNumber();
                    auto gapRowValue = (float) arguments[62].asNumber();

                    auto gapColumnType = (int) arguments[63].asNumber();
                    auto gapColumnValue = (float) arguments[64].asNumber();

                    auto aspectRatio = (float) arguments[65].asNumber();

                    auto gridAutoRowsValue = arguments[66].asObject(runtime).getArray(runtime);

                    auto gridAutoColumnsValue = arguments[67].asObject(runtime).getArray(runtime);

                    auto gridAutoRows = toNonRepeatedTrackSizingFunction(runtime,
                                                                         gridAutoRowsValue);
                    auto gridAutoColumns = toNonRepeatedTrackSizingFunction(runtime,
                                                                            gridAutoColumnsValue);

                    auto gridAutoFlow = (int) arguments[68].asNumber();

                    auto gridColumnStartType = (int) arguments[69].asNumber();
                    auto gridColumnStartValue = (short) arguments[70].asNumber();

                    auto gridColumnEndType = (int) arguments[71].asNumber();
                    auto gridColumnEndValue = (short) arguments[72].asNumber();

                    auto gridRowStartType = (int) arguments[73].asNumber();
                    auto gridRowStartValue = (short) arguments[74].asNumber();

                    auto gridRowEndType = (int) arguments[75].asNumber();
                    auto gridRowEndValue = (short) arguments[76].asNumber();


                    auto gridTemplateRowsValue = arguments[77].asObject(runtime).getArray(runtime);
                    auto gridTemplateColumnsValue = arguments[78].asObject(runtime).getArray(
                            runtime);

                    auto gridTemplateRows = toTrackSizingFunction(runtime, gridTemplateRowsValue);
                    auto gridTemplateColumns = toTrackSizingFunction(runtime,
                                                                     gridTemplateColumnsValue);

                    mason_style_update_with_values(
                            style,
                            display,
                            position,
                            direction,
                            flexDirection,
                            flexWrap,
                            overflow,
                            alignItems,
                            alignSelf,
                            alignContent,
                            justifyItems,
                            justifySelf,
                            justifyContent,

                            insetLeftType, insetLeftValue,
                            insetRightType, insetRightValue,
                            insetTopType, insetTopValue,
                            insetBottomType, insetBottomValue,

                            marginLeftType, marginLeftValue,
                            marginRightType, marginRightValue,
                            marginTopType, marginTopValue,
                            marginBottomType, marginBottomValue,

                            paddingLeftType, paddingLeftValue,
                            paddingRightType, paddingRightValue,
                            paddingTopType, paddingTopValue,
                            paddingBottomType, paddingBottomValue,

                            borderLeftType, borderLeftValue,
                            borderRightType, borderRightValue,
                            borderTopType, borderTopValue,
                            borderBottomType, borderBottomValue,

                            flexGrow, flexShrink,
                            flexBasisType, flexBasisValue,

                            widthType, widthValue,
                            heightType, heightValue,

                            minWidthType, minWidthValue,
                            minHeightType, minHeightValue,

                            maxWidthType, maxWidthValue,
                            maxHeightType, maxHeightValue,

                            gapRowType, gapRowValue,
                            gapColumnType, gapColumnValue,
                            aspectRatio,
                            std::move(gridAutoRows),
                            std::move(gridAutoColumns),
                            gridAutoFlow,

                            gridColumnStartType, gridColumnStartValue,
                            gridColumnEndType, gridColumnEndValue,

                            gridRowStartType, gridRowStartValue,
                            gridRowEndType, gridRowEndValue,

                            std::move(gridTemplateRows), std::move(gridTemplateColumns)
                    );

                    return Value::undefined();
                });


    CREATE_FUNC("__Mason_compute", 2, [](Runtime &runtime, const Value &thisValue,
                                         const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        mason_node_compute(mason, node);

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_computeWH", 4, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto width = (float) arguments[2].asNumber();
        auto height = (float) arguments[3].asNumber();

        mason_node_compute_wh(mason, node, width, height);


        return Value::undefined();

    });

    createFunc(jsiRuntime,
               "__Mason_computeMaxContent", 2,
               [](
                       Runtime &runtime,
                       const Value &thisValue,
                       const Value *arguments, size_t
                       count) -> Value {

                   auto mason = getPointerValue(arguments[0], runtime);

                   auto node = getPointerValue(arguments[1], runtime);

                   mason_node_compute_max_content(mason, node
                   );

                   return

                           Value::undefined();

               });


    CREATE_FUNC("__Mason_computeMinContent", 2, [](Runtime &runtime, const Value &thisValue,
                                                   const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        mason_node_compute_min_content(mason, node);

        return Value::undefined();

    });


/* todo */
/*
createFunc(jsiRuntime, "__Mason_layout", 2, [](Runtime &runtime, const Value &thisValue,
                                                        const Value *arguments,
                                                                  const Value *, size_t) -> Value {

    auto mason = reinterpret_cast<void*>((int64_t)arguments[0].asNumber());
    auto node = reinterpret_cast<void*>((int64_t)arguments[1].asNumber());



    return Value::undefined();

});
*/


//    CREATE_FUNC("__Mason_getComputedLayout", 2, [](Runtime &runtime, const Value &thisValue,
//                                          const Value *arguments, size_t count) -> Value {
//
//        auto style = reinterpret_cast<void*>((int64_t)arguments[0].asNumber());
//
//        auto layout = Object(runtime);
//
//        auto layoutPtr = &layout;
//
//
//        auto width = [MasonReexports style_get_width:style];
//
//        return dimensionToJS(runtime, width);
//
//    });



    CREATE_FUNC("__Mason_isDirty", 2, [](Runtime &runtime, const Value &thisValue,
                                         const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto value = mason_node_dirty(mason, node);

        return Value(value);

    }
    );


    CREATE_FUNC("__Mason_markDirty", 2, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        mason_node_mark_dirty(mason, node);

        return Value::undefined();
    });


    CREATE_FUNC("__Mason_getDisplay", 1, [](Runtime &runtime, const Value &thisValue,
                                            const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto value = mason_style_get_display(style);

        return Value(value);

    }
    );


    CREATE_FUNC("__Mason_setDisplay", 5, [](Runtime &runtime, const Value &thisValue,
                                            const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto display = (int) arguments[3].asNumber();


        mason_style_set_display(style, display);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();
    }

    );


    CREATE_FUNC("__Mason_getPosition", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto position = mason_style_get_position(style);

        return Value(position);

    });


    CREATE_FUNC("__Mason_setPosition", 5, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto position = (int) arguments[3].asNumber();

        mason_style_set_position(style, position);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getFlexWrap", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_flex_wrap(style);

        return Value(ret);

    });


    CREATE_FUNC("__Mason_setFlexWrap", 5, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (int) arguments[3].asNumber();

        mason_style_set_flex_wrap(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getAlignItems", 1, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_align_items(style);

        return Value(ret);

    });


    CREATE_FUNC("__Mason_setAlignItems", 5, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (int) arguments[3].asNumber();

        mason_style_set_align_items(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getAlignContent", 1, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_align_content(style);

        return Value(ret);

    });


    CREATE_FUNC("__Mason_setAlignContent", 5, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (int) arguments[3].asNumber();

        mason_style_set_align_content(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getAlignSelf", 1, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_align_self(style);

        return Value(ret);

    });


    CREATE_FUNC("__Mason_setAlignSelf", 5, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (int) arguments[3].asNumber();

        mason_style_set_align_self(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getJustifyContent", 1, [](Runtime &runtime, const Value &thisValue,
                                                   const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_justify_content(style);

        return Value(ret);

    });


    CREATE_FUNC("__Mason_setJustifyContent", 5, [](Runtime &runtime, const Value &thisValue,
                                                   const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (int) arguments[3].asNumber();

        mason_style_set_justify_content(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_setInset", 6, [](Runtime &runtime, const Value &thisValue,
                                          const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_inset(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getInsetLeft", 1, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_inset_left(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setInsetLeft", 6, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {
        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_inset_left(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getInsetRight", 1, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_inset_right(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setInsetRight", 6, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_inset_right(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getInsetTop", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_inset_top(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setInsetTop", 6, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_inset_top(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getInsetBottom", 1, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_inset_bottom(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setInsetBottom", 6, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_inset_bottom(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_setMargin", 6, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_margin(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getMarginLeft", 1, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_margin_left(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setMarginLeft", 6, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_margin_left(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getMarginRight", 1, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_margin_right(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setMarginRight", 6, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_margin_right(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getMarginTop", 1, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_margin_top(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setMarginTop", 6, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_margin_top(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getMarginBottom", 1, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_margin_bottom(style);

        return lengthPercentageAutoToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setMarginBottom", 6, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_margin_bottom(style, value, jsToLengthPercentageAutoType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_setPadding", 6, [](Runtime &runtime, const Value &thisValue,
                                            const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_padding(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getPaddingLeft", 1, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_padding_left(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setPaddingLeft", 6, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_padding_left(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getPaddingRight", 1, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_padding_right(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setPaddingRight", 6, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_padding_right(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getPaddingTop", 1, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_padding_top(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setPaddingTop", 6, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_padding_top(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getPaddingBottom", 1, [](Runtime &runtime, const Value &thisValue,
                                                  const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_padding_bottom(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setPaddingBottom", 6, [](Runtime &runtime, const Value &thisValue,
                                                  const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_padding_bottom(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_setBorder", 6, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_border(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getBorderLeft", 1, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_border_left(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setBorderLeft", 6, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_border_left(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getBorderRight", 1, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_border_right(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setBorderRight", 6, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_border_right(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getBorderTop", 1, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_border_top(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setBorderTop", 6, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_border_top(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getBorderBottom", 1, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_border_bottom(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setBorderBottom", 6, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_border_bottom(style, value, jsToLengthPercentageType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getFlexGrow", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = (double) mason_style_get_flex_grow(style);

        return Value(ret);

    }

    );


    CREATE_FUNC("__Mason_setFlexGrow", 5, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (float) arguments[3].asNumber();

        mason_style_set_flex_grow(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getFlexShrink", 1, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = (double) mason_style_get_flex_shrink(style);

        return Value(ret);

    }

    );


    CREATE_FUNC("__Mason_setFlexShrink", 5, [](Runtime &runtime, const Value &thisValue,
                                               const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (float) arguments[3].asNumber();

        mason_style_set_flex_shrink(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getFlexBasis", 1, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_flex_basis(style);

        return dimensionToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setFlexBasis", 6, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_flex_basis(style, value, jsToDimensionType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getGap", 1, [](Runtime &runtime, const Value &thisValue,
                                        const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto size = mason_style_get_gap(style);

        return sizeToJS(runtime, size);

    });

    CREATE_FUNC("__Mason_setGap", 8, [](Runtime &runtime, const Value &thisValue,
                                        const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);

        auto update = arguments[7].asBool();

        auto width_value = (float) arguments[3].asNumber();
        auto width_type = (int) arguments[4].asNumber();

        auto height_value = (float) arguments[5].asNumber();
        auto height_type = (int) arguments[6].asNumber();


        mason_style_set_gap(style, width_value, jsToLengthPercentageType(width_type), height_value,
                            jsToLengthPercentageType(height_type));


        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getRowGap", 1, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_row_gap(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setRowGap", 6, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);

        auto update = arguments[5].asBool();

        auto row_value = (float) arguments[3].asNumber();
        auto row_type = (int) arguments[4].asNumber();

        mason_style_set_row_gap(style, row_value, jsToLengthPercentageType(row_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getColumnGap", 1, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = mason_style_get_column_gap(style);

        return lengthPercentageToJS(runtime, ret);

    });

    CREATE_FUNC("__Mason_setColumnGap", 6, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);

        auto update = arguments[5].asBool();

        auto row_value = (float) arguments[3].asNumber();
        auto row_type = (int) arguments[4].asNumber();

        mason_style_set_column_gap(style, row_value, jsToLengthPercentageType(row_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getAspectRatio", 1, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto ret = (double) mason_style_get_aspect_ratio(style);

        return Value(ret);

    }

    );


    CREATE_FUNC("__Mason_setAspectRatio", 5, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto new_value = (float) arguments[3].asNumber();

        mason_style_set_aspect_ratio(style, new_value);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getFlexDirection", 1, [](Runtime &runtime, const Value &thisValue,
                                                  const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto value = mason_style_get_flex_direction(style);

        return Value(value);

    }

    );


    CREATE_FUNC("__Mason_setFlexDirection", 5, [](Runtime &runtime, const Value &thisValue,
                                                  const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto direction = (int) arguments[3].asNumber();

        mason_style_set_flex_direction(style, direction);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getMinWidth", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto width = mason_style_get_min_width(style);

        return dimensionToJS(runtime, width);

    });

    CREATE_FUNC("__Mason_setMinWidth", 6, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_min_width(style, value, jsToDimensionType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getMinHeight", 2, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto height = mason_style_get_min_height(style);

        return dimensionToJS(runtime, height);
    });

    CREATE_FUNC("__Mason_setMinHeight", 6, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_min_height(style, value, jsToDimensionType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    }

    );

    CREATE_FUNC("__Mason_getWidth", 1, [](Runtime &runtime, const Value &thisValue,
                                          const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto width = mason_style_get_width(style);

        return dimensionToJS(runtime, width);

    });

    CREATE_FUNC("__Mason_setWidth", 6, [](Runtime &runtime, const Value &thisValue,
                                          const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_width(style, value, jsToDimensionType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getHeight", 2, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto height = mason_style_get_height(style);

        return dimensionToJS(runtime, height);
    });

    CREATE_FUNC("__Mason_setHeight", 6, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_height(style, value, jsToDimensionType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    }

    );

    CREATE_FUNC("__Mason_getMaxWidth", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto width = mason_style_get_max_width(style);

        return dimensionToJS(runtime, width);

    });

    CREATE_FUNC("__Mason_setMaxWidth", 6, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_max_width(style, value, jsToDimensionType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });

    CREATE_FUNC("__Mason_getMaxHeight", 2, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto height = mason_style_get_max_height(style);

        return dimensionToJS(runtime, height);
    });

    CREATE_FUNC("__Mason_setMaxHeight", 6, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[5].asBool();

        auto value = (float) arguments[3].asNumber();
        auto value_type = (int) arguments[4].asNumber();


        mason_style_set_max_height(style, value, jsToDimensionType(value_type));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    }

    );

    CREATE_FUNC("__Mason_getGridAutoRows", 2, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto rows = mason_style_get_grid_auto_rows(style);

        auto parsed = mason_util_parse_non_repeated_track_sizing_function_value(rows);

        return facebook::jsi::String::createFromUtf8(runtime,
                                                     std::string(parsed.data(), parsed.length()));
    });


    CREATE_FUNC("__Mason_setGridAutoRows", 5, [](Runtime &runtime, const Value &thisValue,
                                                 const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto array = arguments[3].asObject(runtime).getArray(runtime);

        auto value = toNonRepeatedTrackSizingFunction(runtime, array);


        mason_style_set_grid_auto_rows(style, std::move(value));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    }

    );


    CREATE_FUNC("__Mason_getGridAutoColumns", 2, [](Runtime &runtime, const Value &thisValue,
                                                    const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto rows = mason_style_get_grid_auto_columns(style);

        auto parsed = mason_util_parse_non_repeated_track_sizing_function_value(rows);

        return facebook::jsi::String::createFromUtf8(runtime,
                                                     std::string(parsed.data(), parsed.length()));
    });


    CREATE_FUNC("__Mason_setGridAutoColumns", 5, [](Runtime &runtime, const Value &thisValue,
                                                    const Value *arguments, size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto array = arguments[3].asObject(runtime).getArray(runtime);

        auto value = toNonRepeatedTrackSizingFunction(runtime, array);


        mason_style_set_grid_auto_columns(style, std::move(value));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    }

    );


    CREATE_FUNC("__Mason_getAutoFlow", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto value = mason_style_get_grid_auto_flow(style);

        return Value(value);

    }

    );


    CREATE_FUNC("__Mason_setAutoFlow", 5, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto flow = (int) arguments[3].asNumber();

        mason_style_set_grid_auto_flow(style, flow);

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getArea", 1, [](Runtime &runtime, const Value &thisValue,
                                         const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto row_start = mason_style_get_grid_row_start(style);
        auto row_end = mason_style_get_grid_row_start(style);

        auto col_start = mason_style_get_grid_column_start(style);
        auto col_end = mason_style_get_grid_column_start(style);

        auto ret = facebook::jsi::Object(runtime);
        ret.setProperty(runtime, "col_start_type", (int) col_start.value_type);
        ret.setProperty(runtime, "col_start_value", (int) col_start.value);
        ret.setProperty(runtime, "col_end_type", (int) col_end.value_type);
        ret.setProperty(runtime, "col_end_value", (int) col_end.value);

        ret.setProperty(runtime, "row_start_type", (int) row_start.value_type);
        ret.setProperty(runtime, "row_start_value", (int) row_start.value);
        ret.setProperty(runtime, "row_end_type", (int) row_end.value_type);
        ret.setProperty(runtime, "row_end_value", (int) row_end.value);


        std::stringstream ss;
        if (col_start.value == col_end.value && col_start.value_type == col_end.value_type) {
            if (col_start.value_type == CMasonGridPlacementType::Auto) {
                ss << "auto";
            } else {
                ss << col_start.value;
                if (col_start.value_type == CMasonGridPlacementType::Span) {
                    ss << " span";
                }
            }
        } else {
            if (col_start.value_type == CMasonGridPlacementType::Auto) {
                ss << "auto";
            } else {
                ss << col_start.value;
                if (col_start.value_type == CMasonGridPlacementType::Span) {
                    ss << " span";
                }
            }

            ss << " / ";

            if (col_end.value_type == CMasonGridPlacementType::Auto) {
                ss << "auto";
            } else {
                ss << col_end.value;
                if (col_end.value_type == CMasonGridPlacementType::Span) {
                    ss << " span";
                }
            }
        }


        ret.setProperty(runtime, "colFormatted", ss.str().c_str());


        std::stringstream row_ss;
        if (row_start.value == row_end.value && row_start.value_type == row_end.value_type) {
            if (row_start.value_type == CMasonGridPlacementType::Auto) {
                row_ss << "auto";
            } else {
                row_ss << row_start.value;
                if (row_start.value_type == CMasonGridPlacementType::Span) {
                    row_ss << " span";
                }
            }
        } else {
            if (row_start.value_type == CMasonGridPlacementType::Auto) {
                row_ss << "auto";
            } else {
                row_ss << row_start.value;
                if (row_start.value_type == CMasonGridPlacementType::Span) {
                    row_ss << " span";
                }
            }

            row_ss << " / ";

            if (row_end.value_type == CMasonGridPlacementType::Auto) {
                row_ss << "auto";
            } else {
                row_ss << row_end.value;
                if (row_end.value_type == CMasonGridPlacementType::Span) {
                    row_ss << " span";
                }
            }
        }

        ret.setProperty(runtime, "rowFormatted", row_ss.str().c_str());

        return ret;

    }

    );


    CREATE_FUNC("__Mason_setArea", 5, [](Runtime &runtime, const Value &thisValue,
                                         const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto object = arguments[3].asObject(runtime);

        auto rowStartType = (int) object.getProperty(runtime, "row_start_type").asNumber();
        auto rowStartValue = (short) object.getProperty(runtime, "row_start_value").asNumber();

        auto rowEndType = (int) object.getProperty(runtime, "row_end_type").asNumber();
        auto rowEndValue = (short) object.getProperty(runtime, "row_end_value").asNumber();

        auto columnStartType = (int) object.getProperty(runtime, "col_start_type").asNumber();
        auto columnStartValue = (short) object.getProperty(runtime, "col_start_value").asNumber();

        auto columnEndType = (int) object.getProperty(runtime, "col_end_type").asNumber();
        auto columnEndValue = (short) object.getProperty(runtime, "col_end_value").asNumber();

        mason_style_set_grid_area(
                style,
                jsToGridPlacement(rowStartValue, rowStartType),
                jsToGridPlacement(rowEndValue, rowEndType),
                jsToGridPlacement(columnStartValue, columnStartType),
                jsToGridPlacement(columnEndValue, columnEndType)
        );

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getColumn", 1, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto row_start = mason_style_get_grid_row_start(style);
        auto row_end = mason_style_get_grid_row_start(style);

        auto col_start = mason_style_get_grid_column_start(style);
        auto col_end = mason_style_get_grid_column_start(style);

        auto ret = facebook::jsi::Object(runtime);
        ret.setProperty(runtime, "col_start_type", (int) col_start.value_type);
        ret.setProperty(runtime, "col_start_value", (int) col_start.value);
        ret.setProperty(runtime, "col_end_type", (int) col_end.value_type);
        ret.setProperty(runtime, "col_end_value", (int) col_end.value);

        std::stringstream ss;
        if (col_start.value == col_end.value && col_start.value_type == col_end.value_type) {
            if (col_start.value_type == CMasonGridPlacementType::Auto) {
                ss << "auto";
            } else {
                ss << col_start.value;
                if (col_start.value_type == CMasonGridPlacementType::Span) {
                    ss << " span";
                }
            }
        } else {
            if (col_start.value_type == CMasonGridPlacementType::Auto) {
                ss << "auto";
            } else {
                ss << col_start.value;
                if (col_start.value_type == CMasonGridPlacementType::Span) {
                    ss << " span";
                }
            }

            ss << " / ";

            if (col_end.value_type == CMasonGridPlacementType::Auto) {
                ss << "auto";
            } else {
                ss << col_end.value;
                if (col_end.value_type == CMasonGridPlacementType::Span) {
                    ss << " span";
                }
            }
        }


        ret.setProperty(runtime, "colFormatted", ss.str().c_str());

        return ret;

    }

    );


    CREATE_FUNC("__Mason_setColumn", 5, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto column = arguments[3].asObject(runtime);

        auto startValue = (short) column.getProperty(runtime, "col_start_value").asNumber();
        auto startType = (int) column.getProperty(runtime, "col_start_type").asNumber();

        auto endValue = (short) column.getProperty(runtime, "col_end_value").asNumber();
        auto endType = (int) column.getProperty(runtime, "col_end_type").asNumber();


        mason_style_set_grid_column(style, jsToGridPlacement(startValue, startType),
                                    jsToGridPlacement(endValue, endType));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getColumnStart", 1, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto value = mason_style_get_grid_column_start(style);

        return gridPlacementToJS(runtime, value);

    }

    );


    CREATE_FUNC("__Mason_setColumnStart", 5, [](Runtime &runtime, const Value &thisValue,
                                                const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto object = arguments[3].asObject(runtime);

        auto value = (short) object.getProperty(runtime, "value").asNumber();
        auto type = (int) object.getProperty(runtime, "type").asNumber();

        if (type >= 0 && type < 3) {
            mason_style_set_grid_column_start(style, jsToGridPlacement(value, type));

            if (update) {
                mason_node_update_and_set_style(mason, node, style);
            }
        }


        return Value::undefined();

    }

    );


    CREATE_FUNC("__Mason_getColumnEnd", 1, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto value = mason_style_get_grid_column_end(style);

        return gridPlacementToJS(runtime, value);

    }

    );


    CREATE_FUNC("__Mason_setColumnEnd", 5, [](Runtime &runtime, const Value &thisValue,
                                              const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto object = arguments[3].asObject(runtime);

        auto value = (short) object.getProperty(runtime, "value").asNumber();
        auto type = (int) object.getProperty(runtime, "type").asNumber();

        if (type >= 0 && type < 3) {
            mason_style_set_grid_column_end(style, jsToGridPlacement(value, type));

            if (update) {
                mason_node_update_and_set_style(mason, node, style);
            }
        }


        return Value::undefined();

    }

    );

    CREATE_FUNC("__Mason_getRow", 1, [](Runtime &runtime, const Value &thisValue,
                                        const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto row_start = mason_style_get_grid_row_start(style);
        auto row_end = mason_style_get_grid_row_start(style);

        auto col_start = mason_style_get_grid_column_start(style);
        auto col_end = mason_style_get_grid_column_start(style);

        auto ret = facebook::jsi::Object(runtime);

        ret.setProperty(runtime, "row_start_type", (int) row_start.value_type);
        ret.setProperty(runtime, "row_start_value", (int) row_start.value);
        ret.setProperty(runtime, "row_end_type", (int) row_end.value_type);
        ret.setProperty(runtime, "row_end_value", (int) row_end.value);


        std::stringstream row_ss;
        if (row_start.value == row_end.value && row_start.value_type == row_end.value_type) {
            if (row_start.value_type == CMasonGridPlacementType::Auto) {
                row_ss << "auto";
            } else {
                row_ss << row_start.value;
                if (row_start.value_type == CMasonGridPlacementType::Span) {
                    row_ss << " span";
                }
            }
        } else {
            if (row_start.value_type == CMasonGridPlacementType::Auto) {
                row_ss << "auto";
            } else {
                row_ss << row_start.value;
                if (row_start.value_type == CMasonGridPlacementType::Span) {
                    row_ss << " span";
                }
            }

            row_ss << " / ";

            if (row_end.value_type == CMasonGridPlacementType::Auto) {
                row_ss << "auto";
            } else {
                row_ss << row_end.value;
                if (row_end.value_type == CMasonGridPlacementType::Span) {
                    row_ss << " span";
                }
            }
        }

        ret.setProperty(runtime, "rowFormatted", row_ss.str().c_str());

        return ret;

    }

    );


    CREATE_FUNC("__Mason_setRow", 5, [](Runtime &runtime, const Value &thisValue,
                                        const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();


        auto row = arguments[3].asObject(runtime);

        auto startValue = (short) row.getProperty(runtime, "row_start_value").asNumber();
        auto startType = (int) row.getProperty(runtime, "row_start_type").asNumber();

        auto endValue = (short) row.getProperty(runtime, "row_end_value").asNumber();
        auto endType = (int) row.getProperty(runtime, "row_end_type").asNumber();


        mason_style_set_grid_row(style, jsToGridPlacement(startValue, startType),
                                 jsToGridPlacement(endValue, endType));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    });


    CREATE_FUNC("__Mason_getRowStart", 1, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto value = mason_style_get_grid_row_start(style);

        return gridPlacementToJS(runtime, value);

    }

    );


    CREATE_FUNC("__Mason_setRowStart", 5, [](Runtime &runtime, const Value &thisValue,
                                             const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto object = arguments[3].asObject(runtime);

        auto value = (short) object.getProperty(runtime, "value").asNumber();
        auto type = (int) object.getProperty(runtime, "type").asNumber();

        if (type >= 0 && type < 3) {
            mason_style_set_grid_row_start(style, jsToGridPlacement(value, type));

            if (update) {
                mason_node_update_and_set_style(mason, node, style);
            }
        }


        return Value::undefined();

    }

    );


    CREATE_FUNC("__Mason_getRowEnd", 1, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto value = mason_style_get_grid_row_end(style);

        return gridPlacementToJS(runtime, value);

    }

    );


    CREATE_FUNC("__Mason_setRowEnd", 5, [](Runtime &runtime, const Value &thisValue,
                                           const Value *arguments, size_t count) -> Value {


        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto object = arguments[3].asObject(runtime);

        auto value = (short) object.getProperty(runtime, "value").asNumber();
        auto type = (int) object.getProperty(runtime, "type").asNumber();

        if (type >= 0 && type < 3) {
            mason_style_set_grid_row_end(style, jsToGridPlacement(value, type));

            if (update) {
                mason_node_update_and_set_style(mason, node, style);
            }
        }


        return Value::undefined();

    }

    );


    CREATE_FUNC("__Mason_getGridTemplateRows", 2, [](Runtime &runtime, const Value &thisValue,
                                                     const Value *arguments,
                                                     size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto rows = mason_style_get_grid_template_rows(style);

        auto parsed = mason_util_parse_auto_repeating_track_sizing_function(rows);

        return facebook::jsi::String::createFromUtf8(runtime,
                                                     std::string(parsed.data(), parsed.length()));
    });


    CREATE_FUNC("__Mason_setGridTemplateRows", 5, [](Runtime &runtime, const Value &thisValue,
                                                     const Value *arguments,
                                                     size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto array = arguments[3].asObject(runtime).getArray(runtime);

        auto value = toTrackSizingFunction(runtime, array);


        mason_style_set_grid_template_rows(style, std::move(value));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    }

    );


    CREATE_FUNC("__Mason_getGridTemplateColumns", 2, [](Runtime &runtime, const Value &thisValue,
                                                        const Value *arguments,
                                                        size_t count) -> Value {

        auto style = getPointerValue(arguments[0], runtime);

        auto rows = mason_style_get_grid_template_columns(style);

        auto parsed = mason_util_parse_auto_repeating_track_sizing_function(rows);

        return facebook::jsi::String::createFromUtf8(runtime,
                                                     std::string(parsed.data(), parsed.length()));
    });


    CREATE_FUNC("__Mason_setGridTemplateColumns", 5, [](Runtime &runtime, const Value &thisValue,
                                                        const Value *arguments,
                                                        size_t count) -> Value {

        auto mason = getPointerValue(arguments[0], runtime);

        auto node = getPointerValue(arguments[1], runtime);

        auto style = getPointerValue(arguments[2], runtime);
        auto update = arguments[4].asBool();

        auto array = arguments[3].asObject(runtime).getArray(runtime);

        auto value = toTrackSizingFunction(runtime, array);


        mason_style_set_grid_template_columns(style, std::move(value));

        if (update) {
            mason_node_update_and_set_style(mason, node, style);
        }

        return Value::undefined();

    }

    );


}
