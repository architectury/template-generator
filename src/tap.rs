pub trait Tap {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self) -> ();
}

impl<T> Tap for T {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self) -> (),
    {
        f(&self);
        self
    }
}
