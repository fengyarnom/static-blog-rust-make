use tera::{Context, Tera};
use crate::posts::Posts;
use crate::traverse_files::BlogFiles;

pub struct RenderHtml<'a>  {
    template_paths:&'a str,
    posts: &'a Posts,
    blog_files: &'a BlogFiles,
}

impl RenderHtml<'_> {
    pub fn new<'a>(template_paths:&'a str ,posts: &'a Posts, blog_files: &'a BlogFiles)->Option<RenderHtml<'a>>{
        let template_paths = template_paths;
        let posts = posts;
        let blog_files=blog_files;
        Some(RenderHtml{template_paths,posts,blog_files})
    }

    pub fn render_html(&self,output_path: &str){
        let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();
        println!("{:?}",tera);
        for post in self.posts.get_posts(){
            println!("{:?}",post);
            let mut context = Context::new();
            context.insert("post", &post);

            let post_output_path = format!("output/posts/{}.html", post.get_slug());
            render_and_save_to_html(&tera, "post.html", &post_output_path, &context).unwrap();
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