use crate::types::IndexResponse;
use clap::Parser;
use tokio::time::{interval, Duration};

#[derive(Parser)]
pub struct ReporterArg {
    #[clap(help = "IP Address of the host device")]
    host: String,
    #[clap(
        short = 'p',
        long = "port",
        help = "Port of the app running on the host device",
        default_value = "2784"
    )]
    port: String,
}

pub async fn run(arg: &ReporterArg) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = interval(Duration::from_secs(10));
    let host = &arg.host;
    let port = &arg.port;
    let host_address = format!("http://{host}:{port}", host = host, port = port);

    println!("Checking host {} is running...", host_address);

    reqwest::get(&host_address.clone())
        .await
        .expect("Host server is not running")
        .json::<IndexResponse>()
        .await
        .expect("Failed to parse json from host server\nIs host server running on the right port?");

    loop {
        interval.tick().await;
        let resp = reqwest::get(&host_address.clone()).await?.text().await?;

        println!("{:#?}", resp)
    }
}
