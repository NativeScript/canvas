//
//  Utils.m
//  CanvasNative
//
//  Created by Osei Fortune on 5/1/20.
//

#import <Foundation/Foundation.h>

void offsetU8By(uint8_t *data, int offset){
    data += offset;
}

void offsetI8By(int8_t *data, int offset){
    data += offset;
}

void offsetU16By(uint16_t *data, int offset){
    data += offset;
}

void offsetI16By(int16_t *data, int offset){
    data += offset;
}


void offsetU32By(uint32_t *data, int offset){
    data += offset;
}

void offsetI32By(int32_t *data, int offset){
    data += offset;
}


void offsetF32By(float *data, int offset){
    data += offset;
}

void offsetF64By(double *data, int offset){
    data += offset;
}
