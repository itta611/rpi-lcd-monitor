use std::io;

use async_process::Command;
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

pub struct Stats {
    pub used_mem: f32,
    pub total_mem: f32,
    pub cpu_usage: f32,
    pub cpu_tempreture: f32,
}

async fn get_cpu_tempreture() -> Result<String, io::Error> {
    let output = Command::new("cat")
        .arg("/sys/class/thermal/thermal_zone0/temp")
        .output()
        .await?;

    if String::from_utf8_lossy(&output.stderr) != "" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Tempreture file not found",
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stderr);
    Ok(stdout.trim().to_string())
}

pub async fn get() -> Stats {
    let mut system =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    system.refresh_memory();

    let used_mem =
        (system.used_memory() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    let total_mem =
        (system.total_memory() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let cpu_tempreture = get_cpu_tempreture().await.unwrap().parse::<f32>().unwrap() / 1000.0;

    Stats {
        used_mem,
        total_mem,
        cpu_usage,
        cpu_tempreture,
    }
}
