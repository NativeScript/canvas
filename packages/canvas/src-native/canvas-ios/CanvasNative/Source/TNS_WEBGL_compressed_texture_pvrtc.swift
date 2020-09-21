//
//  TNS_WEBGL_compressed_texture_pvrtc.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_WEBGL_compressed_texture_pvrtc)
public class TNS_WEBGL_compressed_texture_pvrtc: NSObject {
    public override init() {

    }

    public var COMPRESSED_RGB_PVRTC_4BPPV1_IMG: Int32 {
        return GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG
    }

    public var COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: Int32 {
        return GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG
    }
    public var COMPRESSED_RGB_PVRTC_2BPPV1_IMG: Int32 {
        return GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG
    }

    public var COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: Int32 {
        return GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG
    }
}
