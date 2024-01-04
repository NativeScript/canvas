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
    var gestureRecognizer: TouchGestureRecognizer?
    var panRecognizer: UIPanGestureRecognizer?
    var pinchRecognizer: UIPinchGestureRecognizer?
    
    
    var pointers: [Pointer] = []
    
    private var nextId: Int = 0
    struct Pointer {
        var pointerId: Int
        var location: CGPoint
        var touch: UITouch
    }
   
    
    
    init(canvas: NSCCanvas) {
        view = canvas
        super.init()
        self.gestureRecognizer = TouchGestureRecognizer(target: self, action: nil)
        self.gestureRecognizer?.handler = self
        self.panRecognizer = UIPanGestureRecognizer(target: self, action: #selector(handle))
        self.pinchRecognizer = UIPinchGestureRecognizer(target: self, action: #selector(handle))
        self.pointers.reserveCapacity(10)
    }


    class TouchGestureRecognizer: UIGestureRecognizer {
        var handler: NSCTouchHandler?
        override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent) {

            for touch in touches {
                
               let location = touch.location(in: view)
                
                
                guard let handler = handler else {
                    view?.touchesBegan(touches, with: event)
                    return
                }
                
                let pointer = Pointer(pointerId: handler.nextId, location: location, touch: touch)
                
                handler.pointers.append(pointer)
                handler.onPress(pointer.pointerId, location.x, location.y, self)
                
                view?.touchesBegan(touches, with: event)
                
                handler.nextId += 1
                
            }
        }
        
        override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent) {
            handler?.onMove(self, touches)
            view?.touchesMoved(touches, with: event)
        }
        
        override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent) {
        
            for touch in touches {
                
               let location = touch.location(in: view)
               
                
                guard let handler = handler else {
                    view?.touchesEnded(touches, with: event)
                    return
                }
                
                
                let index = handler.pointers.firstIndex { pointer in
                    return pointer.touch == touch
                }
                
                guard let index = index else {return}
                let pointer = handler.pointers[index]
                
                handler.onRelease(pointer.pointerId, location.x, location.y, self)
                handler.pointers.remove(at: index)
                
                view?.touchesEnded(touches, with: event)
                
            }
        }
        
        override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent) {
            for touch in touches {
                
               let location = touch.location(in: view)
            
                guard let handler = handler else {
                    view?.touchesCancelled(touches, with: event)
                    return
                }
               
                let index = handler.pointers.firstIndex { pointer in
                    return pointer.touch == touch
                }
                
                guard let index = index else {return}
                let pointer = handler.pointers[index]
                
                handler.onCancel(pointer.pointerId, location.x, location.y, self)
                handler.pointers.remove(at: index)
                
                view?.touchesCancelled(touches, with: event)
                
            }
            
        }
    }
    
    @objc func handle(_ gestureRecognizer: UIGestureRecognizer) {
        
      /*  let numberOfTouches = gestureRecognizer.numberOfTouches

        let action = gestureRecognizer.state
        let ptridx = numberOfTouches > 0 ? numberOfTouches - 1 : 0
        
        let press = (action == .began)
        let move = press || action == .changed
        let release = (action == .ended)
        let cancelled = action == .cancelled || action == .failed
    
        let location = numberOfTouches > 1 ?  gestureRecognizer.location(in: gestureRecognizer.view) : gestureRecognizer.location(ofTouch: ptridx, in: gestureRecognizer.view)
                
        let x = location.x
        let y = location.y
        
        guard let me = gestureRecognizer as? UIPinchGestureRecognizer else {
            
            // pan
            if(press){
                pointers[ptridx] = location
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

            if(release || cancelled){
                pointers[ptridx] = CGPointZero
            }
        
            return
        }
        
        if(release || cancelled){
            if(release){
                onRelease(ptridx, x, y, gestureRecognizer)
            }

          
            if(cancelled){
                onCancel(ptridx, x, y, gestureRecognizer)
            }

            if(release || cancelled){
                pointers[ptridx] = CGPointZero
            }
            
            return
        }
        
               
        let start = pointers[ptridx]
        
        let deltaX = x - start.x
        let deltaY = y - start.y
        
   
        
//        let deltaX = pinchRecognizer?.view?.transform.tx ?? 0
//        let deltaY = pinchRecognizer?.view?.transform.ty ?? 0
        let pointerCount = me.numberOfTouches
        var sb = "{"
        append("event", "scale", &sb)
  
        append("deltaX", deltaX, &sb)
        append("deltaY", deltaY, &sb)
                    // todo
        append("deltaZ", 0, &sb)
        append("deltaMode", 0, &sb)
        append("isInProgress", move, &sb)
        

        let last = pointerCount - 1
        sb.append("\"pointers\": [")
        for p in 0..<pointerCount {
            let pid = p
            sb += "{"
            append("ptrId", pid, &sb)
            let tmpLocation = gestureRecognizer.location(ofTouch: p, in: gestureRecognizer.view)
            append("x", tmpLocation.x, &sb)
            append("y", tmpLocation.y, &sb, true)

            if p != last {
                sb += "},"
            } else {
                sb += "}"
            }
        }
        sb += "]"
        
        sb += "}"
        
        view.touchEventListener?(sb, gestureRecognizer)
        
        */
    }
    
    
    private func onPress(
        _ ptrId: Int, _ x: CGFloat, _ y: CGFloat,
        _ gestureRecognizer: UIGestureRecognizer
    ) {
        var sb = "{"
        append("event", "down", &sb)
        append("ptrId", ptrId, &sb)
        append("isPrimary", pointers.first?.pointerId == ptrId, &sb)
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
        append("isPrimary", pointers.first?.pointerId == ptrId, &sb)
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
        append("isPrimary", pointers.first?.pointerId == ptrId, &sb)
        append("x", x, &sb)
        append("y", y, &sb, true)
        sb += "}"

        view.touchEventListener?(sb, gestureRecognizer)
    }
    
    private func onMove(
        _ gestureRecognizer: UIGestureRecognizer,
        _ touches: Set<UITouch>? = nil
        ) {
            
            if(touches != nil){
              /*  for (ptridx, touch) in touches!.enumerated() {
                    
                   let location = touch.location(in: view)
                    
                    handler?.onMove(ptridx, location.x, location.y, self)
                    
                    
                    
                }
                */
                
                var sb = "{"
                append("event", "move", &sb)

                let last = self.pointers.count - 1
                sb.append("\"pointers\": [")
                
                
                for (index, pointer) in self.pointers.enumerated() {
                    let location = pointer.touch.location(in: view)
                    let pointerId = pointer.pointerId
                    sb += "{"
                    append("ptrId", pointerId, &sb)
                    append("isPrimary", index == 0, &sb)
                    append("x", location.x, &sb)
                    append("y", location.y, &sb, true)

                    if index != last {
                        sb += "},"
                    } else {
                        sb += "}"
                    }
                    
                }
                
                
                sb += "]"
                
                sb += "}"
                view.touchEventListener?(sb, gestureRecognizer)
                
                
                return
            }
            
            
          /*  let pointerCount = gestureRecognizer.numberOfTouches

            var sb = "{"
            append("event", "move", &sb)

            let last = pointerCount - 1
            sb.append("\"pointers\": [")
            for p in 0..<pointerCount {
                let pid = p
                sb += "{"
                append("ptrId", pid, &sb)
                let tmpLocation = gestureRecognizer.location(ofTouch: p, in: gestureRecognizer.view)
                append("x", tmpLocation.x, &sb)
                append("y", tmpLocation.y, &sb, true)

                if p != last {
                    sb += "},"
                } else {
                    sb += "}"
                }
            }
            sb += "]"
            
            sb += "}"
            view.touchEventListener?(sb, gestureRecognizer)
            */

        }
    
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
    
    private func append(_ key: String, _ value: Bool, _ sb: inout String, _ isLast: Bool = false) {
        sb += "\"\(key)\": \(value) \(isLast ? "" : "," )"
    }
}
