//
// Created by Osei Fortune on 25/02/2023.
//

#include "CanvasJSIModule.h"
#include "v8runtime/JSIV8ValueConverter.h"
#include "canvas2d/CanvasRenderingContext2DImpl.h"


void CanvasJSIModule::install(facebook::jsi::Runtime &jsiRuntime) {
    auto canvas_module = facebook::jsi::Object(jsiRuntime);

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
                            auto path_to_copy = obj->asObject(runtime).asHostObject<Path2D>(
                                    runtime);
                            if (path_to_copy != nullptr) {
                                auto path = canvas_native_path_create_with_path(
                                        path_to_copy->GetPath());
                                auto object = std::make_shared<Path2D>(std::move(path));
                                return jsi::Object::createFromHostObject(runtime,
                                                                         std::move(object));
                            }
                        } else {
                            auto path = canvas_native_path_create();
                            auto object = std::make_shared<Path2D>(std::move(path));
                            return jsi::Object::createFromHostObject(runtime, std::move(object));
                        }

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
                                                                       jsi::String::createFromAscii(
                                                                               runtime,
                                                                               "Failed to load image"),
                                                                       Value::null());
                        return Value::undefined();
                    }

                    if (len >= 4 && (sw->isNumber() && sw->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect width is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, error,
                                                                       Value::undefined());
                        return Value::undefined();
                    }
                    if (len >= 5 && (sh->isNumber() && sh->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect height is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, error,
                                                                       Value::undefined());
                        return Value::undefined();
                    }


                    auto image_asset = image->asObject(runtime).asHostObject<ImageAssetImpl>(
                            runtime);
                    auto image_bitmap = image->asObject(runtime).asHostObject<ImageBitmapImpl>(
                            runtime);

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

                        cb->asObject(runtime).asFunction(runtime).call(runtime, Value::null(),
                                                                       std::move(bitmap));

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

                        cb->asObject(runtime).asFunction(runtime).call(runtime, Value::null(),
                                                                       std::move(bitmap));

                        return Value::undefined();
                    }

                    auto object = image->asObject(runtime);

                    // NS Blob
                    auto nsBuffer = object.getProperty(runtime, "_buffer");


                    if (nsBuffer.isNull() || nsBuffer.isUndefined()) {
                        auto error = jsi::JSError(runtime, "Failed to load image");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, error,
                                                                       Value::undefined());
                        return Value::undefined();

                    }

                    if (!nsBuffer.asObject(runtime).isArrayBuffer(runtime)) {

                    }

                    auto ab = nsBuffer.asObject(runtime).getArrayBuffer(runtime);
                    auto data = rust::Slice<const uint8_t>(ab.data(runtime), ab.size(runtime));

                    if (len >= 4 && (sw->isNumber() && sw->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect width is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, error,
                                                                       Value::undefined());
                        return Value::undefined();
                    }
                    if (len >= 5 && (sh->isNumber() && sh->asNumber() == 0)) {
                        auto error = jsi::JSError(runtime,
                                                  "Failed to execute 'createImageBitmap' : The crop rect height is 0");
                        cb->asObject(runtime).asFunction(runtime).call(runtime, error,
                                                                       Value::undefined());
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
                                        cbFunc.call(runtime, jsi::Value::undefined(), object);
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
                                        cbFunc.call(runtime, error, jsi::Value::undefined());
                                    } else {
                                        auto ret = std::make_shared<ImageBitmapImpl>(
                                                std::move(asset));
                                        auto object = jsi::Object::createFromHostObject(runtime,
                                                                                        std::move(
                                                                                                ret));
                                        cbFunc.call(runtime, jsi::Value::undefined(), object);
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
                    auto context_2d = canvas_native_context_create_gl(width, height, density,
                                                                      context,
                                                                      samples, alpha,
                                                                      font_color, ppi, direction);

                    auto obj = CanvasRenderingContext2DImpl(std::move(context_2d));

                }

    );
}
