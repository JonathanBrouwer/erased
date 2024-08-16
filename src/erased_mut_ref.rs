use std::marker::PhantomData;
use std::ptr::NonNull;

/// An erased mutable reference to a value `&'a mut T`
///
/// Example:
/// ```rs
/// use erased::ErasedMut;
///
/// let value = &mut 5usize;
/// let mut erased = ErasedMut::new(value);
///
/// // Safety: Matches the type of `value` exactly, which was used to create the `erased` value
/// let r2 = unsafe { erased.get::<usize>() };
/// *r2 = 42;
///
/// // Safety: Matches the type of `value` exactly, which was used to create the `erased` value
/// assert_eq!(*unsafe { erased.get_ref::<usize>() }, 42);
/// ```
#[derive(Debug)]
pub struct ErasedMut<'a> {
    ptr: NonNull<()>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ErasedMut<'a> {
    /// Create a new erased mutable reference from a mutable reference to `T`
    pub fn new<T>(t: &'a mut T) -> ErasedMut<'a> {
        Self {
            ptr: NonNull::from(t).cast(),
            phantom: PhantomData,
        }
    }

    /// Get a mutable reference to `T` back from the erased mutable reference.
    ///
    /// # Safety
    /// The generic argument `T` of this function must match the `T` that was used to create this erased reference in `ErasedMut::new` exactly.
    /// Pay specific attention that any lifetime parameters of `T` match.
    ///
    /// It is **strongly recommended** to provide `T` explicitly, even if it can be inferred. This is to make sure that the value of `T` is not accidentally changed.
    pub unsafe fn get<T>(&mut self) -> &'a mut T {
        self.ptr.cast::<T>().as_mut()
    }

    /// Get a reference to `T` back from the erased mutable reference.
    ///
    /// # Safety
    /// The generic argument `T` of this function must match the `T` that was used to create this erased reference in `ErasedMut::new` exactly.
    /// Pay specific attention that any lifetime parameters of `T` match.
    ///
    /// It is **strongly recommended** to provide `T` explicitly, even if it can be inferred. This is to make sure that the value of `T` is not accidentally changed.
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
