//
// Created by Osei Fortune on 03/04/2022.
//

#include "MatrixImpl.h"


MatrixImpl::MatrixImpl(rust::Box<Matrix> matrix) : matrix_(std::move(matrix)) {}

std::vector<jsi::PropNameID> MatrixImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(23);

    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("a")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("b")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("c")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("d")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("e")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("f")));

    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m11")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m12")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m13")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m14")));

    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m21")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m22")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m23")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m24")));

    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m31")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m32")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m33")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m34")));

    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m41")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m42")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m43")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("m44")));


    return ret;
}

jsi::Value MatrixImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "a") {
        return {(double) canvas_native_matrix_get_a(this->GetMatrix())};
    } else if (methodName == "b") {
        return {(double) canvas_native_matrix_get_b(this->GetMatrix())};
    } else if (methodName == "c") {
        return {(double) canvas_native_matrix_get_c(this->GetMatrix())};
    } else if (methodName == "d") {
        return {(double) canvas_native_matrix_get_d(this->GetMatrix())};
    } else if (methodName == "e") {
        return {(double) canvas_native_matrix_get_e(this->GetMatrix())};
    } else if (methodName == "f") {
        return {(double) canvas_native_matrix_get_f(this->GetMatrix())};
    } else if (methodName == "m11") {
        return {(double) canvas_native_matrix_get_m11(this->GetMatrix())};
    } else if (methodName == "m12") {
        return {(double) canvas_native_matrix_get_m12(this->GetMatrix())};
    } else if (methodName == "m13") {
        return {(double) canvas_native_matrix_get_m13(this->GetMatrix())};
    } else if (methodName == "m14") {
        return {(double) canvas_native_matrix_get_m14(this->GetMatrix())};
    } else if (methodName == "m21") {
        return {(double) canvas_native_matrix_get_m21(this->GetMatrix())};
    } else if (methodName == "m22") {
        return {(double) canvas_native_matrix_get_m22(this->GetMatrix())};
    } else if (methodName == "m23") {
        return {(double) canvas_native_matrix_get_m23(this->GetMatrix())};
    } else if (methodName == "m24") {
        return {(double) canvas_native_matrix_get_m24(this->GetMatrix())};
    } else if (methodName == "m31") {
        return {(double) canvas_native_matrix_get_m31(this->GetMatrix())};
    } else if (methodName == "m32") {
        return {(double) canvas_native_matrix_get_m32(this->GetMatrix())};
    } else if (methodName == "m33") {
        return {(double) canvas_native_matrix_get_m33(this->GetMatrix())};
    } else if (methodName == "m34") {
        return {(double) canvas_native_matrix_get_m34(this->GetMatrix())};
    } else if (methodName == "m41") {
        return {(double) canvas_native_matrix_get_m41(this->GetMatrix())};
    } else if (methodName == "m42") {
        return {(double) canvas_native_matrix_get_m42(this->GetMatrix())};
    } else if (methodName == "m43") {
        return {(double) canvas_native_matrix_get_m43(this->GetMatrix())};
    } else if (methodName == "m44") {
        return {(double) canvas_native_matrix_get_m44(this->GetMatrix())};
    }

    return jsi::Value::undefined();
}

void MatrixImpl::set(jsi::Runtime &runtime, const jsi::PropNameID &name, const jsi::Value &value) {
    auto methodName = name.utf8(runtime);
    if (!value.isNumber()) { return; }
    auto val = (float) value.asNumber();
    if (methodName == "a") {
        canvas_native_matrix_set_a(this->GetMatrix(), val);
    } else if (methodName == "b") {
        canvas_native_matrix_set_b(this->GetMatrix(), val);
    } else if (methodName == "c") {
        canvas_native_matrix_set_c(this->GetMatrix(), val);
    } else if (methodName == "d") {
        canvas_native_matrix_set_d(this->GetMatrix(), val);
    } else if (methodName == "e") {
        canvas_native_matrix_set_e(this->GetMatrix(), val);
    } else if (methodName == "f") {
        canvas_native_matrix_set_f(this->GetMatrix(), val);
    } else if (methodName == "m11") {
        canvas_native_matrix_set_m11(this->GetMatrix(), val);
    } else if (methodName == "m12") {
        canvas_native_matrix_set_m12(this->GetMatrix(), val);
    } else if (methodName == "m13") {
        canvas_native_matrix_set_m13(this->GetMatrix(), val);
    } else if (methodName == "m14") {
        canvas_native_matrix_set_m14(this->GetMatrix(), val);
    } else if (methodName == "m21") {
        canvas_native_matrix_set_m21(this->GetMatrix(), val);
    } else if (methodName == "m22") {
        canvas_native_matrix_set_m22(this->GetMatrix(), val);
    } else if (methodName == "m23") {
        canvas_native_matrix_set_m23(this->GetMatrix(), val);
    } else if (methodName == "m24") {
        canvas_native_matrix_set_m24(this->GetMatrix(), val);
    } else if (methodName == "m31") {
        canvas_native_matrix_set_m31(this->GetMatrix(), val);
    } else if (methodName == "m32") {
        canvas_native_matrix_set_m32(this->GetMatrix(), val);
    } else if (methodName == "m33") {
        canvas_native_matrix_set_m33(this->GetMatrix(), val);
    } else if (methodName == "m34") {
        canvas_native_matrix_set_m34(this->GetMatrix(), val);
    } else if (methodName == "m41") {
        canvas_native_matrix_set_m41(this->GetMatrix(), val);
    } else if (methodName == "m42") {
        canvas_native_matrix_set_m42(this->GetMatrix(), val);
    } else if (methodName == "m43") {
        canvas_native_matrix_set_m43(this->GetMatrix(), val);
    } else if (methodName == "m44") {
        canvas_native_matrix_set_m44(this->GetMatrix(), val);
    }
}

Matrix &MatrixImpl::GetMatrix() {
    return *this->matrix_;
}
