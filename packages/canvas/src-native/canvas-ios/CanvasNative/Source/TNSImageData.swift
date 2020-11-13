//
//  TNSImageData.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objcMembers
@objc(TNSImageData)
public class TNSImageData: NSObject {
    var imageData: Int64
    private var dataHolder: NSData
    
    init(width: Int32, height: Int32) {
        imageData = image_data_create(Int32(width), Int32(height))
        let length = image_data_data_length(imageData)
        dataHolder = NSData(bytesNoCopy:image_data_data(imageData) , length: Int(length))
    }
    
    init(imageData: Int64) {
        self.imageData = imageData
        let length = image_data_data_length(imageData)
        dataHolder = NSData(bytesNoCopy:image_data_data(imageData) , length: Int(length))
    }
    
    public var width: Int32 {
        get {
            return image_data_width(imageData)
        }
    }
    
    public var height: Int32 {
        get {
            return image_data_height(imageData)
        }
    }
    
    public var data: NSData {
        get {
            return dataHolder
        }
    }
    
    deinit {
        destroy_image_data(imageData)
        imageData = 0
    }
}
