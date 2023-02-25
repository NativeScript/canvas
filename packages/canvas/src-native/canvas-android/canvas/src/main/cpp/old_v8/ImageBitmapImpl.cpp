//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageBitmapImpl.h"
#include "ImageAssetImpl.h"
#include "OnImageBitmapLoadCallbackHolder.h"
#include "rust/cxx.h"

ImageBitmapImpl::ImageBitmapImpl(rust::Box<ImageAsset> asset)
        : bitmap_(std::move(asset)) {}

void ImageBitmapImpl::Init(v8::Isolate *isolate) {
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();

    global->Set(context, Helpers::ConvertToV8String(isolate, "ImageBitmap"),
                ctor->GetFunction(context).ToLocalChecked());

    auto tmpl = v8::FunctionTemplate::New(isolate, &CreateImageBitmap);

    global->Set(context, Helpers::ConvertToV8String(isolate, "_createImageBitmap"),
                tmpl->GetFunction(context).ToLocalChecked());

}

ImageBitmapImpl *ImageBitmapImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageBitmapImpl *>(ptr);
}

void ImageBitmapImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

void ImageBitmapImpl::GetWidth(v8::Local<v8::String> name,
                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto self = info.This();
    auto ptr = GetPointer(self);
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
    } else {
        info.GetReturnValue().Set(canvas_native_image_asset_width(*ptr->bitmap_));
    }
}

void ImageBitmapImpl::GetHeight(v8::Local<v8::String> name,
                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();

    auto self = info.This();
    auto ptr = GetPointer(self);

    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
    } else {
        info.GetReturnValue().Set(canvas_native_image_asset_height(*ptr->bitmap_));
    }
}

void ImageBitmapImpl::Close(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto self = args.This();
    auto ext = self->GetInternalField(0);
    if (!ext.IsEmpty()) {
        ext.Clear();
    }
}

struct Options {
    bool flipY = false;
    ImageBitmapPremultiplyAlpha premultiplyAlpha = ImageBitmapPremultiplyAlpha::Default;
    ImageBitmapColorSpaceConversion colorSpaceConversion = ImageBitmapColorSpaceConversion::Default;
    ImageBitmapResizeQuality resizeQuality = ImageBitmapResizeQuality::Low;
    float resizeWidth = 0;
    float resizeHeight = 0;
};

Options
handleOptions(const v8::Local<v8::Context> &context, const v8::Local<v8::Value> &options) {
    Options ret;

    if (options.IsEmpty() || options->IsNullOrUndefined()) {
        auto isolate = context->GetIsolate();
        auto config = options->ToObject(context).ToLocalChecked();


        auto flipYValue = config->Get(context, Helpers::ConvertToV8String(isolate, "flipY"));
        if (!flipYValue.IsEmpty()) {
            ret.flipY = flipYValue.ToLocalChecked()->BooleanValue(isolate);
        }

        auto premultiplyAlphaValue = config->Get(context,
                                                 Helpers::ConvertToV8String(isolate,
                                                                            "premultiplyAlpha"));

        if (!premultiplyAlphaValue.IsEmpty()) {
            auto premultiplyAlpha = Helpers::ConvertFromV8String(isolate,
                                                                 premultiplyAlphaValue.ToLocalChecked()->ToString(
                                                                         context).ToLocalChecked());

            if (premultiplyAlpha == "premultiply") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::Premultiply;
            }

            if (premultiplyAlpha == "none") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::None;
            }
        }

        auto colorSpaceConversionValue = config->Get(context,
                                                     Helpers::ConvertToV8String(isolate,
                                                                                "colorSpaceConversion"));

        if (!colorSpaceConversionValue.IsEmpty()) {
            auto colorSpaceConversion = Helpers::ConvertFromV8String(isolate,
                                                                     colorSpaceConversionValue.ToLocalChecked()->ToString(
                                                                             context).ToLocalChecked());

            if (colorSpaceConversion == "none") {
                ret.colorSpaceConversion = ImageBitmapColorSpaceConversion::None;
            }
        }

        auto resizeQualityValue = config->Get(context,
                                              Helpers::ConvertToV8String(isolate, "resizeQuality"));

        if (!resizeQualityValue.IsEmpty()) {
            auto resizeQuality = Helpers::ConvertFromV8String(isolate,
                                                              resizeQualityValue.ToLocalChecked()->ToString(
                                                                      context).ToLocalChecked());

            if (resizeQuality == "medium") {
                ret.resizeQuality = ImageBitmapResizeQuality::Medium;
            }

            if (resizeQuality == "high") {
                ret.resizeQuality = ImageBitmapResizeQuality::High;
            }

            if (resizeQuality == "pixelated") {
                ret.resizeQuality = ImageBitmapResizeQuality::Pixelated;
            }
        }

        auto resizeWidthValue = config->Get(context,
                                            Helpers::ConvertToV8String(isolate, "resizeWidth"));

        if (!resizeWidthValue.IsEmpty()) {
            auto val = resizeWidthValue.ToLocalChecked();
            if (val->IsNumber()) {
                ret.resizeWidth = static_cast<float>(val->NumberValue(
                        context).FromMaybe(
                        0));
            }
        }

        auto resizeHeightValue = config->Get(context,
                                             Helpers::ConvertToV8String(isolate, "resizeHeight"));

        if (!resizeHeightValue.IsEmpty()) {
            auto val = resizeHeightValue.ToLocalChecked();
            if (val->IsNumber()) {
                ret.resizeHeight = static_cast<float>(val->NumberValue(
                        context).FromMaybe(
                        0));
            }
        }
    }

    return ret;
}

void ImageBitmapImpl::CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::LogToConsole("CreateImageBitmap");
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto image = args[0];
    auto sx_or_options = args[1];
    auto sy = args[2];
    auto sw = args[3];
    auto sh = args[4];
    auto len = args.Length();

    auto cb = args[len - 1];
    len = len - 1;

    if (len == 1 && args[0]->IsFunction()) {
        Helpers::ThrowIllegalConstructor(isolate);
        return;
    }

    Options options;


    if (len == 0) {
        Helpers::ThrowIllegalConstructor(isolate);
        return;
    }

    if (!cb->IsFunction()) {
        Helpers::ThrowIllegalConstructor(isolate);
        return;
    }

    if (image.IsEmpty() || image->IsNullOrUndefined()) {
        v8::Local<v8::Value> ret[2] = {Helpers::ConvertToV8String(isolate, "Failed to load image"),
                                       v8::Null(isolate).As<v8::Value>()};
        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }

    auto object = image.As<v8::Object>();

    auto instance = ObjectType::Unknown; //Helpers::GetInstanceType(isolate, object);


    auto key = v8::Private::ForApi(isolate, Helpers::ConvertToV8String(isolate, "__instanceType"));

    auto has = object->HasPrivate(context, key).FromMaybe(false);

    if (has) {
        auto value = object->GetPrivate(context, key);

        if (!value.IsEmpty()) {
            auto val = value.ToLocalChecked();

            if (val->IsNumber()) {
                auto ret = val->Uint32Value(context);
                if (ret.IsJust()) {
                    instance = (ObjectType) ret.ToChecked();
                }
            }


        }
    }


    bool isImageAsset = instance == ObjectType::ImageAsset;
    bool isImageBitmap = instance == ObjectType::ImageBitmap;
    if (isImageAsset || isImageBitmap) {
        ImageBitmapImpl *ibi = nullptr;
        ImageAssetImpl *iai = nullptr;
        if (isImageBitmap) {
            ibi = ImageBitmapImpl::GetPointer(object);
        }

        if (isImageAsset) {
            iai = ImageAssetImpl::GetPointer(object);
        }

        if (ibi == nullptr && iai == nullptr) {
            auto error = v8::Exception::RangeError(Helpers::ConvertToV8String(isolate,
                                                                              "Failed to execute 'createImageBitmap' : The crop rect width is 0"));

            v8::Local<v8::Value> argv[2] = {error, v8::Null(isolate)};

            cb.As<v8::Function>()->Call(context, context->Global(), 2, argv);

            return;
        }
        if (len >= 4 && (sw->IsNumber() && sw->Int32Value(context).FromMaybe(0) == 0)) {
            auto error = v8::Exception::RangeError(Helpers::ConvertToV8String(isolate,
                                                                              "Failed to execute 'createImageBitmap' : The crop rect width is 0"));
            v8::Local<v8::Value> argv[2] = {error, v8::Null(isolate)};
            cb.As<v8::Function>()->Call(context, context->Global(), 2, argv);
            return;
        }
        if (len >= 5 && (sh->IsNumber() && sh->Int32Value(context).FromMaybe(0) == 0)) {
            auto error = v8::Exception::RangeError(Helpers::ConvertToV8String(isolate,
                                                                              "Failed to execute 'createImageBitmap' : The crop rect height is 0"));
            v8::Local<v8::Value> argv[2] = {error, v8::Null(isolate)};
            cb.As<v8::Function>()->Call(context, context->Global(), 2, argv);
            return;
        }
        if (len == 1 || len == 2) {
            if (len == 2) {
                options = handleOptions(context, sx_or_options);
            };
            auto ret = canvas_native_image_bitmap_create_from_asset(
                    isImageBitmap ? ibi->GetImageAsset() : iai->GetImageAsset(),
                    options.flipY,
                    options.premultiplyAlpha,
                    options.colorSpaceConversion,
                    options.resizeQuality,
                    options.resizeWidth,
                    options.resizeHeight);

            v8::Local<v8::Value> argv[2] = {v8::Null(isolate),
                                            NewInstance(isolate, std::move(ret))};
            cb.As<v8::Function>()->Call(context, context->Global(), 2, argv);
            return;
        } else if (len == 5 || len == 6) {

            if (len == 6) {
                options = handleOptions(context, args[5]);
            }

            auto ret = canvas_native_image_bitmap_create_from_asset_src_rect(
                    isImageBitmap ? ibi->GetImageAsset() : iai->GetImageAsset(),
                    (float) sx_or_options->NumberValue(
                            context).ToChecked(),
                    (float) sy->NumberValue(context).ToChecked(),
                    (float) sw->NumberValue(context).ToChecked(),
                    (float) sh->NumberValue(context).ToChecked(),
                    options.flipY,
                    options.premultiplyAlpha,
                    options.colorSpaceConversion,
                    options.resizeQuality,
                    options.resizeWidth,
                    options.resizeHeight);

            v8::Local<v8::Value> argv[2] = {v8::Null(isolate),
                                            NewInstance(isolate, std::move(ret))};
            cb.As<v8::Function>()->Call(context, context->Global(), 2, argv);
            return;
        }

    } //else {


    // v8::Local<v8::Value> nsBuffer;
    auto nsBuffer = object->Get(context,
                                Helpers::ConvertToV8String(isolate, "_buffer"));


    if (nsBuffer.IsEmpty() || nsBuffer.ToLocalChecked()->IsNullOrUndefined()) {
        v8::Local<v8::Value> ret[2] = {Helpers::ConvertToV8String(isolate, "Failed to load image"),
                                       v8::Null(isolate).As<v8::Value>()};
        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }


    auto bufferValue = nsBuffer.ToLocalChecked();
    if (bufferValue->IsArrayBufferView() || bufferValue->IsTypedArray()) {
        auto data = Helpers::GetTypedArrayData<const uint8_t>(bufferValue.As<v8::TypedArray>());

        if (len >= 4 && (sw->IsNumber() && sw->Int32Value(context).FromMaybe(0) == 0)) {
            auto error = v8::Exception::RangeError(Helpers::ConvertToV8String(isolate,
                                                                              "Failed to execute 'createImageBitmap' : The crop rect width is 0"));
            v8::Local<v8::Value> argv[2] = {error, v8::Null(isolate)};
            cb.As<v8::Function>()->Call(context, context->Global(), 2, argv);
            return;
        }

        if (len >= 5 && (sh->IsNumber() && sh->Int32Value(context).FromMaybe(0) == 0)) {
            auto error = v8::Exception::RangeError(Helpers::ConvertToV8String(isolate,
                                                                              "Failed to execute 'createImageBitmap' : The crop rect height is 0"));
            v8::Local<v8::Value> argv[2] = {error, v8::Null(isolate)};
            cb.As<v8::Function>()->Call(context, context->Global(), 2, argv);
            return;
        }

        if (len == 1 || len == 2) {
            Helpers::LogToConsole("len == 1 || len == 2");
            if (len == 2) {
                options = handleOptions(context, sx_or_options);
            }

            auto asset = canvas_native_image_asset_create();
            auto asset_ptr = asset.into_raw();


            auto func = v8::Function::New(context,
                                          [](const v8::FunctionCallbackInfo<v8::Value> &info) {
                                              auto isolate = info.GetIsolate();
                                              v8::Locker locker(isolate);
                                              v8::Isolate::Scope isolate_scope(isolate);
                                              v8::HandleScope handle_scope(isolate);
                                              auto context = isolate->GetCurrentContext();
                                              auto cb = info.Data();
                                              v8::Local<v8::Value> argv[2] = {info[0], info[1]};
                                              cb.As<v8::Function>()->Call(context,
                                                                          context->Global(), 2,
                                                                          argv);
                                          }, cb).ToLocalChecked();

            auto callback = std::make_shared<OnImageBitmapLoadCallbackHolder>(isolate, context,
                                                                              func,
                                                                              asset_ptr);


            std::thread thread([](rust::Box<ImageAsset> asset,
                                  rust::Slice<const uint8_t> data,
                                  bool flipY,
                                  ImageBitmapPremultiplyAlpha premultiplyAlpha,
                                  ImageBitmapColorSpaceConversion colorSpaceConversion,
                                  ImageBitmapResizeQuality resizeQuality,
                                  float resizeWidth,
                                  float resizeHeight,
                                  const std::shared_ptr<OnImageBitmapLoadCallbackHolder> &callback) {

                                   auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(data,
                                                                                                                flipY,
                                                                                                                premultiplyAlpha,
                                                                                                                colorSpaceConversion,
                                                                                                                resizeQuality,
                                                                                                                resizeWidth,
                                                                                                                resizeHeight,
                                                                                                                *asset);


                                   if (callback != nullptr) {
                                       callback->complete(done);
                                   }
                               }, std::move(asset), data,
                               options.flipY,
                               options.premultiplyAlpha,
                               options.colorSpaceConversion,
                               options.resizeQuality,
                               options.resizeWidth,
                               options.resizeHeight, std::move(callback));

            return;
        } else if (len == 5 || len == 6) {

            if (len == 6) {
                options = handleOptions(context, args[5]);
            }

            auto asset = canvas_native_image_asset_create();

            auto func = v8::Function::New(context,
                                          [](const v8::FunctionCallbackInfo<v8::Value> &info) {
                                              auto isolate = info.GetIsolate();
                                              auto context = isolate->GetCurrentContext();
                                              auto cb = info.Data();
                                              v8::Local<v8::Value> argv[2] = {info[0], info[1]};
                                              cb.As<v8::Function>()->Call(context,
                                                                          context->Global(), 2,
                                                                          argv);
                                          }, cb).ToLocalChecked();


            auto call = func->NewInstance(context).ToLocalChecked();

            auto callback = std::make_shared<OnImageBitmapLoadCallbackHolder>(isolate, context,
                                                                              call.As<v8::Function>(),
                                                                              canvas_native_image_asset_shared_clone(
                                                                                      *asset).into_raw());


            std::thread thread([](rust::Box<ImageAsset> asset,
                                  rust::Slice<const uint8_t> data,
                                  float sx_or_options,
                                  float sy,
                                  float sw,
                                  float sh,
                                  bool flipY,
                                  ImageBitmapPremultiplyAlpha premultiplyAlpha,
                                  ImageBitmapColorSpaceConversion colorSpaceConversion,
                                  ImageBitmapResizeQuality resizeQuality,
                                  float resizeWidth,
                                  float resizeHeight,
                                  const std::shared_ptr<OnImageBitmapLoadCallbackHolder> &callback) {

                                   auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                           data,
                                           sx_or_options,
                                           sy,
                                           sw,
                                           sh,
                                           flipY,
                                           premultiplyAlpha,
                                           colorSpaceConversion,
                                           resizeQuality,
                                           resizeWidth,
                                           resizeHeight, *asset);


                                   if (callback != nullptr) {
                                       callback->complete(done);
                                   }
                               }, std::move(asset), data,
                               sx_or_options->NumberValue(
                                       context).ToChecked(),
                               sy->NumberValue(
                                       context).ToChecked(),
                               sw->NumberValue(
                                       context).ToChecked(),
                               sh->NumberValue(
                                       context).ToChecked(),
                               options.flipY,
                               options.premultiplyAlpha,
                               options.colorSpaceConversion,
                               options.resizeQuality,
                               options.resizeWidth,
                               options.resizeHeight, std::move(callback));

            return;
        }


    }
    //  }

}

v8::Local<v8::Object>
ImageBitmapImpl::NewInstance(v8::Isolate *isolate, rust::Box<ImageAsset> bitmap) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, ret, ObjectType::ImageBitmap);
    auto *value = new ImageBitmapImpl(std::move(bitmap));
    AddWeakListener(isolate, ret, value);
    return handle_scope.Escape(ret);
}

v8::Local<v8::FunctionTemplate> ImageBitmapImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ImageBitmapTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ImageBitmap"));

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);

    auto tmpl = ctorTmpl->PrototypeTemplate();

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "width"),
            &GetWidth
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "height"),
            &GetHeight
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "close"),
            v8::FunctionTemplate::New(isolate, &Close)
    );

    cache->ImageBitmapTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate >>(isolate, ctorTmpl);
    return ctorTmpl;
}

ImageAsset &ImageBitmapImpl::GetImageAsset() {
    return *this->bitmap_;
}
