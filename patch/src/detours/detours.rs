use std::error::Error;

use crate::{
    com::com_initialized,
    detours::{
        audio_client_detours::{
            attach_audio_client_detours, detach_audio_client_detours,
            initialize_audio_client_detours,
        },
        render_client_detours::{
            attach_render_client_detours, detach_render_client_detours,
            initialize_render_client_detours,
        },
    },
};

use super::get_vtables;

pub unsafe fn init_detours() -> Result<(), detour::Error> {
    com_initialized();
    println!("init_detours");

    let vtables = match get_vtables() {
        Ok(vt) => vt,
        Err(e) => panic!("Error {:?}", e),
    };

    initialize_audio_client_detours(vtables.iaudioclient_vtbl)?;
    initialize_render_client_detours(vtables.iaudiorenderclient_vtbl)?;

    Ok(())
}

pub unsafe fn attach_detours() -> Result<(), detour::Error> {
    println!("attach_detours");

    attach_audio_client_detours()?;
    attach_render_client_detours()?;

    Ok(())
}

pub unsafe fn detach_detours() -> Result<(), detour::Error> {
    println!("detach_detours");

    detach_audio_client_detours()?;
    detach_render_client_detours()?;

    Ok(())
}
