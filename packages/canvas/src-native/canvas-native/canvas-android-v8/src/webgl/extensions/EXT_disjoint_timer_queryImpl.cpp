//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_disjoint_timer_queryImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

EXT_disjoint_timer_queryImpl::EXT_disjoint_timer_queryImpl(rust::Box<EXT_disjoint_timer_query> query) : query_(
        std::move(query)) {

}

EXT_disjoint_timer_queryImpl *EXT_disjoint_timer_queryImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<EXT_disjoint_timer_queryImpl *>(ptr);
}

v8::Local<v8::Object>
EXT_disjoint_timer_queryImpl::NewInstance(v8::Isolate *isolate, rust::Box<EXT_disjoint_timer_query> query) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    EXT_disjoint_timer_queryImpl *queryImpl = new EXT_disjoint_timer_queryImpl(std::move(query));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "EXT_disjoint_timer_query");
    auto ext = v8::External::New(isolate, queryImpl);
    result->SetInternalField(0, ext);

    result->Set(context, Helpers::ConvertToV8String(isolate, "QUERY_COUNTER_BITS_EXT"),
                v8::Uint32::New(isolate, GL_QUERY_COUNTER_BITS_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "CURRENT_QUERY_EXT"),
                v8::Uint32::New(isolate, GL_CURRENT_QUERY_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "QUERY_RESULT_EXT"),
                v8::Uint32::New(isolate, GL_QUERY_RESULT_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "QUERY_RESULT_AVAILABLE_EXT"),
                v8::Uint32::New(isolate, GL_QUERY_RESULT_AVAILABLE_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "TIME_ELAPSED_EXT"),
                v8::Uint32::New(isolate, GL_TIME_ELAPSED_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "TIMESTAMP_EXT"),
                v8::Uint32::New(isolate, GL_TIMESTAMP_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "GPU_DISJOINT_EXT"),
                v8::Uint32::New(isolate, GL_GPU_DISJOINT_EXT));

    return handle_scope.Escape(result);
}

v8::Local<v8::FunctionTemplate> EXT_disjoint_timer_queryImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_disjoint_timer_queryImplTmpl.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "EXT_disjoint_timer_query"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->Set(Helpers::ConvertToV8String(isolate, "createQueryExt"), v8::FunctionTemplate::New(isolate, &CreateQueryExt));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "deleteQueryExt"), v8::FunctionTemplate::New(isolate, &DeleteQueryExt));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "isQueryExt"), v8::FunctionTemplate::New(isolate, &IsQueryExt));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "beginQueryExt"), v8::FunctionTemplate::New(isolate, &BeginQueryExt));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "endQueryExt"), v8::FunctionTemplate::New(isolate, &EndQueryExt));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "queryCounterExt"), v8::FunctionTemplate::New(isolate, &QueryCounterExt));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "getQueryExt"), v8::FunctionTemplate::New(isolate, &GetQueryExt));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "qetQueryObjectExt"), v8::FunctionTemplate::New(isolate, &GetQueryObjectExt));

    cache->EXT_disjoint_timer_queryImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void EXT_disjoint_timer_queryImpl::CreateQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.Holder());
    auto ret = canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(*ptr->query_);
    args.GetReturnValue().Set(ret);

}

void EXT_disjoint_timer_queryImpl::DeleteQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto query = args[0];
    canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(query->Uint32Value(context).ToChecked(),
                                                                  *ptr->query_);
}

void EXT_disjoint_timer_queryImpl::IsQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto query = args[0];
    args.GetReturnValue().Set(
            canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(query->Uint32Value(context).ToChecked(),
                                                                      *ptr->query_)
    );
}

void EXT_disjoint_timer_queryImpl::BeginQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto target = args[0];
    auto query = args[1];
    canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
            target->Uint32Value(context).ToChecked(),
            query->Uint32Value(context).ToChecked(),
            *ptr->query_
    );
}

void EXT_disjoint_timer_queryImpl::EndQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto target = args[0];
    canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
            target->Uint32Value(context).ToChecked(),
            *ptr->query_
    );
}

void EXT_disjoint_timer_queryImpl::QueryCounterExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto query = args[0];
    auto target = args[1];
    canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(query->Uint32Value(context).ToChecked(),
                                                                   target->Uint32Value(context).ToChecked(),
                                                                   *ptr->query_);
}

void EXT_disjoint_timer_queryImpl::GetQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto target = args[0];
    auto pname = args[1];
    auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
            target->Uint32Value(context).ToChecked(),
            pname->Uint32Value(context).ToChecked(),
            *ptr->query_
    );
    args.GetReturnValue().Set(ret);
}

void EXT_disjoint_timer_queryImpl::GetQueryObjectExt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto target = args[0];
    auto pname = args[1];
    auto pnameValue = pname->Uint32Value(context).ToChecked();
    auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
            target->Uint32Value(context).ToChecked(),
            pnameValue,
            *ptr->query_
    );
    if (pnameValue == GL_QUERY_RESULT_AVAILABLE_EXT) {
        args.GetReturnValue().Set(
                canvas_native_webgl_result_get_bool(*ret)
        );
        return;
    }

    args.GetReturnValue().Set(
            canvas_native_webgl_result_get_i32(*ret)
    );
}
