use lru_cache_macros::lru_cache;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn thread_local_ignore_args() {
    #[lru_cache(20)]
    #[lru_config(ignore_args = call_count)]
    #[lru_config(thread_local)]
    fn fib(x: u32, call_count: &mut u32) -> u64 {
        *call_count += 1;
        if x <= 1 {
            1
        } else {
            fib(x - 1, call_count) + fib(x - 2, call_count)
        }
    }

    let mut call_count = 0;
    assert_eq!(fib(39, &mut call_count), 102_334_155);
    assert_eq!(call_count, 40);
}

#[test]
fn multithreaded() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[lru_cache(20)]
    fn fib(x: u32) -> u64 {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        if x <= 1 {
            1
        } else {
            fib(x - 1) + fib(x - 2)
        }
    }

    let t1 = thread::spawn( || {
        assert_eq!(fib(39), 102_334_155);
    });

    let t2 = thread::spawn( || {
        assert_eq!(fib(39), 102_334_155);
    });

    let t3 = thread::spawn( || {
        assert_eq!(fib(39), 102_334_155);
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    // threads should share a cache, so total runs should be less than 40 * 3
    assert!(CALL_COUNT.load(Ordering::SeqCst) < 120);
}
