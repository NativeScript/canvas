//
//  TNSLineCap.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc(TNSLineCap)
public enum TNSLineCap: Int, RawRepresentable {
    case Butt
    case Round
    case Square
    public typealias RawValue = UInt32
    
    public var rawValue: RawValue {
        switch self {
        case .Butt:
            return 0
        case .Round:
            return 1
        case .Square:
            return 2
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Butt
        case 1:
            self = .Round
        case 2:
            self = .Square
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "butt":
            self = .Butt
        case "square":
            self = .Square
        case "round":
            self = .Round
        default:
           return nil
        }
    }
    
}
