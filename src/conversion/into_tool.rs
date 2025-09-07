use std::sync::Arc;

use rmcp::{model::{JsonObject, Tool, ToolAnnotations}, serde_json::{Map, Value}};

use crate::trino_client::{DataType, QueryTemplate};

impl Into<Tool> for QueryTemplate {
    fn into(self) -> Tool {
        let mut properties = Map::new();

        for (name, param) in self.input_parameters {
            let mut prop = Map::new();
            prop.insert(
                "type".to_string(),
                Value::String(match param.data_type {
                    DataType::String => "string",
                    DataType::Integer => "integer",
                    DataType::Float => "number",
                    DataType::Boolean => "boolean",
                    DataType::Date => "string",
                    DataType::DateTime => "string",
                }.to_string()),
            );

            prop.insert("description".to_string(), Value::String(param.description));

            properties.insert(name.clone(), Value::Object(prop));
        }

        let mut input_schema = JsonObject::new();
        input_schema.insert("type".to_string(), Value::String("object".to_string()));
        input_schema.insert("properties".to_string(), Value::Object(properties));

        let mut output_properties = Map::new();
        for (column_name, col) in self.output_columns {
            let mut prop = Map::new();
            prop.insert("type".to_string(), Value::String("string".to_string()));
            prop.insert("description".to_string(), Value::String(col.description.clone()));
            output_properties.insert(column_name, Value::Object(prop));
        }

        let tool = Tool::new(
            self.name,
            self.description,
            Arc::new(input_schema),
        )
        .annotate(
            ToolAnnotations::new()
                .read_only(true)
                .idempotent(true)
                .open_world(true),
        );

        tool
    }
}
