//
//  TNSImageBitmap.swift
//  CanvasNative
//
//  Created by Osei Fortune on 10/04/2021.
//

import Foundation
@objcMembers
@objc(TNSImageBitmap)
public class TNSImageBitmap: NSObject {
    var asset: Int64
    var raw_data: UnsafeMutablePointer<U8Array>? = nil
    private var _error: String?
    private init(asset: Int64) {
        self.asset = asset
    }
    
    func createFromBytes(_ bytes: [UInt8]) {
        // TNSImageBitmap(asset)
    }
    
    func createFromBytes(_ bytes: [UInt8],_ sx: Float32,_ sy: Float32,_ sWidth: Float32,_ sHeight: Float32) {
        // TNSImageBitmap(asset)
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
    
    func close() {
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

