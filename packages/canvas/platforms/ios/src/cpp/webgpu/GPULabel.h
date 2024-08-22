//
// Created by Osei Fortune on 19/08/2024.
//

#ifndef CANVAS_ANDROID_GPULABEL_H
#define CANVAS_ANDROID_GPULABEL_H

#include "Common.h"
#include "Helpers.h"

class GPULabel {
public:
    GPULabel(){}
    GPULabel(v8::Isolate *isolate, const v8::Local<v8::Value> &label) {
        if (label->IsString() || label->IsStringObject()) {
            label_ = ConvertFromV8String(isolate, label);
            hasLabel_ = true;
        }
    }

    const char *operator*() {
        if (hasLabel_) {
            return label_.c_str();
        }
        return nullptr;
    }

private:
    std::string label_;
    bool hasLabel_ = false;
};

#endif //CANVAS_ANDROID_GPULABEL_H
