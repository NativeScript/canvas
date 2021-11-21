//
//  EXT_sRGB.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
#if !targetEnvironment(macCatalyst)
import OpenGLES
#endif


#if targetEnvironment(macCatalyst)
import OpenGL
#endif
@objcMembers
@objc(TNS_EXT_sRGB)
public class TNS_EXT_sRGB: NSObject {
    public override init() {
        
    }
    
    public var SRGB_EXT: Int32 {
        return GL_SRGB_EXT
    }
    
    public var SRGB_ALPHA_EXT: Int32 {
        return GL_SRGB_ALPHA_EXT
    }
    
    public var SRGB8_ALPHA8_EXT: Int32 {
        return GL_SRGB8_ALPHA8_EXT
    }
    
    public var FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: Int32 {
        return GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT
    }
}
