//
//  TNSWebGLActiveInfo.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/16/20.
//

import Foundation
@objcMembers
@objc(TNSWebGLActiveInfo)
public class TNSWebGLActiveInfo: NSObject {
    var _name: String
    var _size: Int32
    var _type: UInt32
    
    public var name: String {
        get {
            return _name
        }
    }
    public var size: Int32{
        get {
            return _size
        }
    }
    
    public var type: UInt32 {
        return _type
    }
    
    public init(name: String, size: Int32, type: UInt32) {
        _name = name
        _size = size
        _type = type
    }
}
