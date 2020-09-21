//
//  EXT_blend_minmax.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_EXT_blend_minmax)
public class TNS_EXT_blend_minmax: NSObject {
    
    public override init(){}
    
    public var MIN_EXT: Int32 {
        get {
            return GL_MIN_EXT
        }
    }
    
    public var MAX_EXT: Int32 {
        get {
            return GL_MAX_EXT
        }
    }
}
