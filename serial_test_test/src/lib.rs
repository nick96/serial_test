#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use serial_test::serial;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    lazy_static! {
        static ref LOCK: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        static ref MULTI_KEY_LOCK_1: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        static ref MULTI_KEY_LOCK_2: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[serial]
    fn test_serial_no_arg() {
        init();
    }

    #[test]
    #[serial(alpha)]
    fn test_serial_1() {
        init();
        println!("Start 1");
        LOCK.store(1, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(100));
        println!("End 1");
        assert_eq!(LOCK.load(Ordering::Relaxed), 1);
    }

    #[test]
    #[serial(alpha)]
    fn test_serial_2() {
        init();
        println!("Start 2");
        LOCK.store(2, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(200));
        println!("End 2");
        assert_eq!(LOCK.load(Ordering::Relaxed), 2);
    }

    #[test]
    #[serial(alpha)]
    fn test_serial_3() {
        init();
        println!("Start 3");
        LOCK.store(3, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(300));
        println!("End 3");
        assert_eq!(LOCK.load(Ordering::Relaxed), 3);
    }

    #[test]
    #[serial]
    #[ignore]
    fn test_ignore_fun() {
        init();
        assert_eq!(2 + 2, 5);
    }

    #[test]
    #[serial]
    fn test_reentrant_fun() {
        init();
        test_serial_no_arg();
    }

    #[test]
    #[serial]
    #[should_panic]
    fn test_should_panic_fun() {
        init();
        panic!("Testing panic");
    }

    #[test]
    #[serial]
    fn test_can_return() -> Result<(), ()> {
        init();
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_async_serial_no_arg() {
        init();
    }

    #[actix_rt::test]
    #[serial]
    async fn test_async_serial_no_arg_actix() {
        init();
    }

    #[tokio::test]
    #[serial]
    async fn test_async_can_return() -> Result<(), ()> {
        init();
        Ok(())
    }

    #[test]
    #[serial(multikey_1, multikey_2)]
    fn test_serial_multiple_keys() {
        init();
        MULTI_KEY_LOCK_1.store(1, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(200));
        assert_eq!(MULTI_KEY_LOCK_1.load(Ordering::Relaxed), 1);
        MULTI_KEY_LOCK_2.store(1, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(200));
        assert_eq!(MULTI_KEY_LOCK_2.load(Ordering::Relaxed), 1);
    }

    #[test]
    #[serial(multikey_1)]
    fn test_serial_multiple_keys_dependant_1() {
        init();
        MULTI_KEY_LOCK_1.store(2, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(100));
        assert_eq!(MULTI_KEY_LOCK_1.load(Ordering::Relaxed), 2);
    }

    #[test]
    #[serial(multikey_2)]
    fn test_serial_multiple_keys_dependant_2() {
        init();
        MULTI_KEY_LOCK_2.store(2, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(100));
        assert_eq!(MULTI_KEY_LOCK_2.load(Ordering::Relaxed), 2);
    }
}
