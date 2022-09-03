use crate::common::utils::error::InjectionError;
use dll_syringe::{process::OwnedProcess, Syringe};

use super::payload::get_payload;

pub fn eject_hook(pid: u32, payload_path: String) -> Result<(), InjectionError> {
    let process = OwnedProcess::from_pid(pid)?;

    let syringe = Syringe::for_process(process);

    let module = syringe.find_or_inject(payload_path)?;

    syringe.eject(module)?;
    Ok(())
}
