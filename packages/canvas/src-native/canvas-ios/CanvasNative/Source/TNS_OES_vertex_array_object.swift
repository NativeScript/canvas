//
//  TNSOES_vertex_array_object.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/27/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_OES_vertex_array_object)
public class TNS_OES_vertex_array_object: NSObject {
    public override init() {}
    
    public var VERTEX_ARRAY_BINDING_OES: Int32 {
        return GL_VERTEX_ARRAY_BINDING_OES
    }
    
    public func createVertexArrayOES() -> UInt32 {
        var id = GLuint()
        glGenVertexArraysOES(1, &id)
        return id
    }
    
    public func deleteVertexArrayOES(arrayObject: UInt32) {
        var id = arrayObject
        glDeleteVertexArraysOES(1, &id)
    }
    
    public func isVertexArrayOES(arrayObject: UInt32) -> Bool {
        glIsVertexArrayOES(arrayObject) == GL_TRUE
    }
    
    public func bindVertexArrayOES(arrayObject: UInt32) {
        glBindVertexArrayOES(arrayObject)
    }
}
