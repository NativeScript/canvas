#include "SVGNodeImpl.h"
#include "SVGCaches.h"

void SVGNodeImpl::Init(v8::Local<v8::Object> module, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    module->Set(context, ConvertToV8String(isolate, "SVGNode"), func).FromJust();
}

SVGNodeImpl *SVGNodeImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0, SVGObjectWrapperImpl::kInternalFieldTag);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<SVGNodeImpl *>(ptr);
}

v8::Local<v8::Object> SVGNodeImpl::NewInstance(v8::Isolate *isolate, SVGNodeImpl *node) {
    auto context = isolate->GetCurrentContext();
    v8::EscapableHandleScope scope(isolate);
    auto object = SVGNodeImpl::GetCtor(isolate)->GetFunction(
            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
    SetNativeType(node, SVGNativeType::SVGNode);
    object->SetAlignedPointerInInternalField(0, node, SVGObjectWrapperImpl::kInternalFieldTag);
    node->BindFinalizer(isolate, object);
    return scope.Escape(object);
}

void SVGNodeImpl::TagName(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGNodeImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto tag = canvas_native_svg_node_tag_name(ptr->GetNode());
    if (tag == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto ret = ConvertToV8String(isolate, tag);
    canvas_native_string_destroy(tag);
    args.GetReturnValue().Set(ret);
}

void SVGNodeImpl::SetAttribute(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGNodeImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    if (args.Length() < 2 || !args[0]->IsString()) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto name = ConvertFromV8String(isolate, args[0]);
    auto value = ConvertFromV8String(isolate, args[1]);

    auto ok = canvas_native_svg_node_set_attribute(ptr->GetNode(), name.c_str(), value.c_str());
    args.GetReturnValue().Set(ok);
}

void SVGNodeImpl::GetAttribute(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGNodeImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1 || !args[0]->IsString()) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto name = ConvertFromV8String(isolate, args[0]);
    auto value = canvas_native_svg_node_get_attribute(ptr->GetNode(), name.c_str());
    if (value == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = ConvertToV8String(isolate, value);
    canvas_native_string_destroy(value);
    args.GetReturnValue().Set(ret);
}

void SVGNodeImpl::CreateElement(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    if (args.Length() < 1 || !args[0]->IsString()) {
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

void SVGNodeImpl::CreateTextNode(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    if (args.Length() < 1 || !args[0]->IsString()) {
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

void SVGNodeImpl::AppendChild(const v8::FunctionCallbackInfo<v8::Value> &args) {
    SVGNodeImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1 || !args[0]->IsObject()) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto childObj = args[0].As<v8::Object>();
    if (GetNativeType(childObj) != SVGNativeType::SVGNode) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto child = GetPointer(childObj);
    if (child == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    auto ok = canvas_native_svg_node_append_child(ptr->GetNode(), child->GetNode());
    args.GetReturnValue().Set(ok);
}

void SVGNodeImpl::RemoveChild(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    SVGNodeImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr || args.Length() < 1) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto index = args[0]->Uint32Value(context).ToChecked();
    auto removed = canvas_native_svg_node_remove_child(ptr->GetNode(), index);
    if (removed == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = SVGNodeImpl::NewInstance(isolate, new SVGNodeImpl(removed));
    args.GetReturnValue().Set(ret);
}

// A text node's text; null for any other node.
void SVGNodeImpl::Text(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGNodeImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto value = canvas_native_svg_node_get_text(ptr->GetNode());
    if (value == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto ret = ConvertToV8String(isolate, value);
    canvas_native_string_destroy(value);
    args.GetReturnValue().Set(ret);
}

void SVGNodeImpl::SetText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    SVGNodeImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }

    std::string text;
    if (args.Length() > 0 && args[0]->IsString()) {
        text = ConvertFromV8String(isolate, args[0]);
    }
    args.GetReturnValue().Set(canvas_native_svg_node_set_text(ptr->GetNode(), text.c_str()));
}

v8::Local<v8::FunctionTemplate> SVGNodeImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = SVGCaches::Get(isolate);
    if (cache->SVGNodeTmpl) {
        return cache->SVGNodeTmpl->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "SVGNode"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(ConvertToV8String(isolate, "tagName"),
              v8::FunctionTemplate::New(isolate, &TagName));
    tmpl->Set(ConvertToV8String(isolate, "setAttribute"),
              v8::FunctionTemplate::New(isolate, &SetAttribute));
    tmpl->Set(ConvertToV8String(isolate, "getAttribute"),
              v8::FunctionTemplate::New(isolate, &GetAttribute));
    tmpl->Set(ConvertToV8String(isolate, "appendChild"),
              v8::FunctionTemplate::New(isolate, &AppendChild));
    tmpl->Set(ConvertToV8String(isolate, "removeChild"),
              v8::FunctionTemplate::New(isolate, &RemoveChild));
    tmpl->Set(ConvertToV8String(isolate, "text"),
              v8::FunctionTemplate::New(isolate, &Text));
    tmpl->Set(ConvertToV8String(isolate, "setText"),
              v8::FunctionTemplate::New(isolate, &SetText));

    cache->SVGNodeTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
