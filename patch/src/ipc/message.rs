use crate::common::{
    audio::MixFormat,
    ipc::IpcMessage,
    utils::error::{IpcError, PatchError},
};

use super::channels::get_message_sender;

pub fn send_log(msg: String) -> Result<(), IpcError> {
    if let Ok(sender) = get_message_sender().lock() {
        sender
            .as_ref()
            .unwrap()
            .send(IpcMessage::Log(msg))
            .map_err(|_| IpcError::SendFailed)?;
    }

    Ok(())
}

pub fn send_success() -> Result<(), IpcError> {
    if let Ok(sender) = get_message_sender().lock() {
        sender
            .as_ref()
            .unwrap()
            .send(IpcMessage::Success)
            .map_err(|_| IpcError::SendFailed)?;
    }

    Ok(())
}

pub fn send_mix_format(fmt: MixFormat) -> Result<(), IpcError> {
    if let Ok(sender) = get_message_sender().lock() {
        sender
            .as_ref()
            .unwrap()
            .send(IpcMessage::MixFormat(fmt))
            .map_err(|_| IpcError::SendFailed)?;
    }

    Ok(())
}

pub fn send_error(err: PatchError) -> Result<(), IpcError> {
    if let Ok(sender) = get_message_sender().lock() {
        sender
            .as_ref()
            .unwrap()
            .send(IpcMessage::Error(err))
            .map_err(|_| IpcError::SendFailed)?;
    }

    Ok(())
}
