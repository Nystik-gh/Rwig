use std::{
    io::Read,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

use ipc_channel::ipc::{IpcBytesReceiver, IpcError, IpcSender};

use std::io::Error as IoError;

pub struct CaptureStream {
    consumer_handle: Option<JoinHandle<()>>,
    consumer_channel: Sender<u32>,
}

impl CaptureStream {
    pub fn from_receiver(receiver: IpcBytesReceiver) -> CaptureStream {
        let (tx, rx) = mpsc::channel();
        let consumer_handle = Some(CaptureStream::consumer(receiver, rx));

        CaptureStream {
            consumer_handle,
            consumer_channel: tx,
        }
    }

    fn consumer(receiver: IpcBytesReceiver, rx: Receiver<u32>) -> JoinHandle<()> {
        let handle = std::thread::spawn(move || {
            loop {
                let signal = rx.try_recv();

                match signal {
                    Ok(stop) => break,
                    Err(e) => match e {
                        mpsc::TryRecvError::Empty => (),
                        mpsc::TryRecvError::Disconnected => break,
                    },
                }

                let res = receiver.try_recv();

                match res {
                    Ok(data) => {
                        println!("received {} bytes", data.len());
                    }
                    Err(err) => match err {
                        ipc_channel::ipc::TryRecvError::IpcError(ipc_err) => match ipc_err {
                            IpcError::Bincode(_) => {
                                println!("bincode error");
                            }
                            IpcError::Io(_) => {
                                println!("io error");
                            }
                            IpcError::Disconnected => {
                                println!("disconnected");
                                break;
                            }
                        },
                        ipc_channel::ipc::TryRecvError::Empty => (),
                    },
                }
            }
            println!("closing stream consumer");
        });
        handle
    }
}

impl Drop for CaptureStream {
    fn drop(&mut self) {
        self.consumer_channel.send(1).ok();

        let handle = self.consumer_handle.take().unwrap();

        println!("wait for consumer to stop");
        handle
            .join()
            .expect("failed to stop capture consumer thread");
    }
}

/*impl Read for CaptureStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let frames = match self.receiver.try_recv() {
            Ok(res) => res,
            Err(e) => match e {
                ipc_channel::ipc::TryRecvError::IpcError(err) => match err {
                    ipc_channel::ipc::IpcError::Bincode(_) => {
                        return Err(IoError::new(std::io::ErrorKind::InvalidData, "bad data"))
                    }
                    ipc_channel::ipc::IpcError::Io(io_err) => return Err(io_err),
                    ipc_channel::ipc::IpcError::Disconnected => {
                        return Err(IoError::new(
                            std::io::ErrorKind::BrokenPipe,
                            "channel disconnected",
                        ))
                    }
                },
                ipc_channel::ipc::TryRecvError::Empty => {
                    return Err(IoError::new(std::io::ErrorKind::Interrupted, "no data"))
                }
            },
        };

        let f_len = frames.len();

        let mut n = 0;
        for (i, b) in buf.iter_mut().enumerate() {
            if i < f_len {
                *b = frames[i];
                n += 1;
            } else {
                break;
            }
        }

        Ok(n)
    }
}*/

unsafe impl Send for CaptureStream {}
unsafe impl Sync for CaptureStream {}
