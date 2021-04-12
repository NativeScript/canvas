//
//  TNSImageBitmap.swift
//  CanvasNative
//
//  Created by Osei Fortune on 10/04/2021.
//

import Foundation
import UIKit
@objcMembers
@objc(TNSImageBitmap)
public class TNSImageBitmap: NSObject {
    static let FAILED_TO_LOAD = "Failed to load image"
    var asset: Int64
    var raw_data: UnsafeMutablePointer<U8Array>? = nil
    
    private static var _queue: DispatchQueue?
    public static var queue: DispatchQueue {
        if(_queue == nil){
            _queue = DispatchQueue(label: "TNSImageBitmapQueue", qos: .background, attributes:.concurrent, autoreleaseFrequency: .never, target: nil)
        }
        return _queue!
    }
    
    private var _error: String?
    private init(_ asset: Int64) {
        self.asset = asset
    }
    
    
    
    public static func createFromImageBitmap(
        _ imageBitmap: TNSImageBitmap,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            let result = image_bitmap_create_from_image_asset(imageBitmap.asset,options.flipY,
                                                             options.premultiplyAlpha.rawValue,
                                                             options.colorSpaceConversion.rawValue,
                                                             options.resizeQuality.rawValue,
                                                             options.resizeWidth,
                                                             options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromImageBitmap(
        _ imageBitmap: TNSImageBitmap,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback: @escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            
            let result = image_bitmap_create_from_image_asset_src_rect(imageBitmap.asset,
                                                                      sx, sy, sWidth, sHeight,
                                                                      options.flipY,
                                                                      options.premultiplyAlpha.rawValue,
                                                                      options.colorSpaceConversion.rawValue,
                                                                      options.resizeQuality.rawValue,
                                                                      options.resizeWidth,
                                                                      options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    
    
    
    public static func createFromImageAsset(
        _ imageAsset: TNSImageAsset,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            let result = image_bitmap_create_from_image_asset(imageAsset.asset,options.flipY,
                                                             options.premultiplyAlpha.rawValue,
                                                             options.colorSpaceConversion.rawValue,
                                                             options.resizeQuality.rawValue,
                                                             options.resizeWidth,
                                                             options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromImageAsset(
        _ imageAsset: TNSImageAsset,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback: @escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            
            let result = image_bitmap_create_from_image_asset_src_rect(imageAsset.asset,
                                                                      sx, sy, sWidth, sHeight,
                                                                      options.flipY,
                                                                      options.premultiplyAlpha.rawValue,
                                                                      options.colorSpaceConversion.rawValue,
                                                                      options.resizeQuality.rawValue,
                                                                      options.resizeWidth,
                                                                      options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    
    public static func createFromImageData(
        _ imageData: TNSImageData,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            
            let result = image_bitmap_create_from_image_data(imageData.imageData,options.flipY,
                                                             options.premultiplyAlpha.rawValue,
                                                             options.colorSpaceConversion.rawValue,
                                                             options.resizeQuality.rawValue,
                                                             options.resizeWidth,
                                                             options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromImageData(
        _ imageData: TNSImageData,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback: @escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            
            let result = image_bitmap_create_from_image_data_src_rect(imageData.imageData,
                                                                      sx, sy, sWidth, sHeight,
                                                                      options.flipY,
                                                                      options.premultiplyAlpha.rawValue,
                                                                      options.colorSpaceConversion.rawValue,
                                                                      options.resizeQuality.rawValue,
                                                                      options.resizeWidth,
                                                                      options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    
    
    public static func createFromCanvas(
        _ canvas: TNSCanvas,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var ss = canvas.snapshot()
            let result = image_bitmap_create_from_bytes(&ss, UInt(ss.count), canvas.width, canvas.height, options.flipY,
                                                        options.premultiplyAlpha.rawValue,
                                                        options.colorSpaceConversion.rawValue,
                                                        options.resizeQuality.rawValue,
                                                        options.resizeWidth,
                                                        options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromCanvas(
        _ canvas: TNSCanvas,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var ss = canvas.snapshot()
            let result = image_bitmap_create_from_bytes_src_rect(&ss, UInt(ss.count), canvas.width, canvas.height, sx,
                                                                 sy,
                                                                 sWidth,
                                                                 sHeight, options.flipY,
                                                                 options.premultiplyAlpha.rawValue,
                                                                 options.colorSpaceConversion.rawValue,
                                                                 options.resizeQuality.rawValue,
                                                                 options.resizeWidth,
                                                                 options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    public static func createFromData(
        _ data: NSData,
        _ imageWidth: Float32,
        _ imageHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes = [UInt8](data)
            let result = image_bitmap_create_from_bytes(&bytes, UInt(bytes.count), imageWidth, imageHeight, options.flipY,
                                                        options.premultiplyAlpha.rawValue,
                                                        options.colorSpaceConversion.rawValue,
                                                        options.resizeQuality.rawValue,
                                                        options.resizeWidth,
                                                        options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromData(
        _ data: NSData,
        _ imageWidth: Float32,
        _ imageHeight: Float32,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes = [UInt8](data)
            let result = image_bitmap_create_from_bytes_src_rect(&bytes, UInt(bytes.count), imageWidth, imageHeight,
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
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    
    
    
    public static func createFromDataEncoded(
        _ data: NSData,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes = [UInt8](data)
            let result = image_bitmap_create_from_bytes_encoded(&bytes, UInt(bytes.count),options.flipY,
                                                        options.premultiplyAlpha.rawValue,
                                                        options.colorSpaceConversion.rawValue,
                                                        options.resizeQuality.rawValue,
                                                        options.resizeWidth,
                                                        options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromDataEncoded(
        _ data: NSData,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes = [UInt8](data)
            let result = image_bitmap_create_from_bytes_encoded_src_rect(&bytes, UInt(bytes.count),
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
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    
    
    public static func createFromBytes(
        _ bytes: [UInt8],
        _ imageWidth: Float32,
        _ imageHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes_in = bytes
            let result = image_bitmap_create_from_bytes(&bytes_in, UInt(bytes_in.count), imageWidth, imageHeight, options.flipY,
                                                        options.premultiplyAlpha.rawValue,
                                                        options.colorSpaceConversion.rawValue,
                                                        options.resizeQuality.rawValue,
                                                        options.resizeWidth,
                                                        options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromBytes(
        _ bytes: [UInt8],
        _ imageWidth: Float32,
        _ imageHeight: Float32,
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes_in = bytes
            let result = image_bitmap_create_from_bytes_src_rect(&bytes_in, UInt(bytes_in.count), imageWidth, imageHeight,
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
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    public static func createFromBytesEncoded(
        _ bytes: [UInt8],
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes_in = bytes
            let result = image_bitmap_create_from_bytes_encoded(&bytes_in, UInt(bytes_in.count),options.flipY,
                                                                options.premultiplyAlpha.rawValue,
                                                                options.colorSpaceConversion.rawValue,
                                                                options.resizeQuality.rawValue,
                                                                options.resizeWidth,
                                                                options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    public static func createFromBytesEncoded(
        _ bytes: [UInt8],
        _ sx: Float32,
        _ sy: Float32,
        _ sWidth: Float32,
        _ sHeight: Float32,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
            var bytes_in = bytes
            let result = image_bitmap_create_from_bytes_encoded_src_rect(&bytes_in, UInt(bytes_in.count),sx,
                                                                         sy,
                                                                         sWidth,
                                                                         sHeight,
                                                                         options.flipY,
                                                                         options.premultiplyAlpha.rawValue,
                                                                         options.colorSpaceConversion.rawValue,
                                                                         options.resizeQuality.rawValue,
                                                                         options.resizeWidth,
                                                                         options.resizeHeight)
            DispatchQueue.main.async {
                if (result > 0) {
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    
    public static func createFromUIImage(
        _ canvas: UIImage,
        _ options: TNSImageBitmapOptions,
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
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
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
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
        _ callback:@escaping (TNSImageBitmap?, String?) -> Void
    ) {
        TNSImageBitmap.queue.async {
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
                    callback(TNSImageBitmap(result),nil)
                } else {
                    callback(nil,TNSImageBitmap.FAILED_TO_LOAD)
                }
            }
        }
    }
    
    
    public var width: Int32 {
        if(asset == 0){
            return 0
        }
        return Int32(image_asset_width(asset))
    }
    
    public var height: Int32 {
        if(asset == 0){
            return 0
        }
        return Int32(image_asset_height(asset))
    }
    
    public func close() {
        free_data()
        if(asset != 0){
            destroy_image_asset(asset)
            asset = 0
        }
    }
    
    
    public var error: String? {
        if(asset == 0){
            return nil
        }
        if(_error != nil){
            return _error
        }
        let cStr = image_asset_get_error(asset)
        if(cStr == nil){return nil}
        let error = String(cString: cStr!)
        destroy_string(cStr)
        return error
    }
    
    private func free_data(){
        if(self.raw_data != nil){
            destroy_u8_array(self.raw_data!)
            self.raw_data = nil
        }
    }
    
    deinit {
        free_data()
        if(asset != 0){
            destroy_image_asset(asset)
            asset = 0
        }
    }
}

