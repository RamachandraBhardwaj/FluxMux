use std::collections::HashMap;
use fluxmux_core::{
    message::Message,
    traits::Sink,
};
use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};
use serde_json::Value;
use anyhow::Context;

pub struct PostgresSink {
    client: Option<Client>,
    table: String,
    connection_string: String,
    required_schema: Option<HashMap<String, String>>, // column_name -> data_type
}

impl PostgresSink {
    pub fn new(
        connection_string: String, 
        table: String, 
        schema: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            client: None,
            table,
            connection_string,
            required_schema: schema,
        }
    }

    async fn get_or_connect_client(&mut self) -> anyhow::Result<()> {
        if self.client.is_none() {
            // Connect and validate table/schema if needed
            let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls)
                .await
                .context("Failed to connect to PostgreSQL")?;

            // The connection object performs the actual communication with the database,
            // so spawn it off to run on its own
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Database connection error: {}", e);
                }
            });

            // If schema validation is requested, verify table schema
            if let Some(schema) = &self.required_schema {
                self.validate_table_schema(&client, schema).await?;
            }

            self.client = Some(client);
        }

        Ok(())
    }

    async fn validate_table_schema(&self, client: &Client, required: &HashMap<String, String>) -> anyhow::Result<()> {
        // Query table information from information_schema
        let rows = client
            .query(
                "SELECT column_name, data_type FROM information_schema.columns 
                 WHERE table_name = $1",
                &[&self.table],
            )
            .await
            .context("Failed to query table schema")?;

        // Build actual schema map
        let mut actual_schema: HashMap<String, String> = HashMap::new();
        for row in rows {
            let column_name: String = row.get("column_name");
            let data_type: String = row.get("data_type");
            actual_schema.insert(column_name, data_type.to_lowercase());
        }

        // Validate required columns exist with correct types
        for (col, required_type) in required {
            match actual_schema.get(col) {
                Some(actual_type) => {
                    if !Self::types_are_compatible(actual_type, required_type) {
                        return Err(anyhow::anyhow!(
                            "Column '{}' has type '{}' but required type is '{}'",
                            col, actual_type, required_type
                        ));
                    }
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "Required column '{}' of type '{}' not found in table",
                        col, required_type
                    ));
                }
            }
        }

        Ok(())
    }

    fn types_are_compatible(actual: &str, required: &str) -> bool {
        // Normalize types for comparison
        let actual = actual.to_lowercase();
        let required = required.to_lowercase();

        // Direct match
        if actual == required {
            return true;
        }

        // Handle type aliases and compatible types
        match (actual.as_str(), required.as_str()) {
            // Numeric types
            ("integer", "int4") | ("int4", "integer") => true,
            ("bigint", "int8") | ("int8", "bigint") => true,
            ("real", "float4") | ("float4", "real") => true,
            ("double precision", "float8") | ("float8", "double precision") => true,
            // Text types
            ("varchar", "text") | ("text", "varchar") => true,
            ("char", "text") | ("text", "char") => true,
            // JSON types
            ("jsonb", "json") | ("json", "jsonb") => true,
            _ => false,
        }
    }

    async fn insert_message(&mut self, msg: &Message) -> anyhow::Result<()> {
        // Parse message payload as JSON if not already parsed
        let data = if let Some(parsed) = &msg.parsed {
            parsed.clone()
        } else {
            serde_json::from_slice(&msg.payload)
                .context("Failed to parse message payload as JSON")?
        };

        // Handle nested JSON objects
        match data {
            Value::Object(map) => {
                // Build the SQL statement dynamically based on the data
                let columns: Vec<String> = map.keys().cloned().collect();
                let values: Vec<&Value> = columns.iter().map(|k| &map[k]).collect();
                
                let placeholders: Vec<String> = (1..=columns.len())
                    .map(|i| format!("${}", i))
                    .collect();

                let query = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    self.table,
                    columns.join(", "),
                    placeholders.join(", ")
                );

                // Convert serde_json::Value to tokio_postgres::types::ToSql
                let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = values
                    .iter()
                    .map(|&v| v as &(dyn tokio_postgres::types::ToSql + Sync))
                    .collect();

                self.client.as_ref().unwrap()
                    .execute(&query, &params)
                    .await
                    .context("Failed to insert row")?;

                Ok(())
            }
            _ => Err(anyhow::anyhow!("Message payload must be a JSON object"))
        }
    }
}

#[async_trait]
impl Sink for PostgresSink {
    async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        self.get_or_connect_client().await?;
        self.insert_message(&msg).await
    }

    async fn flush(&mut self) -> anyhow::Result<()> {
        // PostgreSQL automatically flushes after each transaction
        Ok(())
    }
}
