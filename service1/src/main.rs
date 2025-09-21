use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use reqwest::Client;
use sysinfo::Disks;
use chrono::Utc;
use lazy_static::lazy_static;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;

// Define global start time
lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

async fn get_status() -> impl Responder {

    let mut record = String::new();
    let free_disk = get_disk() as f64 / (1024.0 * 1024.0); // Convert bytes to MB
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let uptime_hours = getuptime_hours();

    record.push_str(&format!("{}: ", timestamp));
    record.push_str(&format!("uptime {:.5} hours, ", uptime_hours));
    record.push_str(&format!("free disk in root: {:.2} MBytes", free_disk));

    post_log(record.clone()).await;
    log_to_vstorage(&record);

    let s2_status = get_s2_status().await;
    record.push_str(&format!("\n{}", s2_status));

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(record)
    
}

async fn get_s2_status() -> String {
    let client = Client::new();

    match client.get("http://service2:8080/status").send().await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => return text,
                Err(err) => return format!("Failed to read body: {}", err),
            }
        }
        Err(err) => return format!("Request failed: {}", err),
    }
}


fn getuptime_hours() -> f64 {
    START_TIME.elapsed().as_secs_f64() / 3600.0
}


fn get_disk() -> u64 {
    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        let mount = disk.mount_point().to_string_lossy();
        if mount == "/" || mount.to_lowercase().starts_with("c:\\") {
            let available = disk.available_space();
            return available;
        }
    }
    0
}


async fn get_log() -> impl Responder {
    let client = Client::new();

    match client.get("http://storage:5000/log").send().await { // CHANGE THE URL
        Ok(response) => {
            match response.text().await {
                Ok(text) => HttpResponse::Ok().body(text),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Failed to real body: {}", err)),
            }
        }
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Request failed: {}", err)),
    }
}

fn log_to_vstorage(record: &str) {
    let path = "/vstorage/log.txt";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{}", record);
    }
}

async fn post_log(record: String) -> impl Responder {
    let client = Client::new();

    let result = client.post("http://storage:5000/log")
        .header("Content-Type", "text/plain")
        .body(record.clone())
        .send()
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok()
                .content_type("text/plain")
                .body(record.clone())
        },
        Err(_) => HttpResponse::InternalServerError()
            .body("Failed to send log to storage."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("")
                .route("status", web::get().to(get_status))
                .route("log", web::get().to(get_log))
            )
    })
    .bind("0.0.0.0:8199")?
    .run()
    .await
}