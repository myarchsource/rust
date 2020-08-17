use crate::{
    core_arch::{simd::*, x86::*},
    mem::transmute,
};

/// Sets packed 64-bit integers in `dst` with the supplied values.
///
/// [Intel's documentation]( https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,4909&text=_mm512_set_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_set_epi64(
    e0: i64,
    e1: i64,
    e2: i64,
    e3: i64,
    e4: i64,
    e5: i64,
    e6: i64,
    e7: i64,
) -> __m512i {
    _mm512_setr_epi64(e7, e6, e5, e4, e3, e2, e1, e0)
}

/// Sets packed 64-bit integers in `dst` with the supplied values in
/// reverse order.
///
/// [Intel's documentation]( https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,4909&text=_mm512_set_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_setr_epi64(
    e0: i64,
    e1: i64,
    e2: i64,
    e3: i64,
    e4: i64,
    e5: i64,
    e6: i64,
    e7: i64,
) -> __m512i {
    let r = i64x8::new(e0, e1, e2, e3, e4, e5, e6, e7);
    transmute(r)
}

#[cfg(test)]
mod tests {
    use std;
    use stdarch_test::simd_test;

    use crate::core_arch::x86::*;
    use crate::core_arch::x86_64::*;

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmplt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmplt_epu64_mask(a, b);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmplt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        let r = _mm512_mask_cmplt_epu64_mask(mask, a, b);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpgt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmpgt_epu64_mask(b, a);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpgt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        let r = _mm512_mask_cmpgt_epu64_mask(mask, b, a);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmple_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmple_epu64_mask(a, b), _mm512_cmpgt_epu64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmple_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmple_epu64_mask(mask, a, b),
            _mm512_mask_cmpgt_epu64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpge_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmpge_epu64_mask(a, b), _mm512_cmplt_epu64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpge_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmpge_epu64_mask(mask, a, b),
            _mm512_mask_cmplt_epu64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpeq_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let m = _mm512_cmpeq_epu64_mask(b, a);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpeq_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let mask = 0b01111010;
        let r = _mm512_mask_cmpeq_epu64_mask(mask, b, a);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmplt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmplt_epi64_mask(a, b);
        assert_eq!(m, 0b00000101);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmplt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01100110;
        let r = _mm512_mask_cmplt_epi64_mask(mask, a, b);
        assert_eq!(r, 0b00000100);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpgt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmpgt_epi64_mask(b, a);
        assert_eq!(m, 0b00000101);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpgt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01100110;
        let r = _mm512_mask_cmpgt_epi64_mask(mask, b, a);
        assert_eq!(r, 0b00000100);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmple_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmple_epi64_mask(a, b), _mm512_cmpgt_epi64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmple_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmple_epi64_mask(mask, a, b),
            _mm512_mask_cmpgt_epi64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpge_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmpge_epi64_mask(a, b), _mm512_cmplt_epi64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpge_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmpge_epi64_mask(mask, a, b),
            _mm512_mask_cmplt_epi64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpeq_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let m = _mm512_cmpeq_epi64_mask(b, a);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpeq_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let mask = 0b01111010;
        let r = _mm512_mask_cmpeq_epi64_mask(mask, b, a);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_set_epi64() {
        let r = _mm512_setr_epi64(0, 1, 2, 3, 4, 5, 6, 7);
        assert_eq_m512i(r, _mm512_set_epi64(7, 6, 5, 4, 3, 2, 1, 0))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_setr_epi64() {
        let r = _mm512_set_epi64(0, 1, 2, 3, 4, 5, 6, 7);
        assert_eq_m512i(r, _mm512_setr_epi64(7, 6, 5, 4, 3, 2, 1, 0))
    }
}
