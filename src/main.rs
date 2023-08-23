mod file_handling;
mod parsing;
mod render;

// mod traverse_files;
// mod render_html;
// mod posts;
// mod tags;
// mod categories;
use std::io::Write;
// use crate::categories::Categories;
// use crate::tags::Tags;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_files::Files;
use crate::file_handling::FileHandler;
use crate::parsing::post_parse;
use crate::render::render_html;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let file_handling = FileHandler::new().unwrap();
    let mut posts = post_parse(&file_handling).unwrap();

    render_html(&file_handling,&mut posts);

    HttpServer::new(|| {
        App::new().service(Files::new("/", "public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// fn main() {
//     // let blog_files = traverse_files::BlogFiles::new("./sources").unwrap();
//     // let posts= posts::Posts::new(blog_files.get_markdown_file_paths()).unwrap();
//     // let tags = Tags::new(&posts).unwrap();
//     // let categories = Categories::new(&posts).unwrap();
//     // let html_render = render_html::RenderHtml::new("./sources/templates/**/*.html", &posts, &blog_files,&tags,&categories).unwrap();
//     // html_render.render_html("./output");
//
// }