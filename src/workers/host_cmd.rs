use crate::types::IndexResponse;
use crate::utils::get_ip_address;
use actix_web::*;
use clap::Parser;
use rppal;

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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(IndexResponse {
        message: "`rpi-lcd-monitor` host server is running".to_string(),
    })
}

pub async fn run(arg: &HostArg) -> std::io::Result<()> {
    const I2C_ADDRESS: u16 = 0x27;
    let mut i2c = I2c::with_bus(1)?;
    let buffer: &[u8] = b"Hello, World!";
    let bytes_written = i2c.write(buffer)?;

    println!("{} bytes written to the LCD", bytes_written);

    if let Ok(ip_address) = get_ip_address::get() {
        println!("Started host server at http://{}:{}", ip_address, arg.port);
    }

    HttpServer::new(|| App::new().service(index))
        .bind(format!("127.0.0.1:{}", arg.port))?
        .run()
        .await
}
