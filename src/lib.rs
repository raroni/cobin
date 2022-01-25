extern crate cty;
#[macro_use]
extern crate objc;
#[macro_use]
extern crate bitflags;

mod strong;
mod obj;
mod auto_release;
pub mod core_graphics;
pub mod runtime;
pub mod foundation;
pub mod core_animation;
pub mod app_kit;
pub mod metal;
pub mod util;

pub type Selector = objc::runtime::Sel;
pub type Strong<T> = strong::Strong<T>;
pub use auto_release::AutoReleaseContext;

// This private zero-size type is used by other types to ensure that they
// cannot be constructed from outside this crate.
type PrivateMarker = [u8; 0];
