//! The native side of the canvas view on each desktop host. Windows (WinUI 3 `SwapChainPanel`) is
//! the first; macOS/Linux hosts add a module here with the same `NSCCanvas` surface.

#[cfg(target_os = "windows")]
pub mod windows;
