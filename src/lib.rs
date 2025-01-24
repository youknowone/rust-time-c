//! std::time::{Duration, Instant} bindings assistant.
//!
//! # Warning: No guarantee for safe compatibility. Use it at your own risk. Data without `#[repr(C)]` is very fragile on the FFI boundary.
//!
//! Using these union types provides a few benefits.
//!
//! First, memory layout optimization can be prevented. Since these types are not `#[repr(C)]`, the layout can be freely optimized on the Rust side. Using a union helps the internal `CDuration` reserve the full size of these types.
//! Second, it provides an error check mechanism for operations outside of Rust.
//!
//! Still, be aware of the risks. This does not mean it is safe, nor does it mean it cannot be broken by future Rust changes.

/// FFI-available `Duration` corresponding to `std::time::Duration`.
///
/// In `rust_time.h`, `rust::time::Duration` matches this type.
///
/// Note: This is a platform-dependent implementation. Major platforms are compatible.
#[repr(C)]
#[derive(Clone, Copy)]
pub union Duration {
    duration: std::time::Duration,
    payload: CDuration,
}
static_assertions::assert_eq_size!(Duration, std::time::Duration);
static_assertions::assert_eq_size!(Duration, CDuration);

impl Duration {
    // Create from a Rust `std::time::Duration` object.
    pub fn from_duration(duration: std::time::Duration) -> Self {
        let mut uninit = std::mem::MaybeUninit::<Duration>::uninit();
        unsafe {
            uninit.as_mut_ptr().write(Self { duration });
            uninit.assume_init()
        }
    }
    pub fn into_duration(self) -> std::option::Option<std::time::Duration> {
        if unsafe { self.payload }.is_valid() {
            Some(unsafe { self.duration })
        } else {
            None
        }
    }
    pub fn as_duration(&self) -> std::option::Option<&std::time::Duration> {
        if unsafe { self.payload }.is_valid() {
            Some(unsafe { &self.duration })
        } else {
            None
        }
    }
    /// # Safety: The payload representation must match a valid `std::time::Duration`.
    pub unsafe fn into_duration_unchecked(self) -> std::time::Duration {
        self.duration
    }
    /// # Safety: The payload representation must match a valid `std::time::Duration`.
    pub unsafe fn as_duration_unchecked(&self) -> &std::time::Duration {
        &self.duration
    }

    #[cfg(test)]
    pub fn as_c_ptr(&self) -> *const libc::c_void {
        unsafe { &raw const self.payload as *const _ }
    }
    #[cfg(test)]
    pub fn as_c_mut_ptr(&mut self) -> *mut libc::c_void {
        unsafe { &raw mut self.payload as *mut _ }
    }
}

impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        Self::from_duration(duration)
    }
}

impl From<CDuration> for Duration {
    fn from(payload: CDuration) -> Self {
        Self { payload }
    }
}

/// FFI-available `Instant` corresponding to `std::time::Instant`.
///
/// In `rust_time.h`, `rust::time::Instant` matches this type.
///
/// Note: This is a platform-dependent implementation. Major platforms are compatible.
#[repr(C)]
#[derive(Clone, Copy)]
pub union Instant {
    instant: std::time::Instant,
    payload: CDuration,
}
static_assertions::assert_eq_size!(Instant, std::time::Instant);
static_assertions::assert_eq_size!(Instant, CDuration);

impl Instant {
    // Create from a Rust `std::time::Instant` object.
    pub fn from_instant(instant: std::time::Instant) -> Self {
        let mut uninit = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            uninit.as_mut_ptr().write(Self { instant });
            uninit.assume_init()
        }
    }
    pub fn into_instant(self) -> std::option::Option<std::time::Instant> {
        if unsafe { self.payload }.is_valid() {
            Some(unsafe { self.instant })
        } else {
            None
        }
    }
    pub fn as_instant(&self) -> std::option::Option<&std::time::Instant> {
        if unsafe { self.payload }.is_valid() {
            Some(unsafe { &self.instant })
        } else {
            None
        }
    }
    /// # Safety: The payload representation must match a valid `std::time::Instant`.
    pub unsafe fn into_instant_unchecked(self) -> std::time::Instant {
        self.instant
    }
    /// # Safety: The payload representation must match a valid `std::time::Instant`.
    pub unsafe fn as_instant_unchecked(&self) -> &std::time::Instant {
        &self.instant
    }

    #[cfg(test)]
    pub fn as_c_ptr(&self) -> *const libc::c_void {
        unsafe { &raw const self.payload as *const _ }
    }
    #[cfg(test)]
    pub fn as_c_mut_ptr(&mut self) -> *mut libc::c_void {
        unsafe { &raw mut self.payload as *mut _ }
    }
}

impl From<std::time::Instant> for Instant {
    fn from(instant: std::time::Instant) -> Self {
        Self::from_instant(instant)
    }
}

impl From<CDuration> for Instant {
    fn from(payload: CDuration) -> Self {
        Self { payload }
    }
}

/// The internal representation of `std::time::Duration` and `std::time::Instant`, but C compatible.
///
/// # Warning: No guarantee for safe compatibility.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CDuration {
    pub secs: u64,
    pub nanos: u32,
    /// This field must never be accessed. Accessing this field might be UB by creation path.
    _padding: u32,
}

impl CDuration {
    const NONE_NANOS: u32 = (-1i32) as u32;
    fn is_valid(&self) -> bool {
        self.nanos < 1_000_000_000
    }
    fn is_none(&self) -> bool {
        self.nanos == Self::NONE_NANOS
    }
}

/// `std::option::Option`-like wrapper of Duration and Instant.
///
/// This is only created from FFI functions.
/// When checked operation is called in C++ side and directly passed to Rust,
/// it must be typed as `Option<Duration>` or `Option<Instant>` to check none value.
#[repr(C)]
pub struct Option<T: From<CDuration>>(CDuration, std::marker::PhantomData<T>);

impl<T: From<CDuration>> Option<T> {
    pub fn unwrap(self) -> T {
        T::from(self.0)
    }
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        if self.0.is_none() {
            panic!("{}", msg);
        }
        T::from(self.0)
    }
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

impl Option<Duration> {
    pub fn into_duration(self) -> std::option::Option<std::time::Duration> {
        if self.0.is_valid() {
            Some(Duration { payload: self.0 }.into_duration().unwrap())
        } else {
            None
        }
    }
}

impl Option<Instant> {
    pub fn into_instant(self) -> std::option::Option<std::time::Instant> {
        if self.0.is_valid() {
            Some(Instant { payload: self.0 }.into_instant().unwrap())
        } else {
            None
        }
    }
}
