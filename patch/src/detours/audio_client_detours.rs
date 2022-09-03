use std::{error::Error, mem::transmute};

use detour::static_detour;
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::HANDLE,
        Media::Audio::{IAudioClient, IAudioClient_Vtbl, AUDCLNT_SHAREMODE},
    },
};

use crate::audio::endpoint::{get_endpoint, BUFFER_SIZE};

static_detour! {
  pub static GET_CURRENT_PADDING_DETOUR: extern "system" fn(IAudioClient, *mut u32) -> HRESULT;
  pub static GET_BUFFER_SIZE_DETOUR: extern "system" fn(IAudioClient, *mut u32) -> HRESULT;

  pub static GET_DEVICE_PERIOD_DETOUR: extern "system" fn(IAudioClient, *mut u32, *mut u32) -> HRESULT;
  pub static GET_MIX_FORMAT_DETOUR: extern "system" fn(IAudioClient, *mut *mut u32) -> HRESULT;
  pub static GET_STREAM_LATENCY_DETOUR: extern "system" fn(IAudioClient, *mut u32) -> HRESULT;
  pub static IS_FORMAT_SUPPORTED_DETOUR: extern "system" fn(IAudioClient, AUDCLNT_SHAREMODE, *mut u32, *mut *mut u32) -> HRESULT;
  pub static SET_EVENT_HANDLE_DETOUR: extern "system" fn(IAudioClient, HANDLE) -> HRESULT;
  pub static RESET_DETOUR: extern "system" fn(IAudioClient) -> HRESULT;
  pub static STOP_DETOUR: extern "system" fn(IAudioClient) -> HRESULT;
  pub static START_DETOUR: extern "system" fn(IAudioClient) -> HRESULT;
}

pub fn get_current_padding(this: IAudioClient, pnumpaddingframes: *mut u32) -> HRESULT {
    let res = unsafe { GET_CURRENT_PADDING_DETOUR.call(this, pnumpaddingframes) };
    if let Ok(mut opt) = get_endpoint().lock() && let Some(endpoint) = opt.as_mut() {
        endpoint.set_current_padding(unsafe { *pnumpaddingframes })
    }
    //unsafe { println!("get_current_padding, {:?}", *pnumpaddingframes) };
    res
}

pub fn get_buffer_size(this: IAudioClient, pnumbufferframes: *mut u32) -> HRESULT {
    let res = unsafe { GET_BUFFER_SIZE_DETOUR.call(this, pnumbufferframes) };
    if let Ok(mut opt) = get_endpoint().lock() && let Some(endpoint) = opt.as_mut() {
        endpoint.set_buffer_size(unsafe { *pnumbufferframes })
    }
    //unsafe { println!("get_buffer_size, {:?}", *pnumbufferframes) };
    res
}

pub unsafe fn initialize_audio_client_detours(
    vtable: *const IAudioClient_Vtbl,
) -> Result<(), detour::Error> {
    println!("initialize_audio_client_detour");
    let vtbl = &(*vtable);

    if let Err(e) = GET_CURRENT_PADDING_DETOUR
        .initialize(transmute(vtbl.GetCurrentPadding), get_current_padding)
    {
        match e {
            detour::Error::AlreadyInitialized => (),
            _ => return Err(e),
        }
    }

    if let Err(e) =
        GET_BUFFER_SIZE_DETOUR.initialize(transmute(vtbl.GetBufferSize), get_buffer_size)
    {
        match e {
            detour::Error::AlreadyInitialized => (),
            _ => return Err(e),
        }
    }

    Ok(())
}

pub unsafe fn attach_audio_client_detours() -> Result<(), detour::Error> {
    println!("attach_audio_client_detour");

    GET_CURRENT_PADDING_DETOUR.enable()?;
    GET_BUFFER_SIZE_DETOUR.enable()?;

    Ok(())
}

pub unsafe fn detach_audio_client_detours() -> Result<(), detour::Error> {
    println!("detach_audio_client_detour");

    GET_CURRENT_PADDING_DETOUR.disable()?;
    GET_BUFFER_SIZE_DETOUR.disable()?;

    Ok(())
}
