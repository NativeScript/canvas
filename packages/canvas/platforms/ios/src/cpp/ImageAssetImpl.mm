//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageAssetImpl.h"
#include <string>
#include "JSICallback.h"



ImageAssetImpl::ImageAssetImpl(rust::Box<ImageAsset> asset)
        : asset_(std::move(asset)) {
}

std::vector<jsi::PropNameID> ImageAssetImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(12);
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("width")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("height")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("error")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("scale")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fromUrlSync")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fromUrlCb")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fromFileSync")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fromFileCb")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fromBytesSync")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fromBytesCb")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("saveSync")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("saveCb")));
    return ret;
}

jsi::Value ImageAssetImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "width") {
        return {(double) canvas_native_image_asset_width(this->GetImageAsset())};
    } else if (methodName == "height") {
        return {(double) canvas_native_image_asset_height(this->GetImageAsset())};
    } else if (methodName == "error") {
        auto error = canvas_native_image_asset_get_error(this->GetImageAsset());
        return jsi::String::createFromAscii(runtime, error.data(), error.size());
    } else if (methodName == "scale") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count > 1) {
                                                             auto x = (uint32_t) arguments[0].asNumber();
                                                             auto y = (uint32_t) arguments[1].asNumber();
                                                             if (x > 0 && y > 0) {
                                                                 canvas_native_image_asset_scale(
                                                                         this->GetImageAsset(), x,
                                                                         y);
                                                             }
                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "fromUrlSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto url = arguments[0].asString(
                                                                 runtime).utf8(runtime);
                                                         auto done = canvas_native_image_asset_load_from_url(
                                                                 this->GetImageAsset(),
                                                                 rust::Str(url.c_str()));
                                                         return {done};
                                                     }
        );
    } else if (methodName == "fromUrlCb") {
        auto current_queue = NSOperationQueue.currentQueue;
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this, current_queue](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count < 2) {
                                                             return jsi::Value::undefined();
                                                         }


                                                         auto url = arguments[0].asString(
                                                                 runtime).utf8(runtime);

                                                         auto asset = canvas_native_image_asset_shared_clone(
                                                                 this->GetImageAsset());

                                                         auto cb = std::make_shared<jsi::Value>(
                                                                 runtime, arguments[1]);


                                                         auto jsi_callback = new JSICallback(
                                                                 std::shared_ptr<jsi::Value>(
                                                                         cb));
            
        
            auto queue = [NSOperationQueue new];
            [queue addOperationWithBlock:^{
             rust::Str asset_url(url.c_str());

             auto asset = canvas_native_image_asset_shared_clone(
                     this->GetImageAsset());
             
             auto done = canvas_native_image_asset_load_from_url(
                                                                  this->GetImageAsset(), asset_url
                     );
             [current_queue addOperationWithBlock:^{
              

        
              
                                              auto func = jsi_callback->value_->asObject(
                                                      runtime).asFunction(
                                                                          runtime);
              
              

                                              func.call(runtime,
                                                        jsi::Value(
                                                                done));

                                              delete static_cast<JSICallback *>(jsi_callback);
              }];
             }];
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "fromFileSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto path = arguments[0].asString(
                                                                 runtime).utf8(runtime);

                                                         auto done = canvas_native_image_asset_load_from_path(
                                                                 this->GetImageAsset(),
                                                                 rust::Str(path.c_str()));

                                                         return {done};
                                                     }
        );
    } else if (methodName == "fromFileCb") {
        auto current_queue = NSOperationQueue.currentQueue;
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this, current_queue](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto path = arguments[0].asString(
                                                                 runtime).utf8(runtime);

                                                        

                                                         auto cb = std::make_shared<jsi::Value>(
                                                                 runtime, arguments[1]);


                                                         auto jsi_callback = new JSICallback(
                                                                 std::shared_ptr<jsi::Value>(
                                                                         cb));
            
            

            
            auto queue = [NSOperationQueue new];
            [queue addOperationWithBlock:^{
             rust::Str asset_path(path.c_str());

             auto asset = canvas_native_image_asset_shared_clone(
                     this->GetImageAsset());
             
             auto done = canvas_native_image_asset_load_from_path(
                                                                  this->GetImageAsset(), asset_path
                     );
             [current_queue addOperationWithBlock:^{
              

        
              
                                              auto func = jsi_callback->value_->asObject(
                                                      runtime).asFunction(
                                                                          runtime);
              
              

                                              func.call(runtime,
                                                        jsi::Value(
                                                                done));

                                              delete static_cast<JSICallback *>(jsi_callback);
              }];
             }];
          
     
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "fromBytesSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (arguments[0].isObject()) {
                                                             auto object = arguments[0].asObject(
                                                                     runtime);
                                                             if (!object.isArrayBuffer(
                                                                     runtime)) { return {false}; }
                                                             auto buf = object.getArrayBuffer(
                                                                     runtime);

                                                             auto size = buf.size(runtime);
                                                             auto data = buf.data(runtime);
                                                             auto done = canvas_native_image_asset_load_from_raw(
                                                                     this->GetImageAsset(),
                                                                     rust::Slice<const uint8_t>(
                                                                             data, size));
                                                             return {done};
                                                         }
                                                         return {false};
                                                     }
        );
    } else if (methodName == "fromBytesCb") {
        auto current_queue = NSOperationQueue.currentQueue;
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this, current_queue](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto bytes = arguments[0].asObject(
                                                                 runtime).getArrayBuffer(runtime);

                                                         auto size = bytes.size(runtime);
                                                         auto data = bytes.data(runtime);
                                                         auto buf = rust::Slice<const uint8_t>(data,
                                                                                               size);

                                                         auto asset = canvas_native_image_asset_shared_clone(
                                                                 this->GetImageAsset());


                                                         auto cb = std::make_shared<jsi::Value>(
                                                                 runtime, arguments[1]);

                                                         auto jsi_callback = new JSICallback(
                                                                 std::shared_ptr<jsi::Value>(
                                                                         cb));
            
            
            auto queue = [NSOperationQueue new];
            [queue addOperationWithBlock:^{

             auto asset = canvas_native_image_asset_shared_clone(
                     this->GetImageAsset());
             
           
                
                
                auto done = canvas_native_image_asset_load_from_raw(
                                                                    this->GetImageAsset(), buf);
                
             [current_queue addOperationWithBlock:^{
              

        
              
                                              auto func = jsi_callback->value_->asObject(
                                                      runtime).asFunction(
                                                                          runtime);
              
              

                                              func.call(runtime,
                                                        jsi::Value(
                                                                done));

                                              delete static_cast<JSICallback *>(jsi_callback);
              }];
             }];

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "saveSync") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count < 2) {
                                                             return {false};
                                                         }

                                                         auto path = arguments[0].asString(
                                                                 runtime).utf8(runtime);
                                                         auto format = (uint32_t) arguments[1].asNumber();
                                                         auto done = canvas_native_image_asset_save_path(
                                                                 this->GetImageAsset(),
                                                                 rust::Str(path.c_str()), format);

                                                         return {done};
                                                     }
        );
    } else if (methodName == "saveCb") {
        auto current_queue = NSOperationQueue.currentQueue;
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this, current_queue](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto path = arguments[0].asString(
                                                                 runtime).utf8(runtime);

                                                         auto format = (uint32_t) arguments[1].asNumber();


                                                         auto asset = canvas_native_image_asset_shared_clone(
                                                                 this->GetImageAsset());


                                                         auto cb = std::make_shared<jsi::Value>(
                                                                 runtime, arguments[1]);

                                                         auto jsi_callback = new JSICallback(
                                                                 std::shared_ptr<jsi::Value>(
                                                                         cb));
            
            
            auto queue = [NSOperationQueue new];
            [queue addOperationWithBlock:^{
                
                rust::Str asset_path(
                                     path.c_str());
            

             auto asset = canvas_native_image_asset_shared_clone(
                     this->GetImageAsset());
             
                
                auto done = canvas_native_image_asset_save_path(
                                                                this->GetImageAsset(),
                        asset_path,
                        format);

           
                
             [current_queue addOperationWithBlock:^{
              

        
              
                                              auto func = jsi_callback->value_->asObject(
                                                      runtime).asFunction(
                                                                          runtime);
              
              

                                              func.call(runtime,
                                                        jsi::Value(
                                                                done));

                                              delete static_cast<JSICallback *>(jsi_callback);
              }];
             }];
            
                                                         return jsi::Value::undefined();
                                                     }
        );
    }

    return jsi::Value::undefined();

}

ImageAsset &ImageAssetImpl::GetImageAsset() {
    return *this->asset_;
}
