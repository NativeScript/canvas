#pragma once

#include "SVGCommon.h"
#include "SVGDocumentImpl.h"
#include "SVGNodeImpl.h"

class SVGJSIModule {
public:
    static void install(v8::Isolate *isolate);
};
