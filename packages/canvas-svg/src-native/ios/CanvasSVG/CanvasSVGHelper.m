//
//  CanvasSVGHelper.m
//  CanvasSVG
//
//  Created by Osei Fortune on 18/03/2024.
//

#import <Foundation/Foundation.h>
// The generated header gates the GPU surface behind the backends the Rust crate was built
// with; canvas-svg-ios enables `metal`.
#define CANVAS_SVG_METAL 1
#import "CanvasSVGHelper.h"

// visionOS has no UIScreen; derive the display scale from the current trait collection
// (scene-derived) instead. iOS keeps using UIScreen.mainScreen.nativeScale.
static CGFloat nscSVGNativeScale(void) {
#if TARGET_OS_VISION
    return UITraitCollection.currentTraitCollection.displayScale;
#else
    return UIScreen.mainScreen.nativeScale;
#endif
}

@implementation CanvasSVGHelper

+ (void)drawFromString:(uint8_t *)data size:(uintptr_t)size width:(float)width height:(float)height svg:(NSString*)svg {
    canvas_native_svg_draw_from_string(data, size, width, height, nscSVGNativeScale(), [svg UTF8String]);
}

+ (void)drawFromPath:(uint8_t *)data size:(uintptr_t)size width:(float)width height:(float)height path:(NSString*)path {
    canvas_native_svg_draw_from_path(data, size, width, height, nscSVGNativeScale(), [path UTF8String]);
}

+ (void)renderDocument:(int64_t)document
                  data:(uint8_t *)data
                  size:(uintptr_t)size
                 width:(int32_t)width
                height:(int32_t)height
              rowBytes:(uintptr_t)rowBytes
                 scale:(float)scale {
    canvas_native_svg_document_render_to_pixels((struct SvgDocument *)document, data, size, width, height, rowBytes, scale);
}

+ (int64_t)gpuCreate:(void *)view width:(int32_t)width height:(int32_t)height backend:(int32_t)backend {
    return (int64_t)canvas_native_svg_gpu_create(view, width, height, backend);
}

+ (int32_t)gpuBackend:(int64_t)gpu {
    return canvas_native_svg_gpu_backend((const struct SvgGpuSurface *)gpu);
}

+ (void)gpuResize:(int64_t)gpu width:(int32_t)width height:(int32_t)height {
    canvas_native_svg_gpu_resize((struct SvgGpuSurface *)gpu, width, height);
}

+ (int32_t)gpuRender:(int64_t)gpu document:(int64_t)document scale:(float)scale {
    return canvas_native_svg_gpu_render((struct SvgGpuSurface *)gpu, (struct SvgDocument *)document, scale);
}

+ (void *)gpuView:(int64_t)gpu {
    return canvas_native_svg_gpu_window((const struct SvgGpuSurface *)gpu);
}

+ (void)gpuDebugLoseContext:(int64_t)gpu {
    canvas_native_svg_gpu_debug_lose_context((struct SvgGpuSurface *)gpu);
}

+ (void)gpuDestroy:(int64_t)gpu {
    canvas_native_svg_gpu_destroy((struct SvgGpuSurface *)gpu);
}

+ (int64_t)renderThreadCreate:(void *)view width:(int32_t)width height:(int32_t)height backend:(int32_t)backend {
    return (int64_t)canvas_native_svg_render_thread_create(view, width, height, backend);
}

+ (BOOL)renderThreadCommit:(int64_t)handle document:(int64_t)document width:(int32_t)width height:(int32_t)height scale:(float)scale {
    return canvas_native_svg_render_thread_commit((struct RenderThread *)handle, (struct SvgDocument *)document, width, height, scale);
}

+ (void)renderThreadResize:(int64_t)handle width:(int32_t)width height:(int32_t)height {
    canvas_native_svg_render_thread_resize((struct RenderThread *)handle, width, height);
}

+ (int32_t)renderThreadStatus:(int64_t)handle {
    return canvas_native_svg_render_thread_status((struct RenderThread *)handle);
}

+ (void)renderThreadDestroy:(int64_t)handle {
    canvas_native_svg_render_thread_destroy((struct RenderThread *)handle);
}

@end
