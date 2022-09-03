use std::{
    sync::{
        Arc, Mutex,
    },
};

use crate::common::{audio::MixFormat, utils::error::PatchError};
use ipc_channel::ipc::IpcBytesSender;
use once_cell::sync::{Lazy};

pub static ENDPOINT: Lazy<Arc<Mutex<Option<SimulatedEndpoint>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub const _BUFFER_SIZE: usize = 88200;

#[derive(Debug, Clone)]
pub struct SimulatedEndpoint {
    pub format: MixFormat,
    pbuffer: Option<*mut u8>,
    current_padding: u32,
    buffer_size: u32,
    sender: IpcBytesSender,
}

unsafe impl Send for SimulatedEndpoint {}
unsafe impl Sync for SimulatedEndpoint {}

impl SimulatedEndpoint {
    pub fn new(sender: IpcBytesSender, format: MixFormat) -> SimulatedEndpoint {

        SimulatedEndpoint {
            format,
            pbuffer: None,
            buffer_size: 0,
            current_padding: 0,
            sender,
        }
    }

    pub fn set_buffer_pointer(&mut self, ptr: *mut u8) {
        let _ = self.pbuffer.insert(ptr);
    }

    pub fn set_buffer_size(&mut self, size: u32) {
        self.buffer_size = size;
    }

    pub fn set_current_padding(&mut self, padding: u32) {
        self.current_padding = padding;
    }

    pub fn release_buffer(&mut self, frames_written: u32) {
        let data = self.read_buffer(frames_written);
        //self.sender.send(data.as_slice()).ok();
    }


    pub fn read_buffer(&mut self, frames_written: u32) -> Vec<u8> {
        let mut out = vec![];

        let frame_size = self.format.format.block_align as u32;

        let padding = self.current_padding * frame_size;
        let buffer_size = self.buffer_size * frame_size;
        let bytes_written = frames_written * frame_size;

        let new_padding = padding + bytes_written;

        if frames_written > 0 && new_padding < buffer_size && let Some(ptr) = self.pbuffer {
          unsafe { 
              let poffset = ptr.offset(padding as isize);

              let slice = std::slice::from_raw_parts(poffset, bytes_written as usize);

              println!("length {}", slice.len());

              for (i, byte) in slice.iter().enumerate() {
                  out.push(*byte);
              }
          }
        }
        out
    }
}

pub fn create_endpoint(sender: IpcBytesSender, format: MixFormat) -> Result<(), PatchError> {
    let endpoint = SimulatedEndpoint::new(sender, format);

    set_endpoint(endpoint)
}

pub fn set_endpoint(
    endpoint: SimulatedEndpoint,
) -> Result<(), PatchError> {
    if let Ok(mut opt) = ENDPOINT.lock() {
        let _ = opt.insert(endpoint);
    }

    Ok(())
}

pub fn get_endpoint() -> Arc<Mutex<Option<SimulatedEndpoint>>> {
    let arc = ENDPOINT.clone();
    arc
}

//must be called after detach detours
pub fn drop_endpoint() {
    if let Ok(mut opt) = ENDPOINT.lock() && opt.is_some() {
        let _ = opt.take();
    }
}