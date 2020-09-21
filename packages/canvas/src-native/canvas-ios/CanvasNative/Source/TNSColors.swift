//
//  TNSColors.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/27/20.
//

import Foundation
import UIKit
public class TNSColors {
    static func color(string: String) -> UIColor {
        let allColors = [
            "aliceblue": "#F0F8FFFF",
            "antiquewhite": "#FAEBD7FF",
            "aqua": "#00FFFFFF",
            "aquamarine": "#7FFFD4FF",
            "azure": "#F0FFFFFF",
            "beige": "#F5F5DCFF",
            "bisque": "#FFE4C4FF",
            "black": "#000000FF",
            "blanchedalmond": "#FFEBCDFF",
            "blue": "#0000FFFF",
            "blueviolet": "#8A2BE2FF",
            "brown": "#A52A2AFF",
            "burlywood": "#DEB887FF",
            "cadetblue": "#5F9EA0FF",
            "chartreuse": "#7FFF00FF",
            "chocolate": "#D2691EFF",
            "coral": "#FF7F50FF",
            "cornflowerblue": "#6495EDFF",
            "cornsilk": "#FFF8DCFF",
            "crimson": "#DC143CFF",
            "cyan": "#00FFFFFF",
            "darkblue": "#00008BFF",
            "darkcyan": "#008B8BFF",
            "darkgoldenrod": "#B8860BFF",
            "darkgray": "#A9A9A9FF",
            "darkgrey": "#A9A9A9FF",
            "darkgreen": "#006400FF",
            "darkkhaki": "#BDB76BFF",
            "darkmagenta": "#8B008BFF",
            "darkolivegreen": "#556B2FFF",
            "darkorange": "#FF8C00FF",
            "darkorchid": "#9932CCFF",
            "darkred": "#8B0000FF",
            "darksalmon": "#E9967AFF",
            "darkseagreen": "#8FBC8FFF",
            "darkslateblue": "#483D8BFF",
            "darkslategray": "#2F4F4FFF",
            "darkslategrey": "#2F4F4FFF",
            "darkturquoise": "#00CED1FF",
            "darkviolet": "#9400D3FF",
            "deeppink": "#FF1493FF",
            "deepskyblue": "#00BFFFFF",
            "dimgray": "#696969FF",
            "dimgrey": "#696969FF",
            "dodgerblue": "#1E90FFFF",
            "firebrick": "#B22222FF",
            "floralwhite": "#FFFAF0FF",
            "forestgreen": "#228B22FF",
            "fuchsia": "#FF00FFFF",
            "gainsboro": "#DCDCDCFF",
            "ghostwhite": "#F8F8FFFF",
            "gold": "#FFD700FF",
            "goldenrod": "#DAA520FF",
            "gray": "#808080FF",
            "grey": "#808080FF",
            "green": "#008000FF",
            "greenyellow": "#ADFF2FFF",
            "honeydew": "#F0FFF0FF",
            "hotpink": "#FF69B4FF",
            "indianred": "#CD5C5CFF",
            "indigo": "#4B0082FF",
            "ivory": "#FFFFF0FF",
            "khaki": "#F0E68CFF",
            "lavender": "#E6E6FAFF",
            "lavenderblush": "#FFF0F5FF",
            "lawngreen": "#7CFC00FF",
            "lemonchiffon": "#FFFACDFF",
            "lightblue": "#ADD8E6FF",
            "lightcoral": "#F08080FF",
            "lightcyan": "#E0FFFFFF",
            "lightgoldenrodyellow": "#FAFAD2FF",
            "lightgray": "#D3D3D3FF",
            "lightgrey": "#D3D3D3FF",
            "lightgreen": "#90EE90FF",
            "lightpink": "#FFB6C1FF",
            "lightsalmon": "#FFA07AFF",
            "lightseagreen": "#20B2AAFF",
            "lightskyblue": "#87CEFAFF",
            "lightslategray": "#778899FF",
            "lightslategrey": "#778899FF",
            "lightsteelblue": "#B0C4DEFF",
            "lightyellow": "#FFFFE0FF",
            "lime": "#00FF00FF",
            "limegreen": "#32CD32FF",
            "linen": "#FAF0E6FF",
            "magenta": "#FF00FFFF",
            "maroon": "#800000FF",
            "mediumaquamarine": "#66CDAAFF",
            "mediumblue": "#0000CDFF",
            "mediumorchid": "#BA55D3FF",
            "mediumpurple": "#9370D8FF",
            "mediumseagreen": "#3CB371FF",
            "mediumslateblue": "#7B68EEFF",
            "mediumspringgreen": "#00FA9AFF",
            "mediumturquoise": "#48D1CCFF",
            "mediumvioletred": "#C71585FF",
            "midnightblue": "#191970FF",
            "mintcream": "#F5FFFAFF",
            "mistyrose": "#FFE4E1FF",
            "moccasin": "#FFE4B5FF",
            "navajowhite": "#FFDEADFF",
            "navy": "#000080FF",
            "oldlace": "#FDF5E6FF",
            "olive": "#808000FF",
            "olivedrab": "#6B8E23FF",
            "orange": "#FFA500FF",
            "orangered": "#FF4500FF",
            "orchid": "#DA70D6FF",
            "palegoldenrod": "#EEE8AAFF",
            "palegreen": "#98FB98FF",
            "paleturquoise": "#AFEEEEFF",
            "palevioletred": "#D87093FF",
            "papayawhip": "#FFEFD5FF",
            "peachpuff": "#FFDAB9FF",
            "peru": "#CD853FFF",
            "pink": "#FFC0CBFF",
            "plum": "#DDA0DDFF",
            "powderblue": "#B0E0E6FF",
            "purple": "#800080FF",
            "rebeccapurple": "#663399FF",
            "red": "#FF0000FF",
            "rosybrown": "#BC8F8FFF",
            "royalblue": "#4169E1FF",
            "saddlebrown": "#8B4513FF",
            "salmon": "#FA8072FF",
            "sandybrown": "#F4A460FF",
            "seagreen": "#2E8B57FF",
            "seashell": "#FFF5EEFF",
            "sienna": "#A0522DFF",
            "silver": "#C0C0C0FF",
            "skyblue": "#87CEEBFF",
            "slateblue": "#6A5ACDFF",
            "slategray": "#708090FF",
            "slategrey": "#708090FF",
            "snow": "#FFFAFAFF",
            "springgreen": "#00FF7FFF",
            "steelblue": "#4682B4FF",
            "tan": "#D2B48CFF",
            "teal": "#008080FF",
            "thistle": "#D8BFD8FF",
            "tomato": "#FF6347FF",
            "turquoise": "#40E0D0FF",
            "violet": "#EE82EEFF",
            "wheat": "#F5DEB3FF",
            "white": "#FFFFFFFF",
            "whitesmoke": "#F5F5F5FF",
            "yellow": "#FFFF00FF",
            "yellowgreen": "#9ACD32FF"
        ]
        let color  = allColors[string.lowercased()] ?? "#000000FF"
        return UIColor(fromString: color)
    }
}


extension UIColor {
    @objc public convenience init(fromString string: String){
        if(string.starts(with: "#")){
            let r, g, b, a: CGFloat
            let start = string.index(string.startIndex, offsetBy: 1)
            let hexColor = string[start...]
            if hexColor.count == 8 {
                let scanner = Scanner(string: String(hexColor))
                var hexNumber: UInt64 = 0
                
                if scanner.scanHexInt64(&hexNumber) {
                    r = CGFloat((hexNumber & 0xff000000) >> 24) / 255
                    g = CGFloat((hexNumber & 0x00ff0000) >> 16) / 255
                    b = CGFloat((hexNumber & 0x0000ff00) >> 8) / 255
                    a = CGFloat(hexNumber & 0x000000ff) / 255
                    
                    self.init(red: r, green: g, blue: b, alpha: a)
                    return
                }
            }
            
            
            var cString:String = string.trimmingCharacters(in: .whitespacesAndNewlines).uppercased()
            if (cString.hasPrefix("#")) {
                cString.remove(at: cString.startIndex)
            }
            if cString.count == 3 {
                let first = cString[0..<1]
                let second = cString[1..<2]
                let third = cString[2..<3]
                cString =  "" + first +  first  + second  + second  + third  + third
            }
            
            if cString.count == 4 {
                let first = cString[0..<1]
                let second = cString[1..<2]
                let third = cString[2..<3]
                let fourth = cString[3..<4]
                cString =  "" + first +  first  + second  + second  + third  + third + fourth + fourth
            }
            var rgbValue:UInt32 = 0
            Scanner(string: cString).scanHexInt32(&rgbValue)
            self.init(
                red: CGFloat((rgbValue & 0xFF0000) >> 16) / 255.0,
                green: CGFloat((rgbValue & 0x00FF00) >> 8) / 255.0,
                blue: CGFloat(rgbValue & 0x0000FF) / 255.0,
                alpha: CGFloat(1.0)
            )
        }else if(string.starts(with: "rgba")){
            let rgba = string.replacingOccurrences(of: "rgba(", with: "").replacingOccurrences(of: ")", with: "").replacingOccurrences(of: " ", with: "").split(separator: ",")
            let formatter = NumberFormatter()
            let red = CGFloat(Int32(truncating: formatter.number(from: String(rgba[0])) ?? 255)) / 255.0
            let green = CGFloat(Int32(truncating:formatter.number(from: String(rgba[1])) ?? 255)) / 255.0
            let blue = CGFloat(Int32(truncating:formatter.number(from: String(rgba[2])) ?? 255)) / 255.0
            let alpha = CGFloat(truncating: formatter.number(from: String(rgba[3])) ?? 1)
            self.init(red: red, green: green, blue: blue, alpha: alpha)
        }
        else if(string.starts(with: "rgb")){
            let rgba = string.replacingOccurrences(of: "rgb(", with: "").replacingOccurrences(of: ")", with: "").replacingOccurrences(of: " ", with: "").split(separator: ",")
            let formatter = NumberFormatter()
            let red = CGFloat(Int32(truncating: formatter.number(from: String(rgba[0])) ?? 255)) / 255.0
            let green = CGFloat(Int32(truncating:formatter.number(from: String(rgba[1])) ?? 255)) / 255.0
            let blue = CGFloat(Int32(truncating:formatter.number(from: String(rgba[2])) ?? 255)) / 255.0
            let alpha: CGFloat = 1.0
            self.init(red: red, green: green, blue: blue, alpha: alpha)
        }else {
            //self.init()
            var red: CGFloat = 0
            var green: CGFloat = 0
            var blue: CGFloat = 0
            var alpha: CGFloat = 0
            let color = TNSColors.color(string: string)
            color.getRed(&red, green: &green, blue: &blue, alpha: &alpha)
            self.init(red: red, green: green, blue: blue, alpha: alpha)
        }
        setIsCached(false)
        cacheColor()
    }
    
    @objc public convenience init(fromHex hex: String) {
        var cString:String = hex.trimmingCharacters(in: .whitespacesAndNewlines).uppercased()
        if (cString.hasPrefix("#")) {
            cString.remove(at: cString.startIndex)
        }
        if cString.count == 3 {
            let first = cString[0..<1]
            let second = cString[1..<2]
            let third = cString[2..<3]
            cString =  "" + first +  first  + second  + second  + third  + third
        }
        var rgbValue:UInt32 = 0
        Scanner(string: cString).scanHexInt32(&rgbValue)
        self.init(
            red: CGFloat((rgbValue & 0xFF0000) >> 16) / 255.0,
            green: CGFloat((rgbValue & 0x00FF00) >> 8) / 255.0,
            blue: CGFloat(rgbValue & 0x0000FF) / 255.0,
            alpha: CGFloat(1.0)
        )
    }
    
    private struct UIColorProperties{
        static var red: UInt8 = 0
        static var green: UInt8 = 0
        static var blue: UInt8 = 0
        static var alpha: UInt8 = 0
        static var isCached: Bool = false
    }
    
    func isSame(_ color: UIColor) -> Bool {
        if(!isCached){
            cacheColor()
        }
        if(!color.isCached){
            color.cacheColor()
        }
        if(self.red == color.red
            && self.green == color.green
            && self.blue == color.blue
            && self.alpha == color.alpha){
            return true
        }
        return false
    }
    
    func setIsCached(_ cached: Bool) {
        objc_setAssociatedObject(self, &UIColorProperties.isCached, cached, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
    }
    
    var isCached : Bool {
        get {
            if let value = objc_getAssociatedObject(self, &UIColorProperties.isCached) as? Bool{
                return value
            }else {
                setIsCached(false)
            }
            return objc_getAssociatedObject(self, &UIColorProperties.isCached) as! Bool
            
        }
    }
    
    
    func setRed(_ red: UInt8) {
        objc_setAssociatedObject(self, &UIColorProperties.red, red, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
    }
    
    var red : UInt8 {
        get {
            return objc_getAssociatedObject(self, &UIColorProperties.red) as! UInt8
        }
    }
    
    func setGreen(_ green: UInt8) {
        objc_setAssociatedObject(self, &UIColorProperties.green, green, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
    }
    
    var green : UInt8 {
        get {
            return objc_getAssociatedObject(self, &UIColorProperties.green) as! UInt8
        }
    }
    
    func setBlue(_ blue: UInt8) {
        objc_setAssociatedObject(self, &UIColorProperties.blue, blue, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
    }
    
    var blue : UInt8 {
        get {
            return objc_getAssociatedObject(self, &UIColorProperties.blue) as! UInt8
        }
    }
    func setAlpha(_ alpha: UInt8) {
        objc_setAssociatedObject(self, &UIColorProperties.alpha, alpha, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
    }
    
    var alpha : UInt8 {
        get {
            return objc_getAssociatedObject(self, &UIColorProperties.alpha) as! UInt8
        }
    }
    
    
    func cacheColor(){
        if(isCached){return}
        let cgColor = self.cgColor
        let components = cgColor.components
        let count = components?.count ?? 0
        var r = UInt8(0)
        var g = UInt8(0)
        var b = UInt8(0)
        var a = UInt8(255)
        if(count > 0){
            r = UInt8((components?[0] ?? 0) * 255.0)
            if(count == 2){
                g = r
                b = r
            }
        }
        if(count > 2){
            if(count > 1){
                g = UInt8((components?[1] ?? 0) * 255.0)
            }
            
            if(count > 2){
                b = UInt8((components?[2] ?? 0) * 255.0)
            }
            if(count > 3){
                a = UInt8((components?[3] ?? 0) * 255.0)
            }
        }
        
        setRed(r)
        setGreen(g)
        setBlue(b)
        setAlpha(a)
        setIsCached(true)
    }
    
    var colorCode: Int {
        get {
            var fRed : CGFloat = 0
            var fGreen : CGFloat = 0
            var fBlue : CGFloat = 0
            var fAlpha: CGFloat = 0
            
            if self.getRed(&fRed, green: &fGreen, blue: &fBlue, alpha: &fAlpha) {
                let iRed = Int32(fRed * 255.0)
                let iGreen = Int32(fGreen * 255.0)
                let iBlue = Int32(fBlue * 255.0)
                let iAlpha = Int32(fAlpha * 255.0)
                let ar = (iAlpha << 24) + (iRed << 16)
                let gb = (iGreen << 8) + iBlue
                return Int(ar + gb)
            } else {
                // Could not extract RGBA components: return black
                let iRed = Int(0.0 * 255.0)
                let iGreen = Int(0.0 * 255.0)
                let iBlue = Int(0.0 * 255.0)
                let iAlpha = Int(1.0 * 255.0)
                
                let ar = (iAlpha << 24) + (iRed << 16)
                let gb = (iGreen << 8) + iBlue
                return Int(ar + gb)
            }
        }
    }
}
