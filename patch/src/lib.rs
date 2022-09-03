#![cfg(all(windows))]
#![feature(once_cell)]
#![feature(slice_from_ptr_range)]
#![feature(vec_into_raw_parts)]
//! A `MessageBoxW` detour example.
//!
//! Ensure the crate is compiled as a 'cdylib' library to allow C interop.

#[macro_use]
extern crate lazy_static;

#[path = "../../src/common/mod.rs"]
mod common;

mod audio;
mod com;
mod detours;
mod ipc;
mod utils;
use utils::DWORD;
use utils::LPVOID;

use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console::AllocConsole;

use crate::detours::detach_detours;
use crate::detours::init_detours;
//use crate::detour::{detatch_detour, initialize_detour};
use crate::utils::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
//use crate::vtable_patch::detatch_vtable_detour;
//use crate::vtable_patch::init_vtable_detour;

dll_syringe::payload_procedure! {
    fn printstr(msg: String) {
        println!("{}", msg);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _module: HINSTANCE,
    call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    AllocConsole();

    println!("DLL main");
    if call_reason == DLL_PROCESS_ATTACH {
        println!("Injected!");
        // A console may be useful for printing to 'stdout'
        // Preferably a thread should be created here instead, since as few
        // operations as possible should be performed within `DllMain`.
        //let s_attr = ptr::null() as *mut SECURITY_ATTRIBUTES;
        //CreateThread(s_attr, 0, &main, NULL, 0, {*mut u32});
        //CreateThread(s_attr, 0, &main, 0, 0, NULL);
        BOOL::from(true)
        //BOOL::from(true)
        //std::thread::spawn(|| main());
        //TRUE
    } else if call_reason == DLL_PROCESS_DETACH {
        println!("Ejected!");
        BOOL::from(true)
        //BOOL::from(true)
        //std::thread::spawn(|| detatch_detour());
        //TRUE
    } else {
        true.into()
    }
}
