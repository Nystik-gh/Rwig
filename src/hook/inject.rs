use dll_syringe::{process::OwnedProcess, rpc::RemotePayloadProcedure, Syringe};

use crate::common::utils::error::{InjectionError, PatchError};

pub fn inject_hook(
    pid: u32,
    payload_path: String,
) -> Result<RemotePayloadProcedure<fn(String) -> Result<(), PatchError>>, InjectionError> {
    let process = OwnedProcess::from_pid(pid)?;

    let syringe = Syringe::for_process(process);

    let pm = syringe.inject(payload_path)?;

    let start_handshake = unsafe {
        syringe
            .get_payload_procedure::<fn(String) -> Result<(), PatchError>>(pm, "handshake")?
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "handshake procedure not found",
            ))?
    };

    Ok(start_handshake)
}

/*
Ok(s) => {
  let (server, server_name) = IpcOneShotServer::<IpcSender<IpcMessage>>::new().unwrap();
            let (_, tx1): (_, IpcSender<IpcMessage>) = server.accept().unwrap();

            let (tx2, rx2) = ipc::channel::<IpcMessage>().unwrap();
            let (txb, rxb) = ipc::bytes_channel().unwrap();

            tx1.send(IpcMessage::Sender(tx2)).unwrap();
            tx1.send(IpcMessage::ByteSender(txb)).unwrap();
            let msg = rx2.recv().unwrap();
            match msg {
                IpcMessage::HandshakeSuccessful => println!("Handshake completed"),
                IpcMessage::Error(e) => println!("{:?}", e),
                _ => (),
            };

            std::thread::spawn(move || loop {
                let msg = rx2.recv().unwrap();
                match msg {
                    IpcMessage::Log(s) => println!("{}", s),
                    _ => (),
                }
            });
            Ok(pid)
        }
*/
