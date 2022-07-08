//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageAssetImpl.h"
#include "OnImageAssetLoadCallbackHolder.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

ImageAssetImpl::ImageAssetImpl(rust::Box<ImageAsset> asset)
        : asset_(std::move(asset)) {}

void ImageAssetImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "ImageAsset"),
                ctor->GetFunction(context).ToLocalChecked());

}

ImageAssetImpl *ImageAssetImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageAssetImpl *>(ptr);
}

void ImageAssetImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    if (!args.IsConstructCall()) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(
                        isolate,
                        "Please use the 'new' operator, this object constructor cannot be called as a function."
                )
        );
        isolate->ThrowException(err);
        return;
    } else {
        v8::Local<v8::Object> ret = args.This();
        Helpers::SetInstanceType(isolate, ret, ObjectType::ImageAsset);
        ImageAssetImpl *asset = new ImageAssetImpl(std::move(canvas_native_image_asset_create()));
        AddWeakListener(isolate, ret, asset);
        args.GetReturnValue().Set(ret);
    }
}

void ImageAssetImpl::GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(canvas_native_image_asset_width(*ptr->asset_));
}

void ImageAssetImpl::GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(canvas_native_image_asset_height(*ptr->asset_));
}

void ImageAssetImpl::GetError(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto has_error = canvas_native_image_asset_has_error(*ptr->asset_);
    if (!has_error) {
        info.GetReturnValue().Set(v8::Undefined(isolate));
    } else {
        auto error = canvas_native_image_asset_get_error(*ptr->asset_);
        info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, error.c_str()));
    }
}

void ImageAssetImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto x = args[0]->Int32Value(context).FromMaybe(0);
    auto y = args[1]->Int32Value(context).FromMaybe(0);
    if (x > 0 && y > 0) {
        canvas_native_image_asset_scale(*ptr->asset_, static_cast<uint32_t>(x), static_cast<uint32_t>(y));
    }
}

void ImageAssetImpl::FromUrl(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ptr = GetPointer(args.This());
    if (Helpers::IsString(args[0])) {
        auto url = Helpers::GetString(isolate, args[0]);
        auto done = canvas_native_image_asset_load_from_url(*ptr->asset_, rust::Str(url.c_str(), url.size()));
        args.GetReturnValue().Set(done);
    } else {
        args.GetReturnValue().Set(false);
    }
}

void ImageAssetImpl::FromUrlAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto url = args[0];
    auto cb = args[1];
    if (Helpers::IsString(url) && cb->IsFunction()) {
        auto val = Helpers::GetString(isolate, url);
        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb.As<v8::Function>());
        auto cache = Caches::Get(isolate);
        auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
        cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);
        canvas_native_image_asset_load_from_url_async(*ptr->asset_, rust::Str(val.c_str(), val.size()), key);
    };
}

void ImageAssetImpl::FromFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (Helpers::IsString(args[0])) {
        auto path = Helpers::GetString(isolate, args[0]);
        auto done = canvas_native_image_asset_load_from_path(ptr->GetImageAsset(),
                                                             rust::Str(path.c_str(), path.size()));
        args.GetReturnValue().Set(done);
    }
}

void ImageAssetImpl::FromFileAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto file = args[0];
    auto cb = args[1];
    if (Helpers::IsString(file) && cb->IsFunction()) {
        auto path = Helpers::GetString(isolate, file);
        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb.As<v8::Function>());
        auto cache = Caches::Get(isolate);
        auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
        cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);
        canvas_native_image_asset_load_from_path_async(*ptr->asset_, rust::Str(path.c_str(), path.size()), key);
    };
}

void ImageAssetImpl::FromBytes(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (args[0]->IsUint8Array() || args[0]->IsUint8ClampedArray()) {
        auto buf = args[0].As<v8::TypedArray>();
        auto offset = buf->ByteOffset();
        auto size = buf->Length();
        auto store = buf->Buffer()->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto buffer = rust::Slice<const uint8_t>(data, size);
        auto done = canvas_native_image_asset_load_from_raw(*ptr->asset_, buffer);
        args.GetReturnValue().Set(done);
    }
}

void ImageAssetImpl::FromBytesAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto cb = args[1];
    if ((args[0]->IsUint8Array() || args[0]->IsUint8ClampedArray()) && cb->IsFunction()) {
        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb.As<v8::Function>());

        auto cache = Caches::Get(isolate);
        auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
        cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);

        auto buf = args[0].As<v8::TypedArray>();
        auto offset = buf->ByteOffset();
        auto size = buf->Length();
        auto store = buf->Buffer()->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto buffer = rust::Slice<const uint8_t>(data, size);
        canvas_native_image_asset_load_from_raw_async(*ptr->asset_, buffer, key);
    };
}

void ImageAssetImpl::Save(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto path = Helpers::GetString(isolate, args[0]);
    auto format = args[1]->Int32Value(context).FromMaybe(0);
    auto done = canvas_native_image_asset_save_path(*ptr->asset_, rust::Str(path.c_str(), path.size()), format);
    args.GetReturnValue().Set(done);
}

void ImageAssetImpl::SaveAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto val = args[0];
    auto formatVal = args[1];
    auto cb = args[2];
    if (Helpers::IsString(val) && Helpers::IsNumber(formatVal) && cb->IsFunction()) {
        auto path = Helpers::GetString(isolate, val->ToString(context).ToLocalChecked());
        auto format = formatVal->Int32Value(context).FromMaybe(0);
        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb.As<v8::Function>());
        auto cache = Caches::Get(isolate);
        auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
        cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);
        canvas_native_image_asset_save_path_async(*ptr->asset_, rust::Str(path.c_str(), path.size()), format, key);
    }
}

v8::Local<v8::FunctionTemplate> ImageAssetImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ImageAssetTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ImageAsset"));

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

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "error"),
            &GetError
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "scale"),
            v8::FunctionTemplate::New(isolate, &Scale)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadUrlSync"),
            v8::FunctionTemplate::New(isolate, &FromUrl)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadUrl"),
            v8::FunctionTemplate::New(isolate, &FromUrlAsync)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFileSync"),
            v8::FunctionTemplate::New(isolate, &FromFile)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFile"),
            v8::FunctionTemplate::New(isolate, &FromFileAsync)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadBytesSync"),
            v8::FunctionTemplate::New(isolate, &FromBytes)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadBytes"),
            v8::FunctionTemplate::New(isolate, &FromBytesAsync)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "saveSync"),
            v8::FunctionTemplate::New(isolate, &Save)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "save"),
            v8::FunctionTemplate::New(isolate, &SaveAsync)
    );

    cache->ImageAssetTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

ImageAsset &ImageAssetImpl::GetImageAsset() {
    return *this->asset_;
}