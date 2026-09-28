#include "SVGJSIModule.h"
#include "SVGHelpers.h"

void SVGJSIModule::install(v8::Isolate *isolate) {
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();

    if (global->HasOwnProperty(context, ConvertToV8String(isolate, "SVGModule")).FromMaybe(false)) {
        return;
    }

    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto svgModule = v8::Object::New(isolate);

    SVGDocumentImpl::Init(svgModule, isolate);
    SVGNodeImpl::Init(svgModule, isolate);

    svgModule->Set(context, ConvertToV8String(isolate, "createElement"),
                    v8::FunctionTemplate::New(isolate, &SVGNodeImpl::CreateElement)->GetFunction(
                            context).ToLocalChecked()).FromJust();
    svgModule->Set(context, ConvertToV8String(isolate, "createTextNode"),
                    v8::FunctionTemplate::New(isolate, &SVGNodeImpl::CreateTextNode)->GetFunction(
                            context).ToLocalChecked()).FromJust();

    global->Set(context, ConvertToV8String(isolate, "SVGModule"), svgModule).FromJust();
}
