//
//  TNSContextAttributes.swift
//  CanvasNative
//
//  Created by Osei Fortune on 12/11/2022.
//

import Foundation

@objc(TNSContextAttributes)
@objcMembers
public class TNSContextAttributes: NSObject, Codable {
    
    static let defaultInstance = TNSContextAttributes()
    
    var alpha = true
    
    var antialias = true
    
    var depth = true
    
    var failIfMajorPerformanceCaveat = false
    
    var powerPreference: String = "default"
    
    var premultipliedAlpha = true
    
    var preserveDrawingBuffer = false
    
    var stencil = false
    
    var desynchronized = false
    
    var xrCompatible = false
    
    private static var decoder = JSONDecoder()
    
    static func fromArgs(_ alpha: Bool,
                         _ antialias: Bool,
                         _ depth: Bool,
                         _ failIfMajorPerformanceCaveat: Bool,
                         _ powerPreference: String,
                         _ premultipliedAlpha: Bool,
                         _ preserveDrawingBuffer: Bool,
                         _ stencil: Bool,
                         _ desynchronized: Bool,
                         _ xrCompatible: Bool
    ) -> TNSContextAttributes {
        let attrs = TNSContextAttributes()
        attrs.alpha = alpha
        attrs.antialias = antialias
        attrs.depth = depth
        attrs.failIfMajorPerformanceCaveat = failIfMajorPerformanceCaveat
        attrs.powerPreference = powerPreference
        attrs.premultipliedAlpha = premultipliedAlpha
        attrs.preserveDrawingBuffer = preserveDrawingBuffer
        attrs.stencil = stencil
        attrs.desynchronized = desynchronized
        attrs.xrCompatible = xrCompatible
        return attrs
    }
    
    static func fromJSONString(_ json: String) -> TNSContextAttributes {
        guard let data = json.data(using: .utf8) else {
            return TNSContextAttributes()
        }
        
        do {
            return try decoder.decode(TNSContextAttributes.self, from: data)
        }catch {
            return TNSContextAttributes()
        }
    }
    
    
    static func fromMap(_ contextAttributes: Dictionary<String, Any>) -> TNSContextAttributes {
        let attr = TNSContextAttributes()
        for(key, value) in contextAttributes {
            switch(key){
            case "alpha":
                attr.alpha = value as? Bool ?? true
                break
            case "antialias":
                attr.antialias = value as? Bool ?? true
                break
            case "depth":
                attr.depth = value as? Bool ?? true
                break
            case "failIfMajorPerformanceCaveat":
                attr.failIfMajorPerformanceCaveat = value as? Bool ?? false
                break
            case "premultipliedAlpha":
                attr.premultipliedAlpha = value as? Bool ?? true
                break
            case "preserveDrawingBuffer":
                attr.preserveDrawingBuffer = value as? Bool ?? false
                break
            case "stencil":
                attr.stencil = value as? Bool ?? false
                break
            case "xrCompatible":
                attr.xrCompatible = value as? Bool ?? false
                break
            case "desynchronized":
                attr.desynchronized = value as? Bool ?? false
                break
            case "powerPreference":
                attr.powerPreference = value as? String ?? "default"
                break
            default:
                break
            }
        }
        return attr
    }
}
