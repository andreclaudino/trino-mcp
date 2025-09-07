use trino_mcp::{mcp_handlers::trino_handler::TrinoHandler, persistence::load_tools_from_dir, server::server::start_server, trino_client::TrinoClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let host = "0.0.0.0";
    let port = 5050;
    let tools_directory = "./resources/queries";
    let server_name = "trino-mcp";
    
    let trino_host = "trino";
    let trino_port = 8080;
    let trino_username = "trino-mcp-user";
    
    let trino_client = TrinoClient::new(trino_host, trino_port, &trino_username)?;
    let query_templates = load_tools_from_dir(tools_directory)?;
    let trino_handler = TrinoHandler::new(server_name, trino_client, query_templates)?;
    
    start_server(host, port, trino_handler).await?;

    Ok(())
}
