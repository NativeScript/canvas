#import "CanvasSVGModule.h"
#import <NativeScript/runtime/Runtime.h>
#import "cpp/SVGJSIModule.h"

@implementation CanvasSVGModule

- (void)install {
    v8::Isolate* isolate = tns::Runtime::GetCurrentRuntime()->GetIsolate();
    SVGJSIModule::install(isolate);
}

@end
