//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageAssetImpl.h"
#include <string>
#include "JSIRuntime.h"
#include "JSICallback.h"
#include "Caches.h"

#include "Helpers.h"
#include "Common.h"

ImageAssetImpl::ImageAssetImpl(const ImageAsset *asset) : asset_(asset) {
}

ImageAssetImpl::~ImageAssetImpl() {
    canvas_native_image_asset_release(this->GetImageAsset());
    asset_ = nullptr;
}

void ImageAssetImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();
    
    canvasModule->Set(context, ConvertToV8String(isolate, "ImageAsset"), func).IsJust();
}

ImageAssetImpl *ImageAssetImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
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
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "ImageAsset"));
    
    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);
    tmpl->SetAccessor(
                      ConvertToV8String(isolate, "width"),
                      GetWidth);
    tmpl->SetAccessor(
                      ConvertToV8String(isolate, "height"),
                      GetHeight);
    tmpl->SetAccessor(
                      ConvertToV8String(isolate, "error"),
                      GetError);
    
    tmpl->SetAccessor(
                      ConvertToV8String(isolate, "__addr"),
                      GetAddr);
    
    tmpl->Set(
              ConvertToV8String(isolate, "__getRef"),
              v8::FunctionTemplate::New(isolate, GetReference));
    
    
    
    //    tmpl->Set(
    //            ConvertToV8String(isolate, "scale"),
    //            v8::FunctionTemplate::New(isolate, &Scale));
    
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
              ConvertToV8String(isolate, "fromEncodedBytesSync"),
              v8::FunctionTemplate::New(isolate, &FromEncodedBytesSync));
    
    tmpl->Set(
              ConvertToV8String(isolate, "fromEncodedBytesCb"),
              v8::FunctionTemplate::New(isolate, &FromEncodedBytesCb));
    
    
    /*   tmpl->Set(
     ConvertToV8String(isolate, "saveSync"),
     v8::FunctionTemplate::New(isolate, &SaveSync));
     
     tmpl->Set(
     ConvertToV8String(isolate, "saveCb"),
     v8::FunctionTemplate::New(isolate, &SaveCb));
     */
    
    cache->ImageAssetTmpl =
    std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void ImageAssetImpl::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    
    auto ret = args.This();
    
    auto image_asset = canvas_native_image_asset_create();
    
    auto object = new ImageAssetImpl(image_asset);
    
    SetNativeType(object, NativeType::ImageAsset);
    
    ret->SetAlignedPointerInInternalField(0, object);
    
    object->BindFinalizer(isolate, ret);
    
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
ImageAssetImpl::GetAddr(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto isolate = info.GetIsolate();
        auto ret = std::to_string(canvas_native_image_asset_get_addr(ptr->GetImageAsset()));
        info.GetReturnValue().Set(ConvertToV8String(isolate, ret));
        return;
    }
    info.GetReturnValue().SetEmptyString();
}


void
ImageAssetImpl::GetReference(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto isolate = args.GetIsolate();
        auto reference = canvas_native_image_asset_reference(ptr->GetImageAsset());
        auto ret = std::to_string(canvas_native_image_asset_get_addr(reference));
        args.GetReturnValue().Set(ConvertToV8String(isolate, ret));
        return;
    }
    
    args.GetReturnValue().SetEmptyString();
}

void
ImageAssetImpl::GetError(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_image_asset_get_error(ptr->GetImageAsset());
        auto isolate = info.GetIsolate();
        info.GetReturnValue().Set(ConvertToV8String(isolate, ret));
        canvas_native_string_destroy((char *) ret);
        return;
    }
    info.GetReturnValue().SetEmptyString();
}

void ImageAssetImpl::FromUrlSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }
    auto isolate = args.GetIsolate();
    auto url = ConvertFromV8String(isolate, args[0]);
    
    auto done = canvas_native_image_asset_load_from_url(ptr->GetImageAsset(), url.c_str());
    
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
    
    auto asset = canvas_native_image_asset_reference(ptr->asset_);
    
    auto callback = args[1].As<v8::Function>();
    
    auto jsi_callback = new JSICallback(isolate, callback);
    
#ifdef __ANDROID__
    
    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  0,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
        auto cb = static_cast<JSICallback *>(data);
        bool done;
        read(fd, &done,
             sizeof(bool));
        
        v8::Isolate *isolate = cb->isolate_;
        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);
        v8::Local<v8::Function> callback = cb->callback_->Get(isolate);
        v8::Local<v8::Context> context = callback->GetCreationContextChecked();
        v8::Context::Scope context_scope(context);
        
        v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
        
        // v8::TryCatch tc(isolate);
        
        callback->Call(context, context->Global(), 1,
                       args);  // ignore JS return value
        
        
        delete static_cast<JSICallback *>(data);
        return 0;
    }, jsi_callback);
    
    
    std::thread thread(
                       [jsi_callback, asset](
                                             const std::string &url) {
                                                 auto done = canvas_native_image_asset_load_from_url(asset, url.c_str());
                                                 
                                                 canvas_native_image_asset_release(asset);
                                                 
                                                 write(jsi_callback->fd_[1],
                                                       &done,
                                                       sizeof(bool));
                                                 
                                             }, std::move(url));
    
    thread.detach();
    
#endif
    
    
#ifdef __APPLE__
    
    auto current_queue = new NSOperationQueueWrapper(true);
    
    auto queue = new NSOperationQueueWrapper(false);
    
    
    
    /*    std::thread task([jsi_callback, current_queue, asset](std::string url) {
     
     
     auto done = canvas_native_image_asset_load_from_url(asset, url.c_str());
     
     
     auto main_task = [jsi_callback, current_queue, url, done]() {
     
     
     v8::Isolate *isolate = jsi_callback->isolate_;
     v8::Locker locker(isolate);
     v8::Isolate::Scope isolate_scope(isolate);
     v8::HandleScope handle_scope(isolate);
     v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
     v8::Local<v8::Context> context = callback->GetCreationContextChecked();
     v8::Context::Scope context_scope(context);
     
     v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
     
     // v8::TryCatch tc(isolate);
     
     callback->Call(context, context->Global(), 1,
     args);  // ignore JS return value
     
     
     delete jsi_callback;
     // delete queue;
     delete current_queue;
     
     
     };
     
     current_queue->addOperation(main_task);
     
     
     }, std::move(url));
     
     task.detach();
     
     */
    
    
    
    
    
    auto task = [jsi_callback, current_queue, queue, asset, url]() {
        
        auto done = canvas_native_image_asset_load_from_url(asset, url.c_str());
        canvas_native_image_asset_release(asset);
        
        auto main_task = [jsi_callback, current_queue, queue, url, done]() {
            
            
            v8::Isolate *isolate = jsi_callback->isolate_;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);
            
            v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
            
            // v8::TryCatch tc(isolate);
            
            callback->Call(context, context->Global(), 1,
                           args);  // ignore JS return value
            
            
            delete jsi_callback;
            delete queue;
            delete current_queue;
            
            
        };
        
        current_queue->addOperation(main_task);
        
    };
    
    
    
    queue->addOperation(task);
#endif
    
}

void ImageAssetImpl::FromFileSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }
    auto isolate = args.GetIsolate();
    auto path = ConvertFromV8String(isolate, args[0]);
    
    auto done = canvas_native_image_asset_load_from_path(ptr->GetImageAsset(), path.c_str());
    
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
    
    auto asset = canvas_native_image_asset_reference(ptr->GetImageAsset());
    
    auto callback = args[1].As<v8::Function>();
    
    auto jsi_callback = new JSICallback(isolate, callback);
    
#ifdef __ANDROID__
    
    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  0,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
        auto cb = static_cast<JSICallback *>(data);
        bool done;
        read(fd, &done,
             sizeof(bool));
        
        v8::Isolate *isolate = cb->isolate_;
        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);
        v8::Local<v8::Function> callback = cb->callback_->Get(isolate);
        v8::Local<v8::Context> context = callback->GetCreationContextChecked();
        v8::Context::Scope context_scope(context);
        
        v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
        
        v8::TryCatch tc(isolate);
        
        callback->Call(context, context->Global(), 1,
                       args);  // ignore JS return value
        
        delete static_cast<JSICallback *>(data);
        return 0;
    }, jsi_callback);
    
    
    std::thread thread(
                       [jsi_callback, asset](
                                             const std::string path) {
                                                 
                                                 auto done = canvas_native_image_asset_load_from_path(asset, path.c_str());
                                                 
                                                 write(jsi_callback->fd_[1],
                                                       &done,
                                                       sizeof(bool));
                                                 
                                                 canvas_native_image_asset_release(asset);
                                                 
                                             }, std::move(path));
    
    thread.detach();
    
#endif
    
    
#ifdef __APPLE__
    
    auto current_queue = new NSOperationQueueWrapper(true);
    
    auto queue = new NSOperationQueueWrapper(false);
    
    auto task = [jsi_callback, current_queue, queue, asset, path]() {
        
        auto done = canvas_native_image_asset_load_from_path(asset, path.c_str());
        
        canvas_native_image_asset_release(asset);
        
        auto main_task = [jsi_callback, current_queue, queue, done]() {
            
            v8::Isolate *isolate = jsi_callback->isolate_;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);
            
            v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
            
            // v8::TryCatch tc(isolate);
            
            callback->Call(context, context->Global(), 1,
                           args);  // ignore JS return value
            
            delete queue;
            delete current_queue;
            delete jsi_callback;
            
        };
        
        current_queue->addOperation(main_task);
        
    };
    
    queue->addOperation(task);
    
    
#endif
}

void ImageAssetImpl::FromBytesSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    ImageAssetImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }
    auto context = args.GetIsolate()->GetCurrentContext();
    auto value = args[2];
    
    if (value->IsObject()) {
        if (!value->IsArrayBuffer()) {
            args.GetReturnValue().Set(false);
            return;
        }
        auto buf = value.As<v8::ArrayBuffer>();
        
        auto size = (uintptr_t) buf->ByteLength();
        auto data = (uint8_t *) buf->GetBackingStore()->Data();
        
        uint32_t width;
        uint32_t height;
        bool done = false;
        if(args[0]->Uint32Value(context).To(&width)
           && args[1]->Uint32Value(context).To(&height)
           ) {
            done = canvas_native_image_asset_load_from_raw(ptr->GetImageAsset(), width, height, data, size);
        }
        
        args.GetReturnValue().Set(done);
        return;
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
    
    
    auto bytes = args[2].As<v8::ArrayBuffer>();
    
    auto size = bytes->ByteLength();
    
    auto data = (uint8_t *) bytes->GetBackingStore()->Data();
    
    auto asset = canvas_native_image_asset_reference(ptr->GetImageAsset());
    
    auto context = args.GetIsolate()->GetCurrentContext();
    
    uint32_t width;
    uint32_t height;
    args[0]->Uint32Value(context).To(&width);
    args[1]->Uint32Value(context).To(&height);
    
    auto callback = args[3].As<v8::Function>();
    
    auto jsi_callback = new JSICallback(isolate, callback);
    
#ifdef __ANDROID__
    
    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  0,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
        auto cb = static_cast<JSICallback *>(data);
        bool done;
        read(fd, &done,
             sizeof(bool));
        
        v8::Isolate *isolate = cb->isolate_;
        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);
        v8::Local<v8::Function> callback = cb->callback_->Get(isolate);
        v8::Local<v8::Context> context = callback->GetCreationContextChecked();
        v8::Context::Scope context_scope(context);
        
        v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
        
        // v8::TryCatch tc(isolate);
        
        callback->Call(context, context->Global(), 1,
                       args);  // ignore JS return value
        
        delete static_cast<JSICallback *>(data);
        return 0;
    }, jsi_callback);
    
    std::thread thread(
                       [jsi_callback, asset, width, height, data, size]() {
                           
                           auto done = canvas_native_image_asset_load_from_raw(asset, width, height, data, size);
                           
                           canvas_native_image_asset_release(asset);
                           
                           write(jsi_callback->fd_[1],
                                 &done,
                                 sizeof(bool));
                           
                       });
    
    thread.detach();
    
#endif
    
    
#ifdef __APPLE__
    
    auto current_queue = new NSOperationQueueWrapper(true);
    
    auto queue = new NSOperationQueueWrapper(false);
    
    auto task = [jsi_callback, current_queue, queue, asset, width, height,data, size]() {
        
        auto done = canvas_native_image_asset_load_from_raw(asset, width, height, data, size);
        
        canvas_native_image_asset_release(asset);
        
        auto main_task = [jsi_callback, current_queue, queue, done]() {
            
            
            v8::Isolate *isolate = jsi_callback->isolate_;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);
            
            v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
            
            // v8::TryCatch tc(isolate);
            
            callback->Call(context, context->Global(), 1,
                           args);  // ignore JS return value
            
            
            delete jsi_callback;
            delete queue;
            delete current_queue;
            
            
        };
        
        current_queue->addOperation(main_task);
        
    };
    
    queue->addOperation(task);
#endif
    
}


void ImageAssetImpl::FromEncodedBytesSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        
        auto size = (uintptr_t) buf->ByteLength();
        auto data = (uint8_t *) buf->GetBackingStore()->Data();
        
        auto done = canvas_native_image_asset_load_from_raw_encoded(ptr->GetImageAsset(), data,
                                                                    size);
        
        args.GetReturnValue().Set(done);
        return;
    }
    
    args.GetReturnValue().Set(false);
}

void ImageAssetImpl::FromEncodedBytesCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
    
    auto asset = canvas_native_image_asset_reference(ptr->GetImageAsset());
    
    auto callback = args[1].As<v8::Function>();
    
    auto jsi_callback = new JSICallback(isolate, callback);
    
#ifdef __ANDROID__
    
    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  0,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {
        auto cb = static_cast<JSICallback *>(data);
        bool done;
        read(fd, &done,
             sizeof(bool));
        
        v8::Isolate *isolate = cb->isolate_;
        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);
        v8::Local<v8::Function> callback = cb->callback_->Get(isolate);
        v8::Local<v8::Context> context = callback->GetCreationContextChecked();
        v8::Context::Scope context_scope(context);
        
        v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
        
        // v8::TryCatch tc(isolate);
        
        callback->Call(context, context->Global(), 1,
                       args);  // ignore JS return value
        
        delete static_cast<JSICallback *>(data);
        return 0;
    }, jsi_callback);
    
    std::thread thread(
                       [jsi_callback, asset, data, size]() {
                           
                           auto done = canvas_native_image_asset_load_from_raw_encoded(asset, data, size);
                           
                           canvas_native_image_asset_release(asset);
                           
                           write(jsi_callback->fd_[1],
                                 &done,
                                 sizeof(bool));
                           
                       });
    
    thread.detach();
    
#endif
    
    
#ifdef __APPLE__
    
    auto current_queue = new NSOperationQueueWrapper(true);
    
    auto queue = new NSOperationQueueWrapper(false);
    
    auto task = [jsi_callback, current_queue, queue, asset, data, size]() {
        
        auto done = canvas_native_image_asset_load_from_raw_encoded(asset, data, size);
        
        canvas_native_image_asset_release(asset);
        
        auto main_task = [jsi_callback, current_queue, queue, done]() {
            
            
            v8::Isolate *isolate = jsi_callback->isolate_;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);
            
            v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
            
            // v8::TryCatch tc(isolate);
            
            callback->Call(context, context->Global(), 1,
                           args);  // ignore JS return value
            
            
            delete jsi_callback;
            delete queue;
            delete current_queue;
            
            
        };
        
        current_queue->addOperation(main_task);
        
    };
    
    queue->addOperation(task);
#endif
    
}

/*
 void ImageAssetImpl::SaveSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
 ImageAssetImpl *ptr = GetPointer(args.This());
 if (ptr == nullptr) {
 args.GetReturnValue().Set(false);
 return;
 }
 
 auto isolate = args.GetIsolate();
 auto context = isolate->GetCurrentContext();
 auto path = ConvertFromV8String(isolate, args[0]);
 auto format = args[1]->Uint32Value(context).ToChecked();
 auto done = canvas_native_image_asset_save_path(ptr->GetImageAsset(), path.c_str(), format);
 
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
 
 auto format = args[1]->Uint32Value(context).ToChecked();
 
 auto asset = canvas_native_image_asset_shared_clone(
 ptr->GetImageAsset());
 
 auto callback = args[1].As<v8::Function>();
 
 auto jsi_callback = new JSICallback(isolate, callback);
 
 
 #ifdef __ANDROID__
 
 ALooper_addFd(jsi_callback->looper_,
 jsi_callback->fd_[0],
 0,
 ALOOPER_EVENT_INPUT,
 [](int fd, int events,
 void *data) {
 auto cb = static_cast<JSICallback *>(data);
 bool done;
 read(fd, &done,
 sizeof(bool));
 
 v8::Isolate *isolate = cb->isolate_;
 v8::Locker locker(isolate);
 v8::Isolate::Scope isolate_scope(isolate);
 v8::HandleScope handle_scope(isolate);
 v8::Local<v8::Function> callback = cb->callback_->Get(isolate);
 v8::Local<v8::Context> context = callback->GetCreationContextChecked();
 v8::Context::Scope context_scope(context);
 
 v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
 
 // v8::TryCatch tc(isolate);
 
 callback->Call(context, context->Global(), 1,
 args);  // ignore JS return value
 
 delete static_cast<JSICallback *>(data);
 return 0;
 }, jsi_callback);
 
 std::thread thread(
 [jsi_callback, asset, format](
 const std::string &path) {
 
 auto done = canvas_native_image_asset_save_path(asset,
 path.c_str(),
 format);
 
 canvas_native_image_asset_destroy(asset);
 
 write(jsi_callback->fd_[1],
 &done,
 sizeof(bool));
 
 }, std::move(path));
 
 
 thread.detach();
 
 #endif
 
 
 #ifdef __APPLE__
 
 auto current_queue = new NSOperationQueueWrapper(true);
 
 auto queue = new NSOperationQueueWrapper(false);
 
 auto task = [jsi_callback, current_queue, queue, asset, path, format]() {
 
 auto done = canvas_native_image_asset_save_path(asset, path.c_str(), format);
 
 canvas_native_image_asset_destroy(asset);
 
 auto main_task = [jsi_callback, current_queue, queue, done]() {
 
 
 v8::Isolate *isolate = jsi_callback->isolate_;
 v8::Locker locker(isolate);
 v8::Isolate::Scope isolate_scope(isolate);
 v8::HandleScope handle_scope(isolate);
 v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
 v8::Local<v8::Context> context = callback->GetCreationContextChecked();
 v8::Context::Scope context_scope(context);
 
 v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
 
 // v8::TryCatch tc(isolate);
 
 callback->Call(context, context->Global(), 1,
 args);  // ignore JS return value
 
 
 delete jsi_callback;
 delete queue;
 delete current_queue;
 
 
 };
 current_queue->addOperation(main_task);
 };
 
 queue->addOperation(task);
 #endif
 
 
 }
 */

const ImageAsset *ImageAssetImpl::GetImageAsset() {
    return this->asset_;
}

