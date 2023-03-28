#import "CanvasModule.h"


using namespace facebook::jsi;
using namespace std;

@implementation CanvasModule

- (void )install {
    std::shared_ptr<facebook::jsi::Runtime> rt = [JSIRuntime runtime];
    CanvasJSIModule::install(*rt);
}

@end
