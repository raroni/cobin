use {
  std::marker::PhantomData,
  crate::{
    obj::Object,
    PrivateMarker,
    runtime::NSObjectBase,
    foundation::NSArrayBase
  }
};

pub struct NSMutableArray<T: 'static>(PrivateMarker, PhantomData<T>);

impl<T> Object for NSMutableArray<T> {}
unsafe impl<T> objc::Message for NSMutableArray<T> {}

impl<T> NSObjectBase for NSMutableArray<T> {
  fn class_name() -> &'static str { "NSMutableArray" }
}

impl<T> NSArrayBase<T> for NSMutableArray<T> {}

impl<T> NSMutableArray<T> {
  pub unsafe fn add_object(&mut self, obj: T) {
    msg_send![self, addObject:obj]
  }
}
