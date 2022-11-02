#[cfg(target_arch = "wasm32")]
#[path = "lib_wasm.rs"]
mod arch_lib;

#[cfg(not(target_arch = "wasm32"))]
#[path = "lib_native.rs"]
mod arch_lib;

pub use arch_lib::*;
