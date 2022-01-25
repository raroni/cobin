use crate::obj::Object;

pub trait MTLCommandEncoderProtocol: Object {
  unsafe fn end_encoding(&self) {
    msg_send![self, endEncoding]
  }
}
