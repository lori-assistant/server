use actix_web::{web, App, HttpResponse, HttpServer, Result, Error, dev::Service};
use awc::Client;
use serde_derive::{Deserialize, Serialize};
use colored::*;
use std::time::Duration;

static LOGO: &str = "    __           _
   / /___  _____(_)     ________  ______   _____  _____
  / / __ \\/ ___/ /_____/ ___/ _ \\/ ___/ | / / _ \\/ ___/
 / / /_/ / /  / /_____(__  )  __/ /   | |/ /  __/ /
/_/\\____/_/  /_/     /____/\\___/_/    |___/\\___/_/\n\n";

fn rainbow_format(s: &str) -> String {
    let colors = [
        "red", "yellow", "green", "cyan", "blue", "magenta"
    ];
    let mut colored_string = String::new();

    for line in s.lines() {
        let mut colored_line = String::new();
        for (char_index, char) in line.chars().enumerate() {
            // This multiplication parameter defines the frequency of the rainbow. The current setting
            // should be in the range of about `2π` to `4π` (one or two full circle rotations).
            let color_index = (char_index as f32 * 0.2) as usize % colors.len();
            colored_line += &char.to_string().color(colors[color_index]).to_string();
        }
        colored_string += &colored_line;
        colored_string += "\n";
    }

    colored_string
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", rainbow_format(LOGO));
    println!("{} Listening on 127.0.0.1:3000", "[server]".blue().bold());

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("{} Incoming request: {} {}", "[request]".green().bold(), req.method(), req.path());
                let fut = srv.call(req);
                async {
                    match fut.await {
                        Ok(res) => {
                            Ok(res)
                        },
                        Err(e) => {
                            println!("{} Error occurred: {:?}", "[error]".red().bold(), e);
                            Err(e)
                        }
                    }
                }
            })
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

async fn generate_completion(data: web::Json<GenerateCompletionRequest>) -> Result<HttpResponse, Error> {
    // Make a client request to the given URL with a timeout
    let forwarded_response = Client::default()
        .post("http://localhost:11434/api/generate")
        .timeout(Duration::from_secs(120)) // Set timeout to 2 minutes
        .send_json(&data)
        .await
        .map_err(|e| {
            println!("{} Error in generate_completion: {:?}", "[error]".red().bold(), e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

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
