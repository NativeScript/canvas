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
@end

#endif /* CanvasSVGHelper_h */
