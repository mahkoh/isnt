// This file was generated

mod file_type_private { pub trait Sealed { } }

/// Extension for [`FileType`](std::fs::FileType)
pub trait IsntFileTypeExt: file_type_private::Sealed {
    /// The negation of [`is_dir`](std::fs::FileType::is_dir)
    #[must_use]
    fn is_not_dir(&self) -> bool;
    /// The negation of [`is_file`](std::fs::FileType::is_file)
    #[must_use]
    fn is_not_file(&self) -> bool;
    /// The negation of [`is_symlink`](std::fs::FileType::is_symlink)
    #[must_use]
    fn is_not_symlink(&self) -> bool;
}

impl file_type_private::Sealed for std::fs::FileType { }

impl IsntFileTypeExt for std::fs::FileType {
    #[inline]
    fn is_not_dir(&self) -> bool {
        !self.is_dir()
    }

    #[inline]
    fn is_not_file(&self) -> bool {
        !self.is_file()
    }

    #[inline]
    fn is_not_symlink(&self) -> bool {
        !self.is_symlink()
    }
}

mod metadata_private { pub trait Sealed { } }

/// Extension for [`Metadata`](std::fs::Metadata)
pub trait IsntMetadataExt: metadata_private::Sealed {
    /// The negation of [`is_dir`](std::fs::Metadata::is_dir)
    #[must_use]
    fn is_not_dir(&self) -> bool;
    /// The negation of [`is_file`](std::fs::Metadata::is_file)
    #[must_use]
    fn is_not_file(&self) -> bool;
    /// The negation of [`is_symlink`](std::fs::Metadata::is_symlink)
    #[must_use]
    fn is_not_symlink(&self) -> bool;
}

impl metadata_private::Sealed for std::fs::Metadata { }

impl IsntMetadataExt for std::fs::Metadata {
    #[inline]
    fn is_not_dir(&self) -> bool {
        !self.is_dir()
    }

    #[inline]
    fn is_not_file(&self) -> bool {
        !self.is_file()
    }

    #[inline]
    fn is_not_symlink(&self) -> bool {
        !self.is_symlink()
    }
}

mod permissions_private { pub trait Sealed { } }

/// Extension for [`Permissions`](std::fs::Permissions)
pub trait IsntPermissionsExt: permissions_private::Sealed {
    /// The negation of [`readonly`](std::fs::Permissions::readonly)
    #[must_use]
    fn not_readonly(&self) -> bool;
}

impl permissions_private::Sealed for std::fs::Permissions { }

impl IsntPermissionsExt for std::fs::Permissions {
    #[inline]
    fn not_readonly(&self) -> bool {
        !self.readonly()
    }
}
