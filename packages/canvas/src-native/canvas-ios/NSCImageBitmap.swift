//
//  NSCImageBitmap.swift
//  CanvasNative
//
//  Created by Osei Fortune on 10/04/2021.
//

import Foundation
import UIKit
@objcMembers
@objc(NSCImageBitmap)
public class NSCImageBitmap: NSObject {
    static let FAILED_TO_LOAD = "Failed to load image"

    
    public static let queue = DispatchQueue(label: "NSCImageBitmapQueue", qos: .background, attributes:.concurrent, autoreleaseFrequency: .never, target: nil)

    
    /*
    
    public static func createFromUIImage(
        _ canvas: UIImage,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (NSCImageBitmap?, String?) -> Void
    ) {
        NSCImageBitmap.queue.async {
            let data = canvas.pngData()
            var result: Int64 = 0
            if data != nil {
                var bytes = [UInt8](data!)
                result = image_bitmap_create_from_bytes_encoded(&bytes, UInt(bytes.count), options.flipY,
                                                                options.premultiplyAlpha.rawValue,
                                                                options.colorSpaceConversion.rawValue,
                                                                options.resizeQuality.rawValue,
                                                                options.resizeWidth,
                                                                options.resizeHeight)
            }
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(NSCImageBitmap(result),nil)
                } else {
                    callback(nil,NSCImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromUIImage(
        _ canvas: UIImage,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (NSCImageBitmap?, String?) -> Void
    ) {
        NSCImageBitmap.queue.async {
            let data = canvas.pngData()
            var result: Int64 = 0
            if data != nil {
                var bytes = [UInt8](data!)
                result = image_bitmap_create_from_bytes_encoded_src_rect(&bytes, UInt(bytes.count),
                                                                         sx,
                                                                         sy,
                                                                         sWidth,
                                                                         sHeight,
                                                                         options.flipY,
                                                                         options.premultiplyAlpha.rawValue,
                                                                         options.colorSpaceConversion.rawValue,
                                                                         options.resizeQuality.rawValue,
                                                                         options.resizeWidth,
                                                                         options.resizeHeight)
            }
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(NSCImageBitmap(result),nil)
                } else {
                    callback(nil,NSCImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    */
    
    
}

