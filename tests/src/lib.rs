#[cfg(test)]
extern "C" {
    fn rust_time_init();
    fn rust_time_instant_now(instant: *mut u8);
    fn rust_time_instant_duration_since(lhs: *const u8, rhs: *const u8, duration: *mut u8);
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

    #[test]
    fn duration_since() {
        unsafe {
            rust_time_init();
        }
        let d = Duration::new(1, 250000);
        let t1 = Instant::now();
        let t2 = t1 + d;
        let diff = unsafe {
            let mut uninit = std::mem::MaybeUninit::<Duration>::uninit();
            rust_time_instant_duration_since(
                &t2 as *const _ as *const _,
                &t1 as *const _ as *const _,
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        unsafe {
            let raw_t1: [u32; 4] = std::mem::transmute_copy(&t1);
            let raw_t2: [u32; 4] = std::mem::transmute_copy(&t2);
            let raw_diff: [u32; 4] = std::mem::transmute_copy(&diff);
            eprintln!("{:?} = {:?} - {:?}", raw_diff, raw_t2, raw_t1);
        }
        assert_eq!(diff, d);
    }
}
