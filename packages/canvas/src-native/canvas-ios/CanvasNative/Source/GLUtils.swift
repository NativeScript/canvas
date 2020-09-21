//
//  GLUtils.swift
//  CanvasNative
//
//  Created by Osei Fortune on 26/07/2020.
//

import Foundation
import UIKit

class GLUtils {
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<UInt8>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<Int8>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place_i8(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<UInt16>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place_u16(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<Int16>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place_i16(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<UInt32>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place_u32(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<Int32>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place_i32(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<Float32>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place_f32(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    
    static func flipYInPlace(_ data: UnsafeMutablePointer<Float64>!,_ length: Int,_ bytesPerRow: Int,_ height: Int){
        native_flip_y_in_place_f64(data, UInt(length), UInt(bytesPerRow), UInt(height))
    }
    
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<UInt8>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<Int8>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d_i8(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<UInt16>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d_u16(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<Int16>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d_i16(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<UInt32>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d_u32(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<Int32>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d_i32(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<Float32>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d_f32(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    static func flipYInPlace3D(_ data: UnsafeMutablePointer<Float64>!,_ length: Int,_ bytesPerRow: Int,_ height: Int,_ depth: Int){
        native_flip_y_in_place_3d_f64(data, UInt(length), UInt(bytesPerRow), UInt(height), UInt(depth))
    }
    
    static var device: MTLDevice?
    static var glContext: EAGLContext?
    static func getBytesFromImage(pixels: UIImage) -> (UnsafeMutableRawPointer?, Int){
        var cgImage: CGImage?
        #if !targetEnvironment(simulator)
        device = MTLCreateSystemDefaultDevice()
        #endif
        
        if(device == nil){
            glContext = EAGLContext(api: .openGLES3)
            if(glContext == nil){
                glContext = EAGLContext(api: .openGLES2)
            }
        }
        if let image = pixels.cgImage {
            cgImage = image
        }else if let image = pixels.ciImage {
            var context: CIContext?
            if let mtlDevice = device {
                context = CIContext(mtlDevice: mtlDevice)
            }
            if let glCtx = glContext {
                context = CIContext(eaglContext: glCtx)
            }
            if let ctx = context {
                cgImage = ctx.createCGImage(image, from: image.extent)
            }
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
