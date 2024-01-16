//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_disjoint_timer_queryImpl.h"
#include "Caches.h"

EXT_disjoint_timer_queryImpl::EXT_disjoint_timer_queryImpl(
        EXT_disjoint_timer_query *query) : query_(query) {}


v8::Local<v8::FunctionTemplate> EXT_disjoint_timer_queryImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_disjoint_timer_queryTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "EXT_disjoint_timer_query"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(ConvertToV8String(isolate, "QUERY_COUNTER_BITS_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8864));

    tmpl->Set(ConvertToV8String(isolate, "CURRENT_QUERY_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8865));

    tmpl->Set(ConvertToV8String(isolate, "QUERY_RESULT_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8866));

    tmpl->Set(ConvertToV8String(isolate, "QUERY_RESULT_AVAILABLE_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8867));

    tmpl->Set(ConvertToV8String(isolate, "TIME_ELAPSED_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x88BF));

    tmpl->Set(ConvertToV8String(isolate, "TIMESTAMP_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8E28));

    tmpl->Set(ConvertToV8String(isolate, "GPU_DISJOINT_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8FBB));


    tmpl->Set(ConvertToV8String(isolate, "createQueryExt"),
              v8::FunctionTemplate::New(isolate, &CreateQueryExt));
    tmpl->Set(ConvertToV8String(isolate, "deleteQueryExt"),
              v8::FunctionTemplate::New(isolate, &DeleteQueryExt));
    tmpl->Set(ConvertToV8String(isolate, "isQueryExt"),
              v8::FunctionTemplate::New(isolate, &IsQueryExt));
    tmpl->Set(ConvertToV8String(isolate, "beginQueryExt"),
              v8::FunctionTemplate::New(isolate, &BeginQueryExt));
    tmpl->Set(ConvertToV8String(isolate, "endQueryExt"),
              v8::FunctionTemplate::New(isolate, &EndQueryExt));
    tmpl->Set(ConvertToV8String(isolate, "queryCounterExt"),
              v8::FunctionTemplate::New(isolate, &QueryCounterExt));
    tmpl->Set(ConvertToV8String(isolate, "getQueryExt"),
              v8::FunctionTemplate::New(isolate, &GetQueryExt));
    tmpl->Set(ConvertToV8String(isolate, "getQueryObjectExt"),
              v8::FunctionTemplate::New(isolate, &GetQueryObjectExt));
    tmpl->Set(ConvertToV8String(isolate, "getQueryParameterExt"),
              v8::FunctionTemplate::New(isolate, &GetQueryParameterExt));

    tmpl->Set(ConvertToV8String(isolate, "ext_name"),
              ConvertToV8String(isolate, "EXT_disjoint_timer_query"));

    cache->EXT_disjoint_timer_queryTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

EXT_disjoint_timer_queryImpl *
EXT_disjoint_timer_queryImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<EXT_disjoint_timer_queryImpl *>(ptr);
}

void EXT_disjoint_timer_queryImpl::CreateQueryExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();

    auto ret = canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
            ptr->GetQuery());

    auto object = new WebGLQuery(
            ret);

    auto value = WebGLQuery::NewInstance(isolate, object);
    args.GetReturnValue().Set(value);
}

void EXT_disjoint_timer_queryImpl::DeleteQueryExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto value = args[0];
    auto type = GetNativeType( value);

    if (type == NativeType::WebGLQuery) {
        auto query = WebGLQuery::GetPointer(value.As<v8::Object>());
        if (query ==
            nullptr) { return; }
        canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
                query->GetQuery(),
                ptr->GetQuery());
    }

}

void EXT_disjoint_timer_queryImpl::IsQueryExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto value = args[0];
    auto type = GetNativeType( value);

    if (type == NativeType::WebGLQuery) {
        auto query = WebGLQuery::GetPointer(value.As<v8::Object>());
        if (query != nullptr) {
            auto ret = canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
                    query->GetQuery(),
                    ptr->GetQuery());

            args.GetReturnValue().Set(ret);
        }
    }
    args.GetReturnValue().Set(false);
}


void EXT_disjoint_timer_queryImpl::BeginQueryExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto targetValue = args[0];
    auto queryValue = args[1];

    auto target = targetValue->Uint32Value(context).ToChecked();
    auto type = GetNativeType( queryValue);
    if (type == NativeType::WebGLQuery) {
        auto query = WebGLQuery::GetPointer(queryValue.As<v8::Object>());

        canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
                target,
                query->GetQuery(),
                ptr->GetQuery()
        );
    }
}

void EXT_disjoint_timer_queryImpl::EndQueryExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    auto target = value->Uint32Value(context).ToChecked();
    canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
            target,
            ptr->GetQuery()
    );
}

void EXT_disjoint_timer_queryImpl::QueryCounterExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto queryValue = args[0];
    auto targetValue = args[1];

    auto target = targetValue->Uint32Value(context).ToChecked();
    auto type = GetNativeType( queryValue.As<v8::Object>());
    if (type == NativeType::WebGLQuery) {
        auto query = WebGLQuery::GetPointer(queryValue.As<v8::Object>());

        canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
                query->GetQuery(),
                target,
                ptr->GetQuery()
        );
    }
}

void EXT_disjoint_timer_queryImpl::GetQueryExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto target = args[0]->Uint32Value(context).ToChecked();
    auto pname = args[1]->Uint32Value(context).ToChecked();
    auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
            target,
            pname,
            ptr->GetQuery()
    );

    args.GetReturnValue().Set(ret);

}

void EXT_disjoint_timer_queryImpl::GetQueryObjectExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto query = WebGLQuery::GetPointer(args[0].As<v8::Object>());
    auto pname = args[1]->Uint32Value(context).ToChecked();

    auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
            query->GetQuery(),
            pname,
            ptr->GetQuery()
    );


    // GL_QUERY_RESULT_AVAILABLE_EXT
    if (pname == 0x8867) {
        args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                ret));
        canvas_native_webgl_WebGLResult_destroy(ret);
        return;
    }


    args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
            ret));

    canvas_native_webgl_WebGLResult_destroy(ret);
}


void EXT_disjoint_timer_queryImpl::GetQueryParameterExt(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    EXT_disjoint_timer_queryImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto query = WebGLQuery::GetPointer(args[0].As<v8::Object>());
    auto pname = args[1]->Uint32Value(context).ToChecked();

    auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
            query->GetQuery(),
            pname,
            ptr->GetQuery()
    );

    // GL_QUERY_RESULT_AVAILABLE_EXT
    if (pname == 0x8867) {
        args.GetReturnValue().Set(canvas_native_webgl_result_get_bool(
                ret));
        canvas_native_webgl_WebGLResult_destroy(ret);
        return;
    }

    args.GetReturnValue().Set(canvas_native_webgl_result_get_i32(
            ret));
    canvas_native_webgl_WebGLResult_destroy(ret);
}


EXT_disjoint_timer_query *EXT_disjoint_timer_queryImpl::GetQuery() {
    return this->query_;
}
