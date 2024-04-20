macro_rules! vec256 {
    (
        $Self:ident,
        $Unit:ty,
        $Len:literal,
        $mod:ident
    ) => {
        /// 256 bit vector suitable for SIMD operations.
        ///
        /// The structure is used to define constant vectors.
        #[derive(Debug, PartialEq, Copy, Clone)]
        #[repr(align(32))]
        pub struct $Self([$Unit; $Len]);

        impl $Self {
            /// Copy content of the array into a new vector
            #[inline(always)]
            pub const fn from_array(arr: [$Unit; $Len]) -> Self {
                Self(arr)
            }

            /// Copy content of the vector and return as array
            #[inline(always)]
            pub const fn into_array(&self) -> [$Unit; $Len] {
                self.0
            }
        }

        impl core::ops::Deref for $Self {
            type Target = [$Unit; $Len];
            
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $Self {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        #[cfg(target_arch = "x86")]
        impl Into<core::arch::x86::__m256i> for $Self {
            #[inline(always)]
            fn into(self) -> core::arch::x86::__m256i {
                unsafe { core::mem::transmute(self.0) }
            }
        }

        #[cfg(target_arch = "x86")]
        impl Into<$Self> for core::arch::x86::__m256i {
            #[inline(always)]
            fn into(self) -> $Self {
                unsafe { core::mem::transmute(self.0) }
            }
        }

        #[cfg(target_arch = "x86_64")]
        impl Into<core::arch::x86_64::__m256i> for $Self {
            #[inline(always)]
            fn into(self) -> core::arch::x86_64::__m256i {
                unsafe { core::mem::transmute(self.0) }
            }
        }

        #[cfg(target_arch = "x86_64")]
        impl Into<$Self> for core::arch::x86_64::__m256i {
            #[inline(always)]
            fn into(self) -> $Self {
                unsafe { core::mem::transmute(self) }
            }
        }

        #[cfg(test)]
        mod $mod {
            use super::$Self;

            #[cfg(target_arch = "x86")]
            #[test]
            fn into_m256i() {
                let zeros = $Self::from_array([0; $Len]);
                let m256i: core::arch::x86::__m256i = zeros.into();
                let struc: $Self = m256i.into();
                assert_eq!(zeros, struc);
            }

            #[cfg(target_arch = "x86_64")]
            #[test]
            fn into_m256i() {
                let zeros = $Self::from_array([0; $Len]);
                let m256i: core::arch::x86_64::__m256i = zeros.into();
                let struc: $Self = m256i.into();
                assert_eq!(zeros, struc);
            }
        }
    };
}

vec256!( I8x32, i8, 32, i8x32 );
vec256!( I16x16, i16, 16, i16x16 );
vec256!( I32x8, i32, 8, i32x8 );
vec256!( I64x4, i64, 4, i64x4 );

impl Into<I16x16> for I8x32 {
    #[inline(always)]
    fn into(self) -> I16x16 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I32x8> for I8x32 {
    #[inline(always)]
    fn into(self) -> I32x8 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I64x4> for I8x32 {
    #[inline(always)]
    fn into(self) -> I64x4 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I8x32> for I16x16 { 
    #[inline(always)]
    fn into(self) -> I8x32 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I32x8> for I16x16 {
    #[inline(always)]
    fn into(self) -> I32x8 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I64x4> for I16x16 {
    #[inline(always)]
    fn into(self) -> I64x4 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I8x32> for I32x8 {
    #[inline]
    fn into(self) -> I8x32 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I16x16> for I32x8 {
    #[inline]
    fn into(self) -> I16x16 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I64x4> for I32x8 {
    #[inline]
    fn into(self) -> I64x4 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I8x32> for I64x4 {
    #[inline(always)]
    fn into(self) -> I8x32 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I16x16> for I64x4 {
    #[inline(always)]
    fn into(self) -> I16x16 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Into<I32x8> for I64x4 {
    #[inline(always)]
    fn into(self) -> I32x8 {
        unsafe { core::mem::transmute(self) }
    }
}
