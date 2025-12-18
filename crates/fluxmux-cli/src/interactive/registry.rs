use std::collections::HashMap;
use colored::Colorize;
use crate::interactive::colors::*;

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub usage_count: u32,
}

pub struct CommandRegistry {
    commands: HashMap<String, CommandInfo>,
    usage_counts: HashMap<String, u32>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
            usage_counts: HashMap::new(),
        };
        registry.register_commands();
        registry
    }

    fn register_commands(&mut self) {
        self.add_command(
            "convert",
            "Convert between file formats (csv, json, yaml, toml, xml, parquet, avro, msgpack, cbor)",
            "convert <input_file> <output_file> --from <format> --to <format>",
        );

        self.add_command(
            "bridge",
            "Bridge data between endpoints (file, kafka, postgres, stdin, stdout)",
            "bridge --source <source> --sink <sink> [--batch-size N] [--deduplicate] [--throttle-per-sec N]",
        );

        self.add_command(
            "pipe",
            "Stream processing: filter, transform, aggregate, normalize, validate, limit, and sink data",
            "pipe <source> [filter 'expr'] [transform 'expr'] [aggregate --group-by field --sum field] [normalize] [validate] [limit N] [tee output] [kafka://host/topic]",
        );

        self.add_command(
            "kafka",
            "Kafka topic inspector (read messages from any topic using variables)",
            "kafka --topic <topic_name> [--broker <host:port>] [--group <group_id>] [--head N | --tail N]",
        );

        self.add_command(
            "help",
            "Show help for all commands or a specific command",
            "help [command_name]",
        );

        self.add_command(
            "status",
            "Display current session state (file paths, brokers, settings)",
            "status",
        );

        self.add_command(
            "set",
            "Set session or custom variables (predefined: file, sink, kafka-broker, batch-size, etc. or any custom var)",
            "set file=<path> | set sink=<path> | set kafka-broker=<host:port> | set myvar=<value>",
        );

        self.add_command(
            "clear",
            "Clear session variables or entire state",
            "clear [file | sink | kafka | all]",
        );

        self.add_command(
            "history",
            "Show command history from this session",
            "history [N]  (show last N commands, default 20)",
        );

        self.add_command(
            "exit",
            "Exit the interactive shell (Ctrl+D also works)",
            "exit | quit",
        );
    }

    fn add_command(&mut self, name: &str, description: &str, usage: &str) {
        self.commands.insert(
            name.to_string(),
            CommandInfo {
                name: name.to_string(),
                description: description.to_string(),
                usage: usage.to_string(),
                usage_count: 0,
            },
        );
    }

    pub fn record_usage(&mut self, cmd: &str) {
        *self.usage_counts.entry(cmd.to_string()).or_insert(0) += 1;
        if let Some(info) = self.commands.get_mut(cmd) {
            info.usage_count = *self.usage_counts.get(cmd).unwrap_or(&0);
        }
    }

    pub fn get_command(&self, name: &str) -> Option<&CommandInfo> {
        self.commands.get(name)
    }

    pub fn help_all(&self) -> String {
        let mut output = String::new();
        output.push_str(&header("FLUXMUX Interactive Commands"));
        output.push_str(&info("Format: command [args]\n"));

        // Sort by usage count (descending) then name
        let mut commands: Vec<_> = self.commands.values().collect();
        commands.sort_by(|a, b| {
            b.usage_count.cmp(&a.usage_count).then(a.name.cmp(&b.name))
        });

        for cmd in commands {
            output.push_str(&format!("{}", command(&cmd.name)));
            output.push_str(" - ");
            output.push_str(&cmd.description);
            if cmd.usage_count > 0 {
                output.push_str(&format!(" {} [used {} times]", "".yellow(), cmd.usage_count.to_string().cyan()));
            }
            output.push_str("\n");
            output.push_str(&format!("  {}\n\n", highlight(&cmd.usage)));
        }

        output.push_str(&info("Tips:"));
        output.push_str("\n");
        output.push_str(&table_row("Use", "'help <command>' for specific help\n"));
        output.push_str(&table_row("Use", "'status' to see current session state\n"));
        output.push_str(&table_row("Use", "'set' to persist settings across commands\n"));
        output.push_str(&table_row("Use", "Ctrl+D, exit, or quit to exit\n"));
        output.push_str(&table_row("Use", "Tab key for command completion (if available)\n"));

        output
    }

    pub fn help_command(&self, name: &str) -> String {
        match self.get_command(name) {
            Some(cmd) => {
                let mut output = String::new();
                output.push_str(&header(&format!("{} Command", cmd.name)));
                output.push_str(&table_row("Description:", &cmd.description));
                output.push_str("\n");
                output.push_str(&table_row("Usage:", &cmd.usage));
                output.push_str("\n");
                
                // Add extended help for specific commands
                if cmd.name == "kafka" {
                    output.push_str(&header("Kafka Parameters & Examples"));
                    output.push_str(&format!("  {}\n", highlight("--topic <name>")));
                    output.push_str("    Topic name to read from (REQUIRED). Use variables: ${topic}\n\n");
                    
                    output.push_str(&format!("  {}\n", highlight("--broker <host:port>")));
                    output.push_str(&format!("    Kafka broker address (optional, defaults to session setting)\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("--group <group_id>")));
                    output.push_str(&format!("    Consumer group ID (optional, defaults to session setting)\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("--head N")));
                    output.push_str(&format!("    Read first N messages from topic\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("--tail N")));
                    output.push_str(&format!("    Read last N messages from topic\n\n"));
                    
                    output.push_str(&header("Usage Examples"));
                    output.push_str(&format!("  {}\n", highlight("kafka --topic my-topic --head 10")));
                    output.push_str(&format!("    Read first 10 messages\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("kafka --topic my-topic --tail 5")));
                    output.push_str(&format!("    Read last 5 messages\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("set my-topic = vasudeva")));
                    output.push_str(&format!("    kafka --topic ${{my-topic}} --head 20\n"));
                    output.push_str(&format!("    Use variables for dynamic topic names\n\n"));
                } else if cmd.name == "pipe" {
                    output.push_str(&header("Pipe Commands (Operations)"));
                    output.push_str(&format!("  {}\n", highlight("filter 'expression'")));
                    output.push_str(&format!("    Filter records matching the expression\n"));
                    output.push_str(&format!("    Example: pipe input.json filter 'age > 25'\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("transform 'expression'")));
                    output.push_str(&format!("    Transform records with custom expressions\n"));
                    output.push_str(&format!("    Example: pipe input.json transform 'name=upper(name)'\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("aggregate [--group-by field] [--sum|--avg|--min|--max|--count field]")));
                    output.push_str(&format!("    Aggregate data with operations\n"));
                    output.push_str(&format!("    Example: pipe input.json aggregate --group-by category --sum price\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("normalize [--schema file]")));
                    output.push_str(&format!("    Normalize records against optional schema\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("validate [--schema file]")));
                    output.push_str(&format!("    Validate records against optional schema\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("limit N")));
                    output.push_str(&format!("    Limit output to N records\n"));
                    output.push_str(&format!("    Example: pipe input.json limit 100\n\n"));
                    
                    output.push_str(&format!("  {}\n", highlight("tee output.json | file:path | kafka://broker/topic")));
                    output.push_str(&format!("    Send output to file or Kafka topic\n\n"));
                }
                
                if cmd.usage_count > 0 {
                    output.push_str(&table_row("Used:", &format!("{} times", cmd.usage_count)));
                    output.push_str("\n");
                }
                output
            }
            None => error(&format!("Unknown command: {}", name)),
        }
    }

    #[allow(dead_code)]
    pub fn list_commands(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }

    pub fn usage_stats(&self) -> String {
        let mut output = String::new();
        output.push_str(&header("📊 Command Usage Statistics"));

        let mut stats: Vec<_> = self.usage_counts.iter().collect();
        stats.sort_by(|a, b| b.1.cmp(a.1));

        if stats.is_empty() {
            output.push_str(&warning("No commands used yet in this session\n"));
            return output;
        }

        for (cmd, count) in stats {
            output.push_str(&format!(
                "  {} → {} times\n",
                command(cmd),
                count.to_string().bright_yellow()
            ));
        }

        output
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}