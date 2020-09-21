//
//  TNS_WEBGL_lose_context.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_WEBGL_lose_context)
public class TNS_WEBGL_lose_context: NSObject {
    var canvas: TNSCanvas

    public init(canvas: TNSCanvas) {
        self.canvas = canvas
    }

    public func loseContext(){
        print("loseContext", canvas)
        canvas.isContextLost = true
    }

    public func restoreContext(){
        print("restoreContext", canvas)
        canvas.isContextLost = false
    }
}
