use crate::common::utils::{
    error::{InjectionError, PatchError},
    get_process_pointer_width, PointerWidth,
};
use dll_syringe::{error::InjectError, rpc::RemotePayloadProcedure};

use super::{
    eject::eject_hook,
    inject::inject_hook,
    payload::{get_payload, PATCH_NAME_DEFAULT},
    CompatibilityInfo,
};

pub struct Injector {
    compatibility: CompatibilityInfo,
    patch_name: String,
}

impl Injector {
    pub fn new(compatibility: CompatibilityInfo) -> Injector {
        Injector {
            compatibility,
            patch_name: PATCH_NAME_DEFAULT.to_string(),
        }
    }

    pub fn override_patch_name(&mut self, patch_name: String) {
        self.patch_name = patch_name;
    }

    pub fn inject(
        &self,
        pid: u32,
    ) -> Result<RemotePayloadProcedure<fn(String) -> Result<(), PatchError>>, InjectionError> {
        let target_pointer_width =
            get_process_pointer_width(pid).map_err(|e| InjectError::ProcessInaccessible)?;

        let unsupported = self.compatibility.module == PointerWidth::_32
            && self.compatibility.os == PointerWidth::_64
            && target_pointer_width == PointerWidth::_64;

        if unsupported {
            return Err(InjectionError::UnsupportedTargetProcess {
                expected: self.compatibility.module.clone(),
                found: target_pointer_width.clone(),
            });
        }

        let payload_path = get_payload(&target_pointer_width, &self.patch_name)?;

        inject_hook(pid, payload_path)
    }

    pub fn eject(&self, pid: u32) -> Result<(), InjectionError> {
        let target_pointer_width =
            get_process_pointer_width(pid).map_err(|e| InjectError::ProcessInaccessible)?;

        let payload_path = get_payload(&target_pointer_width, &self.patch_name)?;

        eject_hook(pid, payload_path)
    }
}
