//
//  TNSColorStyle.swift
//
//  Created by Osei Fortune on 7/16/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
import UIKit

@objc public enum CanvasColorStyleType: Int, RawRepresentable {
    case Color
    case Gradient
    case Pattern
    public typealias RawValue = String
    
    public var rawValue: RawValue {
        switch self {
        case .Color:
            return "color"
        case .Gradient:
            return "gradient"
        case .Pattern:
            return "pattern"
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case "gradient":
            self = .Gradient
        case "pattern":
            self = .Pattern
        default:
            self = .Color
        }
    }
    
}

@objc public protocol ICanvasColorStyle {
    func getStyleType() ->  CanvasColorStyleType
}

@objc public enum TNSPatternRepetition: Int, RawRepresentable {
    case Repeat
    case RepeatX
    case RepeatY
    case NoRepeat
    
    public typealias RawValue = String
    
    public var rawValue: String {
        switch self {
        case .RepeatX:
            return "repeat-x"
        case .RepeatY:
            return "repeat-y"
        case .NoRepeat:
            return "no-repeat"
        default:
            return "repeat"
        }
    }
    public init?(rawValue: RawValue) {
        switch rawValue {
        case "repeat-x":
            self = .RepeatX
        case "repeat-y":
            self = .RepeatY
        case "no-repeat":
            self = .NoRepeat
        default:
            self = .Repeat
        }
    }
}

@objcMembers
@objc(TNSColorStyle)
public class TNSColorStyle: NSObject {
    
    @objcMembers
    @objc(Color)
    public class Color: NSObject, ICanvasColorStyle {
        var color: UIColor
        @objc public init(color: UIColor?) {
            if(color == nil){
                self.color = UIColor.black
            }else {
                self.color = color!
            }
        }
        
        
        public func getStyleType() -> CanvasColorStyleType {
            return .Color
        }
    }
    
    @objcMembers
    @objc(Gradient)
    public class TNSGradient:NSObject, ICanvasColorStyle {
        var gradientMap: NSMutableDictionary = [:]
        
        public func getStyleType() -> CanvasColorStyleType {
            return .Gradient
        }
        
        public func addColorStop(offset: Float, color: UInt32){
            if (offset < 0) {
                return
            }
            
            if (offset > 1) {
                return
            }
            gradientMap[offset] = color
        }
        
        @objc(addColorStopWithOffsetUIColor::)
        public func addColorStop(offset: Float,color: UIColor){
            if (offset < 0) {
                return
            }
            
            if (offset > 1) {
                return
            }
            gradientMap[offset] = UInt32(color.colorCode)
        }
        
        
        func getPostions() -> [Float] {
            return Array(gradientMap.allKeys) as! [Float]
        }
        
        func getColors() -> [UInt32] {
            return Array(gradientMap.allValues) as! [UInt32]
        }
    }
    
    
    @objcMembers
    @objc(TNSLinearGradient)
    public class TNSLinearGradient: TNSGradient {
        let x0: Float
        let y0: Float
        let x1: Float
        let y1: Float
        
        public init(x0: Float, y0: Float, x1: Float, y1: Float) {
            self.x0 = x0
            self.y0 = y0
            self.x1 = x1
            self.y1 = y1
        }
    }
    
    @objcMembers
    @objc(TNSRadialGradient)
    public class TNSRadialGradient: TNSGradient {
        let x0: Float
        let y0: Float
        let r0: Float
        let x1: Float
        let y1: Float
        let r1: Float
        
        public init(x0:Float, y0: Float, r0: Float, x1: Float, y1: Float, r1: Float) {
            self.x0 = x0
            self.y0 = y0
            self.r0 = r0
            self.r1 = r1
            self.x1 = x1
            self.y1 = y1
        }
    }
    
    @objcMembers
    @objc(TNSPattern)
    public class TNSPattern:NSObject, ICanvasColorStyle {
        var nativePattern: Int64 = 0
        public init(src: UIImage, pattern: TNSPatternRepetition){
            super.init()
            
            var cgImage: CGImage?
            
            if let image = src.cgImage {
                cgImage = image
            } else if let image = src.ciImage {
                let ctx = CIContext()
                cgImage = ctx.createCGImage(image, from: image.extent)
            }
            if let image = cgImage {
                let width = Int(src.size.width)
                let height = Int(src.size.height)
                let buffer = calloc(width * height, 4)
                let colorSpace = CGColorSpaceCreateDeviceRGB()
                let imageCtx = CGContext(data: buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
                imageCtx!.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))
                let rep = (pattern.rawValue as NSString).utf8String
                nativePattern = native_create_pattern(buffer?.assumingMemoryBound(to: UInt8.self), width * height  * 4, Int32(width), Int32(height), rep)
                buffer?.deallocate()
            }
        }
        
        
        public init(canvas: TNSCanvas, pattern: TNSPatternRepetition){
            super.init()
            let rep = (pattern.rawValue as NSString).utf8String
            let context = EAGLContext.current()
            var ss = canvas.snapshot()
            if context != nil {
                EAGLContext.setCurrent(context)
            }
            nativePattern = native_create_pattern_encoded(&ss, ss.count, rep)
        }
        
        
        public init(asset src: TNSImageAsset, pattern: TNSPatternRepetition){
            super.init()
            let size = src.width * src.height * 4
            let width = src.width
            let height = src.height
            let rep = (pattern.rawValue as NSString).utf8String
            nativePattern = native_create_pattern(src.getRawBytes(), Int(size), width, height, rep)
        }
        
        
        public func setTransform(matrix: TNSDOMMatrix) {
            nativePattern = native_set_pattern_transform(nativePattern, matrix.matrix)
        }
        
        deinit {
            if(nativePattern != 0){
                native_free_pattern(nativePattern)
                nativePattern = 0
            }
        }
        
        public func getStyleType() -> CanvasColorStyleType {
            return .Pattern
        }
    }
}

