//
//  NSCFontFaceSet.swift
//  CanvasNative
//
//  Created by Osei Fortune on 13/11/2024.
//

import Foundation


@objc(NSCFontFaceSetStatus)
public enum NSCFontFaceSetStatus: Int, RawRepresentable {
    case loading
    case loaded
    public typealias RawValue = Int
    
    public init?(rawValue: Int) {
        switch rawValue {
        case 0:
            self = .loading
        case 1:
            self = .loaded
        default:
            return nil
        }
    }
    
    public var rawValue: Int {
        switch self {
        case .loading:
            return 0
        case .loaded:
            return 1
        }
    }
}

let familyNamePattern = #"(?:\d+px\s+)(['"]?[\w\s]+['"]?)$"#
@objcMembers
@objc(NSCFontFaceSet)
public class NSCFontFaceSet: NSObject {
    private var fontCache: Set<NSCFontFace> = Set()
    
    public static let instance = NSCFontFaceSet()
    
    public var status = NSCFontFaceSetStatus.loaded
    
    public var onStatus: ((NSCFontFaceSetStatus) -> Void)? = nil
    
    private var loaderQueue = DispatchQueue(label: "NSCFontFaceSet Loader Queue")
    
    public func iter() -> NSEnumerator {
        return (fontCache as NSSet).objectEnumerator()
    }
    
    public func array()-> [Any] {
        return (fontCache as NSSet).allObjects
    }
    
    public func add(_ font: NSCFontFace){
        fontCache.insert(font)
        if let _ = font.font {
            if let data = font.fontData {
                canvas_native_font_add_family_with_bytes(nil, data.bytes, UInt(data.length))
            }
        }
    }
    
    public func clear(){
        fontCache.removeAll()
        canvas_native_font_clear()
    }
    
    public func delete(_ font: NSCFontFace){
        fontCache.remove(font)
    }
    
    public func check(_ font: String, _ text: String?) -> Bool {
        do {
            let regex = try NSRegularExpression(pattern: familyNamePattern)
            if let match = regex.firstMatch(in: font, range: NSRange(font.startIndex..., in: font)) {
                if let fontFamilyRange = Range(match.range(at: 1), in: font) {
                    let fontFamily = font[fontFamilyRange].replacingOccurrences(of: #"^['"]|['"]$"#, with: "", options: .regularExpression)
                    return (self.fontCache.first(where: { font in
                        return (font.family == fontFamily) && font.font != nil
                    }) != nil)
                }
            }
            return false
            
        }catch {
            return false
        }
    }
    
    public func load(_ font: String, _ text: String?, _ callback:@escaping ([NSCFontFace],String?) -> Void){
        loaderQueue.async {
            self.status = .loading
            self.onStatus?(.loading)
            do {
                let regex = try NSRegularExpression(pattern: familyNamePattern)
                if let match = regex.firstMatch(in: font, range: NSRange(font.startIndex..., in: font)) {
                    if let fontFamilyRange = Range(match.range(at: 1), in: font) {
                        let fontFamily = font[fontFamilyRange].replacingOccurrences(of: #"^['"]|['"]$"#, with: "", options: .regularExpression)
                        if let first = self.fontCache.first(where: { font in
                            return font.family == fontFamily
                        }) {
                            first.status = .loading
                            first.loadSync { error in
                                guard let error = error else {
                                    first.status = .loaded
                                    
                                    if let data = first.fontData {
                                        canvas_native_font_add_family_with_bytes(first.family, data.bytes, UInt(data.length))
                                    }
                                    
                                    self.status = .loaded
                                    self.onStatus?(.loaded)
                                    
                                    callback([first], nil)
                                    return
                                }
                                first.status = .error
                                self.status = .loaded
                                self.onStatus?(.loaded)
                                callback([first], error)
                            }
                            return
                        }
                        self.status = .loaded
                        self.onStatus?(.loaded)
                        callback([], nil)
                    }
                } else {
                    self.status = .loaded
                    self.onStatus?(.loaded)
                   callback([], nil)
                }
                
            }catch {
                self.status = .loaded
                self.onStatus?(.loaded)
                callback([], error.localizedDescription)
            }
        }
    }
    
    public var size: Int {
        get {
            return fontCache.count
        }
    }
}
