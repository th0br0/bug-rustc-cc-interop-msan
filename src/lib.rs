#[allow(dead_code)]
pub enum Padded {
    Small([u8; 1]),
    Large([u32; 1]),
}

#[repr(C)]
#[allow(dead_code)]
pub struct PaddedStruct {
    pub a: u8,
    // 3 bytes of uninitialized padding here before the aggregate field
    pub b: [u32; 1],
}

#[repr(C)]
pub struct Large(pub [u64; 3]);

#[inline(never)]
pub fn pass_padded(_: Padded) {}

#[inline(never)]
pub fn pass_padded_struct(_: PaddedStruct) {}

#[inline(never)]
pub fn pass_option(_: Option<[u32; 1]>) {}

unsafe extern "C" {
    pub fn cpp_return_large(x: u64) -> Large;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Control test: calling C++ `sret` function directly succeeds when `__msan_param_tls[0]` is clean.
    #[test]
    fn test_control_clean_tls_passes() {
        let res = unsafe { cpp_return_large(42) };
        assert_eq!(res.0[0], 42);
    }

    /// Variant 1: Passing an 8-byte enum with uninitialized padding leaves stale
    /// shadow in `__msan_param_tls[0]`, causing the subsequent C++ `sret` function to trap.
    #[test]
    fn test_msan_repro_padded_enum() {
        pass_padded(Padded::Small([1]));
        let _ = unsafe { cpp_return_large(42) };
    }

    /// Variant 2: passing an 8-byte struct with internal padding bytes (`PassMode::Cast(i64)`)
    /// leaves stale shadow in `__msan_param_tls[0]`, causing the subsequent C++ `sret` function to trap.
    #[test]
    fn test_msan_repro_padded_struct() {
        pass_padded_struct(PaddedStruct { a: 1, b: [2] });
        let _ = unsafe { cpp_return_large(42) };
    }

    /// Variant 3: passing `Option<[u32; 1]>::None` (8-byte aggregate enum with uninitialized payload)
    /// leaves stale shadow in `__msan_param_tls[0]`, causing the subsequent C++ `sret` function to trap.
    #[test]
    fn test_msan_repro_option_none() {
        pass_option(None);
        let _ = unsafe { cpp_return_large(42) };
    }
}
