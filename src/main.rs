use clap::{Parser, Subcommand};
mod reporter_cmd;

#[derive(Parser)]
#[command(author = "Itta Funahashi", version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Reporter(reporter_cmd::ReporterArg),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Reporter(arg)) => reporter_cmd::run(arg).await?,
        None => todo!(),
    }
    Ok(())
}
