use std::{
    error::Error,
    io::Error as IoError,
    sync::{Arc, Mutex},
};

use crate::common::{ipc::IpcMessage, utils::error::PatchError};
use ipc_channel::ipc::{IpcBytesSender, IpcReceiver, IpcSender};
use once_cell::sync::OnceCell;

pub static MESSAGE_SENDER: OnceCell<Arc<Mutex<Option<IpcSender<IpcMessage>>>>> = OnceCell::new();

pub fn initialize_message_channel(sender: IpcSender<IpcMessage>) -> Result<(), PatchError> {
    match MESSAGE_SENDER.get() {
        Some(arc) => {
            if let Ok(mut opt) = arc.lock() {
                let _ = opt.insert(sender);
            }
        }
        None => {
            return MESSAGE_SENDER
                .set(Arc::new(Mutex::new(Some(sender))))
                .map_err(|_| {
                    PatchError::ChannelUnavailable("failed to set MESSAGE_SENDER".to_string())
                });
        }
    };

    Ok(())
}

pub fn drop_message_channel() -> Result<(), PatchError> {
    if let Ok(mut opt) = get_message_sender().lock() {
        let _ = opt.take();
    }

    Ok(())
}

pub fn get_message_sender() -> Arc<Mutex<Option<IpcSender<IpcMessage>>>> {
    let arc = MESSAGE_SENDER
        .get()
        .expect("MESSAGE_SENDER is not initialized");
    arc.clone()
}
