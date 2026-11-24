#[cfg(target_arch = "x86_64")]
use std::arch::asm;

/// x86_64 inline assembly for fast inverse square root (1.0 / sqrt(x)) using SSE `rsqrtss`
#[inline(always)]
pub fn fast_inv_sqrt(val: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        let result: f32;
        unsafe {
            asm!(
                "rsqrtss {0}, {1}",
                out(xmm_reg) result,
                in(xmm_reg) val,
                options(pure, nomem, nostack)
            );
        }
        result
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        1.0 / val.sqrt()
    }
}

/// x86_64 inline assembly reading the Time Stamp Counter (RDTSC) for CPU cycle profiling
#[inline(always)]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let low: u32;
        let high: u32;
        unsafe {
            asm!(
                "rdtsc",
                out("eax") low,
                out("edx") high,
                options(nomem, nostack)
            );
        }
        ((high as u64) << 32) | (low as u64)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

/// Fast float-to-int conversion with truncation using x86 SSE `cvttss2si`
#[inline(always)]
pub fn fast_float_to_int(val: f32) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let result: i32;
        unsafe {
            asm!(
                "cvttss2si {0:e}, {1}",
                out(reg) result,
                in(xmm_reg) val,
                options(pure, nomem, nostack)
            );
        }
        result
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        val as i32
    }
}
