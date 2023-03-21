//! Parse an ext2 filesystem.

use crate::schema::{Superblock, EXT2_MAGIC};

impl Superblock {
    /// Load the superblock from an address.
    ///
    /// # Safety
    /// The pointer must be to a valid superblock. Additionally, it must maintain Rust guarantees,
    /// i.e.
    /// 1. is not mutably borrowed for `'a`
    /// 2. lives for `'a`
    /// 3. is non-null, non-dangled, properly aligned
    #[must_use]
    pub unsafe fn from_addr<'a>(addr: *const usize) -> &'a Self {
        let ptr = addr.cast::<Self>();
        let ret = unsafe { &*ptr };
        debug_assert_eq!(ret.magic, EXT2_MAGIC);
        ret
    }

    /// Load the superblock mutably from an address.
    ///
    /// # Safety
    /// The pointer must be to a valid superblock. Additionally, it must maintain Rust guarantees,
    /// i.e.
    /// 1. is not otherwise borrowed for `'a`
    /// 2. lives for `'a`
    /// 3. is non-null, non-dangled, properly aligned
    #[must_use]
    pub unsafe fn from_addr_mut<'a>(addr: *mut usize) -> &'a mut Self {
        let ptr = addr.cast::<Self>();
        let ret = unsafe { &mut *ptr };
        debug_assert_eq!(ret.magic, EXT2_MAGIC);
        ret
    }

    /// Load the superblock from a slice of bytes.
    ///
    /// # Safety
    /// The bytes must represent a valid superblock. In particular, they must be non-null,
    /// non-dangled, and properly aligned.
    #[must_use]
    pub unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        let addr = bytes.as_ptr().cast();
        // Safety: lifetimes and aliasing are enforced by the borrow checker. Other guarantees are
        // mainted by the caller.
        unsafe { Self::from_addr(addr) }
    }

    /// Load the superblock from a mutable slice of bytes.
    ///
    /// # Safety
    /// The bytes must represent a valid superblock. In particular, they must be non-null,
    /// non-dangled, and properly aligned.
    #[must_use]
    pub unsafe fn from_bytes_mut(bytes: &mut [u8]) -> &mut Self {
        let addr = bytes.as_mut_ptr().cast();
        // Safety: lifetimes and aliasing are enforced by the borrow checker. Other guarantees are
        // mainted by the caller.
        unsafe { Self::from_addr_mut(addr) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn superblock_from_bytes_works() {
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/test-superblock"
        ));
        let sb = unsafe { Superblock::from_bytes(bytes) };

        assert_eq!(sb.magic, EXT2_MAGIC);
    }
}
