use std::{
    error::Error,
    io::Error as IoError,
    sync::{Arc, Mutex},
};

use crate::common::{ipc::IpcMessage, utils::error::PatchError};
use ipc_channel::ipc::{IpcBytesSender, IpcReceiver, IpcSender};
use once_cell::sync::OnceCell;

//pub static MESSAGE_RECEIVER: OnceCell<Arc<Mutex<IpcReceiver<IpcMessage>>>> = OnceCell::new();
pub static MESSAGE_SENDER: OnceCell<Arc<Mutex<Option<IpcSender<IpcMessage>>>>> = OnceCell::new();
//pub static BYTE_SENDER: OnceCell<Arc<Mutex<Option<IpcBytesSender>>>> = OnceCell::new();

pub fn initialize_channels(
    //receiver: IpcReceiver<IpcMessage>,
    sender: IpcSender<IpcMessage>,
    //byte_sender: IpcBytesSender,
) -> Result<(), PatchError> {
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

    /*match BYTE_SENDER.get() {
        Some(arc) => {
            if let Ok(mut opt) = arc.lock() {
                let _ = opt.insert(byte_sender);
            }
        }
        None => {
            return BYTE_SENDER
                .set(Arc::new(Mutex::new(Some(byte_sender))))
                .map_err(|_| {
                    PatchError::ChannelUnavailable("failed to set MESSAGE_SENDER".to_string())
                });
        }
    };*/

    Ok(())
}

pub fn drop_channels() -> Result<(), PatchError> {
    if let Ok(mut opt) = get_message_sender().lock() {
        let _ = opt.take();
    }

    /*if let Ok(mut opt) = get_byte_sender().lock() {
        let _ = opt.take();
    }*/

    Ok(())
}

/*pub fn get_message_receiver() -> Arc<Mutex<IpcReceiver<IpcMessage>>> {
    let arc = MESSAGE_RECEIVER
        .get()
        .expect("MESSAGE_RECEIVER is not initialized");
    arc.clone()
}*/

pub fn get_message_sender() -> Arc<Mutex<Option<IpcSender<IpcMessage>>>> {
    let arc = MESSAGE_SENDER
        .get()
        .expect("MESSAGE_SENDER is not initialized");
    arc.clone()
}

/*pub fn get_byte_sender() -> Arc<Mutex<Option<IpcBytesSender>>> {
    let arc = BYTE_SENDER.get().expect("BYTE_SENDER is not initialized");
    arc.clone()
}*/
