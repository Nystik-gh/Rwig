use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    time::Duration,
};

use crate::common::{audio::MixFormat, utils::error::PatchError};
use ipc_channel::ipc::IpcBytesSender;
use once_cell::sync::{Lazy, OnceCell};

pub static ENDPOINT: Lazy<Arc<Mutex<Option<SimulatedEndpoint>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub const BUFFER_SIZE: usize = 88200;

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
        /*if (frames_written > 0 && self.pbuffer.is_some()) {
            match self.consumer_channel.send(ConsumerInfo {
                format: self.format,
                pbuffer: self.pbuffer.unwrap(),
                buffer_size_frames: self.buffer_size,
                padding_frames: self.current_padding,
                frames_written,
            }) {
                Ok(_) => (),
                Err(e) => println!("failed to send to consumer thread"),
            }
        }*/
        //let length = frames_written as usize * self.format.format.block_align as usize;
        /*println!(
            "my_buffer: {:?}, device_buffer: {:?}",
            self.p_my_buffer, self.pbuffer
        );
        println!(
            "my_buffer_size: {}, device_buffer_size: {}",
            self.my_buffer_size, self.buffer_size
        );
        println!("current_padding: {}", self.current_padding);
        let data = self.read_buffer(length);
        println!("after read my_buffer");
        self.write_buffer(length, data);
        println!("after write device_buffer");*/
        //self.sender.send(data.as_slice()).ok();
        let data = self.read_buffer(frames_written);
        //self.sender.send(data.as_slice()).ok();
    }

    /*fn consumer(rx: Receiver<ConsumerInfo>) {
        std::thread::spawn(move || {
            loop {
                let info = match rx.recv() {
                    Ok(flag) => flag,
                    Err(_) => break,
                };

                let buffer_size_bytes =
                    info.buffer_size_frames * info.format.format.block_align as u32;
                let offset_bytes = info.padding_frames * info.format.format.block_align as u32;
                let length_bytes = info.frames_written * info.format.format.block_align as u32;

                println!("buffer_pointer: {:?}", info.pbuffer);
                println!("buffer_size: {}", buffer_size_bytes);
                println!("current_padding: {}", offset_bytes);

                unsafe {
                    let poffset = info.pbuffer.offset(offset_bytes as isize);

                    let mut out = vec![];
                    for n in 0..length_bytes {
                        let p_byte = poffset.offset(n as isize);
                        let b = *p_byte;
                        out.push(b);
                    }
                }
            }
            println!("consumer stopped");
        });
    }*/

    pub fn read_buffer(&mut self, frames_written: u32) -> Vec<u8> {
        //let mut vec = vec![0u8; length];
        //let out = vec.as_mut_slice();
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

    /*pub fn write_buffer(&self, length: usize, data: Vec<u8>) {
        unsafe {
            if let Some(ptr) = self.pbuffer {
                let poffset = ptr.offset(
                    self.current_padding as isize * self.format.format.block_align as isize,
                );

                for n in 0..length as usize {
                    let w_ptr = poffset.offset(n as isize);
                    *w_ptr = data[n];
                }
            }
        }
    }*/
}

pub fn create_endpoint(sender: IpcBytesSender, format: MixFormat) -> Result<(), PatchError> {
    let endpoint = SimulatedEndpoint::new(sender, format);

    set_endpoint(endpoint)
}

pub fn set_endpoint(
    //receiver: IpcReceiver<IpcMessage>,
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
