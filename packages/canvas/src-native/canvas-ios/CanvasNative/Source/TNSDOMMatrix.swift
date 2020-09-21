//
//  TNSDOMMatrix.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/26/20.
//

import Foundation
@objcMembers
@objc(TNSDOMMatrix)
public class TNSDOMMatrix: NSObject {
    public var a: Float = 1.0 {
        didSet {
            if (internalUpdate) {return}
            updateValues()
        }
    }
    public var b: Float = 0 {
        didSet {
            if (internalUpdate) {return}
            updateValues()
        }
    }
    public var c: Float = 0 {
        didSet {
            if (internalUpdate) {return}
            updateValues()
        }
    }
    public var d: Float = 1.0 {
        didSet {
            if (internalUpdate) {return}
            updateValues()
        }
    }
    public var e: Float = 0 {
        didSet {
            if (internalUpdate) {return}
            updateValues()
        }
    }
    public var f: Float = 0 {
        didSet {
            if (internalUpdate) {return}
            updateValues()
        }
    }
    var internalUpdate: Bool = false
    var matrix: Int64 = 0
    public override init() {
        super.init()
        matrix = native_create_matrix()
        setInitialValues()
    }
    
    init(matrix: Int64) {
        super.init()
        self.matrix = matrix
        setInitialValues()
    }
    func setInitialValues() {
        internalUpdate = true
        refreshValues()
        internalUpdate = false
    }
   
    func refreshValues(){
        let matrixValues = native_get_matrix(matrix)
        if(matrixValues != nil){
            let pointer = matrixValues!.pointee
            let values = Array(UnsafeBufferPointer(start: pointer.array.assumingMemoryBound(to: Float.self), count: pointer.length))
                   native_free_matrix_data(matrixValues)
                 internalUpdate = true
                   a = values[0]
                   b = values[1]
                   c = values[2]
                   d = values[3]
                   e = values[4]
                   f = values[5]
                   internalUpdate = false
        }
    }
    
    func updateValues() {
        if(matrix == 0) {return}
        var values = [a,b,c,d,e,f]
        matrix = native_set_matrix(matrix,&values ,values.count)
        refreshValues()
    }
}
