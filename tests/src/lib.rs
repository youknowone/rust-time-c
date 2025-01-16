#[cfg(test)]
unsafe extern "C" {
    fn rust_time_init();
    fn rust_time_instant_now(instant: *mut libc::c_void);
    fn rust_time_instant_duration_since(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        duration: *mut libc::c_void,
    );
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CDuration {
    secs: u64,
    nanos: u32,
    _padding: u32,
}

impl CDuration {
    pub fn as_c_ptr(&self) -> *const libc::c_void {
        &raw const *self as *const _
    }
    pub fn as_c_mut_ptr(&mut self) -> *mut libc::c_void {
        &raw mut *self as *mut _
    }
}

#[repr(C)]
pub union DurationUnion {
    duration: std::time::Duration,
    payload: CDuration,
}
static_assertions::assert_eq_size!(DurationUnion, std::time::Duration);
static_assertions::assert_eq_size!(DurationUnion, [u32; 4]);

impl DurationUnion {
    pub fn from_duration(duration: std::time::Duration) -> Self {
        let mut uninit = std::mem::MaybeUninit::<DurationUnion>::uninit();
        unsafe {
            uninit.as_mut_ptr().write(Self { duration });
            uninit.assume_init()
        }
    }
    pub fn from_payload(payload: CDuration) -> Self {
        Self { payload }
    }
    pub fn as_payload(&self) -> &CDuration {
        unsafe { &self.payload }
    }
    pub fn as_c_ptr(&self) -> *const libc::c_void {
        unsafe { &raw const self.payload as *const _ }
    }
    pub fn as_c_mut_ptr(&mut self) -> *mut libc::c_void {
        unsafe { &raw mut self.payload as *mut _ }
    }
}

#[repr(C)]
pub union InstantUnion {
    instant: std::time::Instant,
    payload: CDuration,
}
static_assertions::assert_eq_size!(InstantUnion, [u32; 4]);

impl InstantUnion {
    pub fn from_instant(instant: std::time::Instant) -> Self {
        let mut uninit = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            uninit.as_mut_ptr().write(Self { instant });
            uninit.assume_init()
        }
    }
    pub fn from_payload(payload: CDuration) -> Self {
        Self { payload }
    }
    pub fn as_payload(&self) -> &CDuration {
        unsafe { &self.payload }
    }
    pub fn as_c_ptr(&self) -> *const libc::c_void {
        unsafe { &raw const self.payload as *const _ }
    }
    pub fn as_c_mut_ptr(&mut self) -> *mut libc::c_void {
        unsafe { &raw mut self.payload as *mut _ }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn now() {
        unsafe {
            rust_time_init();
        }
        let t1 = Instant::now();
        let t2 = unsafe {
            let mut uninit = std::mem::MaybeUninit::<Instant>::uninit();
            rust_time_instant_now(uninit.as_mut_ptr() as *mut _);
            uninit.assume_init()
        };
        let t3 = unsafe {
            let mut uninit = std::mem::MaybeUninit::<Instant>::uninit();
            rust_time_instant_now(uninit.as_mut_ptr() as *mut _);
            uninit.assume_init()
        };
        let t4 = Instant::now();

        assert!(t1 <= t2);
        assert!(t2 <= t3);
        assert!(t3 <= t4);
    }

    impl CDuration {
        #[track_caller]
        fn validate(&self) {
            assert!(self.nanos < 1_000_000_000);
        }
    }
    static_assertions::assert_eq_size!(CDuration, [u32; 4]);

    #[test]
    fn duration_since() {
        unsafe {
            rust_time_init();
        }
        let d = Duration::new(1, 250000);
        let t1 = Instant::now();
        let t2 = t1 + d;
        let u1 = InstantUnion::from_instant(t1);
        u1.as_payload().validate();

        let u2 = InstantUnion { instant: t2 };
        u2.as_payload().validate();

        eprint!(
            "{:?}({:?}) - {:?}({:?})",
            u2.as_payload(),
            t2,
            u1.as_payload(),
            t1
        );

        let diff = unsafe {
            let mut uninit = std::mem::MaybeUninit::<DurationUnion>::uninit();
            u2.payload.validate();
            u1.payload.validate();

            rust_time_instant_duration_since(
                u2.as_c_ptr(),
                u1.as_c_ptr(),
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        eprintln!(" = {:?}", diff.as_payload());
        diff.as_payload().validate();
        assert_eq!(t2 - t1, d);
        assert_eq!(unsafe { diff.duration }, d);
    }

    proptest::proptest! {
        #[test]
        fn prop_duration_since(mut t1: Instant, mut t2: Instant) {
            unsafe {
                rust_time_init();
            }
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            let c_diff = unsafe {
                let mut uninit = std::mem::MaybeUninit::<std::time::Duration>::uninit();
                rust_time_instant_duration_since(
                    &raw const t2 as *const _,
                    &raw const t1 as *const _,
                    uninit.as_mut_ptr() as *mut _,
                );
                let payload = uninit.assume_init();
                DurationUnion::from_duration(payload)
            };
            let rust_diff = t2.duration_since(t1);
            assert_eq!(rust_diff, unsafe { c_diff.duration });

            let u1 = InstantUnion { instant: t1 };
            u1.as_payload().validate();
            let u2 = InstantUnion { instant: t2 };
            u2.as_payload().validate();
            eprint!("{:?} - {:?}", u2.as_payload(), u1.as_payload());

            let c_diff = unsafe {
                let mut uninit = std::mem::MaybeUninit::<CDuration>::uninit();
                rust_time_instant_duration_since(
                    u2.as_c_ptr(),
                    u1.as_c_ptr(),
                    uninit.as_mut_ptr() as *mut _,
                );
                let payload = uninit.assume_init();
                DurationUnion::from_payload(payload)
            };
            let rust_diff = t2.duration_since(t1);
            assert_eq!(rust_diff, unsafe { c_diff.duration });
        }
    }
}
