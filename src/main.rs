use std::{env, process};
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_key = "HOST";
    let port_key = "PORT";
    let mut interval = interval(Duration::from_secs(10));
    dotenvy::dotenv()?;

    let host = match env::var(host_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, host_key);
            process::exit(1);
        }
    };

    let port = match env::var(port_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, port_key);
            process::exit(1);
        }
    };

    let host_address = format!("http://{host}:{port}", host = host, port = port);

    loop {
        interval.tick().await;
        let resp = reqwest::get(&host_address.clone()).await?.text().await?;

        println!("{:#?}", resp);
    }
}
