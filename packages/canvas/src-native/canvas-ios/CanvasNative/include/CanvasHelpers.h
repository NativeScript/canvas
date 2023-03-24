//
//  CanvasHelpers.h
//  CanvasNative
//
//  Created by Osei Fortune on 23/03/2023.
//

#ifndef CanvasHelpers_h
#define CanvasHelpers_h
#import <UIKit/UIKit.h>
#import <Foundation/Foundation.h>

typedef struct BufferStruct {
    void* buffer;
    size_t size;
} BufferStruct;

@interface CanvasHelpers: NSObject
+(int64_t)initGLWithView:(int64_t)view
                   width:(int32_t)width
                  height:(int32_t)height
                   alpha:(bool)alpha
               antialias:(bool)antialias
                   depth:(bool)depth fail_if_major_performance_caveat:(bool)fail_if_major_performance_caveat
        power_preference:(NSString*)power_preference
     premultiplied_alpha:(bool) premultiplied_alpha
 preserve_drawing_buffer:(bool) preserve_drawing_buffer
                 stencil:(bool)stencil
          desynchronized:(bool)desynchronized
           xr_compatible:(bool)xr_compatible
                 version:(int32_t)version
               is_canvas:(bool) is_canvas;

+(void)releaseGLWithContext:(int64_t)context;

+(int64_t)getGLPointerWithContext:(int64_t)context;

+(void)releaseGLPointerWithContext:(int64_t)context;


+(int64_t)create2DContext:(int64_t)context
                    width:(int)width
                   height:(int)height
                    alpha:(bool)alpha
                  density:(float)density
                  samples:(int)samples
               font_color:(int)font_color
                      ppi:(float)ppi
                direction:(int)direction;


+(void)updateGLSurfaceWithView:(int64_t)view
                         width:(int32_t)width
                        height:(int32_t)height
                       context:(int64_t)context;

+(void)test2D:(int64_t)context;


+(bool)loadImageAssetWithContext:(int64_t)context image:(UIImage*) image;

+(int64_t)createPattern:(int64_t)context image:(UIImage*)image repetition:(NSString*) repetition;

+(bool) drawImageWithContext:(int64_t)context image:(UIImage*)image dx:(float)dx dy:(float)dy;

+(bool) drawImageWithContext:(int64_t)context image:(UIImage*)image dx:(float)dx dy:(float)dy dw:(float)dw dh:(float)dh;

+(bool) drawImageWithContext:(int64_t)context image:(UIImage*)image
                          sx:(float)sx sy:(float)sy sw:(float)sw sh:(float)sh
                          dx:(float)dx dy:(float)dy dw:(float)dw dh:(float)dh;

@end
#endif /* CanvasHelpers_h */
