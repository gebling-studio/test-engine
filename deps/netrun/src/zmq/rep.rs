use std::any::type_name;

use anyhow::{Result, anyhow};
use hreads::spawn;
use log::error;
use serde::{Serialize, de::DeserializeOwned};
use zeromq::{RepSocket, Socket, SocketRecv, SocketSend};

use crate::{
    Function,
    serde::{deserialize, serialize},
};

pub struct Rep<In: DeserializeOwned + 'static, Out: Serialize + 'static> {
    receive: Function<In, Out>,
}

impl<In: DeserializeOwned + 'static, Out: Serialize + 'static> Rep<In, Out> {
    pub async fn new(endpoint: &str) -> Result<Self> {
        let mut socket = RepSocket::new();
        socket.bind(endpoint).await?;

        let function = Function::default();
        let receive = function.clone();

        spawn(async move {
            loop {
                let result: Result<()> = async {
                    let data: Vec<u8> = socket.recv().await?.try_into().map_err(|err| anyhow!("{err}"))?;

                    let input: In = deserialize(&data)?;
                    let output = function.call(input);
                    let out_data = serialize(output)?;

                    socket.send(out_data.into()).await?;

                    Ok(())
                }
                .await;

                _ = result.inspect_err(|err| error!("{}: {err}", type_name::<Self>()));
            }
        });

        Ok(Self { receive })
    }

    pub fn on_receive(&self, action: impl FnMut(In) -> Out + Send + 'static) {
        self.receive.replace(action);
    }
}

#[cfg(test)]
mod test {

    use std::net::{IpAddr, Ipv4Addr};

    use anyhow::Result;

    use crate::{
        scan_for_port,
        zmq::{Rep, Req},
    };

    #[tokio::test]
    async fn test_rep() -> Result<()> {
        let rep = Rep::<i32, i32>::new("tcp://0.0.0.0:6969").await?;

        rep.on_receive(|val| val * 2);

        let ports: Vec<_> = scan_for_port(6969).await?.into_iter().map(|(ip, _)| ip).collect();

        assert!(ports.contains(&IpAddr::V4(Ipv4Addr::LOCALHOST)));

        let req = Req::<i32, i32>::new("tcp://127.0.0.1:6969").await?;

        for i in 0..100 {
            assert_eq!(req.send(i).await?, i * 2);
        }

        drop(req);

        let req2 = Req::<i32, i32>::new(&format!("tcp://{}:6969", IpAddr::V4(Ipv4Addr::LOCALHOST))).await?;

        for i in 0..100 {
            assert_eq!(req2.send(i).await?, i * 2);
        }

        Ok(())
    }
}
