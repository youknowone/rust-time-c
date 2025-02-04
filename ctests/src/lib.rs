#[cfg(test)]
unsafe extern "C" {
    fn rust_time_init();
    fn rust_time_instant_now(instant: *mut libc::c_void);
    fn rust_time_instant_duration_since(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        duration: *mut libc::c_void,
    );

    fn rust_time_duration_add(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        duration: *mut libc::c_void,
    );
    fn rust_time_duration_sub(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        duration: *mut libc::c_void,
    );

    fn rust_time_duration_eq(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_duration_ne(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_duration_lt(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_duration_le(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_duration_gt(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_duration_ge(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;

    fn rust_time_instant_elapsed(instant: *const libc::c_void, duration: *mut libc::c_void);
    fn rust_time_instant_add(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        instant: *mut libc::c_void,
    );
    fn rust_time_instant_sub(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        instant: *mut libc::c_void,
    );

    fn rust_time_instant_eq(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_instant_ne(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_instant_lt(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_instant_le(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_instant_gt(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
    fn rust_time_instant_ge(lhs: *const libc::c_void, rhs: *const libc::c_void) -> bool;
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
        uninit.assume_init().into_instant_unchecked()
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

    let e1 = t1.elapsed();
    let e2 = unsafe {
        let mut uninit = std::mem::MaybeUninit::<ffi_time::Duration>::uninit();
        rust_time_instant_elapsed(&raw const t1 as *const _, uninit.as_mut_ptr() as *mut _);
        uninit.assume_init().into_duration_unchecked()
    };
    let e3 = t1.elapsed();
    assert!(e3 >= e2);
    assert!(e2 >= e1);
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
    let diff = unsafe { diff.unwrap().into_duration_unchecked() };
    eprintln!(" = {:?}", diff);
    assert_eq!(t2 - t1, d);
    assert_eq!(diff, d);
}

#[test]
fn format_debug_instant() {
    unsafe {
        rust_time_init();
    }
    let t = std::time::Instant::now();
    let u = ffi_time::Instant::from_instant(t);
    let s = format!("{:?}", u);
    assert_eq!(s, format!("{:?}", t));
}

#[test]
fn format_debug_duration() {
    unsafe {
        rust_time_init();
    }
    let d = std::time::Duration::new(1, 250000);
    let u = ffi_time::Duration::from_duration(d);
    let s = format!("{:?}", u);
    assert_eq!(s, format!("{:?}", d));
}

#[test]
fn compare_instant() {
    unsafe {
        rust_time_init();
    }
    let t1 = std::time::Instant::now();
    let t2 = t1 + std::time::Duration::new(1, 250000);
    let u1 = ffi_time::Instant::from_instant(t1);
    // u1.validate();
    let u2 = ffi_time::Instant::from_instant(t2);
    // u2.validate();

    assert!(u1 == u1);
    assert!(u1 != u2);
    assert!(u1 <= u2);
    assert!(u1 < u2);
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
        assert_eq!(&rust_diff, unsafe { c_diff.as_duration_unchecked() });

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
        assert_eq!(&rust_diff, unsafe { c_diff.as_duration_unchecked()});
    }

    #[test]
    fn prop_duration_comparison(d1: std::time::Duration, d2: std::time::Duration) {
        proptest::prop_assume!(d1.as_secs() <= 0x7fffffff_ffffffff && d2.as_secs() <= 0x7fffffff_ffffffff);
        unsafe {
            rust_time_init();
        }

        let r = unsafe { rust_time_duration_eq(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 == d2, r);
        let r = unsafe { rust_time_duration_ne(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 != d2, r);
        let r = unsafe { rust_time_duration_lt(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 < d2, r, "{d1:?} < {d2:?}");
        let r = unsafe { rust_time_duration_le(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 <= d2, r, "{d1:?} <= {d2:?}");
        let r = unsafe { rust_time_duration_gt(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 > d2, r, "{d1:?} > {d2:?}");
        let r = unsafe { rust_time_duration_ge(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 >= d2, r, "{d1:?} >= {d2:?}");

        let r = unsafe {
            let mut uninit = std::mem::MaybeUninit::<ffi_time::Option<ffi_time::Duration>>::uninit();
            rust_time_duration_add(
                &raw const d1 as *const _,
                &raw const d2 as *const _,
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        if let Some(ds) = d1.checked_add(d2) {
            assert_eq!(ds, r.into_duration().unwrap());
        } else {
            assert!(r.is_none());
        }
        let (d1, d2) = if d2 > d1 {
            (d2, d1)
        } else {
            (d1, d2)
        };
        let r = unsafe {
            let mut uninit = std::mem::MaybeUninit::<ffi_time::Option<ffi_time::Duration>>::uninit();
            rust_time_duration_sub(
                &raw const d1 as *const _,
                &raw const d2 as *const _,
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        if let Some(ds) = d1.checked_sub(d2) {
            assert_eq!(ds, r.into_duration().unwrap(), "{d1:?} - {d2:?}");
        } else {
            assert!(r.is_none());
        }
    }

    #[test]
    fn prop_instant_comparison(d1: std::time::Instant, d2: std::time::Instant) {
        let (d1, d2) = if d2 > d1 {
            (d2, d1)
        } else {
            (d1, d2)
        };

        unsafe {
            rust_time_init();
        }

        let r = unsafe { rust_time_instant_eq(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 == d2, r);
        let r = unsafe { rust_time_instant_ne(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 != d2, r);
        let r = unsafe { rust_time_instant_lt(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 < d2, r, "{d1:?} < {d2:?}");
        let r = unsafe { rust_time_instant_le(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 <= d2, r, "{d1:?} <= {d2:?}");
        let r = unsafe { rust_time_instant_gt(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 > d2, r, "{d1:?} > {d2:?}");
        let r = unsafe { rust_time_instant_ge(&raw const d1 as *const _, &raw const d2 as *const _) };
        assert_eq!(d1 >= d2, r, "{d1:?} >= {d2:?}");

        let d = d1 - d2;

        let r = unsafe {
            let mut uninit = std::mem::MaybeUninit::<ffi_time::Option<ffi_time::Instant>>::uninit();
            rust_time_instant_add(
                &raw const d1 as *const _,
                &raw const d as *const _,
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        if let Some(ds) = d1.checked_add(d) {
            assert_eq!(ds, r.into_instant().unwrap());
        } else {
            assert!(r.is_none());
        }
        let (d1, d2) = if d2 > d1 {
            (d2, d1)
        } else {
            (d1, d2)
        };
        let r = unsafe {
            let mut uninit = std::mem::MaybeUninit::<ffi_time::Option<ffi_time::Instant>>::uninit();
            rust_time_instant_sub(
                &raw const d1 as *const _,
                &raw const d as *const _,
                uninit.as_mut_ptr() as *mut _,
            );
            uninit.assume_init()
        };
        if let Some(ds) = d1.checked_sub(d) {
            assert_eq!(ds, r.into_instant().unwrap(), "{d1:?} - {d2:?}");
        } else {
            assert!(r.is_none());
        }
    }
}
