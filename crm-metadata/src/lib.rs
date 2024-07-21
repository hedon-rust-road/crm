mod abi;
pub mod config;
pub mod pb;

pub use abi::Tpl;
pub use config::AppConfig;

use futures::Stream;
use pb::{
    metadata_server::{Metadata, MetadataServer},
    Content, MaterializeRequest,
};
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;

#[allow(unused)]
pub struct MetadataService {
    config: AppConfig,
}

#[tonic::async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        req: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        let query = req.into_inner();
        self.materialize(query).await
    }
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        MetadataService { config }
    }

    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}
