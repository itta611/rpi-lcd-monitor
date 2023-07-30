use crate::utils::get_ip_address;
use actix_web::*;
use clap::Parser;
use serde::Serialize;

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

#[derive(Serialize)]
struct IndexResponse {
    status: String,
    message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(IndexResponse {
        status: "OK".to_string(),
        message: "`rpi-lcd-monitor` host server is running".to_string(),
    })
}

pub async fn run(arg: &HostArg) -> std::io::Result<()> {
    if let Ok(ip_address) = get_ip_address::get() {
        println!("Started host server at http://{}:{}", ip_address, arg.port);
    }

    HttpServer::new(|| App::new().service(index))
        .bind(format!("127.0.0.1:{}", arg.port))?
        .run()
        .await
}
