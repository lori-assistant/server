use actix_web::{web, App, HttpResponse, HttpServer, Result};
use awc::Client;
use serde_derive::{Deserialize, Serialize};
use colored::*;

static LOGO: &str = "    __           _
   / /___  _____(_)     ________  ______   _____  _____
  / / __ \\/ ___/ /_____/ ___/ _ \\/ ___/ | / / _ \\/ ___/
 / / /_/ / /  / /_____(__  )  __/ /   | |/ /  __/ /
/_/\\____/_/  /_/     /____/\\___/_/    |___/\\___/_/\n\n";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", LOGO.cyan().bold());

    println!("{} Listening on 127.0.0.1:3000", "[server]".blue().bold());
    HttpServer::new(|| {
        App::new()
            .route("/api/generate", web::post().to(generate_completion))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

#[derive(Deserialize, Serialize)]
struct GenerateCompletionRequest {
    model: String,
    prompt: Option<String>,
    options: Option<String>,
    system: Option<String>,
    template: Option<String>,
    context: Option<Vec<i32>>,
}

async fn generate_completion(data: web::Json<GenerateCompletionRequest>) -> Result<HttpResponse> {
    // Make a client request to the given URL
    let forwarded_response = Client::default()
        .post("http://localhost:11434/api/generate")
        .send_json(&data)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Stream the response back to the client
    Ok(HttpResponse::build(forwarded_response.status())
        .streaming(forwarded_response))
}

// Plugin system stub
trait Plugin {
    fn execute(&self, input: &str) -> String;
}

struct SamplePlugin;

impl Plugin for SamplePlugin {
    fn execute(&self, input: &str) -> String {
        format!("SamplePlugin processed: {}", input)
    }
}

fn execute_plugin(input: &str) -> String {
    let plugin: Box<dyn Plugin> = Box::new(SamplePlugin);
    plugin.execute(input)
}
