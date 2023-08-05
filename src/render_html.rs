use std::fs;
use std::path::PathBuf;
use grass;
use tera::{Context, Tera};
use crate::posts::Posts;
use crate::tags::Tags;
use crate::traverse_files::BlogFiles;

pub struct RenderHtml<'a>  {
    template_paths:&'a str,
    posts: &'a Posts,
    blog_files: &'a BlogFiles,
    tags: &'a Tags<'a>
}

impl RenderHtml<'_> {
    pub fn new<'a>(template_paths:&'a str ,posts: &'a Posts, blog_files: &'a BlogFiles, tags:&'a Tags)->Option<RenderHtml<'a>>{
        let template_paths = template_paths;
        let posts = posts;
        let blog_files=blog_files;
        let tags= tags;
        Some(RenderHtml{template_paths,posts,blog_files,tags})
    }

    pub fn render_html(&self,output_path: &str){
        for scss_file in self.blog_files.get_scss_file_paths(){
            let file_name_without_extension = scss_file.file_stem().unwrap().to_string_lossy().to_string();
            let output_path = format!("public/static/css/{}.css", file_name_without_extension);
            let css_content = grass::from_path(scss_file,&grass::Options::default()).unwrap();

            fs::write(output_path, css_content);

        }
        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        for template in self.blog_files.get_template_file_paths(){
            let template_file_name = template.file_name().unwrap().to_str().unwrap();
            match template_file_name {
                "post.html" => {
                    for post in self.posts.get_posts(){
                        let mut context = Context::new();
                        context.insert("page", &post);

                        let post_output_path = format!("public/posts/{}.html", post.get_slug());
                        render_and_save_to_html(&tera, "post.html", &post_output_path, &context).unwrap();
                    }
                }
                "tags.html" => {
                    for tag in self.tags.get_tags(){
                        let mut context = Context::new();
                        context.insert("tag_name", tag.0);
                        context.insert("page", tag.1);

                        let post_output_path = format!("public/tags/{}.html", tag.0);
                        render_and_save_to_html(&tera, "tags.html", &post_output_path, &context).unwrap();
                    }
                }
                &_ => {
                    let mut context = Context::new();
                    context.insert("posts", &self.posts.get_posts());
                    let output_path = format!("public/{}", template_file_name);
                    render_and_save_to_html(&tera, &template_file_name, &output_path, &context).unwrap();
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
    }
}
// pub fn A(blog_files: &BlogFiles){
//     // 初始化 Tera 模板引擎
//     let tera = Tera::new("./sources/templates/**/*").unwrap();
//
//     let mut context = Context::new();
//     context.insert("title", "My Website");
//     context.insert("date", "2023");
//     context.insert("content", "Welcome to my website!");
//     context.insert("aaa", "hty");
//
//     let output_path = PathBuf::from("output"); // 替换为实际的输出目录
//
//     for entry in blog_files.get_template_file_paths(){
//         let file_path = entry.as_path();
//
//         let file_name = file_path.file_name().unwrap().to_str().unwrap();
//
//         match file_name {
//             "post.html" => {
//                 let mut context = Context::new();
//                 context.insert("posts", );
//             },
//             _ => {}
//         }
//         // println!("{}",file_name);
//         //
//         // if file_path.is_file() {
//         //     let rendered_content = tera.render(file_name, &context).unwrap();
//         //
//         //     let output_file_path = output_path.join(PathBuf::from(file_name));
//         //
//         //     // 将渲染后的内容写入到文件
//         //     fs::write(output_file_path, rendered_content).unwrap();
//         // }
//
//     }
// }