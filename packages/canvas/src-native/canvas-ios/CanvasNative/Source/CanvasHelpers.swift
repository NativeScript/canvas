//
//  CanvasHelpers.swift
//  CanvasNative
//
//  Created by Osei Fortune on 26/03/2023.
//

import Foundation
import UIKit

public class CanvasHelpers: NSObject {
    
    public static func getBytesFromUIImage(_ image: UIImage) -> NSMutableData {
        var cgImage = image.cgImage
        
        if(cgImage == nil && image.ciImage != nil){
            let context = CIContext()
            cgImage = context.createCGImage(image.ciImage!, from: image.ciImage!.extent)
        }
        
        if(cgImage != nil){
            let width = cgImage!.width
            let height = cgImage!.height
            let bytesPerRow = cgImage!.bytesPerRow
            let bytesPerPixel = bytesPerRow / width
            let size = width * height * bytesPerPixel
            var buffer = NSMutableData(length: size)
            let colorSpace = CGColorSpaceCreateDeviceRGB()
            let _ = CGContext(data: &buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: bytesPerRow, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
            
            return buffer!
        }
        
        return NSMutableData(length: 0)!
    }
    

    public static func createPattern(_ context: Int64, _ image: UIImage, _ repetition: String) -> Int64 {
        var bytes = getBytesFromUIImage(image)
        let width = Int32(image.size.width * UIScreen.main.scale)
        let height = Int32(image.size.width * UIScreen.main.scale)
        let repetition = (repetition as NSString).utf8String
        return canvas_native_context_create_pattern(context, width, height, &bytes, UInt(bytes.count), repetition)
    }
    
    public static func loadImageAssetWithContext(_ asset: Int64, _ image: UIImage) -> Bool {
        var bytes = getBytesFromUIImage(image)
        return canvas_native_imageasset_load_from_bytes(asset, &bytes, UInt(bytes.count))
    }
    
    public static func drawImage(context: Int64, image: UIImage, dx: Float, dy: Float) {
        var bytes = getBytesFromUIImage(image)
        let width = Float(image.size.width * UIScreen.main.scale)
        let height = Float(image.size.width * UIScreen.main.scale)
        canvas_native_context_draw_image_dx_dy_with_bytes(context, &bytes, UInt(bytes.count),width, height,dx, dy)
    }
    
    public static func drawImage(context: Int64, image: UIImage, dx: Float, dy: Float, dw: Float, dh: Float) {
        var bytes = getBytesFromUIImage(image)
        let width = Float(image.size.width * UIScreen.main.scale)
        let height = Float(image.size.width * UIScreen.main.scale)
       canvas_native_context_draw_image_dx_dy_dw_dh_with_bytes(context, &bytes, UInt(bytes.count),width, height,dx, dy, dw, dh)
    }
    
    public static func drawImage(context: Int64, image: UIImage, sx: Float, sy: Float, sw: Float, sh: Float ,dx: Float, dy: Float, dw: Float, dh: Float) {
        var bytes = getBytesFromUIImage(image)
        let width = Float(image.size.width * UIScreen.main.scale)
        let height = Float(image.size.width * UIScreen.main.scale)
        canvas_native_context_draw_image_with_bytes(context, &bytes, UInt(bytes.count),width, height, sx,  sy, sw, sh ,dx, dy, dw, dh)
    }
    
    public static func initGLWithView(_ view: Int64, _ width: Int32, _ height: Int32, _ alpha: Bool,
                                 _ antialias: Bool,
                                 _ depth: Bool,
              _ fail_if_major_performance_caveat: Bool,
                      _ power_preference:String,
                   _ premultiplied_alpha: Bool,
               _ preserve_drawing_buffer:Bool,
                               _ stencil:Bool,
                        _ desynchronized:Bool,
                         _ xr_compatible:Bool,
                               _ version:Int32,
                             _ is_canvas:Bool) -> Int64{
        
        let power_preference = (power_preference as NSString).utf8String
        
        
        return canvas_native_init_ios_gl(view, width, height, alpha, antialias, depth, fail_if_major_performance_caveat, power_preference, premultiplied_alpha, premultiplied_alpha, stencil, desynchronized, xr_compatible, version, is_canvas)
    }
  

    public static func releaseGL(_ context: Int64) {
        canvas_native_release_ios_gl(context)
    }

    public static func getGLPointer(_ context: Int64) -> Int64 {
        return canvas_native_get_gl_pointer(context)
    }

    public static func releaseGLPointer(_ context: Int64) {
        canvas_native_release_gl_pointer(context)
    }


    public static func create2DContext(_ context: Int64,
                        _ width:Int32,
                       _ height:Int32,
                        _ alpha:Bool,
                      _ density:Float,
                      _ samples:Int32,
                   _ font_color:Int32,
                          _ ppi:Float,
                                       _ direction:Int32 ) -> Int64 {
        return canvas_native_create_2d_context(context, width, height, alpha, density, samples, font_color, ppi, direction)
    }


    public static func updateGLSurfaceWithView(_ view: Int64,
                             _ width:Int32,
                            _ height:Int32,
                                               _ context:Int64) {
        canvas_native_update_gl_surface(view, width, height, context)
    }

    public static func test2D(_ context: Int64) {
        canvas_native_context_2d_test(context)
    }
    
    public static func testToDataURL(_ context: Int64)-> String {
        let data = canvas_native_context_2d_test_to_data_url(context);
        if(data == nil){
            return String()
        }
        let ret = String(utf8String: data!)
        
        canvas_native_context_2d_destroy_string(data)
        return ret ?? String()
    }

}
