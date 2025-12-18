use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::interactive::colors::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub file_path: Option<String>,
    pub sink_path: Option<String>,
    pub kafka_broker: String,
    pub kafka_group: String,
    pub postgres_conn: Option<String>,
    pub batch_size: Option<usize>,
    pub deduplicate: bool,
    pub throttle_per_sec: Option<u64>,
    #[serde(default)]
    pub custom_vars: HashMap<String, String>,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            file_path: None,
            sink_path: None,
            kafka_broker: "localhost:9092".to_string(),
            kafka_group: "fluxmux-interactive".to_string(),
            postgres_conn: None,
            custom_vars: HashMap::new(),
            batch_size: None,  // Changed from Some(100) to None - don't add Batcher by default
            deduplicate: false,
            throttle_per_sec: None,
        }
    }
}

impl SessionState {
    pub fn get_state_file() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".fluxmux");
        fs::create_dir_all(&path).ok();
        path.push("session.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::get_state_file();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str::<Self>(&content) {
                        Ok(state) => {
                            println!("{}", info("Session loaded"));
                            return state;
                        }
                        Err(_) => {
                            println!("{}", warning("Could not deserialize session, using defaults"));
                        }
                    }
                }
                Err(_) => {
                    println!("{}", warning("Could not read session file, using defaults"));
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::get_state_file();
        match serde_json::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(&path, content) {
                    eprintln!("{}", error(&format!("Failed to save session: {}", e)));
                }
            }
            Err(e) => {
                eprintln!("{}", error(&format!("Failed to serialize session: {}", e)));
            }
        }
    }

    pub fn set_file_path(&mut self, path: String) {
        self.file_path = Some(path.clone());
        println!("{}", success(&format!("File path set to: {}", path)));
    }

    pub fn set_sink_path(&mut self, path: String) {
        self.sink_path = Some(path.clone());
        println!("{}", success(&format!("Sink path set to: {}", path)));
    }

    pub fn set_kafka_broker(&mut self, broker: String) {
        self.kafka_broker = broker.clone();
        println!("{}", success(&format!("Kafka broker set to: {}", broker)));
    }

    pub fn set_kafka_group(&mut self, group: String) {
        self.kafka_group = group.clone();
        println!("{}", success(&format!("Kafka group set to: {}", group)));
    }

    pub fn set_postgres_conn(&mut self, conn: String) {
        self.postgres_conn = Some(conn.clone());
        println!("{}", success(&format!("Postgres connection set to: {}", conn)));
    }

    pub fn set_batch_size(&mut self, size: usize) {
        self.batch_size = Some(size);
        println!("{}", success(&format!("Batch size set to: {}", size)));
    }

    #[allow(dead_code)]
    pub fn toggle_deduplicate(&mut self) {
        self.deduplicate = !self.deduplicate;
        println!("{}", success(&format!("Deduplicate: {}", self.deduplicate)));
    }

    pub fn set_throttle(&mut self, per_sec: u64) {
        self.throttle_per_sec = Some(per_sec);
        println!("{}", success(&format!("Throttle set to: {} per sec", per_sec)));
    }

    pub fn set_custom_var(&mut self, key: String, value: String) {
        self.custom_vars.insert(key, value);
    }

    pub fn get_custom_var(&self, key: &str) -> Option<String> {
        self.custom_vars.get(key).cloned()
    }

    pub fn substitute_vars(&self, input: &str) -> String {
        let mut result = input.to_string();
        
        // Substitute predefined variables first
        if let Some(file) = &self.file_path {
            result = result.replace("${file}", file);
            result = result.replace("$file", file);
        }
        if let Some(sink) = &self.sink_path {
            result = result.replace("${sink}", sink);
            result = result.replace("$sink", sink);
        }
        result = result.replace("${kafka-broker}", &self.kafka_broker);
        result = result.replace("$kafka-broker", &self.kafka_broker);
        result = result.replace("${kafka-group}", &self.kafka_group);
        result = result.replace("$kafka-group", &self.kafka_group);
        if let Some(postgres) = &self.postgres_conn {
            result = result.replace("${postgres}", postgres);
            result = result.replace("$postgres", postgres);
        }
        result = result.replace("${batch-size}", &self.batch_size.unwrap_or(0).to_string());
        result = result.replace("$batch-size", &self.batch_size.unwrap_or(0).to_string());
        result = result.replace("${deduplicate}", &self.deduplicate.to_string());
        result = result.replace("$deduplicate", &self.deduplicate.to_string());
        result = result.replace("${throttle}", &self.throttle_per_sec.unwrap_or(0).to_string());
        result = result.replace("$throttle", &self.throttle_per_sec.unwrap_or(0).to_string());
        
        // Then substitute custom variables
        for (key, value) in &self.custom_vars {
            result = result.replace(&format!("${{{}}}", key), value);
            result = result.replace(&format!("${}", key), value);
        }
        result
    }

    pub fn display(&self) -> String {
        let mut output = format!(
            "{}\n{}",
            header("Session Configuration"),
            table_row("file", &self.file_path.as_ref().map(|s| s.as_str()).unwrap_or("<not set>"))
        );
        output.push('\n');
        output.push_str(&table_row("sink", &self.sink_path.as_ref().map(|s| s.as_str()).unwrap_or("<not set>")));
        output.push('\n');
        output.push_str(&table_row("kafka-broker", &self.kafka_broker));
        output.push('\n');
        output.push_str(&table_row("kafka-group", &self.kafka_group));
        output.push('\n');
        output.push_str(&table_row("postgres", &self.postgres_conn.as_ref().map(|s| s.as_str()).unwrap_or("<not set>")));
        output.push('\n');
        output.push_str(&table_row("batch-size", &format!("{}", self.batch_size.unwrap_or(0))));
        output.push('\n');
        output.push_str(&table_row("deduplicate", &self.deduplicate.to_string()));
        output.push('\n');
        output.push_str(&table_row("throttle", &format!("{}", self.throttle_per_sec.unwrap_or(0))));
        
        if !self.custom_vars.is_empty() {
            output.push('\n');
            output.push_str(&header("Custom Variables"));
            for (key, value) in &self.custom_vars {
                output.push('\n');
                output.push_str(&table_row(key, value));
            }
        }
        
        output
    }
}