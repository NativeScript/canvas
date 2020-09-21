//
//  TNS_WEBGL_compressed_texture_etc.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_WEBGL_compressed_texture_etc)
public class TNS_WEBGL_compressed_texture_etc: NSObject {
    public override init() {

    }

    public var COMPRESSED_R11_EAC: Int32 {
        return GL_COMPRESSED_R11_EAC
    }

    public var COMPRESSED_SIGNED_R11_EAC: Int32 {
        return GL_COMPRESSED_SIGNED_R11_EAC
    }

    public var COMPRESSED_RG11_EAC: Int32 {
        return GL_COMPRESSED_RG11_EAC
    }

    public var COMPRESSED_SIGNED_RG11_EAC: Int32 {
        return GL_COMPRESSED_SIGNED_RG11_EAC
    }

    public var COMPRESSED_RGB8_ETC2: Int32 {
        return GL_COMPRESSED_RGB8_ETC2
    }

    public var COMPRESSED_RGBA8_ETC2_EAC: Int32 {
        return GL_COMPRESSED_RGBA8_ETC2_EAC
    }

    public var COMPRESSED_SRGB8_ETC2: Int32 {
        return GL_COMPRESSED_SRGB8_ETC2
    }

    public var COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: Int32 {
        return GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC
    }

    public var COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: Int32 {
        return GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2
    }

    public var COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: Int32 {
        return GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2
    }
}
