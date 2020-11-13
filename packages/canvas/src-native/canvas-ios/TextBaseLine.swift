//
//  TextBaseLine.swift
//  CanvasNative
//
//  Created by Osei Fortune on 11/11/2020.
//

import Foundation

@objc(TNSTextBaseLine)
public enum TNSTextBaseLine: Int, RawRepresentable {
    case Top
    case Hanging
    case Middle
    case Alphabetic
    case Ideographic
    case Bottom
    
    public typealias RawValue = UInt32
    
    public var rawValue: RawValue {
        switch self {
        case .Top:
            return 0
        case .Hanging:
            return 1
        case .Middle:
            return 2
        case .Alphabetic:
            return 3
        case .Ideographic:
            return 4
        case .Bottom:
            return 5
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Top
        case 1:
            self = .Hanging
        case 2:
            self = .Middle
        case 3:
            self = .Alphabetic
        case 4:
            self = .Ideographic
        case 5:
            self = .Bottom
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "top":
            self = .Top
        case "hanging":
            self = .Hanging
        case "middle":
            self = .Middle
        case "alphabetic":
            self = .Alphabetic
        case "ideographic":
            self = .Ideographic
        case "bottom":
            self = .Bottom
        default:
            return nil
        }
    }
    
}




