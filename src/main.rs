use clap::{Parser, Subcommand};
mod workers;

#[derive(Parser)]
#[command(author = "Itta Funahashi", version, about = "Simple LCD monitoring system for Raspberry Pi cluster", long_about = None)]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(author = "Itta Funahashi", version, about = "Start reporting stats to the host device", long_about = None)]
    Reporter(workers::reporter_cmd::ReporterArg),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Reporter(arg)) => workers::reporter_cmd::run(arg).await?,
        None => todo!(),
    }
    Ok(())
}
