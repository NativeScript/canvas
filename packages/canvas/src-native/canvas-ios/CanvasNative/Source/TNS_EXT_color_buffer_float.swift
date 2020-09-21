//
//  EXT_color_buffer_float.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_EXT_color_buffer_float)
public class TNS_EXT_color_buffer_float: NSObject {
    public override init(){}
    public var R16F: Int32 {
        return GL_R16F_EXT
    }
    
    public var RG16F: Int32 {
        return GL_RG16F_EXT
    }
    
    public var RGB16F: Int32 {
        return GL_RGB16F_EXT
    }
    
    public var R32F: Int32 {
        return GL_R32F_EXT
    }
    
    public var RG32F: Int32 {
        return GL_RG32F_EXT
    }
    
    public var RGBA32F: Int32 {
        return GL_RGBA32F_EXT
    }
    
    public var R11F_G11F_B10F: Int32 {
        return GL_R11F_G11F_B10F_APPLE
    }
}
