//
//  NSCSVG.swift
//  CanvasSVG
//
//  Created by Osei Fortune on 18/03/2024.
//

import Foundation
import UIKit


@objcMembers
@objc(NSCSVG)
public class NSCSVG: UIView {
    public internal(set) var data: UnsafeMutableRawPointer? = nil
    public internal(set) var data_size: CGSize = .zero
    public internal(set) var buf_size: UInt = 0
    var didInitDrawing = false
    var forceResize = false
    var sync = false
    public var autoScale = true {
        didSet {
            forceResize = true
        }
    }
    public var src: String? = nil {
        didSet {
            doDraw()
        }
    }
    
    public var srcPath: String? = nil {
        didSet {
            doDraw()
        }
    }
    
    func deviceScale() -> Float32 {
        if autoScale  {
            return Float32(UIScreen.main.nativeScale)
        }
        return 1
    }
    
    var workItem: DispatchWorkItem?
    func doDraw(){
        if self.srcPath == nil && self.src == nil {return}
        workItem?.cancel()
        
        if(sync){
            if(self.srcPath != nil){
                guard let srcPath = self.srcPath else{return}
                let source = srcPath as NSString
                
                guard let buf = self.data?.assumingMemoryBound(to: UInt8.self) else {return}
                
                CanvasSVGHelper.draw(fromPath: buf, size: self.buf_size, width: Float(self.data_size.width), height: Float(self.data_size.height), path: source as String)
                
                
                
                self.didInitDrawing = true
                self.setNeedsDisplay()
                return
            }
            
            guard let src = self.src else{return}
            let source = src as NSString
            
            guard let buf = self.data?.assumingMemoryBound(to: UInt8.self) else {return}
            
            
            CanvasSVGHelper.draw(fromString: buf, size: self.buf_size, width: Float(self.data_size.width), height: Float(self.data_size.height), svg: source as String)
            
            self.didInitDrawing = true
            self.setNeedsDisplay()
            
            return
        }
        
        
        workItem = DispatchWorkItem {
            [weak self] in
            guard let self =  self else {return}
            
            if(self.srcPath != nil){
                guard let srcPath = self.srcPath else{return}
                let source = srcPath as NSString
                
                
                guard let buf = self.data?.assumingMemoryBound(to: UInt8.self) else {return}
                
                
                CanvasSVGHelper.draw(fromPath: buf, size: self.buf_size, width: Float(self.data_size.width), height: Float(self.data_size.height), path: source as String)
                
                
                DispatchQueue.main.async { [self] in
                    if(self.workItem != nil && self.workItem!.isCancelled){
                        self.workItem = nil
                        return
                    }
                    self.didInitDrawing = true
                    self.setNeedsDisplay()
                }
                return
            }
            
            
            guard let src = self.src else{return}
            let source = src as NSString
            
            guard let buf = self.data?.assumingMemoryBound(to: UInt8.self) else {return}
            
            
            
            CanvasSVGHelper.draw(fromString: buf, size: self.buf_size, width: Float(self.data_size.width), height: Float(self.data_size.height), svg: source as String)
            
            
            DispatchQueue.main.async {
                [self] in
                if(self.workItem != nil && self.workItem!.isCancelled){
                    self.workItem = nil
                    return
                }
                self.didInitDrawing = true
                self.setNeedsDisplay()
            }
        }
        queue.async(execute: workItem!)
    }
    
    
    public func update(){
        let size = layer.frame.size
        let width = Float(size.width) * deviceScale()
        let height = Float(size.height) * deviceScale()
        if (CGFloat(width) != data_size.width && CGFloat(height) != data_size.height) || forceResize {
            data?.deallocate()
            data = calloc(Int(width * height), 4)
            buf_size = UInt(width * height * 4)
            data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
            doDraw()
            
            if forceResize {
                forceResize = false
            }
        }
    }
    
    public override func layoutSubviews() {
        update()
    }
    
    
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    private var queue: DispatchQueue
    public override init(frame: CGRect) {
        queue = DispatchQueue(label: "NSCSVG")
        super.init(frame: frame)
        backgroundColor = .white
    }
    
    required init?(coder: NSCoder) {
        queue = DispatchQueue(label: "NSCSVG")
        super.init(coder: coder)
    }
    
    private func drawImage(_ rect: CGRect){
        let width = Int(self.data_size.width)
        let height = Int(self.data_size.height)
        let ctx = CGContext(data: self.data, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: self.colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
        
        guard let cgImage = ctx?.makeImage() else {return}
        let image = UIImage(cgImage: cgImage)
        image.draw(in: rect)
    }
    
    public override func draw(_ rect: CGRect) {
        if didInitDrawing {
            drawImage(rect)
        }
    }
    
    public func toImage() -> UIImage? {
        if didInitDrawing {
            let width = Int(self.data_size.width)
            let height = Int(self.data_size.height)
            let ctx = CGContext(data: self.data, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: self.colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
            
            guard let cgImage = ctx?.makeImage() else {return nil}
            return UIImage(cgImage: cgImage)
            
        }
        return nil
    }
    
    public func toData() -> NSData? {
        return NSMutableData(bytes: data, length: Int(buf_size))
    }
    
    public static func fromStringSync(_ source: String) -> NSCSVG? {
        var view = NSCSVG()
        let dim = parseSVGDimensions(source)
        if(dim.width.isZero || dim.height.isZero){
            return nil
        }
        view.frame = CGRect(x: 0, y: 0, width: dim.width, height: dim.height)
        let width = dim.width
        let height = dim.height
        view.data = calloc(Int(width * height), 4)
        view.buf_size = UInt(width * height * 4)
        view.data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
        
        
        guard let buf = view.data?.assumingMemoryBound(to: UInt8.self) else {return nil}
        CanvasSVGHelper.draw(fromString: buf, size: view.buf_size, width: Float(view.data_size.width), height: Float(view.data_size.height), svg: source as String)
    
        
        return view
    }
    
    
    public static func fromPathSync(_ path: String) -> NSCSVG? {
        var view = NSCSVG()
        if(!FileManager.default.fileExists(atPath: path)){return nil}
        do {
            let text = try String(contentsOfFile: path)
            if (text.isEmpty) {
                return nil
            }
            
            let dim = parseSVGDimensions(text)
            if(dim.width.isZero || dim.height.isZero){
                return nil
            }
            view.frame = CGRect(x: 0, y: 0, width: dim.width, height: dim.height)
            let width = dim.width
            let height = dim.height
            view.data = calloc(Int(width * height), 4)
            view.buf_size = UInt(width * height * 4)
            view.data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
        
            
            guard let buf = view.data?.assumingMemoryBound(to: UInt8.self) else {return nil}
            CanvasSVGHelper.draw(fromString: buf, size: view.buf_size, width: Float(view.data_size.width), height: Float(view.data_size.height), svg: text as String)
            
        }catch{
            return nil
        }
        return view
    }
    
    
    public static func fromRemoteSync(_ path: String) -> NSCSVG? {
        var view = NSCSVG()
        guard let url = URL(string: path) else {
            return nil
        }
        do {
            let text = try String(contentsOf: url, encoding: .utf8)
            if (text.isEmpty) {
                return nil
            }
            
            let dim = parseSVGDimensions(text)
            if(dim.width.isZero || dim.height.isZero){
                return nil
            }
            view.frame = CGRect(x: 0, y: 0, width: dim.width, height: dim.height)
            let width = dim.width
            let height = dim.height
            view.data = calloc(Int(width * height), 4)
            view.buf_size = UInt(width * height * 4)
            view.data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
            
            
            guard let buf = view.data?.assumingMemoryBound(to: UInt8.self) else {return nil}
            CanvasSVGHelper.draw(fromString: buf, size: view.buf_size, width: Float(view.data_size.width), height: Float(view.data_size.height), svg: text as String)
            
        }catch{
            return nil
        }
        return view
    }
    
    
    public static func fromString(_ source: String, _ callback:@escaping ((NSCSVG?)-> Void)) {
        DispatchQueue.global(qos: .utility).async {
            var view = NSCSVG()
            let dim = parseSVGDimensions(source)
            if(dim.width.isZero || dim.height.isZero){
                callback(nil)
                return
            }
            
            let width = dim.width
            let height = dim.height
            var data = calloc(Int(width * height), 4)
            var buf_size = UInt(width * height * 4)
            var data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
            
            
            guard let buf = view.data?.assumingMemoryBound(to: UInt8.self) else {
                callback(nil)
                return
            }
            
            CanvasSVGHelper.draw(fromString: buf, size: buf_size, width: Float(data_size.width), height: Float(data_size.height), svg: source as String)
            DispatchQueue.main.async {
                var view = NSCSVG()
                view.frame = CGRect(x: 0, y: 0, width: dim.width, height: dim.height)
                view.data = data
                view.data_size = data_size
                view.buf_size = buf_size
                callback(view)
            }
        }
    }
    
    
    public static func fromPath(_ path: String, _ callback:@escaping ((NSCSVG?)-> Void)) {
        DispatchQueue.global(qos: .utility).async {
            var view = NSCSVG()
            if(!FileManager.default.fileExists(atPath: path)){
                DispatchQueue.main.async {
                    callback(nil)
                }
                return
            }
            do {
                let text = try String(contentsOfFile: path)
                if (text.isEmpty) {
                    DispatchQueue.main.async {
                        callback(nil)
                    }
                    return
                }
                
                let dim = parseSVGDimensions(text)
                if(dim.width.isZero || dim.height.isZero){
                    callback(nil)
                    return
                }
                let width = dim.width
                let height = dim.height
                var data = calloc(Int(width * height), 4)
                var buf_size = UInt(width * height * 4)
                var data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
                
                
                guard let buf = data?.assumingMemoryBound(to: UInt8.self) else {
                    callback(nil)
                    return
                }
                CanvasSVGHelper.draw(fromString: buf, size: buf_size, width: Float(data_size.width), height: Float(data_size.height), svg: text as String)
                
                DispatchQueue.main.async {
                    var view = NSCSVG()
                    view.frame = CGRect(x: 0, y: 0, width: dim.width, height: dim.height)
                    view.data = data
                    view.data_size = data_size
                    view.buf_size = buf_size
                    callback(view)
                }
                
            }catch{
                DispatchQueue.main.async {
                    callback(nil)
                }
            }
           
        }
    }
    
    
    public static func fromRemote(_ path: String, _ callback:@escaping ((NSCSVG?)-> Void)) {
        DispatchQueue.global(qos: .utility).async {
            guard let url = URL(string: path) else {
                DispatchQueue.main.async {
                    callback(nil)
                }
                return
            }
            do {
                let text = try String(contentsOf: url, encoding: .utf8)
                if (text.isEmpty) {
                    DispatchQueue.main.async {
                        callback(nil)
                    }
                    return
                }
                
                let dim = parseSVGDimensions(text)
                if(dim.width.isZero || dim.height.isZero){
                    DispatchQueue.main.async {
                        callback(nil)
                    }
                    return
                }
                let width = dim.width
                let height = dim.height
                var data = calloc(Int(width * height), 4)
                var buf_size = UInt(width * height * 4)
                var data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
                
                
                guard let buf = data?.assumingMemoryBound(to: UInt8.self) else {
                    DispatchQueue.main.async {
                        callback(nil)
                    }
                    return
                }
                CanvasSVGHelper.draw(fromString: buf, size: buf_size, width: Float(data_size.width), height: Float(data_size.height), svg: text as String)
                
                DispatchQueue.main.async {
                    var view = NSCSVG()
                    view.frame = CGRect(x: 0, y: 0, width: dim.width, height: dim.height)
                    view.data = data
                    view.data_size = data_size
                    view.buf_size = buf_size
                    callback(view)
                }
                
                
            }catch{
                DispatchQueue.main.async {
                    callback(nil)
                }
            }
        }
    }
    
    struct Dimensions {
        var width: Double
        var height: Double
    }
    
    
    static func parseSVGDimensions(_ svgString: String) -> Dimensions {
        let svgRegex = try! NSRegularExpression(pattern: "<svg\\s*([^>]*)>", options: [.caseInsensitive])
        let matches = svgRegex.matches(in: svgString, options: [], range: NSRange(location: 0, length: svgString.utf16.count))
        
        var dimensions = Dimensions(width: 0, height: 0)
        
        guard let match = matches.first else {
            return dimensions
        }
        
        let svgAttributes = (svgString as NSString).substring(with: match.range(at: 1))
        
        let attributesPattern = try! NSRegularExpression(pattern: "(?:width|height|viewBox)\\s*=\\s*\"([^\"]+)\"", options: [])
        let matchesAttributes = attributesPattern.matches(in: svgAttributes, options: [], range: NSRange(location: 0, length: svgAttributes.utf16.count))
        
        var width: Double = 0.0
        var height: Double = 0.0
        var widthDefined = false
        var heightDefined = false
        var viewBox = [Double](repeating: 0.0, count: 4)
        
        for match in matchesAttributes {
            let attributePair = (svgAttributes as NSString).substring(with: match.range)
            let parts = attributePair.components(separatedBy: "=").map { $0.trimmingCharacters(in: .whitespaces) }
            
            if parts.count == 2 {
                let attributeName = parts[0]
                var attributeValue = parts[1].replacingOccurrences(of: "\"", with: "")
                
                let stringLiteralPattern = try! NSRegularExpression(pattern: "\\\\\"(\\d*\\.?\\d+)\\\\\"")
                let stringLiteralMatches = stringLiteralPattern.matches(in: attributeValue, options: [], range: NSRange(location: 0, length: attributeValue.utf16.count))
                
                if let stringLiteralMatch = stringLiteralMatches.first {
                    attributeValue = (attributeValue as NSString).substring(with: stringLiteralMatch.range(at: 1))
                }
                
                if attributeName == "width" {
                    width = (attributeValue as NSString).doubleValue
                    widthDefined = true
                } else if attributeName == "height" {
                    height = (attributeValue as NSString).doubleValue
                    heightDefined = true
                } else if attributeName == "viewBox" {
                    let viewBoxValues = attributeValue.components(separatedBy: .whitespaces)
                    for i in 0..<min(4, viewBoxValues.count) {
                        var value = viewBoxValues[i].replacingOccurrences(of: "\"", with: "")
                        let stringLiteralMatches = stringLiteralPattern.matches(in: value, options: [], range: NSRange(location: 0, length: value.utf16.count))
                        if let stringLiteralMatch = stringLiteralMatches.first {
                            value = (value as NSString).substring(with: stringLiteralMatch.range(at: 1))
                        }
                        viewBox[i] = (value as NSString).doubleValue
                    }
                }
            }
        }
        
        if width == 0.0 && viewBox.count == 4 {
            let aspectRatio = viewBox[2] / viewBox[3]
            width = UIScreen.main.bounds.width * aspectRatio
        }
        
        if height == 0.0 && viewBox.count == 4 {
            let aspectRatio = viewBox[2] / viewBox[3]
            height = UIScreen.main.bounds.height / aspectRatio
        }
        
        if (width == 0.0 || width.isNaN) && !widthDefined {
            width = UIScreen.main.bounds.width
        }
        
        if (height == 0.0 || height.isNaN) && !heightDefined {
            height = UIScreen.main.bounds.height
        }
        
        dimensions.width = width
        dimensions.height = height
        
        return dimensions
    }
}

