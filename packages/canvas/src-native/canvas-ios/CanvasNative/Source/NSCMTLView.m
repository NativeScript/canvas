//
//  NSCMTLView.m
//  CanvasNative
//
//  Created by Osei Fortune on 08/10/2024.
//
#import "NSCMTLView.h"

@class NSCCanvas;

@interface NSCCanvas : UIView
@property BOOL is2D;
@property int64_t nativeContext;
@end


@class NSSCanvasHelpers;
@interface NSSCanvasHelpers : NSObject
+(void) flush2DContext:(int64_t)context;
+(void) flush2DContextAndSyncCPU:(int64_t)context;
+(void) presentDrawable:(int64_t)context;
@end



@implementation NSCMTLView

- (NSCCanvas *)canvasView {
  return (NSCCanvas *)self.canvas;
}

+ (Class) layerClass
{
    return [CAMetalLayer class];
}


- (void)setDrawableSize:(CGSize)newDrawableSize {
    CAMetalLayer* layer = self.layer;
    layer.drawableSize = newDrawableSize;
}

- (CGSize)drawableSize {
    CAMetalLayer* layer = self.layer;
    return layer.drawableSize;
}

- (instancetype)initWithFrame:(CGRect)frame {
    self = [super initWithFrame:frame];
       if (self) {
           _sampleCount = 1;
           [self setup];
       }
       return self;
}


-(void) setup {
    CAMetalLayer* layer = self.layer;
    [layer setDevice: MTLCreateSystemDefaultDevice()];
    _queue = [[layer device] newCommandQueue];
    layer.presentsWithTransaction = NO;
    layer.framebufferOnly = NO;
    layer.pixelFormat = MTLPixelFormatBGRA8Unorm;
    
}

-(void) present {
    if(self.canvasView != nil && self.canvasView.is2D){
        [NSSCanvasHelpers flush2DContext: self.canvasView.nativeContext];
        [NSSCanvasHelpers presentDrawable: self.canvasView.nativeContext];
    }
}


-(void*) getDevicePtr{
    CAMetalLayer* layer = self.layer;
    if([layer device] == nil){
        [layer setDevice: MTLCreateSystemDefaultDevice()];
    }
    
    return (__bridge void *) [layer device];
}


-(void*) getQueuePtr{
    CAMetalLayer* layer = self.layer;
    if(_queue == nil && [layer device] != nil){
        _queue = [[layer device] newCommandQueue];
    }
    
    return (__bridge void *) _queue;
}

@end
