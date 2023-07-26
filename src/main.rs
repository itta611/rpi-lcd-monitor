use clap::{App, Parser, Subcommand};
use tokio::time::{interval, Duration};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Reporter(ReporterArg),
}

#[derive(Parser)]
struct ReporterArg {
    #[clap(long = "host")]
    host: String,
    #[clap(long = "port", default_value = "2784")]
    port: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut interval = interval(Duration::from_secs(10));

    match &cli.command {
        Some(Commands::Reporter(arg)) => {
            let host = &arg.host;
            let port = &arg.port;
            let host_address = format!("http://{host}:{port}", host = host, port = port);

            println!("Checking host {:} is running...", host_address);

            loop {
                interval.tick().await;
                let resp = reqwest::get(&host_address.clone()).await?.text().await?;

                println!("{:#?}", resp)
            }
        }
        None => todo!(),
    }

    Ok(())
}
