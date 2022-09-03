use std::{error::Error, ffi::c_void};

pub type DWORD = u32;
pub type LPVOID = *mut c_void;
pub type UINT = u32;

pub const DLL_PROCESS_DETACH: DWORD = 0;
pub const DLL_PROCESS_ATTACH: DWORD = 1;
