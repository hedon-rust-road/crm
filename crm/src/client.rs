use crm::pb::{
    crm_client::CrmClient, RecallRequestBuilder, RemindRequestBuilder, WelcomeRequestBuilder,
};
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pem = include_str!("../../fixtures/rootCA.pem");
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(pem))
        .domain_name("localhost");

    let channel = Channel::from_static("https://[::1]:50000")
        .tls_config(tls)?
        .connect()
        .await?;

    // let mut client = CrmClient::new(channel);

    // let mut client = CrmClient::connect("https://[::1]:50000").await?;
    /*
    Error: status: Unknown, message: "h2 protocol error: http2 error: connection error detected: frame with invalid size", details: [], metadata: MetadataMap { headers: {} }

    Caused by:
        0: transport error
        1: http2 error: connection error detected: frame with invalid size
        2: connection error detected: frame with invalid size
     */

    let token = include_str!("../../fixtures/token").trim();
    let token: MetadataValue<_> = format!("Bearer {}", token).parse()?;
    let mut client = CrmClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let req = WelcomeRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .interval(100u32)
        .content_ids([1u32, 2, 3])
        .build()?;

    let response = client.welcome(Request::new(req)).await?.into_inner();
    println!("Welcome Response: {:?}", response);

    let req = RemindRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .last_visit_interval(100u32)
        .build()?;
    let response = client.remind(Request::new(req)).await?.into_inner();
    println!("Remind Response: {:?}", response);

    let req = RecallRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .last_visit_interval(100u32)
        .content_ids([1u32, 2, 3])
        .build()?;
    let response = client.recall(Request::new(req)).await?.into_inner();
    println!("Recall Response: {:?}", response);

    Ok(())
}
