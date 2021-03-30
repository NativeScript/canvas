//
//  TNSSVG.swift
//  CanvasNative
//
//  Created by Osei Fortune on 27/01/2021.
//

import Foundation
import UIKit
@objcMembers
@objc(TNSSVG)
public class TNSSVG: UIView {
    var data: UnsafeMutableRawPointer? = nil
    var data_size: CGSize = .zero
    var buf_size: UInt = 0
    var context: Int64 = 0
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
    
    func doDraw(){
        if self.srcPath == nil && self.src == nil {return}
        queue.async { [weak self] in
            guard let self =  self else {return}
            if(self.context > 0){
                if(self.srcPath != nil){
                    guard let srcPath = self.srcPath else{return}
                        let source = srcPath as NSString
                    svg_draw_from_path(self.context, source.utf8String)
                    guard let buf = self.data?.assumingMemoryBound(to: UInt8.self) else {return}
                    context_custom_with_buffer_flush(self.context, buf, self.buf_size, Float(self.data_size.width * UIScreen.main.nativeScale), Float(self.data_size.height * UIScreen.main.nativeScale))
                    
                    DispatchQueue.main.async {
                        self.setNeedsDisplay()
                    }
                    return
                }
                
                guard let src = self.src else{return}
                let source = src as NSString
                svg_draw_from_string(self.context, source.utf8String)
                guard let buf = self.data?.assumingMemoryBound(to: UInt8.self) else {return}
                context_custom_with_buffer_flush(self.context, buf, self.buf_size, Float(self.data_size.width * UIScreen.main.nativeScale), Float(self.data_size.height * UIScreen.main.nativeScale))
                
                DispatchQueue.main.async {
                    self.setNeedsDisplay()
                }
            }
        }
    }
    

    public override func layoutSubviews() {
        let size = layer.frame.size
        let width = size.width * UIScreen.main.nativeScale
        let height = size.height * UIScreen.main.nativeScale
        if !size.equalTo(data_size){
            data?.deallocate()
            data = malloc(Int(width * height * 4))
            buf_size = UInt(width * height * 4)
            data_size = size
            if context == 0 {
                var direction = TNSTextDirection.Ltr
                if(UIView.userInterfaceLayoutDirection(for: semanticContentAttribute) == .rightToLeft){
                    direction = TNSTextDirection.Rtl
                }
                context = context_init_context_with_custom_surface(Float(width), Float(height), Float(UIScreen.main.nativeScale), true, 0, 0, TextDirection(rawValue: direction.rawValue))
                doDraw()
            }else {
                context_resize_custom_surface(context, Float(width), Float(height), Float(UIScreen.main.nativeScale), true, 0)
                doDraw()
            }
        }
    }
    
    
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    private var queue: DispatchQueue
    public override init(frame: CGRect) {
        queue = DispatchQueue(label: "TNSSVG")
        super.init(frame: frame)
        contentMode = .topLeft
    }
    
    required init?(coder: NSCoder) {
        queue = DispatchQueue(label: "TNSSVG")
        super.init(coder: coder)
    }
    
    public override func draw(_ rect: CGRect) {
        if context > 0 {
            let width = Int(self.data_size.width * UIScreen.main.nativeScale)
            let height = Int(self.data_size.height * UIScreen.main.nativeScale)
            let ctx = CGContext(data: self.data, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: self.colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
         //   ctx?.clear(rect)
            
            guard let cgImage = ctx?.makeImage() else {return}
            let image = UIImage(cgImage: cgImage)
            image.draw(in: rect)
        }
    }
}
