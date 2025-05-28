use ntex::{rt, web::{self, App, Error, HttpRequest, HttpResponse, middleware}, ws::{self, Message, ProtocolError}, server::Server}; // Added Server
use ntex::actor::{Actor, AsyncContext, StreamHandler};
use tokio::sync::{mpsc, broadcast};
use tracing::{info, error, debug};
use serde_json::json;

use crate::action::ApiRequest;
use crate::processor::ActionTuple;
use crate::{MsgRsp, Status};
use crate::event::BaseEvent;

// This enum is specific to the internal error flow of handle_valid_request
#[derive(Debug)]
enum ValidRequestInternalError {
    NoResponseFromProcessor,
    ResponseSerializationFailed(serde_json::Error),
}

// Define the WebSocket connection actor
struct WsConn {
    main_processor_tx: mpsc::Sender<ActionTuple>,
    event_rx: broadcast::Receiver<BaseEvent>, // New field for event receiver
}

impl WsConn {
    // Updated constructor
    fn new(processor_tx: mpsc::Sender<ActionTuple>, event_rx: broadcast::Receiver<BaseEvent>) -> Self {
        Self { main_processor_tx: processor_tx, event_rx }
    }
}

impl Actor for WsConn {
    type Context = ws::WsContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsConn actor started for: {:?}", ctx.address());
        
        // Task to handle incoming broadcast events
        let mut event_rx_clone = self.event_rx.resubscribe(); // Resubscribe to get a fresh receiver
        let writer = ctx.writer().clone(); // Get the writer for sending events

        ctx.spawn(async move {
            loop {
                match event_rx_clone.recv().await {
                    Ok(event_payload) => {
                        info!("WsConn received broadcast event: {:?}", event_payload.post_type);
                        match serde_json::to_string(&event_payload) {
                            Ok(json_event) => {
                                if writer.text(json_event).await.is_err() {
                                    error!("Failed to send event to WebSocket client. Client might have disconnected.");
                                    break; // Stop this event listening task
                                }
                            }
                            Err(e) => {
                                error!("Failed to serialize event to JSON: {:?}", e);
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        error!("Event broadcast channel lagged by {} messages for a WsConn.", n);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        info!("Event broadcast channel closed.");
                        break; // Stop this event listening task
                    }
                }
            }
            info!("Event listening task for WsConn stopped.");
        });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("WsConn actor stopped for: {:?}", ctx.address());
    }
}

// New static async method to process text messages
impl WsConn {
    async fn send_parse_error(text_payload: &str, writer: &ws::WsWriter) {
        error!("Failed to parse text message into ApiRequest: {} - full text will not be logged to avoid large payloads", text_payload.chars().take(200).collect::<String>());
        // Attempt to extract echo from the raw payload for error response
        let echo_value_raw: Option<serde_json::Value> = serde_json::from_str::<serde_json::Value>(text_payload)
            .ok()
            .and_then(|v| v.get("echo").cloned());

        let error_response = MsgRsp {
            status: Status::Failed,
            retcode: 1400, // OneBot error code for request format error
            data: Some(json!({"error": "Invalid request format"})),
            echo: echo_value_raw,
        };
        match serde_json::to_string(&error_response) {
            Ok(json_resp) => {
                if writer.text(json_resp).await.is_err() {
                     error!("Failed to send parsing error response to client.");
                }
            }
            Err(e) => {
                // This case is unlikely if MsgRsp serialization is correct
                error!("Critical: Failed to serialize error MsgRsp for parsing error: {:?}", e);
            }
        }
    }

    async fn process_received_text_message(
        text_payload: String,
        processor_tx: mpsc::Sender<ActionTuple>,
        writer: ws::WsWriter, // Note: writer is moved here now
    ) {
        info!("Processing text message (first 200 chars): {}", text_payload.chars().take(200).collect::<String>());
        match serde_json::from_str::<ApiRequest>(&text_payload) {
            Ok(api_request) => {
                // Call the new helper function, passing a reference to writer
                Self::handle_valid_request(api_request, processor_tx, &writer).await;
            }
            Err(_parse_err) => {
                Self::send_parse_error(&text_payload, &writer).await;
            }
        }
    }

    // New helper function for handling a valid request
    async fn handle_valid_request(
        api_request: ApiRequest,
        processor_tx: mpsc::Sender<ActionTuple>,
        writer: &ws::WsWriter,
    ) {
        debug!("Handling valid ApiRequest: {}", api_request.action);
        let echo_value = api_request.params.get("echo").cloned();

        let (response_tx_to_processor, mut response_rx_from_processor) =
            mpsc::channel::<MsgRsp<serde_json::Value>>(1);
        let action_tuple = (api_request, Some(response_tx_to_processor));

        // Part 1: Send to processor (remains similar, as it's already a flat early exit)
        if let Err(e) = processor_tx.send(action_tuple).await {
            error!("Failed to send ApiRequest to processor: {:?}", e);
            let err_resp = MsgRsp {
                status: Status::Failed,
                retcode: 1,
                data: Some(json!({"error": "Internal server error sending to processor"})),
                echo: echo_value,
            };
            if let Ok(json_err_resp) = serde_json::to_string(&err_resp) {
                writer.text(json_err_resp).await.ok();
            }
            return;
        }

        // Part 2: Receive from processor, serialize, and send. This is the refactored part.
        let processing_result: Result<String, ValidRequestInternalError> = 
            response_rx_from_processor.recv().await
                .ok_or(ValidRequestInternalError::NoResponseFromProcessor) // Convert Option to Result
                .and_then(|response_from_processor| { // If Ok(response), then try to serialize
                    serde_json::to_string(&response_from_processor)
                        .map_err(ValidRequestInternalError::ResponseSerializationFailed)
                });

        match processing_result {
            Ok(json_response) => {
                if writer.text(json_response).await.is_err() {
                    error!("Failed to send successful JSON response to WebSocket client for echo: {:?}", echo_value);
                }
            }
            Err(err_type) => {
                let (retcode, error_message_key) = match err_type {
                    ValidRequestInternalError::NoResponseFromProcessor => {
                        info!("Response channel closed by processor without a message for echo: {:?}", echo_value);
                        (2, "Processor did not respond")
                    }
                    ValidRequestInternalError::ResponseSerializationFailed(serde_err) => {
                        error!("Failed to serialize successful response from processor: {:?} for echo: {:?}", serde_err, echo_value);
                        (3, "Failed to serialize processor response")
                    }
                };
                
                let err_resp = MsgRsp {
                    status: Status::Failed,
                    retcode,
                    data: Some(json!({"error": error_message_key})),
                    echo: echo_value.clone(), // Ensure echo_value is cloned if used after this
                };
                if let Ok(json_err_resp) = serde_json::to_string(&err_resp) {
                    writer.text(json_err_resp).await.ok();
                }
            }
        }
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Ping(msg)) => {
                debug!("Received ping: {:?}", msg);
                ctx.ping(&msg); 
            }
            Ok(Message::Pong(msg)) => {
                debug!("Received pong: {:?}", msg);
            }
            Ok(Message::Text(text)) => {
                // text is ntex::util::ByteString
                let text_string = text.to_string(); 
                ctx.spawn(WsConn::process_received_text_message(
                    text_string,
                    self.main_processor_tx.clone(),
                    ctx.writer().clone(),
                ));
            }
            Ok(Message::Binary(bin)) => {
                info!("Received binary message (not processed): {:?}", bin.len());
                // It's good practice to inform the client if binary messages are not supported.
                // Since process_received_text_message now takes writer, we can't directly use ctx.text()
                // for this specific message type from here if we wanted to reuse process_received_text_message
                // for error signaling. However, this is a distinct case.
                // We can directly use ctx.text() here as handle() has access to ctx.
                ctx.text("Binary messages are not supported for actions.");
            }
            Ok(Message::Close(reason)) => {
                info!("WebSocket connection closed by client: {:?}", reason);
                ctx.close(reason);
                ctx.stop();
            }
            Err(e) => {
                error!("WebSocket stream error: {:?}", e);
                ctx.stop(); // Stop actor on stream error
            }
            _ => (), // Other message types like Continuation, Nop
        }
    }
}

// Updated ws_service_route function
async fn ws_service_route(
    req: HttpRequest,
    stream: web::types::Payload,
    main_processor_tx: web::types::Data<mpsc::Sender<ActionTuple>>,
    event_broadcast_tx: web::types::Data<broadcast::Sender<BaseEvent>>, // New parameter
) -> Result<HttpResponse, Error> {
    let p_tx = main_processor_tx.get_ref().clone();
    let e_rx = event_broadcast_tx.get_ref().subscribe(); // Subscribe to get a receiver
    info!("Starting WsConn actor for new connection {:?} with event receiver.", req.peer_addr());
    ws::start_actor(req, stream, move |_| WsConn::new(p_tx.clone(), e_rx))
}

// Updated start_ws_server function
pub fn start_ws_server( // No longer async
    host: String,
    port: u16,
    mpsc_sender: mpsc::Sender<ActionTuple>,
    event_broadcast_tx: broadcast::Sender<BaseEvent>,
) -> std::io::Result<Server> { // Return type changed
    info!("Configuring WebSocket server on {}:{}", host, port);
    
    let server = web::server(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::types::Data::new(mpsc_sender.clone()))
            .app_data(web::types::Data::new(event_broadcast_tx.clone()))
            .service(web::resource("/ws").to(ws_service_route))
    })
    .bind(format!("{}:{}", host, port))?
    .run(); // run() returns the Server instance, .await removed
    
    Ok(server) // Return the server instance
}
