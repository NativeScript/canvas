//
//  TNSTextMetrics.swift
//
//  Created by Osei Fortune on 7/15/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
@objcMembers
@objc(TNSTextMetrics)
public class TNSTextMetrics: NSObject {
    private var metrics: Int64 = 0
    
    init(metrics: Int64) {
        self.metrics = metrics
    }
    
    deinit {
        destroy_text_metrics(metrics)
        metrics = 0
    }
    
    public var width:Float {
        get {
            return text_metrics_get_width(metrics)
        }
    }
    public var actualBoundingBoxLeft: Float {
        get {
            return text_metrics_get_actual_bounding_box_left(metrics)
        }
    }
    public var actualBoundingBoxRight: Float {
        get {
            return text_metrics_get_actual_bounding_box_right(metrics)
        }
    }
    
    public var actualBoundingBoxAscent: Float {
        get {
            return text_metrics_get_actual_bounding_box_ascent(metrics)
        }
    }
    public var actualBoundingBoxDescent: Float {
        get {
            return text_metrics_get_actual_bounding_box_descent(metrics)
        }
    }
    
    public var fontBoundingBoxAscent: Float {
        get {
            return text_metrics_get_font_bounding_box_ascent(metrics)
        }
    }
    public var fontBoundingBoxDescent: Float {
        get {
            return text_metrics_get_font_bounding_box_descent(metrics)
        }
    }
    
    public var emHeightAscent: Float {
        get {
            return text_metrics_get_em_height_ascent(metrics)
        }
    }
    public var emHeightDescent: Float {
        get {
            return text_metrics_get_em_height_descent(metrics)
        }
    }
    public var hangingBaseline: Float {
        get {
            return text_metrics_get_hanging_baseline(metrics)
        }
    }
    public var alphabeticBaseline: Float {
        get {
            return text_metrics_get_alphabetic_baseline(metrics)
        }
    }
    public var ideographicBaseline: Float {
        get {
            return text_metrics_get_ideographic_baseline(metrics)
        }
    }
}
