use std::ops::Shl;

pub(crate) trait Mask {
    fn is_lit<N>(&self, bit: N) -> bool
    where
        Self: Shl<N, Output = Self>;
}

impl Mask for u64 {
    #[inline(always)]
    fn is_lit<N>(&self, bit: N) -> bool
    where
        Self: Shl<N, Output = Self>,
    {
        (*self & (1 << bit)) != 0
    }
}

impl Mask for u32 {
    #[inline(always)]
    fn is_lit<N>(&self, bit: N) -> bool
    where
        Self: Shl<N, Output = Self>,
    {
        (*self & (1 << bit)) != 0
    }
}

impl Mask for usize {
    #[inline(always)]
    fn is_lit<N>(&self, bit: N) -> bool
    where
        Self: Shl<N, Output = Self>,
    {
        (*self & (1 << bit)) != 0
    }
}

#[cfg(target_arch = "x86")]
pub(crate) mod m32 {
    pub(crate) fn combine_16(m1: u16, m2: u16) -> u32 {
        u32::from(m1) | (u32::from(m2) << 16)
    }
}

#[cfg(target_arch = "x86_64")]
pub(crate) mod m64 {
    pub(crate) fn combine_16(m1: u16, m2: u16, m3: u16, m4: u16) -> u64 {
        u64::from(m1) | (u64::from(m2) << 16) | (u64::from(m3) << 32) | (u64::from(m4) << 48)
    }

    pub(crate) fn combine_32(m1: u32, m2: u32) -> u64 {
        u64::from(m1) | (u64::from(m2) << 32)
    }
}
