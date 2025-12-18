use colored::*;

pub fn success(msg: &str) -> String {
    format!("{} {}", "[OK]".green(), msg.green())
}

pub fn error(msg: &str) -> String {
    format!("{} {}", "[ERROR]".red(), msg.red())
}

pub fn warning(msg: &str) -> String {
    format!("{} {}", "[WARN]".yellow(), msg.yellow())
}

pub fn info(msg: &str) -> String {
    format!("{} {}", "[INFO]".blue(), msg.blue())
}

pub fn prompt() -> &'static str {
    "fluxmux> "
}

pub fn highlight(msg: &str) -> String {
    msg.bright_cyan().to_string()
}

pub fn command(msg: &str) -> String {
    msg.bright_yellow().to_string()
}

pub fn header(msg: &str) -> String {
    format!("\n{}\n", msg.bright_white())
}

pub fn table_row(key: &str, value: &str) -> String {
    format!("  {} = {}", key.cyan(), value)
}
