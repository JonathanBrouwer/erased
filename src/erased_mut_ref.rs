use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct ErasedMut<'a> {
    ptr: NonNull<()>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ErasedMut<'a> {
    pub fn new<T>(t: &'a mut T) -> ErasedMut<'a> {
        Self {
            ptr: NonNull::from(t).cast(),
            phantom: PhantomData::default(),
        }
    }

    pub unsafe fn get<T>(&mut self) -> &'a mut T {
        self.ptr.cast::<T>().as_mut()
    }

    pub unsafe fn get_ref<T>(&self) -> &'a T {
        self.ptr.cast::<T>().as_ref()
    }
}

impl<'a, T> From<&'a mut T> for ErasedMut<'a> {
    fn from(value: &'a mut T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::ErasedMut;

    #[test]
    fn basic_test() {
        let r1 = &mut 5usize;
        let mut erased = ErasedMut::new(r1);
        let r2 = unsafe { erased.get::<usize>() };
        *r2 = 42;
        assert_eq!(*unsafe { erased.get_ref::<usize>() }, 42);
        assert_eq!(*r1, 42);
    }
}
