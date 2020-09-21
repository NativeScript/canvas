//
//  TNS_texture_half_float.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_OES_texture_half_float)
public class TNS_OES_texture_half_float: NSObject {
    public override init() {}
    public var HALF_FLOAT_OES: Int32 {
        return GL_HALF_FLOAT_OES
    }
}
