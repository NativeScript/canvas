//
//  TNSCompositeOperationType.swift
//
//  Created by Osei Fortune on 7/17/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation

@objc(TNSCompositeOperationType)
public enum TNSCompositeOperationType: Int, RawRepresentable {
    case SourceOver
    case SourceIn
    case SourceOut
    case SourceAtop
    case DestinationOver
    case DestinationIn
    case DestinationOut
    case DestinationAtop
    case Lighter
    case Copy
    case Xor
    case Multiply
    case Screen
    case Overlay
    case Darken
    case Lighten
    case ColorDodge
    case ColorBurn
    case HardLight
    case SoftLight
    case Difference
    case Exclusion
    case Hue
    case Saturation
    case Color
    case Luminosity
    
    public typealias RawValue = UInt32
    
    
    public var rawValue: RawValue {
        switch self {
        case .SourceOver:
            return 0
        case .SourceIn:
            return 1
        case .SourceOut:
            return 2
        case .SourceAtop:
            return 3
        case .DestinationOver:
            return 4
        case .DestinationIn:
            return 5
        case .DestinationOut:
            return 6
        case  .DestinationAtop:
            return 7
        case .Lighter:
            return 8
        case .Copy:
            return 9
        case .Xor:
            return 10
        case .Multiply:
            return 11
        case .Screen:
            return 12
        case .Overlay:
            return 13
        case .Darken:
            return 14
        case .Lighten:
            return 15
        case .ColorDodge:
            return 16
        case .ColorBurn:
            return 17
        case .HardLight:
            return 18
        case .SoftLight:
            return 19
        case .Difference:
            return 20
        case .Exclusion:
            return 21
        case .Hue:
            return 22
        case .Saturation:
            return 23
        case .Color:
            return 24
        case .Luminosity:
            return 25
        }
    }
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .SourceOver
        case 1:
            self = .SourceIn
        case 2:
            self = .SourceOut
        case 3:
            self = .SourceAtop
        case 4:
            self = .DestinationOver
        case 5:
            self = .DestinationIn
        case 6:
            self = .DestinationOut
        case 7:
            self = .DestinationAtop
        case 8:
            self = .Lighter
        case 9:
            self = .Copy
        case 10:
            self = .Multiply
        case 11:
            self = .Xor
        case 12:
            self = .Screen
        case 13:
            self = .Overlay
        case 14:
            self = .Darken
        case 15:
            self = .Lighten
        case 16:
            self = .ColorDodge
        case 17:
            self = .ColorBurn
        case 18:
            self = .HardLight
        case 19:
            self = .SoftLight
        case 20:
            self = .Difference
        case 21:
            self = .Exclusion
        case 22:
            self = .Hue
        case 23:
            self = .Saturation
        case 24:
            self = .Luminosity
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "source-over":
            self = .SourceOver
        case "source-in":
            self = .SourceIn
        case "source-out":
            self = .SourceOut
        case "source-atop":
            self = .SourceAtop
        case "destination-over":
            self = .DestinationOver
        case "destination-in":
            self = .DestinationIn
        case "destination-out":
            self = .DestinationOut
        case  "destination-atop":
            self = .DestinationAtop
        case "lighter":
            self = .Lighter
        case "copy":
            self = .Copy
        case "multiply":
            self = .Multiply
        case "xor":
            self = .Xor
        case "screen":
            self = .Screen
        case "overlay":
            self = .Overlay
        case "darken":
            self = .Darken
        case "lighten":
            self = .Lighten
        case "color-dodge":
            self = .ColorDodge
        case  "color-burn":
            self = .ColorBurn
        case "hard-light":
            self = .HardLight
        case "soft-light":
            self = .SoftLight
        case  "difference":
            self = .Difference
        case  "exclusion":
            self = .Exclusion
        case "hue":
            self = .Hue
        case "saturation":
            self = .Saturation
        case "luminosity":
            self = .Luminosity
        default:
            return nil
        }
    }
}
