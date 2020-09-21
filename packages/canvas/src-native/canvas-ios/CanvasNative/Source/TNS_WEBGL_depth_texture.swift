//
//  TNS_WEBGL_depth_texture.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_WEBGL_depth_texture)
public class TNS_WEBGL_depth_texture: NSObject {
    public override init() {

    }

    public var UNSIGNED_INT_24_8_WEBGL: Int32 {
        return GL_UNSIGNED_INT_24_8
    }

}
