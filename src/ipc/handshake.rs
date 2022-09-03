use crate::common::{
    ipc::IpcMessage,
    utils::error::{IpcError, PatchError},
};
use dll_syringe::rpc::RemotePayloadProcedure;
use ipc_channel::ipc::{self, IpcBytesReceiver, IpcOneShotServer, IpcReceiver, IpcSender};

pub fn connect_ipc(
    handshake_proc: RemotePayloadProcedure<fn(String) -> Result<(), PatchError>>,
) -> Result<
    (
        IpcSender<IpcMessage>,
        IpcReceiver<IpcMessage>,
        IpcBytesReceiver,
    ),
    IpcError,
> {
    let (server, server_name) = IpcOneShotServer::<IpcSender<IpcMessage>>::new()
        .map_err(|e| ipc_channel::ipc::IpcError::Io(e))?;

    handshake_proc.call(&server_name).unwrap()?; //Infallable, this unwrap should never panic.

    let (_, tx1): (_, IpcSender<IpcMessage>) = server.accept().unwrap();

    let (tx2, rx2) = ipc::channel::<IpcMessage>().unwrap();
    let (txb, rxb) = ipc::bytes_channel().unwrap();

    tx1.send(IpcMessage::Sender(tx2)).unwrap();
    tx1.send(IpcMessage::ByteSender(txb)).unwrap();

    let msg = rx2.recv().unwrap();

    match msg {
        IpcMessage::Success => Ok((tx1, rx2, rxb)),
        IpcMessage::Error(e) => Err(e.into()),
        _ => Err(IpcError::HandshakeFailed.into()),
    }
}
