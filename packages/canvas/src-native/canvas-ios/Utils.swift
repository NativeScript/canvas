//
//  Utils.swift
//  CanvasNative
//
//  Created by Osei Fortune on 30/03/2021.
//

import Foundation
import OpenGLES
import CoreVideo
import AVFoundation
@objc(Utils)
public class Utils: NSObject {
    private static let BYTES_PER_TEXEL = 4
    @objc public static func createTextureCache(_ context: TNSWebGLRenderingContext) -> CVOpenGLESTextureCache? {
        var out: CVOpenGLESTextureCache?
        CVOpenGLESTextureCacheCreate(kCFAllocatorDefault, nil, context.canvas.renderer.glContext, nil, &out)
        return out
    }
    
    @objc public static func createImage(_ texturecache: CVOpenGLESTextureCache,_ buffer: CVImageBuffer,_ textureAttributes: CFDictionary?, _ target: GLenum,_  internalFormat: GLint,_ width: GLsizei,_ height: GLsizei,_ format: GLenum,_ type: GLenum,_ planeIndex: Int) -> CVOpenGLESTexture?{
        var textureOut: CVOpenGLESTexture?
        CVOpenGLESTextureCacheCreateTextureFromImage(kCFAllocatorDefault, texturecache, buffer, textureAttributes, target, internalFormat, width, height, format, type, planeIndex,&textureOut)
        return textureOut
    }
    
    @objc public static func setupRender() -> TNSRender{
        return TNSRender()
    }
    
    @objc public static func drawFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput,_ videoSize: CGSize, _ render: TNSRender,_ internalFormat: Int32,_ format: Int32,_ flipYWebGL: Bool){

        let currentTime = player.currentTime()
        
        if(!output.hasNewPixelBuffer(forItemTime: currentTime)) {return}
        
        let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: nil)
        
        guard let pixel_buffer = buffer else {return}
        render.drawFrame(buffer: pixel_buffer, width: Int(videoSize.width), height: Int(videoSize.height), internalFormat: internalFormat, format: format, flipYWebGL: flipYWebGL)
    }
    
}
