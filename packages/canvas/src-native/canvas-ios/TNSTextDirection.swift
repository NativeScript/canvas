//
//  TNSTextDirection.swift
//  CanvasNative
//
//  Created by Osei Fortune on 09/11/2020.
//

import Foundation

@objc(TNSTextDirection)
public enum TNSTextDirection: Int, RawRepresentable {
    case Ltr
    case Rtl
    public typealias RawValue = UInt32
    
    public var rawValue: RawValue {
        switch self {
        case .Ltr:
            return 0
        case .Rtl:
            return 1
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Ltr
        case 1:
            self = .Rtl
        default:
            return nil
        }
    }
    
    public init?(string: String) {
        switch string {
        case "ltr":
            self = .Ltr
        case "rtl":
            self = .Rtl
        default:
            return nil
        }
    }
    
}
