//
//  TNSTextAlignment.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc(TNSTextAlignment)
public enum TNSTextAlignment: Int, RawRepresentable {
    case Start
    case Left
    case Center
    case Right
    case End
    public typealias RawValue = UInt32
    
    public var rawValue: RawValue {
        switch self {
        case .Start:
            return 0
        case .Left:
            return 1
        case .Center:
            return 2
        case .Right:
            return 3
        case .End:
            return 4
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Start
        case 1:
            self = .Left
        case 2:
            self = .Center
        case 3:
            self = .Right
        case 4:
            self = .End
        default:
            return nil
        }
    }
    
    public init?(string: String) {
        switch string {
        case "start":
            self = .Start
        case "left":
            self = .Left
        case "center":
            self = .Center
        case "Right":
            self = .Right
        case "end":
            self = .End
        default:
            return nil
        }
    }
    
}
