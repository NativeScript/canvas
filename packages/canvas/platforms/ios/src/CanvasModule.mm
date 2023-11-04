#import "CanvasModule.h"
#import <NativeScript/runtime/Runtime.h>
#import "CanvasJSIModule.h"

using namespace std;

@implementation CanvasModule

- (void )install {
    v8::Isolate* isolate = tns::Runtime::GetCurrentRuntime()->GetIsolate();
    CanvasJSIModule::install(isolate);
}

@end
