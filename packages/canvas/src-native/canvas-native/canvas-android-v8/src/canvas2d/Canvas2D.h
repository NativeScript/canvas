//
// Created by Osei Fortune on 22/03/2022.
//

#ifndef CANVAS_NATIVE_CANVAS2D_H
#define CANVAS_NATIVE_CANVAS2D_H

#include "canvas-android-v8/src/Common.h"
#include "Path2D.h"
//#include "CanvasGradient.h"
//#include "CanvasPattern.h"
//#include "CanvasRenderingContext2DImpl.h"
//#include "ImageDataImpl.h"
#include "MatrixImpl.h"

class Canvas2D {
public:
    static void Init(v8::Isolate *isolate);
};


#endif //CANVAS_NATIVE_CANVAS2D_H
