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
        private var _fillStyle: Any = TNSColorStyle.Color(color: .black)
        private var _strokeStyle: Any = TNSColorStyle.Color(color: .black)
        private var _lineWidth: Float = 1
        private var _globalCompositeOperation = TNSCompositeOperationType.SourceOver
        private var _font = "10px sans-serif"
        private var _globalAlpha:Float = 1
        
        func ensureIsContextIsCurrent() -> Bool {
            return canvas.renderer.ensureIsContextIsCurrent()
        }
        public init(_ canvas: TNSCanvas) {
            self.canvas = canvas
            super.init()
            let _ = (canvas.renderer as? GLRenderer)?.ensureIsContextIsCurrent()
        }
        
        func reset(){
            _fillStyle = TNSColorStyle.Color(color: .black)
            _strokeStyle = TNSColorStyle.Color(color: .black)
            _lineWidth = 1
            _globalCompositeOperation = .SourceOver
            _font = "10px sans-serif"
            _globalAlpha = 1
            _imageSmoothingEnabled = false
            _imageSmoothingQuality = .Low
            _lineCap = .Butt
            _lineDashOffset = 0
            _lineJoin = .Miter
            _miterLimit = 10
            _shadowBlur = 0
            _shadowColor = UIColor.clear
            _shadowOffsetX = 0
            _shadowOffsetY = 0
            _textAlign = .Left
            _fillStyle = TNSColorStyle.Color(color: .black)
            _strokeStyle = TNSColorStyle.Color(color: .black)
        }
        
        public var currentTransform: TNSDOMMatrix {
            get {
                let matrix = native_get_current_transform(canvas.canvas)
                if(matrix == 0){
                    return TNSDOMMatrix()
                }
                return TNSDOMMatrix(matrix: matrix)
            }
            set {
                native_set_current_transform(canvas.canvas, newValue.matrix)
            }
        }
        
        public var font: String {
            get {
                return _font
            }
            set {
                canvas.canvas = native_set_font(canvas.canvas, newValue)
                _font = newValue
            }
        }
        
        public var globalAlpha: Float {
            get{
                return _globalAlpha
            }
            set {
                var val = newValue
                if(val > 1.0){
                    val = 1.0
                }
                canvas.canvas = native_set_global_alpha(canvas.canvas, UInt8(val * 255))
                _globalAlpha = val
            }
        }
        
        public var globalCompositeOperation:TNSCompositeOperationType {
            set {
                canvas.canvas = native_set_global_composite_operation(canvas.canvas, newValue.rawValue)
                _globalCompositeOperation = newValue
            }
            get {
                return _globalCompositeOperation
            }
        }
        private var _imageSmoothingEnabled: Bool = false
        public var imageSmoothingEnabled: Bool {
            get {
                return _imageSmoothingEnabled
            }
            set {
                canvas.canvas = native_image_smoothing_enabled(canvas.canvas, newValue)
                _imageSmoothingEnabled = newValue
            }
        }
        
        private var _imageSmoothingQuality: TNSImageSmoothingQuality = .Low
        public var imageSmoothingQuality: TNSImageSmoothingQuality {
            get {
                return _imageSmoothingQuality
            }
            set {
                let val = newValue.rawValue
                
                canvas.canvas = native_image_smoothing_quality(canvas.canvas, val)
            }
        }
        
        private var _lineCap = TNSLineCap.Butt
        public var lineCap: TNSLineCap {
            get {
                return _lineCap
            }
            set {
                canvas.canvas = native_set_line_cap(canvas.canvas, newValue.rawValue)
                _lineCap = newValue
            }
        }
        
        private var _lineDashOffset: Float = 0
        
        public var lineDashOffset: Float {
            get {
                return _lineDashOffset
            }
            set {
                canvas.canvas = native_line_dash_offset(canvas.canvas, newValue)
                _lineDashOffset = newValue
            }
        }
        public var lineWidth: Float {
            get {
                return _lineWidth
            }
            set {
                canvas.canvas =  native_set_line_width(canvas.canvas, newValue)
                _lineWidth = newValue
            }
        }
        
        private var _lineJoin: TNSLineJoin = .Miter
        public var lineJoin: TNSLineJoin {
            get {
                return _lineJoin
            }
            set {
                canvas.canvas = native_line_join(canvas.canvas, newValue.rawValue)
                _lineJoin = newValue
            }
        }
        private var _miterLimit: Float = 10
        
        public var miterLimit: Float {
            get {
                return _miterLimit
            }
            set {
                canvas.canvas = native_miter_limit(canvas.canvas, newValue)
                _miterLimit = newValue
            }
        }
        
        private var _shadowBlur: Float = 0
        public var shadowBlur: Float {
            get {
                return _shadowBlur
            }
            set {
                canvas.canvas = native_shadow_blur(canvas.canvas, newValue)
                _shadowBlur = newValue
            }
        }
        private var _shadowColor: Any = UIColor.clear
        public var shadowColor: Any {
            get {
                return _shadowColor
            }
            set {
                if let color = newValue as? UIColor {
                    canvas.canvas = native_shadow_color(canvas.canvas, UInt32(color.colorCode))
                    _shadowColor = newValue
                }else if let colorCode = newValue as? UInt32 {
                    canvas.canvas = native_shadow_color(canvas.canvas, colorCode)
                    _shadowColor = newValue
                }
            }
        }
        private var _shadowOffsetX: Float = 0
        public var shadowOffsetX: Float{
            get {
                return _shadowOffsetX
            }
            set {
                canvas.canvas = native_shadow_offset_x(canvas.canvas, newValue)
                _shadowOffsetX = newValue
            }
        }
        
        private var _shadowOffsetY: Float = 0
        public var shadowOffsetY: Float {
            get {
                return _shadowOffsetY
            }
            set {
                canvas.canvas = native_shadow_offset_y(canvas.canvas, newValue)
                _shadowOffsetY = newValue
            }
        }
        
        
        
        private var _textAlign: TNSTextAlignment = TNSTextAlignment.Left
        public var textAlign: TNSTextAlignment{
            get {
                return _textAlign
            }
            set {
                var val = newValue.rawValue
                switch newValue {
                case .Start:
                    val = "left"
                case .End:
                    val = "right"
                default: break
                }
                canvas.canvas = native_text_align(canvas.canvas, val)
                _textAlign = newValue
            }
        }
        
        public var fillStyle: Any {
            get {
                return _fillStyle
            }
            set {
                if let style = newValue as? ICanvasColorStyle {
                    switch style.getStyleType() {
                    case .Color:
                        let fill = newValue as! TNSColorStyle.Color
                        if let current = _fillStyle as? TNSColorStyle.Color{
                            if(current.color.isSame(fill.color)){
                                return
                            }
                        }
                        if(!fill.color.isCached){
                            fill.color.cacheColor()
                        }
                        canvas.canvas = native_set_fill_color_rgba(canvas.canvas, fill.color.red, fill.color.green, fill.color.blue, fill.color.alpha)
                        _fillStyle = fill
                    case .Gradient:
                        if let isLinear = newValue as? TNSColorStyle.TNSLinearGradient {
                            canvas.canvas = native_set_fill_gradient_linear(canvas.canvas,
                                                                            isLinear.x0,
                                                                            isLinear.y0,
                                                                            isLinear.x1,
                                                                            isLinear.y1,
                                                                            isLinear.getColors().count,
                                                                            isLinear.getColors(),
                                                                            isLinear.getPostions().count,
                                                                            isLinear.getPostions())
                            _fillStyle = isLinear
                        }
                        if let isRadial = newValue as? TNSColorStyle.TNSRadialGradient {
                            canvas.canvas = native_set_fill_gradient_radial(
                                canvas.canvas,
                                isRadial.x0,
                                isRadial.y0,
                                isRadial.r0,
                                isRadial.x1,
                                isRadial.y1,
                                isRadial.r1,
                                isRadial.getColors().count,
                                isRadial.getColors(),
                                isRadial.getPostions().count,
                                isRadial.getPostions()
                            )
                            _fillStyle = isRadial
                        }
                    case .Pattern:
                        if let pattern = newValue as? TNSColorStyle.TNSPattern{
                            canvas.canvas = native_set_fill_pattern(canvas.canvas, pattern.nativePattern)
                            _fillStyle = pattern
                        }
                    }
                }else if let color = newValue as? UInt32 {
                    canvas.canvas = native_set_fill_color(canvas.canvas, color)
                    _fillStyle = color
                } else if let color = newValue as? String {
                    let c = UIColor(fromString: color)
                    canvas.canvas = native_set_fill_color_rgba(canvas.canvas, c.alpha, c.green, c.blue, c.alpha)
                    _fillStyle = color
                }
            }
        }
        
        
        
        
        public var strokeStyle: Any {
            get {
                return _strokeStyle
            }
            set {
                if let color = newValue as? UInt32 {
                    canvas.canvas = native_set_stroke_color(canvas.canvas, color)
                } else if let color = newValue as? String {
                    let c = UIColor(fromString: color)
                    canvas.canvas = native_set_stroke_color_rgba(canvas.canvas, c.alpha, c.green, c.blue, c.alpha)
                    _fillStyle = color
                }else if let style = newValue as? ICanvasColorStyle {
                    switch style.getStyleType() {
                    case .Color:
                        let fill = newValue as! TNSColorStyle.Color
                        if let current = _strokeStyle as? TNSColorStyle.Color{
                            if(current.color.isSame(fill.color)){
                                return
                            }
                        }
                        if(!fill.color.isCached){
                            fill.color.cacheColor()
                        }
                        canvas.canvas = native_set_stroke_color_rgba(canvas.canvas, fill.color.red, fill.color.green, fill.color.blue, fill.color.alpha)
                        _strokeStyle = newValue
                    case .Gradient:
                        let isLinear = newValue as? TNSColorStyle.TNSLinearGradient
                        let isRadial = newValue as? TNSColorStyle.TNSRadialGradient
                        if isLinear != nil {
                            canvas.canvas = native_set_stroke_gradient_linear(canvas.canvas,
                                                                              isLinear!.x0,
                                                                              isLinear!.y0,
                                                                              isLinear!.x1,
                                                                              isLinear!.y1,
                                                                              (isLinear?.getColors().count)!,
                                                                              isLinear?.getColors(),
                                                                              (isLinear?.getPostions().count)!,
                                                                              isLinear?.getPostions())
                        }else if isRadial != nil{
                            canvas.canvas = native_set_stroke_gradient_radial(
                                canvas.canvas,
                                isRadial!.x0,
                                isRadial!.y0,
                                isRadial!.r0,
                                isRadial!.x1,
                                isRadial!.y1,
                                isRadial!.r1,
                                (isRadial?.getColors().count)!,
                                isRadial?.getColors(),
                                (isRadial?.getPostions().count)!,
                                isRadial?.getPostions()
                            )
                        }
                        _strokeStyle = newValue
                    case .Pattern:
                        let pattern = newValue as! TNSColorStyle.TNSPattern
                        canvas.canvas = native_set_stroke_pattern(canvas.canvas, pattern.nativePattern)
                        _strokeStyle = newValue
                    }
                }
            }
        }
        
        public func fillRect(_ x: Float,_ y: Float,_ width: Float,_ height: Float) {
            let _ = ensureIsContextIsCurrent()
            self.canvas.canvas = native_fill_rect(self.canvas.canvas, x, y, width, height, self.canvas.getViewPtr())
            self.canvas.doDraw()
        }
        
        public func strokeRect(_ x: Float,_ y: Float,_ width: Float,_ height: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_stroke_rect(canvas.canvas, x, y, width, height,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func fillText(_ text: String,_ x: Float,_ y:Float) {
            let _ = ensureIsContextIsCurrent()
            fillText(text, x, y, 0)
        }
        
        public func fillText(_ text: String,_ x: Float,_ y:Float,_ width: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_fill_text(canvas.canvas, text, x, y, width,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func strokeText(_ text: String,_ x: Float,_ y:Float) {
            let _ = ensureIsContextIsCurrent()
            strokeText(text, x, y, 0)
        }
        
        public func strokeText(_ text: String,_ x: Float,_ y:Float,_ width: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_stroke_text(canvas.canvas, text, x, y, width,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func rect(_ x: Float,_ y: Float,_ width: Float,_ height: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_rect(canvas.canvas, x, y, width, height)
        }
        
        public func fill() {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_fill(canvas.canvas,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func fill(value: Any){
            if let rule = value as? String {
                fill(rule: rule)
            }else if let rule = value as? TNSPath2D {
                fill(path: rule)
            }
        }
        
        @nonobjc func fill(rule: String) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_fill_rule(canvas.canvas,rule,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        @nonobjc func fill(path: TNSPath2D) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_fill_path_rule(canvas.canvas,path.path,"",self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func fill(_ path: TNSPath2D,_ rule: String) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_fill_path_rule(canvas.canvas,path.path,rule,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func stroke() {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_stroke(canvas.canvas,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func stroke(_ path: TNSPath2D) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_stroke_path(canvas.canvas,path.path,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        public func beginPath() {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_begin_path(canvas.canvas)
        }
        
        public func moveTo(_ x: Float,_ y: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_move_to(canvas.canvas, x, y)
        }
        
        public func lineTo(_ x: Float,_ y: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_line_to(canvas.canvas, x, y)
        }
        
        public func closePath() {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_close_path(canvas.canvas)
        }
        
        public func arc(_ x: Float,_ y: Float,_ radius: Float,_ startAngle: Float,_ endAngle: Float) {
            let _ = ensureIsContextIsCurrent()
            arc(x, y, radius, startAngle, endAngle, false);
        }
        
        public func arc(_ x: Float,_ y: Float,_ radius: Float,_ startAngle: Float,_ endAngle: Float,_ anticlockwise: Bool) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_arc(canvas.canvas, x, y, radius, startAngle, endAngle, anticlockwise)
        }
        
        public func arcTo(_ x1: Float,_  y1: Float,_ x2: Float,_ y2: Float,_ radius: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_arc_to(canvas.canvas, x1, y1, x2, y2, radius)
        }
        
        public func bezierCurveTo(_ cp1x: Float,_ cp1y: Float,_ cp2x: Float,_ cp2y: Float,_ x: Float,_ y: Float) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_bezier_curve_to(canvas.canvas, cp1x, cp1y, cp2x, cp2y, x, y)
        }
        
        public func ellipse(_ x: Float,_ y: Float,_ radiusX: Float,_ radiusY: Float,_ rotation: Float,_ startAngle: Float,_ endAngle: Float) {
            let _ = ensureIsContextIsCurrent()
            ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, false)
        }
        
        public func ellipse(_ x: Float,_  y: Float,_ radiusX: Float,_ radiusY: Float,_ rotation: Float,_ startAngle: Float,_ endAngle: Float,_ anticlockwise: Bool) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_ellipse(canvas.canvas, x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise)
        }
        
        public func clip(){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_clip(canvas.canvas,self.canvas.getViewPtr())
        }
        
        public func clip(_ value: Any){
            if let rule = value as? String{
                clip(rule: rule)
            }else if let rule = value as? TNSPath2D{
                clip(path: rule)
            }
        }
        @nonobjc func clip(rule: String) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_clip_rule(canvas.canvas, rule,self.canvas.getViewPtr())
        }
        
        @nonobjc func clip(path:TNSPath2D) {
            clip(path, "nonzero")
        }
        
        
        public func clip(_ path:TNSPath2D ,_  rule: String ) {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_clip_path_rule(canvas.canvas, path.path,rule,self.canvas.getViewPtr())
        }
        
    
        public func clearRect(_ x: Float, _ y: Float,_ width: Float,_ height: Float){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_clear_rect(canvas.canvas, x, y, width, height,self.canvas.getViewPtr())
            canvas.doDraw()
            
        }
        private var _lineDashSegments: [Float32] = []
        
        public func setLineDash(_ segments: [Float32]){
            var array: [Float32] = []
            array.append(contentsOf: segments)
            canvas.canvas = native_set_line_dash(canvas.canvas, segments.count, &array)
            _lineDashSegments = segments
        }
        
        public func getCanvas() -> TNSCanvas {
            return canvas
        }
        
        
        public func createLinearGradient(_ x0: Float,_ y0: Float,_ x1: Float,_ y1: Float) -> TNSColorStyle.TNSLinearGradient {
            return TNSColorStyle.TNSLinearGradient(x0: x0, y0: y0, x1: x1, y1: y1);
        }
        
        public func createRadialGradient(_ x0: Float,_ y0: Float,_ r0: Float,_ x1: Float,_ y1: Float,_ r1: Float) -> TNSColorStyle.TNSRadialGradient {
            return TNSColorStyle.TNSRadialGradient(x0: x0, y0: y0, r0: r0, x1: x1, y1: y1, r1: r1);
        }
        
        
        public func createPattern(_ value: Any,_  repetition: TNSPatternRepetition) -> Any {
            if let canvas = value as? TNSCanvas {
                return TNSColorStyle.TNSPattern(canvas: canvas, pattern: repetition)
            }else if let src = value as? UIImage {
                 return TNSColorStyle.TNSPattern(src: src, pattern: repetition)
            }else if let asset = value as? TNSImageAsset {
                return TNSColorStyle.TNSPattern(asset: asset, pattern: repetition)
            }else {
                return NSNull()
            }
            
        }
        
        
        @nonobjc func createPattern(canvas: TNSCanvas,repetition: TNSPatternRepetition) -> TNSColorStyle.TNSPattern {
            return TNSColorStyle.TNSPattern(canvas: canvas, pattern: repetition)
        }
        
        @nonobjc func createPattern(src: UIImage,repetition: TNSPatternRepetition) -> TNSColorStyle.TNSPattern {
            return TNSColorStyle.TNSPattern(src: src, pattern: repetition)
        }
        
        @nonobjc func createPattern(src: TNSImageAsset,repetition: TNSPatternRepetition) -> TNSColorStyle.TNSPattern {
            return TNSColorStyle.TNSPattern(asset: src, pattern: repetition)
        }
        
        
        public func setTransform(_ a: Float,_ b: Float,_ c: Float,_ d: Float, _ e: Float,_ f: Float){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_set_transform(canvas.canvas, a, b, c, d, e, f,self.canvas.getViewPtr())
            
        }
        
        public func scale(_ x: Float,_ y: Float){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_scale(canvas.canvas, x, y,self.canvas.getViewPtr())
        }
        
        public func rotate(_ angle: Float){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_rotate(canvas.canvas, angle,self.canvas.getViewPtr())
        }
        
        public func translate(_ x: Float,_ y: Float){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_translate(canvas.canvas, x, y,self.canvas.getViewPtr())
        }
        
        public func quadraticCurveTo(_ cpx: Float,_ cpy: Float,_ x: Float,_ y: Float){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_quadratic_curve_to(canvas.canvas, cpx, cpy, x, y)
        }
        
        public func drawImage(_ image: Any,_ dx: Float, _ dy: Float){
            if let image = image as? TNSImageAsset {
                drawImage(image: image, dx: dx, dy: dy)
            }else if let image = image as? UIImage {
                drawImage(image: image, dx: dx, dy: dy)
            }else if let image = image as? TNSCanvas {
                drawImage(canvas: image, dx: dx, dy: dy)
            }
        }
        
        @nonobjc func drawImage(image: TNSImageAsset, dx: Float, dy: Float){
            let _ = ensureIsContextIsCurrent()
            let size = image.width * image.height * 4
            let width = image.width
            let height = image.height
            canvas.canvas = native_draw_image_raw(canvas.canvas, image.getRawBytes(), Int(size), Int32(width),Int32(height), dx, dy,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        @nonobjc func drawImage(image: UIImage, dx: Float, dy: Float){
            let _ = ensureIsContextIsCurrent()
            let cgRef = image.cgImage
            var data = image.pngData() ?? Data()
            let width = cgRef?.width ?? 0
            let height = cgRef?.height ?? 0
            let bitsPerComponent = cgRef?.bitsPerComponent ?? 0
            let bytesPerRow = Int(width * 4)
            let colorSpace = CGColorSpaceCreateDeviceRGB()
            let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipFirst.rawValue)
            let context = CGContext(data: &data, width:width, height: height, bitsPerComponent: bitsPerComponent, bytesPerRow: bytesPerRow, space: colorSpace, bitmapInfo: bitmapInfo.rawValue)
            if context != nil {
                
                /*let count = data.count / MemoryLayout<UInt8>.size
                 // create an array of Uint8
                 var byteArray = [UInt8](repeating: 0, count: count)
                 // copy bytes into array
                 
                 data.copyBytes(to: &byteArray, count: count)
                 */
                var byteArray = [UInt8](data)
                canvas.canvas = native_draw_image(canvas.canvas, &byteArray, data.count, Int32(width),Int32(height), dx, dy,self.canvas.getViewPtr())
                canvas.doDraw()
            }
        }
        
    
        
        
        @nonobjc func drawImage(canvas : TNSCanvas, dx: Float,dy: Float){
                   var ss = canvas.snapshot()
                   let _ = ensureIsContextIsCurrent()
            self.canvas.canvas = native_draw_image(self.canvas.canvas, &ss, ss.count, Int32(canvas.renderer.drawingBufferWidth), Int32(canvas.renderer.drawingBufferHeight), dx, dy, self.canvas.getViewPtr())
                   self.canvas.doDraw()
        }
        
        
        public func drawImage(_ image: Any, _ dx: Float,_ dy: Float,_ dWidth: Float,_ dHeight: Float){
            if let image = image as? TNSImageAsset {
                drawImage(image: image, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? UIImage {
                drawImage(image: image, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }else if let image = image as? TNSCanvas {
                drawImage(canvas: image, dx: dx, dy: dy, dWidth: dWidth, dHeight: dHeight)
            }
        }
        
        
        
        @nonobjc func drawImage(image: TNSImageAsset, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            let _ = ensureIsContextIsCurrent()
            let size = image.width * image.height * 4
            let width = image.width
            let height = image.height
            
            canvas.canvas = native_draw_image_dw_raw(canvas.canvas, image.getRawBytes(), Int(size),Int32(width),Int32(height), dx, dy,dWidth,dHeight,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(canvas: TNSCanvas, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            let _ = ensureIsContextIsCurrent()
            var ss = canvas.snapshot()
            if let gl = self.canvas.renderer as? GLRenderer{
                let _ = gl.ensureIsContextIsCurrent()
            }
            self.canvas.canvas = native_draw_image_dw(self.canvas.canvas, &ss, ss.count, Int32(canvas.renderer.drawingBufferWidth), Int32(canvas.renderer.drawingBufferHeight), dx, dy, dWidth, dHeight, self.canvas.getViewPtr())
            self.canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(image: UIImage, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            let _ = ensureIsContextIsCurrent()
            let cgRef = image.cgImage
            var data = image.pngData() ?? Data()
            let width = cgRef?.width ?? 0
            let height = cgRef?.height ?? 0
            let bitsPerComponent = cgRef?.bitsPerComponent ?? 0
            let bytesPerRow = Int(width * 4)
            let colorSpace = CGColorSpaceCreateDeviceRGB()
            let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipFirst.rawValue)
            let context = CGContext(data: &data, width:width, height: height, bitsPerComponent: bitsPerComponent, bytesPerRow: bytesPerRow, space: colorSpace, bitmapInfo: bitmapInfo.rawValue)
            if context != nil {
                /*let count = data.count / MemoryLayout<UInt8>.size
                 
                 // create an array of Uint8
                 var byteArray = [UInt8](repeating: 0, count: count)
                 // copy bytes into array
                 
                 data.copyBytes(to: &byteArray, count: count)
                 */
                var byteArray = [UInt8](data)
                canvas.canvas = native_draw_image_dw(canvas.canvas, &byteArray, data.count,Int32(width),Int32(height), dx, dy,dWidth,dHeight,self.canvas.getViewPtr())
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
            }
        }
        
        
        @nonobjc func drawImage(image: TNSImageAsset, sx: Float, sy: Float, sWidth: Float, sHeight: Float, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            let _ = ensureIsContextIsCurrent()
            let size = image.width * image.height * 4
            let width = image.width
            let height = image.height
            canvas.canvas = native_draw_image_sw_raw(canvas.canvas, image.getRawBytes(),Int(size), Int32(width), Int32(height), sx, sy,sWidth, sHeight, dx, dy, dWidth, dHeight,self.canvas.getViewPtr())
            canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(canvas: TNSCanvas, sx: Float, sy: Float, sWidth: Float, sHeight: Float, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            var ss = canvas.snapshot()
           let _ = ensureIsContextIsCurrent()
            self.canvas.canvas = native_draw_image_sw(self.canvas.canvas, &ss, ss.count, Int32(canvas.renderer.width), Int32(canvas.renderer.height), sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight, self.canvas.getViewPtr())
            self.canvas.doDraw()
        }
        
        
        @nonobjc func drawImage(image: UIImage, sx: Float, sy: Float, sWidth: Float, sHeight: Float, dx: Float, dy: Float, dWidth: Float, dHeight: Float){
            let _ = ensureIsContextIsCurrent()
            let cgRef = image.cgImage
            var data = image.pngData() ?? Data()
            let width = cgRef?.width ?? 0
            let height = cgRef?.height ?? 0
            let bitsPerComponent = cgRef?.bitsPerComponent ?? 0
            let bytesPerRow = Int(width * 4)
            let colorSpace = CGColorSpaceCreateDeviceRGB()
            let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipFirst.rawValue)
            let context = CGContext(data: &data, width:width, height: height, bitsPerComponent: bitsPerComponent, bytesPerRow: bytesPerRow, space: colorSpace, bitmapInfo: bitmapInfo.rawValue)
            if context != nil {
                var byteArray = [UInt8](data)
                canvas.canvas = native_draw_image_sw(canvas.canvas, &byteArray, data.count, Int32(width), Int32(height), sx, sy,sWidth, sHeight, dx, dy, dWidth, dHeight,self.canvas.getViewPtr())
                canvas.doDraw()
            }
            
        }
        
        public func createImageData(_ width: Int,_ height: Int)-> TNSImageData {
            return TNSImageData(width: width, height: height)
        }
        
        public func createImageData(_ imageData: TNSImageData)-> TNSImageData{
            return TNSImageData(width: imageData.width, height: imageData.height)
        }
        
        public func getImageData(_ sx: Float,_ sy: Float, _ sw:Int,_ sh:Int) -> TNSImageData {
            let _ = ensureIsContextIsCurrent()
            let nativeData = native_get_image_data(canvas.canvas, sx, sy, sw, sh)
            var data = Data()
            if(nativeData != nil){
                let pointer = nativeData!.pointee
                data = Data(bytes: pointer.array, count: pointer.length)
                native_drop_image_data(nativeData)
            }
            return TNSImageData(width:sw , height: sh, data: data)
        }
        
        
        public func putImageData(_ imageData: TNSImageData,_ dx: Float,_ dy: Float){
            putImageData(imageData, dx, dy,  0,  0,  -1, -1)
        }
        
        public func putImageData(_ imageData: TNSImageData,_ dx: Float,_ dy: Float,_ dirtyX: Float,_ dirtyY: Float,_ dirtyWidth:Int,_ dirtyHeight: Int){
            let _ = ensureIsContextIsCurrent()
            /*
             let count = imageData.data.count / MemoryLayout<UInt8>.size
             
             // create an array of Uint8
             var byteArray = [UInt8](repeating: 0, count: count)
             // copy bytes into array
             
             imageData.data.copyBytes(to: &byteArray, count: count)
             
             */
            
            var byteArray = [UInt8](imageData.data)
            canvas.canvas = native_put_image_data(canvas.canvas, imageData.width, imageData.height, &byteArray, byteArray.count, dx, dy, dirtyX, dirtyY, dirtyWidth, dirtyHeight)
            canvas.doDraw()
        }
        
        public func getLineDash() -> [Float32]{
            return _lineDashSegments
        }
        
        public func save() {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_save(canvas.canvas)
        }
        
        public func restore() {
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_restore(canvas.canvas)
        }
        
        public func measureText(_ text: String) -> TNSTextMetrics {
            let _ = ensureIsContextIsCurrent()
            let data = native_measure_text(canvas.canvas, text)
            if(data != nil){
                return TNSTextMetrics(width: data!.pointee.width)
            }
            return TNSTextMetrics(width: 0)
        }
        public func resetTransform(){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_reset_transform(canvas.canvas)
        }
        
        public func transform(_ a: Float,_ b: Float,_ c: Float,_ d: Float,_ e: Float,_ f: Float){
            let _ = ensureIsContextIsCurrent()
            canvas.canvas = native_transform(canvas.canvas, a, b, c, d, e, f,self.canvas.getViewPtr())
        }
        
        public func isPointInPath(_ x: Float,_ y: Float) -> Bool {
            return native_is_point_in_path(canvas.canvas, x, y) == 1
        }
        
        public func isPointInPath(_ x: Float,_ y: Float,_ fillRule: String) -> Bool {
            let ptr = (fillRule as NSString).utf8String
            return native_is_point_in_path_with_rule(canvas.canvas, x, y, ptr) == 1
        }
        
        public func isPointInPath(_ path: TNSPath2D, _ x: Float,_ y: Float,_ fillRule: String) -> Bool {
            let ptr = (fillRule as NSString).utf8String
            return native_is_point_in_path_with_path_rule(canvas.canvas, path.path, x, y, ptr) == 1
        }
        
        public func isPointInStroke(_ x: Float,_ y: Float) -> Bool {
            return native_is_point_in_stroke(canvas.canvas, x, y) == 1
        }
        
        public func isPointInStroke(_ path: TNSPath2D, _ x:Float,_ y: Float) -> Bool {
            return native_is_point_in_stroke_with_path(canvas.canvas, path.path, x, y) == 1
        }
    }
