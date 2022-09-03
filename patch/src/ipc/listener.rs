use std::time::Duration;

use crate::{
    audio::endpoint::drop_endpoint,
    common::{
        ipc::{IpcMessage, PatchCommand},
        utils::error::PatchError,
    },
};
use ipc_channel::ipc::{IpcReceiver, TryRecvError};

use crate::{
    detours::{attach_detours, detach_detours},
    utils,
};

use super::{send_error, send_mix_format, send_success};

pub fn start_listener(rx: IpcReceiver<IpcMessage>) {
    message_listener(rx);
}

fn message_listener(rx: IpcReceiver<IpcMessage>) {
    let timeout = Duration::from_millis(1000);
    std::thread::spawn(move || {
        println!("Started listener");
        loop {
            match rx.try_recv_timeout(timeout) {
                Ok(msg) => match msg {
                    IpcMessage::Log(s) => println!("{}", s),
                    IpcMessage::Command(cmd) => match cmd {
                        PatchCommand::EnableDetour => enable_detour(),
                        PatchCommand::DisableDetour => disable_detour(),
                        PatchCommand::GetMixFormat => get_mix_format(),
                        PatchCommand::Eject => break,
                    },
                    _ => (),
                },
                Err(e) => match e {
                    TryRecvError::IpcError(e) => match e {
                        ipc_channel::ipc::IpcError::Bincode(e) => {
                            send_error(PatchError::Bincode(e.to_string())).ok();
                        }
                        ipc_channel::ipc::IpcError::Io(e) => {
                            send_error(PatchError::Io(e.to_string())).ok();
                        }
                        ipc_channel::ipc::IpcError::Disconnected => break,
                    },
                    TryRecvError::Empty => (),
                },
            }
        }
        println!("stopped listener");
        unsafe { detach_detours().ok() };
        drop_endpoint();
        send_success().ok();
    });
}

fn enable_detour() {
    match unsafe { attach_detours() } {
        Ok(_) => {
            send_success().ok();
        }
        Err(e) => {
            send_error(PatchError::Detour(e.to_string())).ok();
        }
    };
}

fn disable_detour() {
    match unsafe { detach_detours() } {
        Ok(_) => {
            send_success().ok();
        }
        Err(e) => {
            send_error(PatchError::Detour(e.to_string())).ok();
        }
    };
}

fn get_mix_format() {
    match utils::get_mix_format() {
        Ok(fmt) => {
            send_mix_format(fmt).ok();
        }
        Err(e) => {
            send_error(PatchError::Detour(e.to_string())).ok();
        }
    };
}
