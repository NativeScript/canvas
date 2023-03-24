//
//  CanvasHelpers.m
//  CanvasNative
//
//  Created by Osei Fortune on 23/03/2023.
//

#import <UIKit/UIKit.h>
#import <Foundation/Foundation.h>
#import "../include/Helpers.h"
#import "../include/CanvasHelpers.h"

@implementation CanvasHelpers


+ (int64_t)initGLWithView:(int64_t)view width:(int32_t)width height:(int32_t)height alpha:(bool)alpha antialias:(bool)antialias depth:(bool)depth fail_if_major_performance_caveat:(bool)fail_if_major_performance_caveat power_preference:(NSString *)power_preference premultiplied_alpha:(bool)premultiplied_alpha preserve_drawing_buffer:(bool)preserve_drawing_buffer stencil:(bool)stencil desynchronized:(bool)desynchronized xr_compatible:(bool)xr_compatible version:(int32_t)version is_canvas:(bool)is_canvas {
    return canvas_helper_init_gl(view, width, height, alpha,antialias, depth, fail_if_major_performance_caveat, [power_preference UTF8String], premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible, version, is_canvas);
}

+ (void)releaseGLWithContext:(int64_t)context {
    canvas_helper_release_gl(context);
}

+ (int64_t)getGLPointerWithContext:(int64_t)context {
    return canvas_helper_get_gl_pointer(context);
}

+ (void)releaseGLPointerWithContext:(int64_t)context {
    canvas_helper_release_gl_pointer(context);
}

+ (void)updateGLSurfaceWithView:(int64_t)view width:(int32_t)width height:(int32_t)height context:(int64_t)context {
    canvas_helper_update_gl_surface(view, width, height, context);
}

+ (int64_t)create2DContext:(int64_t)context width:(int)width height:(int)height alpha:(bool)alpha density:(float)density samples:(int)samples font_color:(int)font_color ppi:(float)ppi direction:(int)direction {
    return canvas_helper_create_2d_context(context, width, height, alpha, density, samples, font_color, ppi, direction);
}

+ (void)test2D:(int64_t)context {
    return canvas_helper_context_2d_test(context);
}


+(BufferStruct)getBytesFromUIImage:(UIImage*)image {
    CGImageRef cgImage = image.CGImage;
    
    if(cgImage == nil && image.CIImage != nil){
        CIContext* context = [CIContext new];
        cgImage = [context createCGImage:image.CIImage fromRect:image.CIImage.extent];
    }
    
    if(cgImage != nil){
        size_t width = (size_t)image.size.width;
        size_t height = (size_t)image.size.height;
        size_t bytesPerRow = CGImageGetBytesPerRow(cgImage);
        size_t bytesPerPixel = bytesPerRow / width;
        size_t size = width * height * bytesPerPixel;
        void* buffer = malloc(size);
        CGColorSpaceRef colorSpace = CGColorSpaceCreateDeviceRGB();
        CGBitmapContextCreateWithData(buffer, width, height, 8, bytesPerRow, colorSpace, kCGImageAlphaPremultipliedLast | kCGBitmapByteOrder32Big, NULL, NULL);
        BufferStruct ret;
        ret.buffer = buffer;
        ret.size = size;
        return ret;
    }
    BufferStruct ret;
    ret.buffer = NULL;
    ret.size = 0;
    return ret;
}


+(bool)loadImageAssetWithContext:(int64_t)context image:(UIImage*) image {
    BufferStruct buffer = [CanvasHelpers getBytesFromUIImage:image];
    if(buffer.buffer == NULL){return false;}
    bool ret = canvas_helper_imageasset_load_from_bytes(context, buffer.buffer, buffer.size);
    free(buffer.buffer);
    buffer.buffer = NULL;
    return ret;
}

+(int64_t)createPattern:(int64_t)context image:(UIImage*) image repetition:(NSString*) repetition {
    BufferStruct buffer = [CanvasHelpers getBytesFromUIImage:image];
    if(buffer.buffer == NULL){return false;}
    
    float width = (float)image.size.width;
    float height = (float)image.size.height;
    
    bool ret = canvas_helper_context_create_pattern(context,width,height,buffer.buffer, buffer.size, [repetition UTF8String]);
    free(buffer.buffer);
    buffer.buffer = NULL;
    return ret;
}

+(bool) drawImageWithContext:(int64_t)context image:(UIImage*)image dx:(float)dx dy:(float)dy {
    BufferStruct buffer = [CanvasHelpers getBytesFromUIImage:image];
    if(buffer.buffer == NULL){return false;}
    
    float width = (float)image.size.width;
    float height = (float)image.size.height;
    
    bool ret = canvas_helper_context_draw_image_dx_dy_with_bytes(context,width,height,buffer.buffer, buffer.size, dx, dy);
    free(buffer.buffer);
    buffer.buffer = NULL;
    return ret;
    
}

+(bool) drawImageWithContext:(int64_t)context image:(UIImage*)image dx:(float)dx dy:(float)dy dw:(float)dw dh:(float)dh {
    BufferStruct buffer = [CanvasHelpers getBytesFromUIImage:image];
    if(buffer.buffer == NULL){return false;}
    
    float width = (float)image.size.width;
    float height = (float)image.size.height;
    
    bool ret = canvas_helper_content_draw_image_dx_dy_dw_dh_with_bytes(context,width,height,buffer.buffer, buffer.size, dx, dy,dw,dh);
    free(buffer.buffer);
    buffer.buffer = NULL;
    return ret;
    
}

+(bool) drawImageWithContext:(int64_t)context image:(UIImage*)image
                          sx:(float)sx sy:(float)sy sw:(float)sw sh:(float)sh
                          dx:(float)dx dy:(float)dy dw:(float)dw dh:(float)dh {
    BufferStruct buffer = [CanvasHelpers getBytesFromUIImage:image];
    if(buffer.buffer == NULL){return false;}
    
    float width = (float)image.size.width;
    float height = (float)image.size.height;
    
    bool ret = canvas_helper_context_draw_image_with_bytes(context,width,height,buffer.buffer, buffer.size,sx, sy,sw,sh ,dx, dy,dw,dh);
    free(buffer.buffer);
    buffer.buffer = NULL;
    return ret;
    
}

@end
