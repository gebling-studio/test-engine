use std::{
    any::type_name,
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use anyhow::Result;
use hreads::log_spawn;
use log::{debug, error, trace};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    net::{TcpListener, TcpStream},
    select, spawn,
    sync::{
        Mutex,
        mpsc::{Receiver, Sender, channel},
    },
};
use tokio_util::sync::CancellationToken;

use crate::{Service, System, connection::Client};

pub struct Server<In, Out> {
    cancel:    CancellationToken,
    connected: Mutex<Receiver<Client<In, Out>>>,
    port:      u16,
    pub id:    String,
    _p:        PhantomData<Mutex<(In, Out)>>,
}

impl<In: Serialize + DeserializeOwned + Send + 'static, Out: Serialize + DeserializeOwned + Send + 'static>
    Server<In, Out>
{
    pub async fn start(port: u16) -> Result<Self> {
        let id = System::generate_app_instance_id();
        let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)).await?;

        let cancel = CancellationToken::new();

        let cn = cancel.clone();

        let (s, r) = channel(1);

        spawn(async move {
            loop {
                select! {
                    () = cn.cancelled() => {
                        debug!("Stopping server listening on: {port}");
                        break;
                    }
                    connection = listener.accept() => {
                        match connection {
                            Ok((stream, _)) => Self::add_connection(&s, stream).await,
                            Err(err) => error!("Failed to accept connection: {err}"),
                        }
                    }
                }
            }
        });

        Ok(Self {
            cancel,
            connected: Mutex::new(r),
            port,
            id,
            _p: PhantomData,
        })
    }

    pub async fn serve(&self, service: impl Service<In, Out> + Clone + Send + 'static) -> Result<()> {
        loop {
            let connection = self.wait_for_new_connection().await;

            let ser = service.clone();
            log_spawn::<anyhow::Error>(async move {
                loop {
                    let msg = connection.receive().await?;
                    match ser.respond(msg).await {
                        Ok(response) => {
                            connection.send(response).await?;
                        }
                        Err(err) => {
                            error!("Server failed to respond: {err}");
                        }
                    }
                }
            });
        }
    }

    pub async fn wait_for_new_connection(&self) -> Client<In, Out> {
        self.connected.lock().await.recv().await.expect("Dropped server")
    }

    async fn add_connection(new_connection: &Sender<Client<In, Out>>, stream: TcpStream) {
        trace!("New connection");

        let connection = Client::from_stream(stream);

        if let Err(err) = new_connection.send(connection).await {
            error!("Failed to send connection signal: {err}");
        }
    }
}

impl<In, Out> Drop for Server<In, Out> {
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<In, Out> std::fmt::Debug for Server<In, Out> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = type_name::<In>();
        let o = type_name::<Out>();

        f.debug_struct(&format!("Server<{i}, {o}>")).field("port", &self.port).finish()
    }
}
