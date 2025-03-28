use axum::{
    Router,
    extract::{State, ws},
    routing::get,
};
use futures::{SinkExt, StreamExt};
use skeever::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), SkeeverError> {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "api=debug,skeever=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Wrap our dependencies into the app state so routes can access them
    let app_state = AppState::init().await?;

    // Spawn a separate task that receives events from the event stream and forwards them to websockets
    let event_sender = app_state.event_sender.clone();
    let event_stream = app_state
        .get_event_stream()
        .await
        .expect("Could not create connection to event stream");
    tokio::spawn(async move {
        websockets::forward_events_to_websockets(event_stream.clone(), event_sender)
            .await
            .expect("Failed to forward events to websockets")
    });

    // Initialize an axum server
    let router = Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_handler))
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    tracing::info!("Starting server on http://localhost:8000");
    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

/// Handles the root route ("/")
async fn index_handler() -> &'static str {
    "squeak"
}

/// Returns OK for health checks
async fn health_handler() -> &'static str {
    "OK"
}

/// Handles incoming websocket requests
async fn ws_handler(
    ws: axum::extract::WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handles individual websocket connections
async fn handle_socket(socket: ws::WebSocket, state: AppState) {
    let mut event_receiver = state.event_sender.subscribe();
    let (sender, mut receiver) = socket.split();

    // Send historical messages first and get the sender back
    let mut sender = match websockets::send_historical_messages(
        state
            .get_event_stream()
            .await
            .expect("Could not create connection to event stream"),
        sender,
    )
    .await
    {
        Ok(sender) => sender,
        Err(e) => {
            tracing::error!("Failed to send historical messages: {:?}", e);
            return;
        }
    };

    // Handle incoming messages in a separate task
    let receiver_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                ws::Message::Text(text) => {
                    tracing::debug!("Received text message: {}", text);
                    // Handle any client messages if needed
                }
                ws::Message::Close(_) => break,
                _ => continue,
            }
        }
    });

    // Forward events to websocket
    let sender_task = tokio::spawn(async move {
        while let Ok(squeak) = event_receiver.recv().await {
            let Ok(serialized_squeak) = serde_json::to_string(&squeak) else {
                tracing::error!("Failed to serialize squeak from {:?}", &squeak.author.name);
                tracing::debug!("Failed Squeak: {:?}", &squeak);
                continue;
            };
            if sender
                .send(ws::Message::Text(serialized_squeak.into()))
                .await
                .is_err()
            {
                break;
            }
            tracing::debug!("Successfully squeaked {:?}", &squeak);
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = receiver_task => {},
        _ = sender_task => {},
    }

    tracing::debug!("Websocket connection closed");
}
