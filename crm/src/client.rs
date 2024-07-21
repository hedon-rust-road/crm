use crm::pb::{crm_client::CrmClient, WelcomeRequestBuilder};
use tonic::Request;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // connect gRPC by nginx proxy
    let mut client = CrmClient::connect("http://127.0.0.1:8080").await?;

    let req = WelcomeRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .interval(100u32)
        .content_ids([1u32, 2, 3])
        .build()?;

    let response = client.welcome(Request::new(req)).await?.into_inner();
    println!("Response: {:?}", response);
    Ok(())
}
