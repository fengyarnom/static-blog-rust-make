
mod generate;

use std::{env, process};
use std::io::Write;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_files::Files;
use crate::generate::SiteGenerator;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, Result};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use chrono::Utc;
use tokio::spawn;



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
    println!("Running server!");
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => {
                generate();
                println!("Restarted server!");
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();
    watcher.watch(Path::new("./sources/"), RecursiveMode::Recursive).unwrap();

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
        "new" => {
            if args.len() < 3 {
                println!("Usage: new 'post title'");
                return;
            }
            let local_time = Utc::now().naive_utc();
            let post_title = args[2].as_str();
            let file_name = format!("sources/content/posts/{}-{}.md", local_time.format("%Y-%m-%d"),post_title.to_lowercase().replace(" ", "-"));

            let file_content = format!(
                                    "---
title: {}
date: {}
tags:
categories:
---",post_title,local_time.format("%Y-%m-%d %H:%M:%S"));

            let mut file = File::create(&file_name).unwrap();
            file.write_all(file_content.as_bytes()).unwrap();
            println!("New post created: {}", &file_name);

        }
        _ => println!("Unknown command: {}", command),
    }

}

