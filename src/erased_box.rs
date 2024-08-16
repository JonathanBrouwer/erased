use std::ptr::NonNull;

/// Warning: dropping leaks
#[derive(Debug)]
pub struct ErasedBox {
    ptr: NonNull<()>,
}

impl ErasedBox {
    pub fn new<T>(t: Box<T>) -> ErasedBox {
        Self {
            ptr: NonNull::from(Box::leak(t)).cast(),
        }
    }

    pub unsafe fn get<T>(self) -> Box<T> {
        Box::from_raw(self.ptr.cast::<T>().as_mut())
    }

    pub unsafe fn get_ref<T>(&self) -> &T {
        self.ptr.cast::<T>().as_ref()
    }

    pub unsafe fn get_mut<T>(&mut self) -> &mut T {
        self.ptr.cast::<T>().as_mut()
    }
}

impl<T> From<Box<T>> for ErasedBox {
    fn from(value: Box<T>) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::ErasedBox;

    #[test]
    fn basic_test() {
        let erased = ErasedBox::new(Box::new(5usize));
        let r2 = unsafe { erased.get::<usize>() };
        assert_eq!(*r2, 5);
    }

    #[test]
    fn ref_test() {
        let mut erased = ErasedBox::new(Box::new(5usize));
        assert_eq!(*unsafe { erased.get_ref::<usize>() }, 5);
        * unsafe { erased.get_mut::<usize>() } = 42;
        assert_eq!(*unsafe { erased.get_ref::<usize>() }, 42);
    }
}