//
//  EXT_texture_filter_anisotropic.swift
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
@objc(TNS_EXT_texture_filter_anisotropic)
public class TNS_EXT_texture_filter_anisotropic: NSObject {
    public override init() {
        
    }
    
    public var MAX_TEXTURE_MAX_ANISOTROPY_EXT: Int32 {
        return GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT
    }
    
    public var TEXTURE_MAX_ANISOTROPY_EXT: Int32 {
        return GL_TEXTURE_MAX_ANISOTROPY_EXT
    }
}

