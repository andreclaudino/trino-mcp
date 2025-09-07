use std::{sync::Arc};

use rmcp::{
    handler::server::tool::{ToolRoute, ToolRouter},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool_handler, tool_router, ServerHandler
};
use crate::{constants::SERVER_VERSION, conversion::try_into_tool_route::QueryTemplateCaller, trino_client::{QueryTemplate, TrinoClient}};


#[derive(Clone)]
pub struct TrinoHandler {
    server_name: String,
    trino_client: Arc<TrinoClient>,
    tool_router: ToolRouter<Self>,
}


#[tool_router]
impl TrinoHandler {
    pub fn new(server_name: &str, trino_client: TrinoClient, query_templates: Vec<QueryTemplate>) -> anyhow::Result<Self> {

        let mut tool_router = Self::tool_router();
        let trino_client_arc = Arc::new(trino_client);

        for query_template in query_templates.into_iter() {
            let query_name = query_template.name.clone();

            match query_template.try_into() {
                Ok(tool) => {
                    let query_template_caller = QueryTemplateCaller::new(trino_client_arc.clone(), query_template);
                    let router = ToolRoute::new_dyn(query_template, query_template_caller.call_handler);
                    tool_router.add_route(tool);
                    log::info!("Added tool {} to router", query_name);
                },
                Err(error) => {
                    log::error!("Error converting query {} into MCP tool: {:?}", query_name, error);
                    continue;
                }
            }
        }

        let handler = Self {
            server_name: server_name.to_owned(),
            trino_client: trino_client_arc.clone(),
            tool_router
        };

        Ok(handler)
    }

    
    
}


#[tool_handler]
impl ServerHandler for TrinoHandler {
    
    fn get_info(&self) -> ServerInfo {
        let server_capabilities = 
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build();

        ServerInfo {
            instructions: Some("Execute dynamic queries on trino".into()),
            capabilities: server_capabilities,
            server_info: Implementation {
                name: self.server_name.clone(),
                version: SERVER_VERSION.to_owned(),
            },
            ..Default::default()
        }
    }
}
