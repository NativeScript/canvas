//
// Created by Osei Fortune on 25/02/2023.
//

#include "CanvasJSIModule.h"
#include "JSICallback.h"
#include "JSIRuntime.h"
#include "Helpers.h"

void CanvasJSIModule::install(v8::Isolate *isolate) {


    auto context = isolate->GetCurrentContext();
    auto global = context->Global();

    if (!global->
            HasOwnProperty(context, ConvertToV8String(isolate, "CanvasJSIModule")).FromMaybe(
            false)) {


        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);

        auto v8Global = context->Global();

        auto canvasMod = v8::Object::New(isolate);
        TextDecoderImpl::Init(canvasMod, isolate);
        TextEncoderImpl::Init(canvasMod, isolate);
        Path2D::Init(canvasMod, isolate);
        ImageDataImpl::Init(canvasMod, isolate);
        ImageAssetImpl::Init(canvasMod, isolate);
        CanvasGradient::Init(canvasMod, isolate);
        CanvasPattern::Init(canvasMod, isolate);
        MatrixImpl::Init(canvasMod, isolate);
        TextMetricsImpl::Init(canvasMod, isolate);
        v8Global->Set(context, ConvertToV8String(isolate, "CanvasModule"), canvasMod);
        canvasMod->Set(context, ConvertToV8String(isolate, "create2DContext"),
                       v8::FunctionTemplate::New(isolate, &Create2DContext)->GetFunction(
                               context).ToLocalChecked());
        canvasMod->Set(context, ConvertToV8String(isolate, "createImageBitmap"),
                       v8::FunctionTemplate::New(isolate, &CreateImageBitmap)->GetFunction(
                               context).ToLocalChecked());
        canvasMod->Set(context, ConvertToV8String(isolate, "create2DContextWithPointer"),
                       v8::FunctionTemplate::New(isolate, &Create2DContextWithPointer)->GetFunction(
                               context).ToLocalChecked());
        canvasMod->Set(context, ConvertToV8String(isolate, "readFile"),
                       v8::FunctionTemplate::New(isolate, &ReadFile)->GetFunction(
                               context).ToLocalChecked());

        canvasMod->Set(context, ConvertToV8String(isolate, "createWebGLContext"),
                       v8::FunctionTemplate::New(isolate, &CreateWebGLContext)->GetFunction(
                               context).ToLocalChecked());

        canvasMod->Set(context, ConvertToV8String(isolate, "createWebGL2Context"),
                       v8::FunctionTemplate::New(isolate, &CreateWebGL2Context)->GetFunction(
                               context).ToLocalChecked());

        global->Set(context,
                    ConvertToV8String(isolate, "CanvasModule"), canvasMod);

    }
}

void CanvasJSIModule::Create2DContext(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = args[0].As<v8::BigInt>()->Int64Value();
    auto width = (float) args[1]->NumberValue(context).ToChecked();
    auto height = (float) args[2]->NumberValue(context).ToChecked();
    auto density = (float) args[3]->NumberValue(context).ToChecked();
    auto samples = (int) args[4]->NumberValue(context).ToChecked();
    auto alpha = (bool) args[5]->BooleanValue(isolate);
    auto font_color = (int) args[6]->NumberValue(context).ToChecked();
    auto ppi = (float) args[7]->NumberValue(context).ToChecked();
    auto direction = (int) args[8]->NumberValue(context).ToChecked();

    auto context_2d = canvas_native_context_create_gl(width, height, density,
                                                      ptr,
                                                      samples, alpha,
                                                      font_color, ppi, direction);

    auto ret = CanvasRenderingContext2DImpl::NewInstance(isolate, new CanvasRenderingContext2DImpl(
            std::move(context_2d)));

    args.GetReturnValue().Set(ret);
}

void CanvasJSIModule::CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto count = args.Length();
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto image = args[0];
    auto sx_or_options = args[1];
    auto sy = args[2];
    auto sw = args[3];
    auto sh = args[4];

    auto len = count - 1;
    auto cb = args[count - 1];

    if (len == 1 && !image->IsObject() || image->IsFunction()) {
        isolate->ThrowError("Illegal constructor");
        return;
    }

    Options options;

    if (len == 0) {
        isolate->ThrowError("Illegal constructor");
        return;
    }

    if (!cb->IsObject() && !cb->IsFunction()) {
        isolate->ThrowError("Illegal constructor");
        return;
    }


    if (image->IsNullOrUndefined()) {
        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate, "Failed to load image"),
                                       v8::Undefined(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }

    if (len >= 4 && (sw->IsNumber() && sw->IsNumber() == 0)) {

        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate,
                                                         "Failed to execute 'createImageBitmap' : The crop rect width is 0"),
                                       v8::Undefined(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);

        return;
    }
    if (len >= 5 && (sh->IsNumber() && sh->IsNumber() == 0)) {
        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate,
                                                         "Failed to execute 'createImageBitmap' : The crop rect height is 0"),
                                       v8::Undefined(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }


    if (image->IsObject()) {
        auto imageObject = image.As<v8::Object>();
        auto isArrayBuffer = imageObject->IsArrayBuffer();
        auto isTypedArray = imageObject->IsTypedArray();
        if (isArrayBuffer || isTypedArray) {

            if (len == 1 || len == 2) {
                if (len == 2) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[1]);
                }

                auto asset = canvas_native_image_asset_create();

                auto shared_asset = canvas_native_image_asset_shared_clone(*asset);


                auto ret = new ImageBitmapImpl(
                        std::move(asset));

                v8::Global<v8::Function> cbFunc(isolate, args[count - 1].As<v8::Function>());
                v8::Global<v8::Value> data(isolate, v8::External::New(isolate, ret));
                auto jsi_callback = new JSICallback(isolate, std::move(cbFunc), std::move(data));

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
                                  v8::Isolate::Scope isolate_scope(isolate);
                                  v8::HandleScope handle_scope(isolate);
                                  v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                                  v8::Local<v8::External> cbData = cb->data_.Get(
                                          isolate).As<v8::External>();
                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                  v8::Context::Scope context_scope(context);

                                  auto ret = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                                          context).ToLocalChecked()->NewInstance(
                                          context).ToLocalChecked();

                                  SetNativeType(isolate, ret, NativeType::ImageBitmap);

                                  ret->SetInternalField(0, cbData);

                                  v8::Local<v8::Value> args[2];

                                  if (done) {
                                      args[0] = v8::Null(isolate);
                                      args[1] = ret;

                                  } else {
                                      args[0] = v8::Exception::Error(
                                              ConvertToV8String(isolate, "Failed to load image"));
                                      args[1] = v8::Null(isolate);
                                  }


                                  callback->Call(context, context->Global(), 2, args);

                                  delete static_cast<JSICallback *>(data);
                                  return 0;
                              }, jsi_callback);

                ALooper_wake(jsi_callback->looper_);
                v8::Global<v8::Value> ab(isolate, imageObject);

                if (isArrayBuffer) {
                    auto arrayBuffer = imageObject.As<v8::ArrayBuffer>();
                    auto dataBuffer = (uint8_t *) arrayBuffer->GetBackingStore()->Data();

                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options](
                                    rust::Box<ImageAsset> asset,
                                    v8::Global<v8::Value> ab, size_t size) {


                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                        rust::Slice<const uint8_t>(dataBuffer, size),
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight,
                                        *asset);

                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));


                            }, std::move(shared_asset), std::move(ab), arrayBuffer->ByteLength());

                    thread.detach();
                    return;
                }


                auto ta = imageObject.As<v8::TypedArray>();
                auto typedArray = GetTypedArrayData<const uint8_t>(ta);

                std::thread thread(
                        [jsi_callback, &options](
                                rust::Box<ImageAsset> asset, v8::Global<v8::Value> ab,
                                rust::Slice<const uint8_t> typedArray) {

                            auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                    typedArray,
                                    options.flipY,
                                    options.premultiplyAlpha,
                                    options.colorSpaceConversion,
                                    options.resizeQuality,
                                    options.resizeWidth,
                                    options.resizeHeight,
                                    *asset);

                            write(jsi_callback->fd_[1],
                                  &done,
                                  sizeof(bool));


                        }, std::move(shared_asset), std::move(ab), typedArray);
                thread.detach();

                return;
            } else if (len == 5 || len == 6) {
                auto asset = canvas_native_image_asset_create();

                auto shared_asset = canvas_native_image_asset_shared_clone(*asset);

                auto ret = new ImageBitmapImpl(
                        std::move(asset));

                auto cbFunc = args[count - 1].As<v8::Function>();
                v8::Global<v8::Function> globalFunc(isolate, cbFunc);
                v8::Global<v8::Value> data(isolate, v8::External::New(isolate, ret));
                if (len == 6) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[5]);
                }

                auto jsi_callback = new JSICallback(isolate, std::move(globalFunc),
                                                    std::move(data));


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
                                  v8::Isolate::Scope isolate_scope(isolate);
                                  v8::HandleScope handle_scope(isolate);
                                  v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                                  v8::Local<v8::External> cbData = cb->data_.Get(
                                          isolate).As<v8::External>();
                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                  v8::Context::Scope context_scope(context);

                                  auto ret = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                                          context).ToLocalChecked()->NewInstance(
                                          context).ToLocalChecked();

                                  SetNativeType(isolate, ret, NativeType::ImageBitmap);

                                  ret->SetInternalField(0, cbData);

                                  v8::Local<v8::Value> args[2];

                                  if (done) {
                                      args[0] = v8::Null(isolate);
                                      args[1] = ret;


                                  } else {
                                      args[0] = v8::Exception::Error(
                                              ConvertToV8String(isolate, "Failed to load image"));
                                      args[1] = v8::Null(isolate);
                                  }

                                  callback->Call(context, context->Global(), 2, args);

                                  delete static_cast<JSICallback *>(data);
                                  return 0;
                              }, jsi_callback);

                ALooper_wake(jsi_callback->looper_);

                auto bufferValue = args[0];
                v8::Global<v8::Value> ab(isolate, bufferValue);


                if (isArrayBuffer) {
                    auto arrayBuffer = bufferValue.As<v8::ArrayBuffer>();
                    auto dataBuffer = (uint8_t *) arrayBuffer->GetBackingStore()->Data();
                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options](
                                    rust::Box<ImageAsset> asset,
                                    float sx_or_options,
                                    float sy,
                                    float sw,
                                    float sh,
                                    v8::Global<v8::Value> ab,
                                    size_t size
                            ) {

                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                        rust::Slice<const uint8_t>(dataBuffer, size),
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


                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));

                            }, std::move(shared_asset),
                            (float) sx_or_options->NumberValue(context).ToChecked(),
                            (float) sy->NumberValue(context).ToChecked(),
                            (float) sw->NumberValue(context).ToChecked(),
                            (float) sh->NumberValue(context).ToChecked(), std::move(ab),
                            arrayBuffer->ByteLength());

                    thread.detach();

                    return;
                }

                auto ta = bufferValue.As<v8::TypedArray>();
                auto typedArray = GetTypedArrayData<const uint8_t>(ta);
                std::thread thread(
                        [jsi_callback, &options](
                                rust::Box<ImageAsset> asset,
                                float sx_or_options,
                                float sy,
                                float sw,
                                float sh,
                                v8::Global<v8::Value> ab,
                                rust::Slice<const uint8_t> typedArray
                        ) {

                            auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                    typedArray,
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


                            write(jsi_callback->fd_[1],
                                  &done,
                                  sizeof(bool));

                        }, std::move(shared_asset),
                        (float) sx_or_options->NumberValue(context).ToChecked(),
                        (float) sy->NumberValue(context).ToChecked(),
                        (float) sw->NumberValue(context).ToChecked(),
                        (float) sh->NumberValue(context).ToChecked(), std::move(ab), typedArray);
                thread.detach();


                return;
            }
        }
    }


    auto typeValue = GetPrivateValue(isolate, image.As<v8::Object>(),
                                     ConvertToV8String(isolate, "__type"));

    if (len == 1 || len == 2) {
        if (len == 2) {
            options = ImageBitmapImpl::HandleOptions(isolate, args[1]);
        }

        v8::Local<v8::Value> retArgs[2];
        retArgs[0] = v8::Null(isolate);


        if (!typeValue.IsEmpty()) {
            auto type = (NativeType) typeValue->Int32Value(context).ToChecked();
            switch (type) {
                case NativeType::ImageAsset: {
                    auto image_asset = ImageAssetImpl::GetPointer(image.As<v8::Object>());
                    auto ret = canvas_native_image_bitmap_create_from_asset(
                            image_asset->GetImageAsset(),
                            options.flipY,
                            options.premultiplyAlpha,
                            options.colorSpaceConversion,
                            options.resizeQuality,
                            options.resizeWidth,
                            options.resizeHeight);


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

                    retArgs[1] = object;
                }
                    break;
                case NativeType::ImageBitmap: {
                    auto image_bitmap = ImageBitmapImpl::GetPointer(image.As<v8::Object>());
                    auto ret = canvas_native_image_bitmap_create_from_asset(
                            image_bitmap->GetImageAsset(),
                            options.flipY,
                            options.premultiplyAlpha,
                            options.colorSpaceConversion,
                            options.resizeQuality,
                            options.resizeWidth,
                            options.resizeHeight);


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

                    retArgs[1] = object;

                }
                    break;
                default:
                    break;
            }
        }


        cb.As<v8::Function>()->Call(context, context->Global(), 2, retArgs);

        return;
    } else if (len == 5 || len == 6) {

        if (len == 6) {
            options = ImageBitmapImpl::HandleOptions(isolate, args[5]);
        }

        v8::Local<v8::Value> retArgs[2];
        retArgs[0] = v8::Null(isolate);


        if (!typeValue.IsEmpty()) {
            auto type = (NativeType) typeValue->Int32Value(context).ToChecked();
            switch (type) {
                case NativeType::ImageAsset: {
                    auto image_asset = ImageBitmapImpl::GetPointer(image.As<v8::Object>());
                    auto ret = canvas_native_image_bitmap_create_from_asset_src_rect(
                            image_asset->GetImageAsset(),
                            (float) sx_or_options->NumberValue(context).ToChecked(),
                            (float) sy->NumberValue(context).ToChecked(),
                            (float) sw->NumberValue(context).ToChecked(),
                            (float) sh->NumberValue(context).ToChecked(),
                            options.flipY,
                            options.premultiplyAlpha,
                            options.colorSpaceConversion,
                            options.resizeQuality,
                            options.resizeWidth,
                            options.resizeHeight);


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

                    retArgs[1] = object;
                }
                    break;
                case NativeType::ImageBitmap: {
                    auto image_bitmap = ImageBitmapImpl::GetPointer(image.As<v8::Object>());
                    auto ret = canvas_native_image_bitmap_create_from_asset_src_rect(
                            image_bitmap->GetImageAsset(),
                            (float) sx_or_options->NumberValue(context).ToChecked(),
                            (float) sy->NumberValue(context).ToChecked(),
                            (float) sw->NumberValue(context).ToChecked(),
                            (float) sh->NumberValue(context).ToChecked(),
                            options.flipY,
                            options.premultiplyAlpha,
                            options.colorSpaceConversion,
                            options.resizeQuality,
                            options.resizeWidth,
                            options.resizeHeight);


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

                    retArgs[1] = object;

                }
                    break;
                default:
                    break;
            }
        }


        cb.As<v8::Function>()->Call(context, context->Global(), 2, retArgs);

        return;
    }


    args.GetReturnValue().SetUndefined();

}

void CanvasJSIModule::Create2DContextWithPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = args[0]->ToBigInt(context).ToLocalChecked()->Int64Value();

    auto context_2d = canvas_native_context_create_with_pointer(ptr);

    auto ret = CanvasRenderingContext2DImpl::NewInstance(isolate, new CanvasRenderingContext2DImpl(
            std::move(context_2d)));
    args.GetReturnValue().Set(ret);
}

void CanvasJSIModule::ReadFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto file = ConvertFromV8String(isolate, args[0]);
    v8::Global<v8::Function> cbFunc(isolate, args[1].As<v8::Function>());
    auto jsi_callback = new JSICallback(isolate, std::move(cbFunc));


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

                      while (cb->data_.IsEmpty()) {
                          return 1;
                      }

                      {
                          v8::Isolate *isolate = cb->isolate_;
                          v8::Locker locker(isolate);
                          v8::Isolate::Scope isolate_scope(isolate);
                          v8::HandleScope handle_scope(isolate);
                          v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                          auto cbData = cb->data_.Get(
                                  isolate);
                          v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                          v8::Context::Scope context_scope(context);


                          v8::Local<v8::Value> args[2];

                          if (done) {
                              args[0] = v8::Null(isolate);
                              args[1] = v8::ArrayBuffer::New(isolate,
                                                             cbData.As<v8::ArrayBuffer>()->GetBackingStore());
                          } else {
                              auto error = cbData->ToString(context).ToLocalChecked();
                              args[0] = v8::Exception::Error(error);
                              args[1] = v8::Null(isolate);
                          }

                          callback->Call(context, context->Global(), 2, args);
                      }


//                      auto func = cb->value_->asObject(
//                              rt).asFunction(
//                              rt);
//
//                      while (cb->data_ == nullptr) {
//                          return 1;
//                      }
//
//                      if (done) {
//                          auto buf = cb->data_->asObject(rt).getArrayBuffer(rt);
//                          func.call(rt, {jsi::Value::null(), std::move(buf)});
//                      } else {
//                          auto error = cb->data_->asString(rt);
//                          func.call(rt, {std::move(error), jsi::Value::null()});
//                      }

                      delete static_cast<JSICallback *>(data);
                      return 0;
                  }, jsi_callback);

    ALooper_wake(jsi_callback->looper_);


    std::thread thread(
            [jsi_callback](const std::string &file) {


                auto isolate = jsi_callback->isolate_;
                bool done = false;
                auto ret = canvas_native_helper_read_file(
                        rust::Str(file.c_str()));

                if (!canvas_native_helper_read_file_has_error(*ret)) {
                    auto buf = canvas_native_helper_read_file_get_data(
                            std::move(ret));

                    auto vec_buffer = new VecMutableBuffer<uint8_t>(
                            std::move(buf));

                    // testing if isolate needs to be current
                    auto store = v8::ArrayBuffer::NewBackingStore(vec_buffer->data(),
                                                                  vec_buffer->size(),
                                                                  [](void *data, size_t length,
                                                                     void *deleter_data) {
                                                                      if (deleter_data != nullptr) {
                                                                          delete (VecMutableBuffer<uint8_t> *) deleter_data;
                                                                      }
                                                                  },
                                                                  vec_buffer);

                    auto arraybuffer = v8::ArrayBuffer::New(isolate, std::move(store));

                    v8::Global<v8::Value> data(isolate, arraybuffer);

                    jsi_callback->data_ = std::move(data);
                    done = true;
                } else {
                    auto error = canvas_native_helper_read_file_get_error(
                            *ret);

                    v8::Global<v8::Value> data(isolate,
                                               ConvertToV8OneByteString(isolate, std::move(error)));
                    jsi_callback->data_ = std::move(data);
                }


                write(jsi_callback->fd_[1],
                      &done,
                      sizeof(bool));


            }, std::move(file));

    thread.detach();
}

void CanvasJSIModule::CreateWebGLContext(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto configValue = args[0];
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (!configValue->IsNullOrUndefined() && configValue->IsObject()) {
        auto config = configValue.As<v8::Object>();
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

        v8::Local<v8::Value> versionValue;

        config->Get(context, ConvertToV8String(isolate, "version")).ToLocal(&versionValue);

        if (!versionValue.IsEmpty() && versionValue->IsString()) {
            version = ConvertFromV8String(isolate, versionValue);
        }

        v8::Local<v8::Value> alphaValue;

        config->Get(context, ConvertToV8String(isolate, "alpha")).ToLocal(&alphaValue);
        if (!alphaValue.IsEmpty() && alphaValue->IsBoolean()) {
            alpha = alphaValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> antialiasValue;
        config->Get(context, ConvertToV8String(isolate, "antialias")).ToLocal(&antialiasValue);
        if (!antialiasValue.IsEmpty() && antialiasValue->IsBoolean()) {
            antialias = antialiasValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> failIfMajorPerformanceCaveatValue;
        config->Get(context,
                    ConvertToV8String(isolate, "failIfMajorPerformanceCaveat")).ToLocal(
                &failIfMajorPerformanceCaveatValue);
        if (!failIfMajorPerformanceCaveatValue.IsEmpty() &&
            failIfMajorPerformanceCaveatValue->IsBoolean()) {
            fail_if_major_performance_caveat = failIfMajorPerformanceCaveatValue->BooleanValue(
                    isolate);
        }

        v8::Local<v8::Value> powerPreferenceValue;
        config->Get(context, ConvertToV8String(isolate, "powerPreference")).ToLocal(
                &powerPreferenceValue);
        if (!powerPreferenceValue.IsEmpty() && powerPreferenceValue->IsString()) {
            power_preference = ConvertFromV8String(isolate, powerPreferenceValue);
        }

        v8::Local<v8::Value> premultipliedAlphaValue;
        config->Get(context,
                    ConvertToV8String(isolate, "premultipliedAlpha")).ToLocal(
                &premultipliedAlphaValue);
        if (!premultipliedAlphaValue.IsEmpty() && premultipliedAlphaValue->IsBoolean()) {
            premultiplied_alpha = premultipliedAlphaValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> preserveDrawingBufferValue;
        config->Get(context,
                    ConvertToV8String(isolate, "preserveDrawingBuffer")).ToLocal(
                &preserveDrawingBufferValue);
        if (!preserveDrawingBufferValue.IsEmpty() && preserveDrawingBufferValue->IsBoolean()) {
            preserve_drawing_buffer = preserveDrawingBufferValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> stencilValue;
        config->Get(context, ConvertToV8String(isolate, "stencil")).ToLocal(&stencilValue);
        if (!stencilValue.IsEmpty() && stencilValue->IsBoolean()) {
            stencil = stencilValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> desynchronizedValue;
        config->Get(context, ConvertToV8String(isolate, "desynchronized")).ToLocal(
                &desynchronizedValue);
        if (!desynchronizedValue.IsEmpty() && desynchronizedValue->IsBoolean()) {
            desynchronized = desynchronizedValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> xrCompatibleValue;
        config->Get(context,
                    ConvertToV8String(isolate, "xrCompatible")).ToLocal(&xrCompatibleValue);
        if (!xrCompatibleValue.IsEmpty() && xrCompatibleValue->IsBoolean()) {
            xr_compatible = xrCompatibleValue->BooleanValue(isolate);
        }

        if (version !=
            "v1") {
            args.GetReturnValue().SetNull();
            return;
        } else {
            auto count = args.Length();
            if (count == 6) {
                auto ctx = args[1].As<v8::BigInt>()->Int64Value();
//                auto density = args[2]->NumberValue(context).ToChecked();
//                auto fontColor = args[3]->NumberValue(context).ToChecked();
//                auto ppi = args[4]->NumberValue(context).ToChecked();
//                auto direction = args[5]->NumberValue(context).ToChecked();
                auto webgl = canvas_native_webgl_create(
                        ctx,
                        rust::Str(
                                version.c_str()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(
                                power_preference.c_str()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible
                );

                auto renderingContext = WebGLRenderingContext::NewInstance(isolate,
                                                                           new WebGLRenderingContext(
                                                                                   std::move(
                                                                                           webgl)));

                args.GetReturnValue().Set(renderingContext);
                return;
            } else if (count == 7) {
                auto width = args[1]->NumberValue(context).ToChecked();
                auto height = args[2]->NumberValue(context).ToChecked();
//                auto density = args[3]->NumberValue(context).ToChecked();
//                auto fontColor = args[4]->NumberValue(context).ToChecked();
//                auto ppi = args[5]->NumberValue(context).ToChecked();
//                auto direction = args[6]->NumberValue(context).ToChecked();
                auto ctx = canvas_native_webgl_create_no_window(
                        (int32_t) width,
                        (int32_t) height,
                        rust::Str(
                                version.c_str()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(
                                power_preference.c_str()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible,
                        false
                );

                auto renderingContext = WebGLRenderingContext::NewInstance(isolate,
                                                                           new WebGLRenderingContext(
                                                                                   std::move(ctx)));

                args.GetReturnValue().Set(renderingContext);
                return;

            } else {
                auto width = (int32_t) args[1]->NumberValue(context).ToChecked();
                auto height = (int32_t) args[2]->NumberValue(context).ToChecked();

                auto ctx = canvas_native_webgl_create_no_window(
                        width,
                        height,
                        rust::Str(
                                version.c_str()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(
                                power_preference.c_str()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible,
                        false
                );

                auto renderingContext = WebGLRenderingContext::NewInstance(isolate,
                                                                           new WebGLRenderingContext(
                                                                                   std::move(
                                                                                           ctx)));

                args.GetReturnValue().Set(renderingContext);
                return;
            }

        }
    }
    args.GetReturnValue().SetNull();
}

void CanvasJSIModule::CreateWebGL2Context(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto configValue = args[0];
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (!configValue->IsNullOrUndefined() && configValue->IsObject()) {
        auto config = configValue.As<v8::Object>();
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

        v8::Local<v8::Value> versionValue;

        config->Get(context, ConvertToV8String(isolate, "version")).ToLocal(&versionValue);

        if (!versionValue.IsEmpty() && versionValue->IsString()) {
            version = ConvertFromV8String(isolate, versionValue);
        }

        v8::Local<v8::Value> alphaValue;

        config->Get(context, ConvertToV8String(isolate, "alpha")).ToLocal(&alphaValue);
        if (!alphaValue.IsEmpty() && alphaValue->IsBoolean()) {
            alpha = alphaValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> antialiasValue;
        config->Get(context, ConvertToV8String(isolate, "antialias")).ToLocal(&antialiasValue);
        if (!antialiasValue.IsEmpty() && antialiasValue->IsBoolean()) {
            antialias = antialiasValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> failIfMajorPerformanceCaveatValue;
        config->Get(context,
                    ConvertToV8String(isolate, "failIfMajorPerformanceCaveat")).ToLocal(
                &failIfMajorPerformanceCaveatValue);
        if (!failIfMajorPerformanceCaveatValue.IsEmpty() &&
            failIfMajorPerformanceCaveatValue->IsBoolean()) {
            fail_if_major_performance_caveat = failIfMajorPerformanceCaveatValue->BooleanValue(
                    isolate);
        }

        v8::Local<v8::Value> powerPreferenceValue;
        config->Get(context, ConvertToV8String(isolate, "powerPreference")).ToLocal(
                &powerPreferenceValue);
        if (!powerPreferenceValue.IsEmpty() && powerPreferenceValue->IsString()) {
            power_preference = ConvertFromV8String(isolate, powerPreferenceValue);
        }

        v8::Local<v8::Value> premultipliedAlphaValue;
        config->Get(context,
                    ConvertToV8String(isolate, "premultipliedAlpha")).ToLocal(
                &premultipliedAlphaValue);
        if (!premultipliedAlphaValue.IsEmpty() && premultipliedAlphaValue->IsBoolean()) {
            premultiplied_alpha = premultipliedAlphaValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> preserveDrawingBufferValue;
        config->Get(context,
                    ConvertToV8String(isolate, "preserveDrawingBuffer")).ToLocal(
                &preserveDrawingBufferValue);
        if (!preserveDrawingBufferValue.IsEmpty() && preserveDrawingBufferValue->IsBoolean()) {
            preserve_drawing_buffer = preserveDrawingBufferValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> stencilValue;
        config->Get(context, ConvertToV8String(isolate, "stencil")).ToLocal(&stencilValue);
        if (!stencilValue.IsEmpty() && stencilValue->IsBoolean()) {
            stencil = stencilValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> desynchronizedValue;
        config->Get(context, ConvertToV8String(isolate, "desynchronized")).ToLocal(
                &desynchronizedValue);
        if (!desynchronizedValue.IsEmpty() && desynchronizedValue->IsBoolean()) {
            desynchronized = desynchronizedValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> xrCompatibleValue;
        config->Get(context,
                    ConvertToV8String(isolate, "xrCompatible")).ToLocal(&xrCompatibleValue);
        if (!xrCompatibleValue.IsEmpty() && xrCompatibleValue->IsBoolean()) {
            xr_compatible = xrCompatibleValue->BooleanValue(isolate);
        }

        if (version !=
            "v2") {
            args.GetReturnValue().SetNull();
            return;
        } else {
            auto count = args.Length();
            if (count == 6) {
                auto ctx = args[1].As<v8::BigInt>()->Int64Value();
//                auto density = args[2]->NumberValue(context).ToChecked();
//                auto fontColor = args[3]->NumberValue(context).ToChecked();
//                auto ppi = args[4]->NumberValue(context).ToChecked();
//                auto direction = args[5]->NumberValue(context).ToChecked();

                auto webgl = canvas_native_webgl_create(
                        ctx,
                        rust::Str(
                                version.c_str()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(
                                power_preference.c_str()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible
                );

                auto renderingContext = WebGL2RenderingContext::NewInstance(isolate,
                                                                            new WebGL2RenderingContext(
                                                                                    std::move(
                                                                                            webgl),
                                                                                    WebGLRenderingVersion::V2));

                args.GetReturnValue().Set(renderingContext);
                return;

            } else if (count ==
                       7) {
                auto width = args[1]->NumberValue(context).ToChecked();
                auto height = args[2]->NumberValue(context).ToChecked();
//                auto density = args[3]->NumberValue(context).ToChecked();
//                auto fontColor = args[4]->NumberValue(context).ToChecked();
//                auto ppi = args[5]->NumberValue(context).ToChecked();
//                auto direction = args[6]->NumberValue(context).ToChecked();
                auto ctx = canvas_native_webgl_create_no_window(
                        (int32_t) width,
                        (int32_t) height,
                        rust::Str(
                                version.c_str()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(
                                power_preference.c_str()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible,
                        false
                );
                auto renderingContext = WebGL2RenderingContext::NewInstance(isolate,
                                                                            new WebGL2RenderingContext(
                                                                                    std::move(
                                                                                            ctx),
                                                                                    WebGLRenderingVersion::V2));

                args.GetReturnValue().Set(renderingContext);
                return;

            } else {
                auto width = (int32_t) args[1]->NumberValue(context).ToChecked();
                auto height = (int32_t) args[2]->NumberValue(context).ToChecked();
                auto ctx = canvas_native_webgl_create_no_window(
                        width,
                        height,
                        rust::Str(
                                version.c_str()),
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        rust::Str(
                                power_preference.c_str()),
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible,
                        false
                );

                auto renderingContext = WebGL2RenderingContext::NewInstance(isolate,
                                                                            new WebGL2RenderingContext(
                                                                                    std::move(
                                                                                            ctx),
                                                                                    WebGLRenderingVersion::V2));

                args.GetReturnValue().Set(renderingContext);
                return;
            }
        }
    }

    args.GetReturnValue().SetNull();
}
