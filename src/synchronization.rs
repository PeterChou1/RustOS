use core::cell::UnsafeCell;
use core::sync::atomic::{compiler_fence, AtomicBool, Ordering};


pub mod interface {

    /// Any object implementing this trait guarantees exclusive access to the data wrapped within
    /// the Mutex for the duration of the provided closure.
    pub trait Mutex {
        /// The type of the data that is wrapped by this mutex.
        type Data;

        /// Locks the mutex and grants the closure temporary mutable access to the wrapped data.
        fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;
    }
}

/// A pseudo-lock for teaching purposes.
///
/// In contrast to a real Mutex implementation, does not protect against concurrent access from
/// other cores to the contained data. This part is preserved for later lessons.
///
/// The lock will only be used as long as it is safe to do so, i.e. as long as the kernel is
/// executing single-threaded, aka only running on a single core with interrupts disabled.
pub struct NullLock<T>
    where
        T: ?Sized,
{
    data: UnsafeCell<T>,
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Send {}

impl<T> NullLock<T> {
    /// Create an instance.
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

//------------------------------------------------------------------------------
// OS Interface Code
//------------------------------------------------------------------------------

impl<T> interface::Mutex for NullLock<T> {
    type Data = T;

     fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
        // In a real lock, there would be code encapsulating this line that ensures that this
        // mutable reference will ever only be given out once at a time.
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}


pub struct SpinLock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    const fn new(data: T) -> SpinLock<T> {
        SpinLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        while self.lock.compare_and_swap(false, true, Ordering::Acquire) {
            while self.lock.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
        compiler_fence(Ordering::Acquire);
        SpinLockGuard {
            lock: &self.lock,
            data: self.data.get(),
        }
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

struct SpinLockGuard<'a, T> {
    lock: &'a AtomicBool,
    data: *mut T,
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        compiler_fence(Ordering::Release);
        self.lock.store(false, Ordering::Release);
    }
}

impl<T> core::ops::Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T> core::ops::DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}