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

#[test]
fn now() {
    unsafe {
        rust_time_init();
    }
    let t1 = std::time::Instant::now();
    let t2 = unsafe {
        let mut uninit = std::mem::MaybeUninit::<ffi_time::Instant>::uninit();
        rust_time_instant_now(uninit.as_mut_ptr() as *mut _);
        uninit.assume_init().into_instant()
    };
    let t3 = unsafe {
        let mut uninit = std::mem::MaybeUninit::<std::time::Instant>::uninit();
        rust_time_instant_now(uninit.as_mut_ptr() as *mut _);
        uninit.assume_init()
    };
    let t4 = std::time::Instant::now();

    assert!(t1 <= t2);
    assert!(t2 <= t3);
    assert!(t3 <= t4);
}

#[test]
fn duration_since() {
    unsafe {
        rust_time_init();
    }
    let d = std::time::Duration::new(1, 250000);
    let t1 = std::time::Instant::now();
    let t2 = t1 + d;
    let u1 = ffi_time::Instant::from_instant(t1);
    // u1.validate();

    let u2 = ffi_time::Instant::from_instant(t2);
    // u2.validate();

    eprint!("{t2:?} - {t1:?}");
    // eprint!(
    //     "{:?}({:?}) - {:?}({:?})",
    //     u2.as_payload(),
    //     t2,
    //     u1.as_payload(),
    //     t1
    // );

    let diff = unsafe {
        let mut diff = std::mem::MaybeUninit::<ffi_time::Option<ffi_time::Duration>>::uninit();
        // u2.validate();
        // u1.validate();

        rust_time_instant_duration_since(
            &raw const u2 as *const _,
            &raw const u1 as *const _,
            diff.as_mut_ptr() as *mut _,
        );
        diff.assume_init()
    };
    let diff = unsafe { diff.unwrap().into_duration() };
    eprintln!(" = {:?}", diff);
    assert_eq!(t2 - t1, d);
    assert_eq!(diff, d);
}

#[cfg(test)]
proptest::proptest! {
    #[test]
    fn prop_duration_since(mut t1: std::time::Instant, mut t2: std::time::Instant) {
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
            ffi_time::Duration::from_duration(payload)
        };
        let rust_diff = t2.duration_since(t1);
        assert_eq!(&rust_diff, unsafe { c_diff.as_duration() });

        let u1 = ffi_time::Instant::from_instant(t1);
        // u1.as_payload().validate();
        let u2 = ffi_time::Instant::from_instant(t2);
        // u2.as_payload().validate();
        // eprint!("{:?} - {:?}", u2.as_payload(), u1.as_payload());

        let c_diff = unsafe {
            let mut uninit = std::mem::MaybeUninit::<ffi_time::Duration>::uninit();
            rust_time_instant_duration_since(
                &raw const u2 as *const _,
                &raw const u1 as *const _,
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        let rust_diff = t2.duration_since(t1);
        assert_eq!(&rust_diff, unsafe { c_diff.as_duration()});
    }
}
