#[cfg(target_arch = "wasm32")]
pub mod platform {
    mod wasm;
    pub use self::wasm::*;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod platform {
    mod native;
    pub use self::native::*;
}
