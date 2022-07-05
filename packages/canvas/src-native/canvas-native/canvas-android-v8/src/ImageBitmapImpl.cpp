//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageBitmapImpl.h"
#include "ImageAssetImpl.h"
#include "OnImageAssetLoadCallbackHolder.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

ImageBitmapImpl::ImageBitmapImpl(rust::Box<ImageAsset> asset)
        : bitmap_(std::move(asset)) {}

void ImageBitmapImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();

    global->Set(context, Helpers::ConvertToV8String(isolate, "ImageBitmap"),
                ctor->GetFunction(context).ToLocalChecked());

    global->Set(context, Helpers::ConvertToV8String(isolate, "createImageBitmap"),
                v8::Function::New(context, &CreateImageBitmap).ToLocalChecked());

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

void ImageBitmapImpl::GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto self = info.This();
    auto ptr = GetPointer(self);
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
    } else {
        info.GetReturnValue().Set(canvas_native_image_asset_width(*ptr->bitmap_));
    }
}

void ImageBitmapImpl::GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

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

Options handleOptions(const v8::Local<v8::Context> &context, const v8::Local<v8::Value> &options) {
    Options ret;

    if (options.IsEmpty() || options->IsNullOrUndefined()) {
        auto isolate = context->GetIsolate();
        auto config = options->ToObject(context).ToLocalChecked();

        auto flipYValue = Helpers::ObjectGet(context, config, Helpers::ConvertToV8String(isolate, "flipY"));
        if (flipYValue->IsBoolean()) {
            ret.flipY = flipYValue->BooleanValue(isolate);
        }

        auto premultiplyAlphaValue = Helpers::ObjectGet(context, config,
                                                        Helpers::ConvertToV8String(isolate, "premultiplyAlpha"));

        if (premultiplyAlphaValue->IsString()) {
            auto premultiplyAlpha = Helpers::ConvertFromV8String(isolate, premultiplyAlphaValue->ToString(
                    context).ToLocalChecked());

            if (premultiplyAlpha == "premultiply") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::Premultiply;
            }

            if (premultiplyAlpha == "none") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::None;
            }
        }

        auto colorSpaceConversionValue = Helpers::ObjectGet(context, config, Helpers::ConvertToV8String(isolate,
                                                                                                        "colorSpaceConversion"));

        if (colorSpaceConversionValue->IsString()) {
            auto colorSpaceConversion = Helpers::ConvertFromV8String(isolate, colorSpaceConversionValue->ToString(
                    context).ToLocalChecked());

            if (colorSpaceConversion == "none") {
                ret.colorSpaceConversion = ImageBitmapColorSpaceConversion::None;
            }
        }

        auto resizeQualityValue = Helpers::ObjectGet(context, config,
                                                     Helpers::ConvertToV8String(isolate, "resizeQuality"));

        if (resizeQualityValue->IsString()) {
            auto resizeQuality = Helpers::ConvertFromV8String(isolate,
                                                              resizeQualityValue->ToString(context).ToLocalChecked());

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

        auto resizeWidthValue = Helpers::ObjectGet(context, config, Helpers::ConvertToV8String(isolate, "resizeWidth"));

        if (resizeQualityValue->IsNumber()) {
            ret.resizeWidth = static_cast<float>(resizeWidthValue->NumberValue(context).FromMaybe(0));
        }

        auto resizeHeightValue = Helpers::ObjectGet(context, config,
                                                    Helpers::ConvertToV8String(isolate, "resizeHeight"));

        if (resizeHeightValue->IsNumber()) {
            ret.resizeHeight = static_cast<float>(resizeHeightValue->NumberValue(context).FromMaybe(0));
        }
    }

    return ret;
}

void ImageBitmapImpl::CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    auto image = args[0];
    auto sx_or_options = args[1];
    auto sy = args[2];
    auto sw = args[3];
    auto sh = args[4];
    auto len = args.Length();

    Options options;

    if (len == 0) {
        Helpers::ThrowIllegalConstructor(isolate);
        return;
    }

    if (image->IsObject()) {
        auto object = image->ToObject(context).ToLocalChecked();
        bool isImageAsset = Helpers::GetInstanceType(isolate, object) == ObjectType::ImageAsset;
        bool isImageBitmap = Helpers::GetInstanceType(isolate, object) == ObjectType::ImageBitmap;
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
                resolver->Reject(context, error);
                return;
            }
            if (len >= 4 && (sw->IsNumber() && sw->Int32Value(context).FromMaybe(0) == 0)) {
                auto error = v8::Exception::RangeError(Helpers::ConvertToV8String(isolate,
                                                                                  "Failed to execute 'createImageBitmap' : The crop rect width is 0"));
                resolver->Reject(context, error);
                return;
            }

            if (len >= 5 && (sh->IsNumber() && sh->Int32Value(context).FromMaybe(0) == 0)) {
                auto error = v8::Exception::RangeError(Helpers::ConvertToV8String(isolate,
                                                                                  "Failed to execute 'createImageBitmap' : The crop rect height is 0"));
                resolver->Reject(context, error);
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

                resolver->Resolve(context, NewInstance(isolate, std::move(ret)));
                return;
            } else if (len == 5 || len == 6) {

                if (len == 6) {
                    options = handleOptions(context, args[5]);
                }

                auto ret = canvas_native_image_bitmap_create_from_asset_src_rect(
                        isImageBitmap ? ibi->GetImageAsset() : iai->GetImageAsset(),
                        sx_or_options->NumberValue(
                                context).ToChecked(),
                        sy->NumberValue(context).ToChecked(),
                        sw->NumberValue(context).ToChecked(),
                        sh->NumberValue(context).ToChecked(),
                        options.flipY,
                        options.premultiplyAlpha,
                        options.colorSpaceConversion,
                        options.resizeQuality,
                        options.resizeWidth,
                        options.resizeHeight);


                resolver->Resolve(context, NewInstance(isolate, std::move(ret)));
                return;
            }

        }
    }
    resolver->Reject(context, Helpers::ConvertToV8String(isolate, "Failed to load image"));
}

v8::Local<v8::Object> ImageBitmapImpl::NewInstance(v8::Isolate *isolate, rust::Box<ImageAsset> bitmap) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, ret, ObjectType::ImageBitmap);
    ImageBitmapImpl *value = new ImageBitmapImpl(std::move(bitmap));
    auto ext = v8::External::New(isolate, value);
    ret->SetInternalField(0, ext);
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

    cache->ImageBitmapTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

ImageAsset &ImageBitmapImpl::GetImageAsset() {
    return *this->bitmap_;
}