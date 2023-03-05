//
// Created by Osei Fortune on 25/02/2023.
//

#include "CanvasJSIModule.h"

#include <array>



void CanvasJSIModule::install(facebook::jsi::Runtime &jsiRuntime) {
    auto canvas_module = facebook::jsi::Object(jsiRuntime);

    CREATE_FUNC("DOMMatrix", canvas_module, 1,
                ([](Runtime &runtime, const Value &thisValue,
                    const Value *arguments, size_t count) -> Value {

                    if (count > 0) {
                        auto obj = &arguments[0];
                        if (arguments[0].isObject()) {
                            auto initObject = arguments[0].asObject(runtime);
                            if (initObject.isArray(runtime)) {
                                auto init = initObject.getArray(runtime);
                                auto size = init.size(runtime);
                                if (size == 6) {
                                    auto matrix = canvas_native_matrix_create();
                                    rust::Vec<float> buf;
                                    buf.reserve(size);
                                    for (int i = 0; i < size; i++) {
                                        auto item = init.getValueAtIndex(runtime, i).asNumber();
                                        buf.emplace_back((float) item);
                                    }
                                    rust::Slice<const float> slice(buf.data(), buf.size());

                                    canvas_native_matrix_update(*matrix, slice);

                                    auto object = std::make_shared<MatrixImpl>(std::move(matrix));
                                    return jsi::Object::createFromHostObject(runtime,
                                                                             std::move(object));
                                }

                                if (size == 16) {
                                    auto matrix = canvas_native_matrix_create();
                                    std::array<float, 16> buf;

                                    for (int i = 0; i < size; i++) {
                                        auto item = init.getValueAtIndex(runtime, i).asNumber();
                                        buf[i] = (float) item;
                                    }
                                    canvas_native_matrix_update_3d(*matrix, buf);

                                    auto object = std::make_shared<MatrixImpl>(std::move(matrix));
                                    return jsi::Object::createFromHostObject(runtime,
                                                                             std::move(object));
                                }
                            }
                        }

                    } else {
                        auto matrix = canvas_native_matrix_create();
                        auto object = std::make_shared<MatrixImpl>(std::move(matrix));
                        return jsi::Object::createFromHostObject(runtime, std::move(object));
                    }
                    return Value::undefined();
                })

    );

    CREATE_FUNC("Path2D", canvas_module, 1,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {

                    if (count > 0) {
                        auto obj = &arguments[0];
                        if (obj->isString()) {
                            auto d = obj->asString(runtime).utf8(runtime);
                            auto path = canvas_native_path_create_with_str(
                                    rust::Str(d.data(), d.size()));
                            auto object = std::make_shared<Path2D>(std::move(path));
                            return jsi::Object::createFromHostObject(runtime, std::move(object));
                        } else if (obj->isObject() && (!obj->isNull() || !obj->isUndefined())) {
                            auto path_to_copy = getHostObject<Path2D>(runtime, obj);
                            if (path_to_copy != nullptr) {
                                auto path = canvas_native_path_create_with_path(
                                        path_to_copy->GetPath());
                                auto object = std::make_shared<Path2D>(std::move(path));
                                return jsi::Object::createFromHostObject(runtime,
                                                                         std::move(object));
                            }
                        }
                    } else {
                        auto path = canvas_native_path_create();
                        auto object = std::make_shared<Path2D>(std::move(path));
                        return jsi::Object::createFromHostObject(runtime, std::move(object));
                    }
                    return Value::undefined();
                }

    );

    CREATE_FUNC("TextEncoder", canvas_module, 1,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {
                    if (count == 1 && !arguments[0].isString()) {
                        auto arg = arguments[0].toString(runtime).utf8(runtime);
                        throw JSINativeException(
                                "Failed to construct 'TextEncoder': The encoding label provided (" +
                                arg + "') is invalid");
                    }

                    std::string encoding("utf-8");
                    if (count == 1) {
                        encoding = arguments[0].asString(runtime).utf8(runtime);
                    }
                    auto encoder = canvas_native_text_encoder_create(
                            rust::Str(encoding.data(), encoding.size()));
                    auto shared_encoder = std::make_shared<TextEncoderImpl>(std::move(encoder));
                    return jsi::Object::createFromHostObject(runtime, shared_encoder);
                }

    );

    CREATE_FUNC("TextDecoder", canvas_module, 1,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {
                    if (count == 1 && !arguments[0].isString()) {
                        auto arg = arguments[0].toString(runtime).utf8(runtime);
                        throw JSINativeException(
                                "Failed to construct 'TextDecoder': The encoding label provided (" +
                                arg + "') is invalid");
                    }

                    std::string encoding("utf-8");
                    if (count == 1) {
                        encoding = arguments[0].asString(runtime).utf8(runtime);
                    }
                    auto encoder = canvas_native_text_decoder_create(
                            rust::Str(encoding.data(), encoding.size()));
                    auto shared_decoder = std::make_shared<TextDecoderImpl>(std::move(encoder));
                    return jsi::Object::createFromHostObject(runtime, shared_decoder);
                }

    );


    CREATE_FUNC("createImageBitmap", canvas_module, 5,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {

                    auto image = &arguments[0];
                    auto sx_or_options = &arguments[1];
                    auto sy = &arguments[2];
                    auto sw = &arguments[3];
                    auto sh = &arguments[4];

                    auto len = count;
                    auto cb = &arguments[count - 1];
                    len = len - 1;

                    if (len == 1 && !image->isObject() ||
                        image->asObject(runtime).isFunction(runtime)) {
                        throw jsi::JSINativeException("Illegal constructor");
                    }

                    Options options;


                    if (len == 0) {
                        throw jsi::JSINativeException("Illegal constructor");
                    }

                    if (!cb->isObject() && !cb->asObject(runtime).isFunction(runtime)) {
                        throw jsi::JSINativeException("Illegal constructor");
                    }


                    if (image->isNull() || image->isUndefined()) {
                        cb->asObject(runtime).asFunction(runtime).call(runtime,
                                                                       {jsi::String::createFromAscii(
                                                                               runtime,
                                                                               "Failed to load image"),
                                                                        Value::null()});
                        return Value::undefined();
                    }

                    if (len >= 4 && (sw->isNumber() && sw->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect width is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, {error,
                                                                                 Value::undefined()});
                        return Value::undefined();
                    }
                    if (len >= 5 && (sh->isNumber() && sh->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect height is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, {error,
                                                                                 Value::undefined()});
                        return Value::undefined();
                    }


                    auto image_asset = getHostObject<ImageAssetImpl>(
                            runtime, image);

                    auto image_bitmap = getHostObject<ImageBitmapImpl>(
                            runtime, image);

                    if (len == 1 || len == 2) {
                        if (len == 2) {
                            options = ImageBitmapImpl::HandleOptions(runtime, sx_or_options);
                        }


                        auto ret = canvas_native_image_bitmap_create_from_asset(
                                image_asset != nullptr ? image_asset->GetImageAsset()
                                                       : image_bitmap->GetImageAsset(),
                                options.flipY,
                                options.premultiplyAlpha,
                                options.colorSpaceConversion,
                                options.resizeQuality,
                                options.resizeWidth,
                                options.resizeHeight);


                        auto bitmap = std::make_shared<ImageBitmapImpl>(std::move(ret));

                        auto bitmap_object = jsi::Object::createFromHostObject(runtime, bitmap);

                        cb->asObject(runtime).asFunction(runtime).call(runtime, {Value::null(),
                                                                                 std::move(
                                                                                         bitmap)});

                        return Value::undefined();
                    } else if (len == 5 || len == 6) {

                        if (len == 6) {
                            options = ImageBitmapImpl::HandleOptions(runtime, arguments[5]);
                        }

                        auto ret = canvas_native_image_bitmap_create_from_asset_src_rect(
                                image_asset != nullptr ? image_asset->GetImageAsset()
                                                       : image_bitmap->GetImageAsset(),
                                (float) sx_or_options->asNumber(),
                                (float) sy->asNumber(),
                                (float) sw->asNumber(),
                                (float) sh->asNumber(),
                                options.flipY,
                                options.premultiplyAlpha,
                                options.colorSpaceConversion,
                                options.resizeQuality,
                                options.resizeWidth,
                                options.resizeHeight);

                        auto bitmap = std::make_shared<ImageBitmapImpl>(std::move(ret));

                        auto bitmap_object = jsi::Object::createFromHostObject(runtime, bitmap);

                        cb->asObject(runtime).asFunction(runtime).call(runtime, {Value::null(),
                                                                                 std::move(
                                                                                         bitmap)});

                        return Value::undefined();
                    }

                    auto object = image->asObject(runtime);

                    // NS Blob
                    auto nsBuffer = object.getProperty(runtime, "_buffer");


                    if (nsBuffer.isNull() || nsBuffer.isUndefined()) {
                        auto error = jsi::JSError(runtime, "Failed to load image");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, {error,
                                                                                 Value::undefined()});
                        return Value::undefined();

                    }

                    if (!nsBuffer.asObject(runtime).isArrayBuffer(runtime)) {

                    }

                    auto ab = nsBuffer.asObject(runtime).getArrayBuffer(runtime);
                    auto data = rust::Slice<const uint8_t>(ab.data(runtime), ab.size(runtime));

                    if (len >= 4 && (sw->isNumber() && sw->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect width is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, {error,
                                                                                 Value::undefined()});
                        return Value::undefined();
                    }
                    if (len >= 5 && (sh->isNumber() && sh->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect height is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, {error,
                                                                                 Value::undefined()});
                        return Value::undefined();
                    }


                    if (len == 1 || len == 2) {
                        if (len == 2) {
                            options = ImageBitmapImpl::HandleOptions(runtime, sx_or_options);
                        }

                        auto asset = canvas_native_image_asset_create();

                        auto cbFunc = cb->asObject(runtime).asFunction(runtime);

                        std::thread thread(
                                [&data, &runtime, &options, &cbFunc](rust::Box<ImageAsset> asset) {


                                    auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                            data,
                                            options.flipY,
                                            options.premultiplyAlpha,
                                            options.colorSpaceConversion,
                                            options.resizeQuality,
                                            options.resizeWidth,
                                            options.resizeHeight,
                                            *asset);


                                    if (!done) {
                                        auto error = jsi::JSError(runtime, "Failed to load image");
                                        cbFunc.call(runtime, error, jsi::Value::undefined());
                                    } else {
                                        auto ret = std::make_shared<ImageBitmapImpl>(
                                                std::move(asset));
                                        auto object = jsi::Object::createFromHostObject(runtime,
                                                                                        std::move(
                                                                                                ret));
                                        cbFunc.call(runtime, {jsi::Value::undefined(), object});
                                    }
                                }, std::move(asset));

                        return Value::undefined();
                    } else if (len == 5 || len == 6) {

                        auto cbFunc = cb->asObject(runtime).asFunction(runtime);

                        if (len == 6) {
                            options = ImageBitmapImpl::HandleOptions(runtime, arguments[5]);
                        }

                        auto asset = canvas_native_image_asset_create();


                        std::thread thread(
                                [&data, &runtime, &options, &cbFunc](rust::Box<ImageAsset> asset,
                                                                     float sx_or_options,
                                                                     float sy,
                                                                     float sw,
                                                                     float sh) {

                                    auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                            data,
                                            sx_or_options,
                                            sy,
                                            sw,
                                            sh,
                                            options.flipY,
                                            options.premultiplyAlpha,
                                            options.colorSpaceConversion,
                                            options.resizeQuality,
                                            options.resizeWidth,
                                            options.resizeHeight, *asset);


                                    if (!done) {
                                        auto error = jsi::JSError(runtime, "Failed to load image");
                                        cbFunc.call(runtime, {error, jsi::Value::undefined()});
                                    } else {
                                        auto ret = std::make_shared<ImageBitmapImpl>(
                                                std::move(asset));
                                        auto object = jsi::Object::createFromHostObject(runtime,
                                                                                        std::move(
                                                                                                ret));
                                        cbFunc.call(runtime, {jsi::Value::undefined(), object});
                                    }


                                }, std::move(asset), data,
                                sx_or_options->asNumber(),
                                sy->asNumber(),
                                sw->asNumber(),
                                sh->asNumber(), std::move(asset));

                        return Value::undefined();
                    }


                }

    );

    CREATE_FUNC("create2DContext", canvas_module, 7,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {
                    auto width = (float) arguments[0].asNumber();
                    auto height = (float) arguments[1].asNumber();
                    auto density = (float) arguments[2].asNumber();
                    auto context = getPointerValue(runtime, arguments[3]);
                    auto samples = (int) arguments[4].asNumber();
                    auto alpha = (bool) arguments[5].asBool();
                    auto font_color = (int) arguments[6].asNumber();
                    auto ppi = (float) arguments[7].asNumber();
                    auto direction = (int) arguments[7].asNumber();
                    auto context_2d = canvas_native_context_create_gl(width, height, density,
                                                                      context,
                                                                      samples, alpha,
                                                                      font_color, ppi, direction);

                    auto ret = std::make_shared<CanvasRenderingContext2DImpl>(
                            std::move(context_2d));

                    return jsi::Object::createFromHostObject(runtime, ret);
                }

    );

    CREATE_FUNC("createWebGLContext", canvas_module, 7,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {

                    if (arguments[0].isObject()) {
                        auto config = arguments[0].asObject(runtime);
                        std::string version("none");
                        auto alpha = true;
                        auto antialias = true;
                        auto depth = true;
                        auto fail_if_major_performance_caveat = false;
                        std::string power_preference("default");
                        auto premultiplied_alpha = true;
                        auto preserve_drawing_buffer = false;
                        auto stencil = false;
                        auto desynchronized = false;
                        auto xr_compatible = false;


                        auto versionValue = config.getProperty(runtime, "version");
                        if (versionValue.isString()) {
                            version = versionValue.asString(runtime).utf8(runtime);
                        }

                        auto alphaValue = config.getProperty(runtime, "alpha");
                        if (alphaValue.isBool()) {
                            alpha = alphaValue.asBool();
                        }

                        auto antialiasValue = config.getProperty(runtime, "antialias");
                        if (antialiasValue.isBool()) {
                            antialias = antialiasValue.asBool();
                        }

                        auto failIfMajorPerformanceCaveatValue = config.getProperty(runtime,
                                                                                    "failIfMajorPerformanceCaveat");
                        if (failIfMajorPerformanceCaveatValue.isBool()) {
                            fail_if_major_performance_caveat = failIfMajorPerformanceCaveatValue.asBool();
                        }

                        auto powerPreferenceValue = config.getProperty(runtime, "powerPreference");
                        if (powerPreferenceValue.isString()) {
                            power_preference = powerPreferenceValue.asString(runtime).utf8(runtime);
                        }

                        auto premultipliedAlphaValue = config.getProperty(runtime,
                                                                          "premultipliedAlpha");
                        if (premultipliedAlphaValue.isBool()) {
                            premultiplied_alpha = premultipliedAlphaValue.asBool();
                        }

                        auto preserveDrawingBufferValue = config.getProperty(runtime,
                                                                             "preserveDrawingBuffer");
                        if (preserveDrawingBufferValue.isBool()) {
                            preserve_drawing_buffer = preserveDrawingBufferValue.asBool();
                        }

                        auto stencilValue = config.getProperty(runtime, "stencil");
                        if (stencilValue.isBool()) {
                            stencil = stencilValue.asBool();
                        }

                        auto desynchronizedValue = config.getProperty(runtime, "desynchronized");
                        if (desynchronizedValue.isBool()) {
                            desynchronized = desynchronizedValue.asBool();
                        }

                        auto xrCompatibleValue = config.getProperty(
                                runtime,
                                "xrCompatible");
                        if (xrCompatibleValue.isBool()) {
                            xr_compatible = xrCompatibleValue.asBool();
                        }

                        if (version !=
                            "v1" &&
                            version !=
                            "v2") {
                            return Value::undefined();
                        } else {
                            std::shared_ptr<WebGLRenderingContext> renderingContext;

                            if (count == 7) {
                                auto width = arguments[1].asNumber();
                                auto height = arguments[2].asNumber();
                                auto density = arguments[3].asNumber();
                                auto fontColor = arguments[4].asNumber();
                                auto ppi = arguments[5].asNumber();
                                auto direction = arguments[6].asNumber();
                                auto ctx = canvas_native_webgl_create_no_window(
                                        (int32_t) width,
                                        (int32_t) height,
                                        rust::Str(
                                                version.c_str(),
                                                version.size()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str(),
                                                power_preference.size()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );
                                renderingContext = std::make_shared<WebGLRenderingContext>(
                                        std::move(
                                                ctx));

                            } else {
                                auto width = (int32_t) arguments[1].asNumber();
                                auto height = (int32_t) arguments[2].asNumber();

                                auto ctx = canvas_native_webgl_create_no_window(
                                        width,
                                        height,
                                        rust::Str(
                                                version.c_str(),
                                                version.size()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.data(),
                                                power_preference.size()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );

                                renderingContext = std::make_shared<WebGLRenderingContext>(
                                        std::move(
                                                ctx));
                            }

                            return jsi::Object::createFromHostObject(
                                    runtime,
                                    renderingContext);
                        }
                    }


                    return Value::undefined();
                }

    );

    CREATE_FUNC("createWebGL2Context", canvas_module, 3,
                [](Runtime &runtime, const Value &thisValue,
                   const Value *arguments, size_t count) -> Value {

                    if (arguments[0].isObject()) {
                        auto config = arguments[0].asObject(runtime);
                        std::string version("none");
                        auto alpha = true;
                        auto antialias = true;
                        auto depth = true;
                        auto fail_if_major_performance_caveat = false;
                        std::string power_preference("default");
                        auto premultiplied_alpha = true;
                        auto preserve_drawing_buffer = false;
                        auto stencil = false;
                        auto desynchronized = false;
                        auto xr_compatible = false;


                        auto versionValue = config.getProperty(runtime, "version");
                        if (versionValue.isString()) {
                            version = versionValue.asString(runtime).utf8(runtime);
                        }

                        auto alphaValue = config.getProperty(runtime, "alpha");
                        if (alphaValue.isBool()) {
                            alpha = alphaValue.asBool();
                        }

                        auto antialiasValue = config.getProperty(runtime, "antialias");
                        if (antialiasValue.isBool()) {
                            antialias = antialiasValue.asBool();
                        }

                        auto failIfMajorPerformanceCaveatValue = config.getProperty(runtime,
                                                                                    "failIfMajorPerformanceCaveat");
                        if (failIfMajorPerformanceCaveatValue.isBool()) {
                            fail_if_major_performance_caveat = failIfMajorPerformanceCaveatValue.asBool();
                        }

                        auto powerPreferenceValue = config.getProperty(runtime, "powerPreference");
                        if (powerPreferenceValue.isString()) {
                            power_preference = powerPreferenceValue.asString(runtime).utf8(runtime);
                        }

                        auto premultipliedAlphaValue = config.getProperty(runtime,
                                                                          "premultipliedAlpha");
                        if (premultipliedAlphaValue.isBool()) {
                            premultiplied_alpha = premultipliedAlphaValue.asBool();
                        }

                        auto preserveDrawingBufferValue = config.getProperty(runtime,
                                                                             "preserveDrawingBuffer");
                        if (preserveDrawingBufferValue.isBool()) {
                            preserve_drawing_buffer = preserveDrawingBufferValue.asBool();
                        }

                        auto stencilValue = config.getProperty(runtime, "stencil");
                        if (stencilValue.isBool()) {
                            stencil = stencilValue.asBool();
                        }

                        auto desynchronizedValue = config.getProperty(runtime, "desynchronized");
                        if (desynchronizedValue.isBool()) {
                            desynchronized = desynchronizedValue.asBool();
                        }

                        auto xrCompatibleValue = config.getProperty(
                                runtime,
                                "xrCompatible");
                        if (xrCompatibleValue.isBool()) {
                            xr_compatible = xrCompatibleValue.asBool();
                        }

                        if (version !=
                            "v1" &&
                            version !=
                            "v2") {
                            return Value::undefined();
                        } else {
                            std::shared_ptr<WebGL2RenderingContext> renderingContext;

                            if (count ==
                                7) {
                                auto width = arguments[1].asNumber();
                                auto height = arguments[2].asNumber();
                                auto density = arguments[3].asNumber();
                                auto fontColor = arguments[4].asNumber();
                                auto ppi = arguments[5].asNumber();
                                auto direction = arguments[6].asNumber();
                                auto ctx = canvas_native_webgl_create_no_window(
                                        (int32_t) width,
                                        (int32_t) height,
                                        rust::Str(
                                                version.c_str(),
                                                version.size()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str(),
                                                power_preference.size()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );
                                renderingContext = std::make_shared<WebGL2RenderingContext>(
                                        std::move(
                                                ctx));

                            } else {
                                auto width = (int32_t) arguments[1].asNumber();
                                auto height = (int32_t) arguments[2].asNumber();
                                auto ctx = canvas_native_webgl_create_no_window(
                                        width,
                                        height,
                                        rust::Str(
                                                version.c_str(),
                                                version.size()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str(),
                                                power_preference.size()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );

                                renderingContext = std::make_shared<WebGL2RenderingContext>(
                                        std::move(
                                                ctx), WebGLRenderingVersion::V2);
                            }


                            return jsi::Object::createFromHostObject(
                                    runtime,
                                    renderingContext);
                        }
                    }

                    return Value::undefined();
                }

    );
}
