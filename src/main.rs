use clap::{Parser, Subcommand};
pub mod utils;
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
    #[command(author = "Itta Funahashi", version, about, long_about = None)]
    Run(Run),
}

#[derive(Parser)]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
struct Run {
    #[command(subcommand)]
    command: Option<RunnableCommands>,
}

#[derive(Subcommand)]
enum RunnableCommands {
    #[command(author = "Itta Funahashi", version, about = "Start reporting stats to the host device", long_about = None)]
    Reporter(workers::reporter_cmd::ReporterArg),
    Host(workers::host_cmd::HostArg),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run(run)) => match &run.command {
            Some(RunnableCommands::Reporter(arg)) => workers::reporter_cmd::run(arg).await?,
            Some(RunnableCommands::Host(arg)) => workers::host_cmd::run(arg).await?,
            None => todo!(),
        },
        None => todo!(),
    }
    Ok(())
}
