use core::net::SocketAddr;
use std::{any::type_name, marker::PhantomData};

use anyhow::{Result, anyhow};
use log::{debug, error};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs, tcp::OwnedWriteHalf},
    select, spawn,
    sync::{
        Mutex, RwLock,
        mpsc::{Receiver, Sender, channel},
    },
};
use tokio_util::sync::CancellationToken;

use crate::{
    System,
    connection::BUFFER_SIZE,
    serde::{deserialize, serialize},
};

pub struct Client<In, Out> {
    write:    RwLock<OwnedWriteHalf>,
    receiver: Mutex<Receiver<Result<In>>>,
    cancel:   CancellationToken,
    address:  SocketAddr,
    id:       String,
    _p:       PhantomData<Mutex<Out>>,
}

impl<In: DeserializeOwned + Send + 'static, Out: Serialize> Client<In, Out> {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<Self> {
        Ok(Self::from_stream(TcpStream::connect(addr).await?))
    }

    pub fn from_stream(stream: TcpStream) -> Self {
        let addr = stream.local_addr().expect("Failed to get stream local_addr");
        let id = System::generate_app_instance_id();
        let cancel = CancellationToken::new();

        let address = stream.peer_addr().expect("No stream local addr");

        let (s, r) = channel(1);
        let (mut read, write) = stream.into_split();
        let cn = cancel.clone();
        let idd = id.clone();

        spawn(async move {
            let mut buf = vec![0u8; BUFFER_SIZE];

            loop {
                select! {
                    () = cn.cancelled() => {
                        debug!("Client dropped. Stop listening: {addr} - {idd}");
                        break
                    },
                    bytes = read.read(&mut buf) => handle_read(bytes, &buf, &s).await,
                }
            }
        });

        debug!("Connection: {id} created");

        Self {
            write: RwLock::new(write),
            receiver: Mutex::new(r),
            cancel,
            address,
            id,
            _p: PhantomData,
        }
    }

    pub async fn send(&self, val: impl Into<Out>) -> Result<()> {
        let val = val.into();
        let data = serialize(&val)?;

        self.write.write().await.write_all(&data).await?;

        Ok(())
    }

    pub async fn receive(&self) -> Result<In> {
        self.receiver
            .lock()
            .await
            .recv()
            .await
            .ok_or(anyhow!("Receiving from dropped connection"))?
    }

    pub async fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.write.read().await.local_addr()?)
    }

    pub async fn peer_addr(&self) -> Result<SocketAddr> {
        Ok(self.write.read().await.peer_addr()?)
    }
}

impl<In, Out> Drop for Client<In, Out> {
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<In, Out> std::fmt::Debug for Client<In, Out> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = type_name::<In>();
        let o = type_name::<Out>();

        f.debug_struct(&format!("Client<{i}, {o}>"))
            .field("id", &self.id)
            .field("address", &self.address)
            .finish()
    }
}

async fn handle_read<In: DeserializeOwned>(
    bytes: std::io::Result<usize>,
    buf: &[u8],
    sender: &Sender<Result<In>>,
) {
    let bytes = match bytes {
        Ok(b) => b,
        Err(err) => {
            error!("Failed to receive from client: {err}");
            _ = sender
                .send(Err(anyhow!("Failed to receive from client: {err}")))
                .await
                .inspect_err(|e| error!("Failed to send None from client: {e}"));
            return;
        }
    };

    if bytes == 0 {
        return;
    }

    match deserialize::<In>(&buf[..bytes]) {
        Ok(msg) => {
            _ = sender
                .send(Ok(msg))
                .await
                .inspect_err(|e| error!("Failed to send msg from client: {e}"));
        }
        Err(err) => {
            _ = sender
                .send(Err(anyhow!("Failed to deserialize from client: {err}")))
                .await
                .inspect_err(|e| error!("Failed to send None from client: {e}"));
        }
    }
}
