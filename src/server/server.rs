use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp_actix_web::{SseService, StreamableHttpService};

use crate::mcp_handlers::trino_handler::TrinoHandler;

pub async fn start_server(host: &str, port: u16, trino_handler: TrinoHandler) -> anyhow::Result<()> {

    let interface = format!("{host}:{port}");

    let trino_handler_for_http_stream = trino_handler.clone();

    let streamable_http_server = StreamableHttpService::builder()
        .service_factory(Arc::new(move || Ok(trino_handler_for_http_stream.clone())))
        .session_manager(Arc::new(LocalSessionManager::default()))
        .stateful_mode(true)
        .build();

    let trino_handler_for_sse = trino_handler.clone();
    let sse_service = SseService::builder()
        .service_factory(Arc::new(move || Ok(trino_handler_for_sse.clone())))
        .sse_path("/events".to_string())
        .post_path("/messages".to_string())
        .build();

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/mcp/v1/trino/sse")
                    .service(sse_service.clone().scope())
            )
            .service(
                web::scope("/mcp/v1/trino/http-stream")
                    .service(
                        streamable_http_server.clone().scope()
                    )
            )
    })
    .bind(interface)?
    .run()
    .await?;

    Ok(())
}