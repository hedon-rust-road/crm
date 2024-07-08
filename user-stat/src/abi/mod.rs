use crate::{
    pb::{QueryRequest, RawQueryRequest},
    ResponseStream, ServiceResult, UserStatsService,
};

#[allow(unused)]
impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        todo!()
    }

    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        todo!()
    }
}
