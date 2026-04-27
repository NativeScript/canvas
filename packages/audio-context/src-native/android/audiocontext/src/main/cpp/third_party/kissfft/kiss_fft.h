/* Minimal KissFFT-compatible header (small integration)
 * Provides a tiny kiss FFT API used by the native analyser.
 */
#ifndef KISS_FFT_H
#define KISS_FFT_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

typedef struct { float r; float i; } kiss_fft_cpx;

/* Allocate a FFT config. 'inverse_fft' is ignored for this simple implementation.
 * If 'mem' is NULL the function allocates state with malloc and returns pointer.
 */
void *kiss_fft_alloc(int nfft, int inverse_fft, void *mem, size_t *lenmem);

/* Perform FFT: cfg from kiss_fft_alloc, input array 'fin' length nfft, output 'fout'. */
void kiss_fft(void *cfg, const kiss_fft_cpx *fin, kiss_fft_cpx *fout);

/* Free the cfg if allocated by kiss_fft_alloc. */
void kiss_fft_free(void *cfg);

#ifdef __cplusplus
}
#endif

#endif // KISS_FFT_H
