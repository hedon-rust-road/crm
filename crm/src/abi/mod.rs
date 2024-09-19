pub mod auth;

use std::sync::Arc;

use chrono::{Duration, Utc};
use crm_metadata::pb::{Content, MaterializeRequest};
use crm_send::pb::SendRequest;
use futures::StreamExt;
use rand::Rng;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::{info, warn};
use user_stat::pb::QueryRequest;

use crate::{
    pb::{
        RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
        WelcomeResponse,
    },
    CrmService,
};

impl CrmService {
    pub async fn welcome(&self, req: WelcomeRequest) -> Result<Response<WelcomeResponse>, Status> {
        let request_id = req.id;
        let d1 = Utc::now() - Duration::days(req.interval as _);
        let d2 = d1 + Duration::days(1);
        let query = QueryRequest::new_with_dt("created_at", d1, d2);
        let mut res_user_stats = self.user_stats.clone().query(query).await?.into_inner();

        let contents = self
            .metadata
            .clone()
            .materialize(MaterializeRequest::new_with_ids(&req.content_ids))
            .await?
            .into_inner();
        let contents: Vec<Content> = contents
            .filter_map(|v| async move { v.ok() })
            .collect()
            .await;
        let contents = Arc::new(contents);

        let (tx, rx) = mpsc::channel(1024);
        let sender = self.config.server.sender_email.clone();
        tokio::spawn(async move {
            while let Some(Ok(user)) = res_user_stats.next().await {
                info!("Sending welcome message to user: {:?}", user.email);
                let contents = contents.clone();
                let sender = sender.clone();
                let tx = tx.clone();

                let req = SendRequest::new("Welcome".to_string(), sender, &[user.email], &contents);
                if let Err(e) = tx.send(req).await {
                    warn!("Failed to send welcome message: {:?}", e);
                }
            }
        });

        let reqs = ReceiverStream::new(rx);

        // NOTE: this is an alternative solution
        // let sender = self.config.server.sender_email.clone();
        // let reqs = res.filter_map(move |v| {
        //     let sender: String = sender.clone();
        //     let contents = contents.clone();
        //     async move {
        //         let v = v.ok()?;
        //         Some(gen_send_req("Welcome".to_string(), sender, v, &contents))
        //     }
        // });

        self.notification.clone().send(reqs).await?;

        Ok(Response::new(WelcomeResponse { id: request_id }))
    }

    pub async fn remind(&self, req: RemindRequest) -> Result<Response<RemindResponse>, Status> {
        let request_id = req.id;
        let d1 = Utc::now() - Duration::days(req.last_visit_interval as _);
        let d2 = d1 + Duration::days(1);
        let query = QueryRequest::new_with_dt("last_watched_at", d1, d2);
        let mut res_user_stats = self.user_stats.clone().query(query).await?.into_inner();

        let contents = self
            .metadata
            .clone()
            .materialize(MaterializeRequest::new_with_ids(
                &dummy_provide_some_contents().to_vec(),
            ))
            .await?
            .into_inner();

        let contents: Vec<Content> = contents
            .filter_map(|v| async move { v.ok() })
            .collect()
            .await;
        let contents = Arc::new(contents);

        let (tx, rx) = mpsc::channel(1024);
        let sender = self.config.server.sender_email.clone();

        tokio::spawn(async move {
            while let Some(Ok(user)) = res_user_stats.next().await {
                info!("Sending remind message to user: {:?}", user.email);
                let contents = contents.clone();
                let sender = sender.clone();
                let tx = tx.clone();

                let req = SendRequest::new("Remind".to_string(), sender, &[user.email], &contents);
                if let Err(e) = tx.send(req).await {
                    warn!("Failed to send remind message: {:?}", e);
                }
            }
        });

        let reqs: ReceiverStream<SendRequest> = ReceiverStream::new(rx);
        self.notification.clone().send(reqs).await?;

        Ok(Response::new(RemindResponse { id: request_id }))
    }

    pub async fn recall(&self, req: RecallRequest) -> Result<Response<RecallResponse>, Status> {
        let request_id = req.id;
        let d1 = Utc::now() - Duration::days(req.last_visit_interval as _);
        let d2 = d1 + Duration::days(1);
        let query = QueryRequest::new_with_dt("last_watched_at", d1, d2);
        let mut res_user_stats = self.user_stats.clone().query(query).await?.into_inner();

        let contents = self
            .metadata
            .clone()
            .materialize(MaterializeRequest::new_with_ids(&req.content_ids))
            .await?
            .into_inner();

        let contents: Vec<Content> = contents
            .filter_map(|v| async move { v.ok() })
            .collect()
            .await;
        let contents = Arc::new(contents);

        let (tx, rx) = mpsc::channel(1024);
        let sender = self.config.server.sender_email.clone();

        tokio::spawn(async move {
            while let Some(Ok(user)) = res_user_stats.next().await {
                info!("Sending recall message to user: {:?}", user.email);
                let contents = contents.clone();
                let sender = sender.clone();
                let tx = tx.clone();

                let req = SendRequest::new("Recall".to_string(), sender, &[user.email], &contents);
                if let Err(e) = tx.send(req).await {
                    warn!("Failed to send recall message: {:?}", e);
                }
            }
        });

        let reqs = ReceiverStream::new(rx);
        self.notification.clone().send(reqs).await?;
        Ok(Response::new(RecallResponse { id: request_id }))
    }
}

fn dummy_provide_some_contents() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..10).map(|_| rng.gen_range(0..100)).collect()
}
