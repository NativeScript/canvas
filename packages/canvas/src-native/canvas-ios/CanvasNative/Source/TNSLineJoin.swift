//
//  TNSLineJoin.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc public enum TNSLineJoin: Int, RawRepresentable {
    case Bevel
    case Round
    case Miter
    public typealias RawValue = String
    
    public var rawValue: RawValue {
        switch self {
        case .Bevel:
            return "bevel"
        case .Round:
            return "round"
        case .Miter:
            return "miter"
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case "bevel":
            self = .Bevel
        case "round":
            self = .Round
        default:
            self = .Miter
        }
    }
    
}
