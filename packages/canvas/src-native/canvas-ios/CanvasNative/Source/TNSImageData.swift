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
    private var _width: Int
    private var _height: Int
    private var _data: Data
    public init(width: Int, height: Int) {
        _width = width
        _height = height
        let data = native_create_image_data(width, height)
        if(data != nil){
            let pointer = data!.pointee
            _data = Data(bytes: pointer.array, count: pointer.length)
            native_drop_image_data(data)
        }else {
            _data = Data()
        }
    }
    public var width: Int {
        get {
            return _width
        }
    }
    public var height: Int {
        get {
            return _height
        }
    }
    public var data: Data {
        get {
            return _data
        }
    }
    
    init(width: Int, height: Int, data: Data) {
        _width = width
        _height = height
        _data = data
    }
}
