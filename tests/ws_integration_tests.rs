use tokio::sync::{mpsc, broadcast};
use std::time::Duration;
use url::Url;
use ntex::server::Server as NtexServer; // For type hint
use std::net::TcpStream; // For a simple connection attempt check
use tracing; // For tracing::info!
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};

// Assuming your crate name is router_bot
use router_bot::{processor::ActionTuple, event::BaseEvent, ws, processor};

// Helper to spawn the server in the background for testing
async fn spawn_test_server() -> (String, u16, tokio::task::JoinHandle<()>, broadcast::Sender<BaseEvent>) {
    let host = "127.0.0.1".to_string();
    let port = portpicker::pick_unused_port().expect("No ports free");
    // The server_address for connect_async should not have /ws, that's part of the URL path
    let server_base_address = format!("ws://{}:{}", host, port); 

    let (action_tx, action_rx) = mpsc::channel::<ActionTuple>(10);
    let (event_tx, _) = broadcast::channel::<BaseEvent>(10);
    let event_tx_clone_for_server = event_tx.clone();

    let server_handle = tokio::spawn(async move {
        // Spawn the message processor
        tokio::spawn(processor::message_processor_task(action_rx));
        
        // Start the WebSocket server
        if let Err(e) = ws::start_ws_server(host, port, action_tx, event_tx_clone_for_server).await {
            // Use eprintln! for test output, or tracing::error! if tracing is set up for tests
            eprintln!("Test server failed to start or encountered an error: {:?}", e);
        }
    });
    
    // Give the server a moment to start
    tokio::time::sleep(Duration::from_millis(200)).await; // Increased slightly just in case
    
    (server_base_address, port, server_handle, event_tx)
}

#[tokio::test]
async fn test_websocket_connect_and_ping() {
    let (server_base_url, _port, server_handle, _event_tx) = spawn_test_server().await;
    // Construct the full URL including the /ws path for the specific WebSocket endpoint
    let url = Url::parse(&format!("{}/ws", server_base_url)).expect("Failed to parse test server URL with /ws path");

    let connect_attempt = connect_async(url.clone()).await;
    assert!(connect_attempt.is_ok(), "Failed to connect to WebSocket server: {:?}", connect_attempt.err());
    
    let (mut ws_stream, _) = connect_attempt.unwrap();
    
    // Send a Ping
    ws_stream.send(Message::Ping(vec![1, 2, 3])).await.expect("Failed to send ping");
    
    // Expect a Pong back
    let pong_msg_result = tokio::time::timeout(Duration::from_secs(2), ws_stream.next()).await; // Increased timeout slightly
    assert!(pong_msg_result.is_ok(), "Timeout: Did not receive response to ping in time");
    
    let pong_msg = pong_msg_result.unwrap(); // Get the result from timeout
    assert!(pong_msg.is_some(), "WebSocket stream closed before receiving pong (message was None)");

    match pong_msg.unwrap() { // Unwrap the Option<Result<Message, Error>>
        Ok(Message::Pong(data)) => {
            assert_eq!(data, vec![1, 2, 3], "Pong data did not match ping data");
        }
        Ok(other) => {
            panic!("Expected Pong, got {:?}", other);
        }
        Err(e) => {
            panic!("Error receiving pong: {:?}", e);
        }
    }
    
    // Close the connection
    ws_stream.close(None).await.expect("Failed to close WebSocket stream");
    
    // Abort the server task to clean up resources.
    // This is important for ensuring tests don't interfere with each other
    // if they run in parallel or if ports need to be reused quickly.
    server_handle.abort(); 
}

#[tokio::test]
async fn test_server_graceful_stop_mechanism() {
    let host = "127.0.0.1".to_string();
    let port = portpicker::pick_unused_port().expect("No ports free for stop test");
    let server_addr_string = format!("{}:{}", host, port);
    let server_ws_url = format!("ws://{}/ws", server_addr_string);

    let (action_tx, action_rx) = mpsc::channel::<ActionTuple>(10);
    let (event_tx, _) = broadcast::channel::<BaseEvent>(10);
    
    // Spawn the message processor task
    let _processor_handle = tokio::spawn(router_bot::processor::message_processor_task(action_rx));

    // Configure and get the server instance
    let server_handle: NtexServer = router_bot::ws::start_ws_server(
        host.clone(),
        port,
        action_tx.clone(),
        event_tx.clone()
    ).expect("Failed to configure server for stop test");

    // The server returned by .run() is already running in the background.
    // We don't need to spawn it again.
    
    // Give server a moment to be fully up
    tokio::time::sleep(Duration::from_millis(100)).await;

    // 1. Check server is up by trying to connect a WebSocket client
    let url = Url::parse(&server_ws_url).unwrap();
    assert!(connect_async(url.clone()).await.is_ok(), "Server should be connectable initially");
    
    // 2. Stop the server using the mechanism ServerGuard would use
    tracing::info!("[Test] Attempting to stop server via handle...");
    server_handle.stop(true).await; // This is async, so await it
    tracing::info!("[Test] Server stop command issued and awaited.");

    // Give a moment for ports to be released, though stop(true) should be graceful
    tokio::time::sleep(Duration::from_millis(200)).await;

    // 3. Check server is down
    // WebSocket connect_async might hang or timeout. A quicker check is a raw TCP connection.
    match TcpStream::connect(server_addr_string.clone()) {
        Ok(_) => panic!("Server should be down and not accept TCP connections on {}", server_addr_string),
        Err(e) => {
            tracing::info!("[Test] TCP connection failed as expected after stop: {}", e);
            // This is the expected outcome. Error kind might be ConnectionRefused.
            assert_eq!(e.kind(), std::io::ErrorKind::ConnectionRefused, "Expected ConnectionRefused");
        }
    }

    // Also check WebSocket connection fails
    match tokio::time::timeout(Duration::from_secs(2), connect_async(url)).await {
        Ok(Ok(_)) => panic!("WebSocket connection should fail after server stop"),
        Ok(Err(_e)) => {
            tracing::info!("[Test] WebSocket connection failed as expected: {:?}", _e);
            // Expected
        }
        Err(_e)) => {
            tracing::info!("[Test] WebSocket connection timed out as expected: {:?}", _e);
            // Also expected if it just hangs
        }
    }
    
    // _processor_handle.abort(); // Optional: clean up processor task if needed
}
