use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;
use sysinfo::{System, Disks}; 

#[derive(Serialize)]
struct SystemInfo {
    os_name: String,
    kernel_version: String,
    host_name: String,
    cpu_name: String,
    cpu_usage: f32, 
    total_memory_gb: f64,
    used_memory_gb: f64,
    total_disk_gb: f64,
    available_disk_gb: f64,
    uptime_formatted: String,
}


const GB: f64 = 1_073_741_824.0; 


fn round_to_one_decimal(num: f64) -> f64 {
    (num * 10.0).round() / 10.0
}


fn format_uptime(total_seconds: u64) -> String {
    if total_seconds == 0 {
        return "0m".to_string(); 
    }
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;

    let seconds = total_seconds % 60;

    let mut result = String::new();
    if hours > 0 {
        result.push_str(&format!("{}h ", hours));
    }

    if minutes > 0 || hours > 0 || result.is_empty() {
         result.push_str(&format!("{}m", minutes));
    }
    if seconds > 0 || result.is_empty() { 
       result.push_str(&format!(" {}s", seconds));
    }

    result.trim().to_string() 
}


#[get("/system_info")]
async fn get_system_info() -> impl Responder {
    let mut sys = System::new();
    sys.refresh_cpu();
    sys.refresh_memory();
 

    // --- Informações Estáticas ---
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let uptime_seconds = System::uptime();
    let uptime_formatted = format_uptime(uptime_seconds); 
    let cpu_name = sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "Unknown".to_string());
    let cpu_usage = sys.global_cpu_info().cpu_usage(); 

    let total_memory_gb = round_to_one_decimal(sys.total_memory() as f64 / GB);
    let used_memory_gb = round_to_one_decimal(sys.used_memory() as f64 / GB);

    // --- Informações de Disco ---
    let disks = Disks::new_with_refreshed_list();
    let mut total_disk_space: u64 = 0;
    let mut available_disk_space: u64 = 0;

    for disk in disks.list() {
        total_disk_space += disk.total_space();
        available_disk_space += disk.available_space();
    }

    // Conversão de disco para GB e arredondamento
    let total_disk_gb = round_to_one_decimal(total_disk_space as f64 / GB);
    let available_disk_gb = round_to_one_decimal(available_disk_space as f64 / GB);


    let system_info = SystemInfo {
        os_name,
        kernel_version,
        host_name,
        cpu_name,
        cpu_usage, 
        total_memory_gb,
        used_memory_gb,
        total_disk_gb,
        available_disk_gb,
        uptime_formatted, 
    };

    web::Json(system_info)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = ("0.0.0.0", 8080);
    println!("Starting server at http://{}:{}", bind_address.0, bind_address.1);

    HttpServer::new(|| {
        App::new().service(get_system_info)
    })
    .bind(bind_address)?
    .run()
    .await
}
