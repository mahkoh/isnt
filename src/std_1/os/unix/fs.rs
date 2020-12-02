// This file was generated

#[cfg(unix)]
mod file_type_ext_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`FileTypeExt`](std::os::unix::fs::FileTypeExt)
#[cfg(unix)]
pub trait IsntFileTypeExtExt<T: ?Sized>: file_type_ext_private::Sealed<T>+std::os::unix::fs::FileTypeExt {
    /// The negation of [`is_block_device`](std::os::unix::fs::FileTypeExt::is_block_device)
    #[must_use]
    fn is_not_block_device(&self) -> bool;
    /// The negation of [`is_char_device`](std::os::unix::fs::FileTypeExt::is_char_device)
    #[must_use]
    fn is_not_char_device(&self) -> bool;
    /// The negation of [`is_fifo`](std::os::unix::fs::FileTypeExt::is_fifo)
    #[must_use]
    fn is_not_fifo(&self) -> bool;
    /// The negation of [`is_socket`](std::os::unix::fs::FileTypeExt::is_socket)
    #[must_use]
    fn is_not_socket(&self) -> bool;
}

#[cfg(unix)]
impl<T: ?Sized> file_type_ext_private::Sealed<T> for T where T: std::os::unix::fs::FileTypeExt { }

#[cfg(unix)]
impl<T: ?Sized> IsntFileTypeExtExt<T> for T where T: std::os::unix::fs::FileTypeExt {
    #[inline]
    fn is_not_block_device(&self) -> bool {
        !self.is_block_device()
    }

    #[inline]
    fn is_not_char_device(&self) -> bool {
        !self.is_char_device()
    }

    #[inline]
    fn is_not_fifo(&self) -> bool {
        !self.is_fifo()
    }

    #[inline]
    fn is_not_socket(&self) -> bool {
        !self.is_socket()
    }
}
