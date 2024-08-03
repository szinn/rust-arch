use std::{net::SocketAddr, sync::Arc, time::Duration};

use arch_core::ArchService;

use axum::{
    extract::{Request, State},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::{body::Incoming, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle};
use tower::Service;
use tower_http::timeout::TimeoutLayer;

use crate::ApiError;

pub async fn start_server(port: u16, arch_service: Arc<ArchService>, subsys: SubsystemHandle) -> Result<(), ApiError> {
    tracing::trace!("Starting http service");

    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().map_err(|_| ApiError::BadPort(port))?;
    let listener = TcpListener::bind(addr).await.unwrap();

    let routes = get_routes(arch_service.clone());

    tracing::info!("Listening on port {}", port);
    loop {
        let (socket, remote_addr) = tokio::select! {
            _ = subsys.on_shutdown_requested() => {
                break;
            }

            result = listener.accept() => {
                result.unwrap()
            }
        };

        tracing::debug!("connection {} accepted", remote_addr);
        let tower_service = routes.clone();
        let name = format!("handler-{remote_addr}");
        subsys.start(SubsystemBuilder::new(name, move |h| handler(socket, remote_addr, tower_service, h)));
    }

    Ok(())
}

fn get_routes(arch_service: Arc<ArchService>) -> Router<()> {
    axum::Router::new()
        .route("/health", get(health))
        .with_state(arch_service)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

async fn handler(socket: TcpStream, remote_addr: SocketAddr, tower_service: Router<()>, subsys: SubsystemHandle) -> Result<(), ApiError> {
    let socket = TokioIo::new(socket);
    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| tower_service.clone().call(request));
    let conn = hyper::server::conn::http1::Builder::new().serve_connection(socket, hyper_service);
    let mut conn = std::pin::pin!(conn);

    tokio::select! {
        result = conn.as_mut() => {
            if let Err(err) = result {
                tracing::warn!("Failed to serve connection{}: {:#}", remote_addr, err);
            }
        }

        _ = subsys.on_shutdown_requested() => {
            tracing::debug!("signal received, starting graceful shutdown");
        }
    }

    tracing::debug!("Connection {} closed", remote_addr);
    Ok(())
}

#[tracing::instrument(level = "trace", skip(arch_service))]
async fn health(State(arch_service): State<Arc<ArchService>>) -> impl IntoResponse {
    match arch_service.health_service.is_healthy().await {
        true => StatusCode::OK,
        false => StatusCode::BAD_REQUEST,
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self)).into_response()
    }
}
