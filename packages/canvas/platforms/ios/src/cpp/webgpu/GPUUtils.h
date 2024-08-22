//
// Created by Osei Fortune on 18/07/2024.
//

#ifndef CANVAS_ANDROID_GPUUTILS_H
#define CANVAS_ANDROID_GPUUTILS_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUSamplerImpl.h"
#include "GPUTextureViewImpl.h"
#include "GPUBufferImpl.h"

inline static CanvasStoreOp
ParseCanvasStoreOp(v8::Isolate *isolate, const v8::Local<v8::Value> &obj){
    auto ret = CanvasStoreOp::CanvasStoreOpStore;
    if(!obj.IsEmpty()){
        if (obj->IsUint32()) {
            ret = (CanvasStoreOp) obj->Uint32Value(isolate->GetCurrentContext()).ToChecked();
        } else if (obj->IsString()) {
            auto val = ConvertFromV8String(isolate, obj);
            if (val == "discard") {
                ret = CanvasStoreOp::CanvasStoreOpDiscard;
            } else if (val == "store") {
                ret = CanvasStoreOp::CanvasStoreOpStore;
            }
        }
    }
    return ret;
}

inline static CanvasLoadOp
ParseCanvasLoadOp(v8::Isolate *isolate, const v8::Local<v8::Value> &obj){
    auto ret = CanvasLoadOp::CanvasLoadOpClear;
    if(!obj.IsEmpty()){
        if (obj->IsUint32()) {
            ret = (CanvasLoadOp) obj->Uint32Value(isolate->GetCurrentContext()).ToChecked();
        } else if (obj->IsString()) {
            auto val = ConvertFromV8String(isolate, obj);
            if (val == "clear") {
                ret = CanvasLoadOp::CanvasLoadOpClear;
            } else if (val == "load") {
                ret = CanvasLoadOp::CanvasLoadOpLoad;
            }
        }
    }
    return ret;
}

inline static std::vector<CanvasBindGroupEntry>
ParseBindGroupEntries(v8::Isolate *isolate, const v8::Local<v8::Value> &obj) {
    std::vector<CanvasBindGroupEntry> entries;

    if (!obj.IsEmpty() && obj->IsArray()) {
        auto context = isolate->GetCurrentContext();
        auto entriesArray = obj.As<v8::Array>();
        auto len = entriesArray->Length();
        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> entryVal;
            entriesArray->Get(context, i).ToLocal(&entryVal);
            if (!entryVal.IsEmpty() && entryVal->IsObject()) {
                auto entryObj = entryVal.As<v8::Object>();
                auto binding = entryObj->Get(context, ConvertToV8String(isolate,
                                                                        "binding")).ToLocalChecked()->Uint32Value(
                        context).FromJust();
                v8::Local<v8::Value> resourceVal;
                entryObj->Get(context, ConvertToV8String(isolate, "resource")).ToLocal(
                        &resourceVal);

                auto resourceType = ObjectWrapperImpl::GetNativeType(resourceVal);
                switch (resourceType) {
                    case NativeType::GPUSampler: {
                        auto sampler = GPUSamplerImpl::GetPointer(resourceVal.As<v8::Object>());
                        if (sampler != nullptr) {
                            auto resource = CanvasBindGroupEntryResource{
                                    CanvasBindGroupEntryResourceSampler,
                            };
                            resource.sampler = sampler->GetSampler();
                            CanvasBindGroupEntry entry{binding, resource};
                            entries.push_back(entry);
                        }
                    }
                        break;
                    case NativeType::GPUTextureView: {
                        auto textureView = GPUTextureViewImpl::GetPointer(
                                resourceVal.As<v8::Object>());
                        if (textureView != nullptr) {
                            auto resource = CanvasBindGroupEntryResource{
                                    CanvasBindGroupEntryResourceTextureView,
                            };
                            resource.texture_view = textureView->GetTextureView();
                            CanvasBindGroupEntry entry{binding, resource};
                            entries.push_back(entry);
                        }
                    }
                        break;
                    default: {
                        if (!resourceVal.IsEmpty() && resourceVal->IsObject()) {
                            auto resourceObj = resourceVal.As<v8::Object>();
                            v8::Local<v8::Value> bufferVal;
                            resourceObj->Get(context,
                                             ConvertToV8String(isolate, "buffer")).ToLocal(
                                    &bufferVal);
                            if (ObjectWrapperImpl::GetNativeType(bufferVal) ==
                                NativeType::GPUBuffer) {
                                auto bufferObj = bufferVal.As<v8::Object>();
                                auto buffer = GPUBufferImpl::GetPointer(bufferObj);
                                if (buffer != nullptr) {
                                    auto resource = CanvasBindGroupEntryResource{
                                            CanvasBindGroupEntryResourceBuffer,
                                    };
                                    int64_t offset = -1;

                                    v8::Local<v8::Value> offsetVal;

                                    resourceObj->Get(context,
                                                     ConvertToV8String(isolate,
                                                                       "offset")).ToLocal(
                                            &offsetVal);
                                    if (!offsetVal.IsEmpty() && offsetVal->IsNumber()) {
                                        offset = (int64_t) offsetVal->NumberValue(
                                                context).ToChecked();
                                    }

                                    int64_t size = -1;

                                    v8::Local<v8::Value> sizeVal;
                                    resourceObj->Get(context,
                                                     ConvertToV8String(isolate,
                                                                       "size")).ToLocal(
                                            &sizeVal);
                                    if (!sizeVal.IsEmpty() && sizeVal->IsNumber()) {
                                        size = (int64_t) sizeVal->NumberValue(
                                                context).ToChecked();
                                    }

                                    resource.buffer = CanvasBufferBinding{
                                            buffer->GetGPUBuffer(), offset, size
                                    };

                                    CanvasBindGroupEntry entry{binding, resource};
                                    entries.push_back(entry);
                                }
                            }
                        }

                    }
                        break;
                }

            }
        }
    }
    return entries;
}

inline static std::vector<CanvasBindGroupLayoutEntry>
ParseBindGroupLayoutEntries(v8::Isolate *isolate, const v8::Local<v8::Value> &obj) {
    std::vector<CanvasBindGroupLayoutEntry> entries;

    if (!obj.IsEmpty() && obj->IsArray()) {
        auto entriesArray = obj.As<v8::Array>();
        auto context = isolate->GetCurrentContext();
        auto len = entriesArray->Length();
        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> entryVal;
            entriesArray->Get(context, i).ToLocal(&entryVal);
            if (!entryVal.IsEmpty() && entryVal->IsObject()) {
                auto entryObj = entryVal.As<v8::Object>();
                auto binding = entryObj->Get(context, ConvertToV8String(isolate,
                                                                        "binding")).ToLocalChecked()->Uint32Value(
                        context).FromJust();


                auto visibility = entryObj->Get(context, ConvertToV8String(isolate,
                                                                           "visibility")).ToLocalChecked()->Uint32Value(
                        context).FromJust();


                v8::Local<v8::Value> bufferVal;

                entryObj->Get(context, ConvertToV8String(isolate, "buffer")).ToLocal(
                        &bufferVal);

                if (!bufferVal.IsEmpty() && bufferVal->IsObject()) {
                    auto bufferObj = bufferVal.As<v8::Object>();
                    v8::Local<v8::Value> bufferType;
                    bufferObj->Get(context, ConvertToV8String(isolate, "type")).ToLocal(
                            &bufferType);

                    CanvasBufferBindingType type = CanvasBufferBindingTypeUniform;

                    if (!bufferType.IsEmpty() && bufferType->IsString()) {
                        auto typeStr = ConvertFromV8String(isolate, bufferType);
                        if (typeStr == "read-only-storage") {
                            type = CanvasBufferBindingTypeReadOnlyStorage;
                        } else if (typeStr == "storage") {
                            type = CanvasBufferBindingTypeStorage;
                        } else if (typeStr == "uniform") {
                            type = CanvasBufferBindingTypeUniform;
                        }
                    }

                    bool has_dynamic_offset = false;

                    v8::Local<v8::Value> hasDynamicOffsetVal;
                    bufferObj->Get(context,
                                   ConvertToV8String(isolate, "hasDynamicOffset")).ToLocal(
                            &hasDynamicOffsetVal);


                    if (!hasDynamicOffsetVal.IsEmpty() && hasDynamicOffsetVal->IsBoolean()) {
                        has_dynamic_offset = hasDynamicOffsetVal->BooleanValue(isolate);
                    }

                    int64_t min_binding_size = -1;

                    v8::Local<v8::Value> minBindingSizeVal;
                    bufferObj->Get(context,
                                   ConvertToV8String(isolate, "minBindingSize")).ToLocal(
                            &minBindingSizeVal);


                    if (!minBindingSizeVal.IsEmpty() && minBindingSizeVal->IsNumber()) {
                        min_binding_size = (int64_t) minBindingSizeVal->NumberValue(
                                context).ToChecked();
                    }

                    CanvasBindingType buffer{
                            CanvasBindingTypeBuffer
                    };

                    buffer.buffer = CanvasBufferBindingLayout{
                            type, has_dynamic_offset, min_binding_size
                    };


                    CanvasBindGroupLayoutEntry entry{
                            binding,
                            visibility,
                            buffer
                    };

                    entries.push_back(entry);

                    continue;
                }

                v8::Local<v8::Value> externalTextureVal;

                entryObj->Get(context, ConvertToV8String(isolate, "externalTexture")).ToLocal(
                        &externalTextureVal);

                if (!externalTextureVal.IsEmpty() && externalTextureVal->IsObject()) {
                    // todo
//                        CanvasBindingType buffer{
//                                CanvasBindingTypeTexture
//                        };
//
//                        buffer.buffer = CanvasBufferBindingLayout{
//                                type, has_dynamic_offset, min_binding_size
//                        };
//
//
//                        CanvasBindGroupLayoutEntry entry{
//                                binding,
//                                visibility,
//                                buffer
//                        };

                    continue;
                }


                v8::Local<v8::Value> samplerVal;

                entryObj->Get(context, ConvertToV8String(isolate, "sampler")).ToLocal(
                        &samplerVal);

                if (!samplerVal.IsEmpty() && samplerVal->IsObject()) {
                    auto samplerObj = samplerVal.As<v8::Object>();
                    v8::Local<v8::Value> samplerType;
                    samplerObj->Get(context, ConvertToV8String(isolate, "type")).ToLocal(
                            &samplerType);

                    CanvasSamplerBindingType type = CanvasSamplerBindingTypeFiltering;

                    if (!samplerType.IsEmpty() && samplerType->IsString()) {
                        auto typeStr = ConvertFromV8String(isolate, samplerType);
                        if (typeStr == "comparison") {
                            type = CanvasSamplerBindingTypeComparison;
                        } else if (typeStr == "non-filtering") {
                            type = CanvasSamplerBindingTypeNonFiltering;
                        } else if (typeStr == "filtering") {
                            type = CanvasSamplerBindingTypeFiltering;
                        }
                    }

                    CanvasBindingType sampler{
                            CanvasBindingTypeSampler,
                    };

                    sampler.sampler = CanvasSamplerBindingLayout{
                            type
                    };


                    CanvasBindGroupLayoutEntry entry{
                            binding,
                            visibility,
                            sampler
                    };

                    entries.push_back(entry);

                    continue;

                }


                v8::Local<v8::Value> storageTextureVal;

                entryObj->Get(context, ConvertToV8String(isolate, "storageTexture")).ToLocal(
                        &storageTextureVal);

                if (!storageTextureVal.IsEmpty() && storageTextureVal->IsObject()) {
                    auto storageTextureObj = storageTextureVal.As<v8::Object>();

                    CanvasBindingType storage{
                            CanvasBindingTypeStorageTexture
                    };

                    CanvasStorageTextureAccess access = CanvasStorageTextureAccessWriteOnly;

                    v8::Local<v8::Value> accessVal;

                    storageTextureObj->Get(context,
                                           ConvertToV8String(isolate, "access")).ToLocal(
                            &accessVal);


                    if (!accessVal.IsEmpty() && accessVal->IsString()) {
                        auto accessStr = ConvertFromV8String(isolate, accessVal);

                        if (accessStr == "write-only") {
                            access = CanvasStorageTextureAccessWriteOnly;
                        } else if (accessStr == "read-only") {
                            access = CanvasStorageTextureAccessReadOnly;
                        } else if (accessStr == "read-write") {
                            access = CanvasStorageTextureAccessReadWrite;
                        }
                    }


                    CanvasTextureViewDimension view_dimension = CanvasTextureViewDimensionD2;


                    v8::Local<v8::Value> viewDimensionVal;

                    storageTextureObj->Get(context,
                                           ConvertToV8String(isolate, "viewDimension")).ToLocal(
                            &viewDimensionVal);


                    if (!viewDimensionVal.IsEmpty() && viewDimensionVal->IsString()) {
                        auto viewDimensionStr = ConvertFromV8String(isolate, viewDimensionVal);
                        if (viewDimensionStr == "1d") {
                            view_dimension = CanvasTextureViewDimensionD1;
                        } else if (viewDimensionStr == "2d") {
                            view_dimension = CanvasTextureViewDimensionD2;
                        } else if (viewDimensionStr == "2d-array") {
                            view_dimension = CanvasTextureViewDimensionD2Array;
                        } else if (viewDimensionStr == "cube") {
                            view_dimension = CanvasTextureViewDimensionCube;
                        } else if (viewDimensionStr == "cube-array") {
                            view_dimension = CanvasTextureViewDimensionCubeArray;
                        } else if (viewDimensionStr == "3d") {
                            view_dimension = CanvasTextureViewDimensionD3;
                        }
                    }


                    v8::Local<v8::Value> formatVal;

                    storageTextureObj->Get(context,
                                           ConvertToV8String(isolate, "format")).ToLocal(
                            &formatVal);


                    if (!formatVal.IsEmpty() && formatVal->IsString()) {
                        auto formatStr = ConvertFromV8String(isolate, formatVal);
                        auto textureFormat = canvas_native_webgpu_enum_string_to_gpu_texture(
                                formatStr.c_str());
                        if (textureFormat.tag == CanvasOptionalGPUTextureFormatSome) {
                            storage.storage_texture = CanvasStorageTextureBindingLayout{
                                    access,
                                    textureFormat.some,
                                    view_dimension
                            };

                            CanvasBindGroupLayoutEntry entry{
                                    binding,
                                    visibility,
                                    storage
                            };

                            entries.push_back(entry);

                            continue;

                        } else {
                            // todo throw ??
                            continue;
                        }
                    }


                }


                v8::Local<v8::Value> textureVal;

                entryObj->Get(context, ConvertToV8String(isolate, "texture")).ToLocal(
                        &textureVal);

                if (!textureVal.IsEmpty() && textureVal->IsObject()) {
                    auto textureObj = textureVal.As<v8::Object>();
                    bool multisampled = false;

                    v8::Local<v8::Value> multisampledVal;

                    textureObj->Get(context,
                                    ConvertToV8String(isolate, "multisampled")).ToLocal(
                            &multisampledVal);

                    if (!multisampledVal.IsEmpty() && multisampledVal->IsBoolean()) {
                        multisampled = multisampledVal->BooleanValue(isolate);
                    }


                    v8::Local<v8::Value> sampleTypeVal;
                    textureObj->Get(context, ConvertToV8String(isolate, "sampleType")).ToLocal(
                            &sampleTypeVal);

                    CanvasTextureSampleType type = CanvasTextureSampleTypeFloat;

                    if (!sampleTypeVal.IsEmpty() && sampleTypeVal->IsString()) {
                        auto typeStr = ConvertFromV8String(isolate, sampleTypeVal);
                        if (typeStr == "depth") {
                            type = CanvasTextureSampleTypeDepth;
                        } else if (typeStr == "float") {
                            type = CanvasTextureSampleTypeFloat;
                        } else if (typeStr == "sint") {
                            type = CanvasTextureSampleTypeSint;
                        } else if (typeStr == "uint") {
                            type = CanvasTextureSampleTypeUint;
                        } else if (typeStr == "unfilterable-float") {
                            type = CanvasTextureSampleTypeUnfilterableFloat;
                        }
                    }


                    CanvasTextureViewDimension view_dimension = CanvasTextureViewDimensionD2;


                    v8::Local<v8::Value> viewDimensionVal;

                    textureObj->Get(context,
                                    ConvertToV8String(isolate, "viewDimension")).ToLocal(
                            &viewDimensionVal);


                    if (!viewDimensionVal.IsEmpty() && viewDimensionVal->IsString()) {
                        auto viewDimensionStr = ConvertFromV8String(isolate, viewDimensionVal);
                        if (viewDimensionStr == "1d") {
                            view_dimension = CanvasTextureViewDimensionD1;
                        } else if (viewDimensionStr == "2d") {
                            view_dimension = CanvasTextureViewDimensionD2;
                        } else if (viewDimensionStr == "2d-array") {
                            view_dimension = CanvasTextureViewDimensionD2Array;
                        } else if (viewDimensionStr == "cube") {
                            view_dimension = CanvasTextureViewDimensionCube;
                        } else if (viewDimensionStr == "cube-array") {
                            view_dimension = CanvasTextureViewDimensionCubeArray;
                        } else if (viewDimensionStr == "3d") {
                            view_dimension = CanvasTextureViewDimensionD3;
                        }
                    }


                    CanvasBindingType texture{
                            CanvasBindingTypeTexture
                    };

                    texture.texture = CanvasTextureBindingLayout{
                            type,
                            view_dimension,
                            multisampled
                    };


                    CanvasBindGroupLayoutEntry entry{
                            binding,
                            visibility,
                            texture
                    };

                    entries.push_back(entry);

                    continue;

                }

            }
        }
    }

    return entries;
}

inline static CanvasCompareFunction
ParseCompareFunction(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                     CanvasCompareFunction defaultValue) {

    auto compareStr = ConvertFromV8String(isolate, obj);

    if (compareStr == "never") {
        return CanvasCompareFunctionNever;
    } else if (compareStr == "less") {
        return CanvasCompareFunctionLess;
    } else if (compareStr == "equal") {
        return CanvasCompareFunctionEqual;
    } else if (compareStr == "less-equal") {
        return CanvasCompareFunctionLessEqual;
    } else if (compareStr == "greater") {
        return CanvasCompareFunctionGreater;
    } else if (compareStr == "not-equal") {
        return CanvasCompareFunctionNotEqual;
    } else if (compareStr == "greater-equal") {
        return CanvasCompareFunctionGreaterEqual;
    } else if (compareStr == "always") {
        return CanvasCompareFunctionAlways;
    }

    return defaultValue;
}

inline static CanvasStencilOperation
ParseStencilOperation(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                      CanvasStencilOperation defaultValue) {

    auto op = ConvertFromV8String(isolate, obj);

    if (op == "decrement-clamp") {
        return CanvasStencilOperationDecrementClamp;
    } else if (op == "decrement-wrap") {
        return CanvasStencilOperationDecrementWrap;
    } else if (op == "invert") {
        return CanvasStencilOperationInvert;
    } else if (op == "increment-clamp") {
        return CanvasStencilOperationIncrementClamp;
    } else if (op == "increment-wrap") {
        return CanvasStencilOperationIncrementWrap;
    } else if (op == "keep") {
        return CanvasStencilOperationKeep;
    } else if (op == "replace") {
        return CanvasStencilOperationReplace;
    } else if (op == "zero") {
        return CanvasStencilOperationZero;
    }

    return defaultValue;
}


inline static CanvasBlendFactor
ParseBlendFactor(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                 CanvasBlendFactor defaultValue) {

    auto factor = ConvertFromV8String(isolate, obj);

    if (factor == "constant") {
        return CanvasBlendFactorConstant;
    } else if (factor == "dst") {
        return CanvasBlendFactorDst;
    } else if (factor == "dst-alpha") {
        return CanvasBlendFactorDstAlpha;
    } else if (factor == "one") {
        return CanvasBlendFactorOne;
    } else if (factor == "one-minus-dst") {
        return CanvasBlendFactorOneMinusDst;
    } else if (factor == "one-minus-src") {
        return CanvasBlendFactorOneMinusSrc;
    } else if (factor == "one-minus-src-alpha") {
        return CanvasBlendFactorOneMinusSrcAlpha;
    } else if (factor == "one-minus-dst-alpha") {
        return CanvasBlendFactorOneMinusDstAlpha;
    } else if (factor == "one-minus-constant") {
        return CanvasBlendFactorOneMinusConstant;
    } else if (factor == "src") {
        return CanvasBlendFactorSrc;
    } else if (factor == "src-alpha") {
        return CanvasBlendFactorSrcAlpha;
    } else if (factor == "src-alpha-saturated") {
        return CanvasBlendFactorSrcAlphaSaturated;
    } else if (factor == "zero") {
        return CanvasBlendFactorZero;
    }

    return defaultValue;
}


inline static CanvasBlendOperation
ParseBlendOperation(v8::Isolate *isolate, const v8::Local<v8::Value> &obj,
                    CanvasBlendOperation defaultValue) {

    auto op = ConvertFromV8String(isolate, obj);

    if (op == "add") {
        return CanvasBlendOperationAdd;
    } else if (op == "max") {
        return CanvasBlendOperationMax;
    } else if (op == "min") {
        return CanvasBlendOperationMin;
    } else if (op == "reverse-subtract") {
        return CanvasBlendOperationReverseSubtract;
    } else if (op == "subtract") {
        return CanvasBlendOperationSubtract;
    }

    return defaultValue;
}

inline static CanvasExtent3d
ParseExtent3d(v8::Isolate *isolate, const v8::Local<v8::Value> &obj) {

    auto context = isolate->GetCurrentContext();
    CanvasExtent3d ret{
            0, 1, 1
    };

    if (!obj.IsEmpty()) {
        if (obj->IsArray()) {
            auto array = obj.As<v8::Array>();
            v8::Local<v8::Value> width;
            if (array->Get(context, 0).ToLocal(&width) &&
                width->IsUint32()) {
                ret.width = width->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> height;
            if (array->Get(context, 1).ToLocal(&height) &&
                height->IsUint32()) {
                ret.height = height->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> depthOrArrayLayers;
            if (array->Get(context, 2).ToLocal(
                    &depthOrArrayLayers) && depthOrArrayLayers->IsUint32()) {
                ret.depth_or_array_layers = depthOrArrayLayers->Uint32Value(context).FromJust();
            }
        } else if (obj->IsObject()) {
            auto extObj = obj.As<v8::Object>();
            v8::Local<v8::Value> width;
            if (extObj->Get(context, ConvertToV8String(isolate, "width")).ToLocal(&width) &&
                width->IsUint32()) {
                ret.width = width->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> height;
            if (extObj->Get(context, ConvertToV8String(isolate, "height")).ToLocal(&height) &&
                height->IsUint32()) {
                ret.height = height->Uint32Value(context).FromJust();
            }

            v8::Local<v8::Value> depthOrArrayLayers;
            if (extObj->Get(context, ConvertToV8String(isolate, "depthOrArrayLayers")).ToLocal(
                    &depthOrArrayLayers) && depthOrArrayLayers->IsUint32()) {
                ret.depth_or_array_layers = depthOrArrayLayers->Uint32Value(context).FromJust();
            }
        }
    }

    return ret;
}


inline static CanvasColor
ParseColor(v8::Isolate *isolate, const v8::Local<v8::Value> &obj) {

    auto context = isolate->GetCurrentContext();
    CanvasColor ret{
            0, 0, 0, 0
    };

    if (!obj.IsEmpty()) {
        if (obj->IsArray()) {
            auto array = obj.As<v8::Array>();
            v8::Local<v8::Value> r;
            if (array->Get(context, 0).ToLocal(&r) &&
                r->IsNumber()) {
                ret.r = r->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> g;
            if (array->Get(context, 1).ToLocal(&g) &&
                g->IsNumber()) {
                ret.g = g->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> b;
            if (array->Get(context, 2).ToLocal(
                    &b) && b->IsNumber()) {
                ret.b = b->NumberValue(context).FromJust();
            }


            v8::Local<v8::Value> a;
            if (array->Get(context, 3).ToLocal(
                    &a) && a->IsNumber()) {
                ret.a = a->NumberValue(context).FromJust();
            }
        } else if (obj->IsObject()) {
            auto colorObj = obj.As<v8::Object>();
            v8::Local<v8::Value> r;
            if (colorObj->Get(context, ConvertToV8String(isolate, "r")).ToLocal(&r) &&
                r->IsNumber()) {
                ret.r = r->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> g;
            if (colorObj->Get(context, ConvertToV8String(isolate, "g")).ToLocal(&g) &&
                g->IsNumber()) {
                ret.g = g->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> b;
            if (colorObj->Get(context, ConvertToV8String(isolate, "b")).ToLocal(
                    &b) && b->IsNumber()) {
                ret.b = b->NumberValue(context).FromJust();
            }

            v8::Local<v8::Value> a;
            if (colorObj->Get(context, ConvertToV8String(isolate, "a")).ToLocal(
                    &a) && a->IsNumber()) {
                ret.a = a->NumberValue(context).FromJust();
            }
        }
    }

    return ret;
}

#endif //CANVAS_ANDROID_GPUUTILS_H
