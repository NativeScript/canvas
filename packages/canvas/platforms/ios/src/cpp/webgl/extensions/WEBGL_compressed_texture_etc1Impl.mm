//
// Created by Osei Fortune on 29/04/2022.
//

#import "WEBGL_compressed_texture_etc1Impl.h"

jsi::Value
WEBGL_compressed_texture_etc1Impl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "WEBGL_compressed_texture_etc1");
    }


    if (methodName == "COMPRESSED_RGB_ETC1_WEBGL") {
        return {GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG};
    }

    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> WEBGL_compressed_texture_etc1Impl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB_ETC1_WEBGL")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));

    return ret;
}