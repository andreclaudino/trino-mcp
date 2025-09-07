use std::{collections::HashMap, sync::Arc};

use minijinja::Environment;
use rmcp::{handler::server::tool::CallToolHandler, model::{Annotated, CallToolResult, RawContent, RawTextContent}, serde_json};

use crate::{mcp_handlers::trino_handler::TrinoHandler, trino_client::{QueryTemplate, TrinoClient}};


pub struct QueryTemplateToolHandler<'source> {
    query_template: QueryTemplate,
    trino_client: Arc<TrinoClient>,
    mininja_env: Arc<Environment<'source>>
}

impl<'source> QueryTemplateToolHandler<'source> {
    pub fn new(
        query_template: QueryTemplate,
        trino_client: Arc<TrinoClient>,
        mininja_env: Arc<Environment<'source>>
    ) -> Self {
        Self {
            query_template,
            trino_client,
            mininja_env
        }
    }
}

impl<'source> CallToolHandler<TrinoHandler, HashMap<String, serde_json::Value>> for QueryTemplateToolHandler<'source> {
    fn call(
        self,
        tool_context: rmcp::handler::server::tool::ToolCallContext<'_, TrinoHandler>,
    ) -> futures_core::future::BoxFuture<'_, Result<CallToolResult, rmcp::ErrorData>> {
        let async_block = async move {
            let query_template_name = self.query_template.name;
            let query_template_source = self.query_template.query_template;
            
            let mut input_parameters = HashMap::new();
            
            for key in tool_context.request_context().meta.keys() {
                let maybe_value = tool_context.request_context().meta.get(key);

                match maybe_value {
                    Some(value) => {
                        input_parameters.insert(key.to_string(), serde_json::to_value(&value).unwrap());
                    }
                    None => continue,
                }
            }
        
        
            let minijinja_context = minijinja::Value::from_serialize(input_parameters);
            let rendered_query =
                self.mininja_env
                    .render_named_str(&query_template_name, &query_template_source, &minijinja_context).unwrap();

            let result = self.trino_client.run_query(&query_template_source).await.unwrap();
            let annoteted_content = Annotated{
                raw: RawContent::Text(RawTextContent { text: "resultado".to_string() }),
                annotations: None
            };
            Ok(CallToolResult::success(vec![annoteted_content]))
        };
        
        Box::pin(async_block.into_future())
    }
}