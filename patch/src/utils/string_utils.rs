use std::{error::Error, ffi::OsString, os::windows::prelude::OsStringExt};

use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Devices::Properties::DEVPKEY_Device_FriendlyName,
        Media::Audio::IMMDevice,
        System::{
            Com::StructuredStorage::{PropVariantClear, PROPVARIANT, STGM_READ},
            Ole::VT_LPWSTR,
        },
    },
};

pub enum StringPointer {
    PWSTR_(PWSTR),
    PCWSTR_(PCWSTR),
}

impl From<PWSTR> for StringPointer {
    fn from(pwstr: PWSTR) -> Self {
        StringPointer::PWSTR_(pwstr)
    }
}

impl From<PCWSTR> for StringPointer {
    fn from(pcwstr: PCWSTR) -> Self {
        StringPointer::PCWSTR_(pcwstr)
    }
}

pub fn stringw_from_pointer(pstr: StringPointer) -> Result<String, std::io::Error> {
    let ptr = unsafe {
        match pstr {
            StringPointer::PWSTR_(pwstr) => *(&pwstr as *const _ as *const *const u16),
            StringPointer::PCWSTR_(pcwstr) => *(&pcwstr as *const _ as *const *const u16),
        }
    };
    match ptr.is_null() {
        true => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "pointer is null",
        )),
        false => unsafe {
            let mut len = 0;
            while *ptr.offset(len) != 0 {
                len += 1;
            }
            // Create the utf16 slice and convert it into a string.
            let name_slice = std::slice::from_raw_parts(ptr, len as usize);
            let name_os_string: OsString = OsStringExt::from_wide(name_slice);
            match name_os_string.into_string() {
                Ok(string) => Ok(string),
                Err(os_string) => Ok(os_string.to_string_lossy().into()),
            }
        },
    }
}

pub fn get_device_name(device: &IMMDevice) -> Result<String, Box<dyn Error>> {
    unsafe {
        let property_store = device.OpenPropertyStore(STGM_READ)?;

        let mut prop_value: PROPVARIANT =
            property_store.GetValue(&DEVPKEY_Device_FriendlyName as *const _ as *const _)?;

        if prop_value.Anonymous.Anonymous.vt != VT_LPWSTR.0 as u16 {
            let description = format!(
                "property store produced invalid data: {:?}",
                prop_value.Anonymous.Anonymous.vt
            );
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, description).into());
        }

        let name = stringw_from_pointer(prop_value.Anonymous.Anonymous.Anonymous.pwszVal.into())?;

        PropVariantClear(&mut prop_value)?;

        Ok(name)
    }
}
