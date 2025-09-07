
mod conversions;
use clap::{Parser, Subcommand};
use conversions::{Format, convert};

#[derive(Parser)]
#[command(name = "fluxmux", about = "Universal CLI for File Conversion & Stream Inspection")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        input: String,
        output: String,
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
    },
    // ...existing code for Bridge, Pipe, Monitor (if needed)...
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
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
                println!("✅ Converted {input} ({from}) → {output} ({to})");
            }
        }
        // Commands::Bridge { from, to } => {
        //     println!("Bridge from {from} -> {to}");
        // }
        // Commands::Pipe { steps } => {
        //     println!("Pipeline steps: {:?}", steps);
        // }
        // Commands::Monitor { target } => {
        //     println!("Monitoring {target}");
        // }
    }
}
