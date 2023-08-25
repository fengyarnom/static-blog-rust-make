
mod generate;

use std::{env, process};
use std::io::Write;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_files::Files;
use crate::generate::SiteGenerator;
use pulldown_cmark::{Parser, Options, html};
// #[actix_web::main] // or #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let file_handling = FileHandler::new().unwrap();
//     let mut posts = post_parse(&file_handling).unwrap();
//
//     Page::render_static_source();
//     //render_html(&file_handling,&mut posts);
//
//     HttpServer::new(|| {
//         App::new().service(Files::new("/", "public").index_file("index.html"))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
fn generate() {
    // 实现生成操作的逻辑
    println!("Generating...");
    let site_generator =
        SiteGenerator::new("./sources/content",
                           "./sources/templates",
                           "./sources/templates/custom_templates",
                           "./public");

    site_generator.generate_site();
}


async fn server() -> std::io::Result<()> {
    // 实现服务器启动操作的逻辑
    println!("Starting server...");
    generate();
    HttpServer::new(|| {
        App::new().service(Files::new("/", "public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: my_program <command>");
        return;
    }

    let command = args[1].as_str();
    match command {
        "generate" => generate(),
        "server" => {
            if let Err(e) = server().await {
                eprintln!("Error starting server: {}", e);
                process::exit(1);
            }
        }
        _ => println!("Unknown command: {}", command),
    }

}

