// This file was generated

mod non_zero_i128_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroI128Ext: non_zero_i128_private::Sealed {
    /// The negation of [`is_negative`](std::num::NonZero::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::NonZero::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl non_zero_i128_private::Sealed for std::num::NonZero<i128> { }

impl IsntNonZeroI128Ext for std::num::NonZero<i128> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod non_zero_i16_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroI16Ext: non_zero_i16_private::Sealed {
    /// The negation of [`is_negative`](std::num::NonZero::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::NonZero::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl non_zero_i16_private::Sealed for std::num::NonZero<i16> { }

impl IsntNonZeroI16Ext for std::num::NonZero<i16> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod non_zero_i32_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroI32Ext: non_zero_i32_private::Sealed {
    /// The negation of [`is_negative`](std::num::NonZero::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::NonZero::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl non_zero_i32_private::Sealed for std::num::NonZero<i32> { }

impl IsntNonZeroI32Ext for std::num::NonZero<i32> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod non_zero_i64_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroI64Ext: non_zero_i64_private::Sealed {
    /// The negation of [`is_negative`](std::num::NonZero::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::NonZero::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl non_zero_i64_private::Sealed for std::num::NonZero<i64> { }

impl IsntNonZeroI64Ext for std::num::NonZero<i64> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod non_zero_i8_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroI8Ext: non_zero_i8_private::Sealed {
    /// The negation of [`is_negative`](std::num::NonZero::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::NonZero::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl non_zero_i8_private::Sealed for std::num::NonZero<i8> { }

impl IsntNonZeroI8Ext for std::num::NonZero<i8> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod non_zero_isize_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroIsizeExt: non_zero_isize_private::Sealed {
    /// The negation of [`is_negative`](std::num::NonZero::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::NonZero::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl non_zero_isize_private::Sealed for std::num::NonZero<isize> { }

impl IsntNonZeroIsizeExt for std::num::NonZero<isize> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod non_zero_u128_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroU128Ext: non_zero_u128_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::NonZero::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl non_zero_u128_private::Sealed for std::num::NonZero<u128> { }

impl IsntNonZeroU128Ext for std::num::NonZero<u128> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod non_zero_u16_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroU16Ext: non_zero_u16_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::NonZero::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl non_zero_u16_private::Sealed for std::num::NonZero<u16> { }

impl IsntNonZeroU16Ext for std::num::NonZero<u16> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod non_zero_u32_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroU32Ext: non_zero_u32_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::NonZero::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl non_zero_u32_private::Sealed for std::num::NonZero<u32> { }

impl IsntNonZeroU32Ext for std::num::NonZero<u32> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod non_zero_u64_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroU64Ext: non_zero_u64_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::NonZero::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl non_zero_u64_private::Sealed for std::num::NonZero<u64> { }

impl IsntNonZeroU64Ext for std::num::NonZero<u64> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod non_zero_u8_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroU8Ext: non_zero_u8_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::NonZero::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl non_zero_u8_private::Sealed for std::num::NonZero<u8> { }

impl IsntNonZeroU8Ext for std::num::NonZero<u8> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod non_zero_usize_private { pub trait Sealed { } }

/// Extension for [`NonZero`](std::num::NonZero)
pub trait IsntNonZeroUsizeExt: non_zero_usize_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::NonZero::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl non_zero_usize_private::Sealed for std::num::NonZero<usize> { }

impl IsntNonZeroUsizeExt for std::num::NonZero<usize> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod saturating_i128_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingI128Ext: saturating_i128_private::Sealed {
    /// The negation of [`is_negative`](std::num::Saturating::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::Saturating::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl saturating_i128_private::Sealed for std::num::Saturating<i128> { }

impl IsntSaturatingI128Ext for std::num::Saturating<i128> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod saturating_i16_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingI16Ext: saturating_i16_private::Sealed {
    /// The negation of [`is_negative`](std::num::Saturating::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::Saturating::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl saturating_i16_private::Sealed for std::num::Saturating<i16> { }

impl IsntSaturatingI16Ext for std::num::Saturating<i16> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod saturating_i32_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingI32Ext: saturating_i32_private::Sealed {
    /// The negation of [`is_negative`](std::num::Saturating::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::Saturating::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl saturating_i32_private::Sealed for std::num::Saturating<i32> { }

impl IsntSaturatingI32Ext for std::num::Saturating<i32> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod saturating_i64_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingI64Ext: saturating_i64_private::Sealed {
    /// The negation of [`is_negative`](std::num::Saturating::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::Saturating::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl saturating_i64_private::Sealed for std::num::Saturating<i64> { }

impl IsntSaturatingI64Ext for std::num::Saturating<i64> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod saturating_i8_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingI8Ext: saturating_i8_private::Sealed {
    /// The negation of [`is_negative`](std::num::Saturating::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::Saturating::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl saturating_i8_private::Sealed for std::num::Saturating<i8> { }

impl IsntSaturatingI8Ext for std::num::Saturating<i8> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod saturating_isize_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingIsizeExt: saturating_isize_private::Sealed {
    /// The negation of [`is_negative`](std::num::Saturating::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](std::num::Saturating::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl saturating_isize_private::Sealed for std::num::Saturating<isize> { }

impl IsntSaturatingIsizeExt for std::num::Saturating<isize> {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod saturating_u128_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingU128Ext: saturating_u128_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::Saturating::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl saturating_u128_private::Sealed for std::num::Saturating<u128> { }

impl IsntSaturatingU128Ext for std::num::Saturating<u128> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod saturating_u16_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingU16Ext: saturating_u16_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::Saturating::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl saturating_u16_private::Sealed for std::num::Saturating<u16> { }

impl IsntSaturatingU16Ext for std::num::Saturating<u16> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod saturating_u32_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingU32Ext: saturating_u32_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::Saturating::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl saturating_u32_private::Sealed for std::num::Saturating<u32> { }

impl IsntSaturatingU32Ext for std::num::Saturating<u32> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod saturating_u64_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingU64Ext: saturating_u64_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::Saturating::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl saturating_u64_private::Sealed for std::num::Saturating<u64> { }

impl IsntSaturatingU64Ext for std::num::Saturating<u64> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod saturating_u8_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingU8Ext: saturating_u8_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::Saturating::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl saturating_u8_private::Sealed for std::num::Saturating<u8> { }

impl IsntSaturatingU8Ext for std::num::Saturating<u8> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod saturating_usize_private { pub trait Sealed { } }

/// Extension for [`Saturating`](std::num::Saturating)
pub trait IsntSaturatingUsizeExt: saturating_usize_private::Sealed {
    /// The negation of [`is_power_of_two`](std::num::Saturating::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl saturating_usize_private::Sealed for std::num::Saturating<usize> { }

impl IsntSaturatingUsizeExt for std::num::Saturating<usize> {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}
