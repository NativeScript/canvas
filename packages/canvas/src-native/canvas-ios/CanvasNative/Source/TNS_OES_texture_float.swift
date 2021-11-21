//
//  OES_texture_float.swift
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
@objc(TNS_OES_texture_float)
public class TNS_OES_texture_float: NSObject {
    public override init() {}
}
