//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"
#include "webgl2/WebGLQuery.h"

using namespace facebook;

class JSI_EXPORT EXT_disjoint_timer_queryImpl : public jsi::HostObject {
public:
    EXT_disjoint_timer_queryImpl(rust::Box<EXT_disjoint_timer_query> query);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    EXT_disjoint_timer_query &GetQuery();

private:
    rust::Box<EXT_disjoint_timer_query> query_;
};
