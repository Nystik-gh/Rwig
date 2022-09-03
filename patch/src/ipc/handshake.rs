use std::sync::{Arc, Mutex};

use crate::{
    audio::endpoint::create_endpoint,
    com::com_initialized,
    common::{
        ipc::IpcMessage,
        utils::error::{PatchError, PatchError::HandshakeFailed},
    },
    utils::get_mix_format,
};
use ipc_channel::ipc::{self, IpcBytesSender, IpcReceiver, IpcSender};

use crate::{
    detours::init_detours,
    ipc::{channels::initialize_channels, listener::start_listener},
};

dll_syringe::payload_procedure! {
    fn handshake(server_name: String) -> Result<(), PatchError> {
        unsafe { match init_detours() {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }; }

        let tx0 = IpcSender::connect(server_name).map_err(|e| HandshakeFailed(format!("Failed to connect to server: {}", e.to_string())))?;

        let (tx1, rx1): (IpcSender<IpcMessage>, IpcReceiver<IpcMessage>) = ipc::channel().map_err(|e| HandshakeFailed(format!("Failed to create channel: {}", e.to_string())))?;

        // transfer sender to injector
        tx0.send(tx1).map_err(|e| HandshakeFailed(format!("Failed to transfer sender to injector: {}", e.to_string())))?;

        std::thread::spawn(move || {
          if let Ok(msg) = rx1.recv() && let IpcMessage::Sender(tx2) = msg {
            if let Ok(msg) = rx1.recv() && let IpcMessage::ByteSender(txb) = msg {
              match initialize_channels(tx2.clone()) {
                Ok(_) => {
                  start_listener(rx1);
                  tx2.send(IpcMessage::Success).ok();
                  let format = match get_mix_format() {
                      Ok(f) => f,
                      Err(e) => panic!("get mix format error, {:?}", e),
                  };
                  create_endpoint(txb, format).ok();
                },
                Err(e) => {
                  println!("Error: {:?}", e);
                  tx2.send(IpcMessage::Error(HandshakeFailed(e.to_string()))).ok();
                }
              }
            }
          }
          else {
            println!("Failed to receive sender from injector");
          }
        });

        Ok(())
    }
}
