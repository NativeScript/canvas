//
//  CanvasHelpersExt.swift
//  CanvasNative
//
//  Created by Osei Fortune on 26/03/2023.
//

import Foundation
extension CanvasHelpers {
    
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
    

    public static func createPattern(_ context: Int64, _ image: UIImage, _ repetition: NSString) -> Int64 {
        let bytes = getBytesFromUIImage(image)
        let scale = UIScreen.main.scale
        return CanvasHelpers.createPattern(context, width: Int32(image.size.width * scale), height: Int32(image.size.height * scale), data: bytes as Data, repetition: repetition as String)
    }
    
    public static func loadImageAssetWithContext(_ context: Int64, _ image: UIImage) -> Bool {
        let bytes = getBytesFromUIImage(image)
        return CanvasHelpers.loadImageAsset(fromData: context, data: bytes as Data)
    }
    
    public static func drawImage(context: Int64, image: UIImage, dx: Float, dy: Float) {
        let bytes = getBytesFromUIImage(image)
        CanvasHelpers.drawImage(withContext: context, data: bytes as Data, dx: dx, dy: dy)
    }
    
    public static func drawImage(context: Int64, image: UIImage, dx: Float, dy: Float, dw: Float, dh: Float) {
        let bytes = getBytesFromUIImage(image)
        CanvasHelpers.drawImage(withContext: context, data: bytes as Data, dx: dx, dy: dy, dw: dw, dh: dh)
    }
    
    public static func drawImage(context: Int64, image: UIImage, sx: Float, sy: Float, sw: Float, sh: Float ,dx: Float, dy: Float, dw: Float, dh: Float) {
        let bytes = getBytesFromUIImage(image)
        CanvasHelpers.drawImage(withContext: context, data: bytes as Data, sx: sx, sy: sy, sw: sw, sh: sh ,dx: dx, dy: dy, dw: dw, dh: dh)
    }
}
