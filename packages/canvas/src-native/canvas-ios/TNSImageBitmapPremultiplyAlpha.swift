//
//  TNSImageBitmapPremultiplyAlpha.swift
//  CanvasNative
//
//  Created by Osei Fortune on 11/04/2021.
//

import Foundation

@objc(TNSImageBitmapPremultiplyAlpha)
public enum TNSImageBitmapPremultiplyAlpha: Int, RawRepresentable {
    case Default
    case Premultiply
    case None
    public typealias RawValue = Int32
    
    public var rawValue: RawValue {
        switch self {
        case .Default:
            return 0
        case .Premultiply:
            return 1
        case .None:
            return  2
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Default
        case 1:
            self = .Premultiply
        case 2:
            self = .None
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "default":
            self = .Default
        case "premultiply":
            self = .Premultiply
        case "none":
            self = .None
        default:
            return nil
        }
    }
    
}
