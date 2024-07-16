use tonic::Status;
use tracing::warn;

use crate::{
    abi::to_ts,
    pb::{send_request::Msg, EmailMessage, SendRequest, SendResponse},
    NotificationService,
};

use super::Sender;

impl Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            warn!("Failed to send email notification: {:?}", e);
            Status::internal("Failed to send email notification")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<EmailMessage> for Msg {
    fn from(value: EmailMessage) -> Self {
        Msg::Email(value)
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(value: EmailMessage) -> Self {
        let msg: Msg = value.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl EmailMessage {
    pub fn fake() -> Self {
        use fake::{faker::internet::en::SafeEmail, Fake};
        use uuid::Uuid;

        EmailMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            subject: "Email Subject".to_string(),
            body: "Email Body".to_string(),
        }
    }
}
