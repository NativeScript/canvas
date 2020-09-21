//
//  Extensions.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/17/20.
//

import Foundation


extension RangeReplaceableCollection {
    public mutating func resize(_ size: Int, fillWith value: Iterator.Element) {
        let c = count
        if c < size {
            append(contentsOf: repeatElement(value, count: c.distance(to: size)))
        } else if c > size {
            let newEnd = index(startIndex, offsetBy: size)
            removeSubrange(newEnd ..< endIndex)
        }
    }
}
