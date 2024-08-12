#[cfg(test)]
extern "C" {
    fn rust_time_init();
    fn rust_time_instant_now(instant: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

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
}
