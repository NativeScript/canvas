    //
    //  NSCCanvasRenderingContext2D.swift
    //
    //  Created by Osei Fortune on 7/15/19.
    //  Copyright © 2019 Osei Fortune. All rights reserved.
    //
    
    import Foundation
    import UIKit
    @objcMembers
    @objc(NSCCanvasRenderingContext2D)
    public class NSCCanvasRenderingContext2D: NSCCanvasRenderingContext {
       
        public static func createPattern(_ context: Int64, _ src: UIImage, _ repetition: String) -> Int64 {
            return CanvasHelpers.createPattern(context, src, repetition as NSString)
        }
        
        
        
        public static func drawImage(_ context: Int64, _ image: UIImage, _ dx: Float, _ dy: Float){
            CanvasHelpers.drawImage(context: context, image: image, dx: dx, dy: dy)
        }
        


        public static func drawImage(_ context: Int64,_ image: UIImage, _ dx: Float, _ dy: Float, _ dWidth: Float, _ dHeight: Float){
            CanvasHelpers.drawImage(context: context, image: image, dx: dx, dy: dy, dw: dWidth, dh: dHeight)
        }
        
    
        public static func drawImage(_ context: Int64,_ image: UIImage, _ sx: Float, _ sy: Float, _ sWidth: Float, _ sHeight: Float, _ dx: Float, _ dy: Float, _ dWidth: Float, _ dHeight: Float){
            CanvasHelpers.drawImage(context: context, image: image, sx: sx, sy: sy, sw: sWidth, sh: sHeight, dx: dx, dy: dy, dw: dWidth, dh: dHeight)
        }
        
       
    }
