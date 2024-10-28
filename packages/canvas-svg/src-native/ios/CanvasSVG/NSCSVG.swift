//
//  NSCSVG.swift
//  CanvasSVG
//
//  Created by Osei Fortune on 18/03/2024.
//

import Foundation
import UIKit

@objcMembers
@objc(NSCSVGData)
public class NSCSVGData: NSObject {
    var data_ptr: UnsafeMutableRawPointer? = nil
    var data_size: CGSize = .zero
    var buf_size: UInt = 0
    var data_data: NSMutableData? = nil
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    var image: UIImage? = nil
    
    
    func resize(_ width: CGFloat, _ height: CGFloat){
        if(!width.isZero && !width.isNaN && !height.isZero && !height.isNaN){
            data_ptr?.deallocate()
            data_data = nil
            image = nil
            data_ptr = calloc(Int(width * height), 4)
            buf_size = UInt(width * height * 4)
            data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
        }
    }
    
    public var width: CGFloat {
        get {
            return data_size.width
        }
    }
    
    public var height: CGFloat {
        get {
            return data_size.height
        }
    }
    
    public var rawData: UnsafeMutableRawPointer? {
        get {
            return data_ptr
        }
    }
    
    public var data: NSMutableData? {
        get {
            if(data_data != nil){
                return data_data
            }
            guard let data = data_ptr else {return nil}
            let ptr = Unmanaged.passRetained(self)
            data_data = NSMutableData(bytesNoCopy: data, length: Int(buf_size)) { _, _ in
                let _ = Unmanaged.toOpaque(ptr)
            }
            return data_data
        }
    }
    
    public func getImage() -> UIImage? {
        if(image != nil){
            return image
        }
        guard let data = data_ptr else {return nil}
        
        let width = Int(self.data_size.width)
        let height = Int(self.data_size.height)
        let ctx = CGContext(data: data, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
        
        guard let cgImage = ctx?.makeImage() else {return nil}
        self.image = UIImage(cgImage: cgImage)
        return self.image
    }
}


@objcMembers
@objc(NSCSVG)
public class NSCSVG: UIView {
    var didInitDrawing = false
    var forceResize = false
    var sync = false
    var data: NSCSVGData? = nil
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
    
    func deviceScale() -> CGFloat {
        if autoScale  {
            return UIScreen.main.nativeScale
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
                
                guard let data = self.data else {return}
                
                guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
                
                CanvasSVGHelper.draw(fromPath: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), path: source as String)
                
                self.didInitDrawing = true
                self.setNeedsDisplay()
                return
            }
            
            guard let src = self.src else{return}
            let source = src as NSString
            
            guard let data = self.data else {return}
            
            guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
            
            
            CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
            
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
                guard let data = self.data else {return}
                
                guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
                
                
                CanvasSVGHelper.draw(fromPath: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), path: source as String)
                
                
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
            
            guard let data = self.data else {return}
            
            guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
            
            
            
            CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
            
            
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
        let scale = deviceScale()
        let width = size.width * scale
        let height = size.height * scale
        guard let data = data else {return}
        if (width != data.data_size.width && height != data.data_size.height) || forceResize {
            data.resize(width, height)
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
        guard let data = self.data else {return}
        let width = Int(data.data_size.width)
        let height = Int(data.data_size.height)
        guard let image = data.getImage() else {return}
        image.draw(in: rect)
    }
    
    public override func draw(_ rect: CGRect) {
        if didInitDrawing {
            drawImage(rect)
        }
    }
    
    public func toImage() -> UIImage? {
        if didInitDrawing {
            return self.data?.getImage()
        }
        return nil
    }
    
    public func toData() -> NSData? {
        return self.data?.data
    }
    
    public static func fromStringSync(_ source: String) -> NSCSVGData? {
        let dim = parseSVGDimensions(source)
        if(dim.width.isZero || dim.height.isZero){
            return nil
        }
        let width = dim.width
        let height = dim.height
        
        let data = NSCSVGData()
        data.resize(CGFloat(width), CGFloat(height))
       
        guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return nil}
        CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
        
        return data
    }
    
    
    public static func fromPathSync(_ path: String) -> NSCSVGData? {
        var data: NSCSVGData? = nil
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
            
            let ret = NSCSVGData()
            
            ret.resize(CGFloat(dim.width), CGFloat(dim.height))
           
            guard let buf = ret.rawData?.assumingMemoryBound(to: UInt8.self) else {return nil}
            
            CanvasSVGHelper.draw(fromString: buf, size: ret.buf_size, width: Float(ret.data_size.width), height: Float(ret.data_size.height), svg: text as String)
            
            data = ret
            
        }catch{
            return nil
        }
        return data
    }
    
    
    public static func fromRemoteSync(_ path: String) -> NSCSVGData? {
        var data: NSCSVGData? = nil
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
            
         
            let ret = NSCSVGData()
            
            ret.resize(CGFloat(dim.width), CGFloat(dim.height))
           
            guard let buf = ret.rawData?.assumingMemoryBound(to: UInt8.self) else {return nil}
            
            CanvasSVGHelper.draw(fromString: buf, size: ret.buf_size, width: Float(ret.data_size.width), height: Float(ret.data_size.height), svg: text as String)
            
            data = ret
            
            
        }catch{
            return nil
        }
        return data
    }
    
    static func emitEvent(_ loop: RunLoop, _ event: Timer){
        loop.add(event, forMode: .common)
    }
    
    
    public static func fromString(_ source: String, _ callback:@escaping ((NSCSVGData?)-> Void)) {
        let current = RunLoop.current
        DispatchQueue.global(qos: .utility).async {
            let nilEvent = Timer(timeInterval: 0, repeats: false) { _ in
                callback(nil)
            }
     
            let dim = parseSVGDimensions(source)
            
            
            if(dim.width.isZero || dim.height.isZero){
                emitEvent(current, nilEvent)
                return
            }
            
            var data = NSCSVGData()
            data.resize(CGFloat(dim.width), CGFloat(dim.height))
            
            
            guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {
                emitEvent(current, nilEvent)
                return
            }
            
            CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
       
            
            let event = Timer(timeInterval: 0, repeats: false) { _ in
                callback(data)
            }
            
            emitEvent(current, event)
        
        }
    }
    
    
    public static func fromPath(_ path: String, _ callback:@escaping ((NSCSVGData?)-> Void)) {
        let current = RunLoop.current
        DispatchQueue.global(qos: .utility).async {
            let nilEvent = Timer(timeInterval: 0, repeats: false) { _ in
                callback(nil)
            }
            
            if(!FileManager.default.fileExists(atPath: path)){
                emitEvent(current, nilEvent)
                return
            }
            do {
                let text = try String(contentsOfFile: path)
                if (text.isEmpty) {
                    emitEvent(current, nilEvent)
                    return
                }
                
                var dim = parseSVGDimensions(text)
                if(dim.width.isZero || dim.height.isZero){
                    emitEvent(current, nilEvent)
                    return
                }
                
                let data = NSCSVGData()
                data.resize(CGFloat(dim.width), CGFloat(dim.height))
                
                
                guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {
                    emitEvent(current, nilEvent)
                    return
                }
                
                CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: text as String)
                
                
                let event = Timer(timeInterval: 0, repeats: false) { _ in
                    callback(data)
                }
                
                emitEvent(current, event)
                
            }catch{
                emitEvent(current, nilEvent)
            }
            
        }
    }
    
    
    public static func fromRemote(_ path: String, _ callback:@escaping ((NSCSVGData?)-> Void)) {
        let current = RunLoop.current
        DispatchQueue.global(qos: .utility).async {
            let nilEvent = Timer(timeInterval: 0, repeats: false) { _ in
                callback(nil)
            }
            
            guard let url = URL(string: path) else {
                emitEvent(current, nilEvent)
                return
            }
            do {
                let text = try String(contentsOf: url, encoding: .utf8)
                if (text.isEmpty) {
                    emitEvent(current, nilEvent)
                    return
                }
                
                var dim = parseSVGDimensions(text)
                
                let data = NSCSVGData()
                data.resize(CGFloat(dim.width), CGFloat(dim.height))
                
                
                guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {
                    emitEvent(current, nilEvent)
                    return
                }
                CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: text as String)
                
                
                let event = Timer(timeInterval: 0, repeats: false) { _ in
                    callback(data)
                }
                
                emitEvent(current, event)
                
                
            }catch{
                emitEvent(current, nilEvent)
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
            let split = attributePair.components(separatedBy: " ").map { $0.trimmingCharacters(in: .whitespaces) }
            
            for part in split {
                let parts = part.components(separatedBy: "=").map { $0.trimmingCharacters(in: .whitespaces) }
                
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
            
            if (width == 0.0 || width.isNaN) && viewBox.count == 4 {
                let aspectRatio = viewBox[2] / viewBox[3]
                width = 150 * aspectRatio
            }
            
            if (height == 0.0 || height.isNaN)  && viewBox.count == 4 {
                let aspectRatio = viewBox[2] / viewBox[3]
                height = 300 / aspectRatio
            }
            
            if (width == 0.0 || width.isNaN) && !widthDefined {
                width = 300
            }
            
            if (height == 0.0 || height.isNaN) && !heightDefined {
                height = 150
            }
        }
        
        dimensions.width = width
        dimensions.height = height
        
        return dimensions
    }
}

