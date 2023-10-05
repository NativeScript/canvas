//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageAssetImpl.h"
#include <string>
#include "JSIRuntime.h"
#include "JSICallback.h"
#include "Caches.h"

ImageAssetImpl::ImageAssetImpl(rust::Box<ImageAsset> asset)
        : asset_(std::move(asset)) {
}

void ImageAssetImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "ImageAsset"), func);
}

ImageAssetImpl *ImageAssetImpl::GetPointer(const v8::Local<v8::Object>& object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageAssetImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> ImageAssetImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ImageAssetTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, Ctor);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "ImageAsset"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "width"),
            GetWidth);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "height"),
            GetHeight);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "error"),
            GetError);

    tmpl->Set(
            ConvertToV8String(isolate, "scale"),
            v8::FunctionTemplate::New(isolate, &Scale));

    tmpl->Set(
            ConvertToV8String(isolate, "fromUrlSync"),
            v8::FunctionTemplate::New(isolate, &FromUrlSync));

    tmpl->Set(
            ConvertToV8String(isolate, "fromUrlCb"),
            v8::FunctionTemplate::New(isolate, &FromUrlCb));


    tmpl->Set(
            ConvertToV8String(isolate, "fromFileSync"),
            v8::FunctionTemplate::New(isolate, &FromFileSync));

    tmpl->Set(
            ConvertToV8String(isolate, "fromFileCb"),
            v8::FunctionTemplate::New(isolate, &FromFileCb));


    tmpl->Set(
            ConvertToV8String(isolate, "fromBytesSync"),
            v8::FunctionTemplate::New(isolate, &FromBytesSync));

    tmpl->Set(
            ConvertToV8String(isolate, "fromBytesCb"),
            v8::FunctionTemplate::New(isolate, &FromBytesCb));


    tmpl->Set(
            ConvertToV8String(isolate, "saveSync"),
            v8::FunctionTemplate::New(isolate, &SaveSync));

    tmpl->Set(
            ConvertToV8String(isolate, "saveCb"),
            v8::FunctionTemplate::New(isolate, &SaveCb));


    cache->ImageAssetTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void ImageAssetImpl::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ret = args.This();

    SetNativeType(isolate, ret, NativeType::ImageAsset);

    auto image_asset = canvas_native_image_asset_create();

    auto object = new ImageAssetImpl(std::move(image_asset));

    auto ext = v8::External::New(isolate, object);

    ret->SetInternalField(0, ext);

    args.GetReturnValue().Set(ret);
}

void
ImageAssetImpl::GetWidth(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_image_asset_width(ptr->GetImageAsset());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}

void
ImageAssetImpl::GetHeight(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_image_asset_height(ptr->GetImageAsset());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}

void
ImageAssetImpl::GetError(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_image_asset_get_error(ptr->GetImageAsset());
        auto isolate = info.GetIsolate();
        info.GetReturnValue().Set(ConvertToV8String(isolate, ret.c_str()));
        return;
    }
    info.GetReturnValue().SetEmptyString();
}

void ImageAssetImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto x = (uint32_t) args[0]->NumberValue(context).ToChecked();
        auto y = (uint32_t) args[1]->NumberValue(context).ToChecked();
        if (x > 0 && y > 0) {
            canvas_native_image_asset_scale(
                    ptr->GetImageAsset(), x,
                    y);
        }
    }

    args.GetReturnValue().SetUndefined();
}

void ImageAssetImpl::FromUrlSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }
    auto isolate = args.GetIsolate();
    auto url = ConvertFromV8String(isolate, args[0]);

    auto done = canvas_native_image_asset_load_from_url(
            ptr->GetImageAsset(),
            rust::Str(url.c_str()));


    args.GetReturnValue().Set(done);
}

void ImageAssetImpl::FromUrlCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();

    if (args.Length() < 2) {
        return;
    }

    auto url = ConvertFromV8String(isolate, args[0]);

    auto asset = canvas_native_image_asset_shared_clone(
            ptr->GetImageAsset());

    v8::Global<v8::Function> callback(isolate, args[1].As<v8::Function>());

    auto jsi_callback = new JSICallback(isolate, std::move(callback));

    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  ALOOPER_POLL_CALLBACK,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
                      auto cb = static_cast<JSICallback *>(data);
                      bool done;
                      read(fd, &done,
                           sizeof(bool));

                      v8::Isolate *isolate = cb->isolate_;
                      v8::Locker locker(isolate);

                      v8::HandleScope handle_scope(isolate);
                      v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                      v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                      v8::Context::Scope context_scope(context);

                      v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};

                      // v8::TryCatch tc(isolate);

                      callback->Call(context, context->Global(), 1,
                                     args);  // ignore JS return value

                      delete static_cast<JSICallback *>(data);
                      return 0;
                  }, jsi_callback);

    ALooper_wake(jsi_callback->looper_);


    std::thread thread(
            [jsi_callback](
                    const std::string &url,
                    rust::Box<ImageAsset> asset) {

                auto done = canvas_native_image_asset_load_from_url(
                        *asset,
                        rust::Str(
                                url.c_str()));

                write(jsi_callback->fd_[1],
                      &done,
                      sizeof(bool));

            }, std::move(url),
            std::move(asset));

    thread.detach();
}

void ImageAssetImpl::FromFileSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }
    auto isolate = args.GetIsolate();
    auto path = ConvertFromV8String(isolate, args[0]);

    auto done = canvas_native_image_asset_load_from_path(
            ptr->GetImageAsset(),
            rust::Str(path.c_str()));


    args.GetReturnValue().Set(done);
}

void ImageAssetImpl::FromFileCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();

    if (args.Length() < 2) {
        return;
    }


    auto path = ConvertFromV8String(isolate, args[0]);

    auto asset = canvas_native_image_asset_shared_clone(
            ptr->GetImageAsset());

    v8::Global<v8::Function> callback(isolate, args[1].As<v8::Function>());


    auto jsi_callback = new JSICallback(isolate, std::move(callback));

    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  ALOOPER_POLL_CALLBACK,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
                      auto cb = static_cast<JSICallback *>(data);
                      bool done;
                      read(fd, &done,
                           sizeof(bool));

                      v8::Isolate *isolate = cb->isolate_;
                      v8::Locker locker(isolate);

                      v8::HandleScope handle_scope(isolate);
                      v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                      v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                      v8::Context::Scope context_scope(context);

                      v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};

                      // v8::TryCatch tc(isolate);

                      callback->Call(context, context->Global(), 1,
                                     args);  // ignore JS return value

                      delete static_cast<JSICallback *>(data);
                      return 0;
                  }, jsi_callback);

    ALooper_wake(jsi_callback->looper_);


    std::thread thread(
            [jsi_callback](
                    const std::string &path,
                    rust::Box<ImageAsset> asset) {

                auto done = canvas_native_image_asset_load_from_path(
                        *asset,
                        rust::Str(
                                path.c_str()));

                write(jsi_callback->fd_[1],
                      &done,
                      sizeof(bool));

            }, std::move(path),
            std::move(asset));

    thread.detach();
}

void ImageAssetImpl::FromBytesSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto value = args[0];

    if (value->IsObject()) {
        if (!value->IsArrayBuffer()) {
            args.GetReturnValue().Set(false);
            return;
        }
        auto buf = value.As<v8::ArrayBuffer>();

        auto size = buf->ByteLength();
        auto data = (uint8_t *) buf->GetBackingStore()->Data();
        auto done = canvas_native_image_asset_load_from_raw(
                ptr->GetImageAsset(),
                rust::Slice<const uint8_t>(
                        data, size));

        args.GetReturnValue().Set(done);
    }

    args.GetReturnValue().Set(false);
}

void ImageAssetImpl::FromBytesCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();

    if (args.Length() < 2) {
        return;
    }


    auto bytes = args[0].As<v8::ArrayBuffer>();

    auto size = bytes->ByteLength();
    auto data = (uint8_t *) bytes->GetBackingStore()->Data();
    auto buf = rust::Slice<const uint8_t>(data,
                                          size);

    auto asset = canvas_native_image_asset_shared_clone(
            ptr->GetImageAsset());

    v8::Global<v8::Function> callback(isolate, args[1].As<v8::Function>());

    auto jsi_callback = new JSICallback(isolate, std::move(callback));

    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  ALOOPER_POLL_CALLBACK,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
                      auto cb = static_cast<JSICallback *>(data);
                      bool done;
                      read(fd, &done,
                           sizeof(bool));

                      v8::Isolate *isolate = cb->isolate_;
                      v8::Locker locker(isolate);

                      v8::HandleScope handle_scope(isolate);
                      v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                      v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                      v8::Context::Scope context_scope(context);

                      v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};

                      // v8::TryCatch tc(isolate);

                      callback->Call(context, context->Global(), 1,
                                     args);  // ignore JS return value

                      delete static_cast<JSICallback *>(data);
                      return 0;
                  }, jsi_callback);

    ALooper_wake(jsi_callback->looper_);


    std::thread thread(
            [jsi_callback, &buf](rust::Box<ImageAsset> asset) {

                auto done = canvas_native_image_asset_load_from_raw(
                        *asset, buf);

                write(jsi_callback->fd_[1],
                      &done,
                      sizeof(bool));

            }, std::move(asset));

    thread.detach();
}

void ImageAssetImpl::SaveSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto path = ConvertFromV8String(isolate, args[0]);
    auto format = (uint32_t) args[1]->NumberValue(context).ToChecked();
    auto done = canvas_native_image_asset_save_path(
            ptr->GetImageAsset(),
            rust::Str(path.c_str()), format);

    args.GetReturnValue().Set(done);
}

void ImageAssetImpl::SaveCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (args.Length() < 2) {
        return;
    }


    auto path = ConvertFromV8String(isolate, args[0]);

    auto format = (uint32_t) args[1]->NumberValue(context).ToChecked();

    auto asset = canvas_native_image_asset_shared_clone(
            ptr->GetImageAsset());

    v8::Global<v8::Function> callback(isolate, args[1].As<v8::Function>());

    auto jsi_callback = new JSICallback(isolate, std::move(callback));

    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  ALOOPER_POLL_CALLBACK,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
                      auto cb = static_cast<JSICallback *>(data);
                      bool done;
                      read(fd, &done,
                           sizeof(bool));

                      v8::Isolate *isolate = cb->isolate_;
                      v8::Locker locker(isolate);

                      v8::HandleScope handle_scope(isolate);
                      v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                      v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                      v8::Context::Scope context_scope(context);

                      v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};

                      // v8::TryCatch tc(isolate);

                      callback->Call(context, context->Global(), 1,
                                     args);  // ignore JS return value

                      delete static_cast<JSICallback *>(data);
                      return 0;
                  }, jsi_callback);

    ALooper_wake(jsi_callback->looper_);


    std::thread thread(
            [jsi_callback](
                    const std::string &path,
                    std::uint32_t format,
                    rust::Box<ImageAsset> asset) {

                auto done = canvas_native_image_asset_save_path(
                        *asset,
                        rust::Str(
                                path.c_str()),
                        format);


                write(jsi_callback->fd_[1],
                      &done,
                      sizeof(bool));

            }, std::move(path), format,
            std::move(asset));


    thread.detach();
}

ImageAsset &ImageAssetImpl::GetImageAsset() {
    return *this->asset_;
}

