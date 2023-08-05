mod markdown_parse;
mod traverse_files;
mod render_html;
mod posts;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};
use crate::markdown_parse::Post;
use crate::posts::Posts;
use crate::render_html::RenderHtml;


fn generate_html_from_markdown(){
    let posts_path = Path::new("../sources/posts");
    if let Ok(entries) = Path::read_dir(posts_path){
        for entry in entries {
            if let Ok(entry) = entry{
                if let Some(extension) = entry.path().extension(){
                    if extension == "md"{
                        let raw_content = fs::read_to_string(entry.path()).unwrap();
                        let post = markdown_parse::Post::new(&raw_content).unwrap();

                        render_templates(post);
                    }
                }
            }
        }
    }

}

fn render_templates(post: Post){
    let tera = match Tera::new("./templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error initializing Tera: {:?}", e);
            return;
        }
    };

    // 创建数据上下文
    let mut context = Context::new();
    context.insert("title", post.get_title());
    context.insert("content", &tera::Value::String(post.get_content().to_string()));
    context.insert("date", post.get_date());

    let result = tera.render("post.html", &context);

    match result {
        Ok(rendered) => {
            println!("Rendered content:\n{}", rendered);
            let file_path = "output.html";

            let mut file = match File::create(file_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("Error creating file: {:?}", e);
                    return;
                }
            };

            match file.write_all(rendered.as_bytes()) {
                Ok(_) => println!("HTML file generated successfully!"),
                Err(e) => println!("Error writing to file: {:?}", e),
            }
        },
        Err(e) => println!("Error rendering template: {:?}", e),
    }
}

fn main() {
    //generate_html_from_markdown();
    let blog_files = traverse_files::BlogFiles::new("./sources").unwrap();
    let posts= posts::Posts::new(blog_files.get_markdown_file_paths()).unwrap();
    let html_render = render_html::RenderHtml::new("templates/**/*.html", &posts, &blog_files).unwrap();
    html_render.render_html("./output");
}