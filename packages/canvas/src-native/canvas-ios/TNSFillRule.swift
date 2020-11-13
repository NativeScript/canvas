//
//  TNSFillRule.swift
//  CanvasNative
//
//  Created by Osei Fortune on 10/11/2020.
//

import Foundation

@objc(TNSFillRule)
public enum TNSFillRule: Int, RawRepresentable {
    case NonZero
    case EvenOdd
    public typealias RawValue = UInt32
    
    public var rawValue: RawValue {
        switch self {
        case .NonZero:
            return 0
        case .EvenOdd:
            return 1
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .NonZero
        case 1:
            self = .EvenOdd
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "nonzero":
            self = .NonZero
        case "evenodd":
            self = .EvenOdd
        default:
            return nil
        }
    }
    
}
