use tokio::sync::{mpsc, broadcast};
use ntex::server::Server as NtexServer; // Alias for ntex Server

// ServerGuard struct for graceful shutdown
#[derive(Debug)]
struct ServerGuard {
    server_handle: Option<NtexServer>,
}

impl ServerGuard {
    fn new(server_handle: NtexServer) -> Self {
        Self { server_handle: Some(server_handle) }
    }
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        tracing::info!("ServerGuard is being dropped. Initiating server shutdown...");
        if let Some(server) = self.server_handle.take() {
            tracing::info!("Attempting graceful shutdown of the server...");
            match tokio::runtime::Handle::try_current() {
                Ok(handle) => {
                    handle.block_on(async {
                        server.stop(true).await; // server.stop() returns a future
                        tracing::info!("Server shutdown sequence completed.");
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to get Tokio runtime handle in ServerGuard::drop: {:?}. Server might not shut down gracefully.", e);
                }
            }
        } else {
            tracing::warn!("ServerGuard dropped, but no server handle was present.");
        }
    }
}


const CHANNEL_BUFFER_SIZE: usize = 100;

#[ntex::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting bot application...");

    let (action_tx, action_rx) = 
        mpsc::channel::<router_bot::processor::ActionTuple>(CHANNEL_BUFFER_SIZE);
    let (event_tx, _event_rx_main_unused) = 
        broadcast::channel::<router_bot::event::BaseEvent>(CHANNEL_BUFFER_SIZE);

    tracing::info!("Spawning message processor task...");
    tokio::spawn(router_bot::processor::message_processor_task(action_rx));

    let host = "127.0.0.1".to_string();
    let port = 8080;

    tracing::info!("Configuring WebSocket server on {}:{}...", host, port);
    
    // start_ws_server is now synchronous and returns a Result<NtexServer, Error>
    match router_bot::ws::start_ws_server(host.clone(), port, action_tx.clone(), event_tx.clone()) {
        Ok(server_instance) => {
            let _server_guard = ServerGuard::new(server_instance); // Guard will handle shutdown on drop
            tracing::info!("WebSocket server started successfully on {}:{}. Application will run until interrupted.", host, port);
            
            // Keep the main task alive until Ctrl-C or other signal
            // The server itself runs in the background on ntex's runtime threads.
            // The _server_guard will ensure shutdown when main exits.
            if let Err(e) = tokio::signal::ctrl_c().await {
                tracing::error!("Failed to listen for ctrl_c signal: {:?}", e)
            }
            tracing::info!("Ctrl-C received, initiating shutdown via ServerGuard drop.");
            // ServerGuard drops here, triggering server.stop()
        }
        Err(e) => {
            tracing::error!("Failed to start WebSocket server: {:?}", e);
        }
    }
    
    tracing::info!("Bot application finished.");
}
