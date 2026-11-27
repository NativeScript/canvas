// C view of the shared MIT KEMAR set in src-native/common/hrtf_kemar.h.
#import <stdint.h>

#define NSC_HRIR_ELEVATIONS 14

#ifdef __cplusplus
extern "C" {
#endif
extern const int NSCHRIRTaps;
extern const double NSCHRIRRate;
extern const int *const NSCHRIRElevation;
// Azimuths per elevation, 360 / NSCHRIRCircle[row] degrees apart from 0 up to 180.
extern const int *const NSCHRIRAzimuthCount;
extern const int *const NSCHRIRCircle;
// Per elevation, then azimuth: NSCHRIRTaps left samples, then NSCHRIRTaps right.
extern const int16_t *const NSCHRIRData;
#ifdef __cplusplus
}
#endif
