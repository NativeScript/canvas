//! The native side of the canvas view on each desktop host: Windows (WinUI 3 `SwapChainPanel`)
//! and Linux (a GTK 4 widget), with the same `NSCCanvas` surface.

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;
