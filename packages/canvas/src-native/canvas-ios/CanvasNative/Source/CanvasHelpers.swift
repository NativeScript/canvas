//
//  CanvasHelpers.swift
//  CanvasNative
//
//  Created by Osei Fortune on 26/03/2023.
//

import Foundation
import UIKit

@objc(NSSCanvasHelpers)
@objcMembers
public class CanvasHelpers: NSObject {
    
    public static func getBytesFromUIImage(_ image: UIImage) -> NSMutableData {
        var cgImage = image.cgImage
        
        if(cgImage == nil && image.ciImage != nil){
            let context = CIContext()
            cgImage = context.createCGImage(image.ciImage!, from: image.ciImage!.extent)
        }
        
        guard let cgImage = cgImage else {
            return NSMutableData(length: 0)!
        }
       
        let width = cgImage.width
        let height = cgImage.height
        let bytesPerRow = width * 4
        let size = width * height * 4
        let buffer = NSMutableData(length: size)
        let colorSpace = CGColorSpaceCreateDeviceRGB()
        let ctx = CGContext(data: buffer?.mutableBytes, width: width, height: height, bitsPerComponent: 8, bytesPerRow: bytesPerRow, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
        
        ctx?.draw(cgImage, in: CGRect(x: 0, y: 0, width: width, height: height))
        
        return buffer!
    }
    

    public static func createPattern(_ context: Int64, _ image: UIImage, _ repetition: String) -> Int64 {
        let bytes = getBytesFromUIImage(image)
        let width = Int32(image.size.width)
        let height = Int32(image.size.height)
        let repetition = (repetition as NSString).utf8String
        return canvas_native_context_create_pattern_raw(context, width, height, bytes.mutableBytes, UInt(bytes.count), repetition)
    }
    
    public static func loadImageAssetWithContext(_ asset: Int64, _ image: UIImage) -> Bool {
        let bytes = getBytesFromUIImage(image)
        return canvas_native_imageasset_load_from_bytes(asset, bytes.mutableBytes, UInt(bytes.count))
    }
    
    public static func drawImage(context: Int64, image: UIImage, dx: Float, dy: Float) -> Bool {
        let bytes = getBytesFromUIImage(image)
        let width = Float(image.size.width)
        let height = Float(image.size.height)
        return canvas_native_context_draw_image_dx_dy_with_bytes(context, bytes.mutableBytes, UInt(bytes.count),width, height,dx, dy)
    }
    
    public static func drawImage(context: Int64, image: UIImage, dx: Float, dy: Float, dw: Float, dh: Float) -> Bool {
        let bytes = getBytesFromUIImage(image)
        let width = Float(image.size.width)
        let height = Float(image.size.height)
        return canvas_native_context_draw_image_dx_dy_dw_dh_with_bytes(context, bytes.mutableBytes, UInt(bytes.count),width, height,dx, dy, dw, dh)
    }
    
    public static func drawImage(context: Int64, image: UIImage, sx: Float, sy: Float, sw: Float, sh: Float ,dx: Float, dy: Float, dw: Float, dh: Float)  -> Bool {
        let bytes = getBytesFromUIImage(image)
        let width = Float(image.size.width)
        let height = Float(image.size.height)
        return canvas_native_context_draw_image_with_bytes(context, bytes.mutableBytes, UInt(bytes.count),width, height, sx,  sy, sw, sh ,dx, dy, dw, dh)
    }
    
    
    public static func initWebGPUWithViewLayer(_ instance: Int64,_ view: Int64, _ width: UInt32, _ height: UInt32) -> Int64{
        return canvas_native_init_ios_webgpu(instance, view, width, height)
    }
    
    public static func initWebGPUWithView(_ instance: Int64,_ view: Int64, _ width: UInt32, _ height: UInt32) -> Int64{
        return canvas_native_init_ios_webgpu_uiview(instance, view, width, height)
    }
    
    public static func resizeWebGPUWithView(_ context: Int64,_ view: Int64, _ width: UInt32, _ height: UInt32){
       canvas_native_resize_ios_webgpu_uiview(context, view, width, height)
    }
    
    public static func initGLWithView(_ view: Int64, _ alpha: Bool,
                                 _ antialias: Bool,
                                 _ depth: Bool,
              _ fail_if_major_performance_caveat: Bool,
                      _ power_preference:Int32,
                   _ premultiplied_alpha: Bool,
               _ preserve_drawing_buffer:Bool,
                               _ stencil:Bool,
                        _ desynchronized:Bool,
                         _ xr_compatible:Bool,
                               _ version:Int32,
                             _ is_canvas:Bool) -> Int64{
        return canvas_native_init_ios_gl(view, alpha, antialias, depth, fail_if_major_performance_caveat, power_preference, premultiplied_alpha, premultiplied_alpha, stencil, desynchronized, xr_compatible, version, is_canvas)
    }
    
    public static func initSharedGLWithView(_ view: Int64, _ alpha: Bool,
                                 _ antialias: Bool,
                                 _ depth: Bool,
              _ fail_if_major_performance_caveat: Bool,
                      _ power_preference:Int32,
                   _ premultiplied_alpha: Bool,
               _ preserve_drawing_buffer:Bool,
                               _ stencil:Bool,
                        _ desynchronized:Bool,
                         _ xr_compatible:Bool,
                               _ version:Int32,
                             _ is_canvas:Bool,
                                            _ shared_context: Int64) -> Int64{
    
        
        return canvas_native_init_ios_gl_with_shared_gl(view, alpha, antialias, depth, fail_if_major_performance_caveat, power_preference, premultiplied_alpha, premultiplied_alpha, stencil, desynchronized, xr_compatible, version, is_canvas, shared_context)
    }
    
    
    
    public static func initGLWithWidthAndHeight(_ width: Int32, _ height: Int32, _ alpha: Bool,
                                 _ antialias: Bool,
                                 _ depth: Bool,
              _ fail_if_major_performance_caveat: Bool,
                      _ power_preference:Int32,
                   _ premultiplied_alpha: Bool,
               _ preserve_drawing_buffer:Bool,
                               _ stencil:Bool,
                        _ desynchronized:Bool,
                         _ xr_compatible:Bool,
                               _ version:Int32,
                             _ is_canvas:Bool) -> Int64{
        
        return canvas_native_init_offscreen_ios_gl(width, height, alpha, antialias, depth, fail_if_major_performance_caveat, power_preference, premultiplied_alpha, premultiplied_alpha, stencil, desynchronized, xr_compatible, version, is_canvas)
    }
    
    public static func initSharedGLWidthAndHeight(_ width: Int32, _ height: Int32, _ alpha: Bool,
                                 _ antialias: Bool,
                                 _ depth: Bool,
              _ fail_if_major_performance_caveat: Bool,
                      _ power_preference:Int32,
                   _ premultiplied_alpha: Bool,
               _ preserve_drawing_buffer:Bool,
                               _ stencil:Bool,
                        _ desynchronized:Bool,
                         _ xr_compatible:Bool,
                               _ version:Int32,
                             _ is_canvas:Bool,
                                            _ shared_context: Int64) -> Int64{
        
        return canvas_native_init_offscreen_ios_gl_with_shared_gl(width, height, alpha, antialias, depth, fail_if_major_performance_caveat, power_preference, premultiplied_alpha, premultiplied_alpha, stencil, desynchronized, xr_compatible, version, is_canvas, shared_context)
    }
    
    
    public static func resize2DContext(_ context: Int64, _ width: Float, _ height: Float) {
        canvas_native_resize_context_2d(context, width, height)
    }
    
    public static func flush2DContext(_ context: Int64) {
        canvas_native_ios_flush_2d_context(context)
    }
    
    public static func flushGL(_ context: Int64)-> Bool {
        return canvas_native_ios_flush_gl(context)
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
    
    
    private static var _WriteQueue: OperationQueue?
    
    private static var WriteQueue: OperationQueue {
        if(_WriteQueue == nil){
            let queue = OperationQueue()
            queue.maxConcurrentOperationCount = 5
            _WriteQueue = queue
        }
        return _WriteQueue!
    }
    
    
    private static var _ReadQueue: OperationQueue?
    
    private static var ReadQueue: OperationQueue {
        if(_ReadQueue == nil){
            let queue = OperationQueue()
            queue.maxConcurrentOperationCount = 5
            _ReadQueue = queue
        }
        return _ReadQueue!
    }
    
    public static func writeFile(_ data: NSData, _ path: String, _ callback:@escaping (String?, String?) -> Void){
        let queue = OperationQueue.current ?? OperationQueue.main
        WriteQueue.addOperation {
            let done = data.write(toFile: path, atomically: true)
            if(!done){
                queue.addOperation {
                    callback("Failed to write file.", nil)
                }
            }else {
                queue.addOperation {
                    callback(nil, path)
                }
            }
           
        }
    }
    
    
    public static func readFile(_ path: String, _ callback:@escaping (String?, NSData?) -> Void){
        let queue = OperationQueue.current ?? OperationQueue.main
        ReadQueue.addOperation {
            let data = NSData(contentsOfFile: path)
            if(data == nil){
                queue.addOperation {
                    callback("Failed to read file.", nil)
                }
            }else {
                queue.addOperation {
                    callback(nil, data)
                }
            }
        }
    }
    
    
    public static func deleteFile(_ path: String, _ callback:@escaping (NSError?, Bool) -> Void){
        let queue = OperationQueue.current ?? OperationQueue.main
        ReadQueue.addOperation {
            do {
                try FileManager.default.removeItem(atPath: path)
                queue.addOperation {
                    callback(nil, true)
                }
            }catch {
                queue.addOperation {
                    callback(error as NSError, false)
                }
            }
        }
    }
    
    
    public static func handleBase64Image(_ mime: String, _ dir: String, _ base64: String, _ callback:@escaping (String?, String?) -> Void){
        let queue = OperationQueue.current ?? OperationQueue.main
        WriteQueue.addOperation {
    
            let id = UUID().uuidString
            
            let localUri = "\(dir)/\(id)-b64image.\(mime)"
            let toWrite = NSData(base64Encoded: base64)
            
            
            guard let toWrite = toWrite else {
                queue.addOperation {
                    callback("Failed to decode", nil)
                }
                return
            }
            
            let done = toWrite.write(toFile: localUri, atomically: true)
            
            if(!done){
                queue.addOperation {
                    callback("Failed to decode", nil)
                }
            }else {
                queue.addOperation {
                    callback(nil, localUri)
                }
                
            }
            
        }
    }
    
    public static func  getPixelsPerInchForCurrentDevice() -> String{
        var size: Int = 0;
        sysctlbyname("hw.machine", nil, &size, nil, 0);
        let machine = malloc(size);
        sysctlbyname("hw.machine", machine, &size, nil, 0);
        guard let machine = machine else {
            return ""
        }
        let platform = NSString(utf8String: machine) ?? ""
        free(machine);
        return platform as String
    }
}
