use crate::PrivateMarker;

#[repr(C)]
pub struct __CFString(PrivateMarker);

pub type CFStringRef = *const __CFString;
