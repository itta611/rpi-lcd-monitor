use crate::utils::get_ip_address;
use clap::Parser;

#[derive(Parser)]
pub struct HostArg {
    #[clap(
        short = 'p',
        long = "port",
        help = "Port of the app running on the host device",
        default_value = "2784"
    )]
    port: String,
}

pub async fn run(arg: &HostArg) {
    if let Ok(ip_address) = get_ip_address::get() {
        println!("Started host server at http://{}:{}", ip_address, arg.port);
    }
}
