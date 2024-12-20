#ifndef CANVAS_SVG_IOS_H
#define CANVAS_SVG_IOS_H

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(TARGET_OS_IOS)
void canvas_native_svg_draw_from_bytes(uint8_t *data,
                                       uintptr_t size,
                                       float width,
                                       float height,
                                       float scale,
                                       const uint8_t *svg_data,
                                       uintptr_t svg_size);
#endif

#if defined(TARGET_OS_IOS)
void canvas_native_svg_draw_from_string(uint8_t *data,
                                        uintptr_t size,
                                        float width,
                                        float height,
                                        float scale,
                                        const char *svg);
#endif

#if defined(TARGET_OS_IOS)
void canvas_native_svg_draw_from_path(uint8_t *data,
                                      uintptr_t size,
                                      float width,
                                      float height,
                                      float scale,
                                      const char *path);
#endif

#endif /* CANVAS_SVG_IOS_H */
