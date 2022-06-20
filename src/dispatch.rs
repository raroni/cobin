use crate::kernel;

#[allow(non_camel_case_types)]
pub type dispatch_time_t = u64;

#[allow(non_camel_case_types)]
pub type dispatch_semaphore_t = *mut u8;

pub const DISPATCH_TIME_NOW: dispatch_time_t     = 0;
pub const DISPATCH_TIME_FOREVER: dispatch_time_t = !0;

extern "C" {
  pub fn dispatch_semaphore_create(value: kernel::intptr_t) -> dispatch_semaphore_t;
  pub fn dispatch_semaphore_signal(sem: dispatch_semaphore_t) -> kernel::intptr_t;
  pub fn dispatch_semaphore_wait(sem: dispatch_semaphore_t, timeout: dispatch_time_t);
}
