//! This crate provides [`CallOnce`], a simple, thread-safe type that can only be called successfully _once_:
//!
//! ```
//! use call_once::CallOnce;
//!
//! static CALL_ONCE: CallOnce = CallOnce::new();
//!
//! assert!(CALL_ONCE.call_once().is_ok());
//! assert!(CALL_ONCE.call_once().is_err());
//! ```

#![no_std]

use core::fmt;
use core::sync::atomic::{AtomicBool, Ordering};

/// A type that can only be called successfully _once_.
///
/// This is a simple wrapper around an [`AtomicBool`] with a more descriptive API.
///
/// <div class="warning">
/// While <code>CallOnce</code> is synchronized and thread-safe, it does not synchronize other memory accesses.
/// </div>
///
/// # Examples
///
/// ```
/// use call_once::CallOnce;
///
/// static CALL_ONCE: CallOnce = CallOnce::new();
///
/// assert!(CALL_ONCE.call_once().is_ok());
/// assert!(CALL_ONCE.call_once().is_err());
/// ```
#[derive(Default, Debug)]
pub struct CallOnce {
    called: AtomicBool,
}

impl CallOnce {
    /// Creates a new `CallOnce`.
    ///
    /// # Examples
    ///
    /// ```
    /// use call_once::CallOnce;
    ///
    /// let call_once = CallOnce::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            called: AtomicBool::new(false),
        }
    }

    /// Mark this `CallOnce` as called.
    ///
    /// Only the first call returns `Ok`.
    /// All subsequent calls return `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use call_once::CallOnce;
    ///
    /// let call_once = CallOnce::new();
    ///
    /// assert!(call_once.call_once().is_ok());
    /// assert!(call_once.call_once().is_err());
    /// ```
    #[inline]
    pub fn call_once(&self) -> Result<(), CallOnceError> {
        self.called
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .map(drop)
            .map_err(|_| CallOnceError)
    }

    /// Returns `true` if `call_once` has been called.
    ///
    /// # Examples
    ///
    /// ```
    /// use call_once::CallOnce;
    ///
    /// let call_once = CallOnce::new();
    /// assert!(!call_once.was_called());
    ///
    /// call_once.call_once().unwrap();
    /// assert!(call_once.was_called());
    /// ```
    #[inline]
    pub fn was_called(&self) -> bool {
        self.called.load(Ordering::Relaxed)
    }
}

/// The `CallOnceError` error indicates that [`CallOnce::call_once`] has been called more than once.
#[derive(Debug)]
pub struct CallOnceError;

impl fmt::Display for CallOnceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("call_once was executed more than once")
    }
}
