//
//  TNSLineJoin.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc(TNSLineJoin)
public enum TNSLineJoin: Int, RawRepresentable {
    case Round
    case Bevel
    case Miter
    public typealias RawValue = UInt32
    
    public var rawValue: RawValue {
        switch self {
        case .Round:
            return 0
        case .Bevel:
            return 1
        case .Miter:
            return  2
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Round
        case 2:
            self = .Bevel
        case 1:
            self = .Miter
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "bevel":
            self = .Bevel
        case "round":
            self = .Round
        case "miter":
            self = .Miter
        default:
            return nil
        }
    }
    
}
