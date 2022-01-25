use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase
};

pub struct MTLCompileOptions(PrivateMarker);

impl Object for MTLCompileOptions {}
unsafe impl objc::Message for MTLCompileOptions {}

impl NSObjectBase for MTLCompileOptions {
  fn class_name() -> &'static str { "MTLCompileOptions" }
}
