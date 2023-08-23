use std::collections::HashMap;
use crate::parsing::{Post};
use std::fs;
use std::path::PathBuf;
use actix_web::web::post;
use grass;
use serde::Serialize;
use tera::{Context, Tera};
use String;
use crate::file_handling::FileHandler;

#[derive(Serialize, Debug)]
struct Site<'a>{
    posts: &'a Vec<Post>
}

pub struct Tags<'a>{
    tags: HashMap<String, Vec<&'a Post>>,
}
#[derive(Serialize, Clone)]
struct Page<'a>{
    posts: &'a Vec<Post>,
    total_page: usize,
    page_number:usize,
    pre_page:usize,
    next_page:usize
}
pub fn render_html(file_handling: &FileHandler, posts: &Vec<Post>){
    for scss_file in &file_handling.scss_files{
        let scss_file_path = PathBuf::from(&scss_file.file_path);
        let file_name_without_extension = scss_file_path.file_stem().unwrap().to_string_lossy().to_string();

        let output_path = format!("public/static/css/{}.css", file_name_without_extension);
        let css_content = grass::from_path(scss_file_path,&grass::Options::default()).unwrap();

        fs::write(output_path, css_content);
    }

    let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
    for template_file in &file_handling.template_files{
        let template_file_path = PathBuf::from(&template_file.file_path);
        let template_file_name = template_file_path.file_name().unwrap().to_str().unwrap();
        match template_file_name {
            "post.html" => {
                for post in posts{
                    let mut context = Context::new();
                    context.insert("page", &post);

                    let post_output_path = format!("public/posts/{}.html", post.slug);
                    render_and_save_to_html(&tera, "post.html", &post_output_path, &context).unwrap();
                }
            }
            "index.html" => {
                let page_size = posts.len();
                let mut per_page = 6;
                let total_page = page_size/per_page;

                let mut page_number = 1;
                let mut page_start = 0;
                let mut page_end = per_page;

                while page_number<=total_page+1 {
                    let mut pre_page = page_number - 1;
                    let mut next_page = page_number + 1;

                    if pre_page < 1 {
                        pre_page = 1;
                    }
                    if next_page > total_page {
                        next_page = total_page;
                    }

                    let posts = &posts[page_start..page_end].to_vec();
                    let page = Page{ posts,total_page,page_number,pre_page,next_page };

                    let mut context = Context::new();
                    context.insert("page", &page);
                    let mut output_path;
                    if page_number == 1{
                        output_path = format!("public/{}", template_file_name);
                    }else{
                        output_path = format!("public/page");

                        if !fs::metadata(&output_path).is_ok() {
                            if let Err(err) = fs::create_dir(&output_path) {
                                eprintln!("Error creating 'public/page' folder: {}", err);
                            }
                        }

                        output_path = format!("public/page/{}",page_number);

                        if !fs::metadata(&output_path).is_ok() {
                            if let Err(err) = fs::create_dir(&output_path) {
                                eprintln!("Error creating 'public/page/number' folder: {}", err);
                            }
                        }
                        output_path = format!("public/page/{}/{}", page_number,template_file_name);

                    }

                    render_and_save_to_html(&tera, &template_file_name, &output_path, &context).unwrap();

                    page_start += 6;
                    page_end += 6;
                    page_number+=1;

                    if(page_end > page_size){
                        page_end = page_size-1;
                    }
                }
                let posts = &posts[0..4].to_vec();


            }
            "tags.html" => {
                let mut tags: HashMap<String, Vec<&Post>> = HashMap::new();
                for post in posts{
                    for tag in &post.tags{
                        tags.entry(tag.to_string()).or_insert(Vec::new()).push(&post);
                    }
                }
                for tag in &tags{
                    let mut output_path = format!("public/tags/{}", tag.0.to_string());
                    if !fs::metadata(&output_path).is_ok() {
                        if let Err(err) = fs::create_dir(&output_path) {
                            eprintln!("Error creating 'public/tags/???' folder: {}", err);
                        }
                    }
                    let mut context = Context::new();
                    context.insert("page", tag.1);
                    context.insert("tag_name", tag.0);
                    output_path = format!("public/tags/{}/index.html", tag.0.to_string());
                    render_and_save_to_html(&tera, "archive.html", &output_path, &context).unwrap();
                }

                let mut context = Context::new();
                context.insert("page", &tags);

                let output_path = format!("public/tags/index.html");
                render_and_save_to_html(&tera, "tags.html", &output_path, &context).unwrap();
            }
            "categories.html" => {
                let mut categories: HashMap<String, Vec<&Post>> = HashMap::new();
                for post in posts{
                    for category in &post.categories{
                        categories.entry(category.to_string()).or_insert(Vec::new()).push(&post);
                    }
                }
                for category in &categories{
                    let mut output_path = format!("public/categories/{}", category.0.to_string());
                    if !fs::metadata(&output_path).is_ok() {
                        if let Err(err) = fs::create_dir(&output_path) {
                            eprintln!("Error creating 'public/category/???' folder: {}", err);
                        }
                    }
                    let mut context = Context::new();
                    context.insert("page", category.1);
                    context.insert("tag_name", category.0);
                    output_path = format!("public/categories/{}/index.html", category.0.to_string());
                    render_and_save_to_html(&tera, "archive.html", &output_path, &context).unwrap();
                }

                let mut context = Context::new();
                context.insert("page", &categories);

                let output_path = format!("public/categories/index.html");
                render_and_save_to_html(&tera, "categories.html", &output_path, &context).unwrap();
            }
            _ => {}
        }
    }
}


fn render_and_save_to_html(tera: &Tera, template_name: &str, output_path: &str, data: &Context) -> Result<(), Box<dyn std::error::Error>> {
    // 渲染模板并生成 HTML 文件
    let rendered_content = tera.render(template_name, data)?;

    // 将渲染后的内容写入到文件
    std::fs::write(output_path, rendered_content)?;

    Ok(())
}


