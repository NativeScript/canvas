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
    var matrix: Int64
    override public init() {
        matrix = matrix_create()
    }
    
    public var a: Float{
        set {
            matrix_set_a(matrix,newValue)
        }
        get {
            matrix_a(matrix)
        }
    }
    
    public var b: Float{
        set {
            matrix_set_b(matrix,newValue)
        }
        get {
            matrix_b(matrix)
        }
    }
    
    public var c: Float{
        set {
            matrix_set_c(matrix,newValue)
        }
        get {
            matrix_c(matrix)
        }
    }
    public var d: Float{
        set {
            matrix_set_d(matrix,newValue)
        }
        get {
            matrix_d(matrix)
        }
    }
    
    
    public var e: Float{
        set {
            matrix_set_e(matrix,newValue)
        }
        get {
            matrix_e(matrix)
        }
    }
    
    public var f: Float{
        set {
            matrix_set_f(matrix,newValue)
        }
        get {
            matrix_f(matrix)
        }
    }
    
    
    public var m11: Float{
        set {
            matrix_set_m11(matrix,newValue)
        }
        get {
            matrix_m11(matrix)
        }
    }
    
    public var m12: Float{
        set {
            matrix_set_m12(matrix,newValue)
        }
        get {
            matrix_m12(matrix)
        }
    }
    
    public var m13: Float{
        set {
            matrix_set_m13(matrix,newValue)
        }
        get {
            matrix_m13(matrix)
        }
    }
    public var m14: Float{
        set {
            matrix_set_m14(matrix,newValue)
        }
        get {
            matrix_m14(matrix)
        }
    }
    
    
    public var m21: Float{
        set {
            matrix_set_m21(matrix,newValue)
        }
        get {
            matrix_m21(matrix)
        }
    }
    
    public var m22: Float{
        set {
            matrix_set_m22(matrix,newValue)
        }
        get {
            matrix_m22(matrix)
        }
    }
    
    
    public var m23: Float{
        set {
            matrix_set_m23(matrix,newValue)
        }
        get {
            matrix_m23(matrix)
        }
    }
    
    public var m24: Float{
        set {
            matrix_set_m24(matrix,newValue)
        }
        get {
            matrix_m24(matrix)
        }
    }
    
    
    public var m31: Float{
        set {
            matrix_set_m31(matrix,newValue)
        }
        get {
            matrix_m31(matrix)
        }
    }
    
    public var m32: Float{
        set {
            matrix_set_m32(matrix,newValue)
        }
        get {
            matrix_m32(matrix)
        }
    }
    
    public var m33: Float{
        set {
            matrix_set_m33(matrix,newValue)
        }
        get {
            matrix_m33(matrix)
        }
    }
    public var m34: Float{
        set {
            matrix_set_m34(matrix,newValue)
        }
        get {
            matrix_m34(matrix)
        }
    }
    
    
    public var m41: Float{
        set {
            matrix_set_m41(matrix,newValue)
        }
        get {
            matrix_m41(matrix)
        }
    }
    
    public var m42: Float{
        set {
            matrix_set_m42(matrix,newValue)
        }
        get {
            matrix_m42(matrix)
        }
    }
    
    
    public var m43: Float{
        set {
            matrix_set_m43(matrix,newValue)
        }
        get {
            matrix_m43(matrix)
        }
    }
    
    public var m44: Float{
        set {
            matrix_set_m44(matrix,newValue)
        }
        get {
            matrix_m44(matrix)
        }
    }
    
    init(matrix: Int64) {
        self.matrix = matrix
    }
    
    deinit {
        destroy_matrix(matrix)
        matrix = 0
    }
}
