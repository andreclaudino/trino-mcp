use std::fs;
use std::path::{Path, PathBuf};

use crate::trino_client::QueryTemplate;

pub fn load_tools_from_dir(dir_path: &str) -> anyhow::Result<Vec<QueryTemplate>> {
    let mut tools = Vec::new();
    let path = Path::new(dir_path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            
            let tool = load_tool_from_file(&path)?;
            tools.push(tool);
        }
    }

    Ok(tools)
}

fn load_tool_from_file(file_path: &PathBuf) -> anyhow::Result<QueryTemplate> {
    let content = fs::read_to_string(&file_path)?;
    let query_template: QueryTemplate = toml::from_str(&content)?;

    let tool = query_template.try_into()?;
    Ok(tool)
}