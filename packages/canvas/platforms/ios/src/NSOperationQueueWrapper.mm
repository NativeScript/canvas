#import "NSOperationQueueWrapper.h"
#include <Foundation/Foundation.h>

@interface NSOperationQueueWrapperObjC : NSObject
- (void)addOperation:(void (^)())task;
@end

@implementation NSOperationQueueWrapperObjC {
    NSOperationQueue* operationQueue;
    __CFRunLoop* current;
    BOOL isSerial;
}

- (instancetype)init {
    self = [super init];
    if (self) {
        isSerial = false;
        operationQueue = NULL;
        current = NULL;
    }
    return self;
}

-(instancetype)initWithCurrentQueue {
    self = [super init];
    if (self) {
        current = CFRunLoopGetCurrent();
    }
    return self;
}

-(instancetype)initWithSerial:(BOOL)serial {
    self = [super init];
    if (self) {
        isSerial = serial;
    }
    return self;
}

- (void)dealloc {}

- (void)addOperation:(void (^)())task {
    
    if(current != NULL){
        if(current == CFRunLoopGetCurrent()){
            task();
        }else {
            CFRunLoopPerformBlock(current, kCFRunLoopDefaultMode, ^{
                task();
            });
            CFRunLoopWakeUp(current);
        }
    }else if(operationQueue != NULL){
        NSBlockOperation* operation = [NSBlockOperation blockOperationWithBlock:^{
            task();
        }];
        
        [operationQueue addOperation:operation];
    }else if(isSerial){
        dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_SERIAL, 0), ^{
              task();
          });
        
    }else {
        dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
              task();
          });
    }
}

- (NSOperationQueue*)getOperationQueue {
    return operationQueue;
}


@end

NSOperationQueueWrapper::NSOperationQueueWrapper(bool currentQueue, bool serial) {
    if (currentQueue) {
        NSOperationQueueWrapperObjC* objcWrapper = [[NSOperationQueueWrapperObjC alloc] initWithCurrentQueue];
        
        CFTypeRef ptr = (__bridge_retained CFTypeRef)objcWrapper;
    
        operationQueue = (void*)ptr;
    }else {
        NSOperationQueueWrapperObjC* objcWrapper = [[NSOperationQueueWrapperObjC alloc] initWithSerial: serial];
        CFTypeRef ptr = (__bridge_retained CFTypeRef)objcWrapper;
        operationQueue = (void*)ptr;
    }
}

NSOperationQueueWrapper::~NSOperationQueueWrapper() {
    if(operationQueue != nullptr){
        CFBridgingRelease(operationQueue);
        operationQueue = nullptr;
    }
}

void NSOperationQueueWrapper::addOperation(std::function<void()> task) {
    
    auto queue = (__bridge NSOperationQueueWrapperObjC*)(operationQueue);
    
    [queue addOperation:^{
        task();
    }];
}
