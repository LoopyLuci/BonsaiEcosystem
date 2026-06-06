use arc_swap::ArcSwap;
use std::sync::Arc;

/// SwapBuffer: a thin wrapper around ArcSwap for atomic hot swaps of `T`.
#[derive(Debug)]
pub struct SwapBuffer<T> {
    inner: ArcSwap<T>,
}

impl<T> SwapBuffer<T> {
    pub fn new(initial: Arc<T>) -> Self {
        Self { inner: ArcSwap::from(initial) }
    }

    pub fn load(&self) -> Arc<T> {
        self.inner.load_full()
    }

    pub fn store(&self, new: Arc<T>) {
        self.inner.store(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct P(u64);

    #[test]
    fn swapbuffer_basics() {
        let s = Arc::new(P(1));
        let buf = SwapBuffer::new(s.clone());
        let a = buf.load();
        assert_eq!(a.0, 1);
        let s2 = Arc::new(P(2));
        buf.store(s2.clone());
        let b = buf.load();
        assert_eq!(b.0, 2);
    }
}
