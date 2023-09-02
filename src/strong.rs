use {
  std::ops::{Deref, DerefMut},
  objc::runtime
};

pub struct Strong<T> {
  ptr: *mut T
}

impl<T> Strong<T> {
  pub fn new(ptr: *mut T) -> Self {
    Self { ptr }
  }

  pub fn null() -> Self {
    Self::new(std::ptr::null_mut())
  }

  pub fn as_ptr(&self) -> *const T {
    self.ptr
  }

  pub unsafe fn as_ref(&self) -> &T {
    debug_assert!(!self.is_null());
    &(*self.ptr)
  }

  pub unsafe fn as_mut_ref(&self) -> &mut T {
    debug_assert!(!self.is_null());
    &mut (*self.ptr)
  }

  pub fn is_null(&self) -> bool {
    self.as_ptr() == std::ptr::null()
  }

  pub fn as_mut_ptr(&self) -> *mut T {
    self.ptr
  }

  pub fn init(&mut self, ptr: *mut T) {
    self.ptr = ptr;
  }
}

impl<T> Drop for Strong<T> {
  fn drop(&mut self) {
    unsafe {
      runtime::objc_release(self.ptr as *mut objc::runtime::Object);
    }
  }
}

impl<T> Deref for Strong<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.ptr }
  }
}

impl<T> DerefMut for Strong<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut (*self.ptr) }
  }
}
