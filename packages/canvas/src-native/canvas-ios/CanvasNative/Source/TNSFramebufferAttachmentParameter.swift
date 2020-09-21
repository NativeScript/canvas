//
//  TNSFramebufferAttachmentParameter.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/18/20.
//

import Foundation
@objcMembers
@objc(TNSFramebufferAttachmentParameter)
public class TNSFramebufferAttachmentParameter: NSObject {
    var _isTexture: Bool
    var _isRenderbuffer: Bool
    var  _value: Int32
    public var isTexture: Bool{
        get {
            return _isTexture
        }
    }
    public var isRenderbuffer: Bool {
        get {
            return _isRenderbuffer
        }
    }
    public var value: Int32{
        get {
            return _value
        }
    }
    
    public override init() {
        _isTexture = false
        _isRenderbuffer = false
        _value = 0
    }
    public init(isTexture: Bool, isRenderbuffer: Bool, value: Int32) {
        _isTexture = isTexture
        _isRenderbuffer = isRenderbuffer
        _value = value
    }
}
