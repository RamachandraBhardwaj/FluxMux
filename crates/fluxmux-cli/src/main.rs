use fluxmux_core::engine::{MiddlewareConfig, build_middleware_chain, run_pipeline};
use fluxmux_core::pipe_actions::*;
use fluxmux_core::pipe_engine::run_pipe;
use serde_yaml;
use std::fs;
use std::str::FromStr;

mod conversions;
mod endpoints;
mod kafka_inspector;

use clap::{Parser, Subcommand};
use conversions::{Format, convert};
use endpoints::{SourceType, SinkType};
use fluxmux_connectors::{FileSource, KafkaSource, PipeSource};
use fluxmux_sinks::{FileSink, KafkaSink, PostgresSink, PipeSink};

#[derive(Parser)]
#[command(name = "fluxmux", about = "Universal CLI for File Conversion & Stream Inspection")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(long)]
    pub batch_size: Option<usize>,
    #[arg(long)]
    pub deduplicate: Option<bool>,
    #[arg(long)]
    pub throttle_per_sec: Option<u64>,
    #[arg(long)]
    pub config: Option<String>, // path to YAML config
}

#[derive(Subcommand)]
pub enum Commands {
    Convert {
        input: String,
        output: String,
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
    },
    Bridge {
        #[arg(long)]
        source: String,
        #[arg(long)]
        sink: String,
        #[arg(long)]
        batch_size: Option<usize>,
        #[arg(long)]
        batch_timeout_ms: Option<u64>,
        #[arg(long)]
        deduplicate: bool,
        #[arg(long)]
        throttle_per_sec: Option<u64>,
        #[arg(long)]
        retry_max_attempts: Option<u32>,
        #[arg(long)]
        retry_delay_ms: Option<u64>,
        #[arg(long)]
        schema_path: Option<String>,
        #[arg(long)]
        config: Option<String>,
    },
    Pipe {
        /// Source endpoint (file:path, kafka://host/topic, stdin, -)
        source: String,
        /// Pipeline actions and output destinations
        /// Example: filter 'temp>30' transform 'fahrenheit=temp*1.8+32' tee output.json kafka://localhost/hot
        args: Vec<String>,
    },
    Kafka {
        /// Topic name
        #[arg(long)]
        topic: String,
        /// Kafka broker address
        #[arg(long, default_value = "localhost:9092")]
        broker: String,
        /// Consumer group ID
        #[arg(long, default_value = "fluxmux-inspector")]
        group: String,
        /// Show first N messages (with EOF placeholders)
        #[arg(long, conflicts_with = "tail")]
        head: Option<usize>,
        /// Show latest N messages (live monitoring)
        #[arg(long, conflicts_with = "head")]
        tail: Option<usize>,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Helper: merge config from CLI and YAML for Bridge
    fn load_middleware_config_bridge(
        batch_size: Option<usize>,
        batch_timeout_ms: Option<u64>,
        deduplicate: bool,
        throttle_per_sec: Option<u64>,
        retry_max_attempts: Option<u32>,
        retry_delay_ms: Option<u64>,
        schema_path: Option<String>,
        config_path: &Option<String>,
    ) -> MiddlewareConfig {
        let mut config = if let Some(ref path) = config_path {
            let yaml = fs::read_to_string(path).expect("Failed to read config file");
            serde_yaml::from_str::<MiddlewareConfig>(&yaml).expect("Invalid YAML")
        } else {
            MiddlewareConfig::default()
        };
        // CLI args override YAML
        if let Some(bs) = batch_size {
            config.batch_size = Some(bs);
        }
        if let Some(bt) = batch_timeout_ms {
            config.batch_timeout_ms = Some(bt);
        }
        config.deduplicate = Some(deduplicate);
        if let Some(tp) = throttle_per_sec {
            config.throttle_per_sec = Some(tp);
        }
        if let Some(retry) = retry_max_attempts {
            config.retry_max_attempts = Some(retry);
        }
        if let Some(delay) = retry_delay_ms {
            config.retry_delay_ms = Some(delay);
        }
        if let Some(sp) = schema_path {
            config.schema_path = Some(sp);
        }
        config
    }

    // Helper: validate endpoints
    fn validate_endpoints(source: &SourceType, sink: &SinkType) -> Result<(), String> {
        // Prevent file-to-file transfers
        if matches!(source, SourceType::File { .. }) && matches!(sink, SinkType::File { .. }) {
            return Err("File-to-file transfers are not allowed. Use Kafka or Postgres as source or sink.".to_string());
        }
        Ok(())
    }

    match &cli.command {
        Commands::Convert { input, output, from, to } => {
            let from_fmt = Format::from_ext(&from).unwrap_or_else(|| {
                eprintln!("Unsupported input format: {from}");
                std::process::exit(1);
            });
            let to_fmt = Format::from_ext(&to).unwrap_or_else(|| {
                eprintln!("Unsupported output format: {to}");
                std::process::exit(1);
            });

            if let Err(e) = convert(&input, &output, from_fmt, to_fmt) {
                eprintln!("Conversion failed: {e}");
            } else {
                println!("✓ Converted {input} ({from}) → {output} ({to})");
            }
        }
        Commands::Bridge {
            source,
            sink,
            batch_size,
            batch_timeout_ms,
            deduplicate,
            throttle_per_sec,
            retry_max_attempts,
            retry_delay_ms,
            schema_path,
            config,
        } => {
            let mw_config = load_middleware_config_bridge(
                *batch_size,
                *batch_timeout_ms,
                *deduplicate,
                *throttle_per_sec,
                *retry_max_attempts,
                *retry_delay_ms,
                schema_path.clone(),
                config,
            );
            
            // Parse endpoints
            let source_type = match SourceType::from_str(source) {
                Ok(st) => st,
                Err(e) => {
                    eprintln!("Invalid source configuration: {e}");
                    eprintln!("Supported formats:");
                    eprintln!("  - file:PATH");
                    eprintln!("  - kafka://host:port/topic?group=group_id");
                    eprintln!("  - stdin  or  -");
                    std::process::exit(2);
                }
            };
            
            let sink_type = match SinkType::from_str(sink) {
                Ok(st) => st,
                Err(e) => {
                    eprintln!("Invalid sink configuration: {e}");
                    eprintln!("Supported formats:");
                    eprintln!("  - file:PATH");
                    eprintln!("  - kafka://host:port/topic");
                    eprintln!("  - postgres://host:port/dbname?table=tablename[&schema=col1:type1,col2:type2]");
                    eprintln!("  - stdout  or  -");
                    std::process::exit(2);
                }
            };
            
            // Validate endpoints
            if let Err(e) = validate_endpoints(&source_type, &sink_type) {
                eprintln!("Endpoint validation error: {e}");
                std::process::exit(2);
            }
            
            let middleware_chain = build_middleware_chain(&mw_config);

            // Build source
            let source_box: Box<dyn fluxmux_core::traits::Source> = match source_type {
                SourceType::File { path } => Box::new(FileSource { path }),
                SourceType::Kafka { brokers, topic, group_id } => {
                    Box::new(KafkaSource::new(brokers, topic, group_id))
                }
                SourceType::Stdin => Box::new(PipeSource::new()),
            };

            // Build sink
            let sink_box: Box<dyn fluxmux_core::traits::Sink> = match sink_type {
                SinkType::File { path } => Box::new(FileSink::new(path, 1024)),
                SinkType::Kafka { brokers, topic } => {
                    Box::new(KafkaSink::new(brokers, topic))
                }
                SinkType::Postgres { connection_string, table, schema } => {
                    Box::new(PostgresSink::new(connection_string, table, schema))
                }
                SinkType::Stdout => Box::new(PipeSink::new()),
            };

            println!("Starting bridge: {} → {}", source, sink);
            if let Err(e) = run_pipeline(source_box, middleware_chain, sink_box).await {
                eprintln!("Bridge failed: {e}");
                std::process::exit(1);
            }
            println!("✓ Bridge completed successfully");
        }
        Commands::Pipe { source, args } => {
            // Parse source
            let source_type = match SourceType::from_str(source) {
                Ok(st) => st,
                Err(e) => {
                    eprintln!("Invalid source: {e}");
                    std::process::exit(2);
                }
            };

            // Parse actions and sinks from args
            let (actions, sinks) = parse_pipe_args(args);

            // Build source
            let source_box: Box<dyn fluxmux_core::traits::Source> = match source_type {
                SourceType::File { path } => Box::new(FileSource { path }),
                SourceType::Kafka { brokers, topic, group_id } => {
                    Box::new(KafkaSource::new(brokers, topic, group_id))
                }
                SourceType::Stdin => Box::new(PipeSource::new()),
            };

            println!("Starting pipe from {}", source);
            if let Err(e) = run_pipe(source_box, actions, sinks).await {
                eprintln!("Pipe failed: {e}");
                std::process::exit(1);
            }
            println!("✓ Pipe completed successfully");
        }
        Commands::Kafka { topic, broker, group, head, tail } => {
            if head.is_none() && tail.is_none() {
                eprintln!("Error: Either --head or --tail must be specified");
                std::process::exit(1);
            }

            if let Some(n) = head {
                if let Err(e) = kafka_inspector::kafka_head(broker, topic, group, *n).await {
                    eprintln!("Head failed: {}", e);
                    std::process::exit(1);
                }
            } else if let Some(n) = tail {
                if let Err(e) = kafka_inspector::kafka_tail(broker, topic, group, *n).await {
                    eprintln!("Tail failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn parse_pipe_args(args: &[String]) -> (Vec<Box<dyn fluxmux_core::pipe_actions::PipeAction>>, Vec<Box<dyn fluxmux_core::traits::Sink>>) {
    let mut actions: Vec<Box<dyn fluxmux_core::pipe_actions::PipeAction>> = vec![];
    let mut sinks: Vec<Box<dyn fluxmux_core::traits::Sink>> = vec![];
    
    let mut i = 0;
    while i < args.len() {
        let cmd = &args[i];
        match cmd.as_str() {
            "filter" => {
                if i + 1 < args.len() {
                    actions.push(Box::new(FilterAction::new(args[i + 1].clone())));
                    i += 2;
                } else {
                    eprintln!("filter requires an expression");
                    i += 1;
                }
            }
            "transform" => {
                if i + 1 < args.len() {
                    actions.push(Box::new(TransformAction::new(args[i + 1].clone())));
                    i += 2;
                } else {
                    eprintln!("transform requires an expression");
                    i += 1;
                }
            }
            "aggregate" => {
                let mut group_by = None;
                let mut ops = vec![];
                i += 1;
                
                while i < args.len() && !args[i].starts_with("--") {
                    if args[i] == "--group-by" && i + 1 < args.len() {
                        group_by = Some(args[i + 1].clone());
                        i += 2;
                    } else if args[i] == "--avg" && i + 1 < args.len() {
                        ops.push(("avg".to_string(), args[i + 1].clone()));
                        i += 2;
                    } else if args[i] == "--sum" && i + 1 < args.len() {
                        ops.push(("sum".to_string(), args[i + 1].clone()));
                        i += 2;
                    } else if args[i] == "--min" && i + 1 < args.len() {
                        ops.push(("min".to_string(), args[i + 1].clone()));
                        i += 2;
                    } else if args[i] == "--max" && i + 1 < args.len() {
                        ops.push(("max".to_string(), args[i + 1].clone()));
                        i += 2;
                    } else if args[i] == "--count" {
                        ops.push(("count".to_string(), "_".to_string()));
                        i += 1;
                    } else {
                        break;
                    }
                }
                actions.push(Box::new(AggregateAction::new(group_by, ops)));
            }
            "normalize" => {
                if i + 1 < args.len() && args[i + 1].starts_with("--schema") && i + 2 < args.len() {
                    actions.push(Box::new(NormalizeAction::new(Some(args[i + 2].clone()))));
                    i += 3;
                } else {
                    actions.push(Box::new(NormalizeAction::new(None)));
                    i += 1;
                }
            }
            "validate" => {
                if i + 1 < args.len() && args[i + 1].starts_with("--schema") && i + 2 < args.len() {
                    actions.push(Box::new(ValidateAction::new(Some(args[i + 2].clone()))));
                    i += 3;
                } else {
                    actions.push(Box::new(ValidateAction::new(None)));
                    i += 1;
                }
            }
            "limit" => {
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<usize>() {
                        actions.push(Box::new(LimitAction::new(n)));
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "sample" => {
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<usize>() {
                        actions.push(Box::new(SampleAction::new(n)));
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "tee" => {
                i += 1;
                // Collect all following args as sink destinations until next action
                while i < args.len() && !is_action(&args[i]) {
                    if let Ok(sink) = parse_sink_endpoint(&args[i]) {
                        sinks.push(sink);
                    }
                    i += 1;
                }
            }
            other => {
                // Try to parse as sink endpoint (for final output)
                if let Ok(sink) = parse_sink_endpoint(other) {
                    sinks.push(sink);
                }
                i += 1;
            }
        }
    }
    
    // If no sinks specified, default to stdout
    if sinks.is_empty() {
        sinks.push(Box::new(PipeSink::new()));
    }
    
    (actions, sinks)
}

fn is_action(s: &str) -> bool {
    matches!(s, "filter" | "transform" | "aggregate" | "normalize" | "validate" | "limit" | "sample" | "tee" | "buffer")
}

fn parse_sink_endpoint(s: &str) -> anyhow::Result<Box<dyn fluxmux_core::traits::Sink>> {
    if s == "-" || s.eq_ignore_ascii_case("stdout") {
        return Ok(Box::new(PipeSink::new()));
    }
    
    let sink_type = SinkType::from_str(s)?;
    match sink_type {
        SinkType::File { path } => Ok(Box::new(FileSink::new(path, 1024))),
        SinkType::Kafka { brokers, topic } => Ok(Box::new(KafkaSink::new(brokers, topic))),
        SinkType::Postgres { connection_string, table, schema } => {
            Ok(Box::new(PostgresSink::new(connection_string, table, schema)))
        }
        SinkType::Stdout => Ok(Box::new(PipeSink::new())),
    }
}
