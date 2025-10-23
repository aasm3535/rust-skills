use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use chrono::Utc;

mod history;
use history::{History, HistoryEntry};

type AppState = web::Data<Mutex<History>>;

#[derive(Deserialize)]
struct ExecuteRequest {
    code: String,
}

#[derive(Serialize)]
struct ExecuteResponse {
    stdout: String,
    stderr: String,
}

async fn execute(req: web::Json<ExecuteRequest>, app_state: AppState) -> impl Responder {
    match Command::new("python")
        .arg("-c")
        .arg(&req.code)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            let entry = HistoryEntry {
                code: req.code.clone(),
                stdout: stdout.clone(),
                stderr: stderr.clone(),
                timestamp: Utc::now(),
            };

            let mut history = app_state.lock().unwrap();
            history.add(entry);

            HttpResponse::Ok().json(ExecuteResponse { stdout, stderr })
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to execute process: {}", e))
        }
    }
}

async fn get_history(app_state: AppState) -> impl Responder {
    let history = app_state.lock().unwrap();
    HttpResponse::Ok().json(&history.entries)
}

pub async fn run() -> std::io::Result<()> {
    let app_state = web::Data::new(Mutex::new(History::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/execute", web::post().to(execute))
            .route("/history", web::get().to(get_history))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
