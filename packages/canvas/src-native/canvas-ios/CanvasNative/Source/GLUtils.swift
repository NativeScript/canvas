//
//  GLUtils.swift
//  CanvasNative
//
//  Created by Osei Fortune on 26/07/2020.
//

import Foundation
import UIKit

class GLUtils {
    static var device: MTLDevice?
    static var glContext: EAGLContext?
    static func getBytesFromImage(pixels: UIImage) -> (UnsafeMutableRawPointer?, Int){
        var cgImage: CGImage?
   
        if let image = pixels.cgImage {
            cgImage = image
        }else if let image = pixels.ciImage {
            let context = CIContext()
            cgImage = context.createCGImage(image, from: image.extent)
        }
        
        if let image = cgImage {
            let width = Int(pixels.size.width)
            let height = Int(pixels.size.height)
            let bytesPerRow = image.bytesPerRow
            let bytesPerPixel = bytesPerRow / width
            let size = width * height * bytesPerPixel
            let buffer = malloc(size)
            let colorSpace = CGColorSpaceCreateDeviceRGB()
            let imageCtx = CGContext(data: buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: bytesPerRow, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
            imageCtx!.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))
            return (buffer, size)
        }
        return (nil,0)
    }
}
