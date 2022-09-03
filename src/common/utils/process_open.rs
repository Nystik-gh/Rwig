use windows::Win32::{
    Foundation::{BOOL, HANDLE},
    System::Threading::{
        OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_ALL_ACCESS, PROCESS_QUERY_LIMITED_INFORMATION,
    },
};

pub fn open_process_all_access(pid: u32) -> Result<HANDLE, windows::core::Error> {
    Ok(open_process(pid, PROCESS_ALL_ACCESS)?)
}

pub fn open_process_query_limited(pid: u32) -> Result<HANDLE, windows::core::Error> {
    Ok(open_process(pid, PROCESS_QUERY_LIMITED_INFORMATION)?)
}

pub fn open_process(
    pid: u32,
    desired_access: PROCESS_ACCESS_RIGHTS,
) -> Result<HANDLE, windows::core::Error> {
    Ok(unsafe { OpenProcess(desired_access, BOOL::from(false), pid) }?)
}
