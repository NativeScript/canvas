//
// Created by Osei Fortune on 20/06/2024.
//

#include "GPUCanvasContextImpl.h"
#include "Caches.h"

GPUCanvasContextImpl::GPUCanvasContextImpl(CanvasGPUCanvasContext *context) : context_(context) {}

CanvasGPUCanvasContext *GPUCanvasContextImpl::GetContext() {
    return this->context_;
}


void GPUCanvasContextImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUCanvasContext"), func);
}


GPUCanvasContextImpl *GPUCanvasContextImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUCanvasContextImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUCanvasContextImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUCanvasContextTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUCanvasContext"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);


    tmpl->Set(
            ConvertToV8String(isolate, "configure"),
            v8::FunctionTemplate::New(isolate, &Configure));

    tmpl->Set(
            ConvertToV8String(isolate, "unconfigure"),
            v8::FunctionTemplate::New(isolate, &UnConfigure));

    tmpl->Set(
            ConvertToV8String(isolate, "getCurrentTexture"),
            v8::FunctionTemplate::New(isolate, &GetCurrentTexture));


    cache->GPUCanvasContextTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void GPUCanvasContextImpl::Configure(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto optionsValue = args[0];
    if (ptr == nullptr) {
        return;
    }
    
    if (optionsValue->IsNullOrUndefined() || !optionsValue->IsObject()) {
        return;
    }
    
    auto options = optionsValue.As<v8::Object>();
    
    v8::Local<v8::Value> deviceValue;
    
    options->Get(context, ConvertToV8String(isolate, "device")).ToLocal(
                                                                        &deviceValue);
    
    
    if (deviceValue->IsNullOrUndefined() || !deviceValue->IsObject()) {
        return;
    }
    
    auto device = GPUDeviceImpl::GetPointer(deviceValue.As<v8::Object>());
    
    if (device == nullptr) { return; }
    
    CanvasGPUSurfaceConfiguration config{};
    config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModeOpaque;
    config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeFifo;
    config.view_formats = nullptr;
    config.view_formats_size = 0;
    config.usage = 0x10;
    
    
    /* ignore for now
     
     v8::Local<v8::Value> formatValue;
     
     
     options->Get(context, ConvertToV8String(isolate, "format")).ToLocal(
     &deviceValue);
     
     */
    
    v8::Local<v8::Value> usageValue;
    
    options->Get(context, ConvertToV8String(isolate, "usage")).ToLocal(
                                                                       &usageValue);
    
    if (!usageValue.IsEmpty() && usageValue->IsInt32()) {
        config.usage = usageValue->Int32Value(context).ToChecked();
    }
    
    
    v8::Local<v8::Value> presentModeValue;
    
    options->Get(context, ConvertToV8String(isolate, "presentMode")).ToLocal(
                                                                             &presentModeValue);
    
    if (!presentModeValue.IsEmpty()) {
        if(presentModeValue->IsString()){
            auto presentMode = ConvertFromV8String(isolate, presentModeValue);
            if(strcmp(presentMode.c_str(), "autoVsync") == 0){
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeAutoVsync;
            }else if(strcmp(presentMode.c_str(), "autoNoVsync") == 0){
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeAutoNoVsync;
            }else if(strcmp(presentMode.c_str(), "fifo") == 0){
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeFifo;
            }else if(strcmp(presentMode.c_str(), "fifoRelaxed") == 0){
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeFifoRelaxed;
            }else if(strcmp(presentMode.c_str(), "immediate") == 0){
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeImmediate;
            }else if(strcmp(presentMode.c_str(), "mailbox") == 0){
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeMailbox;
            }
            
        }else if (presentModeValue->IsInt32()){
            config.presentMode = (CanvasGPUPresentMode) presentModeValue->Int32Value(
                                                                                     context).ToChecked();
        }
    }
    
    
    
    v8::Local<v8::Value> alphaModeValue;
    options->Get(context, ConvertToV8String(isolate, "alphaMode")).ToLocal(
                                                                           &alphaModeValue);
    
    if (!alphaModeValue.IsEmpty()) {
        if(alphaModeValue->IsString()){
            auto alphaMode = ConvertFromV8String(isolate, alphaModeValue);
            if(strcmp(alphaMode.c_str(), "premultiplied") == 0){
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModePreMultiplied;
            }else if(strcmp(alphaMode.c_str(), "opaque") == 0){
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModeOpaque;
            }else if(strcmp(alphaMode.c_str(), "postmultiplied") == 0){
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModePostMultiplied;
            }
        }else if(alphaModeValue->IsInt32()){
            config.alphaMode = (CanvasGPUSurfaceAlphaMode) alphaModeValue->Int32Value(
                    context).ToChecked();
        }
        
    }

    canvas_native_webgpu_context_configure(ptr->GetContext(), device->GetGPUDevice(), &config);

}

void GPUCanvasContextImpl::UnConfigure(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    //  canvas_native_webgpu_context_unconfigure(ptr->GetContext());

}

void GPUCanvasContextImpl::GetCurrentTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    auto isolate = args.GetIsolate();
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }
    
    auto ctx = ptr->GetContext();

    auto texture = canvas_native_webgpu_context_get_current_texture(ctx);

    if (texture != nullptr) {
        auto textureImpl = new GPUTextureImpl(texture);
        auto ret = GPUTextureImpl::NewInstance(isolate, textureImpl);
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}