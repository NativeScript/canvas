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
    var nativeAsset: Int64 = 0
    public static var _queue: DispatchQueue?
    private static var queue: DispatchQueue {
        if(_queue == nil){
            _queue = DispatchQueue(label: "TNSImageAssetQueue", qos: .background, attributes:.concurrent, autoreleaseFrequency: .never, target: nil)
        }
        return _queue!
    }
    public override init() {
       self.nativeAsset =  native_create_image_asset()
    }
    private var _error: String?
    public func loadImageFromPath(path: String) -> Bool{
        let ptr = (path as NSString).utf8String
        if(self.raw_data != nil){
            native_image_asset_free_bytes(self.raw_data!)
            self.raw_data = nil
        }
        return native_image_asset_load_from_path(nativeAsset, ptr) != 0
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
        if(self.raw_data != nil){
            native_image_asset_free_bytes(self.raw_data!)
            self.raw_data = nil
        }
        return native_image_asset_load_from_raw(nativeAsset, &ptr, array.count) != 0
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
        var cgImage: CGImage?
        if let pixels = image.cgImage {
                   cgImage = pixels
               } else if let image = image.ciImage {
            let ctx = CIContext()
                cgImage = ctx.createCGImage(image, from: image.extent)
               }
               if let pixels = cgImage {
                if(self.raw_data != nil){
                    native_image_asset_free_bytes(self.raw_data!)
                    self.raw_data = nil
                }
                
               let width = Int(pixels.width)
                                  let height = Int(pixels.height)
                                  let buffer = calloc(width * height, 4)
                let size = width * height * 4
                let imageCtx = CGContext(data: buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
                                  imageCtx!.draw(pixels, in: CGRect(x: 0, y: 0, width: width, height: height))
                                 
                let result = native_image_asset_load_from_raw(nativeAsset, buffer?.assumingMemoryBound(to: UInt8.self), size)
                buffer?.deallocate()
               return result != 0
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
    
    var raw_data: NativeByteArray?
    
    var length: Int {
        return raw_data?.length ?? (Int(width * height) * 4)
    }
    public func getRawBytes() -> UnsafeMutablePointer<UInt8>? {
        if(nativeAsset == 0){return nil}
        if(raw_data == nil){
            raw_data = native_image_asset_get_bytes(nativeAsset)
        }
        
        return raw_data?.array
        // use raw pointer directly
       // var bytes = Data(bytes: raw.array, count: raw.length) as NSData
       // native_image_asset_free_bytes(raw)
   
//        return bytes.withUnsafeMutableBytes { (ptr) -> UnsafeMutablePointer<UInt8>? in
//            ptr.baseAddress?.assumingMemoryBound(to: UInt8.self)
//        }
    }
    
    public var width: Int32 {
        if(nativeAsset == 0){
            return 0
        }
        return Int32(native_image_asset_get_width(nativeAsset))
    }
    
    public var height: Int32 {
       if(nativeAsset == 0){
           return 0
       }
        return Int32(native_image_asset_get_height(nativeAsset))
    }
    
    public func flipX(){
        if(nativeAsset == 0){
            return
        }
        self.nativeAsset = native_image_asset_flip_x(nativeAsset)
        if(self.raw_data != nil){
            native_image_asset_free_bytes(self.raw_data!)
            self.raw_data = nil
        }
    }
    
    public func flipY(){
        if(nativeAsset == 0){
            return
        }
        
       self.nativeAsset = native_image_asset_flip_y(nativeAsset)
        if(self.raw_data != nil){
            native_image_asset_free_bytes(self.raw_data!)
            self.raw_data = nil
        }
    }
    
    public func scale(x: UInt32, y: UInt32){
        if(nativeAsset == 0){
            return
        }
       self.nativeAsset = native_image_asset_scale(nativeAsset, x, y)
        
        if(self.raw_data != nil){
            native_image_asset_free_bytes(self.raw_data!)
            self.raw_data = nil
        }
    }
    
    public func save(path: String,format: TNSImageAssetFormat)-> Bool{
        let pathPtr = (path as NSString).utf8String
        return native_native_image_asset_save_path(nativeAsset, pathPtr, UInt32(format.rawValue)) != 0
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
        if(nativeAsset == 0){
            return nil
        }
        let cStr = native_image_asset_get_error(nativeAsset)
        if(cStr == nil){return nil}
        return String(cString: cStr!)
    }
    
    deinit {
       if(self.raw_data != nil){
           native_image_asset_free_bytes(self.raw_data!)
           self.raw_data = nil
       }
        if(nativeAsset != 0){
           native_image_asset_release(nativeAsset)
            nativeAsset = 0
        }
    }
}

