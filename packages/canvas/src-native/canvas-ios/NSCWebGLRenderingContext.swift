//
//  NSCWebGLRenderingContext.swift
//  CanvasNative
//
//  Created by Osei Fortune on 05/04/2024.
//

import Foundation

@objc(NSCWebGLRenderingContext)
@objcMembers
public class NSCWebGLRenderingContext: NSObject {
    
        public static func texImage2D(
            _ context: Int64,
            _ target: Int32,
            _ level: Int32,
            _ internalformat: Int32,
            _ format: Int32,
            _ type: Int32,
            _ data: UnsafeMutablePointer<UInt8>,
            _ size: UInt,
            _ dimensions: CGSize,
            _ flipY: Bool
        ) {
            canvas_native_ios_webgl_tex_image_2d(context, target, level, internalformat, format, type, data, size, Float(dimensions.width), Float(dimensions.height), flipY)
          }


        public static func texSubImage2D(
            _ context: Int64,
            _ target: Int32,
            _ level: Int32,
            _ xoffset: Int32,
            _ yoffset: Int32,
            _ format: Int32,
            _ type: Int32,
            _ data: UnsafeMutablePointer<UInt8>,
            _ size: UInt,
            _ dimensions: CGSize,
            _ flipY: Bool
        ) {
            canvas_native_ios_webgl_tex_sub_image_2d(context, target, level, xoffset, yoffset, format, type, data, size, Float(dimensions.width), Float(dimensions.height), flipY)
        }
    

}
