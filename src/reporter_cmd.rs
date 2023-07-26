use clap::Parser;
use tokio::time::{interval, Duration};

#[derive(Parser)]
pub struct ReporterArg {
    host: String,
    #[clap(long = "port", default_value = "2784")]
    port: String,
}

pub async fn run(arg: &ReporterArg) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = interval(Duration::from_secs(10));
    let host = &arg.host;
    let port = &arg.port;
    let host_address = format!("http://{host}:{port}", host = host, port = port);

    println!("Checking host {} is running...", host_address);

    loop {
        interval.tick().await;
        let resp = reqwest::get(&host_address.clone()).await?.text().await?;

        println!("{:#?}", resp)
    }
}
