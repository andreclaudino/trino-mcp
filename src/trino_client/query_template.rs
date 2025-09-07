use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Clone)]
#[serde(rename_all="lowercase")]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime
}

#[derive(Deserialize, Clone)]
pub struct QueryInputParameter {
    pub data_type: DataType,
    pub description: String,    
}

#[derive(Deserialize, Clone)]
pub struct QueryOutputColumn {
    pub data_type: DataType,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct QueryTemplate {
    pub name: String,
    pub description: String,
    pub input_parameters: HashMap<String, QueryInputParameter>,
    pub output_columns: HashMap<String, QueryOutputColumn>,

    pub query_template: String,
}


