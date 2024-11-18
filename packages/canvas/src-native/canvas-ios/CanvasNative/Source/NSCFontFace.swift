//
//  NSCFontFace.swift
//  CanvasNative
//
//  Created by Osei Fortune on 13/11/2024.
//

import Foundation
import UIKit

enum NSCFontStyle {
    case normal
    case italic
    case oblique(Int)
}

@objc(NSCFontDisplay)
public enum NSCFontDisplay: Int, RawRepresentable, Codable {
    case auto
    case block
    case fallback
    case optional
    case swap
    public typealias RawValue = Int
    public init?(rawValue: Int) {
        switch rawValue {
        case 0:
            self = .auto
        case 1:
            self = .block
        case 2:
            self = .fallback
        case 3:
            self = .optional
        case 4:
            self = .swap
        default:
            return nil
        }
    }
    
    public var rawValue: Int {
        switch(self){
        case .auto:
            return 0
        case .block:
            return 1
        case .fallback:
            return 2
        case .optional:
            return 3
        case .swap:
            return 4
        }
    }
    
}

@objc(NSCFontWeight)
public enum NSCFontWeight: Int, RawRepresentable, Codable {
    case thin
    case extraLight
    case light
    case normal
    case medium
    case semiBold
    case bold
    case extraBold
    case black
    public typealias RawValue = Int
    public init?(rawValue: Int) {
        switch rawValue {
        case 100:
            self = .thin
        case 200:
            self = .extraLight
        case 300:
            self = .light
        case 400:
            self = .normal
        case 500:
            self = .medium
        case 600:
            self = .semiBold
        case 700:
            self = .bold
        case 800:
            self = .extraBold
        case 900:
            self = .black
        default:
            return nil
        }
    }
    
    public var rawValue: Int {
        switch(self){case .thin:
            return 100
        case .extraLight:
            return 200
        case .light:
            return 300
        case .normal:
            return 400
        case .medium:
            return 500
        case .semiBold:
            return 600
        case .bold:
            return 700
        case .extraBold:
            return 800
        case .black:
            return 900
        }
    }
    
    public var uiFontWeight: UIFont.Weight {
        switch(self){
        case .thin:
            return .thin
        case .extraLight:
            return .ultraLight
        case .light:
            return .light
        case .normal:
            return .regular
        case .medium:
            return .medium
        case .semiBold:
            return .semibold
        case .bold:
            return .bold
        case .extraBold:
            return .heavy
        case .black:
            return .black
        }
    }
    
}

@objcMembers
@objc(NSCFontDescriptors)
public class NSCFontDescriptors: NSObject, Codable{
    var weight: NSCFontWeight
    var family: String
    var ascentOverride: String
    var descentOverride: String
    var display: NSCFontDisplay
    var style: String
    var stretch: String
    var unicodeRange: String
    var featureSettings: String
    var lineGapOverride: String
    var variationSettings: String
    public init(family: String) {
        weight = NSCFontWeight.normal
        self.family = family
        self.ascentOverride = "normal"
        self.descentOverride = "normal"
        self.display = .auto
        self.style = "normal"
        self.stretch = "normal";
        self.unicodeRange = "U+0-10FFFF";
        self.featureSettings = "normal";
        self.variationSettings = "normal";
        self.lineGapOverride = "normal";
        
        super.init()
    }
    
    public func update(_ value: String){
        let values = NSCFontFace.parseFontFaceRules(from: value)
        if let first = values.first {
            self.setFontWeight(first.fontWeight)
            self.setFontStyle(first.fontStyle)
            switch(first.fontDisplay){
            case "auto":
                self.display = .auto
                break
            case "block":
                self.display = .block
                break
            case "fallback":
                self.display = .fallback
                break
            case "optional":
                self.display = .optional
                break
            case "swap":
                self.display = .swap
                break
            default: break
                // noop
            }
        }
    }
    
    public func setFontWeight(_ value: String){
        guard let value = Int(value) else {
            switch(value){
            case "normal":
                self.weight = .normal
                break
            case "bold":
                self.weight = .bold
                break
            default:
                // noop
                break
            }
            
            return
        }
        
        if let weight = NSCFontWeight(rawValue: value) {
            self.weight = weight
        }
    }
    
    public func setFontStyle(_ value: String){
        let fontStyle = "font-style: \(value)"
        if let fontStyleMatch = NSCFontFace.matchObliqueWithAngle(fontStylePattern, in: fontStyle) {
            // todo
            self.style = "\(fontStyleMatch.0)\(fontStyleMatch.1 ?? "")"
            
          //  setFontStyle(value: fontStyleMatch.0, angle: fontStyleMatch.1)
        } else if let fontStyleMatch = NSCFontFace.matchPattern(fontStylePattern, in: fontStyle)  {
           //  setFontStyle(value: fontStyleMatch, angle: nil)
            self.style = fontStyleMatch
        }
    }
}

@objc(NSCFontFaceStatus)
public enum NSCFontFaceStatus: Int, RawRepresentable {
    case unloaded
    case loading
    case loaded
    case error
    public typealias RawValue = Int
    
    public init?(rawValue: Int) {
        switch rawValue {
        case 0:
            self = .unloaded
        case 1:
            self = .loading
        case 2:
            self = .loaded
        case 3:
            self = .error
        default:
            return nil
        }
    }
    
    public var rawValue: Int {
        switch self {
        case .unloaded:
            return 0
        case .loading:
            return 1
        case .loaded:
            return 2
        case .error:
            return 3
        }
    }
}

let fontFamilyPattern = #"font-family:\s*'([^']+)';"#
let fontStylePattern = #"font-style:\s*(normal|italic|oblique(?:\s+([-]?\d+(\.\d+)?deg))?);"#
let fontWeightPattern = #"font-weight:\s*([^;]+);"#
let fontDisplayPattern = #"font-display:\s*([^;]+);"#
let srcPattern = #"src:\s*url\(([^)]+)\)\s*format\('([^']+)'\);"#

@objcMembers
@objc(NSCFontFace)
public class NSCFontFace: NSObject {
    public internal (set) var font: CGFont? = nil
    private var fontFamily: String
    public internal (set) var fontData: NSData? = nil
    private var localOrRemoteSource: String? = nil
    private var fontDescriptors: NSCFontDescriptors
    private var loaderQueue = DispatchQueue(label: "NSCFontFace Loader Queue")
    private static let bundle = Bundle(identifier: "com.github.triniwiz.CanvasNative")
    static let genericFontFamilies = [
        "serif": "Times New Roman",
        "sans-serif": "Helvetica",
        "monospace": "Courier",
        "cursive": "Snell Roundhand",
        "fantasy": "Papyrus",
        "system-ui": "San Francisco",
        "ui-serif": "Times New Roman",
        "ui-sans-serif": "San Francisco",
        "ui-monospace": "Menlo",
        "ui-rounded": "SF Rounded",
        "emoji": "Apple Color Emoji",
    ]
    
    public func updateDescriptor(value: String){
        fontDescriptors.update(value)
    }
    
    public static func loadFromStyle(style: String) -> NSCFontFace?{
        return nil
    }
    
    static func matchPattern(_ pattern: String, in text: String) -> String? {
        let regex = try? NSRegularExpression(pattern: pattern, options: [])
        let nsString = text as NSString
        let results = regex?.firstMatch(in: text, options: [], range: NSRange(location: 0, length: nsString.length))
        if let range = results?.range(at: 1) {
            return nsString.substring(with: range)
        }
        return nil
    }
    
    static func matchObliqueWithAngle(_ pattern: String, in text: String) -> (String, String?)? {
        let regex = try? NSRegularExpression(pattern: pattern, options: [])
        let nsString = text as NSString
        if let result = regex?.firstMatch(in: text, options: [], range: NSRange(location: 0, length: nsString.length)) {
            let styleRange = result.range(at: 1)
            let angleRange = result.range(at: 2)
            
            guard styleRange.location != NSNotFound else { return nil }
            let style = nsString.substring(with: styleRange)
            
            let angle = angleRange.location != NSNotFound ? nsString.substring(with: angleRange) : nil
            return (style, angle)
        }
        return nil
    }
    
    
    static func parseFontFaceRules(from css: String) -> [(fontFamily: String, fontStyle: String, angle: String?, fontWeight: String, fontDisplay: String, srcURL: String, srcFormat: String)] {
        var fontFaceRules: [(fontFamily: String, fontStyle: String, angle: String?, fontWeight: String, fontDisplay: String, srcURL: String, srcFormat: String)] = []
        
        let fontFacePattern = #"@font-face\s*{([^}]+)}"#
        
        let regex = try? NSRegularExpression(pattern: fontFacePattern, options: [])
        let nsString = css as NSString
        let matches = regex?.matches(in: css, options: [], range: NSRange(location: 0, length: nsString.length)) ?? []
        
        for match in matches {
            let fontFaceBlock = nsString.substring(with: match.range(at: 1))
            
            let fontFamily = matchPattern(fontFamilyPattern, in: fontFaceBlock) ?? ""
            let fontWeight = matchPattern(fontWeightPattern, in: fontFaceBlock) ?? ""
            let fontDisplay = matchPattern(fontDisplayPattern, in: fontFaceBlock) ?? ""
            
            var fontStyle: String = ""
            var angle: String? = nil
            var srcURL: String? = nil
            var srcFormat: String? = nil
            
            
            if let fontStyleMatch = matchObliqueWithAngle(fontStylePattern, in: fontFaceBlock) {
                fontStyle = fontStyleMatch.0
                angle = fontStyleMatch.1
            } else {
                fontStyle = matchPattern(fontStylePattern, in: fontFaceBlock) ?? ""
            }
            
            
            if let srcMatch = try? NSRegularExpression(pattern: srcPattern).firstMatch(in: fontFaceBlock, range: NSRange(fontFaceBlock.startIndex..., in: fontFaceBlock)) {
                srcURL = (fontFaceBlock as NSString).substring(with: srcMatch.range(at: 1))
                srcFormat = (fontFaceBlock as NSString).substring(with: srcMatch.range(at: 2))
            }
            
            fontFaceRules.append((fontFamily, fontStyle, angle, fontWeight, fontDisplay, srcURL ?? "", srcFormat ?? ""))
        }
        
        return fontFaceRules
    }
    
    public static func importFromRemote(url: String, load:Bool, callback: @escaping ([NSCFontFace]?,String?) -> Void){
        if(url.starts(with: "http")){
            guard let url = URL(string: url) else {
                callback(nil, "Invalid source \(url)")
                return
            }
            
            URLSession.shared.dataTask(with: URLRequest(url: url)) { data, response, error in
                if(error != nil){
                    callback(nil, error?.localizedDescription)
                    return
                }
                guard let data = data else {
                    callback(nil, "Invalid source \(url)")
                    return
                }
                
                let css = String(data: data, encoding: .utf8)
                guard let css = css else {
                    callback(nil, "Invalid source \(url)")
                    return
                }
                
                do {
                    let fontFaceRegex = try! NSRegularExpression(pattern: "@font-face\\s*\\{([^}]+)\\}", options: [])
                    let matches = fontFaceRegex.matches(in: css, range: NSRange(css.startIndex..., in: css))
                    var fonts: [NSCFontFace] = []
                    
                    
                    for match in matches {
                        let range = match.range(at: 1)
                        if let swiftRange = Range(range, in: css) {
                            let rule = String(css[swiftRange]).trimmingCharacters(in: .whitespacesAndNewlines)
                            
                            let fontFamily = matchPattern(fontFamilyPattern, in: rule) ?? ""
                            let fontStyle = matchObliqueWithAngle(fontStylePattern, in: rule) ?? ("normal",nil)
                            let fontWeight = matchPattern(fontWeightPattern, in: rule) ?? ""
                            let fontDisplay = matchPattern(fontDisplayPattern, in: rule) ?? ""
                            var srcURL: String? = nil
                            var srcFormat: String? = nil
                            
                            if let srcMatch = try NSRegularExpression(pattern: srcPattern).firstMatch(in: rule, range: NSRange(rule.startIndex..., in: rule)) {
                                srcURL = (rule as NSString).substring(with: srcMatch.range(at: 1))
                                  srcFormat = (rule as NSString).substring(with: srcMatch.range(at: 2))
                            }
                            let font = NSCFontFace(family: fontFamily, source: srcURL ?? "")
                            font.setFontStyle(value: fontStyle.0, angle: fontStyle.1)
                            font.setFontWeight(value: fontWeight)
                            font.setFontDisplay(value: fontDisplay)
                            if(load){
                                font.loadSync { _ in
                                    if let data = font.fontData {
                                        if(srcFormat == "ttf"){
                                            canvas_native_font_add_family_with_bytes(nil, data.bytes, UInt(data.length))
                                        }else {
                                            canvas_native_font_add_family_with_bytes(fontFamily, data.bytes, UInt(data.length))
                                        }
                                    }
                                }
                            }
                            fonts.append(font)
                            NSCFontFaceSet.instance.add(font)
                        }
                        
                    }
        
                    callback(fonts, nil)
                    
                }catch {
                    callback(nil, error.localizedDescription)
                }
                
                
            }.resume()
            return
        }
        callback(nil, "Invalid source \(url)")
    }
    
    static func getMathFontPath(weight: Int, italic: Bool = false) -> String? {
        let value = min(max(weight, 100), 1000)
        switch(value){
        case 100...499:
            if(italic){
                return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoText-Italic", ofType: "ttf")
            }
            return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoMath-Regular", ofType: "ttf")
        case 500...599:
            if(italic){
                return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoText-MediumItalic", ofType: "ttf")
            }
            return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoText-Medium", ofType: "ttf")
        case 600...699:
            if(italic){
                return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoText-SemiBoldItalic", ofType: "ttf")
            }
            return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoText-SemiBold", ofType: "ttf")
        default:
            if(italic){
                return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoText-BoldItalic", ofType: "ttf")
            }
            return bundle?.path(forResource: "fonts/STIXTwoMath/STIXTwoText-Bold", ofType: "ttf")
        }
    }
    
    
    static func getFangsongFontPath(weight: Int, italic: Bool = false) -> String? {
        let value = min(max(weight, 100), 1000)
        switch(value){
        case 100...299:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-ExtraLight", ofType: "ttf")
        case 300...399:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-Light", ofType: "ttf")
        case 400...499:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-Regular", ofType: "ttf")
        case 500...599:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-Medium", ofType: "ttf")
        case 600...699:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-SemiBold", ofType: "ttf")
        case 700...799:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-Bold", ofType: "ttf")
        case 800...899:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-ExtraBold", ofType: "ttf")
        default:
            return bundle?.path(forResource: "fonts/Noto_Serif_CJK/NotoSerifTC-Black", ofType: "ttf")
        }
    }
    
    
    public init(family: String) {
        fontFamily = family
        fontDescriptors = NSCFontDescriptors(family: family)
        super.init()
    }
    
    public init(family: String, source: String) {
        fontFamily = family
        fontDescriptors = NSCFontDescriptors(family: family)
        localOrRemoteSource = source
        super.init()
    }
    
    public init(family: String, data source: NSData) {
        fontFamily = family
        fontDescriptors = NSCFontDescriptors(family: family)
        fontData = source as NSData
        super.init()
    }
    
    public init(_ family: String, _ source: String? = nil, _ descriptors: NSCFontDescriptors? = nil) {
        fontFamily = family
        fontDescriptors = descriptors ?? NSCFontDescriptors(family: family)
        localOrRemoteSource = source
        super.init()
    }
    
    public init(_ family: String, data: NSData? = nil, _ descriptors: NSCFontDescriptors? = nil) {
        fontFamily = family
        fontDescriptors = descriptors ?? NSCFontDescriptors(family: family)
        fontData = data
        super.init()
    }
    
    
    public var status: NSCFontFaceStatus = .unloaded
    
    public var display: NSCFontDisplay = .auto
    
    public func setFontDisplay(value: String){
        switch(value){
        case "auto":
            fontDescriptors.display = .auto
            break;
        case "block":
            fontDescriptors.display = .block
            break;
        case "fallback":
            fontDescriptors.display = .fallback
            break;
        case "optional":
            fontDescriptors.display = .optional
            break;
        case "swap":
            fontDescriptors.display = .swap
            break;
        default:
            // noop
            break
        }
    }
    
    public var style: String {
        get {
            return fontDescriptors.style
        }
        
        set {
            fontDescriptors.setFontStyle(newValue)
        }
    }
    
    public func setFontStyle(value: String, angle: String?){
        fontDescriptors.style = "\(value)\(angle ?? "")"
    }
    
    public var weight: NSCFontWeight {
        get {
            return fontDescriptors.weight
        }
        
        set {
            fontDescriptors.weight = newValue
        }
        
    }
    
    public func setFontWeight(value: String){
        // todo clone if descriptors change
        fontDescriptors.setFontWeight(value)
    }
    
    public var family: String {
        return fontDescriptors.family
    }
    
    public var ascentOverride: String {
        return fontDescriptors.ascentOverride
    }
    
    public var descentOverride: String {
        return fontDescriptors.descentOverride
    }
    
    private func registerFont(font: CGFont) -> NSError?{
        var error: Unmanaged<CFError>?
        if(!CTFontManagerRegisterGraphicsFont(font, &error)){
            return error?.takeRetainedValue() as? NSError
        }
        return nil
    }
    
    private func loadFont() -> String? {
        var fontPath: String? = nil
        if(fontFamily == "math") {
            fontPath = NSCFontFace.getMathFontPath(weight: fontDescriptors.weight.rawValue)
        }
        
        // todo handle fangsong
        
        guard let fontPath = fontPath else {
            var font: CGFont? = nil
            if(fontData == nil){
                guard let name = NSCFontFace.genericFontFamilies[fontFamily] else {
                    let systemFont = UIFont.systemFont(ofSize: 16)
                    systemFont.fontDescriptor.withSymbolicTraits(.traitBold)
                    font = CGFont(systemFont.fontName as CFString)
                    return nil
                }
                let newFont = UIFont(name: name, size: 16) ?? UIFont.systemFont(ofSize: 16)
                font = CGFont(newFont.fontName as CFString)
            }
            
            guard let font = font else {
                return "Invalid font resource"
            }
            
            let error = registerFont(font: font)
            
            if(error == nil){
                self.font = font
                self.status = .loaded
                return nil
            }
            
            status = .error
            return error?.localizedDescription
        }
        
        let data = NSData(contentsOfFile: fontPath)
        guard let fontData =  data else {
            self.status = .error
            return "Invalid font resource"
        }
        
        let error = loadFontData(fontData)
        
        if(error != nil){
            return error?.localizedDescription
        }
        return nil
    }
    
    private func loadFontData(_ fontData: NSData) -> NSError? {
        let bytes = fontData.bytes.assumingMemoryBound(to: UInt8.self)
        let ref = CFDataCreateWithBytesNoCopy(kCFAllocatorDefault, bytes, fontData.count, kCFAllocatorNull)
        let provider = CGDataProvider(data: ref!)!
        let font = CGFont(provider)!
        
        var error: Unmanaged<CFError>?
        if(CTFontManagerRegisterGraphicsFont(font, &error)){
            self.font = font
            self.status = .loaded
            return nil
        }else {
            status = .error
            return error?.takeRetainedValue() as? NSError
        }
        
    }
    
    private func handleFontData(_ data: NSData?, _ callback : @escaping (String?)-> Void){
        guard let fontData = data else {
            self.status = .error
            callback("Invalid source \(self.localOrRemoteSource!)")
            return
        }
        self.fontData = fontData
        guard let loadError = self.loadFontData(fontData) else {
            callback(nil)
            return
        }
        self.status = .error
        callback(loadError.localizedDescription)
    }
    
    func loadSync(_ callback: @escaping (String?)-> Void){
        if(self.localOrRemoteSource != nil){
            guard let url = URL(string: self.localOrRemoteSource!) else {
                self.status = .error
                callback("Invalid source \(self.localOrRemoteSource!)")
                return
            }
            if((self.localOrRemoteSource?.starts(with: "http")) != nil){
                URLSession.shared.dataTask(with: URLRequest(url: url)) { data, response, error in
                    if(error != nil){
                        self.status = .error
                        callback(error?.localizedDescription)
                        return
                    }
                    self.handleFontData(data as? NSData, callback)
                }.resume()
            }else {
                let data = NSData(contentsOfFile: url.path)
                guard let fontData =  data else {
                    self.status = .error
                    callback("Invalid source \(self.localOrRemoteSource!)")
                    return
                }
                self.handleFontData(fontData, callback)
                
            }
        }else if(self.fontData != nil){
            self.handleFontData(self.fontData, callback)
        }else {
            let error = self.loadFont()
            callback(error)
        }
    }
    
    public func load(_ callback: @escaping (String?)-> Void){
        if(status == .loaded){
            callback(nil)
            return
        }
        status = .loading
        loaderQueue.async {
            self.loadSync(callback)
        }
    }
}
