use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

pub trait Service<
    In: Serialize + DeserializeOwned + Send + 'static,
    Out: Serialize + DeserializeOwned + Send + 'static,
> {
    fn respond(&self, i: In) -> impl std::future::Future<Output = Result<Out>> + Send;
}

#[cfg(test)]
mod test {
    use std::{net::Ipv4Addr, time::Duration};

    use hreads::log_spawn;
    use test_log::test;
    use tokio::time::sleep;

    use super::*;
    use crate::{Client, Server};

    #[derive(Clone)]
    struct IsEvenService;

    impl Service<i32, bool> for IsEvenService {
        fn respond(&self, i: i32) -> impl Future<Output = Result<bool>> + Send {
            async move { Ok(i % 2 == 0) }
        }
    }

    #[test(tokio::test)]
    async fn test_service() -> Result<()> {
        let server = Server::start(65238).await?;

        log_spawn(async move {
            server.serve(IsEvenService).await?;
            Ok(())
        });

        sleep(Duration::from_secs_f32(0.1)).await;

        let client = Client::<bool, i32>::connect((Ipv4Addr::LOCALHOST, 65238)).await?;

        client.send(5).await?;
        assert_eq!(false, client.receive().await?);

        client.send(6).await?;
        assert_eq!(true, client.receive().await?);

        Ok(())
    }
}
