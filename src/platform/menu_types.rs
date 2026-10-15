//! Platform-agnostic menu types.
//!
//! This module provides unified menu types that work across all platforms,
//! using muda on native desktop platforms and wasm_stubs on wasm32, Android,
//! and iOS (where muda has no backend).

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub use muda::{
    CheckMenuItem, Icon, IconMenuItem, IsMenuItem, Menu, MenuId, MenuItem, NativeIcon,
    PredefinedMenuItem, Submenu, accelerator::Accelerator,
};

// MenuItemKind is only used in context_menu.rs, which is only compiled on Linux/FreeBSD/wasm32
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub use muda::MenuItemKind;

#[cfg(any(target_arch = "wasm32", target_os = "android", target_os = "ios"))]
pub use crate::platform::wasm_stubs::{
    Accelerator, CheckMenuItem, Icon, IconMenuItem, IsMenuItem, Menu, MenuId, MenuItem,
    MenuItemKind, NativeIcon, PredefinedMenuItem, Submenu,
};
