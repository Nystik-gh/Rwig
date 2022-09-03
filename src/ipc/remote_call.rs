use std::time::Duration;

use crate::common::{
    audio::MixFormat,
    ipc::{IpcMessage, PatchCommand},
    utils::error::{IpcError, PatchError},
};
use dll_syringe::rpc::RemotePayloadProcedure;
use ipc_channel::ipc::{self, IpcBytesReceiver, IpcOneShotServer, IpcReceiver, IpcSender};

pub fn remote_enable_detour(
    tx_msg: &IpcSender<IpcMessage>,
    rx_msg: &IpcReceiver<IpcMessage>,
) -> Result<(), IpcError> {
    tx_msg
        .send(IpcMessage::Command(PatchCommand::EnableDetour))
        .map_err(|e| IpcError::SendFailed)?;

    let response = rx_msg
        .try_recv_timeout(Duration::from_millis(500))
        .map_err(|e| match e {
            ipc::TryRecvError::IpcError(e) => e.into(),
            ipc::TryRecvError::Empty => IpcError::SendFailed,
        })?;

    match response {
        IpcMessage::Success => Ok(()),
        _ => Err(IpcError::SendFailed),
    }
}

pub fn remote_disable_detour(
    tx_msg: &IpcSender<IpcMessage>,
    rx_msg: &IpcReceiver<IpcMessage>,
) -> Result<(), IpcError> {
    tx_msg
        .send(IpcMessage::Command(PatchCommand::DisableDetour))
        .map_err(|e| IpcError::SendFailed)?;

    let response = rx_msg
        .try_recv_timeout(Duration::from_millis(500))
        .map_err(|e| match e {
            ipc::TryRecvError::IpcError(e) => e.into(),
            ipc::TryRecvError::Empty => IpcError::SendFailed,
        })?;

    match response {
        IpcMessage::Success => Ok(()),
        _ => Err(IpcError::SendFailed),
    }
}

pub fn remote_get_mix_format(
    tx_msg: &IpcSender<IpcMessage>,
    rx_msg: &IpcReceiver<IpcMessage>,
) -> Result<MixFormat, IpcError> {
    tx_msg
        .send(IpcMessage::Command(PatchCommand::GetMixFormat))
        .map_err(|e| IpcError::SendFailed)?;

    let response = rx_msg
        .try_recv_timeout(Duration::from_millis(500))
        .map_err(|e| match e {
            ipc::TryRecvError::IpcError(e) => e.into(),
            ipc::TryRecvError::Empty => IpcError::SendFailed,
        })?;

    match response {
        IpcMessage::MixFormat(fmt) => {
            println!("mix format: {:?}", fmt);
            Ok(fmt)
        }
        _ => Err(IpcError::SendFailed),
    }
}

pub fn remote_eject(
    tx_msg: &IpcSender<IpcMessage>,
    rx_msg: &IpcReceiver<IpcMessage>,
) -> Result<(), IpcError> {
    tx_msg
        .send(IpcMessage::Command(PatchCommand::Eject))
        .map_err(|e| IpcError::SendFailed)?;

    let response = rx_msg
        .try_recv_timeout(Duration::from_millis(500))
        .map_err(|e| match e {
            ipc::TryRecvError::IpcError(e) => e.into(),
            ipc::TryRecvError::Empty => IpcError::SendFailed,
        })?;

    match response {
        IpcMessage::Success => Ok(()),
        _ => Err(IpcError::SendFailed),
    }
}
