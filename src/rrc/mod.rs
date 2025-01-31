//! Resource Reference Counter (RRC)
//!
//! Resources are typically FFI functionalities, symbols or libraries which require manual (de)initialization.
//!
//! # Examples
//!
//! ```
//! static LIBRARY: Rrc = Rrc::new(
//!     || { LibraryInit(); },
//!     || { LibraryDeinit(); }
//! )
//!
//! struct LibStruct {
//!     _resource: RrcGuard
//! };
//!
//! impl LibStruct {
//!     fn new() -> Self {
//!         Self { _resource: LIBRARY.acquire() }
//!     }
//! }
//! ```

use core::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicU32, Ordering},
};

pub type Rrc = ResourceRefCounter<fn(), fn()>;
pub type RrcGuard = ResourceGuard<fn(), fn()>;

pub struct ResourceRefCounter<F: 'static + Fn() + Sync, G: 'static + Fn() + Sync> {
    ref_count: AtomicU32,
    init_fn: F,
    deinit_fn: G,
}

impl<F: 'static + Fn() + Sync, G: 'static + Fn() + Sync> ResourceRefCounter<F, G> {
    #[inline]
    pub const fn new(init_fn: F, deinit_fn: G) -> Self {
        Self {
            ref_count: AtomicU32::new(0),
            init_fn,
            deinit_fn,
        }
    }

    /// Call `init_fn` if first time acquire
    #[inline]
    #[must_use]
    pub fn acquire(&'static self) -> ResourceGuard<F, G> {
        if self.ref_count.fetch_add(1, Ordering::SeqCst) == 0 {
            (self.init_fn)()
        }
        ResourceGuard { rrc: self }
    }

    /// Deinitialize the resource if reference count decreased to 0.
    #[inline]
    fn release(&self) {
        if self.ref_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            (self.deinit_fn)()
        }
    }

    /// Deinitialize the resource and reset the reference count to 0.
    ///
    /// Only use this function if you are sure that no one will acquire the resource anymore   
    #[inline]
    pub fn clear(&self) {
        self.ref_count.store(0, Ordering::SeqCst);
        (self.deinit_fn)()
    }
}

unsafe impl<F: Fn() + Sync, G: Fn() + Sync> Sync for ResourceRefCounter<F, G> {}
impl<F: Fn() + Sync + UnwindSafe, G: Fn() + Sync + UnwindSafe> RefUnwindSafe
    for ResourceRefCounter<F, G>
{
}
impl<F: Fn() + Sync + UnwindSafe, G: Fn() + Sync + UnwindSafe> UnwindSafe
    for ResourceRefCounter<F, G>
{
}

pub struct ResourceGuard<F: 'static + Fn() + Sync, G: 'static + Fn() + Sync> {
    rrc: &'static ResourceRefCounter<F, G>,
}

impl<F: 'static + Fn() + Sync, G: 'static + Fn() + Sync> Drop for ResourceGuard<F, G> {
    fn drop(&mut self) {
        self.rrc.release();
    }
}

unsafe impl<F: Fn() + Sync, G: Fn() + Sync> Sync for ResourceGuard<F, G> {}
impl<F: Fn() + Sync + UnwindSafe, G: Fn() + Sync + UnwindSafe> RefUnwindSafe
    for ResourceGuard<F, G>
{
}
impl<F: Fn() + Sync + UnwindSafe, G: Fn() + Sync + UnwindSafe> UnwindSafe for ResourceGuard<F, G> {}
