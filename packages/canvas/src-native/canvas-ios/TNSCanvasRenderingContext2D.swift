    //
    //  TNSCanvasRenderingContext2D.swift
    //
    //  Created by Osei Fortune on 7/15/19.
    //  Copyright © 2019 Osei Fortune. All rights reserved.
    //
    
    import Foundation
    import UIKit
    @objcMembers
    @objc(TNSCanvasRenderingContext2D)
    public class TNSCanvasRenderingContext2D: TNSCanvasRenderingContext {
        var canvas: TNSCanvas
        @discardableResult func ensureIsContextIsCurrent() -> Bool {
            return canvas.renderer.ensureIsContextIsCurrent()
        }
        public init(_ canvas: TNSCanvas) {
            self.canvas = canvas
            super.init()
            canvas.renderer.ensureIsContextIsCurrent()
        }
        
        
        public var font: String {
            get {
                let ptr = context_get_font(canvas.context)
                if ptr != nil {
                    let string = String(cString: ptr!)
                    destroy_string(string)
                    return string
                }
                return "none"
            }
            
            set {
                context_set_font(canvas.context, (newValue as NSString).utf8String)
            }
        }
        
        public var filter: String {
            get {
                let ptr = context_get_filter(canvas.context)
                if ptr != nil {
                    let string = String(cString: ptr!)
                    destroy_string(string)
                    return string
                }
                return "none"
            }
            
            set {
                context_set_filter(canvas.context, (newValue as NSString).utf8String)
            }
        }
        
        public var globalAlpha: Float {
            get{
                return context_get_global_alpha(canvas.context)
            }
            set {
                context_set_global_alpha(canvas.context, newValue)
            }
        }
        
        public var globalCompositeOperation: TNSCompositeOperationType {
            set {
                context_set_global_composite_operation(canvas.context, CompositeOperationType(rawValue: newValue.rawValue))
            }
            get {
                return TNSCompositeOperationType(rawValue: context_get_global_composite_operation(canvas.context).rawValue)!
            }
        }
        private var _imageSmoothingEnabled: Bool = false
        public var imageSmoothingEnabled: Bool {
            get {
                return context_get_image_smoothing_enabled(canvas.context)
            }
            set {
                context_set_image_smoothing_enabled(canvas.context, newValue)
            }
        }
        
        public var imageSmoothingQuality: TNSImageSmoothingQuality {
            get {
                return TNSImageSmoothingQuality(rawValue: context_get_image_smoothing_quality(canvas.context).rawValue)!
            }
            set {
                context_set_image_smoothing_quality(canvas.context, ImageSmoothingQuality(rawValue: UInt32(newValue.rawValue)))
            }
        }
        
        public var lineCap: TNSLineCap {
            get {
                return TNSLineCap(rawValue: context_get_line_cap(canvas.context).rawValue)!
            }
            set {
                if TNSLineCap(rawValue: newValue.rawValue) != nil {
                    context_set_line_cap(canvas.context, LineCap(rawValue: newValue.rawValue))
                }
            }
        }
        
     
        public var lineDashOffset: Float {
            get {
                return context_get_line_dash_offset(canvas.context)
            }
            set {
                context_set_line_dash_offset(canvas.context, newValue)
            }
        }
        public var lineWidth: Float {
            get {
                return context_get_line_width(canvas.context)
            }
            set {
                context_set_line_width(canvas.context, newValue)
            }
        }
        
        public var lineJoin: TNSLineJoin {
            get {
                return TNSLineJoin(rawValue: context_get_line_join(canvas.context).rawValue)!
            }
            set {
                context_set_line_join(canvas.context, LineJoin(rawValue: newValue.rawValue))
            }
        }
        public var miterLimit: Float {
            get {
                return context_get_miter_limit(canvas.context)
            }
            set {
                context_set_miter_limit(canvas.context, newValue)
            }
        }
    
        public var shadowBlur: Float {
            get {
                return context_get_shadow_blur(canvas.context)
            }
            set {
                context_set_shadow_blur(canvas.context, newValue)
            }
        }
        private var _shadowColor: Any = UIColor.clear
        public var shadowColor: String {
            get {
                let cStr = context_get_shadow_color(canvas.context)
                if cStr != nil {
                    let string = String(cString: cStr!)
                    destroy_string(cStr)
                    return string
                }
                return ""
            }
            set {
                context_set_shadow_color_string(canvas.context,(newValue as NSString).utf8String)
            }
        }
        
        public var shadowOffsetX: Float{
            get {
                return context_get_shadow_offset_x(canvas.context)
            }
            set {
                context_set_shadow_offset_x(canvas.context, newValue)
            }
        }
        
        public var shadowOffsetY: Float {
            get {
                return context_get_shadow_offset_y(canvas.context)
            }
            set {
                context_set_shadow_offset_y(canvas.context, newValue)
            }
        }
        
        
        
        public var textAlign: TNSTextAlignment {
            get {
                return TNSTextAlignment(rawValue: context_get_text_align(canvas.context).rawValue)!
            }
            set {
                context_set_text_align(canvas.context, TextAlign(rawValue: newValue.rawValue))
            }
        }
        
        public var textBaseline: TNSTextBaseLine {
            get {
                return TNSTextBaseLine(rawValue: context_get_text_baseline(canvas.context).rawValue)!
            }
            set {
                context_set_text_baseline(canvas.context, TextBaseLine(rawValue: newValue.rawValue))
            }
        }
        
        public func setFillStyleWithString(_ style: String){
            paint_style_set_fill_color_with_string(canvas.context, (style as NSString).utf8String)
        }
        
        public var fillStyle: ICanvasColorStyle {
            get {
                let ptr = context_get_fill_style(canvas.context)!
                let style = ptr.pointee
                switch style.value_type.rawValue {
                case 0:
                    let color = paint_style_get_color_string(style.value)
                    if color != nil {
                        let string = String(cString: color!)
                        destroy_string(color)
                        return TNSColorStyle.TNSColor(string)
                    }else {
                        return TNSColorStyle.TNSColor("")
                    }
                case 1:
                    return TNSColorStyle.TNSGradient(style.value)
                case 2:
                    return TNSColorStyle.TNSPattern(style.value)
                default:
                    return TNSColorStyle.TNSColor("")
                }
            }
            set {
                
                    switch newValue.getStyleType() {
                    case .Color:
                        paint_style_set_fill_color_with_string(canvas.context, ( newValue as! TNSColorStyle.TNSColor).color)
                    case .Gradient:
                        context_set_fill_style(canvas.context, (newValue as! TNSColorStyle.TNSGradient).style)
                    case .Pattern:
                        context_set_fill_style(canvas.context, (newValue as! TNSColorStyle.TNSPattern).style)
                    }
            }
        }
        
        
        public func setStrokeStyleWithString(_ style: String){
            paint_style_set_stroke_color_with_string(canvas.context, (style as NSString).utf8String)
        }
        
        
        public var strokeStyle: ICanvasColorStyle {
            get {
                let ptr = context_get_stroke_style(canvas.context)!
                let style = ptr.pointee
                switch style.value_type.rawValue {
                case 0:
                    let color = paint_style_get_color_string(style.value)
                    if color != nil {
                        let string = String(cString: color!)
                        destroy_string(color)
                        return TNSColorStyle.TNSColor(string)
                    }else {
                        return TNSColorStyle.TNSColor("")
                    }
                case 1:
                    return TNSColorStyle.TNSGradient(style.value)
                case 2:
                    return TNSColorStyle.TNSPattern(style.value)
                default:
                    return TNSColorStyle.TNSColor("")
                }
            }
            set {
                switch newValue.getStyleType() {
                case .Color:
                    paint_style_set_stroke_color_with_string(canvas.context, ( newValue as! TNSColorStyle.TNSColor).color)
                case .Gradient:
                    context_set_stroke_style(canvas.context, (newValue as! TNSColorStyle.TNSGradient).style)
                case .Pattern:
                    context_set_stroke_style(canvas.context, (newValue as! TNSColorStyle.TNSPattern).style)
                }
            }
        }
        
        public func fillRect(_ x: Float,_ y: Float,_ width: Float,_ height: Float) {
            ensureIsContextIsCurrent()
            context_fill_rect(canvas.context, x, y, width, height)
            canvas.doDraw()
        }
        
        public func strokeRect(_ x: Float,_ y: Float,_ width: Float,_ height: Float) {
            ensureIsContextIsCurrent()
            context_stroke_rect(canvas.context, x, y, width, height)
            canvas.doDraw()
        }
        
        public func fillText(_ text: String,_ x: Float,_ y:Float) {
            fillText(text, x, y, 0)
        }
        
        public func fillText(_ text: String,_ x: Float,_ y:Float,_ width: Float) {
            ensureIsContextIsCurrent()
            context_fill_text(canvas.context, text, x, y, width)
            canvas.doDraw()
        }
        
        public func strokeText(_ text: String,_ x: Float,_ y:Float) {
            strokeText(text, x, y, 0)
        }
        
        public func strokeText(_ text: String,_ x: Float,_ y:Float,_ width: Float) {
            ensureIsContextIsCurrent()
            context_stroke_text(canvas.context, text, x, y, width)
            canvas.doDraw()
        }
        
        public func rect(_ x: Float,_ y: Float,_ width: Float,_ height: Float) {
            ensureIsContextIsCurrent()
            context_rect(canvas.context, x, y, width, height)
        }
        
        public func fill() {
            ensureIsContextIsCurrent()
            context_fill(canvas.context, 0, FillRule(rawValue: TNSFillRule.NonZero.rawValue))
            canvas.doDraw()
        }
        
        public func fill(value: Any){
            if let rule = value as? TNSFillRule {
                fill(rule: rule)
            }else if let rule = value as? TNSPath2D {
                fill(path: rule)
            }
        }
        
        @nonobjc func fill(rule: TNSFillRule) {
            ensureIsContextIsCurrent()
            context_fill(canvas.context,0, FillRule(rawValue: rule.rawValue))
            canvas.doDraw()
        }
        
        @nonobjc func fill(path: TNSPath2D) {
            ensureIsContextIsCurrent()
            context_fill(canvas.context,path.path, FillRule(rawValue: TNSFillRule.NonZero.rawValue))
            canvas.doDraw()
        }
        
        public func fill(_ path: TNSPath2D,_ rule: TNSFillRule) {
            ensureIsContextIsCurrent()
            context_fill(canvas.context,path.path,FillRule(rawValue: rule.rawValue))
            canvas.doDraw()
        }
        
        public func stroke() {
           stroke(nil)
        }
        
        public func stroke(_ path: TNSPath2D?) {
            ensureIsContextIsCurrent()
            if path == nil {
                context_stroke(canvas.context,0)
            }else {
                context_stroke(canvas.context,path!.path)
            }
            canvas.doDraw()
        }
        
        public func beginPath() {
            ensureIsContextIsCurrent()
            context_begin_path(canvas.context)
        }
        
        public func moveTo(_ x: Float,_ y: Float) {
            ensureIsContextIsCurrent()
            context_move_to(canvas.context, x, y)
        }
        
        public func lineTo(_ x: Float,_ y: Float) {
            ensureIsContextIsCurrent()
            context_line_to(canvas.context, x, y)
        }
        
        public func closePath() {
            ensureIsContextIsCurrent()
            context_close_path(canvas.context)
        }
        
        public func arc(_ x: Float,_ y: Float,_ radius: Float,_ startAngle: Float,_ endAngle: Float) {
            arc(x, y, radius, startAngle, endAngle, false);
        }
        
        public func arc(_ x: Float,_ y: Float,_ radius: Float,_ startAngle: Float,_ endAngle: Float,_ anticlockwise: Bool) {
            ensureIsContextIsCurrent()
            context_arc(canvas.context, x, y, radius, startAngle, endAngle, anticlockwise)
        }
        
        public func arcTo(_ x1: Float,_  y1: Float,_ x2: Float,_ y2: Float,_ radius: Float) {
            ensureIsContextIsCurrent()
            context_arc_to(canvas.context, x1, y1, x2, y2, radius)
        }
        
        public func bezierCurveTo(_ cp1x: Float,_ cp1y: Float,_ cp2x: Float,_ cp2y: Float,_ x: Float,_ y: Float) {
            ensureIsContextIsCurrent()
            context_bezier_curve_to(canvas.context, cp1x, cp1y, cp2x, cp2y, x, y)
        }
        
        public func ellipse(_ x: Float,_ y: Float,_ radiusX: Float,_ radiusY: Float,_ rotation: Float,_ startAngle: Float,_ endAngle: Float) {
            ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, false)
        }
        
        public func ellipse(_ x: Float,_  y: Float,_ radiusX: Float,_ radiusY: Float,_ rotation: Float,_ startAngle: Float,_ endAngle: Float,_ anticlockwise: Bool) {
            ensureIsContextIsCurrent()
            context_ellipse(canvas.context, x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise)
        }
        
        public func clip(){
            ensureIsContextIsCurrent()
            context_clip_rule(canvas.context, FillRule(rawValue: TNSFillRule.NonZero.rawValue))
        }
        
        public func clip(_ value: Any){
            if let rule = value as? TNSFillRule {
                clip(rule: rule)
            }else if let path = value as? TNSPath2D {
                clip(path: path)
            }
        }
        @nonobjc func clip(rule: TNSFillRule) {
            ensureIsContextIsCurrent()
            context_clip_rule(canvas.context, FillRule(rawValue: rule.rawValue))
        }
        
        @nonobjc func clip(path: TNSPath2D) {
            ensureIsContextIsCurrent()
            context_clip(canvas.context,path.path,FillRule(rawValue: TNSFillRule.NonZero.rawValue))
        }
        
        
        public func clip(_ path:TNSPath2D ,_  rule: TNSFillRule ) {
            ensureIsContextIsCurrent()
            context_clip(canvas.context,path.path,FillRule(rawValue: rule.rawValue))
        }
        
    
        public func clearRect(_ x: Float, _ y: Float,_ width: Float,_ height: Float){
            ensureIsContextIsCurrent()
            context_clear_rect(canvas.context, x, y, width, height)
            canvas.doDraw()
            
        }
        
        public func setLineDash(_ segments: [Float32]){
            var array: [Float32] = []
            array.append(contentsOf: segments)
            context_set_line_dash(canvas.context, &array, UInt(segments.count))
        }
        
        public func getCanvas() -> TNSCanvas {
            return canvas
        }
        
        
        public func createLinearGradient(_ x0: Float,_ y0: Float,_ x1: Float,_ y1: Float) -> TNSColorStyle.TNSLinearGradient {
            return TNSColorStyle.TNSLinearGradient(context_create_linear_gradient(canvas.context,x0,y0,x1, y1))
        }
        
        public func createRadialGradient(_ x0: Float,_ y0: Float,_ r0: Float,_ x1: Float,_ y1: Float,_ r1: Float) -> TNSColorStyle.TNSRadialGradient {
            return TNSColorStyle.TNSRadialGradient(context_create_radial_gradient(canvas.context,x0,y0,r0,x1,y1, r1))
        }
        
        
        public func createPattern(_ value: Any,_  repetition: TNSPatternRepetition) -> Any? {
            if let canvas = value as? TNSCanvas {
                return TNSColorStyle.TNSPattern(context: canvas.context,canvas: canvas, pattern: repetition)
            }else if let src = value as? UIImage {
                 return TNSColorStyle.TNSPattern(context: canvas.context, src: src, pattern: repetition)
            }else if let asset = value as? TNSImageAsset {
                return TNSColorStyle.TNSPattern(context: canvas.context,asset: asset, pattern: repetition)
            }else {
                return NSNull()
            }
            
        }
        
        
        @nonobjc func createPattern(canvas: TNSCanvas,repetition: TNSPatternRepetition) -> TNSColorStyle.TNSPattern? {
            return TNSColorStyle.TNSPattern(context: canvas.context,canvas: canvas, pattern: repetition)
        }
        
        @nonobjc func createPattern(src: UIImage,repetition: TNSPatternRepetition) -> TNSColorStyle.TNSPattern? {
            return TNSColorStyle.TNSPattern(context: canvas.context,src: src, pattern: repetition)
        }
        
        @nonobjc func createPattern(src: TNSImageAsset,repetition: TNSPatternRepetition) -> TNSColorStyle.TNSPattern? {
            return TNSColorStyle.TNSPattern(context: canvas.context,asset: src, pattern: repetition)
        }
        
        
        public func setTransform(_ a: Float,_ b: Float,_ c: Float,_ d: Float, _ e: Float,_ f: Float){
            ensureIsContextIsCurrent()
            context_set_transform(canvas.context, a, b, c, d, e, f)
            
        }
        
        public func scale(_ x: Float,_ y: Float){
            ensureIsContextIsCurrent()
            context_scale(canvas.context, x, y)
        }
        
        public func rotate(_ angle: Float){
            ensureIsContextIsCurrent()
            context_rotate(canvas.context, angle)
        }
        
        public func translate(_ x: Float,_ y: Float){
            ensureIsContextIsCurrent()
            context_translate(canvas.context, x, y)
        }
        
        public func quadraticCurveTo(_ cpx: Float,_ cpy: Float,_ x: Float,_ y: Float){
            ensureIsContextIsCurrent()
            context_quadratic_curve_to(canvas.context, cpx, cpy, x, y)
        }
        
        public func drawImage(_ image: Any,_ dx: Float, _ dy: Float){
            if let image = image as? TNSImageAsset {
                drawImage(image: image, dx: dx, dy: dy)
            }else if let image = image as? UIImage {
                drawImage(image: image, dx: dx, dy: dy)
            }else if let image = image as? TNSCanvas {
                drawImage(canvas: image, dx: dx, dy: dy)
            }else if let image = image as? TNSImageBitmap {
                drawImage(bitmap: image, dx: dx, dy: dy)
            }
        }
        
        @nonobjc func drawImage(bitmap: TNSImageBitmap, dx: Float, dy: Float){
            ensureIsContextIsCurrent()
            context_draw_image_dx_dy_asset(canvas.context, bitmap.asset, dx, dy)
            canvas.doDraw()
        }
        
        @nonobjc func drawImage(image: TNSImageAsset, dx: Float, dy: Float){
            ensureIsContextIsCurrent()
            context_draw_image_dx_dy_asset(canvas.context, image.asset, dx, dy)
            canvas.doDraw()
        }
        
        @nonobjc func drawImage(image: UIImage, dx: Float, dy: Float){
            ensureIsContextIsCurrent()
            let data = image.pngData()
            if data != nil {
                var byteArray = [UInt8](data!)
                context_draw_image_encoded_dx_dy(canvas.context, &byteArray, UInt(data!.count),dx, dy)
                canvas.doDraw()
            }
        }
        
    
        
        @nonobjc func drawImage(canvas : TNSCanvas, dx: Float,dy: Float){
                   var ss = canvas.snapshot()
                   ensureIsContextIsCurrent()
            context_draw_image_dx_dy(self.canvas.context, &ss, UInt(ss.count), Float32(canvas.renderer.drawingBufferWidth), Float32(canvas.renderer.drawingBufferHeight),dx, dy)
                   self.canvas.doDraw()
        }
        
        
        public func drawImage(_ image: Any, _ dx: Float,_ dy: Float,_ dWidth: Float,_ dHeight: Float){
            if let image = image as? TNSImageAsset {
                drawImage(image: image, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? UIImage {
                drawImage(image: image, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? TNSCanvas {
                drawImage(canvas: image, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? TNSImageBitmap {
                drawImage(bitmap: image, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }
        }
        
        @nonobjc func drawImage(bitmap: TNSImageBitmap, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            ensureIsContextIsCurrent()
            context_draw_image_dx_dy_dw_dh_asset(canvas.context, bitmap.asset, dx, dy,dWidth,dHeight)
            canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(image: TNSImageAsset, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            ensureIsContextIsCurrent()
            context_draw_image_dx_dy_dw_dh_asset(canvas.context, image.asset, dx, dy,dWidth,dHeight)
            canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(canvas: TNSCanvas, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            ensureIsContextIsCurrent()
            var ss = canvas.snapshot()
            self.canvas.renderer.ensureIsContextIsCurrent()
            context_draw_image_dx_dy_dw_dh(self.canvas.context, &ss, UInt(ss.count), Float32(canvas.renderer.drawingBufferWidth), Float32(canvas.renderer.drawingBufferHeight),dx, dy, dWidth, dHeight)
            self.canvas.doDraw()
        }
        

        @nonobjc func drawImage(image: UIImage, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            ensureIsContextIsCurrent()
            let data = image.pngData()
            if data != nil {
                var byteArray = [UInt8](data!)
                context_draw_image_encoded_dx_dy_dw_dh(canvas.context, &byteArray, UInt(data!.count),dx, dy,dWidth,dHeight)
                canvas.doDraw()
            }
        }
        
        
        public func drawImage(_ image: Any,_ sx: Float,_ sy: Float,_ sWidth: Float,_ sHeight: Float,_ dx: Float,_ dy: Float,_ dWidth: Float,_ dHeight: Float){
            if let image = image as? TNSImageAsset {
                drawImage(image: image, sx: sx, sy: sy, sWidth: sWidth, sHeight: sHeight, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? UIImage {
                drawImage(image: image, sx: sx, sy: sy, sWidth: sWidth, sHeight: sHeight, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? TNSCanvas {
                drawImage(canvas: image, sx: sx, sy: sy, sWidth: sWidth, sHeight: sHeight, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? TNSImageBitmap {
                drawImage(bitmap: image, sx: sx, sy: sy, sWidth: sWidth, sHeight: sHeight, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }
        }
        
        @nonobjc func drawImage(image: TNSImageAsset, sx: Float, sy: Float, sWidth: Float, sHeight: Float, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            ensureIsContextIsCurrent()
            context_draw_image_asset(canvas.context, image.asset,sx, sy,sWidth, sHeight, dx, dy, dWidth, dHeight)
            canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(bitmap: TNSImageBitmap, sx: Float, sy: Float, sWidth: Float, sHeight: Float, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            ensureIsContextIsCurrent()
            context_draw_image_asset(canvas.context, bitmap.asset,sx, sy,sWidth, sHeight, dx, dy, dWidth, dHeight)
            canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(canvas: TNSCanvas, sx: Float, sy: Float, sWidth: Float, sHeight: Float, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            var ss = canvas.snapshot()
           ensureIsContextIsCurrent()
            context_draw_image(self.canvas.context, &ss, UInt(ss.count), Float32(canvas.renderer.width),Float32(canvas.renderer.height), sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight)
            self.canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(image: UIImage, sx: Float, sy: Float, sWidth: Float, sHeight: Float, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            ensureIsContextIsCurrent()
            let data = image.pngData()
            if data != nil {
                var byteArray = [UInt8](data!)
                context_draw_image_encoded(canvas.context, &byteArray, UInt(data!.count), sx, sy,sWidth, sHeight, dx, dy, dWidth, dHeight)
                canvas.doDraw()
            }
            
        }
        
        public func createImageData(_ width: Int32,_ height: Int32)-> TNSImageData {
            return TNSImageData(width: width, height: height)
        }
        
        public func createImageData(_ imageData: TNSImageData)-> TNSImageData{
            return TNSImageData(width: imageData.width, height: imageData.height)
        }
        
        public func getImageData(_ sx: Float,_ sy: Float, _ sw: Float,_ sh:Float) -> TNSImageData {
            ensureIsContextIsCurrent()
           return TNSImageData(imageData: context_get_image_data(canvas.context, sx, sy, sw, sh))
        }
        
        
        public func putImageData(_ imageData: TNSImageData,_ dx: Float,_ dy: Float){
            putImageData(imageData, dx, dy,  0,  0,  0, 0)
        }
        
        public func putImageData(_ imageData: TNSImageData,_ dx: Float,_ dy: Float,_ dirtyX: Float,_ dirtyY: Float,_ dirtyWidth:Float32,_ dirtyHeight: Float32){
            ensureIsContextIsCurrent()
  
            context_put_image_data(canvas.context,imageData.imageData , dx, dy, dirtyX, dirtyY, dirtyWidth, dirtyHeight)
            canvas.doDraw()
        }
        
        public func getLineDash() -> [Float32]{
            let dash = context_get_line_dash(canvas.context)
            if dash != nil {
                let ptr = dash!.pointee
                let buffer = UnsafeBufferPointer(start: ptr.data, count: Int(ptr.data_len))
                destroy_f32_array(dash)
                return Array(buffer)
            }
            return []
        }
        
        public func save() {
            ensureIsContextIsCurrent()
            context_save(canvas.context)
        }
        
        public func restore() {
            ensureIsContextIsCurrent()
            context_restore(canvas.context)
        }
        
        public func measureText(_ text: String) -> TNSTextMetrics {
            ensureIsContextIsCurrent()
            return TNSTextMetrics(metrics: context_measure_text(canvas.context, text))
        }
        public func resetTransform(){
            ensureIsContextIsCurrent()
            context_reset_transform(canvas.context)
        }
        
        public func transform(_ a: Float,_ b: Float,_ c: Float,_ d: Float,_ e: Float,_ f: Float){
            ensureIsContextIsCurrent()
            context_transform(canvas.context, a, b, c, d, e, f)
        }
        
        public func isPointInPath(_ x: Float,_ y: Float) -> Bool {
            return context_is_point_in_path(canvas.context,0, x, y, FillRule(rawValue: TNSFillRule.NonZero.rawValue))
        }
        
        public func isPointInPath(_ x: Float,_ y: Float,_ fillRule: TNSFillRule) -> Bool {
            return context_is_point_in_path(canvas.context,0, x, y, FillRule(rawValue: fillRule.rawValue))
        }
        
        public func isPointInPath(_ path: TNSPath2D, _ x: Float,_ y: Float,_ fillRule: TNSFillRule) -> Bool {
            return context_is_point_in_path(canvas.context, path.path, x, y, FillRule(rawValue: fillRule.rawValue))
        }
        
        public func isPointInStroke(_ x: Float,_ y: Float) -> Bool {
            return context_is_point_in_stroke(canvas.context, 0, x, y)
        }
        
        public func isPointInStroke(_ path: TNSPath2D, _ x:Float,_ y: Float) -> Bool {
            return context_is_point_in_stroke(canvas.context, path.path, x, y)
        }
    }
