/*!
Simply increase the alignment of any statics or `include_bytes!`.
*/

#![no_std]

/// Helper which enforces that its given `T` is aligned to at least the requested alignment of `A`.
///
/// If the alignment of `A` is lower than the required alignment of `T` then its alignment is used instead.
#[repr(C)]
pub struct Aligned<A, T: ?Sized>(pub [A; 0], pub T);

/// Align to 16 bytes.
#[repr(align(16))]
pub struct Align16;

/// Align to 32 bytes.
#[repr(align(32))]
pub struct Align32;

/// Align to 64 bytes.
#[repr(align(64))]
pub struct Align64;

/// Align to 128 bytes.
#[repr(align(128))]
pub struct Align128;

/// Align to 256 bytes.
#[repr(align(256))]
pub struct Align256;

/// Align to 512 bytes.
#[repr(align(512))]
pub struct Align512;

/// Align to 4KiB.
#[repr(align(0x1000))]
pub struct Align4K;

/// Returns a static reference of the expression `$e` with requested alignment.
///
/// Unfortunately it is required to specify the type of the expression `$e` because it is stored in a `static`.
///
/// # Examples
///
/// ```
/// use align_data::{aligned, Align16};
///
/// let five = aligned!(Align16, i32, 5);
/// assert_eq!(five as *const _ as usize % 0x10, 0);
/// ```
#[macro_export]
macro_rules! aligned {
	($align:ty, $e_ty:ty, $e:expr) => {{
		static ALIGNED: &'static $crate::Aligned<$align, $e_ty> = &$crate::Aligned([], $e);
		&ALIGNED.1
	}};
}

/// Includes bytes with given alignment.
///
/// This macro ensures that the bytes included by `include_bytes!` is properly aligned.
///
/// # Examples
///
/// ```
/// use align_data::{include_aligned, Align4K};
///
/// static ALIGNED: &[u8] = include_aligned!(Align4K, "lib.rs");
/// assert_eq!(ALIGNED.as_ptr() as usize % 0x1000, 0);
/// ```
#[macro_export]
macro_rules! include_aligned {
	($align:ty, $file:expr) => {{
		static ALIGNED: &'static $crate::Aligned<$align, [u8; include_bytes!($file).len()]> = &$crate::Aligned([], *include_bytes!($file));
		&ALIGNED.1
	}};
}

/// Transmutes the included bytes.
///
/// This macro ensures that the resulting instance is aligned properly.
///
/// # Examples
///
/// ```
/// use align_data::include_transmute;
///
/// static DATA: [u32; 3] = unsafe { include_transmute!("../tests/data.txt") };
/// assert_eq!(DATA[0], u32::from_ne_bytes([b'A'; 4]));
/// ```
///
/// # Safety
///
/// This macro simply transmutes the included byte array to its destination type.
/// It is your responsibility to ensure that this transmutation is correct.
#[macro_export]
macro_rules! include_transmute {
	($file:expr) => {
		::core::mem::transmute(*include_bytes!($file))
	};
}
