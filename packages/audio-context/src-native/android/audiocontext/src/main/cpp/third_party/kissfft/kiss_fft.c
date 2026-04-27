/* Minimal KissFFT-compatible C implementation.
 * This is a compact, self-contained radix-2 Cooley-Tukey FFT implementation
 * that provides the small API expected by callers in the native analyser.
 */

#include "kiss_fft.h"
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef struct {
    int nfft;
    kiss_fft_cpx *twiddles; // precomputed roots of unity
} kiss_fft_state;

void *kiss_fft_alloc(int nfft, int inverse_fft, void *mem, size_t *lenmem) {
    (void) inverse_fft;
    (void) mem;
    if (nfft <= 0) return NULL;
    kiss_fft_state *st = (kiss_fft_state*)malloc(sizeof(kiss_fft_state));
    if (!st) return NULL;
    st->nfft = nfft;
    st->twiddles = (kiss_fft_cpx*)malloc(sizeof(kiss_fft_cpx) * (size_t)nfft);
    if (!st->twiddles) { free(st); return NULL; }
    for (int k = 0; k < nfft; ++k) {
        float ang = -2.0f * (float)M_PI * (float)k / (float)nfft;
        st->twiddles[k].r = cosf(ang);
        st->twiddles[k].i = sinf(ang);
    }
    if (lenmem) *lenmem = sizeof(kiss_fft_state) + sizeof(kiss_fft_cpx) * (size_t)nfft;
    return st;
}

void kiss_fft_free(void *cfg) {
    if (!cfg) return;
    kiss_fft_state *st = (kiss_fft_state*)cfg;
    if (st->twiddles) free(st->twiddles);
    free(st);
}

static inline void cpx_copy(kiss_fft_cpx *dst, const kiss_fft_cpx *src) {
    dst->r = src->r; dst->i = src->i;
}

void kiss_fft(void *cfgv, const kiss_fft_cpx *fin, kiss_fft_cpx *fout) {
    if (!cfgv || !fin || !fout) return;
    kiss_fft_state *st = (kiss_fft_state*)cfgv;
    int n = st->nfft;

    // Work buffer
    kiss_fft_cpx *a = (kiss_fft_cpx*)malloc(sizeof(kiss_fft_cpx) * (size_t)n);
    if (!a) return;

    // Copy input into buffer
    for (int i = 0; i < n; ++i) cpx_copy(&a[i], &fin[i]);

    // Bit reversal permutation (in-place)
    int j = 0;
    for (int i = 1; i < n; ++i) {
        int bit = n >> 1;
        for (; j & bit; bit >>= 1) j ^= bit;
        j ^= bit;
        if (i < j) {
            kiss_fft_cpx tmp = a[i];
            a[i] = a[j];
            a[j] = tmp;
        }
    }

    // Cooley-Tukey iterative FFT using precomputed twiddles
    for (int len = 2; len <= n; len <<= 1) {
        int half = len >> 1;
        int step = n / len;
        for (int i = 0; i < n; i += len) {
            for (int k = 0; k < half; ++k) {
                const kiss_fft_cpx *w = &st->twiddles[(size_t)(k * step)];
                kiss_fft_cpx u = a[i + k];
                kiss_fft_cpx v = a[i + k + half];
                // v *= w
                float vr = v.r * w->r - v.i * w->i;
                float vi = v.r * w->i + v.i * w->r;
                // a[i+k] = u + v*w
                a[i + k].r = u.r + vr;
                a[i + k].i = u.i + vi;
                // a[i+k+half] = u - v*w
                a[i + k + half].r = u.r - vr;
                a[i + k + half].i = u.i - vi;
            }
        }
    }

    // Copy buffer to output
    memcpy(fout, a, sizeof(kiss_fft_cpx) * (size_t)n);
    free(a);
}
