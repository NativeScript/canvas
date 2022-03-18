//
//  TNSCanvasData.swift
//  CanvasNative
//
//  Created by Osei Fortune on 09/03/2022.
//

import Foundation

@objc(TNSCanvasData)
public class TNSCanvasData: NSMutableData {
    private var data: UnsafeMutablePointer<U8Array>
    
    public init(data: UnsafeMutablePointer<U8Array>) {
        self.data = data
        super.init(bytesNoCopy: data.pointee.data, length: Int(data.pointee.data_len), freeWhenDone: false)
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    deinit {
        destroy_u8_array(data)
    }
                   
}
