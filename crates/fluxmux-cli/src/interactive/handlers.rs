use fluxmux_core::engine::{MiddlewareConfig, build_middleware_chain, run_pipeline};
use fluxmux_core::pipe_actions::*;
use fluxmux_core::pipe_engine::run_pipe;
use std::str::FromStr;
use crate::conversions::{Format, convert};
use crate::endpoints::{SourceType, SinkType};
use crate::kafka_inspector;
use crate::interactive::colors::*;
use crate::interactive::state::SessionState;
use fluxmux_connectors::{FileSource, KafkaSource, PipeSource};
use fluxmux_sinks::{FileSink, KafkaSink, PostgresSink, PipeSink};

pub async fn handle_convert(
    input: &str,
    output: &str,
    from: &str,
    to: &str,
    _state: &SessionState,
) -> Result<(), String> {
    let from_fmt = Format::from_ext(from).ok_or_else(|| format!("Unsupported input format: {}", from))?;
    let to_fmt = Format::from_ext(to).ok_or_else(|| format!("Unsupported output format: {}", to))?;

    convert(input, output, from_fmt, to_fmt)
        .map_err(|e| format!("Conversion failed: {}", e))?;

    println!("{}", success(&format!("Converted {} ({}) → {} ({})", input, from, output, to)));
    Ok(())
}

pub async fn handle_bridge(
    source: &str,
    sink: &str,
    batch_size: Option<usize>,
    batch_timeout_ms: Option<u64>,
    deduplicate: bool,
    throttle_per_sec: Option<u64>,
    retry_max_attempts: Option<u32>,
    retry_delay_ms: Option<u64>,
    schema_path: Option<String>,
    config: Option<String>,
    state: &SessionState,
) -> Result<(), String> {
    let mut mw_config = if let Some(ref path) = config {
        let yaml = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        serde_yaml::from_str::<MiddlewareConfig>(&yaml)
            .map_err(|e| format!("Invalid YAML: {}", e))?
    } else {
        MiddlewareConfig::default()
    };

    // CLI args override
    if let Some(bs) = batch_size {
        mw_config.batch_size = Some(bs);
    } else if let Some(bs) = state.batch_size {
        mw_config.batch_size = Some(bs);
    }

    if let Some(bt) = batch_timeout_ms {
        mw_config.batch_timeout_ms = Some(bt);
    }
    mw_config.deduplicate = Some(deduplicate || state.deduplicate);
    if let Some(tp) = throttle_per_sec {
        mw_config.throttle_per_sec = Some(tp);
    } else if let Some(tp) = state.throttle_per_sec {
        mw_config.throttle_per_sec = Some(tp);
    }
    if let Some(retry) = retry_max_attempts {
        mw_config.retry_max_attempts = Some(retry);
    }
    if let Some(delay) = retry_delay_ms {
        mw_config.retry_delay_ms = Some(delay);
    }
    if let Some(sp) = schema_path {
        mw_config.schema_path = Some(sp);
    }

    let source_type = SourceType::from_str(source)
        .map_err(|e| format!("Invalid source: {}", e))?;
    let sink_type = SinkType::from_str(sink)
        .map_err(|e| format!("Invalid sink: {}", e))?;

    // Validate endpoints
    if matches!(source_type, SourceType::File { .. }) && matches!(sink_type, SinkType::File { .. }) {
        return Err("File-to-file transfers are not allowed. Use Kafka or Postgres as source or sink.".to_string());
    }

    let middleware_chain = build_middleware_chain(&mw_config);

    let source_box: Box<dyn fluxmux_core::traits::Source> = match source_type {
        SourceType::File { path } => Box::new(FileSource { path }),
        SourceType::Kafka { brokers, topic, group_id } => {
            Box::new(KafkaSource::new(brokers, topic, group_id))
        }
        SourceType::Stdin => Box::new(PipeSource::new()),
    };

    let sink_box: Box<dyn fluxmux_core::traits::Sink> = match sink_type {
        SinkType::File { path } => Box::new(FileSink::new(path, 1024)),
        SinkType::Kafka { brokers, topic } => Box::new(KafkaSink::new(brokers, topic)),
        SinkType::Postgres { connection_string, table, schema } => {
            Box::new(PostgresSink::new(connection_string, table, schema))
        }
        SinkType::Stdout => Box::new(PipeSink::new()),
    };

    println!("{}", info(&format!("Starting bridge: {} → {}", source, sink)));
    run_pipeline(source_box, middleware_chain, sink_box)
        .await
        .map_err(|e| format!("Bridge failed: {}", e))?;

    println!("{}", success("Bridge completed successfully"));
    Ok(())
}

pub async fn handle_pipe(
    source: &str,
    args: Vec<String>,
    _state: &SessionState,
) -> Result<(), String> {
    let source_type = SourceType::from_str(source)
        .map_err(|e| format!("Invalid source: {}", e))?;

    let (actions, sinks) = parse_pipe_args(&args);

    let source_box: Box<dyn fluxmux_core::traits::Source> = match source_type {
        SourceType::File { path } => Box::new(FileSource { path }),
        SourceType::Kafka { brokers, topic, group_id } => {
            Box::new(KafkaSource::new(brokers, topic, group_id))
        }
        SourceType::Stdin => Box::new(PipeSource::new()),
    };

    println!("{}", info(&format!("Starting pipe from {}", source)));
    run_pipe(source_box, actions, sinks)
        .await
        .map_err(|e| format!("Pipe failed: {}", e))?;

    println!("{}", success("Pipe completed successfully"));
    Ok(())
}

pub async fn handle_kafka(
    topic: &str,
    broker: &str,
    group: &str,
    head: Option<usize>,
    tail: Option<usize>,
) -> Result<(), String> {
    if head.is_none() && tail.is_none() {
        return Err("Either --head or --tail must be specified".to_string());
    }

    if let Some(n) = head {
        kafka_inspector::kafka_head(broker, topic, group, n)
            .await
            .map_err(|e| format!("Head failed: {}", e))?;
    } else if let Some(n) = tail {
        kafka_inspector::kafka_tail(broker, topic, group, n)
            .await
            .map_err(|e| format!("Tail failed: {}", e))?;
    }

    Ok(())
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
                    eprintln!("{}", error("filter requires an expression"));
                    i += 1;
                }
            }
            "transform" => {
                if i + 1 < args.len() {
                    actions.push(Box::new(TransformAction::new(args[i + 1].clone())));
                    i += 2;
                } else {
                    eprintln!("{}", error("transform requires an expression"));
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
                while i < args.len() && !is_action(&args[i]) {
                    if let Ok(sink) = parse_sink_endpoint(&args[i]) {
                        sinks.push(sink);
                    }
                    i += 1;
                }
            }
            other => {
                if let Ok(sink) = parse_sink_endpoint(other) {
                    sinks.push(sink);
                }
                i += 1;
            }
        }
    }

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
