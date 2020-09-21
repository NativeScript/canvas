//
//  TNS_WEBGL_draw_buffers.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/8/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_WEBGL_draw_buffers)
public class TNS_WEBGL_draw_buffers: NSObject {
    public var COLOR_ATTACHMENT0_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT0 }
    public var COLOR_ATTACHMENT1_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT1   }
    public var COLOR_ATTACHMENT2_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT2   }
    public var COLOR_ATTACHMENT3_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT3   }
    public var COLOR_ATTACHMENT4_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT4   }
    public var COLOR_ATTACHMENT5_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT5   }
    public var COLOR_ATTACHMENT6_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT6   }
    public var COLOR_ATTACHMENT7_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT7   }
    public var COLOR_ATTACHMENT8_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT8   }
    public var COLOR_ATTACHMENT9_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT9   }
    public var COLOR_ATTACHMENT10_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT10   }
    public var COLOR_ATTACHMENT11_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT11   }
    public var COLOR_ATTACHMENT12_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT12   }
    public var COLOR_ATTACHMENT13_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT13   }
    public var COLOR_ATTACHMENT14_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT14   }
    public var COLOR_ATTACHMENT15_WEBGL : Int32 {  return GL_COLOR_ATTACHMENT15   }
    public var DRAW_BUFFER0_WEBGL : Int32 {  return GL_DRAW_BUFFER0   }
    public var DRAW_BUFFER1_WEBGL : Int32 {  return GL_DRAW_BUFFER1   }
    public var DRAW_BUFFER2_WEBGL : Int32 {  return GL_DRAW_BUFFER2   }
    public var DRAW_BUFFER3_WEBGL : Int32 {  return GL_DRAW_BUFFER3   }
    public var DRAW_BUFFER4_WEBGL : Int32 {  return GL_DRAW_BUFFER4   }
    public var DRAW_BUFFER5_WEBGL : Int32 {  return GL_DRAW_BUFFER5   }
    public var DRAW_BUFFER6_WEBGL : Int32 {  return GL_DRAW_BUFFER6   }
    public var DRAW_BUFFER7_WEBGL : Int32 {  return GL_DRAW_BUFFER7   }
    public var DRAW_BUFFER8_WEBGL : Int32 {  return GL_DRAW_BUFFER8   }
    public var DRAW_BUFFER9_WEBGL : Int32 {  return GL_DRAW_BUFFER9   }
    public var DRAW_BUFFER10_WEBGL : Int32 {  return GL_DRAW_BUFFER10   }
    public var DRAW_BUFFER11_WEBGL : Int32 {  return GL_DRAW_BUFFER11   }
    public var DRAW_BUFFER12_WEBGL : Int32 {  return GL_DRAW_BUFFER12  }
    public var DRAW_BUFFER13_WEBGL : Int32 {  return GL_DRAW_BUFFER13   }
    public var DRAW_BUFFER14_WEBGL : Int32 {  return GL_DRAW_BUFFER14   }
    public var DRAW_BUFFER15_WEBGL : Int32 {  return GL_DRAW_BUFFER15   }
    public var MAX_COLOR_ATTACHMENTS_WEBGL : Int32 {  return GL_MAX_COLOR_ATTACHMENTS   }
    public var MAX_DRAW_BUFFERS_WEBGL : Int32  {return GL_MAX_DRAW_BUFFERS   }

    public func drawBuffersWEBGL(buffers: [Int32]) {
        let buf = buffers.withUnsafeBytes { (b) -> UnsafeRawPointer? in
            b.baseAddress
            }?.assumingMemoryBound(to: UInt32.self)
        glDrawBuffers(GLsizei(buffers.count), buf)
    }
}
