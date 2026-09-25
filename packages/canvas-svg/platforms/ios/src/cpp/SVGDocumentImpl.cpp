#include "SVGDocumentImpl.h"
#include "SVGCaches.h"
#include "SVGNodeImpl.h"

void SVGDocumentImpl::Init(v8::Local<v8::Object> module, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    module->Set(context, ConvertToV8String(isolate, "SVGDocument"), func).FromJust();

    module->Set(context, ConvertToV8String(isolate, "createSVGDocument"),
                v8::FunctionTemplate::New(isolate, &CreateSVGDocument)->GetFunction(
                        context).ToLocalChecked()).FromJust();
}

SVGDocumentImpl *SVGDocumentImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0, SVGObjectWrapperImpl::kInternalFieldTag);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<SVGDocumentImpl *>(ptr);
}

v8::Local<v8::Object> SVGDocumentImpl::NewInstance(v8::Isolate *isolate, SVGDocumentImpl *document) {
    auto context = isolate->GetCurrentContext();
    v8::EscapableHandleScope scope(isolate);
    auto object = SVGDocumentImpl::GetCtor(isolate)->GetFunction(
            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
    SetNativeType(document, SVGNativeType::SVGDocument);
    object->SetAlignedPointerInInternalField(0, document, SVGObjectWrapperImpl::kInternalFieldTag);
    document->BindFinalizer(isolate, object);
    return scope.Escape(object);
}

void SVGDocumentImpl::CreateSVGDocument(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    SvgDocument *doc;
    if (args.Length() > 0 && args[0]->IsString()) {
        auto src = ConvertFromV8String(isolate, args[0]);
        doc = canvas_native_svg_document_create_with_string(src.c_str());
    } else {
        doc = canvas_native_svg_document_create();
    }

    if (doc == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = SVGDocumentImpl::NewInstance(isolate, new SVGDocumentImpl(doc));
    args.GetReturnValue().Set(ret);
}

void SVGDocumentImpl::Root(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto node = canvas_native_svg_document_root(ptr->GetDocument());
    if (node == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = SVGNodeImpl::NewInstance(isolate, new SVGNodeImpl(node));
    args.GetReturnValue().Set(ret);
}

void SVGDocumentImpl::CreateElement(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1 || !args[0]->IsString()) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto tag = ConvertFromV8String(isolate, args[0]);
    auto node = canvas_native_svg_node_create(tag.c_str());
    if (node == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = SVGNodeImpl::NewInstance(isolate, new SVGNodeImpl(node));
    args.GetReturnValue().Set(ret);
}

void SVGDocumentImpl::CreateTextNode(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1 || !args[0]->IsString()) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto text = ConvertFromV8String(isolate, args[0]);
    auto node = canvas_native_svg_node_create_text(text.c_str());
    if (node == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = SVGNodeImpl::NewInstance(isolate, new SVGNodeImpl(node));
    args.GetReturnValue().Set(ret);
}

void SVGDocumentImpl::GetElementById(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1 || !args[0]->IsString()) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto id = ConvertFromV8String(isolate, args[0]);
    auto node = canvas_native_svg_document_get_element_by_id(ptr->GetDocument(), id.c_str());
    if (node == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = SVGNodeImpl::NewInstance(isolate, new SVGNodeImpl(node));
    args.GetReturnValue().Set(ret);
}

void SVGDocumentImpl::RegisterId(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 2 || !args[0]->IsString() || !args[1]->IsObject()) {
        return;
    }

    auto nodeObj = args[1].As<v8::Object>();
    if (GetNativeType(nodeObj) != SVGNativeType::SVGNode) {
        return;
    }

    auto node = SVGNodeImpl::GetPointer(nodeObj);
    if (node == nullptr) {
        return;
    }

    auto id = ConvertFromV8String(isolate, args[0]);
    canvas_native_svg_document_register_id(ptr->GetDocument(), id.c_str(), node->GetNode());
}

void SVGDocumentImpl::UnregisterId(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1 || !args[0]->IsString()) {
        return;
    }

    auto id = ConvertFromV8String(isolate, args[0]);
    canvas_native_svg_document_unregister_id(ptr->GetDocument(), id.c_str());
}

void SVGDocumentImpl::SetContainerSize(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 2) {
        return;
    }

    auto width = static_cast<float>(args[0]->NumberValue(context).ToChecked());
    auto height = static_cast<float>(args[1]->NumberValue(context).ToChecked());
    canvas_native_svg_document_set_container_size(ptr->GetDocument(), width, height);
}

// Promotes a node out of the static content; null/undefined clears it.
void SVGDocumentImpl::SetLayer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    if (args.Length() < 1 || !args[0]->IsString()) {
        canvas_native_svg_document_set_layer(ptr->GetDocument(), nullptr);
        return;
    }
    auto id = ConvertFromV8String(isolate, args[0]);
    canvas_native_svg_document_set_layer(ptr->GetDocument(), id.c_str());
}

void SVGDocumentImpl::InvalidateBackdrop(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_svg_document_invalidate_backdrop(ptr->GetDocument());
}

void SVGDocumentImpl::SetFrameSharing(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto enabled = args.Length() > 0 && args[0]->BooleanValue(args.GetIsolate());
    canvas_native_svg_document_set_frame_sharing(ptr->GetDocument(), enabled);
}

void SVGDocumentImpl::InvalidateFrames(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_svg_document_invalidate_frames(ptr->GetDocument());
}

// The raw SvgDocument*, so the platform layer can hand it to its own native renderer
// (Android renders straight into a Bitmap). Android pointers fit well inside a double.
void SVGDocumentImpl::NativePointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }
    auto value = static_cast<double>(reinterpret_cast<uintptr_t>(ptr->GetDocument()));
    args.GetReturnValue().Set(value);
}

void SVGDocumentImpl::HasAnimations(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }
    args.GetReturnValue().Set(canvas_native_svg_document_has_animations(ptr->GetDocument()));
}

// CSS `@keyframes` from a stylesheet outside the document.
void SVGDocumentImpl::AddStylesheet(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1 || !args[0]->IsString()) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto css = ConvertFromV8String(isolate, args[0]);
    args.GetReturnValue().Set(canvas_native_svg_document_add_stylesheet(ptr->GetDocument(), css.c_str()));
}

// Seconds until every animation has finished; -1 when one of them repeats forever.
void SVGDocumentImpl::AnimationDuration(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }
    args.GetReturnValue().Set(canvas_native_svg_document_animation_duration(ptr->GetDocument()));
}

void SVGDocumentImpl::CurrentTime(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }
    args.GetReturnValue().Set(canvas_native_svg_document_current_time(ptr->GetDocument()));
}

// Returns whether anything is still animating, which is what stops the JS frame loop.
void SVGDocumentImpl::SetCurrentTime(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(0);
        return;
    }
    auto context = args.GetIsolate()->GetCurrentContext();
    double seconds = 0;
    if (args.Length() > 0) {
        seconds = args[0]->NumberValue(context).FromMaybe(0);
    }
    // Bit 0 still animating, bit 1 something changed. The caller needs both: one decides
    // whether to schedule another frame, the other whether this frame is worth drawing.
    args.GetReturnValue().Set(
            (int32_t) canvas_native_svg_document_set_current_time(ptr->GetDocument(), seconds));
}

void SVGDocumentImpl::RenderToBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    SVGDocumentImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 3 || !args[0]->IsArrayBufferView()) {
        return;
    }

    auto view = args[0].As<v8::ArrayBufferView>();
    auto buffer = view->Buffer();
    auto store = buffer->GetBackingStore();
    auto *pixels = static_cast<uint8_t *>(store->Data()) + view->ByteOffset();
    auto len = view->ByteLength();

    auto width = args[1]->Int32Value(context).ToChecked();
    auto height = args[2]->Int32Value(context).ToChecked();
    float scale = 1.0f;
    if (args.Length() > 3 && args[3]->IsNumber()) {
        scale = static_cast<float>(args[3]->NumberValue(context).ToChecked());
    }

    canvas_native_svg_document_render_to_buffer(ptr->GetDocument(), pixels, len, width, height, scale);
}

v8::Local<v8::FunctionTemplate> SVGDocumentImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = SVGCaches::Get(isolate);
    if (cache->SVGDocumentTmpl) {
        return cache->SVGDocumentTmpl->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "SVGDocument"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(ConvertToV8String(isolate, "root"),
              v8::FunctionTemplate::New(isolate, &Root));
    tmpl->Set(ConvertToV8String(isolate, "createElement"),
              v8::FunctionTemplate::New(isolate, &CreateElement));
    tmpl->Set(ConvertToV8String(isolate, "createTextNode"),
              v8::FunctionTemplate::New(isolate, &CreateTextNode));
    tmpl->Set(ConvertToV8String(isolate, "getElementById"),
              v8::FunctionTemplate::New(isolate, &GetElementById));
    tmpl->Set(ConvertToV8String(isolate, "registerId"),
              v8::FunctionTemplate::New(isolate, &RegisterId));
    tmpl->Set(ConvertToV8String(isolate, "unregisterId"),
              v8::FunctionTemplate::New(isolate, &UnregisterId));
    tmpl->Set(ConvertToV8String(isolate, "setContainerSize"),
              v8::FunctionTemplate::New(isolate, &SetContainerSize));
    tmpl->Set(ConvertToV8String(isolate, "setLayer"),
              v8::FunctionTemplate::New(isolate, &SetLayer));
    tmpl->Set(ConvertToV8String(isolate, "invalidateBackdrop"),
              v8::FunctionTemplate::New(isolate, &InvalidateBackdrop));
    tmpl->Set(ConvertToV8String(isolate, "setFrameSharing"),
              v8::FunctionTemplate::New(isolate, &SetFrameSharing));
    tmpl->Set(ConvertToV8String(isolate, "invalidateFrames"),
              v8::FunctionTemplate::New(isolate, &InvalidateFrames));
    tmpl->Set(ConvertToV8String(isolate, "nativePointer"),
              v8::FunctionTemplate::New(isolate, &NativePointer));
    tmpl->Set(ConvertToV8String(isolate, "renderToBuffer"),
              v8::FunctionTemplate::New(isolate, &RenderToBuffer));
    tmpl->Set(ConvertToV8String(isolate, "hasAnimations"),
              v8::FunctionTemplate::New(isolate, &HasAnimations));
    tmpl->Set(ConvertToV8String(isolate, "addStylesheet"),
              v8::FunctionTemplate::New(isolate, &AddStylesheet));
    tmpl->Set(ConvertToV8String(isolate, "animationDuration"),
              v8::FunctionTemplate::New(isolate, &AnimationDuration));
    tmpl->Set(ConvertToV8String(isolate, "currentTime"),
              v8::FunctionTemplate::New(isolate, &CurrentTime));
    tmpl->Set(ConvertToV8String(isolate, "setCurrentTime"),
              v8::FunctionTemplate::New(isolate, &SetCurrentTime));

    cache->SVGDocumentTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
