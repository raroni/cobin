use crate::{
  Strong,
  obj::Object,
  PrivateMarker,
  foundation::NSString,
  metal::*
};

pub struct MTLLibrary(PrivateMarker);

impl Object for MTLLibrary {}
unsafe impl objc::Message for MTLLibrary {}

impl MTLLibrary {
  pub unsafe fn new_function_with_name(&self, name: *mut NSString) -> Strong<MTLFunction> {
    let ptr = msg_send![self, newFunctionWithName:name];
    Strong::new(ptr)
  }
}
