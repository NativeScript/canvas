//
// Created by Osei Fortune on 20/06/2024.
//

#pragma once
#pragma process_pending_includes

#include "RafImpl.h"
#include "OnRafCallback.h"

#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUDeviceImpl.h"
#include "GPUTextureImpl.h"


class GPUCanvasContextImpl : ObjectWrapperImpl {
public:
	static v8::CFunction fast_start_raf_;
	
	static v8::CFunction fast_stop_raf_;
	
	explicit GPUCanvasContextImpl(const CanvasGPUCanvasContext *context);
	
	void StartRaf();
	
	void StopRaf();
	
	static void __StartRaf(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static void __FastStartRaf(v8::Local<v8::Object> receiver_obj) {
		GPUCanvasContextImpl *ptr = GetPointer(receiver_obj);
		if (ptr == nullptr) {
			return;
		}
		
		ptr->StartRaf();
		
	}
	
	static void __StopRaf(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static void __FastStopRaf(v8::Local<v8::Object> receiver_obj) {
		GPUCanvasContextImpl *ptr = GetPointer(receiver_obj);
		if (ptr == nullptr) {
			return;
		}
		
		ptr->StopRaf();
		
	}
	
	~GPUCanvasContextImpl() {
		this->raf_.reset();
		canvas_native_webgpu_context_release(this->context_);
	}
	
	const CanvasGPUCanvasContext *GetContext();
	
	static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);
	
	static void __ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static GPUCanvasContextImpl *GetPointer(const v8::Local<v8::Object> &object);
	
	static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);
	
	static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUCanvasContextImpl *ctx) {
		auto context = isolate->GetCurrentContext();
		v8::EscapableHandleScope scope(isolate);
		auto object = GPUCanvasContextImpl::GetCtor(isolate)->GetFunction(
																																			context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
		SetNativeType(ctx, NativeType::GPUCanvasContext);
		object->SetAlignedPointerInInternalField(0, ctx);
		ctx->BindFinalizer(isolate, object);
		return scope.Escape(object);
	}
	
	static void Configure(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static void UnConfigure(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static void GetCurrentTexture(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static void PresentSurface(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static void GetCapabilities(const v8::FunctionCallbackInfo<v8::Value> &args);
	
	static void SetContinuousRenderMode(v8::Local<v8::String> property,
																			v8::Local<v8::Value> value,
																			const v8::PropertyCallbackInfo<void> &info);

	static void GetContinuousRenderMode(v8::Local<v8::String> property,
																			const v8::PropertyCallbackInfo<v8::Value> &info);
	
	void SetRaf(std::shared_ptr<RafImpl> raf);
	
	RafImpl *GetRaf();
	
	void Flush();

	static void Flush(intptr_t context);
	
private:
	const CanvasGPUCanvasContext *context_;
	std::shared_ptr<RafImpl> raf_;
	
	bool continuousRender_ = true;
};

