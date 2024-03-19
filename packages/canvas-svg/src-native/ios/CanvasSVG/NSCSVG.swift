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
    var data: UnsafeMutableRawPointer? = nil
    var data_size: CGSize = .zero
    var buf_size: UInt = 0
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
                
                    
        
//                   canvas_native_svg_draw_from_path(buf, self.buf_size, Float(self.data_size.width), Float(self.data_size.height), source.utf8String!)
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
    

    func update(){
        let size = layer.frame.size
        let width = Float(size.width) * deviceScale()
        let height = Float(size.height) * deviceScale()
        if !size.equalTo(data_size) || forceResize {
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
    
    public override func draw(_ rect: CGRect) {
        if didInitDrawing {
            let width = Int(self.data_size.width)
            let height = Int(self.data_size.height)
            let ctx = CGContext(data: self.data, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: self.colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
            
            guard let cgImage = ctx?.makeImage() else {return}
            let image = UIImage(cgImage: cgImage)
            image.draw(in: rect)
            
        }
    }
    
    public func toImage() -> UIImage? {
        if  didInitDrawing {
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
}

