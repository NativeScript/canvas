//
//  TNSTextAlignment.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc public enum TNSTextAlignment: Int, RawRepresentable {
    case Left
    case Start
    case Center
    case End
    case Right
    public typealias RawValue = String
    
    public var rawValue: RawValue {
        switch self {
        case .Left:
            return "left"
        case .Start:
            return "start"
        case .Center:
            return "center"
        case .End:
            return "end"
        case .Right:
            return "right"
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case "start":
            self = .Start
        case "center":
            self = .Center
        case "end":
            self = .End
        case "Right":
            self = .Right
        default:
            self = .Left
        }
    }
    
}
