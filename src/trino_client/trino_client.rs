use prusto::{Client, ClientBuilder};
use rmcp::serde_json;


pub struct TrinoClient {
    client: Client
}


impl TrinoClient {
    pub fn new(host: &str, trino_port: u16, user: &str) -> anyhow::Result<Self> {
        let client =
            ClientBuilder::new(user, host)
                .port(trino_port)
                .build()?;

        let trino_client = Self {
            client,
        };

        Ok(trino_client)
    }

    pub async fn run_query(&self, sql: &str) -> anyhow::Result<TrinoBatch> {
        let query_result = self.client.get::<serde_json::Value>(sql.to_string()).await?;
        let batch = process_query_result(query_result);

        Ok(batch)
    }

    pub async fn get_next_batch(&self, trino_batch: &TrinoBatch) -> anyhow::Result<Option<TrinoBatch>> {
        if let Some(next_uri) = trino_batch.next_uri.clone() {
            let query_result = self.client.get_next(&next_uri).await?;

            let batch = process_query_result(query_result);

            Ok(Some(batch))
        } else {
            Ok(None)
        }
        
    }
}

fn process_query_result(query_result: prusto::QueryResult<serde_json::Value>) -> TrinoBatch {
    let data =
        if let Some(data_set) = query_result.data_set {
            data_set.into_vec()
        } else {
            Vec::new()
        };

    let batch = TrinoBatch {
        data,
        next_uri: query_result.next_uri
    };
    batch
}

pub struct TrinoBatch {
    pub data: Vec<serde_json::Value>,
    next_uri: Option<String>
}
