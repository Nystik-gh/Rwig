use std::{env, path::Path};

use crate::common::utils::{error::InjectionError, PointerWidth};
use dll_syringe::error::InjectError;

pub const PATCH_NAME_DEFAULT: &str = "patch";

pub fn get_payload(
    target_pointer_width: &PointerWidth,
    patch_name: &String,
) -> Result<String, InjectionError> {
    let payload_path = match target_pointer_width {
        PointerWidth::_32 => get_patch_path_absolute(&format!("{}_32.dll", patch_name)),
        PointerWidth::_64 => get_patch_path_absolute(&format!("{}.dll", patch_name)),
    }?;

    println!("{}", payload_path);

    match Path::new(&payload_path).exists() {
        true => Ok(payload_path),
        false => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("patch not found: {}", payload_path),
        )
        .into()),
    }
}

fn get_patch_path_absolute(patch_name: &String) -> Result<String, std::io::Error> {
    env::current_exe().map(|mut exe_path| {
        exe_path.pop();
        exe_path.push(patch_name);
        exe_path.to_string_lossy().to_string()
    })
}
