//
//  TNSCompositeOperationType.swift
//
//  Created by Osei Fortune on 7/17/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation

@objc public enum TNSCompositeOperationType: Int, RawRepresentable {
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

    public typealias RawValue = String

    public var rawValue: RawValue {
        switch self {
        case .SourceOver:
            return "source-over"
        case .SourceIn:
            return "source-in"
        case .SourceOut:
            return "source-out"
        case .SourceAtop:
            return "source-atop"
        case .DestinationOver:
            return "destination-over"
        case .DestinationIn:
            return "destination-in"
        case .DestinationOut:
            return "destination-out"
        case  .DestinationAtop:
            return  "destination-atop"
        case .Lighter:
            return "lighter"
        case .Copy:
            return "copy"
        case .Xor:
            return "xor"
        case .Multiply:
            return "multiply"
        case .Screen:
            return "screen"
        case .Overlay:
            return "overlay"
        case .Darken:
            return "darken"
        case .Lighten:
            return "lighten"
        case .ColorDodge:
            return "color-dodge"
        case .ColorBurn:
            return "color-burn"
        case .HardLight:
            return "hard-light"
        case .SoftLight:
            return "soft-light"
        case .Difference:
            return "difference"
        case .Exclusion:
            return "exclusion"
        case .Hue:
            return "hue"
        case .Saturation:
            return "saturation"
        case .Color:
            return "color"
        case .Luminosity:
            return "luminosity"
        }
    }


    public init?(rawValue: RawValue) {
        switch rawValue {
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
            self = .SourceOver
        }
    }
}
