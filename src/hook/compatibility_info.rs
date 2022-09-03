use crate::common::utils::{
    error::OSError, get_compiled_pointer_width, get_os_pointer_width, PointerWidth,
};

pub struct CompatibilityInfo {
    pub os: PointerWidth,
    pub module: PointerWidth,
    _private: (),
}

impl CompatibilityInfo {
    pub fn get() -> Result<CompatibilityInfo, OSError> {
        Ok(CompatibilityInfo {
            os: get_os_pointer_width()?,
            module: get_compiled_pointer_width(),
            _private: (),
        })
    }
}
