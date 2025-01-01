//! Resource Reference Counter (RRC)
//!
//! Resources are typically FFI functionalties or libraries which require manual (de)initialization.
//!
//! # Examples
//!
//! ```
//! static LIBRARY: Rrc<fn(), fn()> = Rrc::new(
//!     || { LibraryInit(); },
//!     || { LibraryDeinit(); }
//! )
//!
//! struct LibStruct {
//!     _resource: ResourceGuard<'a>
//! };
//!
//! impl LibStruct {
//!     fn new() -> Self {
//!         Self { _resource: LIBRARY.acquire() }
//!     }
//! }
//! ```

use core::sync::atomic::{AtomicI32, Ordering};

pub struct Rrc<F: Fn() + Sync, G: Fn() + Sync> {
    ref_count: AtomicI32,
    init_fn: F,
    deinit_fn: G,
}

pub trait RrcGuarded: Sync {
    fn release(&self);
}

impl<F: Fn() + Sync, G: Fn() + Sync> Rrc<F, G> {
    pub const fn new(init_fn: F, deinit_fn: G) -> Self {
        Self {
            ref_count: AtomicI32::new(0),
            init_fn,
            deinit_fn,
        }
    }

    /// Call `init_fn` if first time acquire
    pub fn acquire(&self) -> ResourceGuard {
        unsafe {
            self.increase();
        }
        ResourceGuard { rrc: self }
    }

    /// Initialize the resource if reference count increased to 1.
    pub unsafe fn increase(&self) {
        // if core::cfg!(debug_assertions) {
        if self.ref_count.load(Ordering::SeqCst) < 0 {
            panic!("Cannot release a non-existing resource!");
        }
        // }

        if self.ref_count.fetch_add(1, Ordering::SeqCst) == 0 {
            (self.init_fn)()
        }
    }

    /// Deinitialize the resource if reference count decreased to 0.
    pub unsafe fn decrease(&self) {
        // if core::cfg!(debug_assertions) {
        if self.ref_count.load(Ordering::SeqCst) <= 0 {
            panic!("Cannot release a non-existing resource!");
        }
        // }

        if self.ref_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            (self.deinit_fn)()
        }
    }

    /// Deinitialize the resource and reset the reference count to 0.
    ///
    /// Only use this function if you are sure that no one will acquire the resource anymore    
    pub unsafe fn clear(&self) {
        self.ref_count.store(0, Ordering::SeqCst);
        (self.deinit_fn)()
    }
}

impl<F: Fn() + Sync, G: Fn() + Sync> RrcGuarded for Rrc<F, G> {
    fn release(&self) {
        unsafe {
            self.decrease();
        }
    }
}

unsafe impl<F: Fn() + Sync, G: Fn() + Sync> Sync for Rrc<F, G> {}

pub struct ResourceGuard<'a> {
    rrc: &'a dyn RrcGuarded,
}

impl Drop for ResourceGuard<'_> {
    fn drop(&mut self) {
        self.rrc.release();
    }
}
