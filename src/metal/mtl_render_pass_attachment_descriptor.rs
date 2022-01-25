use crate::{
  obj::Object,
  metal::*
};

pub trait MTLRenderPassAttachmentDescriptorBase: Object {
  unsafe fn set_texture(&self, texture: *mut MTLTexture) {
    msg_send![self, setTexture:texture]
  }

  unsafe fn set_load_action(&self, action: MTLLoadAction) {
    msg_send![self, setLoadAction:action]
  }

  unsafe fn set_store_action(&self, store_action: MTLStoreAction) {
    msg_send![self, setStoreAction: store_action]
  }
}
