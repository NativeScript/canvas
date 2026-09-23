//
//  CanvasSVGHelper.h
//  CanvasSVG
//
//  Created by Osei Fortune on 18/03/2024.
//

#ifndef CanvasSVGHelper_h
#define CanvasSVGHelper_h
#import <UIKit/UIKit.h>
#import <CanvasSVG/canvas_svg.h>

@interface CanvasSVGHelper : NSObject
+(void) drawFromString:(uint8_t*)data size:(uintptr_t)size width:(float) width height:(float)height svg:(NSString*) svg;
+(void) drawFromPath:(uint8_t*)data size:(uintptr_t)size width:(float) width height:(float)height path:(NSString*) path;

/// Renders a live document straight into `data`, so no frame is staged and copied back.
/// `document` is the pointer JS holds; `rowBytes` is the destination stride.
+(void) renderDocument:(int64_t)document
                  data:(uint8_t*)data
                  size:(uintptr_t)size
                 width:(int32_t)width
                height:(int32_t)height
              rowBytes:(uintptr_t)rowBytes
                 scale:(float)scale;

/// Returns 0 when no GPU surface could be made -- the caller's cue to fall back to the raster
/// path. `backend` is a Backend discriminant; 0 auto-selects.
+(int64_t) gpuCreate:(void*)view width:(int32_t)width height:(int32_t)height backend:(int32_t)backend;
+(int32_t) gpuBackend:(int64_t)gpu;
+(void) gpuResize:(int64_t)gpu width:(int32_t)width height:(int32_t)height;
/// Returns a FrameStatus: 0 presented, 1 skipped, 2 presented after the context was rebuilt,
/// 3 lost -- the caller's cue to fall back to the raster path.
+(int32_t) gpuRender:(int64_t)gpu document:(int64_t)document scale:(float)scale;
/// The view pointer the surface was created with, so the caller can give back the reference it
/// handed over. Must be read before `gpuDestroy`, which frees the surface holding it.
+(void*) gpuView:(int64_t)gpu;
/// Simulates a context loss so the recovery path can be exercised. For testing.
+(void) gpuDebugLoseContext:(int64_t)gpu;
+(void) gpuDestroy:(int64_t)gpu;

/// Starts a render thread that owns its own GPU surface. Returns 0 when one could not be
/// started. Does not wait for the surface to be built -- whether the GPU came up is reported
/// through `renderThreadStatus`, because blocking the caller here deadlocks the UI thread.
+(int64_t) renderThreadCreate:(void*)view width:(int32_t)width height:(int32_t)height backend:(int32_t)backend;
/// Records the document on the *calling* thread and hands the display list over; the
/// rasterizing happens on the render thread.
+(BOOL) renderThreadCommit:(int64_t)handle document:(int64_t)document width:(int32_t)width height:(int32_t)height scale:(float)scale;
+(void) renderThreadResize:(int64_t)handle width:(int32_t)width height:(int32_t)height;
/// The last present's FrameStatus, or -1 if nothing has presented since the last call.
+(int32_t) renderThreadStatus:(int64_t)handle;
/// Stops the thread and tears its surface down; blocks until it has joined.
+(void) renderThreadDestroy:(int64_t)handle;
@end

#endif /* CanvasSVGHelper_h */
