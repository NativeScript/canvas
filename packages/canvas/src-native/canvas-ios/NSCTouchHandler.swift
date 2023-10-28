//
//  NSCTouchHandler.swift
//  CanvasNative
//
//  Created by Osei Fortune on 25/10/2023.
//

import Foundation
import UIKit
class NSCTouchHandler: NSObject {

    private let view: NSCCanvas
    var gestureRecognizer: UIGestureRecognizer?
    var panRecognizer: UIPanGestureRecognizer?
    var pinchRecognizer: UIPinchGestureRecognizer?
    init(canvas: NSCCanvas) {
        view = canvas
        super.init()
        self.gestureRecognizer = UITapGestureRecognizer(target: self, action: #selector(handle))
        self.panRecognizer = UIPanGestureRecognizer(target: self, action: #selector(handle))
        self.pinchRecognizer = UIPinchGestureRecognizer(target: self, action: #selector(handle))
    }

    var pointersBegan: [CGPoint] = []
    @objc func handle(_ gestureRecognizer: UIGestureRecognizer) {

        let action = gestureRecognizer.state
        let ptridx = gestureRecognizer.numberOfTouches > 0 ? gestureRecognizer.numberOfTouches - 1 : 0
        let x = gestureRecognizer.location(ofTouch: ptridx, in: gestureRecognizer.view).x
        let y = gestureRecognizer.location(ofTouch: ptridx, in: gestureRecognizer.view).y
        
        let press = (action == .began)
        let move = press || action == .changed
        let release = (action == .ended)
        let cancelled = action == .cancelled || action == .failed
      
        
//        guard let me = gestureRecognizer as? UIPanGestureRecognizer else {
//
//            return
//        }
        
        
        
        guard let me = gestureRecognizer as? UIPinchGestureRecognizer else {
            if(press){
                onPress(ptridx, x, y, gestureRecognizer)
            }
            
            if(release){
                onRelease(ptridx, x, y, gestureRecognizer)
            }
            
            if(move){
                onMove(gestureRecognizer)
            }
        
            if(cancelled){
                onCancel(ptridx, x, y, gestureRecognizer)
            }
        
            return
        }
        
        
        let count = me.numberOfTouches
        for i in 0..<count {}

        
    }
    
    private func onPress(
        _ ptrId: Int, _ x: CGFloat, _ y: CGFloat,
        _ gestureRecognizer: UIGestureRecognizer
    ) {
        var sb = "{"
        append("event", "down", &sb)
        append("ptrId", ptrId, &sb)
        append("x", x, &sb)
        append("y", y, &sb, true)

        sb += "}"

        view.touchEventListener?(sb, gestureRecognizer)

    }
    
    private func onRelease(
        _ ptrId: Int, _ x: CGFloat, _ y: CGFloat,
        _ gestureRecognizer: UIGestureRecognizer
    ) {
        var sb = "{"
        append("event", "up", &sb)
        append("ptrId", ptrId, &sb)
        append("x", x, &sb)
        append("y", y, &sb, true)

        sb += "}"

        view.touchEventListener?(sb, gestureRecognizer)
    }

    private func onCancel(
        _ ptrId: Int, _ x: CGFloat, _ y: CGFloat,
        _ gestureRecognizer: UIGestureRecognizer
    ) {
        var sb = "{"
        append("event", "cancel", &sb)
        append("ptrId", ptrId, &sb)
        append("x", x, &sb)
        append("y", y, &sb, true)

        sb += "}"

        view.touchEventListener?(sb, gestureRecognizer)
    }
    
    private func onMove(
        _ gestureRecognizer: UIGestureRecognizer
        ) {
            
            
            let pointerCount = gestureRecognizer.numberOfTouches

           
            var sb = "{"
            append("event", "move", &sb)


            let last = pointerCount - 1
            sb.append(",\"pointers\": [")
            for p in 0..<pointerCount {
                let pid = p //gestureRecognizer.hashValue
                sb += "{"
                append("event", "touchCoordinatesCallback", &sb)
                append("ptrId", pid, &sb)
                append("x", gestureRecognizer.location(ofTouch: p, in: gestureRecognizer.view).x, &sb)
                append("y", gestureRecognizer.location(ofTouch: p, in: gestureRecognizer.view).y, &sb, true)

                if p != last {
                    sb += "},"
                } else {
                    sb += "}"
                }
            }
            sb += "]"
            
            sb += "}"
            view.touchEventListener?(sb, gestureRecognizer)

        }
    
    
/*
    private func onTouchEvent(_ ptrid: Int, x: CGFloat, y: CGFloat, press: Bool, release: Bool, gestureRecognizer: UIGestureRecognizer) {
        var sb = "{"
        append("event", "mouseMoveCallback", &sb)
        append("ptrId", ptrid, &sb)
        append("x", x, &sb)
        append("y", y, &sb, true)

        if press {
            sb += ",\"press\": {"
            append("event", "mouseDownCallback", &sb)
            append("ptrId", ptrid, &sb, true)
            sb += "}"
        }

        if release {
            sb += ",\"release\": {"
            append("event", "mouseUpCallback", &sb)
            append("ptrId", ptrid, &sb, true)
            sb += "}"
        }

        sb += "}"

        view.touchEventListener?(sb, gestureRecognizer)
    }

    private func onMultitouchCoordinates(_ ptrid: Int, x: CGFloat, y: CGFloat, gestureRecognizer: UIPanGestureRecognizer) {
        var sb = "{"
        append("event", "touchCoordinatesCallback", &sb)
        append("ptrId", ptrid, &sb)
        append("x", x, &sb)
        append("y", y, &sb, true)
        sb += "}"

        view.touchEventListener?(sb, gestureRecognizer)
    }

    private func onMultitouchCoordinates(gestureRecognizer: UIPanGestureRecognizer) {
        let pointerCount = gestureRecognizer.numberOfTouches

        if pointerCount == 0 {
            return
        }

        let last = pointerCount - 1
        var sb = "["
        for p in 0..<pointerCount {
            let pid = gestureRecognizer.hashValue
            sb += "{"
            append("event", "touchCoordinatesCallback", &sb)
            append("ptrId", pid, &sb)
            append("x", gestureRecognizer.location(ofTouch: p, in: gestureRecognizer.view).x, &sb)
            append("y", gestureRecognizer.location(ofTouch: p, in: gestureRecognizer.view).y, &sb, true)

            if p != last {
                sb += "},"
            } else {
                sb += "}"
            }
        }
        sb += "]"
        view.touchEventListener?(sb, gestureRecognizer)
    }
 
 */

    private func append(_ key: String, _ value: String, _ sb: inout String, _ isLast: Bool = false) {
        sb += "\"\(key)\": \"\(value)\"\(isLast ? "" : ",")"
    }

    private func append(_ key: String, _ value: Int, _ sb: inout String, _ isLast: Bool = false) {
        sb += "\"\(key)\": \(value) \(isLast ? "" : "," )"
    }

    private func append(_ key: String, _ value: CGFloat, _ sb: inout String, _ isLast: Bool = false) {
        sb += "\"\(key)\": \(value) \(isLast ? "" : "," )"
    }
    
    private func append(_ key: String, _ value: Float, _ sb: inout String, _ isLast: Bool = false) {
        sb += "\"\(key)\": \(value) \(isLast ? "" : "," )"
    }
}
