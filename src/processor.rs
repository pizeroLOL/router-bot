use tokio::sync::mpsc;
use crate::action::ApiRequest;
use crate::{MsgRsp, Status}; // Corrected path for MsgRsp and Status
use serde_json::json; // For creating dummy data and extracting echo

// This type alias will simplify function signatures
pub type ActionTuple = (ApiRequest, Option<mpsc::Sender<MsgRsp<serde_json::Value>>>);

pub async fn message_processor_task(
    mut main_rx: mpsc::Receiver<ActionTuple>,
) {
    tracing::info!("Message processor task started.");
    while let Some((api_request, response_tx_opt)) = main_rx.recv().await {
        tracing::info!("Processor received ApiRequest: {:?}", api_request.action);

        let echo = api_request.params.get("echo").cloned();

        // For now, create a dummy success response
        // In a real bot, you'd match on api_request.action and do real work
        let response = MsgRsp {
            status: Status::Ok,
            retcode: 0,
            data: Some(json!({"message": "Action processed successfully"})),
            echo: echo, // Pass through the echo
        };

        if let Some(response_tx) = response_tx_opt {
            tracing::info!("Sending response for action: {}", api_request.action);
            if let Err(e) = response_tx.send(response).await {
                tracing::error!(
                    "Failed to send response for action {}: {:?}",
                    api_request.action,
                    e
                );
            }
        } else {
            tracing::info!("No response channel for action: {}", api_request.action);
        }
    }
    tracing::info!("Message processor task finished.");
}
