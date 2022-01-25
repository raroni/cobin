use crate::{
  Strong,
  PrivateMarker,
  obj::Object
};

pub trait NSObjectProtocol: Object {
  unsafe fn release(&self) {
    msg_send![self, release]
  }
}

pub trait NSObjectBase: Object {
  fn class_name() -> &'static str;

  unsafe fn alloc() -> Strong<Self> {
    let cls = Self::get_class();
    let ptr = msg_send![cls, alloc];
    Strong::new(ptr)
  }

  unsafe fn new() -> Strong<Self> {
    let cls = Self::get_class();
    let ptr = msg_send![cls, new];
    Strong::new(ptr)
  }

  unsafe fn get_class() -> &'static objc::runtime::Class {
    let name = Self::class_name();
    objc::runtime::Class::get(name).unwrap()
  }
}

impl<T: NSObjectBase> NSObjectProtocol for T {}

pub struct NSObject(PrivateMarker);

unsafe impl objc::Message for NSObject {}
impl Object for NSObject {}

impl NSObjectBase for NSObject {
  fn class_name() -> &'static str { "NSObject" }
}
