use crate::{Own, Weak};

pub type OwnVec<T> = Vec<Own<T>>;
pub type WeakVec<T> = Vec<Weak<T>>;

pub trait RefsVec<T> {
    fn into_own(self) -> OwnVec<T>;
}

impl<T: 'static> RefsVec<T> for Vec<T> {
    fn into_own(self) -> OwnVec<T> {
        self.into_iter().map(|v| Own::new(v)).collect()
    }
}

pub trait WeakVecHelper<T: ?Sized> {
    fn remove_freed(&mut self);
}

impl<T: ?Sized> WeakVecHelper<T> for WeakVec<T> {
    fn remove_freed(&mut self) {
        self.retain(Weak::is_ok);
    }
}

#[cfg(test)]
mod test {
    use hreads::set_current_thread_as_main;
    use serial_test::serial;

    use crate::vec::{OwnVec, RefsVec};

    #[test]
    #[serial]
    fn test_own_vec() {
        set_current_thread_as_main();
        let vec: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut owned_vec: OwnVec<u32> = vec.clone().into_own();
        let owned_vec2: OwnVec<u32> = vec.into_own();

        assert_eq!(owned_vec, owned_vec2);

        assert_eq!(owned_vec[3], 4);

        assert_eq!(owned_vec.pop().unwrap(), 5);
        set_current_thread_as_main();
    }
}
