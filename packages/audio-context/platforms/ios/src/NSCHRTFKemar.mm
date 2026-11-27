#import "NSCHRTFKemar.h"
#include "hrtf_kemar.h"

static_assert(kemar::kRows == NSC_HRIR_ELEVATIONS, "KEMAR elevation rows");

const int NSCHRIRTaps = kemar::kLength;
const double NSCHRIRRate = kemar::kRate;
const int *const NSCHRIRElevation = kemar::kElevation;
const int *const NSCHRIRAzimuthCount = kemar::kAzimuths;
const int *const NSCHRIRCircle = kemar::kCircle;
const int16_t *const NSCHRIRData = kemar::kData;
