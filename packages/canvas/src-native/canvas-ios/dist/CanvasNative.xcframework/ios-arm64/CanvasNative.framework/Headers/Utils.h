//
//  Utils.h
//  Pods
//
//  Created by Osei Fortune on 5/1/20.
//

#ifndef Utils_h
#define Utils_h

void offsetU8By(uint8_t *data, int offset);

void offsetI8By(int8_t *data, int offset);

void offsetU16By(uint16_t *data, int offset);

void offsetI16By(int16_t *data, int offset);

void offsetU32By(uint32_t *data, int offset);

void offsetI32By(int32_t *data, int offset);

void offsetF32By(float *data, int offset);

void offsetF64By(double *data, int offset);
#endif /* Utils_h */
