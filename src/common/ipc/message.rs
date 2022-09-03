use ipc_channel::ipc::{IpcBytesSender, IpcSender};
use serde::{Deserialize, Serialize};

use crate::common::{audio::MixFormat, utils::error::PatchError};

#[derive(Serialize, Deserialize, Debug)]
pub enum PatchCommand {
    EnableDetour,
    DisableDetour,
    GetMixFormat,
    Eject,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcMessage {
    Log(String),
    Sender(IpcSender<IpcMessage>),
    Receiver(IpcSender<IpcMessage>),
    ByteSender(IpcBytesSender),
    MixFormat(MixFormat),
    Command(PatchCommand),
    Success,
    Error(PatchError),
}
