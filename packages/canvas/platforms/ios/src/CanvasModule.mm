#import "CanvasModule.h"
#import "CanvasNative/CanvasNative-Swift.h"


using namespace facebook::jsi;
using namespace std;

@implementation CanvasModule

- (void )install {
    std::shared_ptr<facebook::jsi::Runtime> rt = [JSIRuntime runtime];
    install(*rt);
    CanvasJSIModule::install(*rt);
}

@end
