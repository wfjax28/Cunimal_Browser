#[cfg(target_os = "linux")]
//mod linux;
mod linux_test;
#[cfg(target_os = "linux")]
//pub use linux::*;
pub use linux_test::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub use ios::*;

//#[cfg(target_os = "chromeos")]
//mod chromeos;
//#[cfg(target_os = "chromeos")]
//pub use chromeos::*;
