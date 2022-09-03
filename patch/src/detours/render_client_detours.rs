use std::{
    mem::transmute,
    sync::{Arc, Mutex},
};

use detour::static_detour;
use once_cell::sync::OnceCell;
use windows::{
    core::HRESULT,
    Win32::Media::Audio::{
        IAudioRenderClient, IAudioRenderClient_Vtbl, AUDCLNT_BUFFERFLAGS_SILENT,
        AUDCLNT_E_BUFFER_ERROR, AUDCLNT_E_DEVICE_INVALIDATED,
    },
};

use crate::{audio::endpoint::get_endpoint, ipc::send_log};

pub static AUDIO_BUFFER: OnceCell<Arc<Mutex<[u8]>>> = OnceCell::new();

static_detour! {
    pub static GET_BUFFER_DETOUR: extern "system" fn(IAudioRenderClient, u32, *mut *mut u8) -> HRESULT;
    pub static RELEASE_BUFFER_DETOUR: extern "system" fn(IAudioRenderClient, u32, u32) -> HRESULT;

}

fn get_buffer(this: IAudioRenderClient, numframesrequested: u32, ppdata: *mut *mut u8) -> HRESULT {
    println!("get_buffer, {}", numframesrequested);
    let res = unsafe { GET_BUFFER_DETOUR.call(this, numframesrequested, ppdata) };
    //send_log(format!("get_buffer, {}", numframesrequested)).ok();
    if let Ok(mut opt) = get_endpoint().lock() && let Some(endpoint) = opt.as_mut() {
        unsafe { 
          endpoint.set_buffer_pointer(*ppdata);
        }
        //println!("release_buffer, {}", numframeswritten);
    };
    res
}

fn release_buffer(this: IAudioRenderClient, numframeswritten: u32, dwflags: u32) -> HRESULT {
    println!("release_buffer, {}", numframeswritten);
    if let Ok(mut opt) = get_endpoint().lock() && let Some(endpoint) = opt.as_mut() {
        endpoint.release_buffer(numframeswritten);
        //println!("release_buffer, {}", numframeswritten);
    };
    unsafe {
        RELEASE_BUFFER_DETOUR.call(
            this,
            numframeswritten,
            dwflags, /*AUDCLNT_BUFFERFLAGS_SILENT.0.try_into().unwrap()*/
        )
    }
}

pub unsafe fn initialize_render_client_detours(
    vtable: *const IAudioRenderClient_Vtbl,
) -> Result<(), detour::Error> {
    println!("initialize_render_client_detour");
    let vtbl = &(*vtable);
    if let Err(e) = GET_BUFFER_DETOUR.initialize(transmute(vtbl.GetBuffer), get_buffer) {
        match e {
            detour::Error::AlreadyInitialized => (),
            _ => return Err(e),
        }
    }
    if let Err(e) = RELEASE_BUFFER_DETOUR.initialize(transmute(vtbl.ReleaseBuffer), release_buffer)
    {
        match e {
            detour::Error::AlreadyInitialized => (),
            _ => return Err(e),
        }
    }
    Ok(())
}

pub unsafe fn attach_render_client_detours() -> Result<(), detour::Error> {
    println!("attach_render_client_detour");

    GET_BUFFER_DETOUR.enable()?;
    RELEASE_BUFFER_DETOUR.enable()?;

    Ok(())
}

pub unsafe fn detach_render_client_detours() -> Result<(), detour::Error> {
    println!("detach_render_client_detour");

    GET_BUFFER_DETOUR.disable()?;
    RELEASE_BUFFER_DETOUR.disable()?;

    Ok(())
}
