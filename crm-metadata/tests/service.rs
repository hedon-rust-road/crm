use std::{net::SocketAddr, time::Duration};

use crm_metadata::{
    config::AppConfig,
    pb::{metadata_client::MetadataClient, MaterializeRequest},
    MetadataService,
};
use futures::StreamExt;
use tokio::time::sleep;
use tonic::{transport::Server, Request};

#[tokio::test]
async fn test_metadata() -> anyhow::Result<()> {
    let addr = start_server().await?;
    let mut client = MetadataClient::connect(format!("http://{addr}")).await?;
    let stream = tokio_stream::iter(vec![
        MaterializeRequest { id: 1 },
        MaterializeRequest { id: 2 },
        MaterializeRequest { id: 3 },
    ]);
    let req = Request::new(stream);
    let resp = client.materialize(req).await?.into_inner();
    let ret: Vec<_> = resp.then(|res| async { res.unwrap() }).collect().await;
    assert_eq!(ret.len(), 3);
    Ok(())
}

async fn start_server() -> anyhow::Result<SocketAddr> {
    let config = AppConfig::load()?;
    let addr = format!("[::1]:{}", config.server.port).parse()?;

    let svc = MetadataService::new(config).into_server();
    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_micros(1)).await;
    Ok(addr)
}
