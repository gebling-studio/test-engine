#[derive(Debug)]
pub struct Erased;

#[cfg(test)]
mod test {
    use hreads::set_current_thread_as_main;
    use serial_test::serial;

    use crate::{Own, Weak};

    #[test]
    #[serial]
    fn test_erased() {
        set_current_thread_as_main();
        let a: Own<i32> = 5.into();
        let weak = a.weak();
        let erased: Weak = weak.erase();
        dbg!(&erased);
    }
}
