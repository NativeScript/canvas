//
//  TNS_WEBGL_color_buffer_float.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_WEBGL_color_buffer_float)
public class TNS_WEBGL_color_buffer_float: NSObject {
    public override init() {}

    public var RGBA32F_EXT: Int32 {
        return GL_RGBA32F_EXT
    }

    public var RGB32F_EXT: Int32 {
        return GL_RGB32F_EXT
    }

    public var FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: Int32 {
        return GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT
    }

    public var UNSIGNED_NORMALIZED_EXT: Int32 {
        return GL_UNSIGNED_NORMALIZED_EXT
    }
}
