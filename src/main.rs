use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use comrak::{markdown_to_html, ComrakOptions};
use tera::Tera;
use tera::Context;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Post {
    title: String,
    date: String,
    tags: Vec<String>,
    categories: Vec<String>,
    content:String
}

fn generate_html_from_mardown(){
    let posts_path = Path::new("./posts");
    if let Ok(entries) = Path::read_dir(posts_path){
        for entry in entries {
            if let Ok(entry) = entry{
                if let Some(extension) = entry.path().extension(){
                    if extension == "md"{
                        let raw_content = fs::read_to_string(entry.path()).unwrap();
                        let post = parse_markdown(&raw_content).unwrap();

                        render_templates(&post);
                    }
                }
            }
        }
    }
}
fn parse_markdown(raw_content: &str) -> Option<Post>{
    let matter = Matter::<YAML>::new();
    let parsed_entity = matter.parse(&raw_content);

    let frontmatter = parsed_entity.data.as_ref().unwrap().as_hashmap().unwrap();
    let markdown_content = parsed_entity.content;
    let html_content = parse_markdown2html(&markdown_content).unwrap();

    let mut post = Post{
        title: "".to_string(),
        date: "".to_string(),
        tags: vec![],
        categories: vec![],
        content: html_content,
    };

    for item_frontmatter in &frontmatter {
        match item_frontmatter.0.as_str() {
            "title" => {
                post.title = item_frontmatter.1.as_string().unwrap();
            },
            "date" => {
                post.date = item_frontmatter.1.as_string().unwrap();
            },
            "tags" => {
                if !item_frontmatter.1.is_empty(){
                    for tag in item_frontmatter.1.as_vec().unwrap(){
                        post.tags.push(tag.as_string().unwrap())
                    }
                }
            },

            "categories" => {
                if !item_frontmatter.1.is_empty(){
                    for category in item_frontmatter.1.as_vec().unwrap(){
                        post.categories.push(category.as_string().unwrap())
                    }
                }
            },
            _ => println!("Unknown frontmatter item"),
        }

    }

    Some(post)

}

fn render_templates(post: &Post){
    let tera = match Tera::new("./templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error initializing Tera: {:?}", e);
            return;
        }
    };

    // 创建数据上下文
    let mut context = Context::new();
    context.insert("title", &post.title);
    context.insert("content", &tera::Value::String(post.content.to_string()));
    context.insert("date", &post.date);

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

fn parse_markdown2html(markdown_content: &str) -> Option<String>{
    let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());
    Some(html_content)
}

fn main() {
    generate_html_from_mardown();
}