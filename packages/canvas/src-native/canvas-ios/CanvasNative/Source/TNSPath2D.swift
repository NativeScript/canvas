//
//  TNSPath2D.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/26/20.
//

import Foundation
@objcMembers
@objc(TNSPath2D)
public class TNSPath2D: NSObject {
    
    var path: Int64 = 0
    
    deinit {
        if(path != 0 ){
            destroy_path(path)
            path = 0
        }
    }
    public override init() {
        path = path_create()
    }
    public init(path: TNSPath2D) {
        self.path = path_create_with_path(path.path)
    }
    
    public init(data: String){
        self.path = path_create_with_string(data)
    }
    public func addPath(_ path: TNSPath2D){
        path_add_path(self.path, path.path)
    }
    public func addPath(_ path: TNSPath2D,_ transform: TNSDOMMatrix?){
        path_add_path_with_matrix(self.path, path.path, transform?.matrix ?? 0)
    }
    
    public func closePath(){
        path_close_path(path)
    }
    
    public func moveTo(_ x: Float,_ y: Float){
        path_move_to(path, x, y)
    }
    
    public func lineTo(_ x: Float,_ y: Float){
        path_line_to(path, x, y)
    }
    
    public func bezierCurveTo(_ cp1x: Float,_ cp1y: Float,_ cp2x: Float,_ cp2y: Float,_ x: Float,_ y: Float) {
        path_bezier_curve_to(path, cp1x, cp1y, cp2x, cp2y, x, y)
    }
    
    public func quadraticCurveTo(_ cpx: Float,_ cpy: Float,_ x: Float,_ y: Float){
        path_quadratic_curve_to(path, cpx, cpy, x, y)
    }
    
    
    public func arc(_ x: Float,_ y: Float,_ radius: Float,_ startAngle: Float,_ endAngle: Float) {
        arc(x, y, radius, startAngle, endAngle, false);
    }
    
    public func arc(_ x: Float,_ y: Float,_ radius: Float,_ startAngle: Float,_ endAngle: Float,_ anticlockwise: Bool) {
        path_arc(path, x, y, radius, startAngle, endAngle, anticlockwise)
    }
    
    public func arcTo(_ x1: Float,_ y1: Float,_ x2: Float,_ y2: Float,_ radius: Float) {
        path_arc_to(path, x1, y1, x2, y2, radius)
    }
    
    
    public func ellipse(_ x: Float,_ y: Float,_ radiusX: Float,_ radiusY: Float,_ rotation: Float,_ startAngle: Float,_ endAngle: Float) {
        ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, false)
    }
    
    public func ellipse(_ x: Float,_ y: Float, _ radiusX: Float,_ radiusY: Float,_ rotation: Float, _ startAngle: Float,_ endAngle: Float,_ anticlockwise: Bool) {
        path_ellipse(path, x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise)
    }
    
    public func rect(_ x: Float,_ y: Float,_ width: Float,_ height: Float) {
        path_rect(path, x, y, width, height)
    }
    
    public func roundRect(
            _ x: Float, _ y: Float, _ width: Float, _ height: Float,
            _ topLeft: Float,
            _ topRight: Float,
            _ bottomRight: Float,
            _ bottomLeft: Float
        ) {
            path_round_rect(
                path,
                x,
                y,
                width,
                height,
                topLeft,
                topRight,
                bottomRight,
                bottomLeft
            )
        }


    public func roundRectWithRadii(
            _ x: Float, _ y: Float, _ width: Float, _ height: Float, _ radii: Float
        ) {
            path_round_rect(
                path,
                x,
                y,
                width,
                height,
                radii,
                radii,
                radii,
                radii
            )
        }
    
    
    
    public func roundRect(
            x: Float, y: Float, width: Float, height: Float, radii: [Float]
        ) {
            var size = radii.count
            var radii = radii
            if (size == 0) {
                return
            }
            
            if(size > 4){
                radii = Array(radii[0...3])
                size = 4
            }
            /*
            [all-corners]
            [top-left-and-bottom-right, top-right-and-bottom-left]
            [top-left, top-right-and-bottom-left, bottom-right]
            [top-left, top-right, bottom-right, bottom-left]
             */
            var topLeft = Float()
            var topRight = Float()
            var bottomRight = Float()
            var bottomLeft = Float()

            switch (size) {
            case 1: do {
                    topLeft = radii[0]
                    topRight = topLeft
                    bottomRight = topLeft
                    bottomLeft = topLeft
                }

            case 2: do {
                    topLeft = radii[0]
                    topRight = radii[1]
                    bottomRight = topLeft
                    bottomLeft = topRight
                }
            case 3: do {
                    topLeft = radii[0]
                    topRight = radii[1]
                    bottomRight = radii[2]
                    bottomLeft = topRight
                }
            case 4: do {
                    topLeft = radii[0]
                    topRight = radii[1]
                    bottomRight = radii[2]
                    bottomLeft = radii[3]
                }
            default: break
                // noop
            }

            path_round_rect(
                path,
                x,
                y,
                width,
                height,
                topLeft,
                topRight,
                bottomRight,
                bottomLeft
            )
        }
    
}

