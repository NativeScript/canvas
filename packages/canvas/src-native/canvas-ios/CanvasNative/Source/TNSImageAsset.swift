//
//  TNSImageAsset.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/4/20.
//

import Foundation
import UIKit
@objcMembers
@objc(TNSImageAsset)
public class TNSImageAsset: NSObject {
    var asset: Int64 = 0
    var raw_data: UnsafeMutablePointer<U8Array>? = nil
    public static var _queue: DispatchQueue?
    private static var queue: DispatchQueue {
        if(_queue == nil){
            _queue = DispatchQueue(label: "TNSImageAssetQueue", qos: .background, attributes:.concurrent, autoreleaseFrequency: .never, target: nil)
        }
        return _queue!
    }
    public override init() {
       self.asset = image_asset_create()
    }
    private var _error: String?
    public func loadImageFromPath(path: String) -> Bool {
        let ptr = (path as NSString).utf8String
        _error = nil
        free_data()
        return image_asset_load_from_path(asset, ptr)
    }
    
    public func loadImageFromUrl(url: String) -> Bool {
        _error = nil
        do {
            let data = try Data(contentsOf: URL(string: url)!)
            return loadImageFromBytes(array: [UInt8](data))
        } catch let thisError {
            _error = thisError.localizedDescription
            return false
        }
    }
    
    public func loadImageFromUrlAsync(url: String, callback: @escaping (String?)-> ()) {
        TNSImageAsset.queue.async {
            let success = self.loadImageFromUrl(url: url)
            if(success){
                DispatchQueue.main.async {
                    callback(nil)
                }
            }else {
                DispatchQueue.main.async {
                     callback(self.error)
                }
            }
        }
    }
    
    public func loadImageFromPathAsync(path: String, callback: @escaping (String?)-> ()){
        TNSImageAsset.queue.async {
            let success = self.loadImageFromPath(path: path)
            if(success){
                DispatchQueue.main.async {
                    callback(nil)
                }
            }else {
                DispatchQueue.main.async {
                     callback(self.error!)
                }
            }
        }
    }
    
    public func loadImageFromBytes(array: [UInt8]) -> Bool{
        var ptr = array
        _error = nil
        free_data()
        return image_asset_load_from_raw(asset, &ptr, UInt(array.count))
    }
    
    public func loadImageFromBytesAsync(array: [UInt8], callback: @escaping (String?)-> ()){
        TNSImageAsset.queue.async {
            let success = self.loadImageFromBytes(array: array)
            if(success){
                DispatchQueue.main.async {
                    callback(nil)
                }
            }else {
                DispatchQueue.main.async {
                     callback(self.error!)
                }
            }
        }
    }
    
    public func loadImageFromImage(image: UIImage) -> Bool {
        _error = nil
        var cgImage: CGImage?
        if let pixels = image.cgImage {
                   cgImage = pixels
               } else if let image = image.ciImage {
            let ctx = CIContext()
                cgImage = ctx.createCGImage(image, from: image.extent)
               }
               if let pixels = cgImage {
               free_data()
    
               let width = Int(pixels.width)
                                  let height = Int(pixels.height)
                                  let buffer = calloc(width * height, 4)
                let size = width * height * 4
                let imageCtx = CGContext(data: buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
                                  imageCtx!.draw(pixels, in: CGRect(x: 0, y: 0, width: width, height: height))
                                 
                let result = image_asset_load_from_raw(asset, buffer?.assumingMemoryBound(to: UInt8.self), UInt(size))
                buffer?.deallocate()
               return result
               }
        return false
    }
    
    public func loadImageFromImageAsync(image: UIImage, callback: @escaping (String?)-> ()){
        TNSImageAsset.queue.async {
            let success = self.loadImageFromImage(image: image)
            if(success){
                DispatchQueue.main.async {
                    callback(nil)
                }
            }else {
                DispatchQueue.main.async {
                     callback(self.error!)
                }
            }
        }
    }
    
    var length: Int {
        return Int(raw_data?.pointee.data_len ?? 0)
    }
    public func getRawBytes() -> UnsafeMutablePointer<UInt8>? {
        if(asset == 0){return nil}
        if(raw_data == nil){
            raw_data = image_asset_get_bytes(asset)
        }
        
        
        if raw_data != nil {
            return raw_data!.pointee.data
        }
        
        return nil
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
    
    public func flipX(){}
    
    private func free_data(){
        if(self.raw_data != nil){
            destroy_u8_array(self.raw_data!)
            self.raw_data = nil
        }
    }
    
    public func flipY(){}
    
    public func scale(x: UInt32, y: UInt32){
        if(asset == 0){
            return
        }
    
        image_asset_scale(asset, x, y)
        free_data()
    }
    
    public func save(path: String,format: TNSImageAssetFormat)-> Bool{
        let pathPtr = (path as NSString).utf8String
        return image_asset_save_path(asset, pathPtr, UInt32(format.rawValue))
    }
    
    public func saveAsync(path: String,format: TNSImageAssetFormat, callback: @escaping (Bool)-> ()){
        TNSImageAsset.queue.async {
            let success = self.save(path: path, format: format)
            if(success){
                DispatchQueue.main.async {
                    callback(true)
                }
            }else {
                DispatchQueue.main.async {
                     callback(false)
                }
            }
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
    
    deinit {
       free_data()
        if(asset != 0){
           destroy_image_asset(asset)
            asset = 0
        }
    }
}

