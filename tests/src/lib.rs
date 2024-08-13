#[cfg(test)]
extern "C" {
    fn rust_time_init();
    fn rust_time_instant_now(instant: *mut u8);
    fn rust_time_instant_duration_since(
        lhs: *const [u32; 3],
        rhs: *const [u32; 3],
        duration: *mut [u32; 3],
    );
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
            rust_time_instant_now(uninit.as_mut_ptr() as *mut u8);
            uninit.assume_init()
        };
        let t3 = Instant::now();

        assert!(t1 <= t2);
        assert!(t2 <= t3);
    }

    #[derive(Debug)]
    #[repr(C)]
    struct RawDuration {
        secs: u64,
        nanos: u32,
    }

    impl RawDuration {
        fn validate(&self) {
            assert!(self.nanos < 1_000_000_000);
        }
    }

    fn inspect_instant(t: &Instant) -> RawDuration {
        unsafe { std::mem::transmute_copy(t) }
    }
    fn inspect_duration(t: &Duration) -> RawDuration {
        unsafe { std::mem::transmute_copy(t) }
    }

    #[test]
    fn duration_since() {
        unsafe {
            rust_time_init();
        }
        let d = Duration::new(1, 250000);
        let t1 = Instant::now();
        let t2 = t1 + d;
        let raw_t1 = inspect_instant(&t1);
        raw_t1.validate();
        let raw_t2 = inspect_instant(&t2);
        raw_t2.validate();
        eprint!("{:?} - {:?}", raw_t2, raw_t1);

        let diff = unsafe {
            let mut uninit = std::mem::MaybeUninit::<Duration>::uninit();
            rust_time_instant_duration_since(
                &raw_t2 as *const _ as *const _,
                &raw_t1 as *const _ as *const _,
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        let raw_diff = inspect_duration(&diff);
        eprintln!(" = {:?}", raw_diff);
        assert_eq!(diff, d);
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
            let raw_t1 = inspect_instant(&t1);
            raw_t1.validate();
            let raw_t2 = inspect_instant(&t2);
            raw_t2.validate();
            let raw_t1 = std::cell::UnsafeCell::new(raw_t1);
            let raw_t2 = std::cell::UnsafeCell::new(raw_t2);
            let c_diff = unsafe {
                let mut uninit = std::mem::MaybeUninit::<Duration>::uninit();
                rust_time_instant_duration_since(
                    raw_t2.get() as *const _,
                    raw_t1.get() as *const _,
                    uninit.as_mut_ptr() as *mut _,
                );
                uninit.assume_init()
            };
            let rust_diff = t2.duration_since(t1);
            assert_eq!(rust_diff, c_diff);
        }
    }
}
