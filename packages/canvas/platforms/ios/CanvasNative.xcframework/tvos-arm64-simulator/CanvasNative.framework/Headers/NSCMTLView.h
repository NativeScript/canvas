//
//  NSCMTLView.h
//  CanvasNative
//
//  Created by Osei Fortune on 08/10/2024.
//
#ifndef NSCMTLView_h
#define NSCMTLView_h

#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>
#import <MetalKit/MetalKit.h>
typedef enum {
    kNone,
    kPending,
    kInvalidating
} NSCSurfaceState;

@interface NSCMTLView : UIView
@property(weak, nonatomic) UIView* canvas;
@property id<MTLCommandQueue> queue;
@property int state;
@property (nonatomic) CGSize drawableSize;
@property NSUInteger sampleCount;
-(void) present;
-(void) setup;
-(void*) getDevicePtr;
-(void*) getQueuePtr;
@end


#endif /* NSCMTLView_h */
