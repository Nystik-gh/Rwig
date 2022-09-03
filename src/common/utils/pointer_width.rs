use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE},
    System::Threading::{GetCurrentProcess, IsWow64Process},
};

use once_cell::{sync::OnceCell, unsync::Lazy};

use super::{error::OSError, process_open::open_process_query_limited};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PointerWidth {
    _32,
    _64,
}

unsafe impl Send for PointerWidth {}
unsafe impl Sync for PointerWidth {}

const _32: PointerWidth = PointerWidth::_32;
const _64: PointerWidth = PointerWidth::_64;

static OS_POINTER_WIDTH: OnceCell<PointerWidth> = OnceCell::new();

/**
 * If the exe is 32-bit and running in WoW64 compatibility mode, we must be on a 64-bit OS.
 * If the exe is 64-bit, we must be on a 64-bit OS.
 * If the exe is 32-bit, and not running in WoW64 compatibility mode, we must be on a 32-bit OS.
 * If the exe is 64-bit, and running in WoW64 compatibility mode, something must be terribly wrong because that should not be possible.
*/
pub fn init_os_pointer_width() -> Result<PointerWidth, OSError> {
    match (get_compiled_pointer_width(), is_current_process_wow_64()?) {
        (_32, true) | (_64, false) => Ok(_64),
        (_32, false) => Ok(_32),
        (_64, true) => panic!("Impossible! A 64-bit program cannot be run in WoW64"),
    }
}

pub fn get_os_pointer_width() -> Result<PointerWidth, OSError> {
    if let Some(pw) = OS_POINTER_WIDTH.get() {
        Ok(*pw)
    } else {
        let pw = init_os_pointer_width()?;
        OS_POINTER_WIDTH.set(pw.clone()).ok();
        Ok(pw)
    }
}

/** Determine if the running binary was compiled for 64 bit */
pub fn is_compiled_for_64_bit() -> bool {
    cfg!(target_pointer_width = "64")
}

/** Returns the pointer width the binary was compiled with */
pub fn get_compiled_pointer_width() -> PointerWidth {
    match is_compiled_for_64_bit() {
        true => PointerWidth::_64,
        false => PointerWidth::_32,
    }
}

pub fn is_process_wow_64(handle: HANDLE) -> Result<bool, OSError> {
    if (handle.0 as *mut c_void).is_null() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "process handle is null",
        )
        .into());
    }

    let mut is_wow64 = BOOL::from(false);
    let ptr = &mut is_wow64;

    unsafe {
        match IsWow64Process(handle, ptr).as_bool() {
            true => Ok(is_wow64.as_bool() == true),
            false => Err(windows::core::Error::from_win32().into()),
        }
    }
}

pub fn is_current_process_wow_64() -> Result<bool, OSError> {
    unsafe {
        let handle = GetCurrentProcess();

        match !(handle.0 as *mut c_void).is_null() {
            true => {
                let result = is_process_wow_64(handle);
                CloseHandle(handle);
                result
            }
            false => {
                CloseHandle(handle);
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "unable to aquire current process handle",
                )
                .into())
            }
        }
    }
}

pub fn get_process_pointer_width(pid: u32) -> Result<PointerWidth, OSError> {
    let handle = open_process_query_limited(pid)?;

    if (handle.0 as *mut c_void).is_null() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unable to aquire process handle",
        )
        .into());
    }

    let iswow64 = is_process_wow_64(handle)?;
    unsafe { CloseHandle(handle) };

    match !iswow64 && get_os_pointer_width()? == PointerWidth::_64 {
        true => Ok(PointerWidth::_64),
        false => Ok(PointerWidth::_32),
    }
}
