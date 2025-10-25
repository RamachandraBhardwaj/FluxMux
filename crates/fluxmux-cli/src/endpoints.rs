use std::str::FromStr;
use anyhow::anyhow;

// Endpoint types
#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    File { path: String },
    Kafka { brokers: String, topic: String, group_id: String },
    Stdin,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SinkType {
    File { path: String },
    Kafka { brokers: String, topic: String },
    Postgres { 
        connection_string: String, 
        table: String,
        schema: Option<std::collections::HashMap<String, String>>,
    },
    Stdout,
}

impl FromStr for SourceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" || s.eq_ignore_ascii_case("stdin") || s.eq_ignore_ascii_case("stdin:") {
            return Ok(SourceType::Stdin);
        }
        
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid source URI format. Expected: scheme:config"));
        }

        match parts[0] {
            "file" => Ok(SourceType::File {
                path: parts[1].to_string(),
            }),
            "kafka" => {
                let config = parse_kafka_uri(parts[1])?;
                Ok(SourceType::Kafka {
                    brokers: config.brokers,
                    topic: config.topic,
                    group_id: config.group_id.unwrap_or_else(|| "fluxmux-default".to_string()),
                })
            }
            _ => Err(anyhow!("Unsupported source type: {}", parts[0])),
        }
    }
}

impl FromStr for SinkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" || s.eq_ignore_ascii_case("stdout") || s.eq_ignore_ascii_case("stdout:") {
            return Ok(SinkType::Stdout);
        }
        
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid sink URI format. Expected: scheme:config"));
        }

        match parts[0] {
            "file" => Ok(SinkType::File {
                path: parts[1].to_string(),
            }),
            "kafka" => {
                let config = parse_kafka_uri(parts[1])?;
                Ok(SinkType::Kafka {
                    brokers: config.brokers,
                    topic: config.topic,
                })
            }
            "postgres" => {
                let (conn_str, query) = parts[1].split_once('?').unwrap_or((parts[1], ""));
                let mut table = None;
                let mut schema = None;

                if !query.is_empty() {
                    let params: std::collections::HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes())
                        .collect();
                    
                    table = params.get("table").map(|s| s.to_string());
                    
                    if let Some(schema_str) = params.get("schema") {
                        let mut schema_map = std::collections::HashMap::new();
                        for pair in schema_str.split(',') {
                            if let Some((col, typ)) = pair.split_once(':') {
                                schema_map.insert(col.to_string(), typ.to_string());
                            }
                        }
                        if !schema_map.is_empty() {
                            schema = Some(schema_map);
                        }
                    }
                }

                let table = table.ok_or_else(|| anyhow!("Missing table parameter"))?;

                Ok(SinkType::Postgres {
                    connection_string: conn_str.to_string(),
                    table,
                    schema,
                })
            }
            _ => Err(anyhow!("Unsupported sink type: {}", parts[0])),
        }
    }
}

// Helper struct for parsing Kafka URIs
#[derive(Debug)]
struct KafkaConfig {
    brokers: String,
    topic: String,
    group_id: Option<String>,
}

// Parse Kafka URI of form: kafka://host:port/topic?group=group_id
fn parse_kafka_uri(uri: &str) -> anyhow::Result<KafkaConfig> {
    let uri = uri.strip_prefix("//").unwrap_or(uri);
    let (host_path, query) = uri.split_once('?').unwrap_or((uri, ""));
    let (host, topic) = host_path
        .split_once('/')
        .ok_or_else(|| anyhow!("Invalid Kafka URI format. Expected: host:port/topic"))?;

    let group_id = if !query.is_empty() {
        let params: std::collections::HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes())
            .collect();
        params.get("group").map(|s| s.to_string())
    } else {
        None
    };

    Ok(KafkaConfig {
        brokers: host.to_string(),
        topic: topic.to_string(),
        group_id,
    })
}