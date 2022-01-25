use {
  std::marker::PhantomData,
  crate::{
    obj::Object,
    runtime::NSObjectBase,
    PrivateMarker
  }
};

pub trait NSArrayBase<T>: Object { }

pub struct NSArray<T: 'static>(PrivateMarker, PhantomData<T>);

impl<T> Object for NSArray<T> {}
unsafe impl<T> objc::Message for NSArray<T> {}

impl<T> NSObjectBase for NSArray<T> {
  fn class_name() -> &'static str { "NSArray" }
}

impl<T> NSArrayBase<T> for NSArray<T> { }

impl<T> NSArray<T> { }
