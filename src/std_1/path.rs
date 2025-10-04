// This file was generated

mod path_private { pub trait Sealed { } }

/// Extension for [`Path`](std::path::Path)
pub trait IsntPathExt: path_private::Sealed {
    /// The negation of [`is_absolute`](std::path::Path::is_absolute)
    #[must_use]
    fn is_not_absolute(&self) -> bool;
    /// The negation of [`is_relative`](std::path::Path::is_relative)
    #[must_use]
    fn is_not_relative(&self) -> bool;
    /// The negation of [`has_root`](std::path::Path::has_root)
    #[must_use]
    fn not_has_root(&self) -> bool;
    /// The negation of [`starts_with`](std::path::Path::starts_with)
    #[must_use]
    fn not_starts_with<P: std::convert::AsRef<std::path::Path>>(&self, base: P) -> bool;
    /// The negation of [`ends_with`](std::path::Path::ends_with)
    #[must_use]
    fn not_ends_with<P: std::convert::AsRef<std::path::Path>>(&self, child: P) -> bool;
    /// The negation of [`exists`](std::path::Path::exists)
    #[must_use]
    fn not_exists(&self) -> bool;
    /// The negation of [`is_file`](std::path::Path::is_file)
    #[must_use]
    fn is_not_file(&self) -> bool;
    /// The negation of [`is_dir`](std::path::Path::is_dir)
    #[must_use]
    fn is_not_dir(&self) -> bool;
    /// The negation of [`is_symlink`](std::path::Path::is_symlink)
    #[must_use]
    fn is_not_symlink(&self) -> bool;
}

impl path_private::Sealed for std::path::Path { }

impl IsntPathExt for std::path::Path {
    #[inline]
    fn is_not_absolute(&self) -> bool {
        !self.is_absolute()
    }

    #[inline]
    fn is_not_relative(&self) -> bool {
        !self.is_relative()
    }

    #[inline]
    fn not_has_root(&self) -> bool {
        !self.has_root()
    }

    #[inline]
    fn not_starts_with<P: std::convert::AsRef<std::path::Path>>(&self, base: P) -> bool {
        !self.starts_with::<P>(base)
    }

    #[inline]
    fn not_ends_with<P: std::convert::AsRef<std::path::Path>>(&self, child: P) -> bool {
        !self.ends_with::<P>(child)
    }

    #[inline]
    fn not_exists(&self) -> bool {
        !self.exists()
    }

    #[inline]
    fn is_not_file(&self) -> bool {
        !self.is_file()
    }

    #[inline]
    fn is_not_dir(&self) -> bool {
        !self.is_dir()
    }

    #[inline]
    fn is_not_symlink(&self) -> bool {
        !self.is_symlink()
    }
}

mod path_buf_private { pub trait Sealed { } }

/// Extension for [`PathBuf`](std::path::PathBuf)
pub trait IsntPathBufExt: path_buf_private::Sealed {
    /// The negation of [`pop`](std::path::PathBuf::pop)
    #[must_use]
    fn not_pop(&mut self) -> bool;
    /// The negation of [`set_extension`](std::path::PathBuf::set_extension)
    #[must_use]
    fn not_set_extension<S: std::convert::AsRef<std::ffi::OsStr>>(&mut self, extension: S) -> bool;
}

impl path_buf_private::Sealed for std::path::PathBuf { }

impl IsntPathBufExt for std::path::PathBuf {
    #[inline]
    fn not_pop(&mut self) -> bool {
        !self.pop()
    }

    #[inline]
    fn not_set_extension<S: std::convert::AsRef<std::ffi::OsStr>>(&mut self, extension: S) -> bool {
        !self.set_extension::<S>(extension)
    }
}

mod prefix_private { pub trait Sealed<'a> { } }

/// Extension for [`Prefix`](std::path::Prefix)
pub trait IsntPrefixExt<'a>: prefix_private::Sealed<'a> {
    /// The negation of [`is_verbatim`](std::path::Prefix::is_verbatim)
    #[must_use]
    fn is_not_verbatim(&self) -> bool;
}

impl<'a> prefix_private::Sealed<'a> for std::path::Prefix<'a> { }

impl<'a> IsntPrefixExt<'a> for std::path::Prefix<'a> {
    #[inline]
    fn is_not_verbatim(&self) -> bool {
        !self.is_verbatim()
    }
}
