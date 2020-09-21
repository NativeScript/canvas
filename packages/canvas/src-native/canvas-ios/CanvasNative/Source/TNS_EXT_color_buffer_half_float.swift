//
//  EXT_color_buffer_half_float.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_EXT_color_buffer_half_float)
public class TNS_EXT_color_buffer_half_float: NSObject {
    public override init() {
        
    }
    public var RGBA16F_EXT: Int32 {
        return GL_RGBA16F_EXT
    }
    
    public var RGB16F_EXT: Int32 {
        return GL_RGB16F_EXT
    }
    
    public var FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: Int32 {
        return GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT
    }
    
    public var UNSIGNED_NORMALIZED_EXT: Int32 {
        return GL_UNSIGNED_NORMALIZED_EXT
    }
}
