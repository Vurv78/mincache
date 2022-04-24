use core::cell::UnsafeCell;
pub use mincache_impl::*;

/// UnsafeCell + Sync implemented
/// Don't use this!
#[repr(transparent)]
pub struct SyncUnsafeCell<T>(UnsafeCell<T>);
impl<T> SyncUnsafeCell<T> {
	#[inline(always)]
	pub const fn new(val: T) -> Self {
		Self(UnsafeCell::new(val))
	}

	#[inline(always)]
	pub fn get_mut(&self) -> &mut T {
		unsafe { &mut *self.0.get() }
	}
}
unsafe impl<T> Sync for SyncUnsafeCell<T> {}
impl<T> core::ops::Deref for SyncUnsafeCell<T> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0.get() }
	}
}