//
//  TNSColorStyle.swift
//
//  Created by Osei Fortune on 7/16/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
import UIKit

@objc(CanvasColorStyleType)
public enum CanvasColorStyleType: Int, RawRepresentable {
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
        case "color":
            self = .Color
        default:
            return nil
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
    
    public typealias RawValue = UInt32
    
    public var rawValue: UInt32 {
        switch self {
        case .Repeat:
            return 0
        case .RepeatX:
            return 1
        case .RepeatY:
            return 2
        case .NoRepeat:
            return 3
        }
    }
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Repeat
        case 1:
            self = .RepeatX
        case 2:
            self = .RepeatY
        case 3:
            self = .NoRepeat
        default:
            return nil
        }
    }
    
    public init?(rawValue: String) {
        switch rawValue {
        case "repeat":
            self = .Repeat
        case "repeat-x":
            self = .RepeatX
        case "repeat-y":
            self = .RepeatY
        case "no-repeat":
            self = .NoRepeat
        default:
            return nil
        }
    }
}

@objcMembers
@objc(TNSColorStyle)
public class TNSColorStyle: NSObject {
    @objcMembers
    @objc(TNSColor)
    public class TNSColor: NSObject, ICanvasColorStyle {
        private var value: String
        
        @objc public init(_ color: String) {
            value = color
        }
        
        public var color: String {
            get {
                return value
            }
        }
        
        
        public func getStyleType() -> CanvasColorStyleType {
            return .Color
        }
    }
    
    @objcMembers
    @objc(TNSGradient)
    public class TNSGradient:NSObject, ICanvasColorStyle {
        var style: Int64
        init(_ style: Int64) {
            self.style = style
        }
        
        public func getStyleType() -> CanvasColorStyleType {
            return .Gradient
        }
        
        public func addColorStop(_ offset: Float,_ color: String){
            let cStr = (color as NSString).utf8String
            gradient_add_color_stop(style, offset, cStr)
        }
        
    }
    
    
    @objcMembers
    @objc(TNSLinearGradient)
    public class TNSLinearGradient: TNSGradient {
        override init(_ style: Int64) {
            super.init(style)
        }
    }
    
    @objcMembers
    @objc(TNSRadialGradient)
    public class TNSRadialGradient: TNSGradient {
        override init(_ style: Int64) {
            super.init(style)
        }
    }
    
    @objcMembers
    @objc(TNSPattern)
    public class TNSPattern: NSObject, ICanvasColorStyle {
        var style: Int64 = 0
        init(_ style: Int64){
            super.init()
            self.style = style
        }
        
        init?(context: Int64, src: UIImage, pattern: TNSPatternRepetition){
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
                let result = context_create_pattern(context,buffer?.assumingMemoryBound(to: UInt8.self), UInt(width * height)  * 4, Int32(width), Int32(height), Repetition(rawValue: pattern.rawValue))
                                                           buffer?.deallocate()
                if result ==  0 {
                    return nil
                }else {
                    self.style = result
                }
            }else {
                return nil
            }
        }
        
        
        init?(context: Int64, canvas: TNSCanvas, pattern: TNSPatternRepetition){
            super.init()
            let glContext = EAGLContext.current()
            var ss = canvas.snapshot()
            if glContext != nil {
                EAGLContext.setCurrent(glContext)
            }
            let result = context_create_pattern_encoded(context, &ss, UInt(ss.count), Repetition(rawValue: pattern.rawValue))
            if result == 0 {
                return nil
            }else {
                self.style = result
            }
        }
        
        
        init?(context: Int64, asset src: TNSImageAsset, pattern: TNSPatternRepetition){
            super.init()
            let result = context_create_pattern_asset(context,src.asset, Repetition(rawValue: pattern.rawValue))
            
            if result == 0 {
                return nil
            }else {
                self.style = result
            }
        }
        
        
        public func setTransform(matrix: TNSDOMMatrix) {
            pattern_set_transform(style, matrix.matrix)
        }
        
        deinit {
            if(style != 0){
                destroy_paint_style(style)
                style = 0
            }
        }
        
        public func getStyleType() -> CanvasColorStyleType {
            return .Pattern
        }
    }
}

