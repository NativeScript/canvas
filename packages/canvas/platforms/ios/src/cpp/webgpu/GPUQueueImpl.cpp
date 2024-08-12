//
// Created by Osei Fortune on 18/06/2024.
//

#include "GPUQueueImpl.h"
#include "Caches.h"
#include "GPUBufferImpl.h"
#include "GPUCommandBufferImpl.h"
#include "JSICallback.h"
#include "GPUTextureImpl.h"
#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "ImageDataImpl.h"
#include "CanvasRenderingContext2DImpl.h"
#include "WebGLRenderingContextBase.h"

GPUQueueImpl::GPUQueueImpl(const CanvasGPUQueue *queue) : queue_(queue) {}

const CanvasGPUQueue *GPUQueueImpl::GetGPUQueue() {
    return this->queue_;
}


void GPUQueueImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUQueue"), func).FromJust();
}

GPUQueueImpl *GPUQueueImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUQueueImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUQueueImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUQueueTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUQueue"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);


    tmpl->Set(
            ConvertToV8String(isolate, "copyExternalImageToTexture"),
            v8::FunctionTemplate::New(isolate, &CopyExternalImageToTexture));

    tmpl->Set(
            ConvertToV8String(isolate, "submit"),
            v8::FunctionTemplate::New(isolate, &Submit));

    tmpl->Set(
            ConvertToV8String(isolate, "onSubmittedWorkDone"),
            v8::FunctionTemplate::New(isolate, &SubmitWorkDone));


    tmpl->Set(
            ConvertToV8String(isolate, "writeBuffer"),
            v8::FunctionTemplate::New(isolate, &WriteBuffer));


    tmpl->Set(
            ConvertToV8String(isolate, "writeTexture"),
            v8::FunctionTemplate::New(isolate, &WriteTexture));


    cache->GPUQueueTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void GPUQueueImpl::CopyExternalImageToTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto sourceVal = args[0];
    auto destinationVal = args[1];
    auto sizeVal = args[2];

    if (sourceVal->IsObject() && destinationVal->IsObject() &&
        sizeVal->IsObject()) {

        auto sourceObj = sourceVal.As<v8::Object>();

        v8::Local<v8::Value> sourceSourceValue;
        sourceObj->Get(context, ConvertToV8String(isolate, "source")).ToLocal(&sourceSourceValue);

        auto sourceType = GetNativeType(sourceSourceValue);

        U8Buffer *buffer = nullptr;
        uint32_t width = 0;
        uint32_t height = 0;
        if (sourceType == NativeType::ImageBitmap) {
            auto imageAsset = ImageBitmapImpl::GetPointer(sourceSourceValue.As<v8::Object>());
            buffer = canvas_native_image_asset_get_data(imageAsset->GetImageAsset());
            width = canvas_native_image_asset_width(imageAsset->GetImageAsset());
            height = canvas_native_image_asset_height(imageAsset->GetImageAsset());
        } else if (sourceType == NativeType::ImageAsset) {
            auto imageAsset = ImageAssetImpl::GetPointer(sourceSourceValue.As<v8::Object>());
            buffer = canvas_native_image_asset_get_data(imageAsset->GetImageAsset());
            width = canvas_native_image_asset_width(imageAsset->GetImageAsset());
            height = canvas_native_image_asset_height(imageAsset->GetImageAsset());
        } else if (sourceType == NativeType::ImageData) {
            auto imageData = ImageDataImpl::GetPointer(sourceSourceValue.As<v8::Object>());
            buffer = canvas_native_image_data_get_data(imageData->GetImageData());
            width = canvas_native_image_data_get_width(imageData->GetImageData());
            height = canvas_native_image_data_get_height(imageData->GetImageData());
        } else if (sourceType == NativeType::CanvasRenderingContext2D) {
            auto c2d = CanvasRenderingContext2DImpl::GetPointer(sourceSourceValue.As<v8::Object>());
        } else if (sourceType == NativeType::WebGLRenderingContextBase) {
            auto webgl = WebGLRenderingContextBase::GetPointer(sourceSourceValue.As<v8::Object>());
        }

        if (buffer == nullptr) {
            // todo error ??
            return;
        }

        CanvasOrigin2d sourceOrigin{0, 0};


        v8::Local<v8::Value> sourceOriginVal;
        if (sourceObj->Get(context, ConvertToV8String(isolate, "origin")).ToLocal(
                &sourceOriginVal) &&
            sourceOriginVal->IsObject()) {
            auto sourceOriginObj = sourceOriginVal.As<v8::Object>();

            v8::Local<v8::Value> xVal;
            if (sourceOriginObj->Get(context, ConvertToV8String(isolate, "x")).ToLocal(&xVal) &&
                xVal->IsUint32()) {
                sourceOrigin.x = xVal->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> yVal;
            if (sourceOriginObj->Get(context, ConvertToV8String(isolate, "y")).ToLocal(&yVal) &&
                yVal->IsUint32()) {
                sourceOrigin.y = yVal->Uint32Value(context).FromJust();
            }

        }

        bool flipY = false;

        v8::Local<v8::Value> flipYVal;
        if (sourceObj->Get(context, ConvertToV8String(isolate, "flipY")).ToLocal(&flipYVal) &&
            flipYVal->IsBoolean()) {
            flipY = flipYVal->BooleanValue(isolate);
        }

        auto destinationObj = destinationVal.As<v8::Object>();

        auto textureVal = destinationObj->Get(context, ConvertToV8String(isolate,
                                                                         "texture")).ToLocalChecked();
        auto texture = GPUTextureImpl::GetPointer(textureVal.As<v8::Object>())->GetTexture();

        uint32_t mipLevel = 0;
        CanvasOrigin3d origin{0, 0, 0};
        CanvasTextureAspect aspect = CanvasTextureAspectAll;

        v8::Local<v8::Value> mipLevelVal;
        if (destinationObj->Get(context, ConvertToV8String(isolate, "mipLevel")).ToLocal(
                &mipLevelVal) &&
            mipLevelVal->IsUint32()) {
            mipLevel = mipLevelVal->Uint32Value(context).FromJust();
        }


        v8::Local<v8::Value> originVal;
        if (destinationObj->Get(context, ConvertToV8String(isolate, "origin")).ToLocal(
                &originVal) &&
            originVal->IsObject()) {
            auto originObj = originVal.As<v8::Object>();

            v8::Local<v8::Value> xVal;
            if (originObj->Get(context, ConvertToV8String(isolate, "x")).ToLocal(&xVal) &&
                xVal->IsUint32()) {
                origin.x = xVal->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> yVal;
            if (originObj->Get(context, ConvertToV8String(isolate, "y")).ToLocal(&yVal) &&
                yVal->IsUint32()) {
                origin.y = yVal->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> zVal;
            if (originObj->Get(context, ConvertToV8String(isolate, "z")).ToLocal(&zVal) &&
                zVal->IsUint32()) {
                origin.z = zVal->Uint32Value(context).FromJust();
            }

        }


        v8::Local<v8::Value> aspectVal;
        if (destinationObj->Get(context, ConvertToV8String(isolate, "aspect")).ToLocal(
                &aspectVal)) {
            if (aspectVal->IsString()) {
                auto aspectStr = ConvertFromV8String(isolate, aspectVal);
                if (aspectStr == "depth-only") {
                    aspect = CanvasTextureAspectDepthOnly;
                } else if (aspectStr == "stencil-only") {
                    aspect = CanvasTextureAspectStencilOnly;
                } else if (aspectStr == "all") {
                    aspect = CanvasTextureAspectAll;
                }
            }
        }

        CanvasImageCopyTexture destination{
                texture,
                mipLevel,
                origin,
                aspect
        };


        CanvasExtent3d extent3D = ParseExtent3d(isolate, sizeVal);

        auto data = canvas_native_u8_buffer_get_bytes(buffer);
        auto size = canvas_native_u8_buffer_get_length(buffer);

        if (data == nullptr || size == 0) {
            // todo error
            return;
        }

        CanvasImageCopyExternalImage source{
                data,
                size,
                sourceOrigin,
                flipY,
                width,
                height,
        };


        canvas_native_webgpu_queue_copy_external_image_to_texture(ptr->GetGPUQueue(), &source,
                                                                  &destination,
                                                                  &extent3D);

    }
}

void GPUQueueImpl::Submit(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    std::vector<const CanvasGPUCommandBuffer *> commandBuffers;
    auto commandBuffersVal = args[0];

    if (commandBuffersVal->IsArray()) {
        auto commandBuffersArray = commandBuffersVal.As<v8::Array>();
        auto len = commandBuffersArray->Length();

        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> item;
            commandBuffersArray->Get(context, i).ToLocal(&item);
            auto type = GetNativeType(item);
            if (type == NativeType::GPUCommandBuffer) {
                auto buffer = GPUCommandBufferImpl::GetPointer(item.As<v8::Object>());
                if (buffer != nullptr) {
                    commandBuffers.push_back(buffer->GetGPUCommandBuffer());
                }
            }
        }


        canvas_native_webgpu_queue_submit(ptr->GetGPUQueue(), commandBuffers.data(),
                                          commandBuffers.size());

    }


}

void GPUQueueImpl::SubmitWorkDone(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto cb = args[0].As<v8::Function>();
    auto callback = new AsyncCallback(isolate, cb, [](bool done, void *data) {
        if (data != nullptr) {
            auto async_data = static_cast<AsyncCallback *>(data);
            auto func = async_data->inner_.get();
            if (func != nullptr && func->isolate_ != nullptr) {
                v8::Isolate *isolate = func->isolate_;
                v8::Locker locker(isolate);
                v8::Isolate::Scope isolate_scope(
                        isolate);
                v8::HandleScope handle_scope(
                        isolate);
                v8::Local<v8::Function> callback = func->callback_.Get(
                        isolate);
                v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                v8::Context::Scope context_scope(
                        context);

                if (func->data != nullptr) {
                    // todo handle error
                    canvas_native_string_destroy(static_cast<char *>(func->data));
                    func->data = nullptr;
                }

                callback->Call(context,
                               context->Global(),
                               0, nullptr);

                delete static_cast<AsyncCallback *>(data);
            }

        }
    });

    callback->prepare();

    canvas_native_webgpu_queue_on_submitted_work_done(ptr->GetGPUQueue(),
                                                      [](char *error, void *data) {
                                                          if (data != nullptr) {
                                                              auto async_data = static_cast<AsyncCallback *>(data);
                                                              async_data->execute(true);
                                                          }
                                                      },
                                                      callback);


}

void GPUQueueImpl::WriteBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[0];

    if (bufferValue->IsObject()) {
        auto buffer = GPUBufferImpl::GetPointer(bufferValue.As<v8::Object>());

        auto bufferOffset = (uint64_t) args[1].As<v8::Number>()->Value();

        auto dataValue = args[2].As<v8::TypedArray>();

        auto offset = dataValue->ByteOffset();

        auto store = dataValue->Buffer()->GetBackingStore();

        auto data = static_cast<uint8_t *>(store->Data()) + offset;

        auto data_size = store->ByteLength();

        auto dataOffset = (uint64_t) args[3].As<v8::Number>()->Value();

        int64_t size = -1;
        auto sizeValue = args[4];

        if (sizeValue->IsNumber()) {
            size = (int64_t) sizeValue->ToNumber(context).ToLocalChecked()->Value();
        }


        if (buffer != nullptr) {
            canvas_native_webgpu_queue_write_buffer(ptr->GetGPUQueue(), buffer->GetGPUBuffer(),
                                                    bufferOffset, data, data_size, dataOffset,
                                                    size);
        }
    }


}

void GPUQueueImpl::WriteTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    bool flipY = false;


    auto destinationVal = args[0];
    auto dataVal = args[1];
    auto dataLayoutVal = args[2];
    auto sizeVal = args[3];

    if (destinationVal->IsObject() && dataVal->IsObject() && dataLayoutVal->IsObject() &&
        sizeVal->IsObject()) {
        auto destinationObj = destinationVal.As<v8::Object>();

        auto textureVal = destinationObj->Get(context, ConvertToV8String(isolate,
                                                                         "texture")).ToLocalChecked();
        auto texture = GPUTextureImpl::GetPointer(textureVal.As<v8::Object>())->GetTexture();

        uint32_t mipLevel = 0;
        CanvasOrigin3d origin{0, 0, 0};
        CanvasTextureAspect aspect = CanvasTextureAspectAll;

        v8::Local<v8::Value> mipLevelVal;
        if (destinationObj->Get(context, ConvertToV8String(isolate, "mipLevel")).ToLocal(
                &mipLevelVal) &&
            mipLevelVal->IsUint32()) {
            mipLevel = mipLevelVal->Uint32Value(context).FromJust();
        }


        v8::Local<v8::Value> originVal;
        if (destinationObj->Get(context, ConvertToV8String(isolate, "origin")).ToLocal(
                &originVal) &&
            originVal->IsObject()) {
            auto originObj = originVal.As<v8::Object>();

            v8::Local<v8::Value> xVal;
            if (originObj->Get(context, ConvertToV8String(isolate, "x")).ToLocal(&xVal) &&
                xVal->IsUint32()) {
                origin.x = xVal->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> yVal;
            if (originObj->Get(context, ConvertToV8String(isolate, "y")).ToLocal(&yVal) &&
                yVal->IsUint32()) {
                origin.y = yVal->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> zVal;
            if (originObj->Get(context, ConvertToV8String(isolate, "z")).ToLocal(&zVal) &&
                zVal->IsUint32()) {
                origin.z = zVal->Uint32Value(context).FromJust();
            }

        }


        v8::Local<v8::Value> aspectVal;
        if (destinationObj->Get(context, ConvertToV8String(isolate, "aspect")).ToLocal(
                &aspectVal)) {
            if (aspectVal->IsString()) {
                auto aspectStr = ConvertFromV8String(isolate, aspectVal);
                if (aspectStr == "depth-only") {
                    aspect = CanvasTextureAspectDepthOnly;
                } else if (aspectStr == "stencil-only") {
                    aspect = CanvasTextureAspectStencilOnly;
                } else if (aspectStr == "all") {
                    aspect = CanvasTextureAspectAll;
                }
            }
        }

        CanvasImageCopyTexture destination{
                texture,
                mipLevel,
                origin,
                aspect
        };
        auto array = dataVal.As<v8::TypedArray>();
        auto ab = array->Buffer();
        auto offset = array->ByteOffset();
        auto store = ab->GetBackingStore();
        auto data = static_cast<const uint8_t *>(store->Data()) + offset;
        auto size = array->ByteLength();

        auto dataLayoutObj = dataLayoutVal.As<v8::Object>();

        CanvasImageDataLayout layout{
                0, -1, -1
        };

        v8::Local<v8::Value> offsetVal;
        if (dataLayoutObj->Get(context, ConvertToV8String(isolate, "offset")).ToLocal(&offsetVal) &&
            offsetVal->IsNumber()) {
            layout.offset = (uint64_t) offsetVal->NumberValue(context).FromJust();
        }


        v8::Local<v8::Value> bytesPerRowVal;
        if (dataLayoutObj->Get(context, ConvertToV8String(isolate, "bytesPerRow")).ToLocal(
                &bytesPerRowVal) &&
            bytesPerRowVal->IsInt32()) {
            layout.bytes_per_row = bytesPerRowVal->Int32Value(context).FromJust();
        }


        v8::Local<v8::Value> rowsPerImageVal;
        if (dataLayoutObj->Get(context, ConvertToV8String(isolate, "rowsPerImage")).ToLocal(
                &rowsPerImageVal) &&
            rowsPerImageVal->IsInt32()) {
            layout.rows_per_image = rowsPerImageVal->Int32Value(context).FromJust();
        }


        CanvasExtent3d extent3D = ParseExtent3d(isolate, sizeVal);

        canvas_native_webgpu_queue_write_texture(ptr->GetGPUQueue(), &destination, &layout,
                                                 &extent3D, data, size);

    }


}



