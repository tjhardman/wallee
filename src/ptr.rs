use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ptr::NonNull;

#[repr(transparent)]
pub struct OwnPtr<T>
where
    T: ?Sized,
{
    pub ptr: NonNull<T>,
}

unsafe impl<T> Send for OwnPtr<T> where T: ?Sized {}

unsafe impl<T> Sync for OwnPtr<T> where T: ?Sized {}

impl<T> Copy for OwnPtr<T> where T: ?Sized {}

impl<T> Clone for OwnPtr<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> OwnPtr<T>
where
    T: ?Sized,
{
    pub fn new(ptr: Box<T>) -> Self {
        OwnPtr {
            ptr: unsafe { NonNull::new_unchecked(Box::into_raw(ptr)) },
        }
    }

    pub fn from_raw(ptr: NonNull<T>) -> Self {
        OwnPtr { ptr }
    }

    pub fn cast<U: CastTo>(self) -> OwnPtr<U::Target> {
        OwnPtr {
            ptr: self.ptr.cast(),
        }
    }

    pub unsafe fn boxed(self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr.as_ptr()) }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn as_ref(&self) -> RefPtr<T> {
        RefPtr {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }

    pub fn as_mut(&mut self) -> MutPtr<T> {
        MutPtr {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }

    unsafe pub fn deref<'a>(&self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }

    unsafe pub fn deref_mut<'a>(&mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }
}

#[repr(transparent)]
pub struct RefPtr<'a, T>
where
    T: ?Sized,
{
    pub ptr: NonNull<T>,
    lifetime: PhantomData<&'a T>,
}

impl<T> Copy for RefPtr<'_, T> where T: ?Sized {}

impl<T> Clone for RefPtr<'_, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> RefPtr<'a, T>
where
    T: ?Sized,
{
    pub fn new(ptr: &'a T) -> Self {
        RefPtr {
            ptr: NonNull::from(ptr),
            lifetime: PhantomData,
        }
    }

    pub fn cast<U: CastTo>(self) -> RefPtr<'a, U::Target> {
        RefPtr {
            ptr: self.ptr.cast(),
            lifetime: PhantomData,
        }
    }

    unsafe pub fn as_ref(self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }
}

#[repr(transparent)]
pub struct MutPtr<'a, T>
where
    T: ?Sized,
{
    pub ptr: NonNull<T>,
    lifetime: PhantomData<&'a mut T>,
}

impl<T> Copy for MutPtr<'_, T> where T: ?Sized {}

impl<T> Clone for MutPtr<'_, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> MutPtr<'a, T>
where
    T: ?Sized,
{
    pub fn cast<U: CastTo>(self) -> MutPtr<'a, U::Target> {
        MutPtr {
            ptr: self.ptr.cast(),
            lifetime: PhantomData,
        }
    }

    unsafe pub fn as_mut(&mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }
}

// Force turbofish on all calls of `.cast::<U>()`.
pub trait CastTo {
    type Target;
}

impl<T> CastTo for T {
    type Target = T;
}
