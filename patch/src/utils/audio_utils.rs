use std::error::Error;

use crate::{com::com_initialized, common::audio::MixFormat};
use windows::{
    core::PCWSTR,
    Win32::{
        Media::Audio::{
            eMultimedia, eRender, IAudioClient, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
        },
        System::Com::{CoCreateInstance, CLSCTX_ALL, CLSCTX_INPROC_SERVER},
    },
};

pub fn get_enumerator() -> Result<IMMDeviceEnumerator, Box<dyn Error>> {
    let imm_device_enumerator: IMMDeviceEnumerator =
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER) }?;
    Ok(imm_device_enumerator)
}

pub fn get_default_output_device() -> Result<IMMDevice, Box<dyn Error>> {
    let enumerator = get_enumerator()?;
    let immdevice = unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) }?;
    Ok(immdevice)
}

pub fn get_output_devices() -> Result<Vec<IMMDevice>, Box<dyn Error>> {
    let imm_device_enumerator: IMMDeviceEnumerator = get_enumerator()?;

    unsafe {
        let collection = imm_device_enumerator.EnumAudioEndpoints(
            eRender,
            eMultimedia
                .0
                .try_into()
                .expect("should never happen, failed to convert 1i32 to 1u32"),
        )?;

        let mut endpoints: Vec<IMMDevice> = vec![];
        let count = collection.GetCount()?;
        for n in 0..count {
            let immdevice = collection.Item(n)?;
            endpoints.push(immdevice);
        }

        Ok(endpoints)
    }
}

pub fn get_device(device_id: String) -> Result<IMMDevice, Box<dyn Error>> {
    unsafe {
        let imm_device_enumerator: IMMDeviceEnumerator = get_enumerator()?;

        let hstring = core::mem::ManuallyDrop::new(windows::core::HSTRING::from(device_id));
        let pstr = PCWSTR(hstring.as_wide().as_ptr());

        let imm_device = imm_device_enumerator.GetDevice(pstr)?;

        //Cleanup hstring
        let _ = core::mem::ManuallyDrop::into_inner(hstring);

        Ok(imm_device)
    }
}

pub fn get_mix_format() -> Result<MixFormat, Box<dyn Error>> {
    com_initialized();
    let immdevice = get_default_output_device()?;
    let format: MixFormat = unsafe {
        let iaduioclient: IAudioClient = immdevice.Activate(CLSCTX_ALL, std::ptr::null_mut())?;

        let pformat = iaduioclient.GetMixFormat()?;

        pformat.into()
    };

    Ok(format)
}
