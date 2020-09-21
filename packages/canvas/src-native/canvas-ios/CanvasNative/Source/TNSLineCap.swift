//
//  TNSLineCap.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc public enum TNSLineCap: Int, RawRepresentable {
    case Butt
    case Round
    case Square
    public typealias RawValue = String
    
    public var rawValue: RawValue {
        switch self {
        case .Butt:
            return "butt"
        case .Round:
            return "round"
        case .Square:
            return "square"
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case "square":
            self = .Square
        case "round":
            self = .Round
        default:
            self = .Butt
        }
    }
    
}
