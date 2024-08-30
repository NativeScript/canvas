//
//  NSCRender.swift
//  CanvasNative
//
//  Created by Osei Fortune on 10/04/2021.
//

import Foundation
import OpenGLES
import CoreVideo
import UIKit


@objcMembers
@objc(NSCRender)
public class NSCRender: NSObject {
    
    
    static func getPixelFormat(_ cgImage: CGImage) -> CGBitmapInfo? {
        return cgImage.bitmapInfo.intersection(.byteOrderMask)
    }
    
    static func drawFrame(buffer:CVPixelBuffer, width: Int, height: Int, internalFormat: Int32,
                   format: Int32,
                   flipYWebGL: Bool) {
    
        
        let surface = CVPixelBufferGetIOSurface(buffer)
        
        guard let surface = surface else {return}

        let ref = surface.takeUnretainedValue()
        guard let address = IOSurfaceGetBaseAddress(ref) as UnsafeMutableRawPointer? else { return }
        IOSurfaceLock(ref, .readOnly, nil)
            glTexImage2D(GLenum(GL_TEXTURE_2D), 0, GL_RGBA, GLsizei(width), GLsizei(height), 0,  GLenum(GL_BGRA), GLenum(0x8367), address)
        IOSurfaceUnlock(ref, .readOnly, nil)
            glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAX_LEVEL), 100)
            glHint(GLenum(GL_GENERATE_MIPMAP_HINT), GLenum(GL_NICEST))
            glGenerateMipmap(GLenum(GL_TEXTURE_2D))
            glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_REPEAT)
            glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_REPEAT)
            glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
            glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR_MIPMAP_LINEAR)
    }
}
