use rustyline::error::ReadlineError;
use rustyline::{Editor, history::FileHistory};
use crate::interactive::{colors::*, SessionState, CommandRegistry, handlers};

fn print_ascii_art() {
    println!(r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║   ███████╗██╗     ██╗   ██╗██╗  ██╗███╗   ███╗██╗   ██╗██╗  ██╗              ║
║   ██╔════╝██║     ██║   ██║╚██╗██╔╝████╗ ████║██║   ██║╚██╗██╔╝              ║
║   █████╗  ██║     ██║   ██║ ╚███╔╝ ██╔████╔██║██║   ██║ ╚███╔╝               ║
║   ██╔══╝  ██║     ██║   ██║ ██╔██╗ ██║╚██╔╝██║██║   ██║ ██╔██╗               ║
║   ██║     ███████╗╚██████╔╝██╔╝ ██╗██║ ╚═╝ ██║╚██████╔╝██╔╝ ██╗              ║
║   ╚═╝     ╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝              ║
║                                                                              ║
║              Universal Data Format & Stream Processing Tool                  ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#);
}

pub async fn run_interactive() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<(), FileHistory>::new()?;
    let history_file = SessionState::get_state_file()
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("history");

    let _ = rl.load_history(&history_file);

    let mut state = SessionState::load();
    let mut registry = CommandRegistry::new();
    let mut command_history: Vec<String> = Vec::new();

    print_ascii_art();
    println!("{}", header("FluxMux Interactive Mode"));
    println!("{}", info("Type 'help' for available commands or 'exit' to quit\n"));

    loop {
        match rl.readline(&prompt()) {
            Ok(line) => {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    continue;
                }

                command_history.push(trimmed.to_string());
                let _ = rl.add_history_entry(trimmed);

                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                let cmd = parts[0];
                registry.record_usage(cmd);

                match cmd {
                    "exit" | "quit" => {
                        state.save();
                        let _ = rl.save_history(&history_file);
                        println!("{}", success("Session saved. Goodbye."));
                        break;
                    }

                    "help" => {
                        if parts.len() > 1 {
                            println!("{}", registry.help_command(parts[1]));
                        } else {
                            println!("{}", registry.help_all());
                        }
                    }

                    "status" => {
                        println!("{}", state.display());
                    }

                    "set" => {
                        if parts.len() > 1 {
                            // Join all parts after "set" to handle spaces around =
                            let set_arg = parts[1..].join(" ");
                            if let Some(eq_pos) = set_arg.find('=') {
                                let key = set_arg[..eq_pos].trim();
                                let value = set_arg[eq_pos+1..].trim();
                                if !key.is_empty() && !value.is_empty() {
                                    handle_set_command_new(key, value, &mut state);
                                } else {
                                    println!("{}", error("Usage: set key=value (e.g., set file=input.json)"));
                                }
                            } else {
                                println!("{}", error("Usage: set key=value (e.g., set file=input.json)"));
                            }
                        } else {
                            println!("{}", error("Usage: set key=value"));
                        }
                    }

                    "clear" => {
                        handle_clear_command(&parts[1..], &mut state);
                    }

                    "history" => {
                        handle_history_command(&parts[1..], &command_history);
                    }

                    "stats" => {
                        println!("{}", registry.usage_stats());
                    }

                    "convert" => {
                        let args = parts[1..].iter().map(|s| state.substitute_vars(s)).collect::<Vec<_>>();
                        match parse_convert_args(&args) {
                            Ok((input, output, from, to)) => {
                                match handlers::handle_convert(&input, &output, &from, &to, &state).await {
                                    Ok(_) => {}
                                    Err(e) => println!("{}", error(&e)),
                                }
                            }
                            Err(e) => println!("{}", error(&e)),
                        }
                    }

                    "bridge" => {
                        let args = parts[1..].iter().map(|s| state.substitute_vars(s)).collect::<Vec<_>>();
                        match parse_bridge_args(&args) {
                            Ok((source, sink, batch_size, batch_timeout_ms, deduplicate, throttle_per_sec, retry_max_attempts, retry_delay_ms, schema_path, config)) => {
                                match handlers::handle_bridge(
                                    &source,
                                    &sink,
                                    batch_size,
                                    batch_timeout_ms,
                                    deduplicate,
                                    throttle_per_sec,
                                    retry_max_attempts,
                                    retry_delay_ms,
                                    schema_path,
                                    config,
                                    &state,
                                )
                                .await
                                {
                                    Ok(_) => {}
                                    Err(e) => println!("{}", error(&e)),
                                }
                            }
                            Err(e) => println!("{}", error(&e)),
                        }
                    }

                    "pipe" => {
                        let args = parts[1..].iter().map(|s| state.substitute_vars(s)).collect::<Vec<_>>();
                        if args.is_empty() {
                            println!("{}", error("pipe requires a source"));
                        } else {
                            let source = args[0].clone();
                            let pipe_args = args[1..].to_vec();
                            match handlers::handle_pipe(&source, pipe_args, &state).await {
                                Ok(_) => {}
                                Err(e) => println!("{}", error(&e)),
                            }
                        }
                    }

                    "kafka" => {
                        let args = parts[1..].iter().map(|s| state.substitute_vars(s)).collect::<Vec<_>>();
                        match parse_kafka_args(&args, &state) {
                            Ok((topic, broker, group, head, tail)) => {
                                match handlers::handle_kafka(&topic, &broker, &group, head, tail).await {
                                    Ok(_) => {}
                                    Err(e) => println!("{}", error(&e)),
                                }
                            }
                            Err(e) => println!("{}", error(&e)),
                        }
                    }

                    _ => {
                        println!("{}", error(&format!("Unknown command: '{}'. Type 'help' for available commands.", cmd)));
                    }
                }

                state.save();
            }
            Err(ReadlineError::Interrupted) => {
                state.save();
                let _ = rl.save_history(&history_file);
                println!("\n{}", success("Interrupted. Session saved. Goodbye!"));
                break;
            }
            Err(ReadlineError::Eof) => {
                state.save();
                let _ = rl.save_history(&history_file);
                println!("\n{}", success("EOF received. Session saved. Goodbye!"));
                break;
            }
            Err(e) => {
                eprintln!("{}", error(&format!("Readline error: {}", e)));
            }
        }
    }

    Ok(())
}

fn handle_set_command_new(key: &str, value: &str, state: &mut SessionState) {
    match key {
        "file" => {
            state.set_file_path(value.to_string());
            println!("{}", success(&format!("Set file = {}", value)));
        }
        "sink" => {
            state.set_sink_path(value.to_string());
            println!("{}", success(&format!("Set sink = {}", value)));
        }
        "kafka-broker" => {
            state.set_kafka_broker(value.to_string());
            println!("{}", success(&format!("Set kafka-broker = {}", value)));
        }
        "kafka-group" => {
            state.set_kafka_group(value.to_string());
            println!("{}", success(&format!("Set kafka-group = {}", value)));
        }
        "postgres" => {
            state.set_postgres_conn(value.to_string());
            println!("{}", success(&format!("Set postgres = {}", value)));
        }
        "batch-size" => {
            if let Ok(size) = value.parse::<usize>() {
                state.set_batch_size(size);
                println!("{}", success(&format!("Set batch-size = {}", value)));
            } else {
                println!("{}", error("batch-size must be a number"));
            }
        }
        "deduplicate" => {
            let bool_val = value.to_lowercase() == "true" || value == "1" || value == "on";
            if bool_val {
                state.deduplicate = true;
                println!("{}", success("Set deduplicate = true"));
            } else {
                state.deduplicate = false;
                println!("{}", success("Set deduplicate = false"));
            }
        }
        "throttle" => {
            if let Ok(per_sec) = value.parse::<u64>() {
                state.set_throttle(per_sec);
                println!("{}", success(&format!("Set throttle = {}", value)));
            } else {
                println!("{}", error("throttle must be a number"));
            }
        }
        _ => {
            // Accept any custom variable
            state.set_custom_var(key.to_string(), value.to_string());
            println!("{}", success(&format!("Set {} = {}", key, value)));
        }
    }
}

#[allow(dead_code)]
fn handle_set_command(args: &[&str], state: &mut SessionState) {
    if args.is_empty() {
        println!(
            "{}",
            error("Usage: set key=value (e.g., set file=input.json, set batch-size=500)")
        );
        return;
    }

    match args[0] {
        "file" => {
            if args.len() > 1 {
                state.set_file_path(args[1..].join(" "));
            } else {
                println!("{}", error("set file requires a path"));
            }
        }
        "sink" => {
            if args.len() > 1 {
                state.set_sink_path(args[1..].join(" "));
            } else {
                println!("{}", error("set sink requires a path"));
            }
        }
        "kafka-broker" => {
            if args.len() > 1 {
                state.set_kafka_broker(args[1].to_string());
            } else {
                println!("{}", error("set kafka-broker requires a broker address"));
            }
        }
        "kafka-group" => {
            if args.len() > 1 {
                state.set_kafka_group(args[1].to_string());
            } else {
                println!("{}", error("set kafka-group requires a group ID"));
            }
        }
        "postgres" => {
            if args.len() > 1 {
                state.set_postgres_conn(args[1..].join(" "));
            } else {
                println!("{}", error("set postgres requires a connection string"));
            }
        }
        "batch-size" => {
            if args.len() > 1 {
                if let Ok(size) = args[1].parse::<usize>() {
                    state.set_batch_size(size);
                } else {
                    println!("{}", error("batch-size must be a number"));
                }
            } else {
                println!("{}", error("set batch-size requires a number"));
            }
        }
        "deduplicate" => {
            state.toggle_deduplicate();
        }
        "throttle" => {
            if args.len() > 1 {
                if let Ok(per_sec) = args[1].parse::<u64>() {
                    state.set_throttle(per_sec);
                } else {
                    println!("{}", error("throttle must be a number"));
                }
            } else {
                println!("{}", error("set throttle requires a number"));
            }
        }
        _ => {
            println!("{}", error(&format!("Unknown set option: {}", args[0])));
        }
    }
}

fn handle_clear_command(args: &[&str], state: &mut SessionState) {
    if args.is_empty() {
        println!(
            "{}",
            warning("Clearing entire session state")
        );
        *state = SessionState::default();
        println!("{}", success("Session cleared"));
    } else {
        match args[0] {
            "file" => {
                state.file_path = None;
                println!("{}", success("File path cleared"));
            }
            "sink" => {
                state.sink_path = None;
                println!("{}", success("Sink path cleared"));
            }
            "kafka" => {
                state.kafka_broker = "localhost:9092".to_string();
                state.kafka_group = "fluxmux-interactive".to_string();
                println!("{}", success("Kafka settings reset to defaults"));
            }
            "postgres" => {
                state.postgres_conn = None;
                println!("{}", success("Postgres connection cleared"));
            }
            "all" => {
                *state = SessionState::default();
                println!("{}", success("All session settings cleared"));
            }
            _ => {
                println!("{}", error(&format!("Unknown clear option: {}", args[0])));
            }
        }
    }
}

fn handle_history_command(args: &[&str], history: &[String]) {
    let limit = if args.is_empty() {
        20
    } else {
        args[0].parse::<usize>().unwrap_or(20)
    };

    println!("{}", header("Command History"));
    let start = if history.len() > limit {
        history.len() - limit
    } else {
        0
    };

    for (idx, cmd) in history[start..].iter().enumerate() {
        let line_num = start + idx + 1;
        println!("  {} {}", command(&format!("[{}]", line_num)), cmd);
    }
}

fn get_file_extension(path: &str) -> String {
    path.rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase()
}

fn validate_format_consistency(input: &str, output: &str, from: &str, to: &str) -> Result<(), String> {
    let input_ext = get_file_extension(input);
    let output_ext = get_file_extension(output);

    // Normalize format names
    let from_norm = from.to_lowercase();
    let to_norm = to.to_lowercase();
    let input_ext_norm = input_ext.trim_start_matches('.').to_lowercase();
    let output_ext_norm = output_ext.trim_start_matches('.').to_lowercase();

    // Validate input file extension matches --from
    if !input_ext.is_empty() && input_ext_norm != from_norm {
        return Err(format!(
            "Input file extension '{}' does not match --from format '{}'. Use --from {} or rename the file.",
            input_ext, from, input_ext_norm
        ));
    }

    // Validate output file extension matches --to
    if !output_ext.is_empty() && output_ext_norm != to_norm {
        return Err(format!(
            "Output file extension '{}' does not match --to format '{}'. Change output to '*.{}' or use --to {}",
            output_ext, to, to_norm, output_ext_norm
        ));
    }

    Ok(())
}

fn parse_convert_args(args: &[String]) -> Result<(String, String, String, String), String> {
    if args.len() < 4 {
        return Err("convert requires: input output --from <fmt> --to <fmt>".to_string());
    }

    let input = args[0].clone();
    let output = args[1].clone();

    let mut from = String::new();
    let mut to = String::new();

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--from" if i + 1 < args.len() => {
                from = args[i + 1].clone();
                i += 2;
            }
            "--to" if i + 1 < args.len() => {
                to = args[i + 1].clone();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if from.is_empty() || to.is_empty() {
        return Err("--from and --to are required".to_string());
    }

    // Validate format consistency
    validate_format_consistency(&input, &output, &from, &to)?;

    Ok((input, output, from, to))
}

fn parse_bridge_args(
    args: &[String],
) -> Result<
    (
        String,
        String,
        Option<usize>,
        Option<u64>,
        bool,
        Option<u64>,
        Option<u32>,
        Option<u64>,
        Option<String>,
        Option<String>,
    ),
    String,
> {
    let mut source = String::new();
    let mut sink = String::new();
    let mut batch_size = None;
    let mut batch_timeout_ms = None;
    let mut deduplicate = false;
    let mut throttle_per_sec = None;
    let mut retry_max_attempts = None;
    let mut retry_delay_ms = None;
    let mut schema_path = None;
    let mut config = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--source" if i + 1 < args.len() => {
                source = args[i + 1].clone();
                i += 2;
            }
            "--sink" if i + 1 < args.len() => {
                sink = args[i + 1].clone();
                i += 2;
            }
            "--batch-size" if i + 1 < args.len() => {
                batch_size = args[i + 1].parse::<usize>().ok();
                i += 2;
            }
            "--batch-timeout-ms" if i + 1 < args.len() => {
                batch_timeout_ms = args[i + 1].parse::<u64>().ok();
                i += 2;
            }
            "--deduplicate" => {
                deduplicate = true;
                i += 1;
            }
            "--throttle-per-sec" if i + 1 < args.len() => {
                throttle_per_sec = args[i + 1].parse::<u64>().ok();
                i += 2;
            }
            "--retry-max-attempts" if i + 1 < args.len() => {
                retry_max_attempts = args[i + 1].parse::<u32>().ok();
                i += 2;
            }
            "--retry-delay-ms" if i + 1 < args.len() => {
                retry_delay_ms = args[i + 1].parse::<u64>().ok();
                i += 2;
            }
            "--schema-path" if i + 1 < args.len() => {
                schema_path = Some(args[i + 1].clone());
                i += 2;
            }
            "--config" if i + 1 < args.len() => {
                config = Some(args[i + 1].clone());
                i += 2;
            }
            _ => i += 1,
        }
    }

    if source.is_empty() || sink.is_empty() {
        return Err("--source and --sink are required".to_string());
    }

    Ok((
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
    ))
}

fn parse_kafka_args(args: &[String], state: &SessionState) -> Result<(String, String, String, Option<usize>, Option<usize>), String> {
    let mut topic = String::new();
    let mut broker = state.kafka_broker.clone();
    let mut group = state.kafka_group.clone();
    let mut head = None;
    let mut tail = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--topic" if i + 1 < args.len() => {
                topic = args[i + 1].clone();
                i += 2;
            }
            "--broker" if i + 1 < args.len() => {
                broker = args[i + 1].clone();
                i += 2;
            }
            "--group" if i + 1 < args.len() => {
                group = args[i + 1].clone();
                i += 2;
            }
            "--head" if i + 1 < args.len() => {
                head = args[i + 1].parse::<usize>().ok();
                i += 2;
            }
            "--tail" if i + 1 < args.len() => {
                tail = args[i + 1].parse::<usize>().ok();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if topic.is_empty() {
        return Err("--topic is required".to_string());
    }

    if head.is_none() && tail.is_none() {
        return Err("Either --head or --tail must be specified".to_string());
    }

    Ok((topic, broker, group, head, tail))
}
