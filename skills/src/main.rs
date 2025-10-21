use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Deserialize)]
struct ExecuteRequest {
    code: String,
}

#[derive(Serialize)]
struct ExecuteResponse {
    stdout: String,
    stderr: String,
}

async fn execute(req: web::Json<ExecuteRequest>) -> impl Responder {
    match Command::new("python")
        .arg("-c")
        .arg(&req.code)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            HttpResponse::Ok().json(ExecuteResponse { stdout, stderr })
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to execute process: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/execute", web::post().to(execute))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
