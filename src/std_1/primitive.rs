// This file was generated

mod char_private { pub trait Sealed { } }

/// Extension for [`char`](char)
pub trait IsntCharExt: char_private::Sealed {
    /// The negation of [`is_digit`](char::is_digit)
    #[must_use]
    fn is_not_digit(self, radix: u32) -> bool;
    /// The negation of [`is_alphabetic`](char::is_alphabetic)
    #[must_use]
    fn is_not_alphabetic(self) -> bool;
    /// The negation of [`is_lowercase`](char::is_lowercase)
    #[must_use]
    fn is_not_lowercase(self) -> bool;
    /// The negation of [`is_uppercase`](char::is_uppercase)
    #[must_use]
    fn is_not_uppercase(self) -> bool;
    /// The negation of [`is_whitespace`](char::is_whitespace)
    #[must_use]
    fn is_not_whitespace(self) -> bool;
    /// The negation of [`is_alphanumeric`](char::is_alphanumeric)
    #[must_use]
    fn is_not_alphanumeric(self) -> bool;
    /// The negation of [`is_control`](char::is_control)
    #[must_use]
    fn is_not_control(self) -> bool;
    /// The negation of [`is_numeric`](char::is_numeric)
    #[must_use]
    fn is_not_numeric(self) -> bool;
    /// The negation of [`is_ascii`](char::is_ascii)
    #[must_use]
    fn is_not_ascii(&self) -> bool;
    /// The negation of [`is_ascii_alphabetic`](char::is_ascii_alphabetic)
    #[must_use]
    fn is_not_ascii_alphabetic(&self) -> bool;
    /// The negation of [`is_ascii_uppercase`](char::is_ascii_uppercase)
    #[must_use]
    fn is_not_ascii_uppercase(&self) -> bool;
    /// The negation of [`is_ascii_lowercase`](char::is_ascii_lowercase)
    #[must_use]
    fn is_not_ascii_lowercase(&self) -> bool;
    /// The negation of [`is_ascii_alphanumeric`](char::is_ascii_alphanumeric)
    #[must_use]
    fn is_not_ascii_alphanumeric(&self) -> bool;
    /// The negation of [`is_ascii_digit`](char::is_ascii_digit)
    #[must_use]
    fn is_not_ascii_digit(&self) -> bool;
    /// The negation of [`is_ascii_hexdigit`](char::is_ascii_hexdigit)
    #[must_use]
    fn is_not_ascii_hexdigit(&self) -> bool;
    /// The negation of [`is_ascii_punctuation`](char::is_ascii_punctuation)
    #[must_use]
    fn is_not_ascii_punctuation(&self) -> bool;
    /// The negation of [`is_ascii_graphic`](char::is_ascii_graphic)
    #[must_use]
    fn is_not_ascii_graphic(&self) -> bool;
    /// The negation of [`is_ascii_whitespace`](char::is_ascii_whitespace)
    #[must_use]
    fn is_not_ascii_whitespace(&self) -> bool;
    /// The negation of [`is_ascii_control`](char::is_ascii_control)
    #[must_use]
    fn is_not_ascii_control(&self) -> bool;
    /// The negation of [`eq_ignore_ascii_case`](char::eq_ignore_ascii_case)
    #[must_use]
    fn not_eq_ignore_ascii_case(&self, other: &char) -> bool;
}

impl char_private::Sealed for char { }

impl IsntCharExt for char {
    #[inline]
    fn is_not_digit(self, radix: u32) -> bool {
        !self.is_digit(radix)
    }

    #[inline]
    fn is_not_alphabetic(self) -> bool {
        !self.is_alphabetic()
    }

    #[inline]
    fn is_not_lowercase(self) -> bool {
        !self.is_lowercase()
    }

    #[inline]
    fn is_not_uppercase(self) -> bool {
        !self.is_uppercase()
    }

    #[inline]
    fn is_not_whitespace(self) -> bool {
        !self.is_whitespace()
    }

    #[inline]
    fn is_not_alphanumeric(self) -> bool {
        !self.is_alphanumeric()
    }

    #[inline]
    fn is_not_control(self) -> bool {
        !self.is_control()
    }

    #[inline]
    fn is_not_numeric(self) -> bool {
        !self.is_numeric()
    }

    #[inline]
    fn is_not_ascii(&self) -> bool {
        !self.is_ascii()
    }

    #[inline]
    fn is_not_ascii_alphabetic(&self) -> bool {
        !self.is_ascii_alphabetic()
    }

    #[inline]
    fn is_not_ascii_uppercase(&self) -> bool {
        !self.is_ascii_uppercase()
    }

    #[inline]
    fn is_not_ascii_lowercase(&self) -> bool {
        !self.is_ascii_lowercase()
    }

    #[inline]
    fn is_not_ascii_alphanumeric(&self) -> bool {
        !self.is_ascii_alphanumeric()
    }

    #[inline]
    fn is_not_ascii_digit(&self) -> bool {
        !self.is_ascii_digit()
    }

    #[inline]
    fn is_not_ascii_hexdigit(&self) -> bool {
        !self.is_ascii_hexdigit()
    }

    #[inline]
    fn is_not_ascii_punctuation(&self) -> bool {
        !self.is_ascii_punctuation()
    }

    #[inline]
    fn is_not_ascii_graphic(&self) -> bool {
        !self.is_ascii_graphic()
    }

    #[inline]
    fn is_not_ascii_whitespace(&self) -> bool {
        !self.is_ascii_whitespace()
    }

    #[inline]
    fn is_not_ascii_control(&self) -> bool {
        !self.is_ascii_control()
    }

    #[inline]
    fn not_eq_ignore_ascii_case(&self, other: &char) -> bool {
        !self.eq_ignore_ascii_case(other)
    }
}

mod const_ptr_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for const pointers
pub trait IsntConstPtrExt<T: ?Sized>: const_ptr_private::Sealed<T> {
    /// The negation of `is_null`
    #[must_use]
    fn is_not_null(self) -> bool;
}

impl<T: ?Sized> const_ptr_private::Sealed<T> for *const T { }

impl<T: ?Sized> IsntConstPtrExt<T> for *const T {
    #[inline]
    fn is_not_null(self) -> bool {
        !self.is_null()
    }
}

mod f32_private { pub trait Sealed { } }

/// Extension for [`f32`](f32)
pub trait IsntF32Ext: f32_private::Sealed {
    /// The negation of [`is_nan`](f32::is_nan)
    #[must_use]
    fn is_not_nan(self) -> bool;
    /// The negation of [`is_infinite`](f32::is_infinite)
    #[must_use]
    fn is_not_infinite(self) -> bool;
    /// The negation of [`is_finite`](f32::is_finite)
    #[must_use]
    fn is_not_finite(self) -> bool;
    /// The negation of [`is_normal`](f32::is_normal)
    #[must_use]
    fn is_not_normal(self) -> bool;
    /// The negation of [`is_sign_positive`](f32::is_sign_positive)
    #[must_use]
    fn is_not_sign_positive(self) -> bool;
    /// The negation of [`is_sign_negative`](f32::is_sign_negative)
    #[must_use]
    fn is_not_sign_negative(self) -> bool;
}

impl f32_private::Sealed for f32 { }

impl IsntF32Ext for f32 {
    #[inline]
    fn is_not_nan(self) -> bool {
        !self.is_nan()
    }

    #[inline]
    fn is_not_infinite(self) -> bool {
        !self.is_infinite()
    }

    #[inline]
    fn is_not_finite(self) -> bool {
        !self.is_finite()
    }

    #[inline]
    fn is_not_normal(self) -> bool {
        !self.is_normal()
    }

    #[inline]
    fn is_not_sign_positive(self) -> bool {
        !self.is_sign_positive()
    }

    #[inline]
    fn is_not_sign_negative(self) -> bool {
        !self.is_sign_negative()
    }
}

mod f64_private { pub trait Sealed { } }

/// Extension for [`f64`](f64)
pub trait IsntF64Ext: f64_private::Sealed {
    /// The negation of [`is_nan`](f64::is_nan)
    #[must_use]
    fn is_not_nan(self) -> bool;
    /// The negation of [`is_infinite`](f64::is_infinite)
    #[must_use]
    fn is_not_infinite(self) -> bool;
    /// The negation of [`is_finite`](f64::is_finite)
    #[must_use]
    fn is_not_finite(self) -> bool;
    /// The negation of [`is_normal`](f64::is_normal)
    #[must_use]
    fn is_not_normal(self) -> bool;
    /// The negation of [`is_sign_positive`](f64::is_sign_positive)
    #[must_use]
    fn is_not_sign_positive(self) -> bool;
    /// The negation of [`is_sign_negative`](f64::is_sign_negative)
    #[must_use]
    fn is_not_sign_negative(self) -> bool;
}

impl f64_private::Sealed for f64 { }

impl IsntF64Ext for f64 {
    #[inline]
    fn is_not_nan(self) -> bool {
        !self.is_nan()
    }

    #[inline]
    fn is_not_infinite(self) -> bool {
        !self.is_infinite()
    }

    #[inline]
    fn is_not_finite(self) -> bool {
        !self.is_finite()
    }

    #[inline]
    fn is_not_normal(self) -> bool {
        !self.is_normal()
    }

    #[inline]
    fn is_not_sign_positive(self) -> bool {
        !self.is_sign_positive()
    }

    #[inline]
    fn is_not_sign_negative(self) -> bool {
        !self.is_sign_negative()
    }
}

mod i128_private { pub trait Sealed { } }

/// Extension for [`i128`](i128)
pub trait IsntI128Ext: i128_private::Sealed {
    /// The negation of [`is_negative`](i128::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](i128::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl i128_private::Sealed for i128 { }

impl IsntI128Ext for i128 {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod i16_private { pub trait Sealed { } }

/// Extension for [`i16`](i16)
pub trait IsntI16Ext: i16_private::Sealed {
    /// The negation of [`is_negative`](i16::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](i16::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl i16_private::Sealed for i16 { }

impl IsntI16Ext for i16 {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod i32_private { pub trait Sealed { } }

/// Extension for [`i32`](i32)
pub trait IsntI32Ext: i32_private::Sealed {
    /// The negation of [`is_negative`](i32::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](i32::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl i32_private::Sealed for i32 { }

impl IsntI32Ext for i32 {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod i64_private { pub trait Sealed { } }

/// Extension for [`i64`](i64)
pub trait IsntI64Ext: i64_private::Sealed {
    /// The negation of [`is_negative`](i64::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](i64::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl i64_private::Sealed for i64 { }

impl IsntI64Ext for i64 {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod i8_private { pub trait Sealed { } }

/// Extension for [`i8`](i8)
pub trait IsntI8Ext: i8_private::Sealed {
    /// The negation of [`is_negative`](i8::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](i8::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl i8_private::Sealed for i8 { }

impl IsntI8Ext for i8 {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod isize_private { pub trait Sealed { } }

/// Extension for [`isize`](isize)
pub trait IsntIsizeExt: isize_private::Sealed {
    /// The negation of [`is_negative`](isize::is_negative)
    #[must_use]
    fn is_not_negative(self) -> bool;
    /// The negation of [`is_positive`](isize::is_positive)
    #[must_use]
    fn is_not_positive(self) -> bool;
}

impl isize_private::Sealed for isize { }

impl IsntIsizeExt for isize {
    #[inline]
    fn is_not_negative(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    fn is_not_positive(self) -> bool {
        !self.is_positive()
    }
}

mod mut_ptr_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for mut pointers
pub trait IsntMutPtrExt<T: ?Sized>: mut_ptr_private::Sealed<T> {
    /// The negation of `is_null`
    #[must_use]
    fn is_not_null(self) -> bool;
}

impl<T: ?Sized> mut_ptr_private::Sealed<T> for *mut T { }

impl<T: ?Sized> IsntMutPtrExt<T> for *mut T {
    #[inline]
    fn is_not_null(self) -> bool {
        !self.is_null()
    }
}

mod slice_private { pub trait Sealed<T> { } }

/// Extension for slices
pub trait IsntSliceExt<T>: slice_private::Sealed<T> {
    /// The negation of `is_empty`
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<T> slice_private::Sealed<T> for [T] { }

impl<T> IsntSliceExt<T> for [T] {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod slice2_private { pub trait Sealed<T> { } }

/// Extension for slices
pub trait IsntSlice2Ext<T>: slice2_private::Sealed<T> {
    /// The negation of `contains`
    #[must_use]
    fn not_contains(&self, x: &T) -> bool;
    /// The negation of `starts_with`
    #[must_use]
    fn not_starts_with(&self, needle: &[T]) -> bool;
    /// The negation of `ends_with`
    #[must_use]
    fn not_ends_with(&self, needle: &[T]) -> bool;
}

impl<T> slice2_private::Sealed<T> for [T] where T: std::cmp::PartialEq<T> { }

impl<T> IsntSlice2Ext<T> for [T] where T: std::cmp::PartialEq<T> {
    #[inline]
    fn not_contains(&self, x: &T) -> bool {
        !self.contains(x)
    }

    #[inline]
    fn not_starts_with(&self, needle: &[T]) -> bool {
        !self.starts_with(needle)
    }

    #[inline]
    fn not_ends_with(&self, needle: &[T]) -> bool {
        !self.ends_with(needle)
    }
}

mod str_private { pub trait Sealed { } }

/// Extension for [`str`](str)
pub trait IsntStrExt: str_private::Sealed {
    /// The negation of [`is_ascii`](str::is_ascii)
    #[must_use]
    fn is_not_ascii(&self) -> bool;
    /// The negation of [`is_char_boundary`](str::is_char_boundary)
    #[must_use]
    fn is_not_char_boundary(&self, index: usize) -> bool;
    /// The negation of [`is_empty`](str::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
    /// The negation of [`eq_ignore_ascii_case`](str::eq_ignore_ascii_case)
    #[must_use]
    fn not_eq_ignore_ascii_case(&self, other: &str) -> bool;
}

impl str_private::Sealed for str { }

impl IsntStrExt for str {
    #[inline]
    fn is_not_ascii(&self) -> bool {
        !self.is_ascii()
    }

    #[inline]
    fn is_not_char_boundary(&self, index: usize) -> bool {
        !self.is_char_boundary(index)
    }

    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_eq_ignore_ascii_case(&self, other: &str) -> bool {
        !self.eq_ignore_ascii_case(other)
    }
}

mod u128_private { pub trait Sealed { } }

/// Extension for [`u128`](u128)
pub trait IsntU128Ext: u128_private::Sealed {
    /// The negation of [`is_power_of_two`](u128::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl u128_private::Sealed for u128 { }

impl IsntU128Ext for u128 {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod u16_private { pub trait Sealed { } }

/// Extension for [`u16`](u16)
pub trait IsntU16Ext: u16_private::Sealed {
    /// The negation of [`is_power_of_two`](u16::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl u16_private::Sealed for u16 { }

impl IsntU16Ext for u16 {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod u32_private { pub trait Sealed { } }

/// Extension for [`u32`](u32)
pub trait IsntU32Ext: u32_private::Sealed {
    /// The negation of [`is_power_of_two`](u32::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl u32_private::Sealed for u32 { }

impl IsntU32Ext for u32 {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod u64_private { pub trait Sealed { } }

/// Extension for [`u64`](u64)
pub trait IsntU64Ext: u64_private::Sealed {
    /// The negation of [`is_power_of_two`](u64::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl u64_private::Sealed for u64 { }

impl IsntU64Ext for u64 {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}

mod u8_private { pub trait Sealed { } }

/// Extension for [`u8`](u8)
pub trait IsntU8Ext: u8_private::Sealed {
    /// The negation of [`is_ascii`](u8::is_ascii)
    #[must_use]
    fn is_not_ascii(&self) -> bool;
    /// The negation of [`is_ascii_alphabetic`](u8::is_ascii_alphabetic)
    #[must_use]
    fn is_not_ascii_alphabetic(&self) -> bool;
    /// The negation of [`is_ascii_uppercase`](u8::is_ascii_uppercase)
    #[must_use]
    fn is_not_ascii_uppercase(&self) -> bool;
    /// The negation of [`is_ascii_lowercase`](u8::is_ascii_lowercase)
    #[must_use]
    fn is_not_ascii_lowercase(&self) -> bool;
    /// The negation of [`is_ascii_alphanumeric`](u8::is_ascii_alphanumeric)
    #[must_use]
    fn is_not_ascii_alphanumeric(&self) -> bool;
    /// The negation of [`is_ascii_digit`](u8::is_ascii_digit)
    #[must_use]
    fn is_not_ascii_digit(&self) -> bool;
    /// The negation of [`is_ascii_hexdigit`](u8::is_ascii_hexdigit)
    #[must_use]
    fn is_not_ascii_hexdigit(&self) -> bool;
    /// The negation of [`is_ascii_punctuation`](u8::is_ascii_punctuation)
    #[must_use]
    fn is_not_ascii_punctuation(&self) -> bool;
    /// The negation of [`is_ascii_graphic`](u8::is_ascii_graphic)
    #[must_use]
    fn is_not_ascii_graphic(&self) -> bool;
    /// The negation of [`is_ascii_whitespace`](u8::is_ascii_whitespace)
    #[must_use]
    fn is_not_ascii_whitespace(&self) -> bool;
    /// The negation of [`is_ascii_control`](u8::is_ascii_control)
    #[must_use]
    fn is_not_ascii_control(&self) -> bool;
    /// The negation of [`is_power_of_two`](u8::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
    /// The negation of [`eq_ignore_ascii_case`](u8::eq_ignore_ascii_case)
    #[must_use]
    fn not_eq_ignore_ascii_case(&self, other: &u8) -> bool;
}

impl u8_private::Sealed for u8 { }

impl IsntU8Ext for u8 {
    #[inline]
    fn is_not_ascii(&self) -> bool {
        !self.is_ascii()
    }

    #[inline]
    fn is_not_ascii_alphabetic(&self) -> bool {
        !self.is_ascii_alphabetic()
    }

    #[inline]
    fn is_not_ascii_uppercase(&self) -> bool {
        !self.is_ascii_uppercase()
    }

    #[inline]
    fn is_not_ascii_lowercase(&self) -> bool {
        !self.is_ascii_lowercase()
    }

    #[inline]
    fn is_not_ascii_alphanumeric(&self) -> bool {
        !self.is_ascii_alphanumeric()
    }

    #[inline]
    fn is_not_ascii_digit(&self) -> bool {
        !self.is_ascii_digit()
    }

    #[inline]
    fn is_not_ascii_hexdigit(&self) -> bool {
        !self.is_ascii_hexdigit()
    }

    #[inline]
    fn is_not_ascii_punctuation(&self) -> bool {
        !self.is_ascii_punctuation()
    }

    #[inline]
    fn is_not_ascii_graphic(&self) -> bool {
        !self.is_ascii_graphic()
    }

    #[inline]
    fn is_not_ascii_whitespace(&self) -> bool {
        !self.is_ascii_whitespace()
    }

    #[inline]
    fn is_not_ascii_control(&self) -> bool {
        !self.is_ascii_control()
    }

    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }

    #[inline]
    fn not_eq_ignore_ascii_case(&self, other: &u8) -> bool {
        !self.eq_ignore_ascii_case(other)
    }
}

mod u8_slice_private { pub trait Sealed { } }

/// Extension for byte slices
pub trait IsntU8SliceExt: u8_slice_private::Sealed {
    /// The negation of `is_ascii`
    #[must_use]
    fn is_not_ascii(&self) -> bool;
    /// The negation of `eq_ignore_ascii_case`
    #[must_use]
    fn not_eq_ignore_ascii_case(&self, other: &[u8]) -> bool;
}

impl u8_slice_private::Sealed for [u8] { }

impl IsntU8SliceExt for [u8] {
    #[inline]
    fn is_not_ascii(&self) -> bool {
        !self.is_ascii()
    }

    #[inline]
    fn not_eq_ignore_ascii_case(&self, other: &[u8]) -> bool {
        !self.eq_ignore_ascii_case(other)
    }
}

mod usize_private { pub trait Sealed { } }

/// Extension for [`usize`](usize)
pub trait IsntUsizeExt: usize_private::Sealed {
    /// The negation of [`is_power_of_two`](usize::is_power_of_two)
    #[must_use]
    fn is_not_power_of_two(self) -> bool;
}

impl usize_private::Sealed for usize { }

impl IsntUsizeExt for usize {
    #[inline]
    fn is_not_power_of_two(self) -> bool {
        !self.is_power_of_two()
    }
}
