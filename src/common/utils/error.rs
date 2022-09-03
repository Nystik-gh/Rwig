use serde::{Deserialize, Serialize};

use super::PointerWidth;

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum PatchError {
    #[error("Handshake error: {:?}", self)]
    HandshakeFailed(String),
    #[error("Channel error: {}", self)]
    ChannelUnavailable(String),
    #[error("Bincode error: {}", self)]
    Bincode(String),
    #[error("Bincode error: {}", self)]
    Io(String),
    #[error("Detour error: {}", self)]
    Detour(String),
}

#[derive(thiserror::Error, Debug)]
pub enum OSError {
    #[error(transparent)]
    WindowsError(#[from] windows::core::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum InjectionError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("incompatible target process: {}", self)]
    UnsupportedTargetProcess {
        expected: PointerWidth,
        found: PointerWidth,
    },
    #[error("unable to inject hook into target process")]
    InjectFailed(#[from] dll_syringe::error::InjectError),
    #[error("unable to eject payload from target process")]
    EjectFailed(#[from] dll_syringe::error::EjectError),
    #[error("unable to load remote procedure")]
    LoadProcedureFailed(#[from] dll_syringe::error::LoadProcedureError),
}

#[derive(thiserror::Error, Debug)]
pub enum IpcError {
    #[error("Handshake failed")]
    HandshakeFailed,
    #[error(transparent)]
    Patch(#[from] PatchError),
    #[error(transparent)]
    IpcChannel(#[from] ipc_channel::ipc::IpcError),
    #[error("send failed")]
    SendFailed,
    #[error("bad response: {}", self)]
    BadResponse {
        expected: PointerWidth,
        found: PointerWidth,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum CaptureError {}

#[derive(thiserror::Error, Debug)]
pub enum RwigError {
    #[error(transparent)]
    OS(#[from] OSError),
    #[error(transparent)]
    Injector(#[from] InjectionError),
    #[error(transparent)]
    Ipc(#[from] IpcError),
    #[error(transparent)]
    Capture(#[from] CaptureError),
}
